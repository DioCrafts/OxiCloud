//! Puerto de persistencia del dominio para la entidad File.
//!
//! Define el contrato que cualquier implementación de almacenamiento de archivos
//! debe cumplir. Este trait vive en el dominio porque File es una entidad core
//! del sistema y sus contratos de persistencia pertenecen a la capa de dominio,
//! siguiendo los principios de Clean/Hexagonal Architecture.
//!
//! Las implementaciones concretas (filesystem, PostgreSQL, S3, etc.) viven en
//! la capa de infraestructura.

use std::path::PathBuf;
use std::pin::Pin;

use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

use crate::domain::entities::file::File;
use crate::domain::services::path_service::StoragePath;
use crate::common::errors::DomainError;

// ─────────────────────────────────────────────────────
// FileReadRepository — operaciones de lectura/consulta
// ─────────────────────────────────────────────────────

/// Puerto del dominio para **lectura** de archivos.
///
/// Encapsula toda operación que consulta estado sin modificarlo:
/// obtener, listar, contenido, stream, mmap, rango, resolución de rutas.
#[async_trait]
pub trait FileReadRepository: Send + Sync + 'static {
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
// FileWriteRepository — operaciones de escritura/mutación
// ─────────────────────────────────────────────────────

/// Puerto del dominio para **escritura** de archivos.
///
/// Cubre: upload (buffered + streaming), move, delete, update,
/// y el registro diferido para write-behind cache.
#[async_trait]
pub trait FileWriteRepository: Send + Sync + 'static {
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
        stream: Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>,
    ) -> Result<File, DomainError>;

    /// Mueve un archivo a otra carpeta.
    async fn move_file(
        &self,
        file_id: &str,
        target_folder_id: Option<String>,
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
// FileRepository — supertrait unificado
// ─────────────────────────────────────────────────────

/// Puerto unificado para persistencia de archivos.
///
/// Es un supertrait de `FileReadRepository + FileWriteRepository`.
/// Cualquier tipo que implemente ambos ports obtiene `FileRepository`
/// automáticamente vía blanket impl.
pub trait FileRepository: FileReadRepository + FileWriteRepository {}

/// Blanket implementation: cualquier tipo que implemente ambos ports
/// es automáticamente un FileRepository.
impl<T: FileReadRepository + FileWriteRepository> FileRepository for T {}
