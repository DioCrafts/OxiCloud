//! PostgreSQL + Blob-backed file read repository.
//!
//! Implements `FileReadPort` using:
//! - `storage.files` table for metadata lookups
//! - `DedupPort` for reading content-addressable blobs from the filesystem
//!
//! File paths are resolved by JOINing with `storage.folders.path` (the
//! materialized path column), so no recursive CTEs or N+1 queries are needed.

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
use crate::domain::services::path_service::StoragePath;

/// File read repository backed by PostgreSQL metadata + blob storage.
pub struct FileBlobReadRepository {
    pool: Arc<PgPool>,
    dedup: Arc<dyn DedupPort>,
    /// Lightweight cache: file_id → blob_hash.
    /// Populated by `get_file()`, consumed by `resolve_blob_hash()`.
    /// Avoids an extra SQL round-trip on the hot download path.
    hash_cache: std::sync::Mutex<HashMap<String, String>>,
}

impl FileBlobReadRepository {
    pub fn new(
        pool: Arc<PgPool>,
        dedup: Arc<dyn DedupPort>,
        _folder_repo: Arc<super::folder_db_repository::FolderDbRepository>,
    ) -> Self {
        Self {
            pool,
            dedup,
            hash_cache: std::sync::Mutex::new(HashMap::new()),
        }
    }

