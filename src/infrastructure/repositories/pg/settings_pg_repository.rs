use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::repositories::settings_repository::SettingsRepository;
use crate::common::errors::{DomainError, ErrorKind};

pub struct SettingsPgRepository {
    pool: Arc<PgPool>,
}

impl SettingsPgRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingsRepository for SettingsPgRepository {
    async fn get(&self, key: &str) -> Result<Option<String>, DomainError> {
        let row = sqlx::query_scalar::<_, String>(
            "SELECT value FROM auth.admin_settings WHERE key = $1"
        )
        .bind(key)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::new(
            ErrorKind::InternalError, "Settings", format!("DB error: {}", e),
        ))?;

        Ok(row)
    }

    async fn get_by_category(&self, category: &str) -> Result<HashMap<String, String>, DomainError> {
        let rows = sqlx::query_as::<_, (String, String)>(
            "SELECT key, value FROM auth.admin_settings WHERE category = $1"
        )
        .bind(category)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::new(
            ErrorKind::InternalError, "Settings", format!("DB error: {}", e),
        ))?;

        Ok(rows.into_iter().collect())
    }

    async fn set(
        &self,
        key: &str,
        value: &str,
        category: &str,
        is_secret: bool,
        updated_by: Option<&str>,
    ) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO auth.admin_settings (key, value, category, is_secret, updated_by, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW())
             ON CONFLICT (key) DO UPDATE
             SET value = $2, category = $3, is_secret = $4, updated_by = $5, updated_at = NOW()"
        )
        .bind(key)
        .bind(value)
        .bind(category)
        .bind(is_secret)
        .bind(updated_by)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::new(
            ErrorKind::InternalError, "Settings", format!("DB error: {}", e),
        ))?;

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM auth.admin_settings WHERE key = $1")
            .bind(key)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| DomainError::new(
                ErrorKind::InternalError, "Settings", format!("DB error: {}", e),
            ))?;

        Ok(())
    }
}
