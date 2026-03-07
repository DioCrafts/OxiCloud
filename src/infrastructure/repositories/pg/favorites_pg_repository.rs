use sqlx::{PgPool, Row};
use std::collections::HashSet;
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

impl FavoritesRepositoryPort for FavoritesPgRepository {
    async fn get_favorites(&self, user_id: Uuid) -> Result<Vec<FavoriteItemDto>> {
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
                                         AND f.id = uf.item_id::UUID
            LEFT JOIN storage.folders fld ON uf.item_type = 'folder'
                                         AND fld.id = uf.item_id::UUID
            WHERE uf.user_id = $1
            ORDER BY uf.created_at DESC
            LIMIT 500
            "#,
        )
        .bind(user_id)
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
            .map(|row| {
                FavoriteItemDto {
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
                }
                .with_display_fields()
            })
            .collect();

        Ok(favorites)
    }

    async fn add_favorite(&self, user_id: Uuid, item_id: &str, item_type: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO auth.user_favorites (user_id, item_id, item_type)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, item_id, item_type) DO NOTHING
            "#,
        )
        .bind(user_id)
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

    async fn remove_favorite(&self, user_id: Uuid, item_id: &str, item_type: &str) -> Result<bool> {
        let result = sqlx::query(
            r#"
            DELETE FROM auth.user_favorites
            WHERE user_id = $1 AND item_id = $2 AND item_type = $3
            "#,
        )
        .bind(user_id)
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

    async fn is_favorite(&self, user_id: Uuid, item_id: &str, item_type: &str) -> Result<bool> {
        let row = sqlx::query(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM auth.user_favorites
                WHERE user_id = $1 AND item_id = $2 AND item_type = $3
            ) AS "is_favorite"
            "#,
        )
        .bind(user_id)
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

    async fn add_favorites_batch(&self, user_id: Uuid, items: &[(String, String)]) -> Result<u64> {
        if items.is_empty() {
            return Ok(0);
        }

        // Validate all item_types upfront
        for (_, item_type) in items {
            if item_type != "file" && item_type != "folder" {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Favorites",
                    format!("Item type must be 'file' or 'folder', got '{}'", item_type),
                ));
            }
        }

        // Build a multi-row INSERT with ON CONFLICT DO NOTHING
        // Using a single transaction for atomicity
        let mut tx = self.db_pool.begin().await.map_err(|e| {
            error!("Database error starting transaction: {}", e);
            DomainError::new(
                ErrorKind::InternalError,
                "Favorites",
                format!("Failed to start transaction: {}", e),
            )
        })?;

        let mut total_inserted: u64 = 0;

        // Insert in chunks to stay within Postgres' parameter limit (max ~32k params)
        for chunk in items.chunks(5000) {
            let mut query = String::from(
                "INSERT INTO auth.user_favorites (user_id, item_id, item_type) VALUES ",
            );
            let mut param_idx = 1u32;
            let mut first = true;

            for _ in chunk {
                if !first {
                    query.push_str(", ");
                }
                query.push_str(&format!(
                    "(${}, ${}, ${})",
                    param_idx,
                    param_idx + 1,
                    param_idx + 2
                ));
                param_idx += 3;
                first = false;
            }
            query.push_str(" ON CONFLICT (user_id, item_id, item_type) DO NOTHING");

            let mut q = sqlx::query(&query);
            for (item_id, item_type) in chunk {
                q = q.bind(user_id).bind(item_id).bind(item_type);
            }

            let result = q.execute(&mut *tx).await.map_err(|e| {
                error!("Database error in batch insert favorites: {}", e);
                DomainError::new(
                    ErrorKind::InternalError,
                    "Favorites",
                    format!("Failed to batch insert favorites: {}", e),
                )
            })?;

            total_inserted += result.rows_affected();
        }

        tx.commit().await.map_err(|e| {
            error!("Database error committing batch favorites: {}", e);
            DomainError::new(
                ErrorKind::InternalError,
                "Favorites",
                format!("Failed to commit batch favorites: {}", e),
            )
        })?;

        Ok(total_inserted)
    }

    async fn batch_check_favorites(
        &self,
        user_id: Uuid,
        item_ids: &[(&str, &str)],
    ) -> Result<HashSet<String>> {
        if item_ids.is_empty() {
            return Ok(HashSet::new());
        }

        // Collect just the IDs for the IN clause
        let ids: Vec<String> = item_ids.iter().map(|(id, _)| id.to_string()).collect();

        let rows = sqlx::query(
            "SELECT item_id FROM auth.user_favorites WHERE user_id = $1 AND item_id = ANY($2)",
        )
        .bind(user_id)
        .bind(&ids)
        .fetch_all(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database error batch-checking favorites: {}", e);
            DomainError::new(
                ErrorKind::InternalError,
                "Favorites",
                format!("Failed to batch-check favorites: {}", e),
            )
        })?;

        Ok(rows.iter().map(|r| r.get::<String, _>("item_id")).collect())
    }
}
