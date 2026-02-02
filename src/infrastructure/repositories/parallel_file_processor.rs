use std::path::PathBuf;
use std::sync::Arc;
use std::io::{self, SeekFrom};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::task;
use tokio::sync::{Semaphore, Mutex};
use futures::future::join_all;
use tracing::{info, debug, error};
use bytes::{Bytes, BytesMut};

use crate::common::config::AppConfig;
use crate::domain::repositories::file_repository::FileRepositoryError;
use crate::infrastructure::services::buffer_pool::BufferPool;

/// Structure for the byte range to process
#[derive(Debug, Clone, Copy)]
pub struct ChunkRange {
    /// Chunk index
    pub index: usize,
    /// Start position in bytes
    pub start: u64,
    /// Chunk size in bytes
    pub size: usize,
}

/// Specific buffer pooling for BytesMut
pub struct BytesBufferPool {
    buffers: Mutex<Vec<BytesMut>>,
    buffer_size: usize,
    max_buffers: usize,
}

impl BytesBufferPool {
    pub fn new(buffer_size: usize, max_buffers: usize) -> Self {
        Self {
            buffers: Mutex::new(Vec::with_capacity(max_buffers)),
            buffer_size,
            max_buffers,
        }
    }
    
    /// Get a buffer from the pool or create a new one
    pub async fn get_buffer(&self) -> BytesMut {
        let mut buffers = self.buffers.lock().await;
        
        if let Some(mut buffer) = buffers.pop() {
            // Reuse existing buffer
            buffer.clear(); // Keep capacity, clear content
            buffer
        } else {
            // Create new buffer if the pool is empty
            BytesMut::with_capacity(self.buffer_size)
        }
    }
    
    /// Return a buffer to the pool for reuse
    pub async fn return_buffer(&self, mut buffer: BytesMut) {
        // Reset the buffer for reuse
        buffer.clear();
        
        let mut buffers = self.buffers.lock().await;
        
        // Only keep up to max_buffers
        if buffers.len() < self.max_buffers {
            buffers.push(buffer);
        }
        // If we already have enough buffers, this one will be discarded
    }
}

/// Parallel file processor for IO-intensive operations
pub struct ParallelFileProcessor {
    /// Application configuration
    config: AppConfig,
    /// Semaphore to limit global concurrency
    concurrency_limiter: Arc<Semaphore>,
    /// Buffer pool to optimize memory
    buffer_pool: Option<Arc<BufferPool>>,
    /// BytesMut buffer pool for zero-copy operations
    bytes_pool: Arc<BytesBufferPool>,
}

impl ParallelFileProcessor {
    /// Creates a new processor instance
    pub fn new(config: AppConfig) -> Self {
        let concurrency_limiter = Arc::new(Semaphore::new(config.concurrency.max_concurrent_io));
        
        // Create BytesMut pool for efficient operations
        let chunk_size = config.resources.chunk_size_bytes;
        let max_chunks = config.concurrency.max_parallel_chunks;
        let bytes_pool = Arc::new(BytesBufferPool::new(chunk_size, max_chunks * 2));
        
        Self {
            config,
            concurrency_limiter,
            buffer_pool: None,
            bytes_pool,
        }
    }
    
    /// Creates a new processor instance with a buffer pool
    pub fn new_with_buffer_pool(config: AppConfig, buffer_pool: Arc<BufferPool>) -> Self {
        let concurrency_limiter = Arc::new(Semaphore::new(config.concurrency.max_concurrent_io));
        
        // Create BytesMut pool for efficient operations
        let chunk_size = config.resources.chunk_size_bytes;
        let max_chunks = config.concurrency.max_parallel_chunks;
        let bytes_pool = Arc::new(BytesBufferPool::new(chunk_size, max_chunks * 2));
        
        Self {
            config,
            concurrency_limiter,
            buffer_pool: Some(buffer_pool),
            bytes_pool,
        }
    }
    
