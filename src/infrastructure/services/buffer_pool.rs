use std::cmp::min;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use std::time::{Duration, Instant};
use tracing::debug;

/// Default buffer size in the pool
pub const DEFAULT_BUFFER_SIZE: usize = 64 * 1024; // 64KB

/// Default maximum number of buffers in the pool
pub const DEFAULT_MAX_BUFFERS: usize = 100;

/// Default time-to-live for an inactive buffer (in seconds)
pub const DEFAULT_BUFFER_TTL: u64 = 60;

/// Buffer pooling to optimize read/write operations
pub struct BufferPool {
    /// Pool of available buffers
    pool: Mutex<VecDeque<PooledBuffer>>,
    /// Semaphore to limit the maximum number of buffers
    limit: Semaphore,
    /// Size of buffers in the pool
    buffer_size: usize,
    /// Pool statistics
    stats: Mutex<BufferPoolStats>,
    /// Time-to-live for an inactive buffer
    buffer_ttl: Duration,
}

/// Structure for tracking pool statistics
#[derive(Debug, Clone, Default)]
pub struct BufferPoolStats {
    /// Total number of get operations
    pub gets: usize,
    /// Number of pool hits (successful reuse)
    pub hits: usize,
    /// Number of misses (new buffer creation)
    pub misses: usize,
    /// Number of returns to the pool
    pub returns: usize,
    /// Number of TTL evictions
    pub evictions: usize,
    /// Maximum number of buffers reached
    pub max_buffers_reached: usize,
    /// Semaphore waits
    pub waits: usize,
}

/// Pool buffer with management metadata
struct PooledBuffer {
    /// Actual byte buffer
    buffer: Vec<u8>,
    /// Timestamp of when it was added/returned to the pool
    last_used: Instant,
}

/// Borrowed buffer from the pool with automatic cleanup
#[derive(Clone)]
pub struct BorrowedBuffer {
    /// Current buffer
    buffer: Vec<u8>,
    /// Actual used size of the buffer
    used_size: usize,
    /// Reference to the pool for returning
    pool: Arc<BufferPool>,
    /// Whether the buffer should be returned to the pool or not
    return_to_pool: bool,
}

impl BufferPool {
    /// Creates a new buffer pool
    pub fn new(buffer_size: usize, max_buffers: usize, buffer_ttl_secs: u64) -> Arc<Self> {
        Arc::new(Self {
            pool: Mutex::new(VecDeque::with_capacity(max_buffers)),
            limit: Semaphore::new(max_buffers),
            buffer_size,
            stats: Mutex::new(BufferPoolStats::default()),
            buffer_ttl: Duration::from_secs(buffer_ttl_secs),
        })
    }
    
    /// Creates a pool with default configuration
    pub fn default() -> Arc<Self> {
        Self::new(
            DEFAULT_BUFFER_SIZE,
            DEFAULT_MAX_BUFFERS,
            DEFAULT_BUFFER_TTL
        )
    }
    
    /// Gets a buffer from the pool or creates a new one if needed.
    /// This version takes an Arc<Self> to ensure the BorrowedBuffer keeps a proper
    /// reference to the shared pool (not a clone).
    #[allow(unused_variables)]
    pub async fn get_buffer(self: &Arc<Self>) -> BorrowedBuffer {
        // Increment get counter
        {
            let mut stats = self.stats.lock().await;
            stats.gets += 1;
        }
        
        // Concurrency control
        // Acquire a semaphore permit. If none available, wait.
        // We forget() the permit so it doesn't auto-release on drop.
        // Instead, the permit is manually released in return_buffer/Drop via add_permits(1).
        match self.limit.try_acquire() {
            Ok(permit) => permit.forget(),
            Err(_) => {
                // No permits available, waiting
                {
                    let mut stats = self.stats.lock().await;
                    stats.waits += 1;
                    stats.max_buffers_reached += 1;
                }
                
                debug!("Buffer pool: waiting for available buffer");
                let permit = self.limit.acquire().await.expect("Semaphore should not be closed");
                debug!("Buffer pool: acquired buffer after waiting");
                permit.forget();
            }
        };
        
        // Try to get an existing buffer from the pool
        let mut pool_locked = self.pool.lock().await;
        
        let pool_arc = Arc::clone(self);
        
        if let Some(mut pooled_buffer) = pool_locked.pop_front() {
            // Check if the buffer has expired
            if pooled_buffer.last_used.elapsed() > self.buffer_ttl {
                // Expired buffer, discard and create a new one
                let mut stats = self.stats.lock().await;
                stats.evictions += 1;
                stats.misses += 1;
                drop(stats);
                
                debug!("Buffer pool: evicted expired buffer");
                
                // Create new buffer (reusing the permit)
                drop(pool_locked); // Release the lock before returning
                
                BorrowedBuffer {
                    buffer: vec![0; self.buffer_size],
                    used_size: 0,
                    pool: pool_arc,
                    return_to_pool: true,
                }
            } else {
                // Valid buffer, reuse it
                let mut stats = self.stats.lock().await;
                stats.hits += 1;
                drop(stats);
                
                // Release the lock before returning
                drop(pool_locked);
                
                // Clear buffer for security
                pooled_buffer.buffer.fill(0);
                
                BorrowedBuffer {
                    buffer: pooled_buffer.buffer,
                    used_size: 0,
                    pool: pool_arc,
                    return_to_pool: true,
                }
            }
        } else {
            // No buffers available, create a new one
            let mut stats = self.stats.lock().await;
            stats.misses += 1;
            drop(stats);
            
            // Release the lock before returning
            drop(pool_locked);
            
            debug!("Buffer pool: creating new buffer");
            
            BorrowedBuffer {
                buffer: vec![0; self.buffer_size],
                used_size: 0,
                pool: pool_arc,
                return_to_pool: true,
            }
        }
    }
    
