use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock, Semaphore};
use tracing::{debug, error, info, warn};
use async_trait::async_trait;

use crate::domain::services::path_service::StoragePath;
use crate::infrastructure::services::id_mapping_service::{IdMappingService, IdMappingError};
use crate::common::errors::DomainError;
use crate::application::ports::outbound::IdMappingPort;

/// Maximum number of entries in the cache
const MAX_CACHE_SIZE: usize = 10_000;

/// Cache time-to-live (in seconds)
const CACHE_TTL_SECONDS: u64 = 60 * 5; // 5 minutes

/// Optimizer for batch ID mapping operations
pub struct IdMappingOptimizer {
    /// Base ID mapping service
    base_service: Arc<IdMappingService>,
    
    /// Path to ID cache (path -> id)
    path_to_id_cache: RwLock<HashMap<String, (String, Instant)>>,
    
    /// ID to path cache (id -> path)
    id_to_path_cache: RwLock<HashMap<String, (String, Instant)>>,
    
    /// Hit counter
    stats: RwLock<OptimizerStats>,
    
    /// Semaphore to limit batch operations
    batch_limiter: Semaphore,
    
    /// Pending batch queue
    pending_batch: Mutex<BatchQueue>,
}

/// Optimizer statistics
#[derive(Debug, Default, Clone)]
pub struct OptimizerStats {
    /// Total number of get_path_by_id queries
    pub path_by_id_queries: usize,
    /// Number of cache hits for get_path_by_id
    pub path_by_id_hits: usize,
    
    /// Total number of get_or_create_id queries
    pub get_id_queries: usize,
    /// Number of cache hits for get_or_create_id
    pub get_id_hits: usize,
    
    /// Number of batch operations performed
    pub batch_operations: usize,
    /// Total number of IDs processed in batch
    pub batch_items_processed: usize,
    
    /// Last cache cleanup timestamp
    pub last_cleanup: Option<Instant>,
}

/// Queue for batch operations
#[derive(Default)]
struct BatchQueue {
    /// Pending paths to get/create ID
    path_to_id_requests: HashSet<String>,
    /// Pending IDs to get path
    id_to_path_requests: HashSet<String>,
}


/// Result of a batch operation
struct BatchResult {
    /// Path to ID mapping
    path_to_id: HashMap<String, String>,
    /// ID to path mapping
    id_to_path: HashMap<String, String>,
}

impl IdMappingOptimizer {
    /// Creates a new optimizer for the ID mapping service
    pub fn new(base_service: Arc<IdMappingService>) -> Self {
        Self {
            base_service,
            path_to_id_cache: RwLock::new(HashMap::with_capacity(1000)),
            id_to_path_cache: RwLock::new(HashMap::with_capacity(1000)),
            stats: RwLock::new(OptimizerStats::default()),
            batch_limiter: Semaphore::new(2), // Limit to 2 concurrent batch operations
            pending_batch: Mutex::new(BatchQueue::default()),
        }
    }
    
    /// Gets optimizer statistics
    pub async fn get_stats(&self) -> OptimizerStats {
        self.stats.read().await.clone()
    }
    
    /// Cleans expired cache entries
    pub async fn cleanup_cache(&self) {
        let now = Instant::now();
        let ttl = Duration::from_secs(CACHE_TTL_SECONDS);
        
        // Clean path_to_id cache
        {
            let mut cache = self.path_to_id_cache.write().await;
            let initial_size = cache.len();
            
            // Retain only non-expired entries
            cache.retain(|_, (_, timestamp)| {
                now.duration_since(*timestamp) < ttl
            });
            
            let removed = initial_size - cache.len();
            if removed > 0 {
                debug!("Cleaned {} expired entries from path_to_id cache", removed);
            }
        }
        
        // Clean id_to_path cache
        {
            let mut cache = self.id_to_path_cache.write().await;
            let initial_size = cache.len();
            
            // Retain only non-expired entries
            cache.retain(|_, (_, timestamp)| {
                now.duration_since(*timestamp) < ttl
            });
            
            let removed = initial_size - cache.len();
            if removed > 0 {
                debug!("Cleaned {} expired entries from id_to_path cache", removed);
            }
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.last_cleanup = Some(now);
        }
    }
    
