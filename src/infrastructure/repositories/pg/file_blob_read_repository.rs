//! PostgreSQL + Blob-backed file read repository.
//!
//! Implements `FileReadPort` using:
//! - `storage.files` table for metadata lookups
//! - `DedupPort` for reading content-addressable blobs from the filesystem

use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;

use crate::application::ports::dedup_ports::DedupPort;
use crate::application::ports::storage_ports::FileReadPort;
use crate::common::errors::DomainError;
use crate::domain::entities::file::File;
use crate::domain::repositories::folder_repository::FolderRepository;
use crate::domain::services::path_service::StoragePath;

use super::folder_db_repository::FolderDbRepository;

/// File read repository backed by PostgreSQL metadata + blob storage.
pub struct FileBlobReadRepository {
    pool: Arc<PgPool>,
    dedup: Arc<dyn DedupPort>,
    folder_repo: Arc<FolderDbRepository>,
    /// Lightweight cache: file_id → blob_hash.
    /// Populated by `get_file()`, consumed by `get_blob_hash()`.
    /// Avoids an extra SQL round-trip on the hot download path.
    hash_cache: std::sync::Mutex<HashMap<String, String>>,
}

impl FileBlobReadRepository {
    pub fn new(
        pool: Arc<PgPool>,
        dedup: Arc<dyn DedupPort>,
        folder_repo: Arc<FolderDbRepository>,
    ) -> Self {
        Self {
            pool,
            dedup,
            folder_repo,
            hash_cache: std::sync::Mutex::new(HashMap::new()),
        }
    }

    /// Build a virtual StoragePath for a file.
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
        .map_err(|e| DomainError::internal_error("FileBlobRead", format!("entity: {e}")))
    }

    /// Get the blob hash for a file.
    /// Checks the in-memory cache first (populated by `get_file`).
    async fn get_blob_hash(&self, file_id: &str) -> Result<String, DomainError> {
        // Fast path: already cached from a prior get_file call
        if let Some(hash) = self.hash_cache.lock().unwrap().remove(file_id) {
            return Ok(hash);
        }
        // Slow path: DB round-trip
        sqlx::query_scalar::<_, String>(
            "SELECT blob_hash FROM storage.files WHERE id = $1::uuid AND NOT is_trashed",
        )
        .bind(file_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobRead", format!("hash lookup: {e}")))?
        .ok_or_else(|| DomainError::not_found("File", file_id))
    }
}

