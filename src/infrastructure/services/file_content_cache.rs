use bytes::Bytes;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Configuration for the file content cache
#[derive(Debug, Clone)]
pub struct FileContentCacheConfig {
    /// Maximum size of individual files to cache (bytes)
    pub max_file_size: usize,
    /// Maximum total cache size (bytes)
    pub max_total_size: usize,
    /// Maximum number of entries
    pub max_entries: usize,
}

impl Default for FileContentCacheConfig {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024,   // 10MB max per file
            max_total_size: 512 * 1024 * 1024, // 512MB total cache
            max_entries: 10000,                // Max 10k files
        }
    }
}

impl FileContentCacheConfig {
    /// Create a new configuration with custom values
    pub fn new(max_file_mb: usize, max_total_mb: usize, max_entries: usize) -> Self {
        Self {
            max_file_size: max_file_mb * 1024 * 1024,
            max_total_size: max_total_mb * 1024 * 1024,
            max_entries,
        }
    }
}

/// Cache entry with metadata
#[derive(Clone)]
struct CacheEntry {
    content: Bytes,
    etag: String,
    content_type: String,
}

/// LRU-based file content cache for small/frequently accessed files
///
/// This cache stores the actual content of files in memory for ultra-fast access.
/// It uses an LRU eviction policy and respects memory limits.
pub struct FileContentCache {
    cache: RwLock<LruCache<String, CacheEntry>>,
    config: FileContentCacheConfig,
    current_size: AtomicUsize,
    hits: AtomicUsize,
    misses: AtomicUsize,
}

impl FileContentCache {
    /// Create a new file content cache with the given configuration
    pub fn new(config: FileContentCacheConfig) -> Self {
        let max_entries =
            NonZeroUsize::new(config.max_entries).unwrap_or(NonZeroUsize::new(1000).unwrap());

        info!(
            "Initializing FileContentCache: max_file={}MB, max_total={}MB, max_entries={}",
            config.max_file_size / (1024 * 1024),
            config.max_total_size / (1024 * 1024),
            config.max_entries
        );

        Self {
            cache: RwLock::new(LruCache::new(max_entries)),
            config,
            current_size: AtomicUsize::new(0),
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
        }
    }

    /// Create a cache with default configuration
    pub fn default() -> Self {
        Self::new(FileContentCacheConfig::default())
    }

    /// Check if a file should be cached based on its size
    pub fn should_cache(&self, size: usize) -> bool {
        size <= self.config.max_file_size
    }

    /// Get file content from cache
    ///
    /// Returns (content, etag, content_type) if found
    pub async fn get(&self, file_id: &str) -> Option<(Bytes, String, String)> {
        let mut cache = self.cache.write().await;

        if let Some(entry) = cache.get(file_id) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            debug!("Cache HIT for file: {}", file_id);
            return Some((
                entry.content.clone(),
                entry.etag.clone(),
                entry.content_type.clone(),
            ));
        }

