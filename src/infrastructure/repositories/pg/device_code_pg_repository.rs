//! PostgreSQL repository for Device Authorization Grant (RFC 8628) codes.

use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;

use crate::application::ports::auth_ports::DeviceCodeStoragePort;
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::entities::device_code::{DeviceCode, DeviceCodeStatus};

pub struct DeviceCodePgRepository {
    pool: Arc<PgPool>,
}

impl DeviceCodePgRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    fn map_row(row: &sqlx::postgres::PgRow) -> Result<DeviceCode, DomainError> {
        let status_str: String = row.try_get("status").map_err(|e| {
            DomainError::new(
                ErrorKind::DatabaseError,
                "DeviceCode",
                format!("Failed to read status: {}", e),
            )
        })?;

        let status = DeviceCodeStatus::from_str(&status_str).unwrap_or(DeviceCodeStatus::Expired);

        Ok(DeviceCode::from_raw(
            row.try_get("id").unwrap_or_default(),
            row.try_get("device_code").unwrap_or_default(),
            row.try_get("user_code").unwrap_or_default(),
            row.try_get("client_name").unwrap_or_default(),
            row.try_get("scopes").unwrap_or_default(),
            status,
            row.try_get("user_id").ok(),
            row.try_get("access_token").ok(),
            row.try_get("refresh_token").ok(),
            row.try_get("verification_uri").unwrap_or_default(),
            row.try_get("verification_uri_complete").ok(),
            row.try_get("expires_at").unwrap_or_default(),
            row.try_get::<i32, _>("poll_interval_secs").unwrap_or(5),
            row.try_get("last_poll_at").ok(),
            row.try_get("created_at").unwrap_or_default(),
            row.try_get("authorized_at").ok(),
        ))
    }
}

#[async_trait]
impl DeviceCodeStoragePort for DeviceCodePgRepository {
    async fn create_device_code(&self, dc: DeviceCode) -> Result<DeviceCode, DomainError> {
        sqlx::query(
            r#"
            INSERT INTO auth.device_codes (
                id, device_code, user_code, client_name, scopes, status,
                user_id, access_token, refresh_token,
                verification_uri, verification_uri_complete,
                expires_at, poll_interval_secs, last_poll_at,
                created_at, authorized_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6::auth.device_code_status,
                $7, $8, $9,
                $10, $11,
                $12, $13, $14,
                $15, $16
            )
            "#,
        )
        .bind(dc.id())
        .bind(dc.device_code())
        .bind(dc.user_code())
        .bind(dc.client_name())
        .bind(dc.scopes())
        .bind(dc.status().as_str())
        .bind(dc.user_id())
        .bind(dc.access_token())
        .bind(dc.refresh_token())
        .bind(dc.verification_uri())
        .bind(dc.verification_uri_complete())
        .bind(dc.expires_at())
        .bind(dc.poll_interval_secs())
        .bind(dc.last_poll_at())
        .bind(dc.created_at())
        .bind(dc.authorized_at())
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| {
            DomainError::new(
                ErrorKind::DatabaseError,
                "DeviceCode",
                format!("Failed to create device code: {}", e),
            )
        })?;

        Ok(dc)
    }

    async fn get_by_device_code(&self, device_code: &str) -> Result<DeviceCode, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, device_code, user_code, client_name, scopes,
                   status::text AS status, user_id, access_token, refresh_token,
                   verification_uri, verification_uri_complete,
                   expires_at, poll_interval_secs, last_poll_at,
                   created_at, authorized_at
            FROM auth.device_codes
            WHERE device_code = $1
            "#,
        )
        .bind(device_code)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DomainError::new(
                ErrorKind::NotFound,
                "DeviceCode",
                "Device code not found",
            ),
            _ => DomainError::new(
                ErrorKind::DatabaseError,
                "DeviceCode",
                format!("Failed to fetch device code: {}", e),
            ),
        })?;

        Self::map_row(&row)
    }

    async fn get_pending_by_user_code(&self, user_code: &str) -> Result<DeviceCode, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, device_code, user_code, client_name, scopes,
                   status::text AS status, user_id, access_token, refresh_token,
                   verification_uri, verification_uri_complete,
                   expires_at, poll_interval_secs, last_poll_at,
                   created_at, authorized_at
            FROM auth.device_codes
            WHERE user_code = $1
              AND status = 'pending'
              AND expires_at > NOW()
            "#,
        )
        .bind(user_code)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DomainError::new(
                ErrorKind::NotFound,
                "DeviceCode",
                "User code not found or expired",
            ),
            _ => DomainError::new(
                ErrorKind::DatabaseError,
                "DeviceCode",
                format!("Failed to fetch by user code: {}", e),
            ),
        })?;

        Self::map_row(&row)
    }

    async fn update_device_code(&self, dc: DeviceCode) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            UPDATE auth.device_codes SET
                status = $2::auth.device_code_status,
                user_id = $3,
                access_token = $4,
                refresh_token = $5,
                last_poll_at = $6,
                authorized_at = $7
            WHERE id = $1
            "#,
        )
        .bind(dc.id())
        .bind(dc.status().as_str())
        .bind(dc.user_id())
        .bind(dc.access_token())
        .bind(dc.refresh_token())
        .bind(dc.last_poll_at())
        .bind(dc.authorized_at())
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| {
            DomainError::new(
                ErrorKind::DatabaseError,
                "DeviceCode",
                format!("Failed to update device code: {}", e),
            )
        })?;

        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64, DomainError> {
        let result = sqlx::query(
            r#"
            DELETE FROM auth.device_codes
            WHERE expires_at < NOW()
              AND status IN ('pending', 'expired')
            "#,
        )
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| {
            DomainError::new(
                ErrorKind::DatabaseError,
                "DeviceCode",
                format!("Failed to delete expired device codes: {}", e),
            )
        })?;

        Ok(result.rows_affected())
    }

    async fn list_by_user(&self, user_id: &str) -> Result<Vec<DeviceCode>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, device_code, user_code, client_name, scopes,
                   status::text AS status, user_id, access_token, refresh_token,
                   verification_uri, verification_uri_complete,
                   expires_at, poll_interval_secs, last_poll_at,
                   created_at, authorized_at
            FROM auth.device_codes
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| {
            DomainError::new(
                ErrorKind::DatabaseError,
                "DeviceCode",
                format!("Failed to list device codes: {}", e),
            )
        })?;

        rows.iter().map(Self::map_row).collect()
    }

    async fn delete_by_id(&self, id: &str) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM auth.device_codes WHERE id = $1")
            .bind(id)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| {
                DomainError::new(
                    ErrorKind::DatabaseError,
                    "DeviceCode",
                    format!("Failed to delete device code: {}", e),
                )
            })?;

        Ok(())
    }
}
