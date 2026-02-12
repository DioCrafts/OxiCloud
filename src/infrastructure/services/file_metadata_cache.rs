use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant, UNIX_EPOCH};
use tokio::fs;
use tokio::sync::RwLock;
use tokio::time;
use futures::future::BoxFuture;
use tracing::debug;
use mime_guess::from_path;

use crate::domain::entities::file::File;

use crate::common::config::AppConfig;

/// Cache entry types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheEntryType {
    /// File
    File,
    /// Directory
    Directory,
    /// Unknown type
    Unknown,
}

/// Cache statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: usize,
    /// Number of cache misses
    pub misses: usize,
    /// Number of manual invalidations
    pub invalidations: usize,
    /// Number of automatic expirations
    pub expirations: usize,
    /// Number of cache inserts
    pub inserts: usize,
    /// Total time saved (milliseconds)
    pub time_saved_ms: u64,
}

/// Complete cached file metadata
#[derive(Debug, Clone)]
pub struct FileMetadata {
    /// Absolute file path
    pub path: PathBuf,
    /// Whether the file physically exists
    pub exists: bool,
    /// Entry type (file, directory)
    pub entry_type: CacheEntryType,
    /// Size in bytes (for files)
    pub size: Option<u64>,
    /// MIME type (for files)
    pub mime_type: Option<String>,
    /// Creation timestamp (UNIX epoch seconds)
    pub created_at: Option<u64>,
    /// Modification timestamp (UNIX epoch seconds)
    pub modified_at: Option<u64>,
    /// Previous access (used for LRU)
    pub last_access: Instant,
    /// Cache expiration time
    pub expires_at: Instant,
    /// Number of accesses to this entry
    pub access_count: usize,
}

impl FileMetadata {
    /// Creates a new metadata entry
    pub fn new(
        path: PathBuf,
        exists: bool,
        entry_type: CacheEntryType,
        size: Option<u64>,
        mime_type: Option<String>,
        created_at: Option<u64>,
        modified_at: Option<u64>,
        ttl: Duration,
    ) -> Self {
        let now = Instant::now();
        
        Self {
            path,
            exists,
            entry_type,
            size,
            mime_type,
            created_at,
            modified_at,
            last_access: now,
            expires_at: now + ttl,
            access_count: 1,
        }
    }
    
    /// Updates the last access time
    pub fn touch(&mut self) {
        self.last_access = Instant::now();
        self.access_count += 1;
    }
    
    /// Checks if the entry has expired
    pub fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }
    
    /// Updates the expiration time with a new TTL
    pub fn update_expiry(&mut self, ttl: Duration) {
        self.expires_at = Instant::now() + ttl;
    }
}

/// Advanced file metadata cache
pub struct FileMetadataCache {
    /// Main metadata cache
    metadata_cache: RwLock<HashMap<PathBuf, FileMetadata>>,
    /// LRU queue for cache management
    lru_queue: RwLock<VecDeque<PathBuf>>,
    /// Cache usage statistics
    stats: RwLock<CacheStats>,
    /// Global application configuration
    config: AppConfig,
    /// Adaptive TTL for popular entries
    ttl_multiplier: f64,
    /// Popularity threshold for extended TTL
    popularity_threshold: usize,
    /// Maximum cache size
    max_entries: usize,
}

impl FileMetadataCache {
    /// Creates a new metadata cache instance
    pub fn new(config: AppConfig, max_entries: usize) -> Self {
        Self {
            metadata_cache: RwLock::new(HashMap::with_capacity(max_entries)),
            lru_queue: RwLock::new(VecDeque::with_capacity(max_entries)),
            stats: RwLock::new(CacheStats::default()),
            config,
            ttl_multiplier: 5.0, // Popular entries have 5x TTL
            popularity_threshold: 10, // After 10 accesses it's considered popular
            max_entries,
        }
    }
    
