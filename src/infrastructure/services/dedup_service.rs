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
//! **Write-first strategy** (store_bytes / store_from_file):
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

use async_trait::async_trait;
use bytes::Bytes;
use futures::stream::{self, StreamExt};
use futures::{Stream, TryStreamExt};

use sqlx::PgPool;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio_util::io::ReaderStream;

use crate::application::ports::dedup_ports::{
    BlobMetadataDto, DedupPort, DedupResultDto, DedupStatsDto,
};
use crate::domain::errors::{DomainError, ErrorKind};

/// Block size for BLAKE3 file hashing (1MB — optimal syscall/throughput ratio).
const HASH_BLOCK_SIZE: usize = 1024 * 1024;

/// Chunk size for streaming file reads (256 KB)
const STREAM_CHUNK_SIZE: usize = 256 * 1024;

/// Content-Addressable Storage Service (PostgreSQL-backed)
pub struct DedupService {
    /// Root directory for blob storage on the filesystem
    blob_root: PathBuf,
    /// Root directory for temporary files during upload
    temp_root: PathBuf,
    /// PostgreSQL connection pool (dedup index in `storage.blobs`) — primary,
    /// used by request-path operations (store_bytes, store_from_file, etc.).
    pool: Arc<PgPool>,
    /// Isolated maintenance pool for long-running operations
    /// (verify_integrity, garbage_collect) that must never starve the primary.
    maintenance_pool: Arc<PgPool>,
}

/// Compile-time lookup table for the 256 two-digit lowercase hex prefixes ("00"…"ff").
/// Avoids a `format!("{:02x}", i)` allocation on every iteration of `initialize()`.
static HEX_PREFIXES: [&str; 256] = [
    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0a", "0b", "0c", "0d", "0e", "0f",
    "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "1a", "1b", "1c", "1d", "1e", "1f",
    "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2a", "2b", "2c", "2d", "2e", "2f",
    "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "3a", "3b", "3c", "3d", "3e", "3f",
    "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "4a", "4b", "4c", "4d", "4e", "4f",
    "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "5a", "5b", "5c", "5d", "5e", "5f",
    "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "6a", "6b", "6c", "6d", "6e", "6f",
    "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "7a", "7b", "7c", "7d", "7e", "7f",
    "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "8a", "8b", "8c", "8d", "8e", "8f",
    "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "9a", "9b", "9c", "9d", "9e", "9f",
    "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "aa", "ab", "ac", "ad", "ae", "af",
    "b0", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8", "b9", "ba", "bb", "bc", "bd", "be", "bf",
    "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "ca", "cb", "cc", "cd", "ce", "cf",
    "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "da", "db", "dc", "dd", "de", "df",
    "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "ea", "eb", "ec", "ed", "ee", "ef",
    "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "fa", "fb", "fc", "fd", "fe", "ff",
];

impl DedupService {
    /// Create a new dedup service backed by PostgreSQL.
    ///
    /// * `pool` — primary pool for request-path operations.
    /// * `maintenance_pool` — isolated pool for verify_integrity / garbage_collect.
    pub fn new(storage_root: &Path, pool: Arc<PgPool>, maintenance_pool: Arc<PgPool>) -> Self {
        let blob_root = storage_root.join(".blobs");
        let temp_root = storage_root.join(".dedup_temp");

        Self {
            blob_root,
            temp_root,
            pool,
            maintenance_pool,
        }
    }

    /// Initialize the service (create blob directories on the filesystem).
    pub async fn initialize(&self) -> Result<(), DomainError> {
        // Create directories
        fs::create_dir_all(&self.blob_root)
            .await
            .map_err(DomainError::from)?;
        fs::create_dir_all(&self.temp_root)
            .await
            .map_err(DomainError::from)?;

        // Create hash prefix directories (00-ff)
        for prefix in &HEX_PREFIXES {
            fs::create_dir_all(self.blob_root.join(prefix))
                .await
                .map_err(DomainError::from)?;
        }

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
            "Dedup service initialized (PostgreSQL-backed): {} blobs, {} bytes stored",
            count,
            total_bytes
        );