    /// Build a `StoragePath` from the materialized folder path + file name.
    fn make_file_path(folder_path: Option<&str>, file_name: &str) -> StoragePath {
        match folder_path {
            Some(fp) if !fp.is_empty() => StoragePath::from_string(&format!("{fp}/{file_name}")),
            _ => StoragePath::from_string(file_name),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn row_to_file(
        id: String,
        name: String,
        folder_id: Option<String>,
        folder_path: Option<String>,
        size: i64,
        mime_type: String,
        created_at: i64,
        modified_at: i64,
    ) -> Result<File, DomainError> {
        let storage_path = Self::make_file_path(folder_path.as_deref(), &name);
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

    /// Resolve the blob hash for a file (internal helper).
    /// Checks the in-memory cache first (populated by `get_file`).
    async fn resolve_blob_hash(&self, file_id: &str) -> Result<String, DomainError> {
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
                String,         // id
                String,         // name
                Option<String>, // folder_id
                Option<String>, // folder path
                i64,            // size
                String,         // mime_type
                i64,            // created_at
                i64,            // updated_at
                String,         // blob_hash
            ),
        >(
            r#"
            SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                   fi.size, fi.mime_type,
                   EXTRACT(EPOCH FROM fi.created_at)::bigint,
                   EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                   fi.blob_hash
              FROM storage.files fi
              LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
             WHERE fi.id = $1::uuid AND NOT fi.is_trashed
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
            .insert(id.to_string(), row.8.clone());

        Self::row_to_file(row.0, row.1, row.2, row.3, row.4, row.5, row.6, row.7)
    }

    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<File>, DomainError> {
        let rows: Vec<(String, String, Option<String>, Option<String>, i64, String, i64, i64)> =
            if let Some(fid) = folder_id {
                sqlx::query_as(
                    r#"
                SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                       fi.size, fi.mime_type,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint
                  FROM storage.files fi
                  LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
                 WHERE fi.folder_id = $1::uuid AND NOT fi.is_trashed
                 ORDER BY fi.name
                "#,
                )
                .bind(fid)
                .fetch_all(self.pool.as_ref())
                .await
            } else {
                sqlx::query_as(
                    r#"
                SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                       fi.size, fi.mime_type,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint
                  FROM storage.files fi
                  LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
                 WHERE fi.folder_id IS NULL AND NOT fi.is_trashed
                 ORDER BY fi.name
                "#,
                )
                .fetch_all(self.pool.as_ref())
                .await
            }
            .map_err(|e| DomainError::internal_error("FileBlobRead", format!("list: {e}")))?;

        rows.into_iter()
            .map(|(id, name, fid, fpath, size, mime, ca, ma)| {
                Self::row_to_file(id, name, fid, fpath, size, mime, ca, ma)
            })
            .collect()
    }

    async fn get_blob_hash(&self, file_id: &str) -> Result<String, DomainError> {
        self.resolve_blob_hash(file_id).await
    }

    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError> {
        let blob_hash = self.resolve_blob_hash(id).await?;
        self.dedup.read_blob(&blob_hash).await
    }

    async fn get_file_stream(
        &self,
        id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        // True streaming: reads the blob file in 64 KB chunks.
        // Memory usage is ~64 KB regardless of file size.
        let blob_hash = self.resolve_blob_hash(id).await?;
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
        let blob_hash = self.resolve_blob_hash(id).await?;
        let stream = self
            .dedup
            .read_blob_range_stream(&blob_hash, start, end)
            .await?;
        Ok(Box::new(stream))
    }

    async fn get_file_mmap(&self, id: &str) -> Result<Bytes, DomainError> {
        // For RPi targets, mmap is less beneficial than streaming.
        // Keep as a fallback that loads content for small/medium files.
        let blob_hash = self.resolve_blob_hash(id).await?;
        self.dedup.read_blob_bytes(&blob_hash).await
    }

    async fn get_file_path(&self, id: &str) -> Result<StoragePath, DomainError> {
        let row = sqlx::query_as::<_, (String, Option<String>)>(
            r#"
            SELECT fi.name, fo.path
              FROM storage.files fi
              LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
             WHERE fi.id = $1::uuid AND NOT fi.is_trashed
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobRead", format!("path: {e}")))?
        .ok_or_else(|| DomainError::not_found("File", id))?;

        Ok(Self::make_file_path(row.1.as_deref(), &row.0))
    }

    async fn get_parent_folder_id(&self, path: &str) -> Result<String, DomainError> {
        let path = path.trim_start_matches('/').trim_end_matches('/');
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if segments.is_empty() {
            return Err(DomainError::not_found("Folder", "empty path"));
        }

        // For a path like "Home - user/Docs/file.txt", the parent folder path
        // is everything except the last segment: "Home - user/Docs"
        // We try the longest folder path first.
        let folder_path = segments[..segments.len() - 1].join("/");

        if folder_path.is_empty() {
            return Err(DomainError::not_found(
                "Folder",
                format!("parent for path: {path}"),
            ));
        }

        sqlx::query_scalar::<_, String>(
            "SELECT id::text FROM storage.folders WHERE path = $1 AND NOT is_trashed",
        )
        .bind(&folder_path)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("FileBlobRead", format!("parent lookup: {e}")))?
        .ok_or_else(|| DomainError::not_found("Folder", format!("parent for path: {path}")))
    }

    /// Direct SQL lookup using materialized folder paths.
    /// O(1) query instead of O(depth) folder walk.
    async fn find_file_by_path(&self, path: &str) -> Result<Option<File>, DomainError> {
        let path = path.trim_start_matches('/').trim_end_matches('/');
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if segments.is_empty() {
            return Ok(None);
        }

        // Last segment is the filename, preceding segments are the folder path
        let filename = segments[segments.len() - 1];
        let folder_path = segments[..segments.len() - 1].join("/");

        let row = if folder_path.is_empty() {
            // File at root level (no parent folder)
            sqlx::query_as::<_, (String, String, Option<String>, Option<String>, i64, String, i64, i64)>(
                r#"
                SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                       fi.size, fi.mime_type,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint
                  FROM storage.files fi
                  LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
                 WHERE fi.name = $1 AND fi.folder_id IS NULL AND NOT fi.is_trashed
                "#,
            )
            .bind(filename)
            .fetch_optional(self.pool.as_ref())
            .await
        } else {
            // File inside a folder — look up by folder path + filename
            sqlx::query_as::<_, (String, String, Option<String>, Option<String>, i64, String, i64, i64)>(
                r#"
                SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                       fi.size, fi.mime_type,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint
                  FROM storage.files fi
                  JOIN storage.folders fo ON fo.id = fi.folder_id
                 WHERE fo.path = $1 AND fi.name = $2 AND NOT fi.is_trashed
                "#,
            )
            .bind(&folder_path)
            .bind(filename)
            .fetch_optional(self.pool.as_ref())
            .await
        }
        .map_err(|e| DomainError::internal_error("FileBlobRead", format!("find file: {e}")))?;

        match row {
            Some(r) => Ok(Some(Self::row_to_file(
                r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7,
            )?)),
            None => Ok(None),
        }
    }
}
