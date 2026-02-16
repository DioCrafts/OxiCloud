use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

use crate::application::dtos::recent_dto::RecentItemDto;
use crate::application::ports::recent_ports::RecentItemsRepositoryPort;
use crate::common::errors::{DomainError, ErrorKind, Result};

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
                ur.id::TEXT                                     AS "id",
                ur.user_id::TEXT                                AS "user_id",
                ur.item_id                                      AS "item_id",
                ur.item_type                                    AS "item_type",
                ur.accessed_at                                  AS "accessed_at",
                COALESCE(f.name, fld.name)                      AS "item_name",
                f.size                                          AS "item_size",
                f.mime_type                                     AS "item_mime_type",
                COALESCE(f.folder_id::TEXT, fld.parent_id::TEXT) AS "parent_id"
            FROM auth.user_recent_files ur
            LEFT JOIN storage.files   f   ON ur.item_type = 'file'
                                         AND ur.item_id = f.id::TEXT
            LEFT JOIN storage.folders fld ON ur.item_type = 'folder'
                                         AND ur.item_id = fld.id::TEXT
            WHERE ur.user_id = $1::TEXT
            ORDER BY ur.accessed_at DESC
            LIMIT $2
            "#,
        )
        .bind(user_uuid)
        .bind(limit)
        .fetch_all(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database error fetching recent items: {}", e);
            DomainError::new(
                ErrorKind::InternalError,
                "RecentItems",
                format!("Failed to fetch recent items: {}", e),
            )
        })?;

        let items = rows
            .iter()
            .map(|row| RecentItemDto {
                id: row.get("id"),
                user_id: row.get("user_id"),
                item_id: row.get("item_id"),
                item_type: row.get("item_type"),
                accessed_at: row.get("accessed_at"),
                item_name: row.try_get("item_name").ok(),
                item_size: row.try_get("item_size").ok(),
                item_mime_type: row.try_get("item_mime_type").ok(),
                parent_id: row.try_get("parent_id").ok(),
                // Temporary defaults; with_display_fields() computes the real values
                icon_class: String::new(),
                icon_special_class: String::new(),
                category: String::new(),
                size_formatted: String::new(),
            }.with_display_fields())
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
            DomainError::new(
                ErrorKind::InternalError,
                "RecentItems",
                format!("Failed to record item access: {}", e),
            )
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
            DomainError::new(
                ErrorKind::InternalError,
                "RecentItems",
                format!("Failed to remove recent item: {}", e),
            )
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
            DomainError::new(
                ErrorKind::InternalError,
                "RecentItems",
                format!("Failed to clear recent items: {}", e),
            )
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
            DomainError::new(
                ErrorKind::InternalError,
                "RecentItems",
                format!("Failed to prune recent items: {}", e),
            )
        })?;

        Ok(())
    }
}