        self.misses.fetch_add(1, Ordering::Relaxed);
        debug!("Cache MISS for file: {}", file_id);
        None
    }

    /// Check if file exists in cache without updating LRU order
    pub async fn contains(&self, file_id: &str) -> bool {
        let cache = self.cache.read().await;
        cache.contains(file_id)
    }

    /// Put file content into cache
    ///
    /// Will evict older entries if necessary to make room.
    /// Will not cache if file is too large.
    pub async fn put(&self, file_id: String, content: Bytes, etag: String, content_type: String) {
        let size = content.len();

        // Don't cache if too large
        if size > self.config.max_file_size {
            debug!("File {} too large to cache: {} bytes", file_id, size);
            return;
        }

        // Evict entries until we have room
        while self.current_size.load(Ordering::Relaxed) + size > self.config.max_total_size {
            let mut cache = self.cache.write().await;
            if let Some((evicted_id, evicted_entry)) = cache.pop_lru() {
                let evicted_size = evicted_entry.content.len();
                self.current_size.fetch_sub(evicted_size, Ordering::Relaxed);
                debug!(
                    "Evicted file {} ({} bytes) from cache",
                    evicted_id, evicted_size
                );
            } else {
                break;
            }
        }

        // Check again after eviction
        if self.current_size.load(Ordering::Relaxed) + size > self.config.max_total_size {
            warn!("Cannot cache file {}: no room after eviction", file_id);
            return;
        }

        let entry = CacheEntry {
            content,
            etag,
            content_type,
        };

        let mut cache = self.cache.write().await;

        // If replacing an existing entry, subtract its size first
        if let Some(old_entry) = cache.peek(&file_id) {
            self.current_size
                .fetch_sub(old_entry.content.len(), Ordering::Relaxed);
        }

        cache.put(file_id.clone(), entry);
        self.current_size.fetch_add(size, Ordering::Relaxed);

        debug!("Cached file {} ({} bytes)", file_id, size);
    }

    /// Remove a file from cache (e.g., when file is deleted or modified)
    pub async fn invalidate(&self, file_id: &str) {
        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.pop(file_id) {
            self.current_size
                .fetch_sub(entry.content.len(), Ordering::Relaxed);
            debug!("Invalidated cache for file: {}", file_id);
        }
    }

    /// Clear the entire cache
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        self.current_size.store(0, Ordering::Relaxed);
        info!("Cache cleared");
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let total = hits + misses;
        let hit_rate = if total > 0 {
            (hits as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        CacheStats {
            current_size_bytes: self.current_size.load(Ordering::Relaxed),
            max_size_bytes: self.config.max_total_size,
            hits,
            misses,
            hit_rate_percent: hit_rate,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub current_size_bytes: usize,
    pub max_size_bytes: usize,
    pub hits: usize,
    pub misses: usize,
    pub hit_rate_percent: f64,
}

/// Thread-safe wrapper for sharing across handlers
pub type SharedFileContentCache = Arc<FileContentCache>;

// ─── ContentCachePort implementation ─────────────────────────

use crate::application::ports::cache_ports::ContentCachePort;
use async_trait::async_trait;

#[async_trait]
impl ContentCachePort for FileContentCache {
    fn should_cache(&self, size: usize) -> bool {
        FileContentCache::should_cache(self, size)
    }

    async fn get(&self, file_id: &str) -> Option<(Bytes, String, String)> {
        FileContentCache::get(self, file_id).await
    }

    async fn put(&self, file_id: String, content: Bytes, etag: String, content_type: String) {
        FileContentCache::put(self, file_id, content, etag, content_type).await
    }

    async fn invalidate(&self, file_id: &str) {
        FileContentCache::invalidate(self, file_id).await
    }

    async fn clear(&self) {
        FileContentCache::clear(self).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_put_get() {
        let cache = FileContentCache::new(FileContentCacheConfig {
            max_file_size: 1024,
            max_total_size: 4096,
            max_entries: 100,
        });

        let content = Bytes::from("Hello, World!");
        cache
            .put(
                "file1".to_string(),
                content.clone(),
                "etag1".to_string(),
                "text/plain".to_string(),
            )
            .await;

        let result = cache.get("file1").await;
        assert!(result.is_some());
        let (cached_content, etag, content_type) = result.unwrap();
        assert_eq!(cached_content, content);
        assert_eq!(etag, "etag1");
        assert_eq!(content_type, "text/plain");
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let cache = FileContentCache::new(FileContentCacheConfig {
            max_file_size: 100,
            max_total_size: 200,
            max_entries: 100,
        });

        // Add first file (100 bytes)
        let content1 = Bytes::from(vec![0u8; 100]);
        cache
            .put(
                "file1".to_string(),
                content1,
                "e1".to_string(),
                "app/bin".to_string(),
            )
            .await;

        // Add second file (100 bytes)
        let content2 = Bytes::from(vec![1u8; 100]);
        cache
            .put(
                "file2".to_string(),
                content2,
                "e2".to_string(),
                "app/bin".to_string(),
            )
            .await;

        // Add third file - should evict file1
        let content3 = Bytes::from(vec![2u8; 100]);
        cache
            .put(
                "file3".to_string(),
                content3,
                "e3".to_string(),
                "app/bin".to_string(),
            )
            .await;

        // file1 should be evicted
        assert!(cache.get("file1").await.is_none());
        // file2 and file3 should exist
        assert!(cache.get("file2").await.is_some());
        assert!(cache.get("file3").await.is_some());
    }

    #[tokio::test]
    async fn test_cache_invalidate() {
        let cache = FileContentCache::new(FileContentCacheConfig::default());

        let content = Bytes::from("test");
        cache
            .put(
                "file1".to_string(),
                content,
                "e".to_string(),
                "t".to_string(),
            )
            .await;

        assert!(cache.get("file1").await.is_some());

        cache.invalidate("file1").await;

        assert!(cache.get("file1").await.is_none());
    }
}
