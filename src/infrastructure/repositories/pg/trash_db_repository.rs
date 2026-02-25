//! PostgreSQL-backed trash repository.
//!
//! Implements `TrashRepository` using soft-delete columns in `storage.files`
//! and `storage.folders`.  There is no separate trash table — trashed items
//! are files/folders with `is_trashed = TRUE`.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::common::errors::{DomainError, Result};
use crate::domain::entities::trashed_item::{TrashedItem, TrashedItemType};
use crate::domain::repositories::trash_repository::TrashRepository;

/// Default retention period (days) used when computing deletion_date.
const _DEFAULT_RETENTION_DAYS: i64 = 30;

/// PostgreSQL-backed trash repository using soft-delete flags.
pub struct TrashDbRepository {
    pool: Arc<PgPool>,
    retention_days: i64,
}

impl TrashDbRepository {
    pub fn new(pool: Arc<PgPool>, retention_days: u32) -> Self {
        Self {
            pool,
            retention_days: retention_days as i64,
        }
    }

    /// Convert a trash_items view row into a TrashedItem entity.
    fn row_to_trashed_item(
        &self,
        id: Uuid,
        name: String,
        item_type: String,
        user_id: String,
        trashed_at: Option<DateTime<Utc>>,
    ) -> TrashedItem {
        let trashed_at = trashed_at.unwrap_or_else(Utc::now);
        let deletion_date = trashed_at + chrono::Duration::days(self.retention_days);

        let item_type_enum = match item_type.as_str() {
            "folder" => TrashedItemType::Folder,
            _ => TrashedItemType::File,
        };

        let user_uuid = Uuid::parse_str(&user_id).unwrap_or_else(|_| Uuid::nil());

        // In the soft-delete model, the trash entry ID is the same as the
        // original item ID since there is no separate trash table.
        TrashedItem::from_raw(
            id,        // trash entry id (same as original)
            id,        // original item id
            user_uuid, // owner
            item_type_enum,
            name.clone(),
            String::new(), // original_path — not stored separately in soft-delete model
            trashed_at,
            deletion_date,
        )
    }
}

#[async_trait]
impl TrashRepository for TrashDbRepository {
    async fn add_to_trash(&self, _item: &TrashedItem) -> Result<()> {
        // No-op: the actual flagging is done by FileWritePort::move_to_trash
        // or FolderRepository::move_to_trash.  This method exists for interface
        // compatibility with the TrashService.
        Ok(())
    }

    async fn get_trash_items(&self, user_id: &Uuid) -> Result<Vec<TrashedItem>> {
        let rows = sqlx::query_as::<_, (Uuid, String, String, String, Option<DateTime<Utc>>)>(
            r#"
            SELECT id, name, item_type, user_id, trashed_at
              FROM storage.trash_items
             WHERE user_id = $1
             ORDER BY trashed_at DESC
            "#,
        )
        .bind(user_id.to_string())
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("TrashDb", format!("list: {e}")))?;

        Ok(rows
            .into_iter()
            .map(|(id, name, item_type, uid, trashed_at)| {
                self.row_to_trashed_item(id, name, item_type, uid, trashed_at)
            })
            .collect())
    }

    async fn get_trash_item(&self, id: &Uuid, user_id: &Uuid) -> Result<Option<TrashedItem>> {
        let row = sqlx::query_as::<_, (Uuid, String, String, String, Option<DateTime<Utc>>)>(
            r#"
            SELECT id, name, item_type, user_id, trashed_at
              FROM storage.trash_items
             WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id.to_string())
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("TrashDb", format!("get: {e}")))?;

        Ok(row.map(|(id, name, item_type, uid, trashed_at)| {
            self.row_to_trashed_item(id, name, item_type, uid, trashed_at)
        }))
    }

    async fn restore_from_trash(&self, _id: &Uuid, _user_id: &Uuid) -> Result<()> {
        // No-op: the actual restore is done by FileWritePort::restore_from_trash
        // or FolderRepository::restore_from_trash.  The TrashService also removes
        // the index entry — which in the soft-delete model means the flag is
        // already cleared.
        Ok(())
    }

    async fn delete_permanently(&self, _id: &Uuid, _user_id: &Uuid) -> Result<()> {
        // No-op: the actual delete is done by FileWritePort::delete_file_permanently
        // or FolderRepository::delete_folder_permanently.
        Ok(())
    }

    async fn clear_trash(&self, user_id: &Uuid) -> Result<()> {
        // Delete all trashed files for this user
        sqlx::query("DELETE FROM storage.files WHERE user_id = $1 AND is_trashed = TRUE")
            .bind(user_id.to_string())
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| DomainError::internal_error("TrashDb", format!("clear files: {e}")))?;

        // Delete all trashed folders for this user
        sqlx::query("DELETE FROM storage.folders WHERE user_id = $1 AND is_trashed = TRUE")
            .bind(user_id.to_string())
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| DomainError::internal_error("TrashDb", format!("clear folders: {e}")))?;

        Ok(())
    }

    async fn delete_expired_bulk(&self) -> Result<(u64, u64)> {
        let cutoff = Utc::now() - chrono::Duration::days(self.retention_days);

        let mut tx = self.pool.begin().await.map_err(|e| {
            DomainError::internal_error("TrashDb", format!("begin tx: {e}"))
        })?;

        // 1. Bulk-delete expired trashed files.
        //    The PG trigger `trg_files_decrement_blob_ref` automatically
        //    decrements blob ref_count for every deleted row.
        let files_deleted = sqlx::query(
            "DELETE FROM storage.files WHERE is_trashed = TRUE AND trashed_at < $1",
        )
        .bind(cutoff)
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::internal_error("TrashDb", format!("bulk delete files: {e}")))?
        .rows_affected();

        // 2. Bulk-delete expired trashed folders.
        //    FK ON DELETE CASCADE handles descendant folders and their files.
        let folders_deleted = sqlx::query(
            "DELETE FROM storage.folders WHERE is_trashed = TRUE AND trashed_at < $1",
        )
        .bind(cutoff)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            DomainError::internal_error("TrashDb", format!("bulk delete folders: {e}"))
        })?
        .rows_affected();

        tx.commit().await.map_err(|e| {
            DomainError::internal_error("TrashDb", format!("commit tx: {e}"))
        })?;

        Ok((files_deleted, folders_deleted))
    }
}