#[async_trait]
impl FileReadPort for FileBlobReadRepository {
    async fn get_file(&self, id: &str) -> Result<File, DomainError> {
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
            SELECT id::text, name, folder_id::text, size, mime_type,
                   EXTRACT(EPOCH FROM created_at)::bigint,
                   EXTRACT(EPOCH FROM updated_at)::bigint,
                   blob_hash
              FROM storage.files
             WHERE id = $1::uuid AND NOT is_trashed
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobRead", format!("get: {e}")))?
        .ok_or_else(|| DomainError::not_found("File", id))?;

        // Cache blob_hash so the subsequent get_file_stream / get_file_content
        // call doesn't need a separate DB round-trip.
        self.hash_cache
            .lock()
            .unwrap()
            .insert(id.to_string(), row.7.clone());

        self.row_to_file(row.0, row.1, row.2, row.3, row.4, row.5, row.6)
            .await
    }

    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<File>, DomainError> {
        let rows: Vec<(String, String, Option<String>, i64, String, i64, i64)> =
            if let Some(fid) = folder_id {
                sqlx::query_as(
                    r#"
                SELECT id::text, name, folder_id::text, size, mime_type,
                       EXTRACT(EPOCH FROM created_at)::bigint,
                       EXTRACT(EPOCH FROM updated_at)::bigint
                  FROM storage.files
                 WHERE folder_id = $1::uuid AND NOT is_trashed
                 ORDER BY name
                "#,
                )
                .bind(fid)
                .fetch_all(self.pool.as_ref())
                .await
            } else {
                sqlx::query_as(
                    r#"
                SELECT id::text, name, folder_id::text, size, mime_type,
                       EXTRACT(EPOCH FROM created_at)::bigint,
                       EXTRACT(EPOCH FROM updated_at)::bigint
                  FROM storage.files
                 WHERE folder_id IS NULL AND NOT is_trashed
                 ORDER BY name
                "#,
                )
                .fetch_all(self.pool.as_ref())
                .await
            }
            .map_err(|e| DomainError::internal_error("FileBlobRead", format!("list: {e}")))?;

        // ── N+1 fix: resolve the folder path ONCE (all rows share
        //    the same folder_id when listing a specific folder). ──
        let shared_folder_path = if let Some(fid) = folder_id {
            Some(self.folder_repo.get_folder_path(fid).await?)
        } else {
            None
        };

        let mut files = Vec::with_capacity(rows.len());
        for (id, name, fid, size, mime, ca, ma) in rows {
            let storage_path = match &shared_folder_path {
                Some(fp) => fp.join(&name),
                None => StoragePath::from_string(&name),
            };
            let file = File::with_timestamps(
                id,
                name,
                storage_path,
                size as u64,
                mime,
                fid,
                ca as u64,
                ma as u64,
            )
            .map_err(|e| DomainError::internal_error("FileBlobRead", format!("entity: {e}")))?;
            files.push(file);
        }
        Ok(files)
    }

    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError> {
        let blob_hash = self.get_blob_hash(id).await?;
        self.dedup.read_blob(&blob_hash).await
    }

    async fn get_file_stream(
        &self,
        id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        // True streaming: reads the blob file in 64 KB chunks.
        // Memory usage is ~64 KB regardless of file size.
        let blob_hash = self.get_blob_hash(id).await?;
        let stream = self.dedup.read_blob_stream(&blob_hash).await?;
        Ok(Box::new(stream))
    }

    async fn get_file_range_stream(
        &self,
        id: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        // True range streaming: seeks to `start` and reads only the requested range.
        // A 1 MB range on a 1 GB file uses ~64 KB of RAM.
        let blob_hash = self.get_blob_hash(id).await?;
        let stream = self
            .dedup
            .read_blob_range_stream(&blob_hash, start, end)
            .await?;
        Ok(Box::new(stream))
    }

    async fn get_file_mmap(&self, id: &str) -> Result<Bytes, DomainError> {
        // For RPi targets, mmap is less beneficial than streaming.
        // Keep as a fallback that loads content for small/medium files.
        let blob_hash = self.get_blob_hash(id).await?;
        self.dedup.read_blob_bytes(&blob_hash).await
    }

