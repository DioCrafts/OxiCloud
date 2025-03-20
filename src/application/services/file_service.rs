use std::sync::Arc;
use thiserror::Error;
use async_trait::async_trait;

use crate::domain::repositories::file_repository::FileRepositoryError;
use crate::application::dtos::file_dto::FileDto;
use crate::application::ports::inbound::FileUseCase;
use crate::application::ports::outbound::FileStoragePort;
use crate::common::errors::DomainError;
use futures::Stream;
use bytes::Bytes;

/// Errores específicos del servicio de archivos
#[derive(Debug, Error)]
pub enum FileServiceError {
    #[error("Archivo no encontrado: {0}")]
    NotFound(String),
    
    #[error("Archivo ya existe: {0}")]
    Conflict(String),
    
    #[error("Error de acceso al archivo: {0}")]
    AccessError(String),
    
    #[error("Ruta de archivo inválida: {0}")]
    InvalidPath(String),
    
    #[error("Error interno: {0}")]
    InternalError(String),
}

impl From<FileRepositoryError> for FileServiceError {
    fn from(err: FileRepositoryError) -> Self {
        match err {
            FileRepositoryError::NotFound(id) => FileServiceError::NotFound(id),
            FileRepositoryError::AlreadyExists(path) => FileServiceError::Conflict(path),
            FileRepositoryError::InvalidPath(path) => FileServiceError::InvalidPath(path),
            FileRepositoryError::IoError(e) => FileServiceError::AccessError(e.to_string()),
            FileRepositoryError::Timeout(msg) => FileServiceError::AccessError(format!("Operación expiró: {}", msg)),
            _ => FileServiceError::InternalError(err.to_string()),
        }
    }
}

impl From<DomainError> for FileServiceError {
    fn from(err: DomainError) -> Self {
        match err.kind {
            crate::common::errors::ErrorKind::NotFound => FileServiceError::NotFound(err.to_string()),
            crate::common::errors::ErrorKind::AlreadyExists => FileServiceError::Conflict(err.to_string()),
            crate::common::errors::ErrorKind::InvalidInput => FileServiceError::InvalidPath(err.to_string()),
            crate::common::errors::ErrorKind::AccessDenied => FileServiceError::AccessError(err.to_string()),
            _ => FileServiceError::InternalError(err.to_string()),
        }
    }
}

impl From<FileServiceError> for DomainError {
    fn from(err: FileServiceError) -> Self {
        match err {
            FileServiceError::NotFound(id) => DomainError::not_found("File", id),
            FileServiceError::Conflict(path) => DomainError::already_exists("File", path),
            FileServiceError::InvalidPath(path) => DomainError::validation_error("File", format!("Invalid path: {}", path)),
            FileServiceError::AccessError(msg) => DomainError::access_denied("File", msg),
            FileServiceError::InternalError(msg) => DomainError::internal_error("File", msg),
        }
    }
}

pub type FileServiceResult<T> = Result<T, FileServiceError>;

/// Service for file operations
pub struct FileService {
    file_repository: Arc<dyn FileStoragePort>,
}

impl FileService {
    /// Creates a new file service
    pub fn new(file_repository: Arc<dyn FileStoragePort>) -> Self {
        Self { file_repository }
    }
    
    /// Creates a stub implementation for testing and middleware
    pub fn new_stub() -> impl FileUseCase {
        struct FileServiceStub;
        
        #[async_trait]
        impl FileUseCase for FileServiceStub {
            async fn upload_file(
                &self,
                _name: String,
                _folder_id: Option<String>,
                _content_type: String,
                _content: Vec<u8>,
                _user_id: Option<String>,
            ) -> Result<FileDto, DomainError> {
                Ok(FileDto::empty())
            }
            
            async fn get_file(&self, _id: &str) -> Result<FileDto, DomainError> {
                Ok(FileDto::empty())
            }
            
            async fn list_files(&self, _folder_id: Option<&str>) -> Result<Vec<FileDto>, DomainError> {
                Ok(vec![])
            }
            
            async fn delete_file(&self, _id: &str) -> Result<(), DomainError> {
                Ok(())
            }
            
            async fn get_file_content(&self, _id: &str) -> Result<Vec<u8>, DomainError> {
                Ok(vec![])
            }
            
            async fn get_file_stream(&self, _id: &str) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
                let empty_stream = futures::stream::empty();
                Ok(Box::new(empty_stream))
            }
            
            async fn move_file(&self, _file_id: &str, _folder_id: Option<String>) -> Result<FileDto, DomainError> {
                Ok(FileDto::empty())
            }
        }
        
