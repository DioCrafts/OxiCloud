use crate::common::errors::DomainError;
use crate::domain::entities::session::Session;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum SessionRepositoryError {
    #[error("Session not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Timeout error: {0}")]
    Timeout(String),
}

pub type SessionRepositoryResult<T> = Result<T, SessionRepositoryError>;

// Conversion from SessionRepositoryError to DomainError
impl From<SessionRepositoryError> for DomainError {
    fn from(err: SessionRepositoryError) -> Self {
        match err {
            SessionRepositoryError::NotFound(msg) => DomainError::not_found("Session", msg),
            SessionRepositoryError::DatabaseError(msg) => {
                DomainError::internal_error("Database", msg)
            }
            SessionRepositoryError::Timeout(msg) => DomainError::timeout("Database", msg),
        }
    }
}

#[async_trait]
pub trait SessionRepository: Send + Sync + 'static {
    /// Creates a new session
    async fn create_session(&self, session: Session) -> SessionRepositoryResult<Session>;

    /// Gets a session by ID
    async fn get_session_by_id(&self, id: &str) -> SessionRepositoryResult<Session>;

    /// Gets a session by refresh token
    async fn get_session_by_refresh_token(
        &self,
        refresh_token: &str,
    ) -> SessionRepositoryResult<Session>;

    /// Gets all sessions for a user
    async fn get_sessions_by_user_id(&self, user_id: &str)
    -> SessionRepositoryResult<Vec<Session>>;

    /// Revokes a specific session
    async fn revoke_session(&self, session_id: &str) -> SessionRepositoryResult<()>;

    /// Revokes all sessions for a user
    async fn revoke_all_user_sessions(&self, user_id: &str) -> SessionRepositoryResult<u64>;

    /// Deletes expired sessions
    async fn delete_expired_sessions(&self) -> SessionRepositoryResult<u64>;
}
