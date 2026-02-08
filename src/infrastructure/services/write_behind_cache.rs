// ═══════════════════════════════════════════════════════════════════════════════
// WRITE-BEHIND CACHE - Zero-latency uploads for small files
// ═══════════════════════════════════════════════════════════════════════════════
//
// Strategy:
// 1. For files < 1MB, store in RAM and respond immediately (201 Created)
// 2. Flush to disk asynchronously in background
// 3. Serve reads from cache while pending flush
// 4. On read miss, check if pending then serve from cache
//
// This gives users perceived ~0ms upload latency for small files
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, mpsc};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use bytes::Bytes;
use async_trait::async_trait;

use crate::application::ports::cache_ports::{WriteBehindCachePort, WriteBehindStatsDto};
use crate::domain::errors::DomainError;

/// Maximum size for write-behind cache (files larger bypass cache)
const WRITE_BEHIND_MAX_SIZE: usize = 1024 * 1024; // 1MB

/// Maximum total cache size in bytes
const MAX_CACHE_SIZE: usize = 100 * 1024 * 1024; // 100MB total

/// Maximum time a file can stay pending before forced flush
const MAX_PENDING_DURATION: Duration = Duration::from_secs(30);

/// Flush check interval
const FLUSH_INTERVAL: Duration = Duration::from_millis(100);

/// Entry in the write-behind cache
#[derive(Clone)]
pub struct PendingWrite {
    /// File content
    pub content: Bytes,
    /// Target path on disk
    pub target_path: PathBuf,
    /// When this entry was created
    pub created_at: Instant,
    /// File ID for tracking
    pub file_id: String,
}

/// Statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct WriteBehindStats {
    pub pending_count: usize,
    pub pending_bytes: usize,
    pub total_writes: u64,
    pub total_bytes_written: u64,
    pub cache_hits: u64,
    pub avg_flush_time_us: u64,
}

/// Write-Behind Cache for zero-latency small file uploads
pub struct WriteBehindCache {
    /// Pending writes indexed by file ID
    pending: Arc<RwLock<HashMap<String, PendingWrite>>>,
    /// Current total size of pending data
    current_size: Arc<RwLock<usize>>,
    /// Channel to signal flush worker
    flush_tx: mpsc::Sender<FlushCommand>,
    /// Statistics
    stats: Arc<RwLock<WriteBehindStats>>,
}

/// Commands for the flush worker
enum FlushCommand {
    /// Flush a specific file
    FlushFile(String),
    /// Flush all pending files
    FlushAll,
    /// Shutdown the worker
    Shutdown,
}

impl WriteBehindCache {
    /// Create a new write-behind cache with background flush worker
    pub fn new() -> Arc<Self> {
        let (flush_tx, flush_rx) = mpsc::channel(1000);
        
        let cache = Arc::new(Self {
            pending: Arc::new(RwLock::new(HashMap::new())),
            current_size: Arc::new(RwLock::new(0)),
            flush_tx,
            stats: Arc::new(RwLock::new(WriteBehindStats::default())),
        });
        
        // Start the background flush worker
        let cache_clone = cache.clone();
        tokio::spawn(async move {
            cache_clone.flush_worker(flush_rx).await;
        });
        
        // Start the periodic flush checker
        let cache_clone2 = cache.clone();
        tokio::spawn(async move {
            cache_clone2.periodic_flush_checker().await;
        });
        
        tracing::info!("⚡ Write-Behind Cache initialized (max {}MB)", MAX_CACHE_SIZE / (1024 * 1024));
        
        cache
    }
    
    /// Check if a file size is eligible for write-behind caching
    #[inline]
    pub fn is_eligible(size: usize) -> bool {
        size <= WRITE_BEHIND_MAX_SIZE
    }
    
    /// Put a file in the pending write cache
    /// Returns Ok(true) if cached, Ok(false) if cache is full
    pub async fn put_pending(
        &self,
        file_id: String,
        content: Bytes,
        target_path: PathBuf,
    ) -> Result<bool, std::io::Error> {
        let content_size = content.len();
        
        // Check if we have space
        {
            let current = *self.current_size.read().await;
            if current + content_size > MAX_CACHE_SIZE {
                tracing::debug!(
                    "Write-behind cache full ({}/{}MB), bypassing for {}", 
                    current / (1024 * 1024),
                    MAX_CACHE_SIZE / (1024 * 1024),
                    file_id
                );
                return Ok(false);
            }
        }
        
        // Add to pending
        let entry = PendingWrite {
            content,
            target_path,
            created_at: Instant::now(),
            file_id: file_id.clone(),
        };
        
        {
            let mut pending = self.pending.write().await;
            let mut size = self.current_size.write().await;
            
            // If replacing existing entry, adjust size
            if let Some(old) = pending.insert(file_id.clone(), entry) {
                *size -= old.content.len();
            }
            *size += content_size;
        }
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.pending_count += 1;
            stats.pending_bytes += content_size;
        }
        