    /// Creates a FileMetadata object from a File object
    pub fn create_metadata_from_file(file: &File, abs_path: PathBuf) -> FileMetadata {
        let entry_type = CacheEntryType::File;
        let size = Some(file.size());
        let mime_type = Some(file.mime_type().to_string());
        let created_at = Some(file.created_at());
        let modified_at = Some(file.modified_at());
        
        // Use a standard TTL
        let ttl = Duration::from_secs(60); // 1 minute
        
        FileMetadata::new(
            abs_path,
            true,
            entry_type,
            size,
            mime_type,
            created_at,
            modified_at,
            ttl,
        )
    }
    
    /// Creates a default instance
    pub fn default() -> Self {
        Self::new(AppConfig::default(), 10_000)
    }
    
    /// Creates a cache instance with default configuration
    pub fn default_with_config(config: AppConfig) -> Self {
        Self::new(config, 50_000) // Larger cache for production system
    }
    
    /// Gets file metadata if cached
    pub async fn get_metadata(&self, path: &Path) -> Option<FileMetadata> {
        let start_time = Instant::now();
        let mut cache = self.metadata_cache.write().await;
        
        if let Some(metadata) = cache.get_mut(path) {
            // Check if expired
            if metadata.is_expired() {
                // Remove from cache if expired
                cache.remove(path);
                
                // Update statistics
                let mut stats = self.stats.write().await;
                stats.misses += 1;
                stats.expirations += 1;
                
                debug!("Cache entry expired for: {}", path.display());
                
                return None;
            }
            
            // Update access time
            metadata.touch();
            
            // For popular entries, extend TTL
            if metadata.access_count >= self.popularity_threshold {
                let new_ttl = match metadata.entry_type {
                    CacheEntryType::File => Duration::from_millis(
                        (self.config.timeouts.file_operation_ms as f64 * self.ttl_multiplier) as u64
                    ),
                    CacheEntryType::Directory => Duration::from_millis(
                        (self.config.timeouts.dir_operation_ms as f64 * self.ttl_multiplier) as u64
                    ),
                    _ => Duration::from_secs(60), // 1 minute by default
                };
                
                metadata.update_expiry(new_ttl);
                debug!("Extended TTL for popular entry: {}", path.display());
            }
            
            // Calculate approximate time saved
            let elapsed = start_time.elapsed().as_millis() as u64;
            let estimated_io_time: u64 = 10; // We assume 10ms minimum for IO operation
            let time_saved = estimated_io_time.saturating_sub(elapsed);
            
            // Update statistics
            let mut stats = self.stats.write().await;
            stats.hits += 1;
            stats.time_saved_ms += time_saved;
            
            debug!("Cache hit for: {}", path.display());
            
            // Also keep the LRU queue updated
            self.update_lru(path.to_path_buf()).await;
            
            // Clone to return
            return Some(metadata.clone());
        }
        
        // Not found in cache
        let mut stats = self.stats.write().await;
        stats.misses += 1;
        
        debug!("Cache miss for: {}", path.display());
        None
    }
    
    /// Updates the LRU queue
    async fn update_lru(&self, path: PathBuf) {
        let mut lru = self.lru_queue.write().await;
        
        // Remove if already exists
        if let Some(pos) = lru.iter().position(|p| p == &path) {
            lru.remove(pos);
        }
        
        // Add to the end (most recent)
        lru.push_back(path);
    }
    
    /// Checks if a file exists
    pub async fn exists(&self, path: &Path) -> Option<bool> {
        if let Some(metadata) = self.get_metadata(path).await {
            return Some(metadata.exists);
        }
        
        None
    }
    
    /// Checks if a path is a directory
    pub async fn is_dir(&self, path: &Path) -> Option<bool> {
        if let Some(metadata) = self.get_metadata(path).await {
            return Some(metadata.entry_type == CacheEntryType::Directory);
        }
        
        None
    }
    
    /// Checks if a path is a file
    pub async fn is_file(&self, path: &Path) -> Option<bool> {
        if let Some(metadata) = self.get_metadata(path).await {
            return Some(metadata.entry_type == CacheEntryType::File);
        }
        
        None
    }
    
    /// Gets the size of a file
    pub async fn get_size(&self, path: &Path) -> Option<u64> {
        if let Some(metadata) = self.get_metadata(path).await {
            return metadata.size;
        }
        
        None
    }
    
