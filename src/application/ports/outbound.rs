use std::path::PathBuf;
use async_trait::async_trait;

use crate::domain::services::path_service::StoragePath;
use crate::common::errors::DomainError;

// Re-export domain repository traits for backward compatibility
pub use crate::domain::repositories::folder_repository::FolderRepository;

use super::storage_ports::{FileReadPort, FileWritePort};

/// Secondary port for storage operations
#[async_trait]
pub trait StoragePort: Send + Sync + 'static {
    /// Resolves a domain path to a physical path
    fn resolve_path(&self, storage_path: &StoragePath) -> PathBuf;
    
    /// Creates directories if they don't exist
    async fn ensure_directory(&self, storage_path: &StoragePath) -> Result<(), DomainError>;
    
    /// Checks if a file exists at the given path
    async fn file_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError>;
    
    /// Checks if a directory exists at the given path
    async fn directory_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError>;
}

/// Unified port for file persistence (backward-compatible).
///
/// Now it is a **supertrait** of `FileReadPort + FileWritePort`.
/// Any type that implements both ports gets `FileStoragePort`
/// automatically via blanket impl. This allows consumers to be migrated
/// gradually to granular ports while existing ones continue
/// working without changes.
pub trait FileStoragePort: FileReadPort + FileWritePort {}

/// Blanket implementation: any type that implements both ports
/// is automatically a FileStoragePort.
impl<T: FileReadPort + FileWritePort> FileStoragePort for T {}

/// Secondary port for folder persistence (application layer).
///
/// Has the same signature as the domain's `FolderRepository`.
/// Concrete implementations must implement `FolderRepository`,
/// getting `FolderStoragePort` automatically via blanket impl.
pub trait FolderStoragePort: FolderRepository {}

/// Blanket implementation: any type that implements FolderRepository
/// is automatically a FolderStoragePort.
impl<T: FolderRepository> FolderStoragePort for T {}

/// Secondary port for ID mapping
#[async_trait]
pub trait IdMappingPort: Send + Sync + 'static {
    /// Gets or creates an ID for a path
    async fn get_or_create_id(&self, path: &StoragePath) -> Result<String, DomainError>;
    
    /// Gets a path by its ID
    async fn get_path_by_id(&self, id: &str) -> Result<StoragePath, DomainError>;
    
    /// Updates the path for an existing ID
    async fn update_path(&self, id: &str, new_path: &StoragePath) -> Result<(), DomainError>;
    
    /// Removes an ID from the mapping
    async fn remove_id(&self, id: &str) -> Result<(), DomainError>;
    
    /// Saves pending changes
    async fn save_changes(&self) -> Result<(), DomainError>;
    
    /// Gets the file path as a PathBuf
    async fn get_file_path(&self, file_id: &str) -> Result<PathBuf, DomainError> {
        let storage_path = self.get_path_by_id(file_id).await?;
        Ok(PathBuf::from(storage_path.to_string()))
    }
    
    /// Updates a file's path
    async fn update_file_path(&self, file_id: &str, new_path: &PathBuf) -> Result<(), DomainError> {
        let storage_path = StoragePath::from_string(&new_path.to_string_lossy().to_string());
        self.update_path(file_id, &storage_path).await
    }
}