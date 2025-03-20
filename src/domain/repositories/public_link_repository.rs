use async_trait::async_trait;
use thiserror::Error;
use chrono::{DateTime, Utc};

use crate::domain::entities::public_link::PublicLink;
use crate::domain::entities::shared_file::PermissionLevel;
use crate::common::errors::DomainError;

/// Error types for public link repository operations
#[derive(Debug, Error)]
pub enum PublicLinkRepositoryError {
    #[error("Public link not found: {0}")]
    NotFound(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Invalid permission: {0}")]
    InvalidPermission(String),
    
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    
    #[error("Link expired")]
    Expired,
    
    #[error("Password required")]
    PasswordRequired,
    
    #[error("Invalid password")]
    InvalidPassword,
    
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for public link repository operations
pub type PublicLinkRepositoryResult<T> = Result<T, PublicLinkRepositoryError>;

/// Repository interface for public link operations
#[async_trait]
pub trait PublicLinkRepository: Send + Sync + 'static {
    /// Creates a new public link for a file
    async fn create_public_link(
        &self,
        file_id: &str,
        owner_id: &str,
        permission: PermissionLevel,
        password: Option<&str>,
        expires_at: Option<DateTime<Utc>>,
    ) -> PublicLinkRepositoryResult<PublicLink>;
    
    /// Gets a public link by its ID
    async fn get_public_link(
        &self,
        link_id: &str,
    ) -> PublicLinkRepositoryResult<PublicLink>;
    
    /// Updates the permission for a public link
    async fn update_permission(
        &self,
        link_id: &str,
        owner_id: &str,
        permission: PermissionLevel,
    ) -> PublicLinkRepositoryResult<PublicLink>;
    
    /// Updates the password for a public link
    async fn update_password(
        &self,
        link_id: &str,
        owner_id: &str,
        password: Option<&str>,
    ) -> PublicLinkRepositoryResult<PublicLink>;
    
    /// Updates the expiration date for a public link
    async fn update_expiration(
        &self,
        link_id: &str,
        owner_id: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> PublicLinkRepositoryResult<PublicLink>;
    
    /// Deletes a public link
    async fn delete_public_link(
        &self,
        link_id: &str,
        owner_id: &str,
    ) -> PublicLinkRepositoryResult<()>;
    
    /// Gets all public links for a file
    async fn get_links_for_file(
        &self,
        file_id: &str,
        owner_id: &str,
    ) -> PublicLinkRepositoryResult<Vec<PublicLink>>;
    
    /// Gets all public links created by a user
    async fn get_links_by_user(
        &self,
        owner_id: &str,
    ) -> PublicLinkRepositoryResult<Vec<PublicLink>>;
    
    /// Verifies a password for a public link
    async fn verify_password(
        &self,
        link_id: &str,
        password: &str,
    ) -> PublicLinkRepositoryResult<bool>;
    
    /// Increments the access counter for a public link
    async fn increment_access_count(
        &self,
        link_id: &str,
    ) -> PublicLinkRepositoryResult<PublicLink>;
}