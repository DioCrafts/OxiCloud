//! PostgreSQL-backed folder repository.
//!
//! Implements `FolderRepository` (and thus `FolderStoragePort`) using the
//! `storage.folders` table.  Folders are purely virtual — no physical
//! directories are created on the filesystem.
//!
//! Folder paths are **materialized** in a `path TEXT` column maintained by
//! database triggers, so reading a folder's full path is always O(1) — no
//! recursive CTEs or N+1 queries.

use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use crate::common::errors::DomainError;
use crate::domain::entities::folder::Folder;
use crate::domain::repositories::folder_repository::FolderRepository;
use crate::domain::services::path_service::StoragePath;

/// PostgreSQL-backed folder repository.
///
/// All folder metadata lives in the `storage.folders` table.  The physical
/// filesystem is never touched for folder operations.
pub struct FolderDbRepository {
    pool: Option<Arc<PgPool>>,
}

impl FolderDbRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool: Some(pool) }
    }

    /// Creates a stub instance for `AppState::default()`.
    /// This is never called in production — only used for route scaffolding.
    pub fn new_stub() -> Self {
        Self { pool: None }
    }

    /// Get the pool, panicking if stub.
    fn pool(&self) -> &PgPool {
        self.pool
            .as_deref()
            .expect("FolderDbRepository: pool not available (stub instance)")
    }

    // ── helpers ──────────────────────────────────────────────────

    /// Convert a database row into a `Folder` domain entity.
    ///
    /// The `path` comes directly from the materialized `path` column — no
    /// extra queries needed.
    fn row_to_folder(
        id: String,
        name: String,
        path: String,
        parent_id: Option<String>,
        user_id: Option<String>,
        created_at: i64,
        modified_at: i64,
    ) -> Result<Folder, DomainError> {
        let storage_path = StoragePath::from_string(&path);
        Folder::with_timestamps_and_owner(
            id,
            name,
            storage_path,
            parent_id,
            user_id,
            created_at as u64,
            modified_at as u64,
        )
        .map_err(|e| DomainError::internal_error("FolderDb", format!("entity: {e}")))
    }
}

#[async_trait]
impl FolderRepository for FolderDbRepository {
    async fn create_folder(
        &self,
        name: String,
        parent_id: Option<String>,
    ) -> Result<Folder, DomainError> {
        // Derive user_id from parent folder.  Root-level folders require the
        // caller to have set up the home folder beforehand (done during user
        // registration).
        let user_id: String = if let Some(ref pid) = parent_id {
            sqlx::query_scalar::<_, String>(
                "SELECT user_id FROM storage.folders WHERE id = $1::uuid",
            )
            .bind(pid)
            .fetch_optional(self.pool())
            .await
            .map_err(|e| DomainError::internal_error("FolderDb", format!("parent lookup: {e}")))?
            .ok_or_else(|| DomainError::not_found("Folder", pid))?
        } else {
            return Err(DomainError::internal_error(
                "FolderDb",
                "Cannot create root folder without user_id — use create_home_folder instead",
            ));
        };

        let row = sqlx::query_as::<_, (String, String, i64, i64)>(
            r#"
            INSERT INTO storage.folders (name, parent_id, user_id)
            VALUES ($1, $2::uuid, $3)
            RETURNING id::text,
                      path,
                      EXTRACT(EPOCH FROM created_at)::bigint,
                      EXTRACT(EPOCH FROM updated_at)::bigint
            "#,
        )
        .bind(&name)
        .bind(&parent_id)
        .bind(&user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.code().as_deref() == Some("23505")
            {
                return DomainError::already_exists(
                    "Folder",
                    format!("{name} already exists in parent"),
                );
            }
            DomainError::internal_error("FolderDb", format!("insert: {e}"))
        })?;