    /// Gets the MIME type of a file
    pub async fn get_mime_type(&self, path: &Path) -> Option<String> {
        if let Some(metadata) = self.get_metadata(path).await {
            return metadata.mime_type;
        }
        
        None
    }
    
    /// Refreshes metadata for a path
    pub async fn refresh_metadata(&self, path: &Path) -> Result<FileMetadata, std::io::Error> {
        // Perform actual filesystem read
        let metadata = fs::metadata(path).await?;
        
        // Determine entry type
        let entry_type = if metadata.is_dir() {
            CacheEntryType::Directory
        } else if metadata.is_file() {
            CacheEntryType::File
        } else {
            CacheEntryType::Unknown
        };
        
        // Get size for files
        let size = if metadata.is_file() {
            Some(metadata.len())
        } else {
            None
        };
        
        // Get MIME type for files
        let mime_type = if metadata.is_file() {
            Some(from_path(path).first_or_octet_stream().to_string())
        } else {
            None
        };
        
        // Get timestamps
        let created_at = metadata.created()
            .map(|time| time.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs())
            .ok();
            
        let modified_at = metadata.modified()
            .map(|time| time.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs())
            .ok();
        
        // Determine appropriate TTL
        let ttl = if metadata.is_dir() {
            Duration::from_millis(self.config.timeouts.dir_operation_ms)
        } else {
            Duration::from_millis(self.config.timeouts.file_operation_ms)
        };
        
        // Create metadata entry
        let file_metadata = FileMetadata::new(
            path.to_path_buf(),
            true,
            entry_type,
            size,
            mime_type,
            created_at,
            modified_at,
            ttl,
        );
        
        // Update cache
        self.update_cache(file_metadata.clone()).await;
        
        Ok(file_metadata)
    }
    
    /// Updates the cache with new metadata
    pub async fn update_cache(&self, metadata: FileMetadata) {
        // Avoid full cache before inserting
        self.ensure_capacity().await;
        
        let path = metadata.path.clone();
        
        // Insert into cache
        {
            let mut cache = self.metadata_cache.write().await;
            cache.insert(path.clone(), metadata);
            
            // Update statistics
            let mut stats = self.stats.write().await;
            stats.inserts += 1;
        }
        
        // Update the LRU queue
        self.update_lru(path).await;
    }
    
    /// Ensures there is space in the cache
    async fn ensure_capacity(&self) {
        let cache_size = {
            let cache = self.metadata_cache.read().await;
            cache.len()
        };
        
        if cache_size >= self.max_entries {
            self.evict_lru_entries(cache_size / 10).await; // Free up 10%
        }
    }
    
    /// Removes least recently used entries
    async fn evict_lru_entries(&self, count: usize) {
        let mut paths_to_remove = Vec::with_capacity(count);
        
        // Get entries to remove from the LRU queue
        {
            let mut lru = self.lru_queue.write().await;
            for _ in 0..count {
                if let Some(path) = lru.pop_front() {
                    paths_to_remove.push(path);
                } else {
                    break;
                }
            }
        }
        
        // Remove from the main cache
        {
            let mut cache = self.metadata_cache.write().await;
            for path in paths_to_remove {
                cache.remove(&path);
            }
        }
        
        debug!("Evicted {} LRU entries from cache", count);
    }
    
    /// Invalidate a specific cache entry
    pub async fn invalidate(&self, path: &Path) {
        // Remove from the main cache
        {
            let mut cache = self.metadata_cache.write().await;
            cache.remove(path);
            
            // Update statistics
            let mut stats = self.stats.write().await;
            stats.invalidations += 1;
        }
        
        // Remove from the LRU queue
        let path_buf = path.to_path_buf();
        {
            let mut lru = self.lru_queue.write().await;
            if let Some(pos) = lru.iter().position(|p| p == &path_buf) {
                lru.remove(pos);
            }
        }
        
        debug!("Invalidated cache entry for: {}", path.display());
    }
    
