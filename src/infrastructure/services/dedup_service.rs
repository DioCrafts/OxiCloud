//! Content-Addressable Storage with Deduplication (PostgreSQL-backed)
//!
//! Implements hash-based deduplication to eliminate redundant file storage.
//! Files are stored by their SHA-256 hash, and multiple references can point
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
//! HashMap, no JSON file, no WAL.  All concurrency is handled by
//! `SELECT … FOR UPDATE` and PostgreSQL transactions.
//!
//! Benefits:
//! - ACID durability — crash-safe, zero orphaned index entries
//! - TOCTOU-free — `SELECT … FOR UPDATE` serialises concurrent mutations
//! - 30-50% storage reduction typical
//! - Faster uploads for existing content (instant dedup)

use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncSeekExt, BufReader};
use tokio_util::io::ReaderStream;

use crate::application::ports::dedup_ports::{
    BlobMetadataDto, DedupPort, DedupResultDto, DedupStatsDto,
};
use crate::domain::errors::{DomainError, ErrorKind};

/// Chunk size for streaming hash calculation (256KB)
const HASH_CHUNK_SIZE: usize = 256 * 1024;

/// Chunk size for streaming file reads (256 KB — 4x fewer iterations)
const STREAM_CHUNK_SIZE: usize = 256 * 1024;

/// Content-Addressable Storage Service (PostgreSQL-backed)
pub struct DedupService {
    /// Root directory for blob storage on the filesystem
    blob_root: PathBuf,
    /// Root directory for temporary files during upload
    temp_root: PathBuf,
    /// PostgreSQL connection pool (dedup index in `storage.blobs`)
    pool: Arc<PgPool>,
}

impl DedupService {
    /// Create a new dedup service backed by PostgreSQL.
    pub fn new(storage_root: &Path, pool: Arc<PgPool>) -> Self {
        let blob_root = storage_root.join(".blobs");
        let temp_root = storage_root.join(".dedup_temp");

        Self {
            blob_root,
            temp_root,
            pool,
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
        for i in 0..=255u8 {
            let prefix = format!("{:02x}", i);
            fs::create_dir_all(self.blob_root.join(&prefix))
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

    /// Calculate SHA-256 hash of content.
    pub fn hash_bytes(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        hex::encode(hasher.finalize())
    }

    /// Calculate SHA-256 hash of a file (streaming).
    pub async fn hash_file(path: &Path) -> std::io::Result<String> {
        let file = File::open(path).await?;
        let mut reader = BufReader::with_capacity(HASH_CHUNK_SIZE, file);
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; HASH_CHUNK_SIZE];

        loop {
            let bytes_read = reader.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        Ok(hex::encode(hasher.finalize()))
    }

    // ── Core store operations ────────────────────────────────────

    /// Store content with deduplication (from bytes).
    ///
    /// Uses `SELECT … FOR UPDATE` + `INSERT … ON CONFLICT` for atomic
    /// upsert — completely TOCTOU-free.
    pub async fn store_bytes(
        &self,
        content: &[u8],
        content_type: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        let size = content.len() as u64;
        let hash = Self::hash_bytes(content);

        // Begin transaction — all index mutations happen atomically
        let mut tx = self.pool.begin().await.map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to begin transaction: {}", e))
        })?;

        // SELECT FOR UPDATE: locks the row if it exists, preventing
        // concurrent remove_reference from deleting it mid-operation
        let existing = sqlx::query_scalar::<_, i32>(
            "SELECT ref_count FROM storage.blobs WHERE hash = $1 FOR UPDATE",
        )
        .bind(&hash)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to check blob: {}", e))
        })?;

