use async_trait::async_trait;
use crate::domain::entities::folder::Folder;
use crate::domain::services::path_service::StoragePath;
use crate::common::errors::DomainError;

/// Error types for folder repository operations
#[derive(Debug, thiserror::Error)]
pub enum FolderRepositoryError {
    #[error("Folder not found: {0}")]
    NotFound(String),
    
    #[error("Folder already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Invalid folder path: {0}")]
    InvalidPath(String),
    
    #[error("Operation not supported: {0}")]
    OperationNotSupported(String),
    
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Mapping error: {0}")]
    MappingError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for folder repository operations
pub type FolderRepositoryResult<T> = Result<T, FolderRepositoryError>;

/// Repository interface for folder operations (primary port)
#[async_trait]
pub trait FolderRepository: Send + Sync + 'static {
    /// Creates a new folder
    async fn create_folder(&self, name: String, parent_id: Option<String>) -> FolderRepositoryResult<Folder>;
    
    /// Gets a folder by its ID
    async fn get_folder_by_id(&self, id: &str) -> FolderRepositoryResult<Folder>;
    
    /// Gets a folder by its path
    async fn get_folder_by_storage_path(&self, storage_path: &StoragePath) -> FolderRepositoryResult<Folder>;
    
    /// Lists all folders in a parent folder (use with caution for large directories)
    async fn list_folders(&self, parent_id: Option<&str>) -> FolderRepositoryResult<Vec<Folder>>;
    
    /// Lists folders in a parent folder with pagination support
    /// 
    /// * `parent_id` - Optional parent folder ID
    /// * `offset` - Number of folders to skip
    /// * `limit` - Maximum number of folders to return
    /// * `include_total` - If true, returns the total count of folders as well
    async fn list_folders_paginated(
        &self, 
        parent_id: Option<&str>, 
        offset: usize, 
        limit: usize,
        include_total: bool
    ) -> FolderRepositoryResult<(Vec<Folder>, Option<usize>)>;
    
    /// Renames a folder
    async fn rename_folder(&self, id: &str, new_name: String) -> FolderRepositoryResult<Folder>;
    
    /// Moves a folder to a new parent
    async fn move_folder(&self, id: &str, new_parent_id: Option<&str>) -> FolderRepositoryResult<Folder>;
    
    /// Deletes a folder
    async fn delete_folder(&self, id: &str) -> FolderRepositoryResult<()>;
    
    /// Checks if a folder exists at the given path
    async fn folder_exists_at_storage_path(&self, storage_path: &StoragePath) -> FolderRepositoryResult<bool>;
    
    /// Gets the storage path for a folder
    async fn get_folder_storage_path(&self, id: &str) -> FolderRepositoryResult<StoragePath>;
    
    /// Legacy method - checks if a folder exists at the given PathBuf path
    #[deprecated(note = "Use folder_exists_at_storage_path instead")]
    async fn folder_exists(&self, path: &std::path::PathBuf) -> FolderRepositoryResult<bool>;
    
    /// Legacy method - gets a folder by its PathBuf path
    #[deprecated(note = "Use get_folder_by_storage_path instead")]
    async fn get_folder_by_path(&self, path: &std::path::PathBuf) -> FolderRepositoryResult<Folder>;
    
    /// Moves a folder to trash
    async fn move_to_trash(&self, folder_id: &str) -> FolderRepositoryResult<()>;
    
    /// Restores a folder from trash
    async fn restore_from_trash(&self, folder_id: &str, original_path: &str) -> FolderRepositoryResult<()>;
    
    /// Permanently deletes a folder (used for trash cleanup)
    async fn delete_folder_permanently(&self, folder_id: &str) -> FolderRepositoryResult<()>;
}