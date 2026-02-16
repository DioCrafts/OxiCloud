use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

use crate::application::dtos::favorites_dto::FavoriteItemDto;
use crate::application::ports::favorites_ports::FavoritesRepositoryPort;
use crate::common::errors::{DomainError, ErrorKind, Result};

/// PostgreSQL implementation of the favorites persistence port.
pub struct FavoritesPgRepository {
    db_pool: Arc<PgPool>,
}

impl FavoritesPgRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl FavoritesRepositoryPort for FavoritesPgRepository {
    async fn get_favorites(&self, user_id: &str) -> Result<Vec<FavoriteItemDto>> {
        let user_uuid = Uuid::parse_str(user_id)?;

        let rows = sqlx::query(
            r#"
            SELECT
                uf.id::TEXT                                     AS "id",
                uf.user_id::TEXT                                AS "user_id",
                uf.item_id                                      AS "item_id",
                uf.item_type                                    AS "item_type",
                uf.created_at                                   AS "created_at",
                COALESCE(f.name, fld.name)                      AS "item_name",
                f.size                                          AS "item_size",
                f.mime_type                                     AS "item_mime_type",
                COALESCE(f.folder_id::TEXT, fld.parent_id::TEXT) AS "parent_id",
                COALESCE(f.updated_at, fld.updated_at)          AS "modified_at"
            FROM auth.user_favorites uf
            LEFT JOIN storage.files   f   ON uf.item_type = 'file'
                                         AND uf.item_id = f.id::TEXT
            LEFT JOIN storage.folders fld ON uf.item_type = 'folder'
                                         AND uf.item_id = fld.id::TEXT
            WHERE uf.user_id = $1::TEXT
            ORDER BY uf.created_at DESC
            "#,
        )
        .bind(user_uuid)
        .fetch_all(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database error fetching favorites: {}", e);
            DomainError::new(
                ErrorKind::InternalError,
                "Favorites",
                format!("Failed to fetch favorites: {}", e),
            )
        })?;

        let favorites = rows
            .iter()
            .map(|row| FavoriteItemDto {
                id: row.get("id"),
                user_id: row.get("user_id"),
                item_id: row.get("item_id"),
                item_type: row.get("item_type"),
                created_at: row.get("created_at"),
                item_name: row.try_get("item_name").ok(),
                item_size: row.try_get("item_size").ok(),
                item_mime_type: row.try_get("item_mime_type").ok(),
                parent_id: row.try_get("parent_id").ok(),
                modified_at: row.try_get("modified_at").ok(),
                // Temporary defaults; with_display_fields() computes the real values
                icon_class: String::new(),
                icon_special_class: String::new(),
                category: String::new(),
                size_formatted: String::new(),
            }.with_display_fields())
            .collect();

        Ok(favorites)
    }

    async fn add_favorite(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<()> {
        let user_uuid = Uuid::parse_str(user_id)?;

        sqlx::query(
            r#"
            INSERT INTO auth.user_favorites (user_id, item_id, item_type)
            VALUES ($1::TEXT, $2, $3)
            ON CONFLICT (user_id, item_id, item_type) DO NOTHING
            "#,
        )
        .bind(user_uuid)
        .bind(item_id)
        .bind(item_type)
        .execute(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database error adding favorite: {}", e);
            DomainError::new(
                ErrorKind::InternalError,
                "Favorites",
                format!("Failed to add to favorites: {}", e),
            )
        })?;

        Ok(())
    }

    async fn remove_favorite(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool> {
        let user_uuid = Uuid::parse_str(user_id)?;

        let result = sqlx::query(
            r#"
            DELETE FROM auth.user_favorites
            WHERE user_id = $1::TEXT AND item_id = $2 AND item_type = $3
            "#,
        )
        .bind(user_uuid)
        .bind(item_id)
        .bind(item_type)
        .execute(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database error removing favorite: {}", e);
            DomainError::new(
                ErrorKind::InternalError,
                "Favorites",
                format!("Failed to remove from favorites: {}", e),
            )
        })?;

        Ok(result.rows_affected() > 0)
    }

    async fn is_favorite(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool> {
        let user_uuid = Uuid::parse_str(user_id)?;

        let row = sqlx::query(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM auth.user_favorites
                WHERE user_id = $1::TEXT AND item_id = $2 AND item_type = $3
            ) AS "is_favorite"
            "#,
        )
        .bind(user_uuid)
        .bind(item_id)
        .bind(item_type)
        .fetch_one(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database error checking favorite status: {}", e);
            DomainError::new(
                ErrorKind::InternalError,
                "Favorites",
                format!("Failed to check favorite status: {}", e),
            )
        })?;

        Ok(row.try_get("is_favorite").unwrap_or(false))
    }
}
