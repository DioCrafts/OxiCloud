use bytes::{Bytes, BytesMut};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;

use crate::application::dtos::file_dto::FileDto;
use crate::application::ports::file_ports::{FileRetrievalUseCase, OptimizedFileContent};
use crate::application::ports::storage_ports::FileReadPort;
use crate::common::errors::DomainError;
use tracing::{debug, info};
use crate::infrastructure::repositories::pg::file_blob_read_repository::FileBlobReadRepository;
use crate::infrastructure::services::file_content_cache::FileContentCache;
use crate::infrastructure::services::image_transcode_service::{ImageTranscodeService, OutputFormat};

/// Threshold below which files are served from RAM cache (10 MB).
const CACHE_THRESHOLD: u64 = 10 * 1024 * 1024;

/// Service for file retrieval operations
///
/// Implements a multi-tier download strategy:
/// - Tier 0: Write-behind cache (just-uploaded files still in RAM)
/// - Tier 1: Hot cache + optional WebP transcoding (<10 MB)
/// - Tier 2: Memory-mapped I/O (10–100 MB)
/// - Tier 3: Streaming (≥100 MB)
pub struct FileRetrievalService {
    file_read: Arc<FileBlobReadRepository>,
    content_cache: Option<Arc<FileContentCache>>,
    transcode: Option<Arc<ImageTranscodeService>>,
}

impl FileRetrievalService {
    /// Backward-compatible constructor (simple pass-through).
    pub fn new(file_repository: Arc<FileBlobReadRepository>) -> Self {
        Self {
            file_read: file_repository,
            content_cache: None,
            transcode: None,
        }
    }

    /// Constructor for blob-storage model: read + content cache + transcode.
    pub fn new_with_cache(
        file_read: Arc<FileBlobReadRepository>,
        content_cache: Arc<FileContentCache>,
        transcode: Arc<ImageTranscodeService>,
    ) -> Self {
        Self {
            file_read,
            content_cache: Some(content_cache),
            transcode: Some(transcode),
        }
    }

    // ── private helpers ──────────────────────────────────────────

    /// Try to transcode image content to WebP and return transcoded variant.
    async fn try_transcode(
        &self,
        id: &str,
        content: &Bytes,
        mime: &str,
        file_size: u64,
        accept_webp: bool,
    ) -> Option<(Bytes, Arc<str>)> {
        if !accept_webp {
            return None;
        }
        let transcode = self.transcode.as_ref()?;
        if !ImageTranscodeService::should_transcode(mime, file_size) {
            return None;
        }
        let format = OutputFormat::WebP;
        match transcode
            .get_transcoded(id, content.clone(), mime, format)
            .await
        {
            Ok((transcoded, webp_mime, true)) => {
                debug!(
                    "🖼️ WebP transcode: {} -> {} bytes ({:.0}% smaller)",
                    content.len(),
                    transcoded.len(),
                    (1.0 - transcoded.len() as f64 / content.len().max(1) as f64) * 100.0
                );
                Some((transcoded, Arc::from(&*webp_mime)))
            }
            _ => None,
        }
    }

    /// Core multi-tier download logic shared by `get_file_optimized` and
    /// `get_file_optimized_preloaded`.
    async fn optimized_inner(
        &self,
        id: &str,
        dto: FileDto,
        accept_webp: bool,
        prefer_original: bool,
    ) -> Result<(FileDto, OptimizedFileContent), DomainError> {
        let mime_type = dto.mime_type.clone();
        let file_size = dto.size;
        let file_name = dto.name.clone();
        let modified_at = dto.modified_at;
        let do_transcode = accept_webp && !prefer_original;

        // ── Tier 1: Hot cache + transcode (<10 MB) ──────────
        if file_size < CACHE_THRESHOLD {
            // Check content cache first
            if let Some(cache) = &self.content_cache
                && let Some((cached, _etag, _ct)) = cache.get(id).await
            {
                debug!(
                    "🔥 TIER 1 Cache HIT: {} ({} bytes)",
                    file_name,
                    cached.len()
                );
                if do_transcode
                    && let Some((t, m)) = self
                        .try_transcode(id, &cached, &mime_type, file_size, true)
                        .await
                {
                    return Ok((
                        dto,
                        OptimizedFileContent::Bytes {
                            data: t,
                            mime_type: m,
                            was_transcoded: true,
                        },
                    ));
                }
                return Ok((
                    dto,
                    OptimizedFileContent::Bytes {
                        data: cached,
                        mime_type: mime_type.clone(),
                        was_transcoded: false,
                    },
                ));
            }

            // Cache miss – load from disk via streaming (constant 64 KB memory)
            debug!("💾 TIER 1 Cache MISS: {} – loading from disk", file_name);
            let stream = self.file_read.get_file_stream(id).await?;
            let mut stream = std::pin::Pin::from(stream);
            let mut buf = BytesMut::with_capacity(file_size as usize);
            while let Some(chunk) = stream.next().await {
                buf.extend_from_slice(&chunk.map_err(|e| {
                    DomainError::internal_error("File", format!("Stream read error: {}", e))
                })?);
            }
            let content_bytes = buf.freeze();

            // Store in cache
            if let Some(cache) = &self.content_cache {
                let etag: Arc<str> = format!("\"{}-{}\"", id, modified_at).into();
                let ct: Arc<str> = mime_type.clone();
                cache
                    .put(id.to_string(), content_bytes.clone(), etag, ct)
                    .await;
            }

            if do_transcode
                && let Some((t, m)) = self
                    .try_transcode(id, &content_bytes, &mime_type, file_size, true)
                    .await
            {
                return Ok((
                    dto,
                    OptimizedFileContent::Bytes {
                        data: t,
                        mime_type: m,
                        was_transcoded: true,
                    },
                ));
            }
            return Ok((
                dto,
                OptimizedFileContent::Bytes {
                    data: content_bytes,
                    mime_type: mime_type.clone(),
                    was_transcoded: false,
                },
            ));
        }

        // ── Tier 2 + 3: Streaming (≥10 MB) ──────────────────
        info!(
            "📡 TIER 2 STREAMING: {} ({} MB)",
            file_name,
            file_size / (1024 * 1024)
        );
        let stream = self.file_read.get_file_stream(id).await?;
        Ok((dto, OptimizedFileContent::Stream(Box::into_pin(stream))))
    }
}