    /// Starts periodic cleanup task
    pub fn start_cleanup_task(optimizer: Arc<Self>) {
        tokio::spawn(async move {
            let cleanup_interval = Duration::from_secs(CACHE_TTL_SECONDS / 2);
            
            loop {
                tokio::time::sleep(cleanup_interval).await;
                optimizer.cleanup_cache().await;
                
                // Log statistics periodically
                let stats = optimizer.get_stats().await;
                info!("ID Mapping Optimizer stats - Path queries: {}, hits: {} ({}%), ID queries: {}, hits: {} ({}%), Batch ops: {}, items: {}",
                    stats.path_by_id_queries,
                    stats.path_by_id_hits,
                    if stats.path_by_id_queries > 0 { stats.path_by_id_hits as f64 * 100.0 / stats.path_by_id_queries as f64 } else { 0.0 },
                    stats.get_id_queries,
                    stats.get_id_hits,
                    if stats.get_id_queries > 0 { stats.get_id_hits as f64 * 100.0 / stats.get_id_queries as f64 } else { 0.0 },
                    stats.batch_operations,
                    stats.batch_items_processed
                );
            }
        });
    }
    
    /// Adds a request to the pending queue for batch processing
    async fn queue_path_to_id_request(&self, path: &StoragePath) -> Result<Option<String>, IdMappingError> {
        let path_str = path.to_string();
        
        // Check first in the cache
        {
            let cache = self.path_to_id_cache.read().await;
            if let Some((id, _)) = cache.get(&path_str) {
                // Update statistics
                {
                    let mut stats = self.stats.write().await;
                    stats.get_id_hits += 1;
                }
                
                return Ok(Some(id.clone()));
            }
        }
        
        // If not in cache, add to batch queue
        {
            let mut batch_queue = self.pending_batch.lock().await;
            batch_queue.path_to_id_requests.insert(path_str);
        }
        
        // Not found in cache, must be processed in batch
        Ok(None)
    }
    
    /// Processes pending requests in batch
    async fn process_batch(&self) -> Result<BatchResult, IdMappingError> {
        // Acquire permit for batch operation
        let _permit = self.batch_limiter.acquire().await.unwrap();
        
        // Get pending requests
        let (path_requests, id_requests) = {
            let mut batch_queue = self.pending_batch.lock().await;
            
            let paths = std::mem::take(&mut batch_queue.path_to_id_requests);
            let ids = std::mem::take(&mut batch_queue.id_to_path_requests);
            
            (paths, ids)
        };
        
        // Create results
        let mut result = BatchResult {
            path_to_id: HashMap::with_capacity(path_requests.len()),
            id_to_path: HashMap::with_capacity(id_requests.len()),
        };
        
        // Process path->id requests in batch
        for path_str in path_requests {
            let path = StoragePath::from_string(&path_str);
            match self.base_service.get_or_create_id(&path).await {
                Ok(id) => {
                    result.path_to_id.insert(path_str.clone(), id.clone());
                    result.id_to_path.insert(id, path_str);
                },
                Err(e) => {
                    error!("Error batch-processing path {}: {}", path_str, e);
                    // Continue with remaining requests
                }
            }
        }
        
        // Process id->path requests in batch
        for id in id_requests {
            match self.base_service.get_path_by_id(&id).await {
                Ok(path) => {
                    let path_str = path.to_string();
                    result.id_to_path.insert(id.clone(), path_str.clone());
                    result.path_to_id.insert(path_str, id);
                },
                Err(e) => {
                    error!("Error batch-processing ID {}: {}", id, e);
                    // Continue with remaining requests
                }
            }
        }
        
        // Update cache with batch results
        {
            let mut path_cache = self.path_to_id_cache.write().await;
            let mut id_cache = self.id_to_path_cache.write().await;
            
            let now = Instant::now();
            
            for (path, id) in &result.path_to_id {
                path_cache.insert(path.clone(), (id.clone(), now));
            }
            
            for (id, path) in &result.id_to_path {
                id_cache.insert(id.clone(), (path.clone(), now));
            }
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.batch_operations += 1;
            stats.batch_items_processed += result.path_to_id.len() + result.id_to_path.len();
        }
        
        // Save changes to disk in the background
        let service_clone = self.base_service.clone();
        tokio::spawn(async move {
            if let Err(e) = service_clone.save_pending_changes().await {
                error!("Error saving ID mapping changes: {}", e);
            }
        });
        
        Ok(result)
    }
    
    /// Forces processing of pending requests if there are enough
    async fn trigger_batch_if_needed(&self, min_batch_size: usize) -> Result<(), IdMappingError> {
        // Check if there are enough pending requests
        let should_process = {
            let batch_queue = self.pending_batch.lock().await;
            batch_queue.path_to_id_requests.len() + batch_queue.id_to_path_requests.len() >= min_batch_size
        };
        
        // Process if necessary
        if should_process {
            self.process_batch().await?;
        }
        
        Ok(())
    }
    
