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
use moka::sync::Cache;
use sqlx::PgPool;
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

    /// Search files with filtering and pagination at database level.
    /// This is much more efficient than loading all files and filtering in memory.
    ///
    /// Note: This implements a simplified version focusing on the key optimizations:
    /// - LIMIT/OFFSET at database level (not loading all rows)
    /// - Basic name filtering
    /// - Sorting at database level
    ///
    /// For full criteria support (file types, date ranges, size ranges),
    /// the search service will continue to use in-memory filtering.
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

        // Build query based on whether we have a folder_id and name filter
        let (rows, total_count) = match (folder_id, &criteria.name_contains) {
            (Some(fid), Some(name)) if !name.is_empty() => {
                // Folder scope + name search
                let name_pattern = format!("%{}%", name.to_lowercase());

                // Count query
                let count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM storage.files fi
                     WHERE fi.user_id = $1::uuid AND fi.folder_id = $2::uuid
                     AND fi.is_trashed = false AND LOWER(fi.name) LIKE $3",
                )
                .bind(user_id)
                .bind(fid)
                .bind(&name_pattern)
                .fetch_one(self.pool.as_ref())
                .await
                .map_err(|e| DomainError::internal_error("FileBlobRead", format!("count: {e}")))?;

                // Data query with LIMIT/OFFSET
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
                )> = sqlx::query_as(&format!(
                    "SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                            fi.size, fi.mime_type,
                            EXTRACT(EPOCH FROM fi.created_at)::bigint,
                            EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                            fi.user_id::text
                       FROM storage.files fi
                       LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
                      WHERE fi.user_id = $1::uuid AND fi.folder_id = $2::uuid
                        AND fi.is_trashed = false AND LOWER(fi.name) LIKE $3
                      ORDER BY {} {}
                      LIMIT $4 OFFSET $5",
                    order_column, order_dir
                ))
                .bind(user_id)
                .bind(fid)
                .bind(&name_pattern)
                .bind(limit)
                .bind(offset)
                .fetch_all(self.pool.as_ref())
                .await
                .map_err(|e| DomainError::internal_error("FileBlobRead", format!("search: {e}")))?;

                (rows, count as usize)
            }
            (Some(fid), None) | (Some(fid), Some(_)) => {
                // Folder scope only (no name filter)
                let count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM storage.files fi
                     WHERE fi.user_id = $1::uuid AND fi.folder_id = $2::uuid
                     AND fi.is_trashed = false",
                )
                .bind(user_id)
                .bind(fid)
                .fetch_one(self.pool.as_ref())
                .await
                .map_err(|e| DomainError::internal_error("FileBlobRead", format!("count: {e}")))?;

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
                )> = sqlx::query_as(&format!(
                    "SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                            fi.size, fi.mime_type,
                            EXTRACT(EPOCH FROM fi.created_at)::bigint,
                            EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                            fi.user_id::text
                       FROM storage.files fi
                       LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
                      WHERE fi.user_id = $1::uuid AND fi.folder_id = $2::uuid
                        AND fi.is_trashed = false
                      ORDER BY {} {}
                      LIMIT $3 OFFSET $4",
                    order_column, order_dir
                ))
                .bind(user_id)
                .bind(fid)
                .bind(limit)
                .bind(offset)
                .fetch_all(self.pool.as_ref())
                .await
                .map_err(|e| DomainError::internal_error("FileBlobRead", format!("search: {e}")))?;

                (rows, count as usize)
            }
            (None, Some(name)) if !name.is_empty() => {
                // Global search with name filter
                let name_pattern = format!("%{}%", name.to_lowercase());

                let count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM storage.files fi
                     WHERE fi.user_id = $1::uuid AND fi.is_trashed = false
                     AND LOWER(fi.name) LIKE $2",
                )
                .bind(user_id)
                .bind(&name_pattern)
                .fetch_one(self.pool.as_ref())
                .await
                .map_err(|e| DomainError::internal_error("FileBlobRead", format!("count: {e}")))?;

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
                )> = sqlx::query_as(&format!(
                    "SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                            fi.size, fi.mime_type,
                            EXTRACT(EPOCH FROM fi.created_at)::bigint,
                            EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                            fi.user_id::text
                       FROM storage.files fi
                       LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
                      WHERE fi.user_id = $1::uuid AND fi.is_trashed = false
                        AND LOWER(fi.name) LIKE $2
                      ORDER BY {} {}
                      LIMIT $3 OFFSET $4",
                    order_column, order_dir
                ))
                .bind(user_id)
                .bind(&name_pattern)
                .bind(limit)
                .bind(offset)
                .fetch_all(self.pool.as_ref())
                .await
                .map_err(|e| DomainError::internal_error("FileBlobRead", format!("search: {e}")))?;

                (rows, count as usize)
            }
            (None, _) => {
                // No folder scope, no name filter - get all files for user
                let count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM storage.files fi
                     WHERE fi.user_id = $1::uuid AND fi.is_trashed = false",
                )
                .bind(user_id)
                .fetch_one(self.pool.as_ref())
                .await
                .map_err(|e| DomainError::internal_error("FileBlobRead", format!("count: {e}")))?;

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
                )> = sqlx::query_as(&format!(
                    "SELECT fi.id::text, fi.name, fi.folder_id::text, fo.path,
                            fi.size, fi.mime_type,
                            EXTRACT(EPOCH FROM fi.created_at)::bigint,
                            EXTRACT(EPOCH FROM fi.updated_at)::bigint,
                            fi.user_id::text
                       FROM storage.files fi
                       LEFT JOIN storage.folders fo ON fo.id = fi.folder_id
                      WHERE fi.user_id = $1::uuid AND fi.is_trashed = false
                      ORDER BY {} {}
                      LIMIT $2 OFFSET $3",
                    order_column, order_dir
                ))
                .bind(user_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(self.pool.as_ref())
                .await
                .map_err(|e| DomainError::internal_error("FileBlobRead", format!("search: {e}")))?;

                (rows, count as usize)
            }
        };

        let files = rows
            .into_iter()
            .map(|(id, name, fid, fpath, size, mime, ca, ma, uid)| {
                Self::row_to_file(id, name, fid, fpath, size, mime, ca, ma, uid)
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| DomainError::internal_error("FileBlobRead", format!("mapping: {e}")))?;

        Ok((files, total_count))
    }

    /// Count files matching the search criteria (without loading them).
    async fn count_files(
        &self,
        folder_id: Option<&str>,
        criteria: &SearchCriteriaDto,
        user_id: &str,
    ) -> Result<usize, DomainError> {
        // Simplified count - delegates to search_files_paginated for actual counting
        // In a full implementation, this would be a separate optimized query
        let (_, count) = self
            .search_files_paginated(folder_id, criteria, user_id)
            .await?;
        Ok(count)
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

        repo.hash_cache
            .insert("a".to_string(), "ha".to_string());
        repo.hash_cache
            .insert("b".to_string(), "hb".to_string());
        repo.hash_cache
            .insert("c".to_string(), "hc".to_string());

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
            h.join().expect("Thread must not panic — no poison possible with moka");
        }
    }
}
