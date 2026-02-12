use std::io::{Read};
use std::sync::Arc;
use async_trait::async_trait;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use tracing::error;
use std::io;
use flate2::Compression;
use flate2::read::GzEncoder as GzEncoderRead;
use flate2::bufread::GzDecoder;

use crate::application::ports::compression_ports::{
    CompressionPort,
    CompressionLevel as PortCompressionLevel,
};
use crate::domain::errors::DomainError;
use crate::infrastructure::services::buffer_pool::BufferPool;

/// Compression level for files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// No compression (transfer only)
    None = 0,
    /// Fast compression with lower ratio
    Fast = 1,
    /// Balanced compression (default)
    Default = 6,
    /// Maximum compression (slower)
    Best = 9,
}

impl From<CompressionLevel> for Compression {
    fn from(level: CompressionLevel) -> Self {
        match level {
            CompressionLevel::None => Compression::none(),
            CompressionLevel::Fast => Compression::fast(),
            CompressionLevel::Default => Compression::default(),
            CompressionLevel::Best => Compression::best(),
        }
    }
}

/// Size threshold to decide whether to compress or not
const COMPRESSION_SIZE_THRESHOLD: u64 = 1024 * 50; // 50KB

/// Interface for compression services
#[async_trait]
pub trait CompressionService: Send + Sync {
    /// Compresses data in memory
    async fn compress_data(&self, data: &[u8], level: CompressionLevel) -> io::Result<Vec<u8>>;
    
    /// Decompresses data in memory
    async fn decompress_data(&self, compressed_data: &[u8]) -> io::Result<Vec<u8>>;
    
    /// Compresses a data stream
    fn compress_stream<S>(&self, stream: S, level: CompressionLevel) 
        -> impl Stream<Item = io::Result<Bytes>> + Send
    where
        S: Stream<Item = io::Result<Bytes>> + Send + 'static + Unpin;
    
    /// Decompresses a data stream
    fn decompress_stream<S>(&self, compressed_stream: S) 
        -> impl Stream<Item = io::Result<Bytes>> + Send
    where
        S: Stream<Item = io::Result<Bytes>> + Send + 'static + Unpin;
    
    /// Determines whether a file should be compressed based on its MIME type and size
    fn should_compress(&self, mime_type: &str, size: u64) -> bool;
}

/// Gzip compression service implementation
pub struct GzipCompressionService {
    /// Buffer pool for memory optimization
    buffer_pool: Option<Arc<BufferPool>>,
}

impl GzipCompressionService {
    /// Creates a new service instance
    pub fn new() -> Self {
        Self {
            buffer_pool: None,
        }
    }
    
    /// Creates a new service instance with buffer pool
    pub fn new_with_buffer_pool(buffer_pool: Arc<BufferPool>) -> Self {
        Self {
            buffer_pool: Some(buffer_pool),
        }
    }
}

