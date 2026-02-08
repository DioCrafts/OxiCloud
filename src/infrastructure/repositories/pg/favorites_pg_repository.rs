use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use tracing::error;
use uuid::Uuid;

use crate::application::dtos::favorites_dto::FavoriteItemDto;
use crate::application::ports::favorites_ports::FavoritesRepositoryPort;
use crate::common::errors::{Result, DomainError, ErrorKind};

/// Implementación PostgreSQL del puerto de persistencia de favoritos.
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
                id::TEXT   AS "id",
                user_id::TEXT AS "user_id",
                item_id    AS "item_id",
                item_type  AS "item_type",
                created_at AS "created_at"
            FROM auth.user_favorites
            WHERE user_id = $1::TEXT
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_uuid)
        .fetch_all(&*self.db_pool)
        .await
        .map_err(|e| {
            error!("Database error fetching favorites: {}", e);
            DomainError::new(ErrorKind::InternalError, "Favorites", format!("Failed to fetch favorites: {}", e))
        })?;

        let favorites = rows
            .iter()
            .map(|row| FavoriteItemDto {
                id: row.get("id"),
                user_id: row.get("user_id"),
                item_id: row.get("item_id"),
                item_type: row.get("item_type"),
                created_at: row.get("created_at"),
            })
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
            DomainError::new(ErrorKind::InternalError, "Favorites", format!("Failed to add to favorites: {}", e))
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
            DomainError::new(ErrorKind::InternalError, "Favorites", format!("Failed to remove from favorites: {}", e))
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
            DomainError::new(ErrorKind::InternalError, "Favorites", format!("Failed to check favorite status: {}", e))
        })?;

        Ok(row.try_get("is_favorite").unwrap_or(false))
    }
}