        Self::row_to_folder(row.0, name, row.1, parent_id, Some(user_id), row.2, row.3)
    }

    async fn get_folder(&self, id: &str) -> Result<Folder, DomainError> {
        let row = sqlx::query_as::<_, (String, String, String, Option<String>, String, i64, i64)>(
            r#"
            SELECT id::text, name, path, parent_id::text, user_id,
                   EXTRACT(EPOCH FROM created_at)::bigint,
                   EXTRACT(EPOCH FROM updated_at)::bigint
              FROM storage.folders
             WHERE id = $1::uuid AND NOT is_trashed
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| DomainError::internal_error("FolderDb", format!("get: {e}")))?
        .ok_or_else(|| DomainError::not_found("Folder", id))?;

        Self::row_to_folder(row.0, row.1, row.2, row.3, Some(row.4), row.5, row.6)
    }

    async fn get_folder_by_path(&self, storage_path: &StoragePath) -> Result<Folder, DomainError> {
        let path_str = storage_path.to_string();
        // Strip leading '/' if present — DB stores "Home - user/Docs", not "/Home - user/Docs"
        let lookup = path_str.strip_prefix('/').unwrap_or(&path_str);

        if lookup.is_empty() {
            return Err(DomainError::not_found("Folder", "empty path"));
        }

        let row = sqlx::query_as::<_, (String, String, String, Option<String>, String, i64, i64)>(
            r#"
            SELECT id::text, name, path, parent_id::text, user_id,
                   EXTRACT(EPOCH FROM created_at)::bigint,
                   EXTRACT(EPOCH FROM updated_at)::bigint
              FROM storage.folders
             WHERE path = $1 AND NOT is_trashed
            "#,
        )
        .bind(lookup)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| DomainError::internal_error("FolderDb", format!("path lookup: {e}")))?
        .ok_or_else(|| DomainError::not_found("Folder", lookup))?;

        Self::row_to_folder(row.0, row.1, row.2, row.3, Some(row.4), row.5, row.6)
    }

    async fn list_folders(&self, parent_id: Option<&str>) -> Result<Vec<Folder>, DomainError> {
        let rows: Vec<(String, String, String, Option<String>, String, i64, i64)> = if let Some(pid) = parent_id {
            sqlx::query_as(
                r#"
                SELECT id::text, name, path, parent_id::text, user_id,
                       EXTRACT(EPOCH FROM created_at)::bigint,
                       EXTRACT(EPOCH FROM updated_at)::bigint
                  FROM storage.folders
                 WHERE parent_id = $1::uuid AND NOT is_trashed
                 ORDER BY name
                "#,
            )
            .bind(pid)
            .fetch_all(self.pool())
            .await
        } else {
            sqlx::query_as(
                r#"
                SELECT id::text, name, path, parent_id::text, user_id,
                       EXTRACT(EPOCH FROM created_at)::bigint,
                       EXTRACT(EPOCH FROM updated_at)::bigint
                  FROM storage.folders
                 WHERE parent_id IS NULL AND NOT is_trashed
                 ORDER BY name
                "#,
            )
            .fetch_all(self.pool())
            .await
        }
        .map_err(|e| DomainError::internal_error("FolderDb", format!("list: {e}")))?;

        rows.into_iter()
            .map(|(id, name, path, pid, uid, ca, ma)| {
                Self::row_to_folder(id, name, path, pid, Some(uid), ca, ma)
            })
            .collect()
    }

    async fn list_folders_by_owner(
        &self,
        parent_id: Option<&str>,
        owner_id: &str,
    ) -> Result<Vec<Folder>, DomainError> {
        let rows: Vec<(String, String, String, Option<String>, String, i64, i64)> = if let Some(pid) = parent_id {
            sqlx::query_as(
                r#"
                SELECT id::text, name, path, parent_id::text, user_id,
                       EXTRACT(EPOCH FROM created_at)::bigint,
                       EXTRACT(EPOCH FROM updated_at)::bigint
                  FROM storage.folders
                 WHERE parent_id = $1::uuid AND user_id = $2 AND NOT is_trashed
                 ORDER BY name
                "#,
            )
            .bind(pid)
            .bind(owner_id)
            .fetch_all(self.pool())
            .await
        } else {
            sqlx::query_as(
                r#"
                SELECT id::text, name, path, parent_id::text, user_id,
                       EXTRACT(EPOCH FROM created_at)::bigint,
                       EXTRACT(EPOCH FROM updated_at)::bigint
                  FROM storage.folders
                 WHERE parent_id IS NULL AND user_id = $1 AND NOT is_trashed
                 ORDER BY name
                "#,
            )
            .bind(owner_id)
            .fetch_all(self.pool())
            .await
        }
        .map_err(|e| DomainError::internal_error("FolderDb", format!("list_by_owner: {e}")))?;

        rows.into_iter()
            .map(|(id, name, path, pid, uid, ca, ma)| {
                Self::row_to_folder(id, name, path, pid, Some(uid), ca, ma)
            })
            .collect()
    }

    async fn list_folders_paginated(
        &self,
        parent_id: Option<&str>,
        offset: usize,
        limit: usize,
        include_total: bool,
    ) -> Result<(Vec<Folder>, Option<usize>), DomainError> {
        let total = if include_total {
            let count: i64 = if let Some(pid) = parent_id {
                sqlx::query_scalar(
                    "SELECT COUNT(*) FROM storage.folders WHERE parent_id = $1::uuid AND NOT is_trashed",
                )
                .bind(pid)
                .fetch_one(self.pool())
                .await
            } else {
                sqlx::query_scalar(
                    "SELECT COUNT(*) FROM storage.folders WHERE parent_id IS NULL AND NOT is_trashed",
                )
                .fetch_one(self.pool())
                .await
            }
            .map_err(|e| DomainError::internal_error("FolderDb", format!("count: {e}")))?;
            Some(count as usize)
        } else {
            None
        };

        let rows: Vec<(String, String, String, Option<String>, String, i64, i64)> = if let Some(pid) = parent_id {
            sqlx::query_as(
                r#"
                SELECT id::text, name, path, parent_id::text, user_id,
                       EXTRACT(EPOCH FROM created_at)::bigint,
                       EXTRACT(EPOCH FROM updated_at)::bigint
                  FROM storage.folders
                 WHERE parent_id = $1::uuid AND NOT is_trashed
                 ORDER BY name
                 LIMIT $2 OFFSET $3
                "#,
            )
            .bind(pid)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(self.pool())
            .await
        } else {
            sqlx::query_as(
                r#"
                SELECT id::text, name, path, parent_id::text, user_id,
                       EXTRACT(EPOCH FROM created_at)::bigint,
                       EXTRACT(EPOCH FROM updated_at)::bigint
                  FROM storage.folders
                 WHERE parent_id IS NULL AND NOT is_trashed
                 ORDER BY name
                 LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(self.pool())
            .await
        }
        .map_err(|e| DomainError::internal_error("FolderDb", format!("paginate: {e}")))?;

        let folders: Result<Vec<Folder>, DomainError> = rows
            .into_iter()
            .map(|(id, name, path, pid, uid, ca, ma)| {
                Self::row_to_folder(id, name, path, pid, Some(uid), ca, ma)
            })
            .collect();
        Ok((folders?, total))
    }

    async fn list_folders_by_owner_paginated(
        &self,
        parent_id: Option<&str>,
        owner_id: &str,
        offset: usize,
        limit: usize,
        include_total: bool,
    ) -> Result<(Vec<Folder>, Option<usize>), DomainError> {
        let total = if include_total {
            let count: i64 = if let Some(pid) = parent_id {
                sqlx::query_scalar(
                    "SELECT COUNT(*) FROM storage.folders WHERE parent_id = $1::uuid AND user_id = $2 AND NOT is_trashed",
                )
                .bind(pid)
                .bind(owner_id)
                .fetch_one(self.pool())
                .await
            } else {
                sqlx::query_scalar(
                    "SELECT COUNT(*) FROM storage.folders WHERE parent_id IS NULL AND user_id = $1 AND NOT is_trashed",
                )
                .bind(owner_id)
                .fetch_one(self.pool())
                .await
            }
            .map_err(|e| DomainError::internal_error("FolderDb", format!("count_by_owner: {e}")))?;
            Some(count as usize)
        } else {
            None
        };

        let rows: Vec<(String, String, String, Option<String>, String, i64, i64)> = if let Some(pid) = parent_id {
            sqlx::query_as(
                r#"
                SELECT id::text, name, path, parent_id::text, user_id,
                       EXTRACT(EPOCH FROM created_at)::bigint,
                       EXTRACT(EPOCH FROM updated_at)::bigint
                  FROM storage.folders
                 WHERE parent_id = $1::uuid AND user_id = $2 AND NOT is_trashed
                 ORDER BY name
                 LIMIT $3 OFFSET $4
                "#,
            )
            .bind(pid)
            .bind(owner_id)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(self.pool())
            .await
        } else {
            sqlx::query_as(
                r#"
                SELECT id::text, name, path, parent_id::text, user_id,
                       EXTRACT(EPOCH FROM created_at)::bigint,
                       EXTRACT(EPOCH FROM updated_at)::bigint
                  FROM storage.folders
                 WHERE parent_id IS NULL AND user_id = $1 AND NOT is_trashed
                 ORDER BY name
                 LIMIT $2 OFFSET $3
                "#,
            )
            .bind(owner_id)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(self.pool())
            .await
        }
        .map_err(|e| DomainError::internal_error("FolderDb", format!("paginate_by_owner: {e}")))?;

        let folders: Result<Vec<Folder>, DomainError> = rows
            .into_iter()
            .map(|(id, name, path, pid, uid, ca, ma)| {
                Self::row_to_folder(id, name, path, pid, Some(uid), ca, ma)
            })
            .collect();
        Ok((folders?, total))
    }

    async fn rename_folder(&self, id: &str, new_name: String) -> Result<Folder, DomainError> {
        // The BEFORE UPDATE trigger on `name` will recompute path/lpath
        // and cascade to descendants automatically.
        sqlx::query(
            r#"
            UPDATE storage.folders
               SET name = $1, updated_at = NOW()
             WHERE id = $2::uuid AND NOT is_trashed
            "#,
        )
        .bind(&new_name)
        .bind(id)
        .execute(self.pool())
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.code().as_deref() == Some("23505")
            {
                return DomainError::already_exists("Folder", format!("{new_name} already exists"));
            }
            DomainError::internal_error("FolderDb", format!("rename: {e}"))
        })?;

        self.get_folder(id).await
    }

    async fn move_folder(
        &self,
        id: &str,
        new_parent_id: Option<&str>,
    ) -> Result<Folder, DomainError> {
        // The BEFORE UPDATE trigger on `parent_id` will recompute path/lpath
        // and cascade to descendants automatically.
        sqlx::query(
            r#"
            UPDATE storage.folders
               SET parent_id = $1::uuid, updated_at = NOW()
             WHERE id = $2::uuid AND NOT is_trashed
            "#,
        )
        .bind(new_parent_id)
        .bind(id)
        .execute(self.pool())
        .await
        .map_err(|e| DomainError::internal_error("FolderDb", format!("move: {e}")))?;

        self.get_folder(id).await
    }

    async fn delete_folder(&self, id: &str) -> Result<(), DomainError> {
        // Hard delete folder and all descendants (CASCADE handles children)
        let result = sqlx::query("DELETE FROM storage.folders WHERE id = $1::uuid")
            .bind(id)
            .execute(self.pool())
            .await
            .map_err(|e| DomainError::internal_error("FolderDb", format!("delete: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::not_found("Folder", id));
        }
        Ok(())
    }

    async fn folder_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError> {
        let path_str = storage_path.to_string();
        let lookup = path_str.strip_prefix('/').unwrap_or(&path_str);

        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM storage.folders WHERE path = $1 AND NOT is_trashed)",
        )
        .bind(lookup)
        .fetch_one(self.pool())
        .await
        .map_err(|e| DomainError::internal_error("FolderDb", format!("exists: {e}")))?;

        Ok(exists)
    }

    async fn get_folder_path(&self, id: &str) -> Result<StoragePath, DomainError> {
        let path: String = sqlx::query_scalar(
            "SELECT path FROM storage.folders WHERE id = $1::uuid",
        )
        .bind(id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| DomainError::internal_error("FolderDb", format!("get_path: {e}")))?
        .ok_or_else(|| DomainError::not_found("Folder", id))?;

        Ok(StoragePath::from_string(&path))
    }

    // ── Trash operations ──

    async fn move_to_trash(&self, folder_id: &str) -> Result<(), DomainError> {
        // Atomic CTE: trash folder + all descendant files in a single statement.
        // PostgreSQL executes the entire CTE as one atomic operation — no
        // intermediate state where the folder is trashed but files are not.
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            WITH trash_folder AS (
                UPDATE storage.folders
                   SET is_trashed = TRUE,
                       trashed_at = NOW(),
                       original_parent_id = parent_id,
                       updated_at = NOW()
                 WHERE id = $1::uuid AND NOT is_trashed
                RETURNING id
            ),
            descendants AS (
                SELECT id FROM trash_folder
                UNION ALL
                SELECT f.id FROM storage.folders f JOIN descendants d ON f.parent_id = d.id
            ),
            trash_files AS (
                UPDATE storage.files
                   SET is_trashed = TRUE, trashed_at = NOW(), original_folder_id = folder_id
                 WHERE folder_id IN (SELECT id FROM descendants) AND NOT is_trashed
                RETURNING 1
            )
            SELECT COUNT(*) FROM trash_folder
            "#,
        )
        .bind(folder_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| DomainError::internal_error("FolderDb", format!("trash: {e}")))?;

        if result == 0 {
            return Err(DomainError::not_found("Folder", folder_id));
        }

        Ok(())
    }

    async fn restore_from_trash(
        &self,
        folder_id: &str,
        _original_path: &str,
    ) -> Result<(), DomainError> {
        // Atomic CTE: restore folder + all descendant files in a single statement.
        // The BEFORE UPDATE trigger on parent_id will recompute path/lpath
        // automatically when original_parent_id is restored.
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            WITH restore_folder AS (
                UPDATE storage.folders
                   SET is_trashed = FALSE,
                       trashed_at = NULL,
                       parent_id = COALESCE(original_parent_id, parent_id),
                       original_parent_id = NULL,
                       updated_at = NOW()
                 WHERE id = $1::uuid AND is_trashed
                RETURNING id
            ),
            descendants AS (
                SELECT id FROM restore_folder
                UNION ALL
                SELECT f.id FROM storage.folders f JOIN descendants d ON f.parent_id = d.id
            ),
            restore_files AS (
                UPDATE storage.files
                   SET is_trashed = FALSE,
                       trashed_at = NULL,
                       folder_id = COALESCE(original_folder_id, folder_id),
                       original_folder_id = NULL
                 WHERE folder_id IN (SELECT id FROM descendants) AND is_trashed
                RETURNING 1
            )
            SELECT COUNT(*) FROM restore_folder
            "#,
        )
        .bind(folder_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| DomainError::internal_error("FolderDb", format!("restore: {e}")))?;

        if result == 0 {
            return Err(DomainError::not_found("Folder", folder_id));
        }

        Ok(())
    }

    async fn delete_folder_permanently(&self, folder_id: &str) -> Result<(), DomainError> {
        // Permanently delete — CASCADE handles children
        let result = sqlx::query("DELETE FROM storage.folders WHERE id = $1::uuid")
            .bind(folder_id)
            .execute(self.pool())
            .await
            .map_err(|e| DomainError::internal_error("FolderDb", format!("perm delete: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::not_found("Folder", folder_id));
        }
        Ok(())
    }

    async fn create_home_folder(
        &self,
        user_id: &str,
        name: String,
    ) -> Result<Folder, DomainError> {
        let row = sqlx::query_as::<_, (String, String, i64, i64)>(
            r#"
            INSERT INTO storage.folders (name, parent_id, user_id)
            VALUES ($1, NULL, $2)
            ON CONFLICT DO NOTHING
            RETURNING id::text,
                      path,
                      EXTRACT(EPOCH FROM created_at)::bigint,
                      EXTRACT(EPOCH FROM updated_at)::bigint
            "#,
        )
        .bind(&name)
        .bind(user_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| DomainError::internal_error("FolderDb", format!("home folder: {e}")))?;

        match row {
            Some((id, path, ca, ma)) => Self::row_to_folder(id, name.clone(), path, None, Some(user_id.to_string()), ca, ma),
            None => {
                // Already exists — fetch it
                let existing = sqlx::query_as::<_, (String, String, i64, i64)>(
                    r#"
                    SELECT id::text,
                           path,
                           EXTRACT(EPOCH FROM created_at)::bigint,
                           EXTRACT(EPOCH FROM updated_at)::bigint
                      FROM storage.folders
                     WHERE name = $1 AND user_id = $2 AND parent_id IS NULL
                    "#,
                )
                .bind(&name)
                .bind(user_id)
                .fetch_one(self.pool())
                .await
                .map_err(|e| DomainError::internal_error("FolderDb", format!("home fetch: {e}")))?;
                Self::row_to_folder(existing.0, name, existing.1, None, Some(user_id.to_string()), existing.2, existing.3)
            }
        }
    }
}

// ── Extra helpers for blob-storage bootstrap ──

impl FolderDbRepository {
    /// Returns user_id for a given folder. Used by file repositories.
    pub async fn get_folder_user_id(&self, folder_id: &str) -> Result<String, DomainError> {
        sqlx::query_scalar::<_, String>("SELECT user_id FROM storage.folders WHERE id = $1::uuid")
            .bind(folder_id)
            .fetch_optional(self.pool())
            .await
            .map_err(|e| DomainError::internal_error("FolderDb", format!("user_id lookup: {e}")))?
            .ok_or_else(|| DomainError::not_found("Folder", folder_id))
    }
}
