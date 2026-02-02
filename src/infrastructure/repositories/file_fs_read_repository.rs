use std::path::PathBuf;
use std::sync::Arc;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

use crate::domain::entities::file::File;
use crate::application::ports::storage_ports::FileReadPort;
use crate::common::errors::DomainError;
use crate::domain::repositories::file_repository::FileRepositoryResult;
use crate::infrastructure::repositories::file_metadata_manager::{FileMetadataManager, MetadataError};
use crate::infrastructure::repositories::file_path_resolver::FilePathResolver;
use crate::domain::services::path_service::StoragePath;
use crate::infrastructure::repositories::parallel_file_processor::ParallelFileProcessor;
use crate::common::config::AppConfig;

/// Implementación de repositorio para operaciones de lectura de archivos
pub struct FileFsReadRepository {
    metadata_manager: Arc<FileMetadataManager>,
    path_resolver: Arc<FilePathResolver>,
}

impl FileFsReadRepository {
    /// Crea un nuevo repositorio de lectura de archivos
    pub fn new(
        _root_path: PathBuf,
        metadata_manager: Arc<FileMetadataManager>,
        path_resolver: Arc<FilePathResolver>,
        _config: AppConfig,
        _parallel_processor: Option<Arc<ParallelFileProcessor>>,
    ) -> Self {
        Self {
            metadata_manager,
            path_resolver,
        }
    }
    
    /// Crea un stub para pruebas
    pub fn default_stub() -> Self {
        Self {
            metadata_manager: Arc::new(FileMetadataManager::default()),
            path_resolver: Arc::new(FilePathResolver::default_stub()),
        }
    }
    
    /// Crea una entidad de archivo a partir de metadatos
    async fn create_file_entity(
        &self,
        id: String,
        name: String,
        storage_path: StoragePath,
        size: u64,
        mime_type: String,
        folder_id: Option<String>,
        created_at: Option<u64>,
        modified_at: Option<u64>,
    ) -> FileRepositoryResult<File> {
        // If timestamps are provided, use them; otherwise, let File::new create default timestamps
        if let (Some(created), Some(modified)) = (created_at, modified_at) {
            File::with_timestamps(
                id, 
                name, 
                storage_path, 
                size, 
                mime_type, 
                folder_id,
                created,
                modified,
            )
            .map_err(|e| crate::domain::repositories::file_repository::FileRepositoryError::Other(e.to_string()))
        } else {
            File::new(
                id, 
                name, 
                storage_path, 
                size, 
                mime_type, 
                folder_id,
            )
            .map_err(|e| crate::domain::repositories::file_repository::FileRepositoryError::Other(e.to_string()))
        }
    }
    
    /// Obtiene un archivo por su ID
    async fn get_file_by_id(&self, id: &str) -> FileRepositoryResult<File> {
        // Obtener la ruta del archivo usando el resolver de rutas
        let storage_path = self.path_resolver.get_path_by_id(id).await?;
        
        // Verificar que el archivo existe físicamente
        let abs_path = self.path_resolver.resolve_storage_path(&storage_path);
        if !self.metadata_manager.file_exists(&abs_path).await
            .map_err(|e| crate::domain::repositories::file_repository::FileRepositoryError::Other(e.to_string()))? {
            return Err(crate::domain::repositories::file_repository::FileRepositoryError::NotFound(
                format!("File {} not found at {}", id, storage_path.to_string())
            ));
        }
        
        // Obtener metadatos del archivo
        let (size, created_at, modified_at) = self.metadata_manager.get_file_metadata(&abs_path).await
            .map_err(|e| match e {
                MetadataError::IoError(io_err) => crate::domain::repositories::file_repository::FileRepositoryError::IoError(io_err),
                MetadataError::Timeout(msg) => crate::domain::repositories::file_repository::FileRepositoryError::Timeout(msg),
                MetadataError::Unavailable(msg) => crate::domain::repositories::file_repository::FileRepositoryError::NotFound(msg),
            })?;
        
        // Obtener nombre del archivo de la ruta
        let name = match storage_path.file_name() {
            Some(name) => name,
            None => {
                return Err(crate::domain::repositories::file_repository::FileRepositoryError::InvalidPath(
                    storage_path.to_string()
                ));
            }
        };
        
        // Determinar ID de carpeta padre
        let parent = storage_path.parent();
        let folder_id: Option<String> = if parent.is_none() || parent.as_ref().unwrap().is_empty() {
            None // Root folder
        } else {
            None // En implementación real, buscar ID de la carpeta padre
        };
        
        // Determinar tipo MIME
        let mime_type = mime_guess::from_path(&abs_path)
            .first_or_octet_stream()
            .to_string();
        
        // Crear entidad de archivo
        let file = self.create_file_entity(
            id.to_string(),
            name,
            storage_path,
            size,
            mime_type,
            folder_id,
            Some(created_at),
            Some(modified_at),
        ).await?;
        
        Ok(file)
    }
}

#[async_trait]
impl FileReadPort for FileFsReadRepository {
    async fn get_file(&self, id: &str) -> Result<File, DomainError> {
        self.get_file_by_id(id).await
            .map_err(|e| match e {
                crate::domain::repositories::file_repository::FileRepositoryError::NotFound(msg) => DomainError::not_found("File", msg),
                crate::domain::repositories::file_repository::FileRepositoryError::IoError(io_err) => DomainError::internal_error("File", io_err.to_string()),
                crate::domain::repositories::file_repository::FileRepositoryError::Timeout(msg) => DomainError::internal_error("File", msg),
                _ => DomainError::internal_error("File", e.to_string()),
            })
    }
    
    async fn list_files(&self, _folder_id: Option<&str>) -> Result<Vec<File>, DomainError> {
        // Implementación real debe obtener la lista de archivos en una carpeta
        // Por ahora, devolvemos lista vacía
        Ok(Vec::new())
    }
    
    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError> {
        // Primero obtenemos el archivo para verificar existencia
        let file = self.get_file_by_id(id).await
            .map_err(|e| match e {
                crate::domain::repositories::file_repository::FileRepositoryError::NotFound(msg) => DomainError::not_found("File", msg),
                crate::domain::repositories::file_repository::FileRepositoryError::IoError(io_err) => DomainError::internal_error("File", io_err.to_string()),
                crate::domain::repositories::file_repository::FileRepositoryError::Timeout(msg) => DomainError::internal_error("File", msg),
                _ => DomainError::internal_error("File", e.to_string()),
            })?;
        
        // Ruta absoluta del archivo
        let _abs_path = self.path_resolver.resolve_storage_path(file.storage_path());
        
        // Implementación real debe leer el contenido del archivo
        // Por ahora, devolvemos un vector vacío
        Ok(Vec::new())
    }
    
    async fn get_file_stream(&self, _id: &str) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        // Implementación real debe devolver un stream de bytes del archivo
        // Por ahora, lanzamos un error
        Err(DomainError::internal_error("File stream", "Stream functionality not yet implemented"))
    }
}