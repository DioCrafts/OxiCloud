use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use tracing::error;
use uuid::Uuid;

use crate::application::dtos::recent_dto::RecentItemDto;
use crate::application::ports::recent_ports::RecentItemsRepositoryPort;
use crate::common::errors::{Result, DomainError, ErrorKind};

/// PostgreSQL implementation of the recent items persistence port.
pub struct RecentItemsPgRepository {
    db_pool: Arc<PgPool>,
}

impl RecentItemsPgRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl RecentItemsRepositoryPort for RecentItemsPgRepository {
    async fn get_recent_items(&self, user_id: &str, limit: i32) -> Result<Vec<RecentItemDto>> {
        let user_uuid = Uuid::parse_str(user_id)?;

        let rows = sqlx::query(
            r#"
            SELECT
                id::TEXT      AS "id",
                user_id::TEXT AS "user_id",
                item_id       AS "item_id",
                item_type     AS "item_type",
                accessed_at   AS "accessed_at"
            FROM auth.user_recent_files
            WHERE user_id = $1::TEXT
            ORDER BY accessed_at DESC
            LIMIT $2
            "#,
        )
        .bind(user_uuid)
        .bind(limit)
        .fetch_all(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database error fetching recent items: {}", e);
            DomainError::new(ErrorKind::InternalError, "RecentItems", format!("Failed to fetch recent items: {}", e))
        })?;

        let items = rows
            .iter()
            .map(|row| RecentItemDto {
                id: row.get("id"),
                user_id: row.get("user_id"),
                item_id: row.get("item_id"),
                item_type: row.get("item_type"),
                accessed_at: row.get("accessed_at"),
            })
            .collect();

        Ok(items)
    }

    async fn upsert_access(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<()> {
        let user_uuid = Uuid::parse_str(user_id)?;

        sqlx::query(
            r#"
            INSERT INTO auth.user_recent_files (user_id, item_id, item_type, accessed_at)
            VALUES ($1::TEXT, $2, $3, CURRENT_TIMESTAMP)
            ON CONFLICT (user_id, item_id, item_type)
            DO UPDATE SET accessed_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(user_uuid)
        .bind(item_id)
        .bind(item_type)
        .execute(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database error upserting recent item access: {}", e);
            DomainError::new(ErrorKind::InternalError, "RecentItems", format!("Failed to record item access: {}", e))
        })?;

        Ok(())
    }

    async fn remove_item(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool> {
        let user_uuid = Uuid::parse_str(user_id)?;

        let result = sqlx::query(
            r#"
            DELETE FROM auth.user_recent_files
            WHERE user_id = $1::TEXT AND item_id = $2 AND item_type = $3
            "#,
        )
        .bind(user_uuid)
        .bind(item_id)
        .bind(item_type)
        .execute(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database error removing recent item: {}", e);
            DomainError::new(ErrorKind::InternalError, "RecentItems", format!("Failed to remove recent item: {}", e))
        })?;

        Ok(result.rows_affected() > 0)
    }

    async fn clear_all(&self, user_id: &str) -> Result<()> {
        let user_uuid = Uuid::parse_str(user_id)?;

        sqlx::query(
            r#"
            DELETE FROM auth.user_recent_files
            WHERE user_id = $1::TEXT
            "#,
        )
        .bind(user_uuid)
        .execute(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database error clearing recent items: {}", e);
            DomainError::new(ErrorKind::InternalError, "RecentItems", format!("Failed to clear recent items: {}", e))
        })?;

        Ok(())
    }

    async fn prune(&self, user_id: &str, max_items: i32) -> Result<()> {
        let user_uuid = Uuid::parse_str(user_id)?;

        sqlx::query(
            r#"
            DELETE FROM auth.user_recent_files
            WHERE id IN (
                SELECT id FROM auth.user_recent_files
                WHERE user_id = $1::TEXT
                ORDER BY accessed_at DESC
                OFFSET $2
            )
            "#,
        )
        .bind(user_uuid)
        .bind(max_items)
        .execute(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database error pruning old recent items: {}", e);
            DomainError::new(ErrorKind::InternalError, "RecentItems", format!("Failed to prune recent items: {}", e))
        })?;

        Ok(())
    }
}