#[async_trait]
impl CompressionService for GzipCompressionService {
    /// Compresses data in memory using Gzip
    async fn compress_data(&self, data: &[u8], level: CompressionLevel) -> io::Result<Vec<u8>> {
        // If we have a buffer pool, use a borrowed buffer for compression
        if let Some(pool) = &self.buffer_pool {
            // Estimate the compression size (approximately 80% of original for typical cases)
            let estimated_size = (data.len() as f64 * 0.8) as usize;
            
            // Get a buffer from the pool
            let buffer = pool.get_buffer().await;
            
            // Check if the buffer is large enough
            if buffer.capacity() >= estimated_size {
                // Run compression in a worker thread using the buffer
                let buffer_ptr = Arc::new(tokio::sync::Mutex::new(buffer));
                let buffer_clone = buffer_ptr.clone();
                
                // Compress data
                // Clone the data to avoid lifetime issues
                let data_owned = data.to_vec();
                
                let result = tokio::task::spawn_blocking(move || {
                    let mut encoder = GzEncoderRead::new(&data_owned[..], level.into());
                    
                    // Try to lock the mutex (should not fail since we are in a separate thread)
                    let mut buffer_guard = match futures::executor::block_on(buffer_clone.lock()) {
                        buffer => buffer,
                    };
                    
                    // Read directly into the buffer
                    let read_bytes = encoder.read(buffer_guard.as_mut_slice())?;
                    buffer_guard.set_used(read_bytes);
                    
                    Ok(()) as io::Result<()>
                }).await;
                
                // Verify result
                match result {
                    Ok(Ok(())) => {
                        // Get the buffer and convert it to Vec<u8>
                        let buffer = buffer_ptr.lock().await;
                        let cloned_buffer = buffer.clone();
                        drop(buffer); // Release the mutex first
                        return Ok(cloned_buffer.into_vec());
                    },
                    Ok(Err(e)) => {
                        error!("Compression error with buffer pool: {}", e);
                        // Fall back to standard implementation
                    },
                    Err(e) => {
                        error!("Compression task error with buffer pool: {}", e);
                        // Fall back to standard implementation
                    }
                }
            }
        }
        
        // Standard implementation if there is no buffer pool or the buffer is insufficient
        // Clone the data to avoid lifetime issues
        let data_owned = data.to_vec();
        
        tokio::task::spawn_blocking(move || {
            let mut encoder = GzEncoderRead::new(&data_owned[..], level.into());
            let mut compressed = Vec::new();
            encoder.read_to_end(&mut compressed)?;
            Ok(compressed)
        }).await.unwrap_or_else(|e| {
            error!("Compression task error: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        })
    }
    
    /// Decompresses data in memory
    async fn decompress_data(&self, compressed_data: &[u8]) -> io::Result<Vec<u8>> {
        // If we have a buffer pool, use a borrowed buffer for decompression
        if let Some(pool) = &self.buffer_pool {
            // Estimate the decompression size (approximately 5x of compressed for typical cases)
            let estimated_size = compressed_data.len() * 5;
            
            // Get a buffer from the pool
            let buffer = pool.get_buffer().await;
            
            // Check if the buffer is large enough
            if buffer.capacity() >= estimated_size {
                // Clone compressed data to move to the worker
                let data = compressed_data.to_vec();
                let buffer_ptr = Arc::new(tokio::sync::Mutex::new(buffer));
                let buffer_clone = buffer_ptr.clone();
                
                // Decompress data
                let result = tokio::task::spawn_blocking(move || {
                    let mut decoder = GzDecoder::new(&data[..]);
                    
                    // Try to lock the mutex
                    let mut buffer_guard = match futures::executor::block_on(buffer_clone.lock()) {
                        buffer => buffer,
                    };
                    
                    // Read directly into the buffer
                    let read_bytes = decoder.read(buffer_guard.as_mut_slice())?;
                    buffer_guard.set_used(read_bytes);
                    
                    Ok(()) as io::Result<()>
                }).await;
                
                // Verify result
                match result {
                    Ok(Ok(())) => {
                        // Get the buffer and convert it to Vec<u8>
                        let buffer = buffer_ptr.lock().await;
                        let cloned_buffer = buffer.clone();
                        drop(buffer); // Release the mutex first
                        return Ok(cloned_buffer.into_vec());
                    },
                    Ok(Err(e)) => {
                        error!("Decompression error with buffer pool: {}", e);
                        // Fall back to standard implementation
                    },
                    Err(e) => {
                        error!("Decompression task error with buffer pool: {}", e);
                        // Fall back to standard implementation
                    }
                }
            }
        }
        
        // Standard implementation if there is no buffer pool or the buffer is insufficient
        let data = compressed_data.to_vec(); // Clone to move to the worker
        tokio::task::spawn_blocking(move || {
            let mut decoder = GzDecoder::new(&data[..]);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)?;
            Ok(decompressed)
        }).await.unwrap_or_else(|e| {
            error!("Decompression task error: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        })
    }
    
    /// Compresses a byte stream
    fn compress_stream<S>(&self, stream: S, level: CompressionLevel) 
        -> impl Stream<Item = io::Result<Bytes>> + Send
    where
        S: Stream<Item = io::Result<Bytes>> + Send + 'static + Unpin
    {
        // For now, simplify the implementation to avoid complex pinning issues
        // This implementation collects all stream data and then compresses it at once
        // Future optimization would be to implement true streaming compression
        let compression_level = level;
        
        Box::pin(async_stream::stream! {
            let mut data = Vec::new();
            
            // Collect all bytes from the stream
            let mut stream = Box::pin(stream);
            while let Some(result) = stream.next().await {
                match result {
                    Ok(bytes) => {
                        data.extend_from_slice(&bytes);
                    },
                    Err(e) => {
                        yield Err(e);
                        return;
                    }
                }
            }
            
            // Compress collected data
            match CompressionService::compress_data(self, &data, compression_level).await {
                Ok(compressed) => {
                    // Return compressed data as a single chunk
                    yield Ok(Bytes::from(compressed));
                },
                Err(e) => {
                    yield Err(e);
                }
            }
        })
    }
    
    /// Decompresses a byte stream
    fn decompress_stream<S>(&self, compressed_stream: S) 
        -> impl Stream<Item = io::Result<Bytes>> + Send
    where
        S: Stream<Item = io::Result<Bytes>> + Send + 'static + Unpin
    {
        // For now, simplify the implementation to avoid complex pinning issues
        // This implementation collects all stream data and then decompresses it at once
        // Future optimization would be to implement streaming decompression correctly
        Box::pin(async_stream::stream! {
            let mut compressed_data = Vec::new();
            
            // Collect all bytes from the stream
            let mut stream = Box::pin(compressed_stream);
            while let Some(result) = stream.next().await {
                match result {
                    Ok(bytes) => {
                        compressed_data.extend_from_slice(&bytes);
                    },
                    Err(e) => {
                        yield Err(e);
                        return;
                    }
                }
            }
            
            // Decompress collected data
            match CompressionService::decompress_data(self, &compressed_data).await {
                Ok(decompressed) => {
                    // Return decompressed data as a single chunk
                    yield Ok(Bytes::from(decompressed));
                },
                Err(e) => {
                    yield Err(e);
                }
            }
        })
    }
    
    /// Determines whether a file should be compressed based on its MIME type and size
    fn should_compress(&self, mime_type: &str, size: u64) -> bool {
        // Do not compress very small files (overhead)
        if size < COMPRESSION_SIZE_THRESHOLD {
            return false;
        }
        
        // Do not compress already compressed files
        if mime_type.starts_with("image/")
            && !mime_type.contains("svg")
            && !mime_type.contains("bmp") {
            return false;
        }
        
        if mime_type.starts_with("audio/") 
            || mime_type.starts_with("video/") 
            || mime_type.contains("zip")
            || mime_type.contains("gzip")
            || mime_type.contains("compressed")
            || mime_type.contains("7z")
            || mime_type.contains("rar")
            || mime_type.contains("bz2")
            || mime_type.contains("xz")
            || mime_type.contains("jpg")
            || mime_type.contains("jpeg")
            || mime_type.contains("png")
            || mime_type.contains("gif")
            || mime_type.contains("webp")
            || mime_type.contains("mp3")
            || mime_type.contains("mp4")
            || mime_type.contains("ogg")
            || mime_type.contains("webm") {
            return false;
        }
        
        // Compress text files, documents, and other compressible types
        true
    }
}

// ─── Port implementation ─────────────────────────────────────────────────────

/// Convert application-layer CompressionLevel to infrastructure CompressionLevel.
impl From<PortCompressionLevel> for CompressionLevel {
    fn from(level: PortCompressionLevel) -> Self {
        match level {
            PortCompressionLevel::None => CompressionLevel::None,
            PortCompressionLevel::Fast => CompressionLevel::Fast,
            PortCompressionLevel::Default => CompressionLevel::Default,
            PortCompressionLevel::Best => CompressionLevel::Best,
        }
    }
}

#[async_trait]
impl CompressionPort for GzipCompressionService {
    async fn compress_data(&self, data: &[u8], level: PortCompressionLevel) -> Result<Vec<u8>, DomainError> {
        CompressionService::compress_data(self, data, level.into()).await.map_err(DomainError::from)
    }

    async fn decompress_data(&self, compressed_data: &[u8]) -> Result<Vec<u8>, DomainError> {
        CompressionService::decompress_data(self, compressed_data).await.map_err(DomainError::from)
    }

    fn should_compress(&self, mime_type: &str, size: u64) -> bool {
        CompressionService::should_compress(self, mime_type, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::TryStreamExt;
    
    #[tokio::test]
    async fn test_compress_decompress_data() {
        let service = GzipCompressionService::new();
        
        // Test data
        let data = "Hello, world! ".repeat(1000).into_bytes();
        
        // Compress
        let compressed = CompressionService::compress_data(&service, &data, CompressionLevel::Default).await.unwrap();
        
        // Verify that compression reduces the size
        assert!(compressed.len() < data.len());
        
        // Decompress
        let decompressed = CompressionService::decompress_data(&service, &compressed).await.unwrap();
        
        // Verify that the original data is recovered correctly
        assert_eq!(decompressed, data);
    }
    
    #[tokio::test]
    async fn test_compress_decompress_stream() {
        let service = GzipCompressionService::new();
        
        // Create test data
        let chunks = vec![
            Ok(Bytes::from("Hello, ")),
            Ok(Bytes::from("world! ")),
            Ok(Bytes::from("This is a test of streaming compression.")),
        ];
        
        // Convert to stream
        let input_stream = futures::stream::iter(chunks);
        
        // Compress the stream
        let compressed_stream = service.compress_stream(input_stream, CompressionLevel::Default);
        
        // Collect the compressed bytes
        let compressed_bytes = compressed_stream
            .try_fold(Vec::new(), |mut acc, chunk| async move {
                acc.extend_from_slice(&chunk);
                Ok(acc)
            }).await.unwrap();
        
        // Decompress the data
        let decompressed = CompressionService::decompress_data(&service, &compressed_bytes).await.unwrap();
        
        // Verify result
        let expected = "Hello, world! This is a test of streaming compression.";
        assert_eq!(String::from_utf8(decompressed).unwrap(), expected);
    }
    
    #[test]
    fn test_should_compress() {
        let service = GzipCompressionService::new();
        
        // Cases that should not be compressed
        assert!(!CompressionService::should_compress(&service, "image/jpeg", 100 * 1024));
        assert!(!CompressionService::should_compress(&service, "video/mp4", 10 * 1024 * 1024));
        assert!(!CompressionService::should_compress(&service, "application/zip", 5 * 1024 * 1024));
        
        // Cases that should be compressed
        assert!(CompressionService::should_compress(&service, "text/html", 100 * 1024));
        assert!(CompressionService::should_compress(&service, "application/json", 200 * 1024));
        assert!(CompressionService::should_compress(&service, "text/plain", 1024 * 1024));
        
        // Small files should not be compressed regardless of type
        assert!(!CompressionService::should_compress(&service, "text/html", 10 * 1024));
    }
}