    /// Divides a file into chunks for parallel processing
    pub fn calculate_chunks(&self, file_size: u64) -> Vec<ChunkRange> {
        // Determine if the file needs parallel processing
        let needs_parallel = self.config.resources.needs_parallel_processing(
            file_size, &self.config.concurrency
        );
        
        if !needs_parallel {
            // For small files, use a single chunk
            return vec![ChunkRange { 
                index: 0,
                start: 0,
                size: file_size as usize 
            }];
        }
        
        // Calculate optimal number of chunks
        let chunk_count = self.config.resources.calculate_optimal_chunks(
            file_size, &self.config.concurrency
        );
        
        // Calculate size of each chunk
        let chunk_size = self.config.resources.calculate_chunk_size(file_size, chunk_count);
        
        // Create chunk ranges
        let mut chunks = Vec::with_capacity(chunk_count);
        
        let mut start = 0;
        for i in 0..chunk_count {
            let current_chunk_size = if i == chunk_count - 1 {
                // Last chunk might be smaller
                (file_size - start) as usize
            } else {
                chunk_size
            };
            
            chunks.push(ChunkRange {
                index: i,
                start,
                size: current_chunk_size,
            });
            
            start += current_chunk_size as u64;
        }
        
        debug!("File size: {} bytes, divided into {} chunks of ~{} bytes each", 
              file_size, chunks.len(), chunk_size);
        
        chunks
    }
    
    /// Reads a file in parallel and returns the complete content
    /// Optimized implementation using BytesMut to reduce memory copies
    pub async fn read_file_parallel(&self, file_path: &PathBuf) -> Result<Vec<u8>, FileRepositoryError> {
        // Get file size
        let metadata = tokio::fs::metadata(file_path).await
            .map_err(FileRepositoryError::IoError)?;
        
        let file_size = metadata.len();
        
        // Check if the file is too large for memory
        if !self.config.resources.can_load_in_memory(file_size) {
            return Err(FileRepositoryError::Other(
                format!("File too large to load in memory: {} MB (max: {} MB)", 
                       file_size / (1024 * 1024), 
                       self.config.resources.max_in_memory_file_size_mb)
            ));
        }
        
        // Calculate chunks
        let chunks = self.calculate_chunks(file_size);
        
        if chunks.len() == 1 {
            // For a single chunk, use simple reading with buffer pool if available
            info!("Reading file with size {}MB as a single chunk", file_size / (1024 * 1024));
            
            if let Some(pool) = &self.buffer_pool {
                // Use buffer from the pool for efficient reading
                debug!("Using buffer pool for single chunk read");
                let mut buffer = pool.get_buffer().await;
                
                // If the buffer is too small, revert to standard implementation
                if buffer.capacity() < file_size as usize {
                    debug!("Buffer from pool too small ({}), using standard read", buffer.capacity());
                    let content = tokio::fs::read(file_path).await
                        .map_err(FileRepositoryError::IoError)?;
                    
                    return Ok(content);
                }
                
                // Use memory buffer from the pool
                let mut file = File::open(file_path).await
                    .map_err(FileRepositoryError::IoError)?;
                
                let read_size = file.read(buffer.as_mut_slice()).await
                    .map_err(FileRepositoryError::IoError)?;
                
                buffer.set_used(read_size);
                
                // Convert to Vec<u8>
                let content = buffer.into_vec();
                return Ok(content);
            } else {
                // Standard implementation without pool
                let content = tokio::fs::read(file_path).await
                    .map_err(FileRepositoryError::IoError)?;
                
                return Ok(content);
            }
        }
        
        // For multiple chunks, use parallel reading
        info!("Reading file with size {}MB in {} parallel chunks using BytesMut", 
             file_size / (1024 * 1024), chunks.len());
        
        // Create final result buffer (pre-allocated)
        let mut result = BytesMut::with_capacity(file_size as usize);
        result.resize(file_size as usize, 0);
        let result_mutex = Arc::new(Mutex::new(result));
        
        // Create tasks for each chunk
        let mut tasks = Vec::with_capacity(chunks.len());
        
        // Open file once and share it
        let file = Arc::new(File::open(file_path).await
            .map_err(FileRepositoryError::IoError)?);
        
        // Reference to BytesMut pool
        let bytes_pool = self.bytes_pool.clone();
        
        // Process chunks in parallel
        for chunk in chunks {
            let file_clone = file.clone();
            let result_clone = result_mutex.clone();
            let semaphore_clone = self.concurrency_limiter.clone();
            let bytes_pool_clone = bytes_pool.clone();
            
            // Spawn task for this chunk - no need to copy the original data
            let task = task::spawn(async move {
                // Acquire semaphore permit
                let _permit = semaphore_clone.acquire().await.unwrap();
                
                // Get a reusable buffer from the BytesMut pool
                let mut chunk_buffer = bytes_pool_clone.get_buffer().await;
                
                // Ensure it has sufficient capacity
                if chunk_buffer.capacity() < chunk.size {
                    chunk_buffer = BytesMut::with_capacity(chunk.size);
                }
                // Resize to the exact size needed
                chunk_buffer.resize(chunk.size, 0);
                
                // Create a duplicate file descriptor for independent use
                let mut file_handle = file_clone.try_clone().await?;
                
                // Position and read directly into the BytesMut
                file_handle.seek(SeekFrom::Start(chunk.start)).await?;
                let bytes_read = file_handle.read_exact(&mut chunk_buffer[..chunk.size]).await?;
                
                if bytes_read != chunk.size {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        format!("Expected to read {} bytes but got {}", chunk.size, bytes_read)
                    ));
                }
                
                // Write to final result
                let mut result_lock = result_clone.lock().await;
                let start_pos = chunk.start as usize;
                let end_pos = start_pos + chunk.size;
                
                // Use copy_from_slice to copy from BytesMut to result buffer
                result_lock[start_pos..end_pos].copy_from_slice(&chunk_buffer[..chunk.size]);
                
                // Return the buffer to the pool for reuse
                bytes_pool_clone.return_buffer(chunk_buffer).await;
                
                // Log progress
                debug!("Chunk {} processed: {} bytes from offset {}", 
                      chunk.index, chunk.size, chunk.start);
                
                Ok::<_, io::Error>(())
            });
            
