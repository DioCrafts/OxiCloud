use std::path::PathBuf;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

use crate::domain::entities::file::File;
use crate::domain::entities::folder::Folder;
use crate::domain::services::path_service::StoragePath;
use crate::common::errors::DomainError;

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

/// Puerto secundario para persistencia de archivos
#[async_trait]
pub trait FileStoragePort: Send + Sync + 'static {
    /// Guarda un nuevo archivo desde bytes
    async fn save_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<File, DomainError>;
    
    /// Guarda un archivo desde stream (streaming upload)
    /// Escribe chunks directamente al disco sin acumular en memoria
    async fn save_file_from_stream(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        stream: std::pin::Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>,
    ) -> Result<File, DomainError>;
    
    /// Obtiene un archivo por su ID
    async fn get_file(&self, id: &str) -> Result<File, DomainError>;
    
    /// Lista archivos en una carpeta
    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<File>, DomainError>;
    
    /// Elimina un archivo
    async fn delete_file(&self, id: &str) -> Result<(), DomainError>;
    
    /// Obtiene contenido de archivo como bytes
    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError>;
    
    /// Obtiene contenido de archivo como stream
    async fn get_file_stream(&self, id: &str) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError>;
    
    /// Obtiene un rango de contenido como stream (para HTTP Range Requests)
    async fn get_file_range_stream(
        &self, 
        id: &str, 
        start: u64, 
        end: Option<u64>
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError>;
    
    /// Memory-maps archivo para acceso zero-copy (ideal para 10-100MB)
    async fn get_file_mmap(&self, id: &str) -> Result<Bytes, DomainError>;
    
    /// Mueve un archivo a otra carpeta
    async fn move_file(&self, file_id: &str, target_folder_id: Option<String>) -> Result<File, DomainError>;
    
    /// Obtiene la ruta de almacenamiento de un archivo
    async fn get_file_path(&self, id: &str) -> Result<StoragePath, DomainError>;
    
    /// Obtiene el ID de la carpeta padre para una ruta dada (necesario para WebDAV)
    async fn get_parent_folder_id(&self, path: &str) -> Result<String, DomainError>;
    
    /// Actualiza el contenido de un archivo existente
    async fn update_file_content(&self, file_id: &str, content: Vec<u8>) -> Result<(), DomainError>;
    
    /// Registra metadatos de archivo SIN escribir contenido a disco (write-behind)
    /// 
    /// Este método:
    /// 1. Genera ID único para el archivo
    /// 2. Calcula la ruta de destino
    /// 3. Registra el mapping ID->path
    /// 4. Devuelve File entity con path real (pero archivo no existe aún)
    /// 
    /// El contenido se escribe posteriormente via WriteBehindCache
    /// Beneficio: Respuesta ~0ms para uploads pequeños
    async fn register_file_deferred(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        size: u64,
    ) -> Result<(File, PathBuf), DomainError>;
}

/// Puerto secundario para persistencia de carpetas
#[async_trait]
pub trait FolderStoragePort: Send + Sync + 'static {
    /// Crea una nueva carpeta
    async fn create_folder(&self, name: String, parent_id: Option<String>) -> Result<Folder, DomainError>;
    
    /// Obtiene una carpeta por su ID
    async fn get_folder(&self, id: &str) -> Result<Folder, DomainError>;
    
    /// Obtiene una carpeta por su ruta
    async fn get_folder_by_path(&self, storage_path: &StoragePath) -> Result<Folder, DomainError>;
    
    /// Lista carpetas dentro de una carpeta padre
    async fn list_folders(&self, parent_id: Option<&str>) -> Result<Vec<Folder>, DomainError>;
    
    /// Lista carpetas con paginación
    async fn list_folders_paginated(
        &self, 
        parent_id: Option<&str>,
        offset: usize,
        limit: usize,
        include_total: bool
    ) -> Result<(Vec<Folder>, Option<usize>), DomainError>;
    
    /// Renombra una carpeta
    async fn rename_folder(&self, id: &str, new_name: String) -> Result<Folder, DomainError>;
    
    /// Mueve una carpeta a otro padre
    async fn move_folder(&self, id: &str, new_parent_id: Option<&str>) -> Result<Folder, DomainError>;
    
    /// Elimina una carpeta
    async fn delete_folder(&self, id: &str) -> Result<(), DomainError>;
    
    /// Verifica si existe una carpeta en la ruta dada
    async fn folder_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError>;
    
    /// Obtiene la ruta de una carpeta
    async fn get_folder_path(&self, id: &str) -> Result<StoragePath, DomainError>;
}

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