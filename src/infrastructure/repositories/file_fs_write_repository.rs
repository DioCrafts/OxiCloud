use std::path::PathBuf;
use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::entities::file::File;
use crate::application::ports::storage_ports::FileWritePort;
use crate::common::errors::DomainError;
use crate::domain::repositories::file_repository::FileRepositoryResult;
use crate::infrastructure::repositories::file_metadata_manager::{FileMetadataManager, MetadataError};
use crate::infrastructure::repositories::file_path_resolver::FilePathResolver;
use crate::domain::services::path_service::StoragePath;
use crate::infrastructure::repositories::parallel_file_processor::ParallelFileProcessor;
use crate::common::config::AppConfig;
use crate::application::services::storage_mediator::StorageMediator;

/// Implementación de repositorio para operaciones de escritura de archivos
pub struct FileFsWriteRepository {
    root_path: PathBuf,
    metadata_manager: Arc<FileMetadataManager>,
    path_resolver: Arc<FilePathResolver>,
    storage_mediator: Arc<dyn StorageMediator>,
    config: AppConfig,
    parallel_processor: Option<Arc<ParallelFileProcessor>>,
}

impl FileFsWriteRepository {
    /// Crea un nuevo repositorio de escritura de archivos
    pub fn new(
        root_path: PathBuf,
        metadata_manager: Arc<FileMetadataManager>,
        path_resolver: Arc<FilePathResolver>,
        storage_mediator: Arc<dyn StorageMediator>,
        config: AppConfig,
        parallel_processor: Option<Arc<ParallelFileProcessor>>,
    ) -> Self {
        Self {
            root_path,
            metadata_manager,
            path_resolver,
            storage_mediator,
            config,
            parallel_processor,
        }
    }
    
    /// Crea un stub para pruebas
    pub fn default_stub() -> Self {
        Self {
            root_path: PathBuf::from("./storage"),
            metadata_manager: Arc::new(FileMetadataManager::default()),
            path_resolver: Arc::new(FilePathResolver::default_stub()),
            storage_mediator: Arc::new(crate::application::services::storage_mediator::FileSystemStorageMediator::new_stub()),
            config: AppConfig::default(),
            parallel_processor: None,
        }
    }
    
    /// Crea directorios padres si es necesario
    async fn ensure_parent_directory(&self, abs_path: &PathBuf) -> FileRepositoryResult<()> {
        if let Some(parent) = abs_path.parent() {
            tokio::time::timeout(
                self.config.timeouts.dir_timeout(),
                tokio::fs::create_dir_all(parent)
            ).await
            .map_err(|_| crate::domain::repositories::file_repository::FileRepositoryError::Timeout(
                format!("Timeout creating parent directory: {}", parent.display())
            ))?
            .map_err(crate::domain::repositories::file_repository::FileRepositoryError::IoError)?;
        }
        Ok(())
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
        user_id: Option<String>,
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
                user_id,
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
                user_id,
            )
            .map_err(|e| crate::domain::repositories::file_repository::FileRepositoryError::Other(e.to_string()))
        }
    }
    
    /// Elimina un archivo de forma no bloqueante
    async fn delete_file_non_blocking(&self, _abs_path: PathBuf) -> FileRepositoryResult<()> {
        // Implementación real debe eliminar el archivo
        // Por ahora, devolvemos OK
        Ok(())
    }
}

#[async_trait]
impl FileWritePort for FileFsWriteRepository {
    async fn save_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
        user_id: Option<String>,
    ) -> Result<File, DomainError> {
        // Generate a unique ID for the file
        let file_id = uuid::Uuid::new_v4().to_string();
        
        // Calculate the storage path for this file
        let storage_path = match &folder_id {
            Some(folder_id) => {
                StoragePath::from_string(
                    &format!("/{}/{}", folder_id, name)
                )
            },
            None => {
                StoragePath::from_string(
                    &format!("/{}", name)
                )
            }
        };
        
        // Resolve the absolute path on disk
        let abs_path = self.path_resolver.resolve_file_path(&storage_path);
        
        // Ensure the parent directory exists
        self.ensure_parent_directory(&abs_path).await
            .map_err(|e| DomainError::internal_error("File system", e.to_string()))?;
        
        // Write the file to disk
        tokio::time::timeout(
            self.config.timeouts.file_write_timeout(),
            tokio::fs::write(&abs_path, &content)
        ).await
        .map_err(|_| DomainError::internal_error(
            "File write", 
            format!("Timeout writing file: {}", abs_path.display())
        ))?
        .map_err(|e| DomainError::internal_error(
            "File system", 
            format!("Error writing file: {} - {}", abs_path.display(), e)
        ))?;
        
        // Create and return a File entity
        let size = content.len() as u64;
        let file = self.create_file_entity(
            file_id, 
            name, 
            storage_path, 
            size, 
            content_type, 
            folder_id,
            user_id,
            None,
            None,
        ).await
        .map_err(|e| DomainError::internal_error("File entity creation", e.to_string()))?;
        
        // Save metadata
        self.metadata_manager.update_file_metadata(&file)
            .await
            .map_err(|e| match e {
                MetadataError::IoError(e) => DomainError::internal_error("File metadata", e.to_string()),
                MetadataError::Timeout(msg) => DomainError::internal_error("File metadata", msg),
                MetadataError::Unavailable(msg) => DomainError::not_found("File metadata", msg)
            })?;
            
        tracing::info!("File saved successfully: {} (ID: {})", file.name(), file.id());
        Ok(file)
    }
    
    async fn move_file(&self, _file_id: &str, _target_folder_id: Option<String>) -> Result<File, DomainError> {
        // Implementación real debe mover el archivo a otra carpeta
        // Por ahora, devolvemos un error
        Err(DomainError::internal_error("File move", "Move functionality not yet implemented"))
    }
    
    async fn delete_file(&self, _id: &str) -> Result<(), DomainError> {
        // Por ahora, devolvemos OK simulando éxito
        // En una implementación real, buscaríamos el archivo por ID y lo eliminaríamos
        tracing::info!("File deletion simulated successfully");
        Ok(())
    }
}