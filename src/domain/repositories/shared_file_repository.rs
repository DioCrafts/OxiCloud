use async_trait::async_trait;
use thiserror::Error;

use crate::domain::entities::shared_file::{SharedFile, PermissionLevel};
use crate::common::errors::DomainError;

/// Error types for shared file repository operations
#[derive(Debug, Error)]
pub enum SharedFileRepositoryError {
    #[error("Shared file not found: {0}")]
    NotFound(String),
    
    #[error("Shared file already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Invalid permission: {0}")]
    InvalidPermission(String),
    
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for shared file repository operations
pub type SharedFileRepositoryResult<T> = Result<T, SharedFileRepositoryError>;

/// Repository interface for shared file operations
#[async_trait]
pub trait SharedFileRepository: Send + Sync + 'static {
    /// Shares a file with a user with specified permission
    async fn share_file_with_user(
        &self,
        file_id: &str,
        owner_id: &str,
        user_id: &str,
        permission: PermissionLevel,
    ) -> SharedFileRepositoryResult<SharedFile>;
    
    /// Updates the permission level for a shared file
    async fn update_permission(
        &self,
        file_id: &str,
        user_id: &str,
        permission: PermissionLevel,
    ) -> SharedFileRepositoryResult<SharedFile>;
    
    /// Removes sharing for a file with a specific user
    async fn unshare_file(
        &self,
        file_id: &str,
        user_id: &str,
    ) -> SharedFileRepositoryResult<()>;
    
    /// Gets a shared file by file_id and user_id
    async fn get_shared_file(
        &self,
        file_id: &str,
        user_id: &str,
    ) -> SharedFileRepositoryResult<SharedFile>;
    
    /// Gets all files shared with a specific user
    async fn get_files_shared_with_user(
        &self,
        user_id: &str,
    ) -> SharedFileRepositoryResult<Vec<SharedFile>>;
    
    /// Gets all users a file is shared with
    async fn get_users_with_access(
        &self,
        file_id: &str,
    ) -> SharedFileRepositoryResult<Vec<SharedFile>>;
    
    /// Gets all files a user has shared with others
    async fn get_files_shared_by_user(
        &self,
        owner_id: &str,
    ) -> SharedFileRepositoryResult<Vec<SharedFile>>;
    
    /// Checks if a user has access to a file
    async fn check_user_has_access(
        &self,
        file_id: &str,
        user_id: &str,
    ) -> SharedFileRepositoryResult<Option<PermissionLevel>>;
}