    /// Recursively invalidate entries under a directory
    pub async fn invalidate_directory(&self, dir_path: &Path) {
        let dir_str = dir_path.to_string_lossy().to_string();
        let mut paths_to_remove = Vec::new();
        
        // Find all paths that start with the directory
        {
            let cache = self.metadata_cache.read().await;
            for path in cache.keys() {
                let path_str = path.to_string_lossy().to_string();
                if path_str.starts_with(&dir_str) {
                    paths_to_remove.push(path.clone());
                }
            }
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.invalidations += paths_to_remove.len();
        }
        
        // Remove each found path
        for path in paths_to_remove {
            self.invalidate(&path).await;
        }
        
        debug!("Invalidated directory and contents: {}", dir_path.display());
    }
    
    /// Get current cache statistics
    pub async fn get_stats(&self) -> CacheStats {
        let stats = self.stats.read().await;
        stats.clone()
    }
    
    /// Clears all expired entries from the cache
    pub async fn clear_expired(&self) {
        let now = Instant::now();
        let mut paths_to_remove = Vec::new();
        
        // Find expired entries
        {
            let cache = self.metadata_cache.read().await;
            for (path, metadata) in cache.iter() {
                if now > metadata.expires_at {
                    paths_to_remove.push(path.clone());
                }
            }
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.expirations += paths_to_remove.len();
        }
        
        // Save the number of entries for logging
        let num_paths = paths_to_remove.len();
        
        // Remove expired entries
        for path in paths_to_remove {
            self.invalidate(&path).await;
        }
        
        debug!("Cleared {} expired entries from cache", num_paths);
    }
    
    /// Starts the periodic cleanup process
    pub fn start_cleanup_task(cache: Arc<Self>) -> BoxFuture<'static, ()> {
        Box::pin(async move {
            let cleanup_interval = Duration::from_secs(60); // Every minute
            
            loop {
                // Wait for the interval
                time::sleep(cleanup_interval).await;
                
                // Clean expired entries
                cache.clear_expired().await;
                
                // Log statistics
                let stats = cache.get_stats().await;
                let cache_size = {
                    let cache_map = cache.metadata_cache.read().await;
                    cache_map.len()
                };
                
                debug!(
                    "Cache stats: size={}, hits={}, misses={}, hit_ratio={:.2}%, time_saved={}ms",
                    cache_size,
                    stats.hits,
                    stats.misses,
                    if stats.hits + stats.misses > 0 { 
                        (stats.hits as f64 * 100.0) / (stats.hits + stats.misses) as f64
                    } else { 
                        0.0 
                    },
                    stats.time_saved_ms
                );
            }
        })
    }
    
    /// Preloads metadata for entire directories (useful for initialization)
    pub async fn preload_directory(&self, dir_path: &Path, recursive: bool, max_depth: usize) -> Result<usize, std::io::Error> {
        self._preload_directory_internal(dir_path, recursive, max_depth, 0).await
    }
    
    /// Internal preload implementation with depth tracking
    async fn _preload_directory_internal(
        &self, 
        dir_path: &Path, 
        recursive: bool, 
        max_depth: usize, 
        current_depth: usize
    ) -> Result<usize, std::io::Error> {
        Box::pin(async move {
        if current_depth > max_depth {
            return Ok(0);
        }
        
        // Get directory entries
        let mut entries = fs::read_dir(dir_path).await?;
        let mut count = 0;
        
        // Process each entry
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = fs::metadata(&path).await?;
            
            // Refresh metadata for this entry
            self.refresh_metadata(&path).await?;
            count += 1;
            
            // Recursively process subdirectories if needed
            if recursive && metadata.is_dir() {
                // Box to break recursion
                count += self._preload_directory_internal(
                    &path, 
                    recursive, 
                    max_depth, 
                    current_depth + 1
                ).await?;
            }
        }
        
        Ok(count)
    }).await
    }
}

// ─── MetadataCachePort implementation ────────────────────────

use async_trait::async_trait;
use crate::application::ports::cache_ports::{MetadataCachePort, CachedMetadataDto};
use crate::common::errors::DomainError;

