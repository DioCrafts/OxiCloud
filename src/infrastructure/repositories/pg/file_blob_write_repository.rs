//! PostgreSQL + Blob-backed file write repository.
//!
//! Implements `FileWritePort` using:
//! - `storage.files` table for metadata
//! - `DedupPort` for content-addressable blob storage on the filesystem

use async_trait::async_trait;
use sqlx::PgPool;
use std::path::PathBuf;
use std::sync::Arc;

use crate::application::ports::dedup_ports::DedupPort;
use crate::application::ports::storage_ports::FileWritePort;
use crate::common::errors::DomainError;
use crate::domain::entities::file::File;
use crate::domain::repositories::folder_repository::FolderRepository;
use crate::domain::services::path_service::StoragePath;

use super::folder_db_repository::FolderDbRepository;

/// File write repository backed by PostgreSQL metadata + blob storage.
pub struct FileBlobWriteRepository {
    pool: Arc<PgPool>,
    dedup: Arc<dyn DedupPort>,
    folder_repo: Arc<FolderDbRepository>,
}

impl FileBlobWriteRepository {
    pub fn new(
        pool: Arc<PgPool>,
        dedup: Arc<dyn DedupPort>,
        folder_repo: Arc<FolderDbRepository>,
    ) -> Self {
        Self {
            pool,
            dedup,
            folder_repo,
        }
    }

    /// Build a virtual StoragePath for a file from its DB metadata.
    async fn build_file_path(
        &self,
        folder_id: Option<&str>,
        file_name: &str,
    ) -> Result<StoragePath, DomainError> {
        if let Some(fid) = folder_id {
            let folder_path = self.folder_repo.get_folder_path(fid).await?;
            Ok(folder_path.join(file_name))
        } else {
            Ok(StoragePath::from_string(file_name))
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn row_to_file(
        &self,
        id: String,
        name: String,
        folder_id: Option<String>,
        size: i64,
        mime_type: String,
        created_at: i64,
        modified_at: i64,
    ) -> Result<File, DomainError> {
        let storage_path = self.build_file_path(folder_id.as_deref(), &name).await?;
        File::with_timestamps(
            id,
            name,
            storage_path,
            size as u64,
            mime_type,
            folder_id,
            created_at as u64,
            modified_at as u64,
        )
        .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("entity: {e}")))
    }

    /// Derive user_id from the parent folder, or error if folder_id is None.
    async fn resolve_user_id(&self, folder_id: Option<&str>) -> Result<String, DomainError> {
        match folder_id {
            Some(fid) => self.folder_repo.get_folder_user_id(fid).await,
            None => Err(DomainError::internal_error(
                "FileBlobWrite",
                "folder_id is required to determine file owner",
            )),
        }
    }
}

#[async_trait]
impl FileWritePort for FileBlobWriteRepository {
    async fn save_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<File, DomainError> {
        let user_id = self.resolve_user_id(folder_id.as_deref()).await?;
        let size = content.len() as i64;

        // Store content in blob store
        let dedup_result = self
            .dedup
            .store_bytes(&content, Some(content_type.clone()))
            .await?;
        let blob_hash = dedup_result.hash().to_string();

        // Insert file metadata — if this fails, compensate by removing the blob ref
        let row = match sqlx::query_as::<_, (String, i64, i64)>(
            r#"
            INSERT INTO storage.files (name, folder_id, user_id, blob_hash, size, mime_type)
            VALUES ($1, $2::uuid, $3, $4, $5, $6)
            RETURNING id::text,
                      EXTRACT(EPOCH FROM created_at)::bigint,
                      EXTRACT(EPOCH FROM updated_at)::bigint
            "#,
        )
        .bind(&name)
        .bind(&folder_id)
        .bind(&user_id)
        .bind(&blob_hash)
        .bind(size)
        .bind(&content_type)
        .fetch_one(self.pool.as_ref())
        .await
        {
            Ok(row) => row,
            Err(e) => {
                // ── Compensation: undo the blob ref so it doesn't become orphaned ──
                if let Err(rollback_err) = self.dedup.remove_reference(&blob_hash).await {
                    tracing::error!(
                        "Blob orphaned after failed INSERT — hash: {}, err: {}",
                        &blob_hash[..12],
                        rollback_err
                    );
                }
                if let sqlx::Error::Database(ref db_err) = e
                    && db_err.code().as_deref() == Some("23505")
                {
                    return Err(DomainError::already_exists(
                        "File",
                        format!("{name} already exists in folder"),
                    ));
                }
                return Err(DomainError::internal_error(
                    "FileBlobWrite",
                    format!("insert: {e}"),
                ));
            }
        };

        tracing::info!(
            "💾 BLOB WRITE: {} ({} bytes, hash: {})",
            name,
            size,
            &blob_hash[..12]
        );

        self.row_to_file(row.0, name, folder_id, size, content_type, row.1, row.2)
            .await
    }

