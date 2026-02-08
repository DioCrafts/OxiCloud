use std::path::PathBuf;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use serde_json::Value;

use crate::domain::entities::file::File;
use crate::domain::services::path_service::StoragePath;
use crate::common::errors::DomainError;

// Re-export domain repository traits for backward compatibility.
// The canonical definitions now live in domain/repositories/.
pub use crate::domain::repositories::file_repository::{FileReadRepository, FileWriteRepository, FileRepository};
pub use crate::domain::repositories::folder_repository::FolderRepository;

// ─────────────────────────────────────────────────────
// FileReadPort — application-layer alias for FileReadRepository
// ─────────────────────────────────────────────────────

/// Puerto secundario para **lectura** de archivos.
///
/// Encapsula toda operación que consulta estado sin modificarlo:
/// get, list, content, stream, mmap, range, resolución de rutas.
#[async_trait]
pub trait FileReadPort: Send + Sync + 'static {
    /// Obtiene un archivo por su ID.
    async fn get_file(&self, id: &str) -> Result<File, DomainError>;

    /// Lista archivos en una carpeta.
    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<File>, DomainError>;

    /// Obtiene contenido completo como bytes (solo archivos pequeños/medianos).
    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError>;

    /// Obtiene contenido como stream (ideal para archivos grandes).
    async fn get_file_stream(
        &self,
        id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError>;

    /// Stream de un rango de bytes (HTTP Range Requests, video seek).
    async fn get_file_range_stream(
        &self,
        id: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError>;

    /// Memory-map de archivo para acceso zero-copy (10–100 MB).
    async fn get_file_mmap(&self, id: &str) -> Result<Bytes, DomainError>;

    /// Obtiene la ruta de almacenamiento lógica de un archivo.
    async fn get_file_path(&self, id: &str) -> Result<StoragePath, DomainError>;

    /// Obtiene el ID de la carpeta padre a partir de una ruta (WebDAV).
    async fn get_parent_folder_id(&self, path: &str) -> Result<String, DomainError>;
}

// ─────────────────────────────────────────────────────
// FileWritePort — all write / mutate operations
// ─────────────────────────────────────────────────────

/// Puerto secundario para **escritura** de archivos.
///
/// Cubre: upload (buffered + streaming), move, delete, update,
/// y el registro diferido para write-behind cache.
#[async_trait]
pub trait FileWritePort: Send + Sync + 'static {
    /// Guarda un nuevo archivo desde bytes.
    async fn save_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<File, DomainError>;

    /// Upload en streaming — escribe chunks a disco sin acumular en RAM.
    async fn save_file_from_stream(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        stream: std::pin::Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>,
    ) -> Result<File, DomainError>;

    /// Mueve un archivo a otra carpeta.
    async fn move_file(
        &self,
        file_id: &str,
        target_folder_id: Option<String>,
    ) -> Result<File, DomainError>;

    /// Renombra un archivo (same folder, different name).
    async fn rename_file(
        &self,
        file_id: &str,
        new_name: &str,
    ) -> Result<File, DomainError>;

    /// Elimina un archivo.
    async fn delete_file(&self, id: &str) -> Result<(), DomainError>;

    /// Actualiza el contenido de un archivo existente.
    async fn update_file_content(&self, file_id: &str, content: Vec<u8>) -> Result<(), DomainError>;

    /// Registra metadatos de archivo SIN escribir contenido a disco (write-behind).
    ///
    /// Devuelve `(File, PathBuf)` donde `PathBuf` es la ruta destino para la
    /// escritura diferida que realizará el `WriteBehindCache`.
    async fn register_file_deferred(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        size: u64,
    ) -> Result<(File, PathBuf), DomainError>;

    // ── Trash operations ──

    /// Mueve un archivo a la papelera
    async fn move_to_trash(&self, file_id: &str) -> Result<(), DomainError>;

    /// Restaura un archivo desde la papelera a su ubicación original
    async fn restore_from_trash(&self, file_id: &str, original_path: &str) -> Result<(), DomainError>;

    /// Elimina un archivo permanentemente (usado por la papelera)
    async fn delete_file_permanently(&self, file_id: &str) -> Result<(), DomainError>;
}

// ─────────────────────────────────────────────────────
// Auxiliary ports (unchanged)
// ─────────────────────────────────────────────────────

/// Puerto secundario para resolución de rutas de archivos
#[async_trait]
pub trait FilePathResolutionPort: Send + Sync + 'static {
    /// Obtiene la ruta de almacenamiento de un archivo
    async fn get_file_path(&self, id: &str) -> Result<StoragePath, DomainError>;

    /// Resuelve una ruta de dominio a una ruta física
    fn resolve_path(&self, storage_path: &StoragePath) -> PathBuf;
}

/// Puerto secundario para verificación de existencia de archivos/directorios
#[async_trait]
pub trait StorageVerificationPort: Send + Sync + 'static {
    /// Verifica si existe un archivo en la ruta dada
    async fn file_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError>;

    /// Verifica si existe un directorio en la ruta dada
    async fn directory_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError>;
}

/// Puerto secundario para gestión de directorios
#[async_trait]
pub trait DirectoryManagementPort: Send + Sync + 'static {
    /// Crea directorios si no existen
    async fn ensure_directory(&self, storage_path: &StoragePath) -> Result<(), DomainError>;
}

/// Puerto secundario para gestión de uso de almacenamiento
#[async_trait]
pub trait StorageUsagePort: Send + Sync + 'static {
    /// Actualiza estadísticas de uso de almacenamiento para un usuario
    async fn update_user_storage_usage(&self, user_id: &str) -> Result<i64, DomainError>;

    /// Actualiza estadísticas de uso de almacenamiento para todos los usuarios
    async fn update_all_users_storage_usage(&self) -> Result<(), DomainError>;
}

/// Generic storage service interface for calendar and contact services
#[async_trait]
pub trait StorageUseCase: Send + Sync + 'static {
    /// Handle a request with the specified action and parameters
    async fn handle_request(&self, action: &str, params: Value) -> Result<Value, DomainError>;
}