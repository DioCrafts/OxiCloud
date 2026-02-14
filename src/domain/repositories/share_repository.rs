use async_trait::async_trait;
use thiserror::Error;

use crate::domain::{
    entities::share::{Share, ShareItemType},
    repositories::user_repository::UserRepositoryError,
};

#[derive(Debug, Error)]
pub enum ShareRepositoryError {
    #[error("Share not found: {0}")]
    NotFound(String),
    #[error("Item not found: {0}")]
    ItemNotFound(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("User repository error: {0}")]
    UserRepository(#[from] UserRepositoryError),
    #[error("Share already exists: {0}")]
    AlreadyExists(String),
}

#[async_trait]
pub trait ShareRepository: Send + Sync + 'static {
    /// Save a new share or update an existing one
    async fn save(&self, share: &Share) -> Result<Share, ShareRepositoryError>;

    /// Find a share by its ID
    async fn find_by_id(&self, id: &str) -> Result<Share, ShareRepositoryError>;

    /// Find a share by its token
    async fn find_by_token(&self, token: &str) -> Result<Share, ShareRepositoryError>;

    /// Find all shares for a specific item
    async fn find_by_item(
        &self,
        item_id: &str,
        item_type: &ShareItemType,
    ) -> Result<Vec<Share>, ShareRepositoryError>;

    /// Delete a share by its ID
    async fn delete(&self, id: &str) -> Result<(), ShareRepositoryError>;

    /// Find all shares created by a specific user
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<Share>, ShareRepositoryError>;

    /// Find all shares (admin operation)
    async fn find_all(&self) -> Result<Vec<Share>, ShareRepositoryError>;
}
