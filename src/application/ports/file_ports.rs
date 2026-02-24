use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

use crate::application::dtos::file_dto::FileDto;
use crate::common::errors::DomainError;

// ─────────────────────────────────────────────────────
// Upload port
// ─────────────────────────────────────────────────────

/// Primary port for file upload operations.
///
/// All upload paths converge on streaming-to-disk:
/// - Normal uploads: handler spools multipart to temp file → `upload_file_streaming`
/// - WebDAV PUT: small in-memory buffer → `upload_file`
/// - Chunked uploads: chunks already on disk → `upload_file_from_path`
#[async_trait]
pub trait FileUploadUseCase: Send + Sync + 'static {
    /// Upload from a temp file already on disk (true streaming, ~64 KB RAM).
    ///
    /// When `pre_computed_hash` is `Some`, the blob store skips the hash
    /// re-read — the handler already computed it during the multipart spool.
    async fn upload_file_streaming(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        temp_path: &Path,
        size: u64,
        pre_computed_hash: Option<String>,
    ) -> Result<FileDto, DomainError>;

    /// Upload from in-memory bytes (for small payloads: WebDAV, empty files).
    ///
    /// Only used for WebDAV PUT and empty files where the content is already
    /// buffered by the protocol handler. For normal uploads, prefer
    /// `upload_file_streaming`.
    async fn upload_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<FileDto, DomainError>;

    /// Upload from a file already assembled on disk (chunked uploads).
    ///
    /// Same as `upload_file_streaming` but with a separate name for clarity.
    async fn upload_file_from_path(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        file_path: &Path,
        pre_computed_hash: Option<String>,
    ) -> Result<FileDto, DomainError>;

    /// Creates a new file at the specified path (for WebDAV)
    async fn create_file(
        &self,
        parent_path: &str,
        filename: &str,
        content: &[u8],
        content_type: &str,
    ) -> Result<FileDto, DomainError>;

    /// Updates the content of an existing file (for WebDAV)
    async fn update_file(&self, path: &str, content: &[u8]) -> Result<(), DomainError>;

    /// Streaming update — spools body to a temp file with incremental hash,
    /// then atomically replaces the file content via dedup store.
    ///
    /// Peak RAM: ~256 KB regardless of file size.
    /// Used by WebDAV PUT for large files.
    async fn update_file_streaming(
        &self,
        path: &str,
        temp_path: &Path,
        size: u64,
        content_type: &str,
        pre_computed_hash: Option<String>,
    ) -> Result<(), DomainError>;
}

// ─────────────────────────────────────────────────────
// Retrieval / download port
// ─────────────────────────────────────────────────────

/// Optimized file content returned by the retrieval service.
///
/// The handler only needs to map each variant to the appropriate HTTP
/// response; all caching / transcoding / mmap decisions happen in the
/// application layer.
pub enum OptimizedFileContent {
    /// Small-file content (possibly transcoded / compressed) already in RAM.
    Bytes {
        data: Bytes,
        mime_type: String,
        was_transcoded: bool,
    },
    /// Memory-mapped file (10–100 MB).
    Mmap(Bytes),
    /// Streaming download for very large files (≥100 MB).
    Stream(Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>),
}

/// Primary port for file retrieval operations
#[async_trait]
pub trait FileRetrievalUseCase: Send + Sync + 'static {
    /// Gets a file by its ID
    async fn get_file(&self, id: &str) -> Result<FileDto, DomainError>;

    /// Gets a file by its path (for WebDAV)
    async fn get_file_by_path(&self, path: &str) -> Result<FileDto, DomainError>;

    /// Lists files in a folder
    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<FileDto>, DomainError>;

    /// Gets file content as a stream (for large files)
    async fn get_file_stream(
        &self,
        id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError>;

    /// Optimized multi-tier download.
    ///
    /// Internalises: write-behind lookup → content-cache → WebP transcode →
    /// mmap → streaming, returning an `OptimizedFileContent` variant so the
    /// handler only builds the HTTP response.
    async fn get_file_optimized(
        &self,
        id: &str,
        accept_webp: bool,
        prefer_original: bool,
    ) -> Result<(FileDto, OptimizedFileContent), DomainError>;

    /// Like `get_file_optimized` but accepts an already-fetched `FileDto`,
    /// avoiding a redundant metadata query when the handler already has it.
    async fn get_file_optimized_preloaded(
        &self,
        id: &str,
        file_dto: FileDto,
        accept_webp: bool,
        prefer_original: bool,
    ) -> Result<(FileDto, OptimizedFileContent), DomainError> {
        // Default: ignore pre-fetched meta, re-fetch everything.
        let _ = file_dto;
        self.get_file_optimized(id, accept_webp, prefer_original)
            .await
    }

    /// Range-based streaming for HTTP Range Requests (video seek, resumable DL).
    async fn get_file_range_stream(
        &self,
        id: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError>;

    /// Lists every file in the subtree rooted at `folder_id`.
    ///
    /// Default: falls back to `list_files(Some(folder_id))` (one level).
    async fn list_files_in_subtree(
        &self,
        folder_id: &str,
    ) -> Result<Vec<FileDto>, DomainError> {
        self.list_files(Some(folder_id)).await
    }

    /// Lists files in a folder with LIMIT/OFFSET pagination.
    ///
    /// Used by streaming WebDAV PROPFIND to avoid loading all files at once.
    /// Default: falls back to `list_files` (loads all, then slices in memory).
    async fn list_files_batch(
        &self,
        folder_id: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<FileDto>, DomainError> {
        let all = self.list_files(folder_id).await?;
        Ok(all.into_iter().skip(offset as usize).take(limit as usize).collect())
    }
}

// ─────────────────────────────────────────────────────
// Management port (delete, move)
// ─────────────────────────────────────────────────────

/// Primary port for file management operations
#[async_trait]
pub trait FileManagementUseCase: Send + Sync + 'static {
    /// Moves a file to another folder
    async fn move_file(
        &self,
        file_id: &str,
        folder_id: Option<String>,
    ) -> Result<FileDto, DomainError>;

    /// Copies a file to another folder (zero-copy with dedup).
    async fn copy_file(
        &self,
        file_id: &str,
        target_folder_id: Option<String>,
    ) -> Result<FileDto, DomainError>;

    /// Renames a file
    async fn rename_file(&self, file_id: &str, new_name: &str) -> Result<FileDto, DomainError>;

    /// Deletes a file
    async fn delete_file(&self, id: &str) -> Result<(), DomainError>;

    /// Smart delete: trash-first with dedup reference cleanup.
    ///
    /// 1. Tries to move to trash (soft delete).
    /// 2. Falls back to permanent delete if trash unavailable/failed.
    /// 3. Decrements the dedup reference count for the content hash.
    ///
    /// Returns `Ok(true)` when trashed, `Ok(false)` when permanently deleted.
    async fn delete_with_cleanup(&self, id: &str, user_id: &str) -> Result<bool, DomainError>;
}

/// Factory for creating file use case implementations
pub trait FileUseCaseFactory: Send + Sync + 'static {
    fn create_file_upload_use_case(&self) -> Arc<dyn FileUploadUseCase>;
    fn create_file_retrieval_use_case(&self) -> Arc<dyn FileRetrievalUseCase>;
    fn create_file_management_use_case(&self) -> Arc<dyn FileManagementUseCase>;
}
