use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;

use crate::application::dtos::file_dto::FileDto;
use crate::common::errors::DomainError;

// ─────────────────────────────────────────────────────
// Upload port
// ─────────────────────────────────────────────────────

/// Strategy chosen by the upload service based on file size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UploadStrategy {
    /// Instant (<256KB): write-behind cache, ~0ms latency
    WriteBehind,
    /// Buffered (256KB–1MB): full bytes in memory then write
    Buffered,
    /// Streaming (≥1MB): pipe chunks directly to disk
    Streaming,
}

/// Primary port for file upload operations
#[async_trait]
pub trait FileUploadUseCase: Send + Sync + 'static {
    /// Uploads a new file from bytes
    async fn upload_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<FileDto, DomainError>;

    /// Smart upload: picks the best strategy (write-behind / buffered / streaming)
    /// and handles dedup automatically.
    ///
    /// Returns `(FileDto, UploadStrategy)` so the handler can log the chosen tier.
    async fn smart_upload(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        chunks: Vec<Bytes>,
        total_size: usize,
    ) -> Result<(FileDto, UploadStrategy), DomainError>;

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

    /// Gets file content as bytes (for small files)
    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError>;

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

    /// Range-based streaming for HTTP Range Requests (video seek, resumable DL).
    async fn get_file_range_stream(
        &self,
        id: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError>;
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