#[async_trait]
impl MetadataCachePort for FileMetadataCache {
    async fn get_metadata(&self, path: &Path) -> Option<CachedMetadataDto> {
        // Delegate to the existing rich get_metadata, then project into the DTO.
        let fm = FileMetadataCache::get_metadata(self, path).await?;
        Some(CachedMetadataDto {
            path: fm.path,
            exists: fm.exists,
            is_file: fm.entry_type == CacheEntryType::File,
            size: fm.size,
            mime_type: fm.mime_type,
            created_at: fm.created_at,
            modified_at: fm.modified_at,
        })
    }

    async fn is_file(&self, path: &Path) -> Option<bool> {
        FileMetadataCache::is_file(self, path).await
    }

    async fn refresh_metadata(&self, path: &Path) -> Result<CachedMetadataDto, DomainError> {
        let fm = FileMetadataCache::refresh_metadata(self, path)
            .await
            .map_err(|e| DomainError::internal_error("MetadataCache", e.to_string()))?;
        Ok(CachedMetadataDto {
            path: fm.path,
            exists: fm.exists,
            is_file: fm.entry_type == CacheEntryType::File,
            size: fm.size,
            mime_type: fm.mime_type,
            created_at: fm.created_at,
            modified_at: fm.modified_at,
        })
    }

    async fn invalidate(&self, path: &Path) {
        FileMetadataCache::invalidate(self, path).await
    }

    async fn invalidate_directory(&self, dir_path: &Path) {
        FileMetadataCache::invalidate_directory(self, dir_path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;
    
    #[tokio::test]
    async fn test_cache_operations() {
        // Create temporary directory for tests
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        
        // Create a test file
        let mut file = File::create(&file_path).await.unwrap();
        file.write_all(b"test content").await.unwrap();
        file.flush().await.unwrap();
        drop(file);
        
        // Create cache
        let config = AppConfig::default();
        let cache = FileMetadataCache::new(config, 1000);
        
        // Verify initial miss
        assert!(cache.exists(&file_path).await.is_none());
        
        // Refresh and verify hit
        let metadata = cache.refresh_metadata(&file_path).await.unwrap();
        assert_eq!(metadata.entry_type, CacheEntryType::File);
        assert_eq!(metadata.size, Some(12)); // "test content" = 12 bytes
        
        // Verify it now exists in cache
        assert_eq!(cache.exists(&file_path).await, Some(true));
        assert_eq!(cache.is_file(&file_path).await, Some(true));
        
        // Invalidate and verify it no longer exists in cache
        cache.invalidate(&file_path).await;
        assert!(cache.exists(&file_path).await.is_none());
        
        // Verify statistics
        let stats = cache.get_stats().await;
        assert_eq!(stats.inserts, 1);
        assert_eq!(stats.invalidations, 1);
        assert!(stats.hits > 0);
    }
    
    #[tokio::test]
    async fn test_directory_operations() {
        // Create directory structure for tests
        let temp_dir = tempdir().unwrap();
        // Canonicalize to handle macOS /var -> /private/var symlinks
        let base_path = temp_dir.path().canonicalize().unwrap();
        let sub_dir = base_path.join("subdir");
        fs::create_dir(&sub_dir).await.unwrap();
        
        let file1 = base_path.join("file1.txt");
        let file2 = sub_dir.join("file2.txt");
        
        File::create(&file1).await.unwrap();
        File::create(&file2).await.unwrap();
        
        // Create cache
        let config = AppConfig::default();
        let cache = FileMetadataCache::new(config, 1000);
        
        // Preload directory recursively
        // preload_directory caches the *contents* of the directory, not the root itself
        let count = cache.preload_directory(&base_path, true, 2).await.unwrap();
        assert_eq!(count, 3); // subdir, file1, file2
        
        // Verify existence in cache (only contents, not the root)
        assert_eq!(cache.is_dir(&sub_dir).await, Some(true));
        assert_eq!(cache.is_file(&file1).await, Some(true));
        assert_eq!(cache.is_file(&file2).await, Some(true));
        
        // Invalidate directory and contents
        cache.invalidate_directory(&base_path).await;
        
        // Verify nothing exists in cache
        assert!(cache.exists(&sub_dir).await.is_none());
        assert!(cache.exists(&file1).await.is_none());
        assert!(cache.exists(&file2).await.is_none());
    }
}