impl FileRetrievalUseCase for FileRetrievalService {
    async fn get_file(&self, id: &str) -> Result<FileDto, DomainError> {
        let file = self.file_read.get_file(id).await?;
        Ok(FileDto::from(file))
    }

    async fn get_file_owned(&self, id: &str, caller_id: &str) -> Result<FileDto, DomainError> {
        let file = self.file_read.get_file_for_owner(id, caller_id).await?;
        Ok(FileDto::from(file))
    }

    async fn get_file_by_path(&self, path: &str) -> Result<FileDto, DomainError> {
        // Direct SQL lookup — O(folder_depth) queries instead of O(total_files)
        if let Some(file) = self.file_read.find_file_by_path(path).await? {
            return Ok(FileDto::from(file));
        }

        Err(DomainError::not_found(
            "File",
            format!("not found at path: {}", path),
        ))
    }

    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<FileDto>, DomainError> {
        let files = self.file_read.list_files(folder_id).await?;
        Ok(files.into_iter().map(FileDto::from).collect())
    }

    async fn get_file_stream(
        &self,
        id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        self.file_read.get_file_stream(id).await
    }

    /// Multi-tier optimized download.
    async fn get_file_optimized(
        &self,
        id: &str,
        accept_webp: bool,
        prefer_original: bool,
    ) -> Result<(FileDto, OptimizedFileContent), DomainError> {
        let file = self.file_read.get_file(id).await?;
        let dto = FileDto::from(file);
        self.optimized_inner(id, dto, accept_webp, prefer_original)
            .await
    }

    async fn get_file_optimized_owned(
        &self,
        id: &str,
        caller_id: &str,
        accept_webp: bool,
        prefer_original: bool,
    ) -> Result<(FileDto, OptimizedFileContent), DomainError> {
        let file = self.file_read.get_file_for_owner(id, caller_id).await?;
        let dto = FileDto::from(file);
        self.optimized_inner(id, dto, accept_webp, prefer_original)
            .await
    }

    /// Like `get_file_optimized` but skips the metadata re-fetch.
    async fn get_file_optimized_preloaded(
        &self,
        id: &str,
        file_dto: FileDto,
        accept_webp: bool,
        prefer_original: bool,
    ) -> Result<(FileDto, OptimizedFileContent), DomainError> {
        self.optimized_inner(id, file_dto, accept_webp, prefer_original)
            .await
    }

    /// Range-based streaming for HTTP Range Requests.
    async fn get_file_range_stream(
        &self,
        id: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        self.file_read.get_file_range_stream(id, start, end).await
    }

    async fn get_file_range_stream_owned(
        &self,
        id: &str,
        caller_id: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        // Verify ownership first, then delegate to the unscoped stream
        self.file_read.verify_file_owner(id, caller_id).await?;
        self.file_read.get_file_range_stream(id, start, end).await
    }

    async fn stream_files_in_subtree(
        &self,
        folder_id: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<FileDto, DomainError>> + Send>>, DomainError> {
        let inner = self.file_read.stream_files_in_subtree(folder_id).await?;
        let mapped = inner.map(|r| r.map(FileDto::from));
        Ok(Box::pin(mapped))
    }

    async fn list_files_batch(
        &self,
        folder_id: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<FileDto>, DomainError> {
        let files = self
            .file_read
            .list_files_batch(folder_id, offset, limit)
            .await?;
        Ok(files.into_iter().map(FileDto::from).collect())
    }
}
