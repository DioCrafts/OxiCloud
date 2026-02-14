use async_trait::async_trait;
use chrono::Utc;
use futures::future::BoxFuture;
use sqlx::{PgPool, Row};
use std::sync::Arc;

use crate::application::ports::auth_ports::SessionStoragePort;
use crate::common::errors::DomainError;
use crate::domain::entities::session::Session;
use crate::domain::repositories::session_repository::{
    SessionRepository, SessionRepositoryError, SessionRepositoryResult,
};
use crate::infrastructure::repositories::pg::transaction_utils::with_transaction;

// Implement From<sqlx::Error> for SessionRepositoryError to allow automatic conversions
impl From<sqlx::Error> for SessionRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        SessionPgRepository::map_sqlx_error(err)
    }
}

pub struct SessionPgRepository {
    pool: Arc<PgPool>,
}

impl SessionPgRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // Helper method to map SQL errors to domain errors
    pub fn map_sqlx_error(err: sqlx::Error) -> SessionRepositoryError {
        match err {
            sqlx::Error::RowNotFound => {
                SessionRepositoryError::NotFound("Session not found".to_string())
            }
            _ => SessionRepositoryError::DatabaseError(format!("Database error: {}", err)),
        }
    }
}

#[async_trait]
impl SessionRepository for SessionPgRepository {
    /// Creates a new session using a transaction
    async fn create_session(&self, session: Session) -> SessionRepositoryResult<Session> {
        // Create a copy of the session for the closure
        let session_clone = session.clone();

        with_transaction(&self.pool, "create_session", |tx| {
            Box::pin(async move {
                // Insert the session
                sqlx::query(
                    r#"
                        INSERT INTO auth.sessions (
                            id, user_id, refresh_token, expires_at, 
                            ip_address, user_agent, created_at, revoked
                        ) VALUES (
                            $1, $2, $3, $4, $5, $6, $7, $8
                        )
                        "#,
                )
                .bind(session_clone.id())
                .bind(session_clone.user_id())
                .bind(session_clone.refresh_token())
                .bind(session_clone.expires_at())
                .bind(session_clone.ip_address())
                .bind(session_clone.user_agent())
                .bind(session_clone.created_at())
                .bind(session_clone.is_revoked())
                .execute(&mut **tx)
                .await
                .map_err(Self::map_sqlx_error)?;

                // Optionally, update the user's last login
                // within the same transaction
                sqlx::query(
                    r#"
                        UPDATE auth.users
                        SET last_login_at = NOW(), updated_at = NOW()
                        WHERE id = $1
                        "#,
                )
                .bind(session_clone.user_id())
                .execute(&mut **tx)
                .await
                .map_err(|e| {
                    // Convert the error but without interrupting session
                    // creation if the update fails
                    tracing::warn!(
                        "Could not update last_login_at for user {}: {}",
                        session_clone.user_id(),
                        e
                    );
                    SessionRepositoryError::DatabaseError(format!(
                        "Session created but could not update last_login_at: {}",
                        e
                    ))
                })?;

                Ok(session_clone)
            }) as BoxFuture<'_, SessionRepositoryResult<Session>>
        })
        .await?;

        Ok(session)
    }