    async fn save_file_from_temp(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        temp_path: &std::path::Path,
        size: u64,
        pre_computed_hash: Option<String>,
    ) -> Result<File, DomainError> {
        let user_id = self.resolve_user_id(folder_id.as_deref()).await?;

        // True streaming: pass pre-computed hash (or let dedup compute it).
        // When hash is pre-computed, zero extra disk reads.
        let dedup_result = self
            .dedup
            .store_from_file(temp_path, Some(content_type.clone()), pre_computed_hash)
            .await?;
        let blob_hash = dedup_result.hash().to_string();

        // Insert file metadata — if this fails, compensate by removing the blob ref
        let row = match sqlx::query_as::<_, (String, i64, i64)>(
            r#"
            INSERT INTO storage.files (name, folder_id, user_id, blob_hash, size, mime_type)
            VALUES ($1, $2::uuid, $3, $4, $5, $6)
            RETURNING id::text,
                      EXTRACT(EPOCH FROM created_at)::bigint,
                      EXTRACT(EPOCH FROM updated_at)::bigint
            "#,
        )
        .bind(&name)
        .bind(&folder_id)
        .bind(&user_id)
        .bind(&blob_hash)
        .bind(size as i64)
        .bind(&content_type)
        .fetch_one(self.pool.as_ref())
        .await
        {
            Ok(row) => row,
            Err(e) => {
                if let Err(rollback_err) = self.dedup.remove_reference(&blob_hash).await {
                    tracing::error!(
                        "Blob orphaned after failed INSERT — hash: {}, err: {}",
                        &blob_hash[..12],
                        rollback_err
                    );
                }
                if let sqlx::Error::Database(ref db_err) = e
                    && db_err.code().as_deref() == Some("23505")
                {
                    return Err(DomainError::already_exists(
                        "File",
                        format!("{name} already exists in folder"),
                    ));
                }
                return Err(DomainError::internal_error(
                    "FileBlobWrite",
                    format!("insert: {e}"),
                ));
            }
        };

        tracing::info!(
            "📡 STREAMING WRITE: {} ({} bytes, hash: {})",
            name,
            size,
            &blob_hash[..12]
        );

        self.row_to_file(
            row.0,
            name,
            folder_id,
            size as i64,
            content_type,
            row.1,
            row.2,
        )
        .await
    }

    async fn move_file(
        &self,
        file_id: &str,
        target_folder_id: Option<String>,
    ) -> Result<File, DomainError> {
        // If moving to a different folder, get the new user_id (must be same user)
        let row = sqlx::query_as::<_, (String, String, Option<String>, i64, String, i64, i64)>(
            r#"
            UPDATE storage.files
               SET folder_id = $1::uuid, updated_at = NOW()
             WHERE id = $2::uuid AND NOT is_trashed
            RETURNING id::text, name, folder_id::text, size, mime_type,
                      EXTRACT(EPOCH FROM created_at)::bigint,
                      EXTRACT(EPOCH FROM updated_at)::bigint
            "#,
        )
        .bind(&target_folder_id)
        .bind(file_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("move: {e}")))?
        .ok_or_else(|| DomainError::not_found("File", file_id))?;