        FileServiceStub
    }
    
    /// Uploads a new file from bytes
    pub async fn upload_file_from_bytes(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
        user_id: Option<String>,
    ) -> FileServiceResult<FileDto>
    {
        // Pass the user_id to the repository to associate the file with the user
        let file = self.file_repository.save_file(name, folder_id, content_type, content, user_id).await
            .map_err(FileServiceError::from)?;
        Ok(FileDto::from(file))
    }
    
    /// List files for a specific user
    pub async fn list_files_by_user(&self, user_id: &str, folder_id: Option<&str>) -> FileServiceResult<Vec<FileDto>> {
        tracing::info!("Listing files for user {} in folder {:?}", user_id, folder_id);
        
        // Get all files in the folder
        let all_files = self.file_repository.list_files(folder_id).await
            .map_err(FileServiceError::from)?;
        
        // Filter files by user_id
        let user_files = all_files.into_iter()
            .filter(|file| file.user_id() == Some(user_id) || file.user_id().is_none())
            .collect::<Vec<_>>();
            
        Ok(user_files.into_iter().map(FileDto::from).collect())
    }
    
    /// Check if a user has access to a specific file
    pub async fn check_file_access(&self, file_id: &str, user_id: &str) -> bool {
        // Try to get the file first to make sure it exists
        match self.file_repository.get_file(file_id).await {
            Ok(file) => {
                // If the file has no user_id (legacy file), allow access
                if let Some(file_user_id) = file.user_id() {
                    // Only allow access if the file belongs to this user
                    return file_user_id == user_id;
                } else {
                    // For files without an owner, assume access is allowed
                    return true;
                }
            },
            Err(e) => {
                tracing::error!("Error getting file for access check: {}", e);
                return false;
            }
        }
    }
    
    /// Gets a file by ID
    pub async fn get_file(&self, id: &str) -> FileServiceResult<FileDto> {
        let file = self.file_repository.get_file(id).await
            .map_err(FileServiceError::from)?;
        Ok(FileDto::from(file))
    }
    
    /// Lists files in a folder
    pub async fn list_files(&self, folder_id: Option<&str>) -> FileServiceResult<Vec<FileDto>> {
        let files = self.file_repository.list_files(folder_id).await
            .map_err(FileServiceError::from)?;
        Ok(files.into_iter().map(FileDto::from).collect())
    }
    
    /// Deletes a file
    pub async fn delete_file(&self, id: &str) -> FileServiceResult<()> {
        self.file_repository.delete_file(id).await
            .map_err(FileServiceError::from)
    }
    
    /// Gets file content as bytes - use for small files only
    pub async fn get_file_content(&self, id: &str) -> FileServiceResult<Vec<u8>> {
        self.file_repository.get_file_content(id).await
            .map_err(FileServiceError::from)
    }
    
    /// Gets file content as stream - better for large files
    pub async fn get_file_stream(&self, id: &str) -> FileServiceResult<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>> {
        self.file_repository.get_file_stream(id).await
            .map_err(FileServiceError::from)
    }
    
    /// Moves a file to a new folder using filesystem operations directly
    pub async fn move_file(&self, file_id: &str, folder_id: Option<String>) -> FileServiceResult<FileDto> {
        tracing::info!("Moviendo archivo con ID: {} a carpeta: {:?}", file_id, folder_id);
        
        // Usar la implementación eficiente del repositorio que utiliza rename
        let moved_file = self.file_repository.move_file(file_id, folder_id).await
            .map_err(|e| {
                tracing::error!("Error al mover archivo (ID: {}): {}", file_id, e);
                FileServiceError::from(e)
            })?;
        
        tracing::info!("Archivo movido exitosamente: {} (ID: {}) a carpeta: {:?}", 
                       moved_file.name(), moved_file.id(), moved_file.folder_id());
        
        Ok(FileDto::from(moved_file))
    }
}

#[async_trait]
impl FileUseCase for FileService {
    async fn upload_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
        user_id: Option<String>,
    ) -> Result<FileDto, DomainError> {
        FileService::upload_file_from_bytes(self, name, folder_id, content_type, content, user_id).await
            .map_err(DomainError::from)
    }
    
    async fn get_file(&self, id: &str) -> Result<FileDto, DomainError> {
        FileService::get_file(self, id).await
            .map_err(DomainError::from)
    }
    
    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<FileDto>, DomainError> {
        FileService::list_files(self, folder_id).await
            .map_err(DomainError::from)
    }
    
    async fn delete_file(&self, id: &str) -> Result<(), DomainError> {
        FileService::delete_file(self, id).await
            .map_err(DomainError::from)
    }
    
    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError> {
        FileService::get_file_content(self, id).await
            .map_err(DomainError::from)
    }
    
    async fn get_file_stream(&self, id: &str) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        FileService::get_file_stream(self, id).await
            .map_err(DomainError::from)
    }
    
    async fn move_file(&self, file_id: &str, folder_id: Option<String>) -> Result<FileDto, DomainError> {
        FileService::move_file(self, file_id, folder_id).await
            .map_err(DomainError::from)
    }
}