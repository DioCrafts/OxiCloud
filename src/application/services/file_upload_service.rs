use std::sync::Arc;
use std::pin::Pin;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

use crate::application::dtos::file_dto::FileDto;
use crate::application::ports::file_ports::{FileUploadUseCase, UploadStrategy};
use crate::application::ports::storage_ports::{FileWritePort, FileReadPort};
use crate::application::ports::cache_ports::WriteBehindCachePort;
use crate::application::ports::dedup_ports::DedupPort;
use crate::common::errors::DomainError;
use tracing::{debug, info, warn};

/// Threshold for using streaming upload (files >= 1MB use streaming)
const STREAMING_UPLOAD_THRESHOLD: usize = 1 * 1024 * 1024;
/// Threshold for write-behind cache (files < 256KB get instant response)
const WRITE_BEHIND_THRESHOLD: usize = 256 * 1024;

/// Helper function to extract username from folder path string
fn extract_username_from_path(path: &str) -> Option<String> {
    // Support both new ("My Folder - ") and legacy ("Mi Carpeta - ") prefixes
    let prefix = if path.contains("My Folder - ") {
        "My Folder - "
    } else if path.contains("Mi Carpeta - ") {
        "Mi Carpeta - "
    } else {
        return None;
    };
    let parts: Vec<&str> = path.split(prefix).collect();
    if parts.len() <= 1 {
        return None;
    }
    Some(parts[1].trim().to_string())
}

/// Service for file upload operations
///
/// Encapsulates the three-tier upload strategy:
/// 1. **Write-Behind** (<256 KB): store in RAM, respond instantly, flush async.
/// 2. **Buffered** (256 KB – 1 MB): collect bytes, write, respond.
/// 3. **Streaming** (≥1 MB): pipe chunks to disk with constant memory.
///
/// Also runs deduplication so duplicate content is never stored twice.
pub struct FileUploadService {
    /// Write port — handles save, streaming, deferred registration
    file_write: Arc<dyn FileWritePort>,
    /// Read port — needed for WebDAV create_file / update_file
    file_read: Option<Arc<dyn FileReadPort>>,
    /// Optional write-behind cache for instant uploads
    write_behind: Option<Arc<dyn WriteBehindCachePort>>,
    /// Optional dedup service for content-addressable storage
    dedup: Option<Arc<dyn DedupPort>>,
    /// Optional storage usage tracking
    storage_usage_service: Option<Arc<dyn crate::application::ports::storage_ports::StorageUsagePort>>,
}

impl FileUploadService {
    /// Backward-compatible constructor (no write-behind, no dedup).
    pub fn new(file_repository: Arc<dyn FileWritePort>) -> Self {
        Self {
            file_write: file_repository,
            file_read: None,
            write_behind: None,
            dedup: None,
            storage_usage_service: None,
        }
    }

    /// Full constructor with all infrastructure ports.
    pub fn new_full(
        file_write: Arc<dyn FileWritePort>,
        file_read: Arc<dyn FileReadPort>,
        write_behind: Arc<dyn WriteBehindCachePort>,
        dedup: Arc<dyn DedupPort>,
    ) -> Self {
        Self {
            file_write,
            file_read: Some(file_read),
            write_behind: Some(write_behind),
            dedup: Some(dedup),
            storage_usage_service: None,
        }
    }

    /// Configures the storage usage service
    pub fn with_storage_usage_service(
        mut self,
        storage_usage_service: Arc<dyn crate::application::ports::storage_ports::StorageUsagePort>,
    ) -> Self {
        self.storage_usage_service = Some(storage_usage_service);
        self
    }

    // ── private helpers ──────────────────────────────────────────

    /// Run dedup tracking (non-fatal on failure).
    async fn run_dedup(&self, data: &[u8], content_type: &str) {
        let Some(dedup) = &self.dedup else { return };
        match dedup.store_bytes(data, Some(content_type.to_string())).await {
            Ok(result) => {
                if result.was_deduplicated() {
                    info!(
                        "🔗 DEDUP: content already exists (hash: {}, saved {} bytes)",
                        &result.hash()[..12],
                        result.size()
                    );
                } else {
                    info!("💾 DEDUP: new content stored (hash: {})", &result.hash()[..12]);
                }
            }
            Err(e) => {
                warn!("⚠️ DEDUP: Failed to store in blob store: {}", e);
            }
        }
    }

    /// Optionally update storage usage after a successful upload.
    fn maybe_update_storage_usage(&self, file: &FileDto) {
        if let Some(storage_service) = &self.storage_usage_service {
            // Extract username from the file's own path (contains folder structure)
            let file_path = file.path.clone();
            if let Some(username) = extract_username_from_path(&file_path) {
                let service_clone = Arc::clone(storage_service);
                tokio::spawn(async move {
                    match service_clone.update_user_storage_usage(&username).await {
                        Ok(usage) => debug!("Updated storage usage for user {} to {} bytes", username, usage),
                        Err(e) => warn!("Failed to update storage usage for {}: {}", username, e),
                    }
                });
            }
        }
    }
}

#[async_trait]
impl FileUploadUseCase for FileUploadService {
    /// Simple byte-based upload (backward compatible).
    async fn upload_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<FileDto, DomainError> {
        let file = self.file_write.save_file(name, folder_id, content_type, content).await?;
        let dto = FileDto::from(file);
        self.maybe_update_storage_usage(&dto);
        Ok(dto)
    }