    async fn get_file_path(&self, id: &str) -> Result<StoragePath, DomainError> {
        let row = sqlx::query_as::<_, (String, Option<String>)>(
            r#"
            SELECT name, folder_id::text
              FROM storage.files
             WHERE id = $1::uuid AND NOT is_trashed
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobRead", format!("path: {e}")))?
        .ok_or_else(|| DomainError::not_found("File", id))?;

        self.build_file_path(row.1.as_deref(), &row.0).await
    }

    async fn get_parent_folder_id(&self, path: &str) -> Result<String, DomainError> {
        // Walk the path to find the parent folder, searching by folder names
        let path = path.trim_start_matches('/').trim_end_matches('/');
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if segments.is_empty() {
            return Err(DomainError::not_found("Folder", "empty path"));
        }

        // For path "a/b/c/file.txt", the parent folder path is "a/b/c"
        // But we don't know which part is folders vs filename.
        // Walk segments trying to find matching folders.
        let mut current_parent: Option<String> = None;

        for segment in &segments {
            let row = if let Some(ref pid) = current_parent {
                sqlx::query_as::<_, (String,)>(
                    r#"
                    SELECT id::text FROM storage.folders
                     WHERE name = $1 AND parent_id = $2::uuid AND NOT is_trashed
                    "#,
                )
                .bind(segment)
                .bind(pid)
                .fetch_optional(self.pool.as_ref())
                .await
            } else {
                sqlx::query_as::<_, (String,)>(
                    r#"
                    SELECT id::text FROM storage.folders
                     WHERE name = $1 AND parent_id IS NULL AND NOT is_trashed
                    "#,
                )
                .bind(segment)
                .fetch_optional(self.pool.as_ref())
                .await
            }
            .map_err(|e| DomainError::internal_error("FileBlobRead", format!("path walk: {e}")))?;

            match row {
                Some(r) => current_parent = Some(r.0),
                None => break, // This segment is not a folder → it's the filename
            }
        }

        current_parent
            .ok_or_else(|| DomainError::not_found("Folder", format!("parent for path: {path}")))
    }

    /// Direct SQL lookup: split path into folder segments + filename,
    /// walk the folder hierarchy, then match the file by name + folder_id.
    /// O(depth) queries instead of O(total_files).
    async fn find_file_by_path(&self, path: &str) -> Result<Option<File>, DomainError> {
        let path = path.trim_start_matches('/').trim_end_matches('/');
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if segments.is_empty() {
            return Ok(None);
        }

        // Last segment is the filename, preceding segments are folders
        let filename = segments[segments.len() - 1];
        let folder_segments = &segments[..segments.len() - 1];

        // Walk folder hierarchy to find parent folder_id
        let mut current_parent: Option<String> = None;
        for segment in folder_segments {
            let row = if let Some(ref pid) = current_parent {
                sqlx::query_scalar::<_, String>(
                    "SELECT id::text FROM storage.folders WHERE name = $1 AND parent_id = $2::uuid AND NOT is_trashed",
                )
                .bind(segment)
                .bind(pid)
                .fetch_optional(self.pool.as_ref())
                .await
            } else {
                sqlx::query_scalar::<_, String>(
                    "SELECT id::text FROM storage.folders WHERE name = $1 AND parent_id IS NULL AND NOT is_trashed",
                )
                .bind(segment)
                .fetch_optional(self.pool.as_ref())
                .await
            }
            .map_err(|e| DomainError::internal_error("FileBlobRead", format!("path walk: {e}")))?;

            match row {
                Some(id) => current_parent = Some(id),
                None => return Ok(None), // Folder not found → file doesn't exist at this path
            }
        }

        // Now find the file by name + folder_id
        let row = if let Some(ref fid) = current_parent {
            sqlx::query_as::<_, (String, String, Option<String>, i64, String, i64, i64)>(
                r#"
                SELECT id::text, name, folder_id::text, size, mime_type,
                       EXTRACT(EPOCH FROM created_at)::bigint,
                       EXTRACT(EPOCH FROM updated_at)::bigint
                  FROM storage.files
                 WHERE name = $1 AND folder_id = $2::uuid AND NOT is_trashed
                "#,
            )
            .bind(filename)
            .bind(fid)
            .fetch_optional(self.pool.as_ref())
            .await
        } else {
            sqlx::query_as::<_, (String, String, Option<String>, i64, String, i64, i64)>(
                r#"
                SELECT id::text, name, folder_id::text, size, mime_type,
                       EXTRACT(EPOCH FROM created_at)::bigint,
                       EXTRACT(EPOCH FROM updated_at)::bigint
                  FROM storage.files
                 WHERE name = $1 AND folder_id IS NULL AND NOT is_trashed
                "#,
            )
            .bind(filename)
            .fetch_optional(self.pool.as_ref())
            .await
        }
        .map_err(|e| DomainError::internal_error("FileBlobRead", format!("find file: {e}")))?;

        match row {
            Some(r) => Ok(Some(
                self.row_to_file(r.0, r.1, r.2, r.3, r.4, r.5, r.6).await?,
            )),
            None => Ok(None),
        }
    }
}