    /// Returns a buffer to the pool
    async fn return_buffer(&self, mut buffer: Vec<u8>) {
        // If the buffer is the wrong size, discard it
        if buffer.capacity() != self.buffer_size {
            debug!("Buffer pool: discarding buffer of wrong size: {} (expected {})", 
                 buffer.capacity(), self.buffer_size);
            // Release the semaphore permit even if we discard the buffer
            self.limit.add_permits(1);
            return;
        }
        
        // Resize to ensure correct capacity
        buffer.resize(self.buffer_size, 0);
        
        // Add to the pool
        let mut pool_locked = self.pool.lock().await;
        
        pool_locked.push_back(PooledBuffer {
            buffer,
            last_used: Instant::now(),
        });
        
        // Update statistics
        let mut stats = self.stats.lock().await;
        stats.returns += 1;
        
        // Release the semaphore permit so another caller can acquire a buffer
        drop(pool_locked);
        drop(stats);
        self.limit.add_permits(1);
    }
    
    /// Cleans expired buffers from the pool
    pub async fn clean_expired_buffers(&self) {
        let _now = Instant::now();
        let mut pool_locked = self.pool.lock().await;
        
        // Count expired
        let count_before = pool_locked.len();
        
        // Filter keeping only non-expired
        pool_locked.retain(|buffer| {
            buffer.last_used.elapsed() <= self.buffer_ttl
        });
        
        // Count how many were removed
        let removed = count_before - pool_locked.len();
        
        if removed > 0 {
            // Update statistics
            let mut stats = self.stats.lock().await;
            stats.evictions += removed;
            
            debug!("Buffer pool: cleaned {} expired buffers", removed);
        }
    }
    
    /// Gets current pool statistics
    pub async fn get_stats(&self) -> BufferPoolStats {
        self.stats.lock().await.clone()
    }
    
    /// Starts the periodic cleanup task
    pub fn start_cleaner(pool: Arc<Self>) {
        tokio::spawn(async move {
            let interval = Duration::from_secs(30); // Clean every 30 seconds
            
            loop {
                tokio::time::sleep(interval).await;
                pool.clean_expired_buffers().await;
                
                // Log statistics periodically
                let stats = pool.get_stats().await;
                debug!("Buffer pool stats: gets={}, hits={}, misses={}, hit_ratio={:.2}%, returns={}, \
                      evictions={}, max_reached={}, waits={}",
                     stats.gets, 
                     stats.hits, 
                     stats.misses,
                     if stats.gets > 0 { (stats.hits as f64 * 100.0) / stats.gets as f64 } else { 0.0 },
                     stats.returns,
                     stats.evictions,
                     stats.max_buffers_reached,
                     stats.waits);
            }
        });
    }
}

impl Clone for BufferPool {
    fn clone(&self) -> Self {
        Self {
            pool: Mutex::new(VecDeque::new()),
            limit: Semaphore::new(self.limit.available_permits()),
            buffer_size: self.buffer_size,
            stats: Mutex::new(BufferPoolStats::default()),
            buffer_ttl: self.buffer_ttl,
        }
    }
}

impl BorrowedBuffer {
    /// Accesses the internal buffer
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
    
    /// Gets a reference to the used data
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer[..self.used_size]
    }
    
    /// Sets how many bytes were actually used
    pub fn set_used(&mut self, size: usize) {
        self.used_size = min(size, self.buffer.len());
    }
    
    /// Converts into a Vec<u8> that includes only the used data
    pub fn into_vec(mut self) -> Vec<u8> {
        // Mark to not return to pool
        self.return_to_pool = false;
        
        // Create a new vector with only the used data
        self.buffer[..self.used_size].to_vec()
    }
    
    /// Copies data to this buffer and updates the used size
    pub fn copy_from_slice(&mut self, data: &[u8]) -> usize {
        let copy_size = min(data.len(), self.buffer.len());
        self.buffer[..copy_size].copy_from_slice(&data[..copy_size]);
        self.used_size = copy_size;
        copy_size
    }
    
    /// Prevents the buffer from being returned to the pool on destruction
    pub fn do_not_return(mut self) -> Self {
        self.return_to_pool = false;
        self
    }
    
    /// Gets the total buffer size
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }
    
    /// Gets the used buffer size
    pub fn used_size(&self) -> usize {
        self.used_size
    }
}