            tasks.push(task);
        }
        
        // Wait for all tasks to complete
        let results = join_all(tasks).await;
        
        // Check for errors
        for (i, task_result) in results.into_iter().enumerate() {
            match task_result {
                Ok(Ok(())) => {},
                Ok(Err(e)) => {
                    error!("Error in chunk {}: {}", i, e);
                    return Err(FileRepositoryError::IoError(e));
                },
                Err(e) => {
                    error!("Task error in chunk {}: {}", i, e);
                    return Err(FileRepositoryError::Other(format!("Task error: {}", e)));
                }
            }
        }
        
        // Get the final result and convert to Vec<u8>
        let result_buffer = result_mutex.lock().await;
        let result_vec = result_buffer.to_vec();
        
        info!("Successfully read file of {}MB in parallel with optimized BytesMut", file_size / (1024 * 1024));
        Ok(result_vec)
    }
    
    /// Writes a file in parallel from a buffer
    /// Optimized implementation using BytesMut/Bytes to reduce memory copies
    pub async fn write_file_parallel(
        &self, 
        file_path: &PathBuf, 
        content: &[u8]
    ) -> Result<(), FileRepositoryError> {
        let file_size = content.len() as u64;
        
        // Calculate chunks
        let chunks = self.calculate_chunks(file_size);
        
        if chunks.len() == 1 {
            // For a single chunk, use simple writing
            info!("Writing file with size {}MB as a single chunk", file_size / (1024 * 1024));
            
            // Standard implementation (buffer pooling offers no advantages for simple writing)
            tokio::fs::write(file_path, content).await
                .map_err(FileRepositoryError::IoError)?;
            
            return Ok(());
        }
        
        // For multiple chunks, use parallel writing
        info!("Writing file with size {}MB in {} parallel chunks using Bytes", 
             file_size / (1024 * 1024), chunks.len());
        
        // Create file (we don't use Mutex to reduce contention)
        let file = File::create(file_path).await
            .map_err(FileRepositoryError::IoError)?;
        
        // Convert content to Bytes (single copy step)
        let content_bytes = Bytes::copy_from_slice(content);
        
        // Create tasks for each chunk
        let mut tasks = Vec::with_capacity(chunks.len());
        
        // Process chunks in parallel
        for chunk in chunks {
            let file_clone = file.try_clone().await
                .map_err(FileRepositoryError::IoError)?;
            let semaphore_clone = self.concurrency_limiter.clone();
            
            // Create Bytes slice (doesn't copy data, only references)
            let start_idx = chunk.start as usize;
            let end_idx = start_idx + chunk.size;
            let chunk_data = content_bytes.slice(start_idx..end_idx);
            
            // Create and launch task
            let task = task::spawn(async move {
                // Acquire semaphore permit
                let _permit = semaphore_clone.acquire().await.unwrap();
                
                // Position and write
                let mut file_handle = file_clone;
                file_handle.seek(SeekFrom::Start(chunk.start)).await?;
                file_handle.write_all(&chunk_data).await?;
                
                // Log progress
                debug!("Chunk {} written: {} bytes at offset {}", 
                      chunk.index, chunk.size, chunk.start);
                
                Ok::<_, io::Error>(())
            });
            
            tasks.push(task);
        }
        
        // Wait for all tasks to complete
        let results = join_all(tasks).await;
        
        // Check for errors
        for (i, task_result) in results.into_iter().enumerate() {
            match task_result {
                Ok(Ok(())) => {},
                Ok(Err(e)) => {
                    error!("Error in chunk {}: {}", i, e);
                    return Err(FileRepositoryError::IoError(e));
                },
                Err(e) => {
                    error!("Task error in chunk {}: {}", i, e);
                    return Err(FileRepositoryError::Other(format!("Task error: {}", e)));
                }
            }
        }
        
        // Ensure everything has been written correctly
        let mut file_handle = file;
        file_handle.flush().await.map_err(FileRepositoryError::IoError)?;
        
        info!("Successfully wrote file of {}MB in parallel with optimized Bytes", file_size / (1024 * 1024));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_parallel_read_write() {
        // Create configuration with low threshold for testing
        let mut config = AppConfig::default();
        config.concurrency.min_size_for_parallel_chunks_mb = 1; // 1MB for testing
        config.concurrency.max_parallel_chunks = 4;
        
        let processor = ParallelFileProcessor::new(config);
        
        // Create temporary directory
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.bin");
        
        // Create test data (2MB)
        let size = 2 * 1024 * 1024;
        let mut test_data = Vec::with_capacity(size);
        for i in 0..size {
            test_data.push((i % 256) as u8);
        }
        
        // Write file in parallel
        processor.write_file_parallel(&file_path, &test_data).await.unwrap();
        
        // Read file in parallel
        let read_data = processor.read_file_parallel(&file_path).await.unwrap();
        
        // Verify that the data is identical
        assert_eq!(test_data.len(), read_data.len());
        assert_eq!(test_data, read_data);
    }
    
    #[tokio::test]
    async fn test_bytesmut_pool() {
        // Create pool
        let pool = BytesBufferPool::new(1024, 5);
        
        // Get buffer
        let mut buffer1 = pool.get_buffer().await;
        buffer1.put_slice(b"test data");
        assert_eq!(&buffer1[..9], b"test data");
        
        // Return buffer to the pool
        pool.return_buffer(buffer1).await;
        
        // Get another buffer (should be the same one)
        let buffer2 = pool.get_buffer().await;
        assert_eq!(buffer2.capacity(), 1024);
        
        // The buffer should be empty (cleared)
        assert_eq!(buffer2.len(), 0);
    }
    
    #[test]
    fn test_chunk_calculation() {
        // Create test configuration
        let mut config = AppConfig::default();
        config.concurrency.min_size_for_parallel_chunks_mb = 100; // 100MB
        config.concurrency.max_parallel_chunks = 4;
        config.concurrency.parallel_chunk_size_bytes = 50 * 1024 * 1024; // 50MB
        
        let processor = ParallelFileProcessor::new(config);
        
        // Small file (10MB)
        let small_file_size = 10 * 1024 * 1024;
        let chunks = processor.calculate_chunks(small_file_size);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].size as u64, small_file_size);
        
        // Large file (300MB)
        let large_file_size = 300 * 1024 * 1024;
        let chunks = processor.calculate_chunks(large_file_size);
        assert_eq!(chunks.len(), 4); // Limited to max_parallel_chunks
        
        // Verify that all chunks add up to the total size
        let total_size: u64 = chunks.iter().map(|c| c.size as u64).sum();
        assert_eq!(total_size, large_file_size);
    }
}