        // Signal flush worker (non-blocking)
        let _ = self.flush_tx.try_send(FlushCommand::FlushFile(file_id.clone()));
        
        tracing::debug!("⚡ Cached pending write: {} ({} bytes)", file_id, content_size);
        
        Ok(true)
    }
    
    /// Get content from cache if pending (for reads before flush completes)
    pub async fn get_pending(&self, file_id: &str) -> Option<Bytes> {
        let pending = self.pending.read().await;
        if let Some(entry) = pending.get(file_id) {
            // Update cache hit stats
            let mut stats = self.stats.write().await;
            stats.cache_hits += 1;
            
            tracing::debug!("⚡ Cache hit for pending file: {}", file_id);
            return Some(entry.content.clone());
        }
        None
    }
    
    /// Check if a file is pending flush
    pub async fn is_pending(&self, file_id: &str) -> bool {
        self.pending.read().await.contains_key(file_id)
    }
    
    /// Force immediate flush of a specific file (for critical operations)
    pub async fn force_flush(&self, file_id: &str) -> Result<(), std::io::Error> {
        let entry = {
            let pending = self.pending.read().await;
            pending.get(file_id).cloned()
        };
        
        if let Some(entry) = entry {
            self.flush_single(&entry.file_id, &entry).await?;
        }
        
        Ok(())
    }
    
    /// Flush all pending writes immediately
    pub async fn flush_all(&self) -> Result<(), std::io::Error> {
        let _ = self.flush_tx.send(FlushCommand::FlushAll).await;
        
        // Wait a bit for flush to complete
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        Ok(())
    }
    
    /// Gracefully shutdown the write-behind cache
    /// Flushes all pending writes before stopping the background worker
    pub async fn shutdown(&self) -> Result<(), std::io::Error> {
        tracing::info!("🛑 Shutting down write-behind cache...");
        
        // First flush all pending writes
        self.flush_all().await?;
        
        // Then signal the worker to stop
        let _ = self.flush_tx.send(FlushCommand::Shutdown).await;
        
        // Give worker time to process shutdown
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        tracing::info!("✅ Write-behind cache shutdown complete");
        Ok(())
    }
    
    /// Get current statistics
    pub async fn get_stats(&self) -> WriteBehindStats {
        self.stats.read().await.clone()
    }
    
    /// Background worker that handles actual disk writes
    async fn flush_worker(&self, mut rx: mpsc::Receiver<FlushCommand>) {
        tracing::info!("🔄 Write-behind flush worker started");
        
        while let Some(cmd) = rx.recv().await {
            match cmd {
                FlushCommand::FlushFile(file_id) => {
                    // Small delay to batch nearby writes
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    
                    let entry = {
                        let pending = self.pending.read().await;
                        pending.get(&file_id).cloned()
                    };
                    
                    if let Some(entry) = entry {
                        if let Err(e) = self.flush_single(&file_id, &entry).await {
                            tracing::error!("Failed to flush {}: {}", file_id, e);
                            // Keep in cache for retry
                            continue;
                        }
                    }
                }
                FlushCommand::FlushAll => {
                    let entries: Vec<_> = {
                        let pending = self.pending.read().await;
                        pending.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
                    };
                    
                    for (file_id, entry) in entries {
                        if let Err(e) = self.flush_single(&file_id, &entry).await {
                            tracing::error!("Failed to flush {}: {}", file_id, e);
                        }
                    }
                }
                FlushCommand::Shutdown => {
                    tracing::info!("Write-behind flush worker shutting down");
                    break;
                }
            }
        }
    }
    
    /// Flush a single file to disk
    async fn flush_single(&self, file_id: &str, entry: &PendingWrite) -> Result<(), std::io::Error> {
        let start = Instant::now();
        
        // Ensure parent directory exists
        if let Some(parent) = entry.target_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        // Write atomically using temp file + rename
        let temp_path = entry.target_path.with_extension("tmp");
        
        {
            let mut file = fs::File::create(&temp_path).await?;
            file.write_all(&entry.content).await?;
            file.sync_all().await?;
        }
        
        fs::rename(&temp_path, &entry.target_path).await?;
        
        let elapsed = start.elapsed();
        let content_len = entry.content.len();
        
        // Remove from pending
        {
            let mut pending = self.pending.write().await;
            let mut size = self.current_size.write().await;
            
            if pending.remove(file_id).is_some() {
                *size = size.saturating_sub(content_len);
            }
        }
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.pending_count = stats.pending_count.saturating_sub(1);
            stats.pending_bytes = stats.pending_bytes.saturating_sub(content_len);
            stats.total_writes += 1;
            stats.total_bytes_written += content_len as u64;
            
            // Running average of flush time
            let flush_us = elapsed.as_micros() as u64;
            if stats.avg_flush_time_us == 0 {
                stats.avg_flush_time_us = flush_us;
            } else {
                stats.avg_flush_time_us = (stats.avg_flush_time_us * 9 + flush_us) / 10;
            }
        }
        
        tracing::debug!(
            "💾 Flushed {} to disk ({} bytes in {:?})",
            file_id,
            content_len,
            elapsed
        );
        
        Ok(())
    }
    
    /// Periodic checker for stale pending writes
    async fn periodic_flush_checker(&self) {
        let mut interval = tokio::time::interval(FLUSH_INTERVAL);
        
        loop {
            interval.tick().await;
            
            let stale_files: Vec<String> = {
                let pending = self.pending.read().await;
                pending
                    .iter()
                    .filter(|(_, entry)| entry.created_at.elapsed() > MAX_PENDING_DURATION)
                    .map(|(id, _)| id.clone())
                    .collect()
            };
            
            for file_id in stale_files {
                tracing::warn!("Forcing flush of stale pending file: {}", file_id);
                let _ = self.flush_tx.try_send(FlushCommand::FlushFile(file_id));
            }
        }
    }
}

