//! Content-Addressable Storage with Deduplication (PostgreSQL-backed)
//!
//! Implements hash-based deduplication to eliminate redundant file storage.
//! Files are stored by their BLAKE3 hash, and multiple references can point
//! to the same physical blob.
//!
//! Architecture:
//! ```text
//! ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
//! │ storage.files   │────▶│ storage.blobs   │────▶│ Blob Store      │
//! │ (references)    │     │ (PG dedup index)│     │ (.blobs/ on FS) │
//! └─────────────────┘     └─────────────────┘     └─────────────────┘
//! ```
//!
//! The dedup index lives in PostgreSQL (`storage.blobs`) — no in-memory
//! HashMap, no JSON file, no WAL.
//!
//! **Write-first strategy** (store_from_file):
//!   1. Write/move the blob file to disk *before* touching PostgreSQL.
//!   2. Single `INSERT … ON CONFLICT … RETURNING ref_count` upsert
//!      (~2-4 ms) — no explicit transaction, no `SELECT FOR UPDATE`.
//!   3. PG connection is never held during disk I/O.
//!
//! `remove_reference` retains `SELECT … FOR UPDATE` inside a short
//! transaction because it must atomically decide whether to delete the
//! row *and* the blob file.
//!
//! Benefits:
//! - ACID durability — crash-safe, zero orphaned index entries
//! - PG connections never blocked by disk I/O (write-first)
//! - 30-50% storage reduction typical
//! - Faster uploads for existing content (instant dedup)

use bytes::Bytes;
use futures::stream::{self, StreamExt};
use futures::{Stream, TryStreamExt};

use sqlx::PgPool;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use tokio::fs;

use crate::application::ports::blob_storage_ports::BlobStorageBackend;
use crate::application::ports::dedup_ports::{
    BlobMetadataDto, DedupPort, DedupResultDto, DedupStatsDto,
};
use crate::domain::errors::{DomainError, ErrorKind};

/// Content-Addressable Storage Service (PostgreSQL-backed)
///
/// Delegates all byte-level I/O to a [`BlobStorageBackend`] implementation
/// (local filesystem, S3, etc.) while keeping BLAKE3 hashing, ref-counting
/// and the PostgreSQL dedup index here.
pub struct DedupService {
    /// Pluggable blob storage backend (local FS, S3, …).
    backend: Arc<dyn BlobStorageBackend>,
    /// PostgreSQL connection pool (dedup index in `storage.blobs`) — primary,
    /// used by request-path operations (store_from_file, etc.).
    pool: Arc<PgPool>,
    /// Isolated maintenance pool for long-running operations
    /// (verify_integrity, garbage_collect) that must never starve the primary.
    maintenance_pool: Arc<PgPool>,
}

impl DedupService {
    /// Create a new dedup service backed by PostgreSQL.
    ///
    /// * `backend` — pluggable blob storage (local filesystem, S3, etc.).
    /// * `pool` — primary pool for request-path operations.
    /// * `maintenance_pool` — isolated pool for verify_integrity / garbage_collect.
    pub fn new(
        backend: Arc<dyn BlobStorageBackend>,
        pool: Arc<PgPool>,
        maintenance_pool: Arc<PgPool>,
    ) -> Self {
        Self {
            backend,
            pool,
            maintenance_pool,
        }
    }

    /// Creates a stub instance for testing — never hits PG or the filesystem.
    #[cfg(any(test, feature = "integration_tests"))]
    pub fn new_stub() -> Self {
        use crate::infrastructure::services::local_blob_backend::LocalBlobBackend;
        let stub_pool = Arc::new(
            sqlx::pool::PoolOptions::<sqlx::Postgres>::new()
                .max_connections(1)
                .connect_lazy("postgres://invalid:5432/none")
                .unwrap(),
        );
        Self {
            backend: Arc::new(LocalBlobBackend::new(Path::new("/tmp/oxicloud_stub_blobs"))),
            pool: stub_pool.clone(),
            maintenance_pool: stub_pool,
        }
    }

    /// Initialize the service (delegate to backend + log stats from PG).
    pub async fn initialize(&self) -> Result<(), DomainError> {
        self.backend.initialize().await?;

        // Log existing blob stats from PG
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM storage.blobs")
            .fetch_one(self.pool.as_ref())
            .await
            .unwrap_or(0);

        let total_bytes: i64 =
            sqlx::query_scalar("SELECT COALESCE(SUM(size), 0) FROM storage.blobs")
                .fetch_one(self.pool.as_ref())
                .await
                .unwrap_or(0);

        tracing::info!(
            "Dedup service initialized (backend={}): {} blobs, {} bytes stored",
            self.backend.backend_type(),
            count,
            total_bytes
        );

        Ok(())
    }