    /// Gets a session by ID
    async fn get_session_by_id(&self, id: &str) -> SessionRepositoryResult<Session> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, user_id, refresh_token, expires_at, 
                ip_address, user_agent, created_at, revoked
            FROM auth.sessions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(Session::from_raw(
            row.get("id"),
            row.get("user_id"),
            row.get("refresh_token"),
            row.get("expires_at"),
            row.get("ip_address"),
            row.get("user_agent"),
            row.get("created_at"),
            row.get("revoked"),
        ))
    }

    /// Gets a session by refresh token
    async fn get_session_by_refresh_token(
        &self,
        refresh_token: &str,
    ) -> SessionRepositoryResult<Session> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, user_id, refresh_token, expires_at, 
                ip_address, user_agent, created_at, revoked
            FROM auth.sessions
            WHERE refresh_token = $1
            "#,
        )
        .bind(refresh_token)
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(Session::from_raw(
            row.get("id"),
            row.get("user_id"),
            row.get("refresh_token"),
            row.get("expires_at"),
            row.get("ip_address"),
            row.get("user_agent"),
            row.get("created_at"),
            row.get("revoked"),
        ))
    }

    /// Gets all sessions for a user
    async fn get_sessions_by_user_id(
        &self,
        user_id: &str,
    ) -> SessionRepositoryResult<Vec<Session>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, user_id, refresh_token, expires_at, 
                ip_address, user_agent, created_at, revoked
            FROM auth.sessions
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        let sessions = rows
            .into_iter()
            .map(|row| {
                Session::from_raw(
                    row.get("id"),
                    row.get("user_id"),
                    row.get("refresh_token"),
                    row.get("expires_at"),
                    row.get("ip_address"),
                    row.get("user_agent"),
                    row.get("created_at"),
                    row.get("revoked"),
                )
            })
            .collect();

        Ok(sessions)
    }

    /// Revokes a specific session using a transaction
    async fn revoke_session(&self, session_id: &str) -> SessionRepositoryResult<()> {
        let id = session_id.to_string(); // Clone for use in closure

        with_transaction(&self.pool, "revoke_session", |tx| {
            Box::pin(async move {
                // Revoke the session
                let result = sqlx::query(
                    r#"
                        UPDATE auth.sessions
                        SET revoked = true
                        WHERE id = $1
                        RETURNING user_id
                        "#,
                )
                .bind(&id)
                .fetch_optional(&mut **tx)
                .await
                .map_err(Self::map_sqlx_error)?;

                // If we found the session, we can log a security event
                if let Some(row) = result {
                    let user_id: String = row.try_get("user_id").unwrap_or_default();

                    // Log security event (in a security table)
                    // This is optional but shows how additional operations
                    // can be performed in the same transaction
                    tracing::info!("Session with ID {} for user {} revoked", id, user_id);
                }

                Ok(())
            }) as BoxFuture<'_, SessionRepositoryResult<()>>
        })
        .await
    }

    /// Revokes all sessions for a user using a transaction
    async fn revoke_all_user_sessions(&self, user_id: &str) -> SessionRepositoryResult<u64> {
        let user_id_clone = user_id.to_string(); // Clone for use in closure

        with_transaction(&self.pool, "revoke_all_user_sessions", |tx| {
            Box::pin(async move {
                // Revoke all sessions for the user
                let result = sqlx::query(
                    r#"
                        UPDATE auth.sessions
                        SET revoked = true
                        WHERE user_id = $1 AND revoked = false
                        "#,
                )
                .bind(&user_id_clone)
                .execute(&mut **tx)
                .await
                .map_err(Self::map_sqlx_error)?;

                let affected = result.rows_affected();

                // Log security event
                if affected > 0 {
                    tracing::info!("Revoked {} sessions for user {}", affected, user_id_clone);
                }

                Ok(affected)
            }) as BoxFuture<'_, SessionRepositoryResult<u64>>
        })
        .await
    }

    /// Deletes expired sessions
    async fn delete_expired_sessions(&self) -> SessionRepositoryResult<u64> {
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            DELETE FROM auth.sessions
            WHERE expires_at < $1
            "#,
        )
        .bind(now)
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(result.rows_affected())
    }
}

// Implementation of the storage port for the application layer
#[async_trait]
impl SessionStoragePort for SessionPgRepository {
    async fn create_session(&self, session: Session) -> Result<Session, DomainError> {
        SessionRepository::create_session(self, session)
            .await
            .map_err(DomainError::from)
    }

    async fn get_session_by_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<Session, DomainError> {
        SessionRepository::get_session_by_refresh_token(self, refresh_token)
            .await
            .map_err(DomainError::from)
    }

    async fn revoke_session(&self, session_id: &str) -> Result<(), DomainError> {
        SessionRepository::revoke_session(self, session_id)
            .await
            .map_err(DomainError::from)
    }

    async fn revoke_all_user_sessions(&self, user_id: &str) -> Result<u64, DomainError> {
        SessionRepository::revoke_all_user_sessions(self, user_id)
            .await
            .map_err(DomainError::from)
    }
}