// ─── Port implementation ─────────────────────────────────────────────────────

#[async_trait]
impl WriteBehindCachePort for WriteBehindCache {
    fn is_eligible_size(&self, size: usize) -> bool {
        WriteBehindCache::is_eligible(size)
    }

    async fn put_pending(
        &self,
        file_id: String,
        content: Bytes,
        target_path: PathBuf,
    ) -> Result<bool, DomainError> {
        self.put_pending(file_id, content, target_path).await.map_err(DomainError::from)
    }

    async fn get_pending(&self, file_id: &str) -> Option<Bytes> {
        self.get_pending(file_id).await
    }

    async fn is_pending(&self, file_id: &str) -> bool {
        self.is_pending(file_id).await
    }

    async fn force_flush(&self, file_id: &str) -> Result<(), DomainError> {
        self.force_flush(file_id).await.map_err(DomainError::from)
    }

    async fn flush_all(&self) -> Result<(), DomainError> {
        self.flush_all().await.map_err(DomainError::from)
    }

    async fn shutdown(&self) -> Result<(), DomainError> {
        self.shutdown().await.map_err(DomainError::from)
    }

    async fn get_stats(&self) -> WriteBehindStatsDto {
        let stats = self.get_stats().await;
        WriteBehindStatsDto {
            pending_count: stats.pending_count,
            pending_bytes: stats.pending_bytes,
            total_writes: stats.total_writes,
            total_bytes_written: stats.total_bytes_written,
            cache_hits: stats.cache_hits,
            avg_flush_time_us: stats.avg_flush_time_us,
        }
    }
}

impl Default for WriteBehindCache {
    fn default() -> Self {
        // Note: This creates a non-Arc version, prefer using new()
        let (flush_tx, _) = mpsc::channel(1);
        Self {
            pending: Arc::new(RwLock::new(HashMap::new())),
            current_size: Arc::new(RwLock::new(0)),
            flush_tx,
            stats: Arc::new(RwLock::new(WriteBehindStats::default())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_write_behind_basic() {
        let cache = WriteBehindCache::new();
        let temp_dir = TempDir::new().unwrap();
        let target = temp_dir.path().join("test.txt");
        
        let content = Bytes::from("Hello, World!");
        
        // Put in cache
        let cached = cache.put_pending(
            "test-id".to_string(),
            content.clone(),
            target.clone(),
        ).await.unwrap();
        
        assert!(cached);
        assert!(cache.is_pending("test-id").await);
        
        // Should be readable from cache
        let cached_content = cache.get_pending("test-id").await.unwrap();
        assert_eq!(cached_content, content);
        
        // Force flush
        cache.force_flush("test-id").await.unwrap();
        
        // Should no longer be pending
        assert!(!cache.is_pending("test-id").await);
        
        // File should exist on disk
        assert!(target.exists());
        let disk_content = std::fs::read(&target).unwrap();
        assert_eq!(disk_content, content.as_ref());
    }
    
    #[tokio::test]
    async fn test_eligibility() {
        // 500KB should be eligible
        assert!(WriteBehindCache::is_eligible(500 * 1024));
        
        // 1MB exactly should be eligible
        assert!(WriteBehindCache::is_eligible(1024 * 1024));
        
        // Over 1MB should not be eligible
        assert!(!WriteBehindCache::is_eligible(1024 * 1024 + 1));
    }
}
