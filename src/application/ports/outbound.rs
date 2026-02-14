use async_trait::async_trait;
use std::path::PathBuf;

use crate::common::errors::DomainError;
use crate::domain::services::path_service::StoragePath;

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