        if existing.is_some() {
            // Blob exists — just increment ref_count (still under row lock)
            sqlx::query("UPDATE storage.blobs SET ref_count = ref_count + 1 WHERE hash = $1")
                .bind(&hash)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    DomainError::internal_error(
                        "Dedup",
                        format!("Failed to increment ref_count: {}", e),
                    )
                })?;

            tx.commit().await.map_err(|e| {
                DomainError::internal_error("Dedup", format!("Failed to commit: {}", e))
            })?;

            let blob_path = self.blob_path(&hash);

            tracing::info!("DEDUP HIT: {} ({} bytes saved)", &hash[..12], size);

            return Ok(DedupResultDto::ExistingBlob {
                hash,
                size,
                blob_path,
                saved_bytes: size,
            });
        }

        // Blob is new — write file to disk, then register in PG
        let blob_path = self.blob_path(&hash);

        if let Some(parent) = blob_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                DomainError::internal_error(
                    "Dedup",
                    format!("Failed to create blob directory: {}", e),
                )
            })?;
        }

        // Atomic write: temp file → rename
        let temp_path = self.temp_root.join(format!("{}.tmp", uuid::Uuid::new_v4()));
        fs::write(&temp_path, content).await.map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to write temp blob: {}", e))
        })?;

        fs::rename(&temp_path, &blob_path).await.map_err(|e| {
            let _ = std::fs::remove_file(&temp_path);
            DomainError::internal_error("Dedup", format!("Failed to move blob: {}", e))
        })?;

        // Register in PostgreSQL (ON CONFLICT handles rare race with another writer)
        sqlx::query(
            "INSERT INTO storage.blobs (hash, size, ref_count, content_type)
             VALUES ($1, $2, 1, $3)
             ON CONFLICT (hash) DO UPDATE SET ref_count = storage.blobs.ref_count + 1",
        )
        .bind(&hash)
        .bind(size as i64)
        .bind(&content_type)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to register blob: {}", e))
        })?;

        tx.commit().await.map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to commit: {}", e))
        })?;

        tracing::info!("NEW BLOB: {} ({} bytes)", &hash[..12], size);

        Ok(DedupResultDto::NewBlob {
            hash,
            size,
            blob_path,
        })
    }

    /// Store content with deduplication (streaming from file).
    /// Store content with deduplication (streaming from file).
    ///
    /// If `pre_computed_hash` is `Some`, the file will NOT be re-read for
    /// SHA-256 — saving one full sequential read (the biggest I/O win).
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

        // Begin transaction
        let mut tx = self.pool.begin().await.map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to begin transaction: {}", e))
        })?;

        // SELECT FOR UPDATE
        let existing = sqlx::query_scalar::<_, i32>(
            "SELECT ref_count FROM storage.blobs WHERE hash = $1 FOR UPDATE",
        )
        .bind(&hash)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to check blob: {}", e))
        })?;

        if existing.is_some() {
            // Blob already exists — increment and delete source file
            sqlx::query("UPDATE storage.blobs SET ref_count = ref_count + 1 WHERE hash = $1")
                .bind(&hash)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    DomainError::internal_error(
                        "Dedup",
                        format!("Failed to increment ref_count: {}", e),
                    )
                })?;

            tx.commit().await.map_err(|e| {
                DomainError::internal_error("Dedup", format!("Failed to commit: {}", e))
            })?;

            // Delete source file — we don't need it
            let _ = fs::remove_file(source_path).await;

            let blob_path = self.blob_path(&hash);

            tracing::info!(
                "DEDUP HIT (file): {} ({} bytes saved)",
                &hash[..12],
                file_size
            );

            return Ok(DedupResultDto::ExistingBlob {
                hash,
                size: file_size,
                blob_path,
                saved_bytes: file_size,
            });
        }

        // Move source file to blob store
        let blob_path = self.blob_path(&hash);

        if let Some(parent) = blob_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                DomainError::internal_error(
                    "Dedup",
                    format!("Failed to create blob directory: {}", e),
                )
            })?;
        }

        fs::rename(source_path, &blob_path).await.map_err(|e| {
            DomainError::internal_error(
                "Dedup",
                format!("Failed to move file to blob store: {}", e),
            )
        })?;

        // Register in PostgreSQL
        sqlx::query(
            "INSERT INTO storage.blobs (hash, size, ref_count, content_type)
             VALUES ($1, $2, 1, $3)
             ON CONFLICT (hash) DO UPDATE SET ref_count = storage.blobs.ref_count + 1",
        )
        .bind(&hash)
        .bind(file_size as i64)
        .bind(&content_type)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to register blob: {}", e))
        })?;

        tx.commit().await.map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to commit: {}", e))
        })?;

        tracing::info!("NEW BLOB (file): {} ({} bytes)", &hash[..12], file_size);

        Ok(DedupResultDto::NewBlob {
            hash,
            size: file_size,
            blob_path,
        })
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

    /// Read blob content from the filesystem.
    pub async fn read_blob(&self, hash: &str) -> Result<Vec<u8>, DomainError> {
        let blob_path = self.blob_path(hash);

        fs::read(&blob_path).await.map_err(|e| {
            DomainError::new(
                ErrorKind::NotFound,
                "Blob",
                format!("Failed to read blob {}: {}", hash, e),
            )
        })
    }

    /// Read blob content as Bytes.
    pub async fn read_blob_bytes(&self, hash: &str) -> Result<Bytes, DomainError> {
        self.read_blob(hash).await.map(Bytes::from)
    }

    /// Stream blob content in 64 KB chunks — constant memory (~64 KB per stream).
    ///
    /// Unlike `read_blob()`, this never loads the entire file into RAM.
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
    pub async fn verify_integrity(&self) -> Result<Vec<String>, DomainError> {
        let mut corrupted = Vec::new();

        let rows = sqlx::query_as::<_, (String, i64)>(
            "SELECT hash, size FROM storage.blobs ORDER BY hash",
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to list blobs: {}", e))
        })?;

        for (hash, expected_size) in &rows {
            let blob_path = self.blob_path(hash);

            // Check file exists
            if !blob_path.exists() {
                corrupted.push(format!("{}: file missing on disk", hash));
                continue;
            }

            // Verify hash
            match Self::hash_file(&blob_path).await {
                Ok(actual_hash) => {
                    if actual_hash != *hash {
                        corrupted
                            .push(format!("{}: hash mismatch (actual: {})", hash, actual_hash));
                    }
                }
                Err(e) => {
                    corrupted.push(format!("{}: read error ({})", hash, e));
                }
            }

            // Check size
            if let Ok(file_meta) = fs::metadata(&blob_path).await
                && file_meta.len() != *expected_size as u64
            {
                corrupted.push(format!(
                    "{}: size mismatch (expected: {}, actual: {})",
                    hash,
                    expected_size,
                    file_meta.len()
                ));
            }
        }

        if corrupted.is_empty() {
            tracing::info!("Integrity check passed for {} blobs", rows.len());
        } else {
            tracing::warn!("Integrity check found {} issues", corrupted.len());
        }

        Ok(corrupted)
    }

    /// Garbage collect orphaned blobs (ref_count = 0).
    ///
    /// Uses `DELETE … RETURNING` for an atomic "find and remove" operation.
    pub async fn garbage_collect(&self) -> Result<(u64, u64), DomainError> {
        let orphans = sqlx::query_as::<_, (String, i64)>(
            "DELETE FROM storage.blobs WHERE ref_count = 0 RETURNING hash, size",
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| {
            DomainError::internal_error("Dedup", format!("Failed to garbage collect: {}", e))
        })?;

        let mut deleted_count = 0u64;
        let mut deleted_bytes = 0u64;

        for (hash, size) in &orphans {
            let blob_path = self.blob_path(hash);
            if let Err(e) = fs::remove_file(&blob_path).await {
                tracing::warn!("Failed to delete orphaned blob file {}: {}", hash, e);
            }
            deleted_count += 1;
            deleted_bytes += *size as u64;
        }

        if deleted_count > 0 {
            tracing::info!(
                "Garbage collected {} blobs ({} bytes)",
                deleted_count,
                deleted_bytes
            );
        }

        Ok((deleted_count, deleted_bytes))
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

    async fn read_blob(&self, hash: &str) -> Result<Vec<u8>, DomainError> {
        self.read_blob(hash).await
    }

    async fn read_blob_bytes(&self, hash: &str) -> Result<Bytes, DomainError> {
        self.read_blob_bytes(hash).await
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

    fn hash_bytes(&self, content: &[u8]) -> String {
        DedupService::hash_bytes(content)
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
