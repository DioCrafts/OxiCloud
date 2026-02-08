use std::path::PathBuf;
use async_trait::async_trait;

use crate::domain::services::path_service::StoragePath;
use crate::common::errors::DomainError;

// Re-export domain repository traits for backward compatibility
pub use crate::domain::repositories::folder_repository::FolderRepository;

use super::storage_ports::{FileReadPort, FileWritePort};

/// Puerto secundario para operaciones de almacenamiento
#[async_trait]
pub trait StoragePort: Send + Sync + 'static {
    /// Resuelve una ruta de dominio a una ruta física
    fn resolve_path(&self, storage_path: &StoragePath) -> PathBuf;
    
    /// Crea directorios si no existen
    async fn ensure_directory(&self, storage_path: &StoragePath) -> Result<(), DomainError>;
    
    /// Verifica si existe un archivo en la ruta dada
    async fn file_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError>;
    
    /// Verifica si existe un directorio en la ruta dada
    async fn directory_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError>;
}

/// Puerto unificado para persistencia de archivos (backward-compatible).
///
/// Ahora es un **supertrait** de `FileReadPort + FileWritePort`.
/// Cualquier tipo que implemente ambos ports obtiene `FileStoragePort`
/// automáticamente via blanket impl. Esto permite migrar consumidores
/// gradualmente a los ports granulares mientras los existentes siguen
/// funcionando sin cambios.
pub trait FileStoragePort: FileReadPort + FileWritePort {}

/// Blanket implementation: cualquier tipo que implemente ambos ports
/// es automáticamente un FileStoragePort.
impl<T: FileReadPort + FileWritePort> FileStoragePort for T {}

/// Puerto secundario para persistencia de carpetas (application layer).
///
/// Tiene la misma firma que `FolderRepository` del dominio.
/// Las implementaciones concretas deben implementar `FolderRepository`,
/// obteniendo `FolderStoragePort` automáticamente vía blanket impl.
pub trait FolderStoragePort: FolderRepository {}

/// Blanket implementation: cualquier tipo que implemente FolderRepository
/// es automáticamente un FolderStoragePort.
impl<T: FolderRepository> FolderStoragePort for T {}

/// Puerto secundario para mapeo de IDs
#[async_trait]
pub trait IdMappingPort: Send + Sync + 'static {
    /// Obtiene o crea un ID para una ruta
    async fn get_or_create_id(&self, path: &StoragePath) -> Result<String, DomainError>;
    
    /// Obtiene una ruta por su ID
    async fn get_path_by_id(&self, id: &str) -> Result<StoragePath, DomainError>;
    
    /// Actualiza la ruta para un ID existente
    async fn update_path(&self, id: &str, new_path: &StoragePath) -> Result<(), DomainError>;
    
    /// Elimina un ID del mapeo
    async fn remove_id(&self, id: &str) -> Result<(), DomainError>;
    
    /// Guarda cambios pendientes
    async fn save_changes(&self) -> Result<(), DomainError>;
    
    /// Obtiene la ruta de archivo como PathBuf
    async fn get_file_path(&self, file_id: &str) -> Result<PathBuf, DomainError> {
        let storage_path = self.get_path_by_id(file_id).await?;
        Ok(PathBuf::from(storage_path.to_string()))
    }
    
    /// Actualiza la ruta de un archivo
    async fn update_file_path(&self, file_id: &str, new_path: &PathBuf) -> Result<(), DomainError> {
        let storage_path = StoragePath::from_string(&new_path.to_string_lossy().to_string());
        self.update_path(file_id, &storage_path).await
    }
}