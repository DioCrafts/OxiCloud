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
use futures::{Stream, TryStreamExt};
use moka::sync::Cache;
use sqlx::PgPool;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use crate::application::dtos::search_dto::SearchCriteriaDto;
use crate::application::ports::dedup_ports::DedupPort;
use crate::application::ports::storage_ports::FileReadPort;
use crate::common::errors::DomainError;
use crate::domain::entities::file::File;
use crate::domain::services::path_service::StoragePath;

/// File read repository backed by PostgreSQL metadata + blob storage.
pub struct FileBlobReadRepository {
    pool: Arc<PgPool>,
    dedup: Arc<dyn DedupPort>,
    /// Lock-free cache: file_id → blob_hash.
    /// Populated by `get_file()`, consumed by `resolve_blob_hash()`.
    /// Avoids an extra SQL round-trip on the hot download path.
    /// Uses moka with TTI eviction to prevent unbounded growth.
    hash_cache: Cache<String, String>,
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
            hash_cache: Cache::builder()
                .max_capacity(10_000)
                .time_to_idle(Duration::from_secs(30))
                .build(),
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
        owner_id: Option<String>,
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
            owner_id,
        )
        .map_err(|e| DomainError::internal_error("FileBlobRead", format!("entity: {e}")))
    }

    /// Resolve the blob hash for a file (internal helper).
    /// Checks the lock-free moka cache first (populated by `get_file`).
    async fn resolve_blob_hash(&self, file_id: &str) -> Result<String, DomainError> {
        // Fast path: cached from a prior get_file call (lock-free read)
        if let Some(hash) = self.hash_cache.get(file_id) {
            self.hash_cache.invalidate(file_id);
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
                Option<String>, // user_id (owner)
            ),
        >(
            r#"
            SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                   fi.size, fi.mime_type,
                   EXTRACT(EPOCH FROM fi.created_at)::bigint,
                   EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                   fi.blob_hash,
                   fi.user_id::text
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
        self.hash_cache.insert(id.to_string(), row.8.clone());

        Self::row_to_file(
            row.0, row.1, row.2, row.3, row.4, row.5, row.6, row.7, row.9,
        )
    }

    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<File>, DomainError> {
        let rows: Vec<(
            String,
            String,
            Option<String>,
            Option<String>,
            i64,
            String,
            i64,
            i64,
            Option<String>,
        )> = if let Some(fid) = folder_id {
            sqlx::query_as(
                r#"
                SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                       fi.size, fi.mime_type,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                       fi.user_id::text
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
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                       fi.user_id::text
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
            .map(|(id, name, fid, fpath, size, mime, ca, ma, uid)| {
                Self::row_to_file(id, name, fid, fpath, size, mime, ca, ma, uid)
            })
            .collect()
    }

    async fn get_blob_hash(&self, file_id: &str) -> Result<String, DomainError> {
        self.resolve_blob_hash(file_id).await
    }

    /// Paginated file listing — fetches only `limit` rows starting at `offset`.
    ///
    /// Uses a single SQL query with `LIMIT/OFFSET` to avoid loading the full
    /// folder contents into memory.  Ideal for streaming WebDAV PROPFIND.
    async fn list_files_batch(
        &self,
        folder_id: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<File>, DomainError> {
        let rows: Vec<(
            String,
            String,
            Option<String>,
            Option<String>,
            i64,
            String,
            i64,
            i64,
            Option<String>,
        )> = if let Some(fid) = folder_id {
            sqlx::query_as(
                r#"
                SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                       fi.size, fi.mime_type,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                       fi.user_id::text
                  FROM storage.files fi
                  LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
                 WHERE fi.folder_id = $1::uuid AND NOT fi.is_trashed
                 ORDER BY fi.name
                 LIMIT $2 OFFSET $3
                "#,
            )
            .bind(fid)
            .bind(limit)
            .bind(offset)
            .fetch_all(self.pool.as_ref())
            .await
        } else {
            sqlx::query_as(
                r#"
                SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                       fi.size, fi.mime_type,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                       fi.user_id::text
                  FROM storage.files fi
                  LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
                 WHERE fi.folder_id IS NULL AND NOT fi.is_trashed
                 ORDER BY fi.name
                 LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(self.pool.as_ref())
            .await
        }
        .map_err(|e| DomainError::internal_error("FileBlobRead", format!("list_batch: {e}")))?;

        rows.into_iter()
            .map(|(id, name, fid, fpath, size, mime, ca, ma, uid)| {
                Self::row_to_file(id, name, fid, fpath, size, mime, ca, ma, uid)
            })
            .collect()
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
            sqlx::query_as::<
                _,
                (
                    String,
                    String,
                    Option<String>,
                    Option<String>,
                    i64,
                    String,
                    i64,
                    i64,
                    Option<String>,
                ),
            >(
                r#"
                SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                       fi.size, fi.mime_type,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                       fi.user_id::text
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
            sqlx::query_as::<
                _,
                (
                    String,
                    String,
                    Option<String>,
                    Option<String>,
                    i64,
                    String,
                    i64,
                    i64,
                    Option<String>,
                ),
            >(
                r#"
                SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                       fi.size, fi.mime_type,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                       fi.user_id::text
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
                r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7, r.8,
            )?)),
            None => Ok(None),
        }
    }

    /// Streams every file in the subtree rooted at `folder_id`.
    ///
    /// Single GiST-indexed query via ltree `<@`.  Results are delivered
    /// through a PostgreSQL cursor — RAM stays O(1) per row.
    async fn stream_files_in_subtree(
        &self,
        folder_id: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<File, DomainError>> + Send>>, DomainError> {
        let pool = Arc::clone(&self.pool);
        let folder_id = folder_id.to_owned();

        let stream = async_stream::try_stream! {
            let mut row_stream = sqlx::query_as::<_, (
                String, String, Option<String>, Option<String>,
                i64, String, i64, i64, Option<String>,
            )>(
                r#"
                SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                       fi.size, fi.mime_type,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                       fi.user_id::text
                  FROM storage.files fi
                  JOIN storage.folders fo ON fo.id = fi.folder_id
                 WHERE fo.lpath <@ (SELECT lpath FROM storage.folders WHERE id = $1::uuid)
                   AND NOT fi.is_trashed
                 ORDER BY fo.path, fi.name
                "#,
            )
            .bind(&folder_id)
            .fetch(pool.as_ref());

            while let Some(row) = row_stream.try_next().await.map_err(|e| {
                DomainError::internal_error("FileBlobRead", format!("subtree stream: {e}"))
            })? {
                let (id, name, fid, fpath, size, mime, ca, ma, uid) = row;
                let file = FileBlobReadRepository::row_to_file(
                    id, name, fid, fpath, size, mime, ca, ma, uid,
                )?;
                yield file;
            }
        };

        Ok(Box::pin(stream))
    }

    /// Search files with filtering and pagination at database level.
    ///
    /// Uses `COUNT(*) OVER()` window function to return the total matching
    /// count alongside the paginated rows in a **single query** — no separate
    /// COUNT round-trip.
    async fn search_files_paginated(
        &self,
        folder_id: Option<&str>,
        criteria: &SearchCriteriaDto,
        user_id: &str,
    ) -> Result<(Vec<File>, usize), DomainError> {
        let offset = criteria.offset as i64;
        let limit = criteria.limit as i64;

        // Determine sort order
        let (order_column, order_dir) = match criteria.sort_by.as_str() {
            "name" => ("fi.name", "ASC"),
            "name_desc" => ("fi.name", "DESC"),
            "date" => ("fi.updated_at", "ASC"),
            "date_desc" => ("fi.updated_at", "DESC"),
            "size" => ("fi.size", "ASC"),
            "size_desc" => ("fi.size", "DESC"),
            _ => ("fi.name", "ASC"),
        };

        // ── Build dynamic WHERE + bind indices ───────────────────────────
        let mut conditions: Vec<String> = vec![
            "fi.user_id = $1::uuid".to_string(),
            "fi.is_trashed = false".to_string(),
        ];
        let mut bind_idx = 1u32; // $1 = user_id

        if folder_id.is_some() {
            bind_idx += 1;
            conditions.push(format!("fi.folder_id = ${bind_idx}::uuid"));
        }

        if let Some(name) = &criteria.name_contains
            && !name.is_empty()
        {
            bind_idx += 1;
            conditions.push(format!("LOWER(fi.name) LIKE ${bind_idx}"));
        }

        let where_clause = conditions.join(" AND ");
        let limit_bind = bind_idx + 1;
        let offset_bind = bind_idx + 2;

        let sql = format!(
            "SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path, \
                    fi.size, fi.mime_type, \
                    EXTRACT(EPOCH FROM fi.created_at)::bigint, \
                    EXTRACT(EPOCH FROM fi.updated_at)::bigint, \
                    fi.user_id::text, \
                    COUNT(*) OVER() AS total_count \
               FROM storage.files fi \
               LEFT JOIN storage.folders fo ON fo.id = fi.folder_id \
              WHERE {where_clause} \
              ORDER BY {order_column} {order_dir} \
              LIMIT ${limit_bind} OFFSET ${offset_bind}"
        );

        // ── Bind parameters dynamically ──────────────────────────────────
        let mut query = sqlx::query_as::<
            _,
            (
                String,
                String,
                Option<String>,
                Option<String>,
                i64,
                String,
                i64,
                i64,
                Option<String>,
                i64,
            ),
        >(&sql)
        .bind(user_id);

        if let Some(fid) = folder_id {
            query = query.bind(fid);
        }
        if let Some(name) = &criteria.name_contains
            && !name.is_empty()
        {
            query = query.bind(format!("%{}%", name.to_lowercase()));
        }
        query = query.bind(limit).bind(offset);

        // ── Execute single query ─────────────────────────────────────────
        let rows = query
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| DomainError::internal_error("FileBlobRead", format!("search: {e}")))?;

        // total_count is the same in every row; 0 when result set is empty.
        let total_count = rows.first().map_or(0, |r| r.9) as usize;

        let files = rows
            .into_iter()
            .map(|(id, name, fid, fpath, size, mime, ca, ma, uid, _total)| {
                Self::row_to_file(id, name, fid, fpath, size, mime, ca, ma, uid)
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| DomainError::internal_error("FileBlobRead", format!("mapping: {e}")))?;

        Ok((files, total_count))
    }

    /// Recursive subtree search using ltree — single SQL query.
    ///
    /// When `root_folder_id` is Some, JOINs `storage.files` with
    /// `storage.folders` using `lpath <@ (root's lpath)` to find all
    /// files in the entire subtree.
    /// When None, delegates to `search_files_paginated`.
    ///
    /// Uses `COUNT(*) OVER()` to return the total count alongside the
    /// paginated rows — no separate COUNT round-trip.
    async fn search_files_in_subtree(
        &self,
        root_folder_id: Option<&str>,
        criteria: &SearchCriteriaDto,
        user_id: &str,
    ) -> Result<(Vec<File>, usize), DomainError> {
        // When no root folder specified, delegate to existing paginated search
        let root_id = match root_folder_id {
            None => {
                return self.search_files_paginated(None, criteria, user_id).await;
            }
            Some(id) => id,
        };

        let offset = criteria.offset as i64;
        let limit = criteria.limit as i64;

        // Determine sort order
        let (order_column, order_dir) = match criteria.sort_by.as_str() {
            "name" => ("fi.name", "ASC"),
            "name_desc" => ("fi.name", "DESC"),
            "date" => ("fi.updated_at", "ASC"),
            "date_desc" => ("fi.updated_at", "DESC"),
            "size" => ("fi.size", "ASC"),
            "size_desc" => ("fi.size", "DESC"),
            _ => ("fi.name", "ASC"),
        };

        // ── Build dynamic WHERE clauses ──
        let mut conditions = Vec::new();
        let mut bind_idx = 2u32; // $1 = user_id, $2 = root_folder_id

        conditions.push("fi.is_trashed = false".to_string());
        conditions.push("fi.user_id = $1".to_string());
        conditions.push(
            "fo.lpath <@ (SELECT lpath FROM storage.folders WHERE id = $2::uuid)".to_string(),
        );

        if let Some(name) = &criteria.name_contains
            && !name.is_empty()
        {
            bind_idx += 1;
            conditions.push(format!("LOWER(fi.name) LIKE ${bind_idx}"));
        }
        if let Some(types) = &criteria.file_types
            && !types.is_empty()
        {
            bind_idx += 1;
            conditions.push(format!(
                "LOWER(SUBSTRING(fi.name FROM '\\.([^.]+)$')) = ANY(${bind_idx})"
            ));
        }
        if criteria.created_after.is_some() {
            bind_idx += 1;
            conditions.push(format!(
                "EXTRACT(EPOCH FROM fi.created_at)::bigint >= ${bind_idx}"
            ));
        }
        if criteria.created_before.is_some() {
            bind_idx += 1;
            conditions.push(format!(
                "EXTRACT(EPOCH FROM fi.created_at)::bigint <= ${bind_idx}"
            ));
        }
        if criteria.modified_after.is_some() {
            bind_idx += 1;
            conditions.push(format!(
                "EXTRACT(EPOCH FROM fi.updated_at)::bigint >= ${bind_idx}"
            ));
        }
        if criteria.modified_before.is_some() {
            bind_idx += 1;
            conditions.push(format!(
                "EXTRACT(EPOCH FROM fi.updated_at)::bigint <= ${bind_idx}"
            ));
        }
        if criteria.min_size.is_some() {
            bind_idx += 1;
            conditions.push(format!("fi.size >= ${bind_idx}"));
        }
        if criteria.max_size.is_some() {
            bind_idx += 1;
            conditions.push(format!("fi.size <= ${bind_idx}"));
        }

        let where_clause = conditions.join(" AND ");
        let limit_bind = bind_idx + 1;
        let offset_bind = bind_idx + 2;

        // ── Single query with COUNT(*) OVER() ──
        let sql = format!(
            "SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path, \
                    fi.size, fi.mime_type, \
                    EXTRACT(EPOCH FROM fi.created_at)::bigint, \
                    EXTRACT(EPOCH FROM fi.updated_at)::bigint, \
                    fi.user_id::text, \
                    COUNT(*) OVER() AS total_count \
               FROM storage.files fi \
               JOIN storage.folders fo ON fo.id = fi.folder_id \
              WHERE {where_clause} \
              ORDER BY {order_column} {order_dir} \
              LIMIT ${limit_bind} OFFSET ${offset_bind}"
        );

        // ── Bind parameters dynamically ──
        let mut query = sqlx::query_as::<
            _,
            (
                String,
                String,
                Option<String>,
                Option<String>,
                i64,
                String,
                i64,
                i64,
                Option<String>,
                i64,
            ),
        >(&sql)
        .bind(user_id)
        .bind(root_id);

        if let Some(name) = &criteria.name_contains
            && !name.is_empty()
        {
            query = query.bind(format!("%{}%", name.to_lowercase()));
        }
        if let Some(types) = &criteria.file_types
            && !types.is_empty()
        {
            let lower_types: Vec<String> = types.iter().map(|t| t.to_lowercase()).collect();
            query = query.bind(lower_types);
        }
        if let Some(v) = criteria.created_after {
            query = query.bind(v as i64);
        }
        if let Some(v) = criteria.created_before {
            query = query.bind(v as i64);
        }
        if let Some(v) = criteria.modified_after {
            query = query.bind(v as i64);
        }
        if let Some(v) = criteria.modified_before {
            query = query.bind(v as i64);
        }
        if let Some(v) = criteria.min_size {
            query = query.bind(v as i64);
        }
        if let Some(v) = criteria.max_size {
            query = query.bind(v as i64);
        }

        query = query.bind(limit).bind(offset);

        // ── Execute single query ──
        let rows = query.fetch_all(self.pool.as_ref()).await.map_err(|e| {
            DomainError::internal_error("FileBlobRead", format!("subtree search: {e}"))
        })?;

        let total_count = rows.first().map_or(0, |r| r.9) as usize;

        let files = rows
            .into_iter()
            .map(|(id, name, fid, fpath, size, mime, ca, ma, uid, _total)| {
                Self::row_to_file(id, name, fid, fpath, size, mime, ca, ma, uid)
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                DomainError::internal_error("FileBlobRead", format!("subtree mapping: {e}"))
            })?;

        Ok((files, total_count))
    }

    /// Count files matching the search criteria (without loading them).
    async fn count_files(
        &self,
        folder_id: Option<&str>,
        criteria: &SearchCriteriaDto,
        user_id: &str,
    ) -> Result<usize, DomainError> {
        let (_, count) = self
            .search_files_paginated(folder_id, criteria, user_id)
            .await?;
        Ok(count)
    }

    async fn suggest_files_by_name(
        &self,
        folder_id: Option<&str>,
        query: &str,
        limit: usize,
    ) -> Result<Vec<File>, DomainError> {
        let pattern = format!("%{}%", query.to_lowercase());
        let limit_i64 = limit as i64;
        let query_lower = query.to_lowercase();

        let rows: Vec<(
            String,
            String,
            Option<String>,
            Option<String>,
            i64,
            String,
            i64,
            i64,
            Option<String>,
        )> = if let Some(fid) = folder_id {
            sqlx::query_as(
                r#"
                SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                       fi.size, fi.mime_type,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                       fi.user_id::text
                  FROM storage.files fi
                  LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
                 WHERE fi.folder_id = $1::uuid
                   AND NOT fi.is_trashed
                   AND LOWER(fi.name) LIKE $2
                 ORDER BY CASE
                            WHEN LOWER(fi.name) = $3 THEN 0
                            WHEN LOWER(fi.name) LIKE $3 || '%' THEN 1
                            ELSE 2
                          END,
                          fi.name
                 LIMIT $4
                "#,
            )
            .bind(fid)
            .bind(&pattern)
            .bind(&query_lower)
            .bind(limit_i64)
            .fetch_all(self.pool.as_ref())
            .await
        } else {
            sqlx::query_as(
                r#"
                SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                       fi.size, fi.mime_type,
                       EXTRACT(EPOCH FROM fi.created_at)::bigint,
                       EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                       fi.user_id::text
                  FROM storage.files fi
                  LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
                 WHERE fi.folder_id IS NULL
                   AND NOT fi.is_trashed
                   AND LOWER(fi.name) LIKE $1
                 ORDER BY CASE
                            WHEN LOWER(fi.name) = $2 THEN 0
                            WHEN LOWER(fi.name) LIKE $2 || '%' THEN 1
                            ELSE 2
                          END,
                          fi.name
                 LIMIT $3
                "#,
            )
            .bind(&pattern)
            .bind(&query_lower)
            .bind(limit_i64)
            .fetch_all(self.pool.as_ref())
            .await
        }
        .map_err(|e| DomainError::internal_error("FileBlobRead", format!("suggest: {e}")))?;

        rows.into_iter()
            .map(|(id, name, fid, fpath, size, mime, ca, ma, uid)| {
                Self::row_to_file(id, name, fid, fpath, size, mime, ca, ma, uid)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::stubs::StubDedupPort;
    use crate::infrastructure::repositories::pg::folder_db_repository::FolderDbRepository;

    /// Helper: build a `FileBlobReadRepository` without a real PgPool.
    /// Only the moka `hash_cache` is exercised — no SQL is executed.
    fn make_repo() -> FileBlobReadRepository {
        let _folder_repo = Arc::new(FolderDbRepository::new_stub());
        // StubDedupPort satisfies the trait but is never called in cache-only tests
        let dedup: Arc<dyn DedupPort> = Arc::new(StubDedupPort);
        // PgPool is required by the struct but we won't hit any SQL in these tests.
        // We create a repo with a stub pool placeholder — only hash_cache is tested.
        FileBlobReadRepository {
            pool: Arc::new(
                // Use an intentionally invalid URL; tests never reach PG.
                sqlx::pool::PoolOptions::<sqlx::Postgres>::new()
                    .max_connections(1)
                    .connect_lazy("postgres://invalid:5432/none")
                    .unwrap(),
            ),
            dedup,
            hash_cache: Cache::builder()
                .max_capacity(10_000)
                .time_to_idle(Duration::from_secs(30))
                .build(),
        }
    }

    #[tokio::test]
    async fn test_cache_insert_and_consume() {
        let repo = make_repo();

        // Insert a hash
        repo.hash_cache
            .insert("file-1".to_string(), "abc123".to_string());

        // First read should return the cached value
        let cached = repo.hash_cache.get("file-1");
        assert_eq!(cached.as_deref(), Some("abc123"));

        // Simulate the one-shot consume pattern used in resolve_blob_hash
        repo.hash_cache.invalidate("file-1");
        assert!(
            repo.hash_cache.get("file-1").is_none(),
            "Entry must be gone after invalidation"
        );
    }

    #[tokio::test]
    async fn test_cache_miss_returns_none() {
        let repo = make_repo();

        assert!(
            repo.hash_cache.get("nonexistent").is_none(),
            "Cache miss must return None"
        );
    }

    #[tokio::test]
    async fn test_cache_multiple_files_independent() {
        let repo = make_repo();

        repo.hash_cache
            .insert("file-a".to_string(), "hash-a".to_string());
        repo.hash_cache
            .insert("file-b".to_string(), "hash-b".to_string());

        // Consuming file-a should not affect file-b
        assert_eq!(repo.hash_cache.get("file-a").as_deref(), Some("hash-a"));
        repo.hash_cache.invalidate("file-a");

        assert!(repo.hash_cache.get("file-a").is_none());
        assert_eq!(
            repo.hash_cache.get("file-b").as_deref(),
            Some("hash-b"),
            "Independent entries must not interfere"
        );
    }

    #[tokio::test]
    async fn test_cache_overwrite_updates_value() {
        let repo = make_repo();

        repo.hash_cache
            .insert("file-1".to_string(), "old-hash".to_string());
        repo.hash_cache
            .insert("file-1".to_string(), "new-hash".to_string());

        assert_eq!(
            repo.hash_cache.get("file-1").as_deref(),
            Some("new-hash"),
            "Last insert wins"
        );
    }

    #[tokio::test]
    async fn test_cache_capacity_eviction() {
        // Build a tiny cache to verify eviction behaviour
        let repo = FileBlobReadRepository {
            pool: Arc::new(
                sqlx::pool::PoolOptions::<sqlx::Postgres>::new()
                    .max_connections(1)
                    .connect_lazy("postgres://invalid:5432/none")
                    .unwrap(),
            ),
            dedup: Arc::new(StubDedupPort),
            hash_cache: Cache::builder()
                .max_capacity(2) // only 2 entries
                .build(),
        };

        repo.hash_cache.insert("a".to_string(), "ha".to_string());
        repo.hash_cache.insert("b".to_string(), "hb".to_string());
        repo.hash_cache.insert("c".to_string(), "hc".to_string());

        // Force moka to run pending eviction tasks
        repo.hash_cache.run_pending_tasks();

        // At most 2 entries should survive
        let alive = ["a", "b", "c"]
            .iter()
            .filter(|k| repo.hash_cache.get(**k).is_some())
            .count();
        assert!(
            alive <= 2,
            "Cache must evict when capacity is exceeded (alive: {alive})"
        );
    }

    #[tokio::test]
    async fn test_cache_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let repo = Arc::new(make_repo());
        let mut handles = vec![];

        // Spawn 50 threads doing inserts + reads simultaneously
        for i in 0..50 {
            let repo = Arc::clone(&repo);
            handles.push(thread::spawn(move || {
                let key = format!("file-{i}");
                let hash = format!("hash-{i}");
                repo.hash_cache.insert(key.clone(), hash.clone());
                // Read back — should be our value or already evicted, never panic
                let _ = repo.hash_cache.get(&key);
                repo.hash_cache.invalidate(&key);
            }));
        }

        for h in handles {
            h.join()
                .expect("Thread must not panic — no poison possible with moka");
        }
    }
}
