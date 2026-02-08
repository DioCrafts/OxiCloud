use std::sync::Arc;
use std::pin::Pin;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

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

/// Puerto primario para operaciones de subida de archivos
#[async_trait]
pub trait FileUploadUseCase: Send + Sync + 'static {
    /// Sube un nuevo archivo desde bytes
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

    /// Crea un nuevo archivo en la ruta especificada (para WebDAV)
    async fn create_file(&self, parent_path: &str, filename: &str, content: &[u8], content_type: &str) -> Result<FileDto, DomainError>;

    /// Actualiza el contenido de un archivo existente (para WebDAV)
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

/// Puerto primario para operaciones de recuperación de archivos
#[async_trait]
pub trait FileRetrievalUseCase: Send + Sync + 'static {
    /// Obtiene un archivo por su ID
    async fn get_file(&self, id: &str) -> Result<FileDto, DomainError>;
    
    /// Obtiene un archivo por su ruta (para WebDAV)
    async fn get_file_by_path(&self, path: &str) -> Result<FileDto, DomainError>;
    
    /// Lista archivos en una carpeta
    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<FileDto>, DomainError>;
    
    /// Obtiene contenido de archivo como bytes (para archivos pequeños)
    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError>;
    
    /// Obtiene contenido de archivo como stream (para archivos grandes)
    async fn get_file_stream(&self, id: &str) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError>;

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

/// Puerto primario para operaciones de gestión de archivos
#[async_trait]
pub trait FileManagementUseCase: Send + Sync + 'static {
    /// Mueve un archivo a otra carpeta
    async fn move_file(&self, file_id: &str, folder_id: Option<String>) -> Result<FileDto, DomainError>;
    
    /// Renombra un archivo
    async fn rename_file(&self, file_id: &str, new_name: &str) -> Result<FileDto, DomainError>;
    
    /// Elimina un archivo
    async fn delete_file(&self, id: &str) -> Result<(), DomainError>;

    /// Smart delete: trash-first with dedup reference cleanup.
    ///
    /// 1. Tries to move to trash (soft delete).
    /// 2. Falls back to permanent delete if trash unavailable/failed.
    /// 3. Decrements the dedup reference count for the content hash.
    ///
    /// Returns `Ok(true)` when trashed, `Ok(false)` when permanently deleted.
    async fn delete_with_cleanup(
        &self,
        id: &str,
        user_id: &str,
    ) -> Result<bool, DomainError>;
}

/// Factory para crear implementaciones de casos de uso de archivos
pub trait FileUseCaseFactory: Send + Sync + 'static {
    fn create_file_upload_use_case(&self) -> Arc<dyn FileUploadUseCase>;
    fn create_file_retrieval_use_case(&self) -> Arc<dyn FileRetrievalUseCase>;
    fn create_file_management_use_case(&self) -> Arc<dyn FileManagementUseCase>;
}