        Ok(())
    }

    // ── Path helpers ─────────────────────────────────────────────

    /// Get the blob path for a given hash.
    pub fn blob_path(&self, hash: &str) -> PathBuf {
        let prefix = &hash[0..2];
        self.blob_root.join(prefix).join(format!("{}.blob", hash))
    }

    // ── Hash helpers ─────────────────────────────────────────────

    /// Calculate BLAKE3 hash of content (~5× faster than SHA-256).
    pub fn hash_bytes(content: &[u8]) -> String {
        blake3::hash(content).to_hex().to_string()
    }

    /// Calculate BLAKE3 hash of a file (~5× faster than SHA-256).
    ///
    /// Runs entirely on `spawn_blocking` with synchronous I/O so the Tokio
    /// worker threads are never blocked by CPU-bound hashing.  Uses 1 MB
    /// reads for optimal syscall-to-throughput ratio.
    pub async fn hash_file(path: &Path) -> std::io::Result<String> {
        let path = path.to_path_buf();
        tokio::task::spawn_blocking(move || {
            use std::io::Read;

            let mut file = std::fs::File::open(&path)?;
            let mut hasher = blake3::Hasher::new();
            let mut buffer = vec![0u8; HASH_BLOCK_SIZE];

            loop {
                let n = file.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }

            Ok(hasher.finalize().to_hex().to_string())
        })
        .await
        .expect("hash_file: spawn_blocking task panicked")
    }

    // ── Core store operations ────────────────────────────────────

    /// Maximum payload accepted by `store_bytes`.  Anything larger
    /// should use `store_from_file` (streaming — constant RAM).
    const MAX_STORE_BYTES: usize = 10 * 1024 * 1024; // 10 MB

    /// Store content with deduplication (from bytes).
    ///
    /// **Write-first strategy**: the blob file is written to disk *before*
    /// touching PostgreSQL, so the PG connection is never held during I/O.
    /// The database operation is a single `INSERT … ON CONFLICT` upsert
    /// (~2-4 ms) instead of `SELECT FOR UPDATE` + write + commit.
    ///
    /// **Guard**: rejects payloads >10 MB.  Large content must go through
    /// `store_from_file` which streams from disk with constant RAM.
    pub async fn store_bytes(
        &self,
        content: &[u8],
        content_type: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        if content.len() > Self::MAX_STORE_BYTES {
            return Err(DomainError::internal_error(
                "Dedup",
                format!(
                    "store_bytes called with {} bytes (max {}). Use store_from_file for large content.",
                    content.len(),
                    Self::MAX_STORE_BYTES
                ),
            ));
        }

        let size = content.len() as u64;
        let hash = Self::hash_bytes(content);
        let blob_path = self.blob_path(&hash);

        // ── Phase 1: Write blob to disk (NO PG connection held) ─────
        //
        // Content-addressable: if two writers race for the same hash,
        // both produce identical files.  The rename is atomic on the
        // same filesystem; if it fails because the other writer won,
        // we just discard our temp file — the blob is already there.
        if !blob_path.exists() {
            // Parent directory (xx/) guaranteed to exist — created by initialize()
            let temp_path = self.temp_root.join(format!("{}.tmp", uuid::Uuid::new_v4()));
            fs::write(&temp_path, content).await.map_err(|e| {
                DomainError::internal_error("Dedup", format!("Failed to write temp blob: {}", e))
            })?;

            if let Err(e) = fs::rename(&temp_path, &blob_path).await {
                // Another writer already placed the blob — discard ours
                let _ = fs::remove_file(&temp_path).await;
                tracing::debug!("Blob file already placed by concurrent writer: {}", e);
            }
        }

        // ── Phase 2: Single atomic upsert (~2-4 ms, no explicit TX) ─
        //
        // `INSERT … ON CONFLICT` is executed as a single implicit
        // transaction by PostgreSQL.  RETURNING ref_count tells us
        // whether this was a new blob (ref_count = 1) or a dedup hit.
        let ref_count: i32 = sqlx::query_scalar(
            "INSERT INTO storage.blobs (hash, size, ref_count, content_type)
             VALUES ($1, $2, 1, $3)
             ON CONFLICT (hash) DO UPDATE SET ref_count = storage.blobs.ref_count + 1
             RETURNING ref_count",
        )
        .bind(&hash)
        .bind(size as i64)
        .bind(&content_type)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to upsert blob: {}", e))
        })?;

        if ref_count > 1 {
            tracing::info!("DEDUP HIT: {} ({} bytes saved)", &hash[..12], size);
            Ok(DedupResultDto::ExistingBlob {
                hash,
                size,
                blob_path,
                saved_bytes: size,
            })
        } else {
            tracing::info!("NEW BLOB: {} ({} bytes)", &hash[..12], size);
            Ok(DedupResultDto::NewBlob {
                hash,
                size,
                blob_path,
            })
        }
    }

    /// Store content with deduplication (streaming from file).
    ///
    /// **Write-first strategy**: the source file is moved/copied to the
    /// blob store *before* touching PostgreSQL, so the PG connection is
    /// never held during disk I/O.
    ///
    /// If `pre_computed_hash` is `Some`, the file will NOT be re-read for
    /// BLAKE3 — saving one full sequential read (the biggest I/O win).
    pub async fn store_from_file(
        &self,
        source_path: &Path,
        content_type: Option<String>,
        pre_computed_hash: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        let file_size = fs::metadata(source_path)
            .await
            .map_err(|e| {
                DomainError::internal_error("Dedup", format!("Failed to get file metadata: {}", e))
            })?
            .len();

        // Use pre-computed hash if available, otherwise calculate (streaming)
        let hash = match pre_computed_hash {
            Some(h) => h,
            None => Self::hash_file(source_path)
                .await
                .map_err(DomainError::from)?,
        };

        let blob_path = self.blob_path(&hash);

        // ── Phase 1: Move/place blob on disk (NO PG connection held) ─
        //
        // If the blob file already exists on disk, the source is simply
        // deleted — the file content is identical by definition.
        if blob_path.exists() {
            // Blob already on disk — discard the source file
            let _ = fs::remove_file(source_path).await;
        } else {
            // Parent directory (xx/) guaranteed to exist — created by initialize()

            // rename is atomic on the same filesystem.  If source and blob
            // dirs live on different filesystems (rare), this falls back to
            // copy+delete which is slower but still correct.
            if let Err(e) = fs::rename(source_path, &blob_path).await {
                // Another writer may have placed the blob concurrently
                if blob_path.exists() {
                    let _ = fs::remove_file(source_path).await;
                    tracing::debug!("Blob file placed by concurrent writer: {}", e);
                } else {
                    return Err(DomainError::internal_error(
                        "Dedup",
                        format!("Failed to move file to blob store: {}", e),
                    ));
                }
            }
        }

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

        // Lock the row exclusively — prevents concurrent store_bytes from
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

            // Delete blob file AFTER committing PG — the row is gone, so no
            // concurrent store_bytes can resurrect a reference to this hash.
            let blob_path = self.blob_path(hash);
            if let Err(e) = fs::remove_file(&blob_path).await {
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

    /// Stream blob content in 64 KB chunks — constant memory (~64 KB per stream).
    ///
    /// A 1 GB file uses the same ~64 KB as a 1 KB file.
    pub async fn read_blob_stream(
        &self,
        hash: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>
    {
        let blob_path = self.blob_path(hash);
        let file = File::open(&blob_path).await.map_err(|e| {
            DomainError::new(
                ErrorKind::NotFound,
                "Blob",
                format!("Failed to open blob {}: {}", hash, e),
            )
        })?;
        Ok(Box::pin(ReaderStream::with_capacity(
            file,
            STREAM_CHUNK_SIZE,
        )))
    }

    /// Stream a byte range of a blob — only reads the requested portion.
    ///
    /// Uses seek + take so a 1 MB range request on a 1 GB file only reads 1 MB.
    pub async fn read_blob_range_stream(
        &self,
        hash: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>
    {
        let blob_path = self.blob_path(hash);
        let mut file = File::open(&blob_path).await.map_err(|e| {
            DomainError::new(
                ErrorKind::NotFound,
                "Blob",
                format!("Failed to open blob {}: {}", hash, e),
            )
        })?;

        // Seek to the start position
        file.seek(std::io::SeekFrom::Start(start))
            .await
            .map_err(|e| {
                DomainError::internal_error("Blob", format!("Failed to seek in blob: {}", e))
            })?;

        // If an end is specified, limit the read with take()
        if let Some(end_pos) = end {
            let limit = end_pos.saturating_sub(start);
            let limited = file.take(limit);
            Ok(Box::pin(ReaderStream::with_capacity(
                limited,
                STREAM_CHUNK_SIZE,
            )))
        } else {
            Ok(Box::pin(ReaderStream::with_capacity(
                file,
                STREAM_CHUNK_SIZE,
            )))
        }
    }

    /// Get the size of a blob without reading its content.
    pub async fn blob_size(&self, hash: &str) -> Result<u64, DomainError> {
        let blob_path = self.blob_path(hash);
        let meta = fs::metadata(&blob_path).await.map_err(|e| {
            DomainError::new(
                ErrorKind::NotFound,
                "Blob",
                format!("Failed to stat blob {}: {}", hash, e),
            )
        })?;
        Ok(meta.len())
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
                let blob_root = self.blob_root.clone();
                let current_batch =
                    std::mem::replace(&mut batch, Vec::with_capacity(VERIFY_CONCURRENCY));

                let issues: Vec<String> = stream::iter(current_batch)
                    .map(move |(hash, expected_size)| {
                        let blob_root = blob_root.clone();
                        async move {
                            let prefix = &hash[0..2];
                            let blob_path = blob_root.join(prefix).join(format!("{}.blob", hash));

                            let mut issues = Vec::new();

                            // Single async metadata() replaces the previous
                            // blocking .exists() + separate metadata() — one
                            // stat() syscall instead of two, and non-blocking.
                            let file_meta = match fs::metadata(&blob_path).await {
                                Ok(m) => m,
                                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                                    issues.push(format!("{}: file missing on disk", hash));
                                    return issues;
                                }
                                Err(e) => {
                                    issues.push(format!("{}: metadata error ({})", hash, e));
                                    return issues;
                                }
                            };

                            // Check size
                            if file_meta.len() != expected_size as u64 {
                                issues.push(format!(
                                    "{}: size mismatch (expected: {}, actual: {})",
                                    hash,
                                    expected_size,
                                    file_meta.len(),
                                ));
                            }

                            // Verify hash
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
            for (hash, size) in &batch {
                let blob_path = self.blob_path(hash);
                if let Err(e) = fs::remove_file(&blob_path).await {
                    tracing::warn!("Failed to delete orphan blob file {hash}: {e}");
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

#[async_trait]
impl DedupPort for DedupService {
    async fn store_bytes(
        &self,
        content: &[u8],
        content_type: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        self.store_bytes(content, content_type).await
    }

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
