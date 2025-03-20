use async_trait::async_trait;
use crate::domain::entities::file::File;
use crate::domain::services::path_service::StoragePath;
use crate::common::errors::DomainError;
use futures::Stream;
use bytes::Bytes;

/// Error types for file repository operations
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum FileRepositoryError {
    #[error("File not found: {0}")]
    NotFound(String),
    
    #[error("File already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Invalid file path: {0}")]
    InvalidPath(String),
    
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Mapping error: {0}")]
    MappingError(String),
    
    #[error("ID Mapping error: {0}")]
    IdMappingError(String),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for file repository operations
pub type FileRepositoryResult<T> = Result<T, FileRepositoryError>;

/// Repository interface for file operations (primary port)
/// Esta interfaz define las operaciones de negocio relacionadas con archivos
/// sin exponer detalles de implementación como rutas o sistemas de archivos
#[async_trait]
pub trait FileRepository: Send + Sync + 'static {
    /// Saves a file from bytes
    async fn save_file_from_bytes(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
        user_id: Option<String>,
    ) -> FileRepositoryResult<File>;
    
    /// Saves a file with a specific ID
    #[allow(dead_code)]
    async fn save_file_with_id(
        &self,
        id: String,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
        user_id: Option<String>,
    ) -> FileRepositoryResult<File>;
    
    /// Gets a file by its ID
    async fn get_file_by_id(&self, id: &str) -> FileRepositoryResult<File>;
    
    /// Lists files in a folder
    async fn list_files(&self, folder_id: Option<&str>) -> FileRepositoryResult<Vec<File>>;
    
    /// Lists files for a specific user
    async fn list_files_by_user(&self, user_id: &str, folder_id: Option<&str>) -> FileRepositoryResult<Vec<File>>;
    
    /// Deletes a file
    async fn delete_file(&self, id: &str) -> FileRepositoryResult<()>;
    
    /// Deletes a file and its entry from mapping systems
    #[allow(dead_code)]
    async fn delete_file_entry(&self, id: &str) -> FileRepositoryResult<()>;
    
    /// Gets file content as bytes - use only for small files
    async fn get_file_content(&self, id: &str) -> FileRepositoryResult<Vec<u8>>;
    
    /// Gets file content as a stream - better for large files
    #[allow(clippy::type_complexity)]
    async fn get_file_stream(&self, id: &str) -> FileRepositoryResult<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>;
    
    /// Moves a file to a different folder
    async fn move_file(&self, id: &str, target_folder_id: Option<String>) -> FileRepositoryResult<File>;
    
    /// Gets the storage path for a file
    async fn get_file_path(&self, id: &str) -> FileRepositoryResult<StoragePath>;
    
    /// Updates the owner of a file
    async fn update_file_owner(&self, id: &str, user_id: Option<String>) -> FileRepositoryResult<File>;
    
    /// Checks if a user has access to a file
    async fn check_file_access(&self, file_id: &str, user_id: &str) -> FileRepositoryResult<bool>;
}