    /// Smart three-tier upload with write-behind cache and dedup.
    async fn smart_upload(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        chunks: Vec<Bytes>,
        total_size: usize,
    ) -> Result<(FileDto, UploadStrategy), DomainError> {
        use futures::stream;

        // ─── Dedup (runs for all tiers) ──────────────────────
        {
            let dedup_data: Vec<u8> = {
                let mut combined = Vec::with_capacity(total_size);
                for chunk in &chunks {
                    combined.extend_from_slice(chunk);
                }
                combined
            };
            self.run_dedup(&dedup_data, &content_type).await;
        }

        // ─── TIER 1: Write-Behind (<256 KB) ──────────────────
        if total_size < WRITE_BEHIND_THRESHOLD {
            if let Some(wb) = &self.write_behind {
                if wb.is_eligible_size(total_size) {
                    let data: Bytes = if chunks.len() == 1 {
                        chunks.into_iter().next().unwrap()
                    } else {
                        let mut combined = Vec::with_capacity(total_size);
                        for chunk in chunks {
                            combined.extend_from_slice(&chunk);
                        }
                        combined.into()
                    };

                    let (file, target_path) = self
                        .file_write
                        .register_file_deferred(name.clone(), folder_id, content_type, total_size as u64)
                        .await?;
                    let dto = FileDto::from(file);

                    if let Err(e) = wb.put_pending(dto.id.clone(), data, target_path).await {
                        return Err(DomainError::internal_error("file", format!(
                            "Write-behind cache failed: {}",
                            e
                        )));
                    }

                    info!("⚡ WRITE-BEHIND UPLOAD: {} (ID: {}, ~0ms latency)", name, dto.id);
                    self.maybe_update_storage_usage(&dto);
                    return Ok((dto, UploadStrategy::WriteBehind));
                }
            }
        }

        // ─── TIER 2: Streaming (≥1 MB) ──────────────────────
        if total_size >= STREAMING_UPLOAD_THRESHOLD {
            let chunk_stream = stream::iter(chunks.into_iter().map(|c| Ok::<_, std::io::Error>(c)));
            let pinned_stream: Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>> =
                Box::pin(chunk_stream);

            let file = self
                .file_write
                .save_file_from_stream(name.clone(), folder_id, content_type, pinned_stream)
                .await?;
            let dto = FileDto::from(file);
            info!(
                "✅ STREAMING UPLOAD: {} ({} MB, ID: {})",
                name,
                total_size / (1024 * 1024),
                dto.id
            );
            self.maybe_update_storage_usage(&dto);
            return Ok((dto, UploadStrategy::Streaming));
        }

        // ─── TIER 3: Buffered (256 KB – 1 MB) ───────────────
        let data = if chunks.len() == 1 {
            chunks.into_iter().next().unwrap().to_vec()
        } else {
            let mut combined = Vec::with_capacity(total_size);
            for chunk in chunks {
                combined.extend_from_slice(&chunk);
            }
            combined
        };

        let file = self
            .file_write
            .save_file(name.clone(), folder_id, content_type, data)
            .await?;
        let dto = FileDto::from(file);
        info!("✅ BUFFERED UPLOAD: {} (ID: {})", name, dto.id);
        self.maybe_update_storage_usage(&dto);
        Ok((dto, UploadStrategy::Buffered))
    }

    /// Creates a file at a specific path (for WebDAV PUT on new resource).
    async fn create_file(
        &self,
        parent_path: &str,
        filename: &str,
        content: &[u8],
        content_type: &str,
    ) -> Result<FileDto, DomainError> {
        // Resolve parent folder ID from path
        let parent_id = if !parent_path.is_empty() {
            if let Some(file_read) = &self.file_read {
                match file_read.get_parent_folder_id(parent_path).await {
                    Ok(id) => Some(id),
                    Err(_) => None, // If parent doesn't exist, use root
                }
            } else {
                None
            }
        } else {
            None
        };

        let file = self
            .file_write
            .save_file(
                filename.to_string(),
                parent_id,
                content_type.to_string(),
                content.to_vec(),
            )
            .await?;
        let dto = FileDto::from(file);
        self.maybe_update_storage_usage(&dto);
        Ok(dto)
    }

    /// Updates an existing file's content, or creates it if not found (for WebDAV PUT).
    async fn update_file(&self, path: &str, content: &[u8]) -> Result<(), DomainError> {
        let path_normalized = path.trim_start_matches('/').trim_end_matches('/');

        // Try to find the existing file by path
        if let Some(file_read) = &self.file_read {
            let all_files = file_read.list_files(None).await?;
            for file in &all_files {
                let dto = FileDto::from(file.clone());
                let dto_path = dto.path.trim_start_matches('/').trim_end_matches('/');
                if dto_path == path_normalized
                    || dto_path.ends_with(&format!("/{}", path_normalized))
                    || path_normalized.ends_with(&format!("/{}", dto_path))
                {
                    // Found it — update in place
                    self.file_write
                        .update_file_content(file.id(), content.to_vec())
                        .await?;
                    return Ok(());
                }
            }
        }

        // File not found — create it
        let (parent_path, filename) = if let Some(idx) = path_normalized.rfind('/') {
            (&path_normalized[..idx], &path_normalized[idx + 1..])
        } else {
            ("", path_normalized)
        };
        self.create_file(parent_path, filename, content, "application/octet-stream")
            .await?;
        Ok(())
    }
}