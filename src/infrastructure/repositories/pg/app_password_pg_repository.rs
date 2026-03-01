//! PostgreSQL repository for App Passwords.

use crate::application::ports::auth_ports::AppPasswordStoragePort;
use crate::common::errors::DomainError;
use crate::domain::entities::app_password::AppPassword;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppPasswordPgRepository {
    pool: Arc<PgPool>,
}

impl AppPasswordPgRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    fn pool(&self) -> &PgPool {
        &self.pool
    }
}

#[async_trait]
impl AppPasswordStoragePort for AppPasswordPgRepository {
    async fn create(&self, ap: AppPassword) -> Result<AppPassword, DomainError> {
        sqlx::query(
            r#"
            INSERT INTO auth.app_passwords
                (id, user_id, label, password_hash, prefix, scopes,
                 created_at, last_used_at, expires_at, active)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(&ap.id)
        .bind(&ap.user_id)
        .bind(&ap.label)
        .bind(&ap.password_hash)
        .bind(&ap.prefix)
        .bind(&ap.scopes)
        .bind(ap.created_at)
        .bind(ap.last_used_at)
        .bind(ap.expires_at)
        .bind(ap.active)
        .execute(self.pool())
        .await
        .map_err(|e| DomainError::internal_error("AppPasswordPg", format!("create: {e}")))?;

        Ok(ap)
    }

    async fn list_by_user(&self, user_id: &str) -> Result<Vec<AppPassword>, DomainError> {
        let rows = sqlx::query_as::<_, AppPasswordRow>(
            r#"
            SELECT id, user_id, label, password_hash, prefix, scopes,
                   created_at, last_used_at, expires_at, active
              FROM auth.app_passwords
             WHERE user_id = $1
             ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(self.pool())
        .await
        .map_err(|e| DomainError::internal_error("AppPasswordPg", format!("list: {e}")))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn get_by_id(&self, id: &str) -> Result<AppPassword, DomainError> {
        let row = sqlx::query_as::<_, AppPasswordRow>(
            r#"
            SELECT id, user_id, label, password_hash, prefix, scopes,
                   created_at, last_used_at, expires_at, active
              FROM auth.app_passwords
             WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| DomainError::internal_error("AppPasswordPg", format!("get_by_id: {e}")))?
        .ok_or_else(|| DomainError::not_found("AppPassword", id))?;

        Ok(row.into())
    }

    async fn get_active_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<AppPassword>, DomainError> {
        let rows = sqlx::query_as::<_, AppPasswordRow>(
            r#"
            SELECT id, user_id, label, password_hash, prefix, scopes,
                   created_at, last_used_at, expires_at, active
              FROM auth.app_passwords
             WHERE user_id = $1
               AND active = TRUE
               AND (expires_at IS NULL OR expires_at > NOW())
            "#,
        )
        .bind(user_id)
        .fetch_all(self.pool())
        .await
        .map_err(|e| {
            DomainError::internal_error("AppPasswordPg", format!("get_active: {e}"))
        })?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn touch_last_used(&self, id: &str) -> Result<(), DomainError> {
        sqlx::query("UPDATE auth.app_passwords SET last_used_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(self.pool())
            .await
            .map_err(|e| {
                DomainError::internal_error("AppPasswordPg", format!("touch: {e}"))
            })?;
        Ok(())
    }

    async fn revoke(&self, id: &str) -> Result<(), DomainError> {
        let result =
            sqlx::query("UPDATE auth.app_passwords SET active = FALSE WHERE id = $1")
                .bind(id)
                .execute(self.pool())
                .await
                .map_err(|e| {
                    DomainError::internal_error("AppPasswordPg", format!("revoke: {e}"))
                })?;

        if result.rows_affected() == 0 {
            return Err(DomainError::not_found("AppPassword", id));
        }
        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64, DomainError> {
        let result = sqlx::query(
            r#"
            DELETE FROM auth.app_passwords
             WHERE (active = FALSE)
                OR (expires_at IS NOT NULL AND expires_at < NOW())
            "#,
        )
        .execute(self.pool())
        .await
        .map_err(|e| {
            DomainError::internal_error("AppPasswordPg", format!("delete_expired: {e}"))
        })?;

        Ok(result.rows_affected())
    }
}

/// Internal row struct for sqlx mapping.
#[derive(sqlx::FromRow)]
struct AppPasswordRow {
    id: String,
    user_id: String,
    label: String,
    password_hash: String,
    prefix: String,
    scopes: String,
    created_at: DateTime<Utc>,
    last_used_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    active: bool,
}

impl From<AppPasswordRow> for AppPassword {
    fn from(r: AppPasswordRow) -> Self {
        AppPassword {
            id: r.id,
            user_id: r.user_id,
            label: r.label,
            password_hash: r.password_hash,
            prefix: r.prefix,
            scopes: r.scopes,
            created_at: r.created_at,
            last_used_at: r.last_used_at,
            expires_at: r.expires_at,
            active: r.active,
        }
    }
}
