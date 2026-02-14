//! Infrastructure-layer error types for file and folder repository operations.
//!
//! These error types are used internally by the filesystem repository implementations
//! (FileFsReadRepository, FileFsWriteRepository, FolderFsRepository, etc.) to represent
//! errors that can occur during storage operations. They are converted to `DomainError`
//! at the port boundary before crossing into the application layer.

use crate::common::errors::DomainError;

/// Error types for file repository operations.
#[derive(Debug, thiserror::Error)]
pub enum FileRepositoryError {
    #[error("File not found: {0}")]
    NotFound(String),

    #[error("File already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid file path: {0}")]
    InvalidPath(String),

    #[error("Operation not supported: {0}")]
    OperationNotSupported(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),

    #[error("Other error: {0}")]
    Other(String),
}

pub type FileRepositoryResult<T> = Result<T, FileRepositoryError>;

/// Error types for folder repository operations.
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

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),

    #[error("Other error: {0}")]
    Other(String),
}

pub type FolderRepositoryResult<T> = Result<T, FolderRepositoryError>;

// ── Conversions to DomainError ──

impl From<FileRepositoryError> for DomainError {
    fn from(err: FileRepositoryError) -> Self {
        match err {
            FileRepositoryError::NotFound(id) => DomainError::not_found("File", id),
            FileRepositoryError::AlreadyExists(path) => DomainError::already_exists("File", path),
            FileRepositoryError::InvalidPath(path) => {
                DomainError::validation_error(format!("Invalid path: {}", path))
            }
            FileRepositoryError::StorageError(msg) => {
                DomainError::internal_error("File", format!("Storage error: {}", msg))
            }
            FileRepositoryError::Other(msg) => DomainError::internal_error("File", msg),
            FileRepositoryError::OperationNotSupported(msg) => {
                DomainError::operation_not_supported("File", msg)
            }
            FileRepositoryError::DomainError(e) => e,
        }
    }
}

impl From<FolderRepositoryError> for DomainError {
    fn from(err: FolderRepositoryError) -> Self {
        match err {
            FolderRepositoryError::NotFound(id) => DomainError::not_found("Folder", id),
            FolderRepositoryError::AlreadyExists(path) => {
                DomainError::already_exists("Folder", path)
            }
            FolderRepositoryError::InvalidPath(path) => {
                DomainError::validation_error(format!("Invalid path: {}", path))
            }
            FolderRepositoryError::StorageError(msg) => {
                DomainError::internal_error("Folder", format!("Storage error: {}", msg))
            }
            FolderRepositoryError::ValidationError(msg) => DomainError::validation_error(msg),
            FolderRepositoryError::Other(msg) => DomainError::internal_error("Folder", msg),
            FolderRepositoryError::OperationNotSupported(msg) => {
                DomainError::operation_not_supported("Folder", msg)
            }
            FolderRepositoryError::DomainError(e) => e,
        }
    }
}
