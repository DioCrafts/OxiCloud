use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;

use crate::{
    application::ports::share_ports::ShareStoragePort,
    common::errors::DomainError,
    domain::entities::share::{Share, ShareItemType, SharePermissions},
};

/// PostgreSQL implementation of [`ShareStoragePort`].
///
/// Replaces the legacy file-based `ShareFsRepository` that read/wrote the
/// entire `shares.json` on every operation. Each method now issues a single
/// indexed SQL statement — O(1) lookups, ACID transactions, and no data-race
/// risk.
pub struct SharePgRepository {
    db_pool: Arc<PgPool>,
}

impl SharePgRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }

    /// Maps a [`sqlx::postgres::PgRow`] to the domain [`Share`] entity.
    fn row_to_entity(row: &sqlx::postgres::PgRow) -> Result<Share, DomainError> {
        let id: String = row.try_get("id").map_err(|e| {
            DomainError::internal_error("Share", format!("Failed to read id: {e}"))
        })?;
        let item_id: String = row.try_get("item_id").map_err(|e| {
            DomainError::internal_error("Share", format!("Failed to read item_id: {e}"))
        })?;
        let item_name: Option<String> = row.try_get("item_name").unwrap_or(None);
        let item_type_str: String = row.try_get("item_type").map_err(|e| {
            DomainError::internal_error("Share", format!("Failed to read item_type: {e}"))
        })?;
        let token: String = row.try_get("token").map_err(|e| {
            DomainError::internal_error("Share", format!("Failed to read token: {e}"))
        })?;
        let password_hash: Option<String> = row.try_get("password_hash").unwrap_or(None);
        let expires_at: Option<i64> = row.try_get("expires_at").unwrap_or(None);
        let permissions_read: bool = row.try_get("permissions_read").unwrap_or(true);
        let permissions_write: bool = row.try_get("permissions_write").unwrap_or(false);
        let permissions_reshare: bool = row.try_get("permissions_reshare").unwrap_or(false);
        let created_at: i64 = row.try_get("created_at").map_err(|e| {
            DomainError::internal_error("Share", format!("Failed to read created_at: {e}"))
        })?;
        let created_by: String = row.try_get("created_by").map_err(|e| {
            DomainError::internal_error("Share", format!("Failed to read created_by: {e}"))
        })?;
        let access_count: i64 = row.try_get("access_count").unwrap_or(0);

        let item_type =
            ShareItemType::try_from(item_type_str.as_str()).unwrap_or(ShareItemType::File);
        let permissions = SharePermissions::new(permissions_read, permissions_write, permissions_reshare);

        Ok(Share::from_raw(
            id,
            item_id,
            item_name,
            item_type,
            token,
            password_hash,
            expires_at.map(|v| v as u64),
            permissions,
            created_at as u64,
            created_by,
            access_count as u64,
        ))
    }
}

#[async_trait]
impl ShareStoragePort for SharePgRepository {
    async fn save_share(&self, share: &Share) -> Result<Share, DomainError> {
        let row = sqlx::query(
            r#"
            INSERT INTO storage.shares
                (id, item_id, item_name, item_type, token, password_hash,
                 expires_at, permissions_read, permissions_write, permissions_reshare,
                 created_at, created_by, access_count)
            VALUES
                ($1::UUID, $2, $3, $4, $5, $6,
                 $7, $8, $9, $10,
                 $11, $12, $13)
            ON CONFLICT (id) DO UPDATE SET
                item_name         = EXCLUDED.item_name,
                password_hash     = EXCLUDED.password_hash,
                expires_at        = EXCLUDED.expires_at,
                permissions_read  = EXCLUDED.permissions_read,
                permissions_write = EXCLUDED.permissions_write,
                permissions_reshare = EXCLUDED.permissions_reshare,
                access_count      = EXCLUDED.access_count
            RETURNING
                id::TEXT, item_id, item_name, item_type, token, password_hash,
                expires_at, permissions_read, permissions_write, permissions_reshare,
                created_at, created_by, access_count
            "#,
        )
        .bind(share.id())
        .bind(share.item_id())
        .bind(share.item_name())
        .bind(share.item_type().to_string())
        .bind(share.token())
        .bind(share.password_hash())
        .bind(share.expires_at().map(|v| v as i64))
        .bind(share.permissions().read())
        .bind(share.permissions().write())
        .bind(share.permissions().reshare())
        .bind(share.created_at() as i64)
        .bind(share.created_by())
        .bind(share.access_count() as i64)
        .fetch_one(&*self.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error saving share: {}", e);
            DomainError::internal_error("Share", format!("Failed to save share: {e}"))
        })?;