    /// Preload a set of paths to get their IDs in batch
    pub async fn preload_paths(&self, paths: Vec<StoragePath>) -> Result<(), IdMappingError> {
        // Only proceed if there are paths to load
        if paths.is_empty() {
            return Ok(());
        }
        
        // Paths we need to load (those not in cache)
        let mut paths_to_load = Vec::new();
        
        // Check cache first
        {
            let cache = self.path_to_id_cache.read().await;
            for path in paths {
                let path_str = path.to_string();
                if !cache.contains_key(&path_str) {
                    paths_to_load.push(path_str);
                }
            }
        }
        
        // If all were in cache, finish
        if paths_to_load.is_empty() {
            return Ok(());
        }
        
        // Add paths to queue for batch processing
        {
            let mut batch_queue = self.pending_batch.lock().await;
            for path in paths_to_load {
                batch_queue.path_to_id_requests.insert(path);
            }
        }
        
        // Execute batch processing immediately
        self.process_batch().await?;
        
        Ok(())
    }
    
    /// Preload a set of IDs to get their paths in batch
    pub async fn preload_ids(&self, ids: Vec<String>) -> Result<(), IdMappingError> {
        // Only proceed if there are IDs to load
        if ids.is_empty() {
            return Ok(());
        }
        
        // IDs we need to load (those not in cache)
        let mut ids_to_load = Vec::new();
        
        // Check cache first
        {
            let cache = self.id_to_path_cache.read().await;
            for id in ids {
                if !cache.contains_key(&id) {
                    ids_to_load.push(id);
                }
            }
        }
        
        // If all were in cache, finish
        if ids_to_load.is_empty() {
            return Ok(());
        }
        
        // Add IDs to queue for batch processing
        {
            let mut batch_queue = self.pending_batch.lock().await;
            for id in ids_to_load {
                batch_queue.id_to_path_requests.insert(id);
            }
        }
        
        // Execute batch processing immediately
        self.process_batch().await?;
        
        Ok(())
    }
}

#[async_trait]
impl IdMappingPort for IdMappingOptimizer {
    async fn get_or_create_id(&self, path: &StoragePath) -> Result<String, DomainError> {
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.get_id_queries += 1;
        }
        
        let path_str = path.to_string();
        
        // Check cache first
        {
            let cache = self.path_to_id_cache.read().await;
            if let Some((id, _)) = cache.get(&path_str) {
                // Update statistics
                {
                    let mut stats = self.stats.write().await;
                    stats.get_id_hits += 1;
                }
                
                return Ok(id.clone());
            }
        }
        
        // If not in cache, try adding to batch queue first
        let queued_result = self.queue_path_to_id_request(path).await?;
        if let Some(id) = queued_result {
            return Ok(id);
        }
        
        // Trigger batch processing if enough items accumulated
        self.trigger_batch_if_needed(20).await?;
        
        // Try to get from the base service
        let id = self.base_service.get_or_create_id(path).await?;
        
        // Update cache with the new ID
        {
            let mut path_cache = self.path_to_id_cache.write().await;
            let mut id_cache = self.id_to_path_cache.write().await;
            
            let now = Instant::now();
            
            // Control cache size
            if path_cache.len() >= MAX_CACHE_SIZE {
                warn!("Path-to-ID cache size reached limit ({}), clearing oldest entries", MAX_CACHE_SIZE);
                path_cache.clear();
            }
            
            if id_cache.len() >= MAX_CACHE_SIZE {
                warn!("ID-to-path cache size reached limit ({}), clearing oldest entries", MAX_CACHE_SIZE);
                id_cache.clear();
            }
            
            path_cache.insert(path_str.clone(), (id.clone(), now));
            id_cache.insert(id.clone(), (path_str, now));
        }
        