    /// Return a reference to the underlying blob storage backend.
    pub fn backend(&self) -> &Arc<dyn BlobStorageBackend> {
        &self.backend
    }

    // ── Path helpers ─────────────────────────────────────────────

    /// Get the local blob path for a given hash (if the backend supports it).
    pub fn blob_path(&self, hash: &str) -> PathBuf {
        self.backend
            .local_blob_path(hash)
            .unwrap_or_else(|| PathBuf::from(format!("remote://{}", hash)))
    }

    // ── Hash helpers ─────────────────────────────────────────────

    /// Calculate BLAKE3 hash of a file (~5× faster than SHA-256).
    ///
    /// Runs entirely on `spawn_blocking` with synchronous I/O so the Tokio
    /// worker threads are never blocked by CPU-bound hashing.
    ///
    /// Uses memory-mapped I/O (`update_mmap_rayon`) which avoids loading the
    /// entire file into the heap.  The OS pages in data on demand and BLAKE3
    /// parallelises the computation across all available cores via rayon.
    /// Peak RAM for a 500 MB file is only a few MB of active pages instead
    /// of the full 500 MB.
    pub async fn hash_file(path: &Path) -> std::io::Result<String> {
        let path = path.to_path_buf();
        tokio::task::spawn_blocking(move || {
            let mut hasher = blake3::Hasher::new();
            hasher.update_mmap_rayon(&path)?;
            Ok(hasher.finalize().to_hex().to_string())
        })
        .await
        .expect("hash_file: spawn_blocking task panicked")
    }

    // ── Core store operations ────────────────────────────────────