        self.row_to_file(row.0, row.1, row.2, row.3, row.4, row.5, row.6)
            .await
    }

    async fn copy_file(
        &self,
        file_id: &str,
        target_folder_id: Option<String>,
    ) -> Result<File, DomainError> {
        // Atomic CTE: read source file → insert new row with same blob_hash → increment ref_count.
        // Single round-trip; blob content is NOT copied (dedup makes this zero-copy).
        let target_fid = target_folder_id.clone();

        let row = sqlx::query_as::<
            _,
            (
                String,
                String,
                Option<String>,
                i64,
                String,
                i64,
                i64,
                String,
            ),
        >(
            r#"
            WITH src AS (
                SELECT name, folder_id, user_id, blob_hash, size, mime_type
                  FROM storage.files
                 WHERE id = $1::uuid AND NOT is_trashed
            ),
            new_file AS (
                INSERT INTO storage.files (name, folder_id, user_id, blob_hash, size, mime_type)
                SELECT name,
                       COALESCE($2::uuid, folder_id),
                       user_id,
                       blob_hash,
                       size,
                       mime_type
                  FROM src
                RETURNING id::text, name, folder_id::text, size, mime_type,
                          EXTRACT(EPOCH FROM created_at)::bigint,
                          EXTRACT(EPOCH FROM updated_at)::bigint,
                          blob_hash
            )
            SELECT * FROM new_file
            "#,
        )
        .bind(file_id)
        .bind(&target_fid)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.code().as_deref() == Some("23505")
            {
                return DomainError::already_exists(
                    "File",
                    "File with that name already exists in target folder".to_string(),
                );
            }
            DomainError::internal_error("FileBlobWrite", format!("copy: {e}"))
        })?
        .ok_or_else(|| DomainError::not_found("File", file_id))?;

        let blob_hash = &row.7;

        // Increment blob reference count (best-effort; INSERT already succeeded)
        if let Err(e) = self.dedup.add_reference(blob_hash).await {
            tracing::warn!(
                "Failed to increment blob ref for copy {}: {}",
                &blob_hash[..12],
                e
            );
        }

        tracing::info!(
            "📋 BLOB COPY: {} (hash: {}, zero-copy via dedup)",
            row.1,
            &blob_hash[..12]
        );

        self.row_to_file(row.0, row.1, row.2, row.3, row.4, row.5, row.6)
            .await
    }

    async fn rename_file(&self, file_id: &str, new_name: &str) -> Result<File, DomainError> {
        let row = sqlx::query_as::<_, (String, String, Option<String>, i64, String, i64, i64)>(
            r#"
            UPDATE storage.files
               SET name = $1, updated_at = NOW()
             WHERE id = $2::uuid AND NOT is_trashed
            RETURNING id::text, name, folder_id::text, size, mime_type,
                      EXTRACT(EPOCH FROM created_at)::bigint,
                      EXTRACT(EPOCH FROM updated_at)::bigint
            "#,
        )
        .bind(new_name)
        .bind(file_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.code().as_deref() == Some("23505")
            {
                return DomainError::already_exists("File", format!("{new_name} already exists"));
            }
            DomainError::internal_error("FileBlobWrite", format!("rename: {e}"))
        })?
        .ok_or_else(|| DomainError::not_found("File", file_id))?;

        self.row_to_file(row.0, row.1, row.2, row.3, row.4, row.5, row.6)
            .await
    }

    async fn delete_file(&self, id: &str) -> Result<(), DomainError> {
        // Atomic DELETE RETURNING — one round-trip instead of SELECT + DELETE
        let hash = sqlx::query_scalar::<_, String>(
            "DELETE FROM storage.files WHERE id = $1::uuid RETURNING blob_hash",
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("delete: {e}")))?
        .ok_or_else(|| DomainError::not_found("File", id))?;

        // Decrement blob reference (best-effort after successful DELETE)
        if let Err(e) = self.dedup.remove_reference(&hash).await {
            tracing::warn!("Failed to decrement blob ref for {}: {}", &hash[..12], e);
        }

        Ok(())
    }

    async fn update_file_content(
        &self,
        file_id: &str,
        content: Vec<u8>,
    ) -> Result<(), DomainError> {
        // Store new content first (blob store is idempotent)
        let new_size = content.len() as i64;
        let dedup_result = self.dedup.store_bytes(&content, None).await?;
        let new_hash = dedup_result.hash().to_string();

        // Atomic CTE: capture old hash then update in one round-trip, no TOCTOU.
        // The `old` CTE locks + reads the row *before* the update touches it.
        let old_hash = match sqlx::query_scalar::<_, String>(
            r#"
            WITH old AS (
                SELECT id, blob_hash FROM storage.files WHERE id = $3::uuid FOR UPDATE
            )
            UPDATE storage.files f
               SET blob_hash = $1, size = $2, updated_at = NOW()
              FROM old
             WHERE f.id = old.id
            RETURNING old.blob_hash
            "#,
        )
        .bind(&new_hash)
        .bind(new_size)
        .bind(file_id)
        .fetch_optional(self.pool.as_ref())
        .await
        {
            Ok(Some(old)) => old,
            Ok(None) => {
                // File not found — compensate: remove the new blob ref
                if let Err(e) = self.dedup.remove_reference(&new_hash).await {
                    tracing::error!("Blob orphaned after missing file: {}", e);
                }
                return Err(DomainError::not_found("File", file_id));
            }
            Err(e) => {
                // UPDATE failed — compensate: remove the new blob ref
                if let Err(rollback_err) = self.dedup.remove_reference(&new_hash).await {
                    tracing::error!(
                        "Blob orphaned after failed UPDATE — hash: {}, err: {}",
                        &new_hash[..12],
                        rollback_err
                    );
                }
                return Err(DomainError::internal_error(
                    "FileBlobWrite",
                    format!("update: {e}"),
                ));
            }
        };

        // Decrement old blob ref (only if hash changed, best-effort)
        if old_hash != new_hash
            && let Err(e) = self.dedup.remove_reference(&old_hash).await
        {
            tracing::warn!(
                "Failed to decrement old blob ref {}: {}",
                &old_hash[..12],
                e
            );
        }

        Ok(())
    }

    async fn register_file_deferred(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        size: u64,
    ) -> Result<(File, PathBuf), DomainError> {
        let user_id = self.resolve_user_id(folder_id.as_deref()).await?;

        // For deferred registration we use a placeholder hash.
        // The write-behind cache will call update_file_content later.
        let placeholder_hash = "0000000000000000000000000000000000000000000000000000000000000000";

        let row = sqlx::query_as::<_, (String, i64, i64)>(
            r#"
            INSERT INTO storage.files (name, folder_id, user_id, blob_hash, size, mime_type)
            VALUES ($1, $2::uuid, $3, $4, $5, $6)
            RETURNING id::text,
                      EXTRACT(EPOCH FROM created_at)::bigint,
                      EXTRACT(EPOCH FROM updated_at)::bigint
            "#,
        )
        .bind(&name)
        .bind(&folder_id)
        .bind(&user_id)
        .bind(placeholder_hash)
        .bind(size as i64)
        .bind(&content_type)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("deferred: {e}")))?;

        let file = self
            .row_to_file(
                row.0.clone(),
                name,
                folder_id,
                size as i64,
                content_type,
                row.1,
                row.2,
            )
            .await?;

        // The target_path is not meaningful for blob storage (content goes to .blobs/)
        // but the WriteBehindCache API requires it. We return a synthetic path.
        let target_path = PathBuf::from(format!(".pending/{}", row.0));

        Ok((file, target_path))
    }

    // ── Trash operations ──

    async fn move_to_trash(&self, file_id: &str) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            UPDATE storage.files
               SET is_trashed = TRUE,
                   trashed_at = NOW(),
                   original_folder_id = folder_id,
                   updated_at = NOW()
             WHERE id = $1::uuid AND NOT is_trashed
            "#,
        )
        .bind(file_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("trash: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::not_found("File", file_id));
        }
        Ok(())
    }

    async fn restore_from_trash(
        &self,
        file_id: &str,
        _original_path: &str,
    ) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            UPDATE storage.files
               SET is_trashed = FALSE,
                   trashed_at = NULL,
                   folder_id = COALESCE(original_folder_id, folder_id),
                   original_folder_id = NULL,
                   updated_at = NOW()
             WHERE id = $1::uuid AND is_trashed
            "#,
        )
        .bind(file_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobWrite", format!("restore: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::not_found("File", file_id));
        }
        Ok(())
    }

    async fn delete_file_permanently(&self, file_id: &str) -> Result<(), DomainError> {
        // Same as delete_file — removes from DB and decrements blob ref
        self.delete_file(file_id).await
    }
}