        Ok(id)
    }
    
    async fn get_path_by_id(&self, id: &str) -> Result<StoragePath, DomainError> {
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.path_by_id_queries += 1;
        }
        
        // Check first in the cache
        {
            let cache = self.id_to_path_cache.read().await;
            if let Some((path_str, _)) = cache.get(id) {
                // Update statistics
                {
                    let mut stats = self.stats.write().await;
                    stats.path_by_id_hits += 1;
                }
                
                return Ok(StoragePath::from_string(path_str));
            }
        }
        
        // Get from the base service
        let path = self.base_service.get_path_by_id(id).await?;
        
        // Update cache
        {
            let mut id_cache = self.id_to_path_cache.write().await;
            let mut path_cache = self.path_to_id_cache.write().await;
            
            let now = Instant::now();
            let path_str = path.to_string();
            
            // Control cache size
            if id_cache.len() >= MAX_CACHE_SIZE {
                warn!("ID-to-path cache size reached limit ({}), clearing oldest entries", MAX_CACHE_SIZE);
                id_cache.clear();
            }
            
            if path_cache.len() >= MAX_CACHE_SIZE {
                warn!("Path-to-ID cache size reached limit ({}), clearing oldest entries", MAX_CACHE_SIZE);
                path_cache.clear();
            }
            
            id_cache.insert(id.to_string(), (path_str.clone(), now));
            path_cache.insert(path_str, (id.to_string(), now));
        }
        
        Ok(path)
    }
    
    async fn update_path(&self, id: &str, new_path: &StoragePath) -> Result<(), DomainError> {
        // Invalidate cache for this ID
        {
            let mut id_cache = self.id_to_path_cache.write().await;
            let mut path_cache = self.path_to_id_cache.write().await;
            
            // Remove the ID entry
            if let Some((old_path, _)) = id_cache.remove(id) {
                path_cache.remove(&old_path);
            }
        }
        
        // Update in the base service
        let result = self.base_service.update_path(id, new_path).await?;
        
        // Update cache with new mapping
        {
            let mut id_cache = self.id_to_path_cache.write().await;
            let mut path_cache = self.path_to_id_cache.write().await;
            
            let now = Instant::now();
            let path_str = new_path.to_string();
            
            id_cache.insert(id.to_string(), (path_str.clone(), now));
            path_cache.insert(path_str, (id.to_string(), now));
        }
        
        Ok(result)
    }
    
    async fn remove_id(&self, id: &str) -> Result<(), DomainError> {
        // Invalidate cache for this ID
        {
            let mut id_cache = self.id_to_path_cache.write().await;
            let mut path_cache = self.path_to_id_cache.write().await;
            
            // Remove the ID entry
            if let Some((path, _)) = id_cache.remove(id) {
                path_cache.remove(&path);
            }
        }
        
        // Remove from the base service
        self.base_service.remove_id(id).await?;
        
        Ok(())
    }
    
    async fn save_changes(&self) -> Result<(), DomainError> {
        // Delegate to the base service
        self.base_service.save_changes().await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    async fn create_test_service() -> (Arc<IdMappingService>, Arc<IdMappingOptimizer>) {
        let temp_dir = tempdir().unwrap();
        let map_path = temp_dir.path().join("id_map.json");
        
        let base_service = Arc::new(IdMappingService::new(map_path).await.unwrap());
        let optimizer = Arc::new(IdMappingOptimizer::new(base_service.clone()));
        
        (base_service, optimizer)
    }
    
    #[tokio::test]
    async fn test_basic_caching() {
        let (_, optimizer) = create_test_service().await;
        
        let path = StoragePath::from_string("/test/file.txt");
        
        // First call should use the base service
        let id = optimizer.get_or_create_id(&path).await.unwrap();
        assert!(!id.is_empty(), "ID should not be empty");
        
        // Second call should use cache
        let id2 = optimizer.get_or_create_id(&path).await.unwrap();
        assert_eq!(id, id2, "Same path should return same ID");
        
        // Verify cache statistics
        let stats = optimizer.get_stats().await;
        assert_eq!(stats.get_id_queries, 2, "Should have 2 queries");
        assert_eq!(stats.get_id_hits, 1, "Should have 1 hit");
    }
    
    #[tokio::test]
    async fn test_batch_processing() {
        let (_, optimizer) = create_test_service().await;
        
        // Create a batch of paths
        let mut paths = Vec::new();
        for i in 0..50 {
            paths.push(StoragePath::from_string(&format!("/test/batch/file{}.txt", i)));
        }
        
        // Preload the paths
        optimizer.preload_paths(paths.clone()).await.unwrap();
        
        // Verify all are in cache
        for path in &paths {
            let id = optimizer.get_or_create_id(path).await.unwrap();
            assert!(!id.is_empty(), "ID should be available for path");
        }
        
        // Verify statistics
        let stats = optimizer.get_stats().await;
        assert_eq!(stats.batch_operations, 1, "Should have 1 batch operation");
        assert!(stats.batch_items_processed >= 50, "Should have processed at least 50 items");
        
        // Verify all subsequent queries are cache hits
        assert_eq!(stats.get_id_hits, 50, "All subsequente queries should be cache hits");
    }
    
    #[tokio::test]
    async fn test_cache_cleanup() {
        let (_, optimizer) = create_test_service().await;
        
        // Create some entries
        let path = StoragePath::from_string("/test/cleanup.txt");
        let id = optimizer.get_or_create_id(&path).await.unwrap();
        
        // Verify initial statistics
        {
            let stats = optimizer.get_stats().await;
            assert_eq!(stats.get_id_queries, 1, "Should have 1 query");
            assert_eq!(stats.get_id_hits, 0, "Should have 0 hits");
        }
        
        // Run cleanup (should not remove anything yet)
        optimizer.cleanup_cache().await;
        
        // Verify cache is still working
        let id2 = optimizer.get_or_create_id(&path).await.unwrap();
        assert_eq!(id, id2, "Cache should still work after cleanup");
        
        {
            let stats = optimizer.get_stats().await;
            assert_eq!(stats.get_id_hits, 1, "Should have 1 hit after cleanup");
        }
    }
}