// When a BorrowedBuffer is dropped, it is returned to the pool
impl Drop for BorrowedBuffer {
    fn drop(&mut self) {
        if self.return_to_pool {
            // Take ownership of the buffer and create a clone of the pool
            let buffer = std::mem::take(&mut self.buffer);
            let pool = self.pool.clone();
            
            // Spawn the return so that drop doesn't block
            // return_buffer will release the semaphore permit
            tokio::spawn(async move {
                pool.return_buffer(buffer).await;
            });
        } else {
            // Buffer not returned to pool, but we still need to release the semaphore permit
            self.pool.limit.add_permits(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_buffer_pooling() {
        // Create small pool for testing
        let pool = BufferPool::new(1024, 5, 60);
        
        // Get a buffer
        let mut buffer1 = pool.get_buffer().await;
        buffer1.copy_from_slice(b"test data");
        assert_eq!(buffer1.as_slice(), b"test data");
        
        // Get another buffer
        let buffer2 = pool.get_buffer().await;
        
        // Verify stats
        let stats = pool.get_stats().await;
        assert_eq!(stats.gets, 2);
        assert_eq!(stats.hits, 0); // no hits yet
        assert_eq!(stats.misses, 2); // all are misses
        
        // Return buffer1 to pool (implicitly via drop)
        drop(buffer1);
        
        // Allow the async return to occur
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // Get another buffer (should reuse the returned one)
        let buffer3 = pool.get_buffer().await;
        
        // Verify updated stats
        let stats = pool.get_stats().await;
        assert_eq!(stats.gets, 3);
        assert_eq!(stats.hits, 1); // now there should be a hit
        assert_eq!(stats.returns, 1); // one buffer returned
        
        // Cleanup
        drop(buffer2);
        drop(buffer3);
    }
    
    #[tokio::test]
    async fn test_buffer_operations() {
        let pool = BufferPool::new(1024, 10, 60);
        
        // Get buffer
        let mut buffer = pool.get_buffer().await;
        
        // Write data
        buffer.copy_from_slice(b"Hello, world!");
        assert_eq!(buffer.used_size(), 13);
        assert_eq!(buffer.as_slice(), b"Hello, world!");
        
        // Convert to vec and verify
        let vec = buffer.into_vec(); // This prevents returning to pool
        assert_eq!(vec, b"Hello, world!");
        
        // Verify that returns are not incremented (buffer not returned)
        tokio::time::sleep(Duration::from_millis(10)).await;
        let stats = pool.get_stats().await;
        assert_eq!(stats.returns, 0);
    }
    
    #[tokio::test]
    async fn test_pool_limit() {
        // Pool with only 3 buffers
        let pool = BufferPool::new(1024, 3, 60);
        
        // Get 3 buffers (reaches the limit)
        let buffer1 = pool.get_buffer().await;
        let buffer2 = pool.get_buffer().await;
        let buffer3 = pool.get_buffer().await;
        
        // Verify stats
        let stats = pool.get_stats().await;
        assert_eq!(stats.gets, 3);
        assert_eq!(stats.waits, 0); // no waits yet
        
        // Try to get a 4th buffer in a separate task (should wait)
        let pool_clone = pool.clone();
        let handle = tokio::spawn(async move {
            let _buffer4 = pool_clone.get_buffer().await;
            true
        });
        
        // Give time for the task to try to take the buffer
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // Verify there is a wait
        let stats = pool.get_stats().await;
        assert_eq!(stats.waits, 1);
        
        // Release a buffer
        drop(buffer1);
        
        // Give time for the async return and for the waiting task to get its buffer
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // Verify the task was able to continue
        assert!(handle.await.unwrap());
        
        // Cleanup
        drop(buffer2);
        drop(buffer3);
    }
    
    #[tokio::test]
    async fn test_ttl_expiration() {
        // Pool with very short TTL for testing
        let pool = BufferPool::new(1024, 5, 1); // 1 second TTL
        
        // Get and return a buffer
        let buffer = pool.get_buffer().await;
        drop(buffer);
        
        // Allow the async return to occur
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // Verify there is a buffer in the pool
        let stats = pool.get_stats().await;
        assert_eq!(stats.returns, 1);
        
        // Wait for the TTL to expire
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        // Clean expired
        pool.clean_expired_buffers().await;
        
        // Get another buffer (should be a miss since the previous one expired)
        let _buffer2 = pool.get_buffer().await;
        
        // Verify stats
        let stats = pool.get_stats().await;
        assert_eq!(stats.evictions, 1); // one expired buffer
        assert_eq!(stats.hits, 0); // no hits (the buffer expired)
        assert_eq!(stats.misses, 2); // two misses (1st and 3rd get)
    }
}