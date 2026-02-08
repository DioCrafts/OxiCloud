use std::path::{Path, PathBuf};
use std::sync::Arc;
use async_trait::async_trait;
use thiserror::Error;

use crate::domain::entities::folder::Folder;
use crate::domain::services::path_service::StoragePath;
use crate::application::ports::outbound::{IdMappingPort, StoragePort, FolderStoragePort};

/// Errores específicos del mediador de almacenamiento
#[derive(Debug, Error)]
pub enum StorageMediatorError {
    #[error("Entidad no encontrada: {0}")]
    NotFound(String),
    
    #[error("Entidad ya existe: {0}")]
    AlreadyExists(String),
    
    #[error("Ruta inválida: {0}")]
    InvalidPath(String),
    
    #[error("Error de acceso: {0}")]
    AccessError(String),
    
    #[error("Error interno: {0}")]
    InternalError(String),
    
    #[error("Error de dominio: {0}")]
    DomainError(#[from] crate::common::errors::DomainError),
}

/// Tipo de resultado para las operaciones del mediador
pub type StorageMediatorResult<T> = Result<T, StorageMediatorError>;

/// Interfaz del servicio mediador entre repositorios de archivos y carpetas
#[async_trait]
pub trait StorageMediator: Send + Sync + 'static {
    /// Obtiene la ruta de una carpeta por su ID
    async fn get_folder_path(&self, folder_id: &str) -> StorageMediatorResult<PathBuf>;
    
    /// Obtiene la ruta de dominio de una carpeta por su ID
    async fn get_folder_storage_path(&self, folder_id: &str) -> StorageMediatorResult<StoragePath>;
    
    /// Obtiene todos los detalles de una carpeta por su ID
    async fn get_folder(&self, folder_id: &str) -> StorageMediatorResult<Folder>;
    
    /// Verifica si existe un archivo en una ruta específica
    async fn file_exists_at_path(&self, path: &Path) -> StorageMediatorResult<bool>;
    
    /// Verifica si existe un archivo en una ruta de dominio específica
    async fn file_exists_at_storage_path(&self, storage_path: &StoragePath) -> StorageMediatorResult<bool>;
    
    /// Verifica si existe una carpeta en una ruta específica
    async fn folder_exists_at_path(&self, path: &Path) -> StorageMediatorResult<bool>;
    
    /// Verifica si existe una carpeta en una ruta de dominio específica
    async fn folder_exists_at_storage_path(&self, storage_path: &StoragePath) -> StorageMediatorResult<bool>;
    
    /// Resuelve una ruta relativa a absoluta (legacy)
    fn resolve_path(&self, relative_path: &Path) -> PathBuf;
    
    /// Resuelve una ruta de dominio a una ruta física absoluta
    fn resolve_storage_path(&self, storage_path: &StoragePath) -> PathBuf;
    
    /// Crea un directorio si no existe (legacy)
    async fn ensure_directory(&self, path: &Path) -> StorageMediatorResult<()>;
    
    /// Crea un directorio si no existe
    async fn ensure_storage_directory(&self, storage_path: &StoragePath) -> StorageMediatorResult<()>;
}

/// Implementación concreta del mediador de almacenamiento
pub struct FileSystemStorageMediator {
    pub folder_storage_port: Arc<dyn FolderStoragePort>,
    pub path_service: Arc<dyn StoragePort>,
    pub id_mapping: Arc<dyn IdMappingPort>,
}

impl FileSystemStorageMediator {
    pub fn new(folder_storage_port: Arc<dyn FolderStoragePort>, path_service: Arc<dyn StoragePort>, id_mapping: Arc<dyn IdMappingPort>) -> Self {
        Self { folder_storage_port, path_service, id_mapping }
    }
    
    /// Creates a stub implementation for initialization bootstrapping
    pub fn new_stub() -> StubStorageMediator {
        StubStorageMediator::new()
    }
}

/// Stub implementation for initialization dependency issues
/// This is a minimal stub that doesn't require any infrastructure dependencies
pub struct StubStorageMediator;

impl StubStorageMediator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StubStorageMediator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StorageMediator for StubStorageMediator {
    async fn get_folder_path(&self, _folder_id: &str) -> StorageMediatorResult<PathBuf> {
        // Return a stub path
        Ok(PathBuf::from("/tmp"))
    }
    
    async fn get_folder_storage_path(&self, _folder_id: &str) -> StorageMediatorResult<StoragePath> {
        // Return a stub storage path
        Ok(StoragePath::root())
    }
    
    async fn get_folder(&self, _folder_id: &str) -> StorageMediatorResult<Folder> {
        // This is a stub that should never be called during initialization
        Err(StorageMediatorError::NotFound("Stub not implemented".to_string()))
    }
    
    async fn file_exists_at_path(&self, _path: &Path) -> StorageMediatorResult<bool> {
        Ok(false)
    }
    
    async fn file_exists_at_storage_path(&self, _storage_path: &StoragePath) -> StorageMediatorResult<bool> {
        Ok(false)
    }
    
