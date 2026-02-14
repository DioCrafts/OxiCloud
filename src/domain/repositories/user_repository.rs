use crate::common::errors::DomainError;
use crate::domain::entities::user::{User, UserRole};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum UserRepositoryError {
    #[error("User not found: {0}")]
    NotFound(String),

    #[error("User already exists: {0}")]
    AlreadyExists(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Operation not allowed: {0}")]
    OperationNotAllowed(String),
}

pub type UserRepositoryResult<T> = Result<T, UserRepositoryError>;

// Conversion from UserRepositoryError to DomainError
impl From<UserRepositoryError> for DomainError {
    fn from(err: UserRepositoryError) -> Self {
        match err {
            UserRepositoryError::NotFound(msg) => DomainError::not_found("User", msg),
            UserRepositoryError::AlreadyExists(msg) => DomainError::already_exists("User", msg),
            UserRepositoryError::DatabaseError(msg) => DomainError::internal_error("Database", msg),
            UserRepositoryError::ValidationError(msg) => DomainError::validation_error(msg),
            UserRepositoryError::Timeout(msg) => DomainError::timeout("Database", msg),
            UserRepositoryError::OperationNotAllowed(msg) => {
                DomainError::access_denied("User", msg)
            }
        }
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    /// Creates a new user
    async fn create_user(&self, user: User) -> UserRepositoryResult<User>;

    /// Gets a user by ID
    async fn get_user_by_id(&self, id: &str) -> UserRepositoryResult<User>;

    /// Gets a user by username
    async fn get_user_by_username(&self, username: &str) -> UserRepositoryResult<User>;

    /// Gets a user by email
    async fn get_user_by_email(&self, email: &str) -> UserRepositoryResult<User>;

    /// Updates an existing user
    async fn update_user(&self, user: User) -> UserRepositoryResult<User>;

    /// Updates only a user's storage usage
    async fn update_storage_usage(
        &self,
        user_id: &str,
        usage_bytes: i64,
    ) -> UserRepositoryResult<()>;

    /// Updates the last login date
    async fn update_last_login(&self, user_id: &str) -> UserRepositoryResult<()>;

    /// Lists users with pagination
    async fn list_users(&self, limit: i64, offset: i64) -> UserRepositoryResult<Vec<User>>;

    /// Activates or deactivates a user
    async fn set_user_active_status(&self, user_id: &str, active: bool)
    -> UserRepositoryResult<()>;

    /// Changes a user's password
    async fn change_password(&self, user_id: &str, password_hash: &str)
    -> UserRepositoryResult<()>;

    /// Changes a user's role
    async fn change_role(&self, user_id: &str, role: UserRole) -> UserRepositoryResult<()>;

    /// Lists users by role (admin or user)
    async fn list_users_by_role(&self, role: &str) -> UserRepositoryResult<Vec<User>>;

    /// Deletes a user
    async fn delete_user(&self, user_id: &str) -> UserRepositoryResult<()>;

    /// Finds a user by OIDC provider + subject pair
    async fn get_user_by_oidc_subject(
        &self,
        provider: &str,
        subject: &str,
    ) -> UserRepositoryResult<User>;

    /// Updates a user's storage quota
    async fn update_storage_quota(
        &self,
        user_id: &str,
        quota_bytes: i64,
    ) -> UserRepositoryResult<()>;

    /// Counts the total number of users
    async fn count_users(&self) -> UserRepositoryResult<i64>;

    /// Gets aggregated storage statistics
    async fn get_storage_stats(&self) -> UserRepositoryResult<StorageStats>;
}

/// Aggregated storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_users: i64,
    pub active_users: i64,
    pub total_quota_bytes: i64,
    pub total_used_bytes: i64,
    pub users_over_80_percent: i64,
    pub users_over_quota: i64,
}