    /// Store content with deduplication (streaming from file).
    ///
    /// **Write-first strategy**: the source file is moved/uploaded to the
    /// blob backend *before* touching PostgreSQL, so the PG connection is
    /// never held during I/O.
    ///
    /// If `pre_computed_hash` is `Some`, the file will NOT be re-read for
    /// BLAKE3 — saving one full sequential read (the biggest I/O win).
    pub async fn store_from_file(
        &self,
        source_path: &Path,
        content_type: Option<String>,
        pre_computed_hash: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        // Use pre-computed hash if available, otherwise calculate (streaming)
        let hash = match pre_computed_hash {
            Some(h) => h,
            None => Self::hash_file(source_path)
                .await
                .map_err(DomainError::from)?,
        };

        // ── Phase 1: Place blob in backend (NO PG connection held) ───
        let file_size = self.backend.put_blob(&hash, source_path).await?;

        let blob_path = self.blob_path(&hash);

        // ── Phase 2: Single atomic upsert (~2-4 ms, no explicit TX) ─
        let ref_count: i32 = sqlx::query_scalar(
            "INSERT INTO storage.blobs (hash, size, ref_count, content_type)
             VALUES ($1, $2, 1, $3)
             ON CONFLICT (hash) DO UPDATE SET ref_count = storage.blobs.ref_count + 1
             RETURNING ref_count",
        )
        .bind(&hash)
        .bind(file_size as i64)
        .bind(&content_type)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to upsert blob: {}", e))
        })?;

        if ref_count > 1 {
            tracing::info!(
                "DEDUP HIT (file): {} ({} bytes saved)",
                &hash[..12],
                file_size
            );
            Ok(DedupResultDto::ExistingBlob {
                hash,
                size: file_size,
                blob_path,
                saved_bytes: file_size,
            })
        } else {
            tracing::info!("NEW BLOB (file): {} ({} bytes)", &hash[..12], file_size);
            Ok(DedupResultDto::NewBlob {
                hash,
                size: file_size,
                blob_path,
            })
        }
    }

    // ── Reference counting ───────────────────────────────────────

    /// Check if a blob with the given hash exists in the PG index.
    pub async fn blob_exists(&self, hash: &str) -> bool {
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM storage.blobs WHERE hash = $1)")
            .bind(hash)
            .fetch_one(self.pool.as_ref())
            .await
            .unwrap_or(false)
    }

    /// Returns `true` if `user_id` owns at least one (non-trashed) file that
    /// references the blob identified by `hash`.
    ///
    /// Used by the dedup API handlers to enforce per-user access control on
    /// the content-addressed blob store.
    pub async fn user_owns_blob_reference(&self, hash: &str, user_id: &str) -> bool {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM storage.files WHERE blob_hash = $1 AND user_id = $2 AND NOT is_trashed)",
        )
        .bind(hash)
        .bind(user_id)
        .fetch_one(self.pool.as_ref())
        .await
        .unwrap_or(false)
    }

    /// Get metadata for a blob from PostgreSQL.
    pub async fn get_blob_metadata(&self, hash: &str) -> Option<BlobMetadataDto> {
        let row = sqlx::query_as::<_, (String, i64, i32, Option<String>)>(
            "SELECT hash, size, ref_count, content_type FROM storage.blobs WHERE hash = $1",
        )
        .bind(hash)
        .fetch_optional(self.pool.as_ref())
        .await
        .ok()
        .flatten()?;

        Some(BlobMetadataDto {
            hash: row.0,
            size: row.1 as u64,
            ref_count: row.2 as u32,
            content_type: row.3,
        })
    }

    /// Add a reference to a blob (increment ref_count).
    pub async fn add_reference(&self, hash: &str) -> Result<(), DomainError> {
        let rows_affected =
            sqlx::query("UPDATE storage.blobs SET ref_count = ref_count + 1 WHERE hash = $1")
                .bind(hash)
                .execute(self.pool.as_ref())
                .await
                .map_err(|e| {
                    DomainError::internal_error(
                        "Dedup",
                        format!("Failed to increment ref_count: {}", e),
                    )
                })?
                .rows_affected();

        if rows_affected == 0 {
            return Err(DomainError::new(
                ErrorKind::NotFound,
                "Blob",
                format!("Blob not found: {}", hash),
            ));
        }

        Ok(())
    }

    /// Remove a reference from a blob.
    ///
    /// Uses a single transaction with `SELECT … FOR UPDATE` to atomically
    /// decrement ref_count and delete the row + blob file if it reaches 0.
    /// Returns `true` if the blob was deleted.
    pub async fn remove_reference(&self, hash: &str) -> Result<bool, DomainError> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to begin transaction: {}", e))
        })?;

        // Lock the row exclusively — prevents concurrent store_from_file from
        // incrementing ref_count while we might be deleting
        let row = sqlx::query_as::<_, (i32, i64)>(
            "SELECT ref_count, size FROM storage.blobs WHERE hash = $1 FOR UPDATE",
        )
        .bind(hash)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to lock blob row: {}", e))
        })?;

        let Some((ref_count, _size)) = row else {
            // Blob doesn't exist — nothing to do
            tx.rollback().await.ok();
            return Ok(false);
        };

        let new_ref_count = (ref_count - 1).max(0);

        if new_ref_count == 0 {
            // Last reference — delete row from PG
            sqlx::query("DELETE FROM storage.blobs WHERE hash = $1")
                .bind(hash)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    DomainError::internal_error(
                        "Dedup",
                        format!("Failed to delete blob row: {}", e),
                    )
                })?;

            tx.commit().await.map_err(|e| {
                DomainError::internal_error("Dedup", format!("Failed to commit: {}", e))
            })?;

            // Delete blob from backend AFTER committing PG — the row is gone,
            // so no concurrent store_from_file can resurrect a reference.
            if let Err(e) = self.backend.delete_blob(hash).await {
                tracing::warn!("Failed to delete blob file {}: {}", hash, e);
            }

            tracing::info!("BLOB DELETED: {} (no more references)", &hash[..12]);
            Ok(true)
        } else {
            // Still has references — just decrement
            sqlx::query("UPDATE storage.blobs SET ref_count = $1 WHERE hash = $2")
                .bind(new_ref_count)
                .bind(hash)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    DomainError::internal_error(
                        "Dedup",
                        format!("Failed to decrement ref_count: {}", e),
                    )
                })?;

            tx.commit().await.map_err(|e| {
                DomainError::internal_error("Dedup", format!("Failed to commit: {}", e))
            })?;

            tracing::debug!("Reference removed from blob {}", &hash[..12]);
            Ok(false)
        }
    }

    // ── Read operations ──────────────────────────────────────────

    /// Stream blob content in chunks — constant memory usage.
    pub async fn read_blob_stream(
        &self,
        hash: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>
    {
        self.backend.get_blob_stream(hash).await
    }

    /// Stream a byte range of a blob — only reads the requested portion.
    pub async fn read_blob_range_stream(
        &self,
        hash: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>
    {
        self.backend.get_blob_range_stream(hash, start, end).await
    }

    /// Get the size of a blob without reading its content.
    pub async fn blob_size(&self, hash: &str) -> Result<u64, DomainError> {
        self.backend.blob_size(hash).await
    }

    // ── Statistics (computed from PG) ────────────────────────────

    /// Get deduplication statistics by querying PostgreSQL.
    pub async fn get_stats(&self) -> DedupStatsDto {
        let row = sqlx::query_as::<_, (i64, i64, i64)>(
            "SELECT
                 COUNT(*)                                    AS total_blobs,
                 COALESCE(SUM(size), 0)                     AS total_bytes_stored,
                 COALESCE(SUM(size::BIGINT * ref_count), 0) AS total_bytes_referenced
             FROM storage.blobs",
        )
        .fetch_one(self.pool.as_ref())
        .await
        .unwrap_or((0, 0, 0));

        let total_blobs = row.0 as u64;
        let total_bytes_stored = row.1 as u64;
        let total_bytes_referenced = row.2 as u64;
        let bytes_saved = total_bytes_referenced.saturating_sub(total_bytes_stored);
        let dedup_ratio = if total_bytes_stored > 0 {
            total_bytes_referenced as f64 / total_bytes_stored as f64
        } else {
            1.0
        };

        DedupStatsDto {
            total_blobs,
            total_bytes_stored,
            total_bytes_referenced,
            bytes_saved,
            dedup_hits: 0, // Not tracked per-session — derive from SUM(ref_count - 1)
            dedup_ratio,
        }
    }

    // ── Maintenance ──────────────────────────────────────────────

    /// Verify integrity of all blobs (PG index vs filesystem).
    ///
    /// Uses a **streaming cursor** (`fetch()`) so memory stays O(batch)
    /// instead of O(total_blobs).  Blobs are verified in micro-batches
    /// of `VERIFY_CONCURRENCY` using `buffer_unordered`.
    pub async fn verify_integrity(&self) -> Result<Vec<String>, DomainError> {
        /// Max blobs verified concurrently.  Each spawns a blocking
        /// thread for BLAKE3 so this also caps blocking-pool pressure.
        const VERIFY_CONCURRENCY: usize = 16;

        let mut row_stream = sqlx::query_as::<_, (String, i64)>(
            "SELECT hash, size FROM storage.blobs ORDER BY hash",
        )
        .fetch(self.maintenance_pool.as_ref());

        let mut total = 0usize;
        let mut corrupted = Vec::<String>::new();
        let mut batch = Vec::with_capacity(VERIFY_CONCURRENCY);

        loop {
            let maybe_row = row_stream.try_next().await.map_err(|e| {
                DomainError::internal_error("Dedup", format!("Failed to list blobs: {}", e))
            })?;

            let is_done = maybe_row.is_none();

            if let Some(row) = maybe_row {
                total += 1;
                batch.push(row);
            }

            // Flush when batch is full or we've exhausted the cursor
            if batch.len() >= VERIFY_CONCURRENCY || (is_done && !batch.is_empty()) {
                let backend = self.backend.clone();
                let current_batch =
                    std::mem::replace(&mut batch, Vec::with_capacity(VERIFY_CONCURRENCY));

                let issues: Vec<String> = stream::iter(current_batch)
                    .map(move |(hash, expected_size)| {
                        let backend = backend.clone();
                        async move {
                            let mut issues = Vec::new();

                            // Check existence + size via backend
                            match backend.blob_size(&hash).await {
                                Ok(actual_size) => {
                                    if actual_size != expected_size as u64 {
                                        issues.push(format!(
                                            "{}: size mismatch (expected: {}, actual: {})",
                                            hash, expected_size, actual_size,
                                        ));
                                    }
                                }
                                Err(_) => {
                                    issues.push(format!("{}: blob missing in backend", hash));
                                    return issues;
                                }
                            };

                            // Verify hash — only possible for local backends
                            if let Some(blob_path) = backend.local_blob_path(&hash) {
                                match Self::hash_file(&blob_path).await {
                                    Ok(actual_hash) => {
                                        if actual_hash != hash {
                                            issues.push(format!(
                                                "{}: hash mismatch (actual: {})",
                                                hash, actual_hash,
                                            ));
                                        }
                                    }
                                    Err(e) => {
                                        issues.push(format!("{}: read error ({})", hash, e));
                                    }
                                }
                            }

                            issues
                        }
                    })
                    .buffer_unordered(VERIFY_CONCURRENCY)
                    .flat_map(stream::iter)
                    .collect()
                    .await;

                corrupted.extend(issues);
            }

            if is_done {
                break;
            }
        }

        if corrupted.is_empty() {
            tracing::info!("Integrity check passed for {} blobs", total);
        } else {
            tracing::warn!("Integrity check found {} issues", corrupted.len());
        }

        Ok(corrupted)
    }

    /// Garbage collect orphaned blobs (ref_count = 0).
    ///
    /// Deletes in small batches (BATCH_SIZE rows per TX) so that each
    /// transaction lasts only a few milliseconds.  This avoids:
    /// - massive row-lock accumulation in PostgreSQL,
    /// - WAL bloat from a single giant DELETE,
    /// - blocking concurrent uploads that touch `storage.blobs`.
    ///
    /// Blob files are removed **after** each batch commits, so a crash
    /// mid-GC only leaves a few orphan files on disk (reclaimed next run).
    pub async fn garbage_collect(&self) -> Result<(u64, u64), DomainError> {
        /// Max rows deleted per mini-transaction.
        const BATCH_SIZE: i64 = 500;

        let mut total_deleted = 0u64;
        let mut total_bytes = 0u64;

        loop {
            // Each DELETE is its own implicit TX — short and bounded.
            // The `ctid` sub-select is the canonical way to do
            // `DELETE … LIMIT` in PostgreSQL.
            let batch: Vec<(String, i64)> = sqlx::query_as(
                "DELETE FROM storage.blobs
                  WHERE ctid = ANY(
                      SELECT ctid FROM storage.blobs
                       WHERE ref_count = 0
                       LIMIT $1
                  )
                  RETURNING hash, size",
            )
            .bind(BATCH_SIZE)
            .fetch_all(self.maintenance_pool.as_ref())
            .await
            .map_err(|e| DomainError::internal_error("Dedup", format!("GC batch failed: {e}")))?;

            if batch.is_empty() {
                break;
            }

            // Delete blob files OUTSIDE the TX (already committed).
            // Also clean up any thumbnail files for these blob hashes
            // (thumbnails are keyed by blob_hash and live under
            // storage_root/.thumbnails/{icon,preview,large}/{hash}.jpg).

            for (hash, size) in &batch {
                if let Err(e) = self.backend.delete_blob(hash).await {
                    tracing::warn!("Failed to delete orphan blob file {hash}: {e}");
                }
                // Remove associated thumbnail files (best-effort, always local)
                if let Some(blob_path) = self.backend.local_blob_path(hash)
                    && let Some(storage_root) = blob_path.ancestors().nth(3)
                {
                    let thumbnails_root = storage_root.join(".thumbnails");
                    for dir in &["icon", "preview", "large"] {
                        let thumb = thumbnails_root.join(dir).join(format!("{hash}.jpg"));
                        let _ = fs::remove_file(&thumb).await;
                    }
                }
                total_bytes += *size as u64;
            }
            total_deleted += batch.len() as u64;

            // Yield so uploads / other tasks are not starved.
            tokio::task::yield_now().await;
        }

        if total_deleted > 0 {
            tracing::info!("GC: removed {total_deleted} blobs ({total_bytes} bytes)");
        }

        Ok((total_deleted, total_bytes))
    }
}

// ─── Port implementation ─────────────────────────────────────────────────────

impl DedupPort for DedupService {
    async fn store_from_file(
        &self,
        source_path: &Path,
        content_type: Option<String>,
        pre_computed_hash: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        self.store_from_file(source_path, content_type, pre_computed_hash)
            .await
    }

    async fn blob_exists(&self, hash: &str) -> bool {
        self.blob_exists(hash).await
    }

    async fn get_blob_metadata(&self, hash: &str) -> Option<BlobMetadataDto> {
        self.get_blob_metadata(hash).await
    }

    async fn read_blob_stream(
        &self,
        hash: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>
    {
        self.read_blob_stream(hash).await
    }

    async fn read_blob_range_stream(
        &self,
        hash: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>
    {
        self.read_blob_range_stream(hash, start, end).await
    }

    async fn blob_size(&self, hash: &str) -> Result<u64, DomainError> {
        self.blob_size(hash).await
    }

    async fn add_reference(&self, hash: &str) -> Result<(), DomainError> {
        self.add_reference(hash).await
    }

    async fn remove_reference(&self, hash: &str) -> Result<bool, DomainError> {
        self.remove_reference(hash).await
    }

    async fn hash_file(&self, path: &Path) -> Result<String, DomainError> {
        DedupService::hash_file(path)
            .await
            .map_err(DomainError::from)
    }

    fn blob_path(&self, hash: &str) -> PathBuf {
        self.blob_path(hash)
    }

    async fn get_stats(&self) -> DedupStatsDto {
        self.get_stats().await
    }

    async fn flush(&self) -> Result<(), DomainError> {
        // No-op: PostgreSQL handles persistence automatically via WAL/commit
        Ok(())
    }

    async fn verify_integrity(&self) -> Result<Vec<String>, DomainError> {
        self.verify_integrity().await
    }
}
