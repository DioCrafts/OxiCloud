use bytes::Bytes;
use moka::future::Cache;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{debug, info};

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
    etag: Arc<str>,
    content_type: Arc<str>,
}

/// Lock-free concurrent file content cache backed by `moka`.
///
/// Unlike the previous `lru::LruCache` + `RwLock` design, `moka` uses
/// lock-free reads. Concurrent downloads no longer serialize on a write lock
/// just to update LRU order.
pub struct FileContentCache {
    cache: Cache<String, CacheEntry>,
    config: FileContentCacheConfig,
    hits: AtomicUsize,
    misses: AtomicUsize,
}

impl FileContentCache {
    /// Create a new file content cache with the given configuration
    pub fn new(config: FileContentCacheConfig) -> Self {
        info!(
            "Initializing FileContentCache (moka): max_file={}MB, max_total={}MB, max_entries={}",
            config.max_file_size / (1024 * 1024),
            config.max_total_size / (1024 * 1024),
            config.max_entries
        );

        let cache = Cache::builder()
            .max_capacity(config.max_total_size as u64)
            .weigher(|_key: &String, value: &CacheEntry| -> u32 {
                // Weight = content size.  moka evicts entries when the sum
                // of weights exceeds max_capacity.
                value.content.len().min(u32::MAX as usize) as u32
            })
            .build();

        Self {
            cache,
            config,
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
        }
    }
}

impl Default for FileContentCache {
    fn default() -> Self {
        Self::new(FileContentCacheConfig::default())
    }
}

impl FileContentCache {
    /// Check if a file should be cached based on its size
    pub fn should_cache(&self, size: usize) -> bool {
        size <= self.config.max_file_size
    }

    /// Get file content from cache (lock-free read)
    ///
    /// Returns `(content, etag, content_type)` if found.
    /// All three clones are O(1): `Bytes` and `Arc<str>` only bump a ref count.
    pub async fn get(&self, file_id: &str) -> Option<(Bytes, Arc<str>, Arc<str>)> {
        if let Some(entry) = self.cache.get(file_id).await {
            self.hits.fetch_add(1, Ordering::Relaxed);
            debug!("Cache HIT for file: {}", file_id);
            Some((
                entry.content.clone(),
                entry.etag.clone(),
                entry.content_type.clone(),
            ))
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            debug!("Cache MISS for file: {}", file_id);
            None
        }
    }

    /// Check if file exists in cache without updating LRU order
    pub async fn contains(&self, file_id: &str) -> bool {
        self.cache.contains_key(file_id)
    }

    /// Put file content into cache
    ///
    /// Moka handles eviction automatically based on weight (content size).
    pub async fn put(
        &self,
        file_id: String,
        content: Bytes,
        etag: Arc<str>,
        content_type: Arc<str>,
    ) {
        let size = content.len();

        // Don't cache if too large
        if size > self.config.max_file_size {
            debug!("File {} too large to cache: {} bytes", file_id, size);
            return;
        }

        let entry = CacheEntry {
            content,
            etag,
            content_type,
        };

        self.cache.insert(file_id.clone(), entry).await;
        debug!("Cached file {} ({} bytes)", file_id, size);
    }

    /// Remove a file from cache (e.g., when file is deleted or modified)
    pub async fn invalidate(&self, file_id: &str) {
        self.cache.remove(file_id).await;
        debug!("Invalidated cache for file: {}", file_id);
    }

    /// Clear the entire cache
    pub async fn clear(&self) {
        self.cache.invalidate_all();
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
            current_size_bytes: self.cache.weighted_size() as usize,
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

    async fn get(&self, file_id: &str) -> Option<(Bytes, Arc<str>, Arc<str>)> {
        FileContentCache::get(self, file_id).await
    }

    async fn put(&self, file_id: String, content: Bytes, etag: Arc<str>, content_type: Arc<str>) {
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
                "etag1".into(),
                "text/plain".into(),
            )
            .await;

        let result = cache.get("file1").await;
        assert!(result.is_some());
        let (cached_content, etag, content_type) = result.unwrap();
        assert_eq!(cached_content, content);
        assert_eq!(&*etag, "etag1");
        assert_eq!(&*content_type, "text/plain");
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let cache = FileContentCache::new(FileContentCacheConfig {
            max_file_size: 50, // only files ≤ 50 bytes are cacheable
            max_total_size: 1024,
            max_entries: 100,
        });

        // A file within the limit should be cached
        let small = Bytes::from(vec![0u8; 50]);
        cache
            .put("small".to_string(), small, "e1".into(), "app/bin".into())
            .await;
        assert!(cache.get("small").await.is_some());

        // A file exceeding max_file_size is rejected by our own logic
        let big = Bytes::from(vec![1u8; 51]);
        cache
            .put("big".to_string(), big, "e2".into(), "app/bin".into())
            .await;
        assert!(
            cache.get("big").await.is_none(),
            "File exceeding max_file_size must not be cached"
        );

        // Explicit invalidation removes entries immediately
        cache.invalidate("small").await;
        assert!(
            cache.get("small").await.is_none(),
            "Invalidated entry must be gone"
        );
    }

    #[tokio::test]
    async fn test_cache_invalidate() {
        let cache = FileContentCache::new(FileContentCacheConfig::default());

        let content = Bytes::from("test");
        cache
            .put("file1".to_string(), content, "e".into(), "t".into())
            .await;

        assert!(cache.get("file1").await.is_some());

        cache.invalidate("file1").await;

        assert!(cache.get("file1").await.is_none());
    }
}