    async fn folder_exists_at_path(&self, _path: &Path) -> StorageMediatorResult<bool> {
        Ok(false)
    }
    
    async fn folder_exists_at_storage_path(&self, _storage_path: &StoragePath) -> StorageMediatorResult<bool> {
        Ok(false)
    }
    
    fn resolve_path(&self, _relative_path: &Path) -> PathBuf {
        PathBuf::from("/tmp")
    }
    
    fn resolve_storage_path(&self, _storage_path: &StoragePath) -> PathBuf {
        PathBuf::from("/tmp")
    }
    
    async fn ensure_directory(&self, _path: &Path) -> StorageMediatorResult<()> {
        Ok(())
    }
    
    async fn ensure_storage_directory(&self, _storage_path: &StoragePath) -> StorageMediatorResult<()> {
        Ok(())
    }
}

#[async_trait]
impl StorageMediator for FileSystemStorageMediator {
    async fn get_folder_path(&self, folder_id: &str) -> StorageMediatorResult<PathBuf> {
        let folder = self.folder_storage_port.get_folder(folder_id).await
            .map_err(StorageMediatorError::from)?;
        
        // Need to get the path from folder ID
        let storage_path = self.id_mapping.get_path_by_id(folder.id()).await
            .map_err(StorageMediatorError::from)?;
        
        // Convert StoragePath to PathBuf
        let path_buf = self.path_service.resolve_path(&storage_path);
        Ok(path_buf)
    }
    
    async fn get_folder_storage_path(&self, folder_id: &str) -> StorageMediatorResult<StoragePath> {
        let folder = self.folder_storage_port.get_folder(folder_id).await
            .map_err(StorageMediatorError::from)?;
        
        // Get path by folder ID - will already be a StoragePath
        let storage_path = self.id_mapping.get_path_by_id(folder.id()).await
            .map_err(StorageMediatorError::from)?;
        
        Ok(storage_path)
    }
    
    async fn get_folder(&self, folder_id: &str) -> StorageMediatorResult<Folder> {
        let folder = self.folder_storage_port.get_folder(folder_id).await
            .map_err(StorageMediatorError::from)?;
        
        Ok(folder)
    }
    
    async fn file_exists_at_path(&self, path: &Path) -> StorageMediatorResult<bool> {
        let abs_path = self.resolve_path(path);
        
        // Verificar si existe como archivo (no como directorio)
        let exists = abs_path.exists() && abs_path.is_file();
        
        Ok(exists)
    }
    
    async fn file_exists_at_storage_path(&self, storage_path: &StoragePath) -> StorageMediatorResult<bool> {
        let abs_path = self.resolve_storage_path(storage_path);
        
        // Verificar si existe como archivo (no como directorio)
        let exists = abs_path.exists() && abs_path.is_file();
        
        Ok(exists)
    }
    
    async fn folder_exists_at_path(&self, path: &Path) -> StorageMediatorResult<bool> {
        let abs_path = self.resolve_path(path);
        
        // Verificar si existe como directorio
        let exists = abs_path.exists() && abs_path.is_dir();
        
        Ok(exists)
    }
    
    async fn folder_exists_at_storage_path(&self, storage_path: &StoragePath) -> StorageMediatorResult<bool> {
        let abs_path = self.resolve_storage_path(storage_path);
        
        // Verificar si existe como directorio
        let exists = abs_path.exists() && abs_path.is_dir();
        
        Ok(exists)
    }
    
    fn resolve_path(&self, relative_path: &Path) -> PathBuf {
        // Legacy method using PathBuf
        let path_str = relative_path.to_string_lossy().to_string();
        let storage_path = StoragePath::from_string(&path_str);
        self.path_service.resolve_path(&storage_path)
    }
    
    fn resolve_storage_path(&self, storage_path: &StoragePath) -> PathBuf {
        self.path_service.resolve_path(storage_path)
    }
    
    async fn ensure_directory(&self, path: &Path) -> StorageMediatorResult<()> {
        let abs_path = self.resolve_path(path);
        
        // Crear directorios si no existen
        if !abs_path.exists() {
            tokio::fs::create_dir_all(&abs_path).await
                .map_err(|e| StorageMediatorError::AccessError(format!("No se pudo crear el directorio: {}", e)))?;
        } else if !abs_path.is_dir() {
            return Err(StorageMediatorError::InvalidPath(
                format!("La ruta existe pero no es un directorio: {}", abs_path.display())
            ));
        }
        
        Ok(())
    }
    
    async fn ensure_storage_directory(&self, storage_path: &StoragePath) -> StorageMediatorResult<()> {
        let abs_path = self.resolve_storage_path(storage_path);
        
        // Crear directorios si no existen
        if !abs_path.exists() {
            tokio::fs::create_dir_all(&abs_path).await
                .map_err(|e| StorageMediatorError::AccessError(format!("No se pudo crear el directorio: {}", e)))?;
        } else if !abs_path.is_dir() {
            return Err(StorageMediatorError::InvalidPath(
                format!("La ruta existe pero no es un directorio: {}", abs_path.display())
            ));
        }
        
        Ok(())
    }
}