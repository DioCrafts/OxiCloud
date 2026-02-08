//! Cache Ports — Application-layer abstractions for all caching concerns.
//!
//! This module defines ports (traits) for:
//! - **WriteBehindCachePort**: deferred write caching for zero-latency uploads.
//! - **MetadataCachePort**: file/directory metadata caching (existence, size, timestamps).
//! - **ContentCachePort**: hot file content caching (small files served from RAM).
//!
//! The application and interface layers remain independent of the caching
//! implementation details.

use std::path::{Path, PathBuf};
use async_trait::async_trait;
use bytes::Bytes;
use crate::common::errors::DomainError;

/// Statistics for monitoring write-behind cache status.
#[derive(Debug, Clone, Default)]
pub struct WriteBehindStatsDto {
    pub pending_count: usize,
    pub pending_bytes: usize,
    pub total_writes: u64,
    pub total_bytes_written: u64,
    pub cache_hits: u64,
    pub avg_flush_time_us: u64,
}

/// Port for write-behind cache operations.
///
/// Provides deferred write semantics: small files are held in memory
/// and the response is returned immediately, while actual disk writes
/// happen asynchronously in the background.
#[async_trait]
pub trait WriteBehindCachePort: Send + Sync + 'static {
    /// Check if a file size is eligible for write-behind caching.
    fn is_eligible_size(&self, size: usize) -> bool;

    /// Put a file in the pending write cache.
    ///
    /// Returns `Ok(true)` if cached successfully, `Ok(false)` if cache is full.
    async fn put_pending(
        &self,
        file_id: String,
        content: Bytes,
        target_path: PathBuf,
    ) -> Result<bool, DomainError>;

    /// Get content from cache if the file is still pending flush.
    async fn get_pending(&self, file_id: &str) -> Option<Bytes>;

    /// Check if a file is pending flush.
    async fn is_pending(&self, file_id: &str) -> bool;

    /// Force immediate flush of a specific file.
    async fn force_flush(&self, file_id: &str) -> Result<(), DomainError>;

    /// Flush all pending writes immediately.
    async fn flush_all(&self) -> Result<(), DomainError>;

    /// Gracefully shutdown the cache, flushing all pending writes.
    async fn shutdown(&self) -> Result<(), DomainError>;

    /// Get current cache statistics.
    async fn get_stats(&self) -> WriteBehindStatsDto;
}

// ─── Metadata Cache ──────────────────────────────────────────

/// Lightweight DTO for cached file/directory metadata.
#[derive(Debug, Clone)]
pub struct CachedMetadataDto {
    pub path: PathBuf,
    pub exists: bool,
    pub is_file: bool,
    pub size: Option<u64>,
    pub mime_type: Option<String>,
    pub created_at: Option<u64>,
    pub modified_at: Option<u64>,
}

/// Port for file/directory metadata caching.
///
/// Provides fast lookups for existence, size, timestamps and MIME types
/// without hitting the filesystem on every request.
#[async_trait]
pub trait MetadataCachePort: Send + Sync + 'static {
    /// Get cached metadata for a path, or `None` on miss / expired.
    async fn get_metadata(&self, path: &Path) -> Option<CachedMetadataDto>;

    /// Check whether a path is a file (cached). Returns `None` on miss.
    async fn is_file(&self, path: &Path) -> Option<bool>;

    /// Read actual filesystem metadata and update the cache entry.
    async fn refresh_metadata(&self, path: &Path) -> Result<CachedMetadataDto, DomainError>;

    /// Invalidate a single cache entry.
    async fn invalidate(&self, path: &Path);

    /// Invalidate all entries under a directory (recursive prefix match).
    async fn invalidate_directory(&self, dir_path: &Path);
}

// ─── Content Cache ───────────────────────────────────────────

/// Port for hot file content caching (small frequently-accessed files in RAM).
///
/// Implementations should use LRU eviction and respect size limits so that
/// the application layer never needs to know the concrete cache type.
#[async_trait]
pub trait ContentCachePort: Send + Sync + 'static {
    /// Check whether a file of the given size should be cached.
    fn should_cache(&self, size: usize) -> bool;

    /// Get cached content. Returns `(content, etag, content_type)` on hit.
    async fn get(&self, file_id: &str) -> Option<(Bytes, String, String)>;

    /// Store content in the cache (may evict older entries).
    async fn put(&self, file_id: String, content: Bytes, etag: String, content_type: String);

    /// Remove a file from the cache (e.g. on delete/update).
    async fn invalidate(&self, file_id: &str);

    /// Clear the entire cache.
    async fn clear(&self);
}