        Self::row_to_entity(&row)
    }

    async fn find_share_by_id(&self, id: &str) -> Result<Share, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id::TEXT, item_id, item_name, item_type, token, password_hash,
                   expires_at, permissions_read, permissions_write, permissions_reshare,
                   created_at, created_by, access_count
            FROM storage.shares
            WHERE id = $1::UUID
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error finding share by id: {}", e);
            DomainError::internal_error("Share", format!("Failed to find share: {e}"))
        })?;

        match row {
            Some(r) => Self::row_to_entity(&r),
            None => Err(DomainError::not_found(
                "Share",
                format!("Share with ID {id} not found"),
            )),
        }
    }

    async fn find_share_by_token(&self, token: &str) -> Result<Share, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id::TEXT, item_id, item_name, item_type, token, password_hash,
                   expires_at, permissions_read, permissions_write, permissions_reshare,
                   created_at, created_by, access_count
            FROM storage.shares
            WHERE token = $1
            "#,
        )
        .bind(token)
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error finding share by token: {}", e);
            DomainError::internal_error("Share", format!("Failed to find share by token: {e}"))
        })?;

        match row {
            Some(r) => Self::row_to_entity(&r),
            None => Err(DomainError::not_found(
                "Share",
                format!("Share with token {token} not found"),
            )),
        }
    }

    async fn find_shares_by_item(
        &self,
        item_id: &str,
        item_type: &ShareItemType,
    ) -> Result<Vec<Share>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id::TEXT, item_id, item_name, item_type, token, password_hash,
                   expires_at, permissions_read, permissions_write, permissions_reshare,
                   created_at, created_by, access_count
            FROM storage.shares
            WHERE item_id = $1 AND item_type = $2
            ORDER BY created_at DESC
            "#,
        )
        .bind(item_id)
        .bind(item_type.to_string())
        .fetch_all(&*self.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error finding shares by item: {}", e);
            DomainError::internal_error("Share", format!("Failed to find shares by item: {e}"))
        })?;

        rows.iter().map(Self::row_to_entity).collect()
    }

    async fn update_share(&self, share: &Share) -> Result<Share, DomainError> {
        let row = sqlx::query(
            r#"
            UPDATE storage.shares SET
                item_name         = $2,
                password_hash     = $3,
                expires_at        = $4,
                permissions_read  = $5,
                permissions_write = $6,
                permissions_reshare = $7,
                access_count      = $8
            WHERE id = $1::UUID
            RETURNING
                id::TEXT, item_id, item_name, item_type, token, password_hash,
                expires_at, permissions_read, permissions_write, permissions_reshare,
                created_at, created_by, access_count
            "#,
        )
        .bind(share.id())
        .bind(share.item_name())
        .bind(share.password_hash())
        .bind(share.expires_at().map(|v| v as i64))
        .bind(share.permissions().read())
        .bind(share.permissions().write())
        .bind(share.permissions().reshare())
        .bind(share.access_count() as i64)
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error updating share: {}", e);
            DomainError::internal_error("Share", format!("Failed to update share: {e}"))
        })?;

        match row {
            Some(r) => Self::row_to_entity(&r),
            None => Err(DomainError::not_found(
                "Share",
                format!("Share with ID {} not found for update", share.id()),
            )),
        }
    }

    async fn delete_share(&self, id: &str) -> Result<(), DomainError> {
        let result = sqlx::query(
            "DELETE FROM storage.shares WHERE id = $1::UUID",
        )
        .bind(id)
        .execute(&*self.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error deleting share: {}", e);
            DomainError::internal_error("Share", format!("Failed to delete share: {e}"))
        })?;

        if result.rows_affected() == 0 {
            return Err(DomainError::not_found(
                "Share",
                format!("Share with ID {id} not found for deletion"),
            ));
        }

        Ok(())
    }

    async fn find_shares_by_user(
        &self,
        user_id: &str,
        offset: usize,
        limit: usize,
    ) -> Result<(Vec<Share>, usize), DomainError> {
        // Single query with window function — count + rows in one roundtrip
        let rows = sqlx::query(
            r#"
            SELECT id::TEXT, item_id, item_name, item_type, token, password_hash,
                   expires_at, permissions_read, permissions_write, permissions_reshare,
                   created_at, created_by, access_count,
                   COUNT(*) OVER() AS total_count
            FROM storage.shares
            WHERE created_by = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&*self.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error finding shares by user: {}", e);
            DomainError::internal_error("Share", format!("Failed to find shares by user: {e}"))
        })?;

        let total: usize = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0) as usize;

        let shares: Result<Vec<Share>, DomainError> =
            rows.iter().map(Self::row_to_entity).collect();

        Ok((shares?, total))
    }
}
