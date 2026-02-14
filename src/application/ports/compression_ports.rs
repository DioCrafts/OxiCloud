//! Compression Port - Application layer abstraction for compression services.
//!
//! This module defines the port (trait) for compression operations,
//! keeping the application and interface layers independent of specific
//! compression implementations (gzip, zstd, etc.).

use crate::common::errors::DomainError;
use async_trait::async_trait;

/// Compression level settings for file compression operations.
///
/// These levels control the trade-off between compression speed and ratio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// No compression (passthrough)
    None = 0,
    /// Fast compression with lower ratio
    Fast = 1,
    /// Balanced compression (default)
    Default = 6,
    /// Maximum compression (slower)
    Best = 9,
}

/// Port for compression/decompression operations.
///
/// Implementations of this trait provide the actual compression logic
/// (e.g., gzip, zstd) while the application layer remains agnostic
/// of the specific algorithm used.
#[async_trait]
pub trait CompressionPort: Send + Sync + 'static {
    /// Compress data in memory.
    async fn compress_data(
        &self,
        data: &[u8],
        level: CompressionLevel,
    ) -> Result<Vec<u8>, DomainError>;

    /// Decompress data in memory.
    async fn decompress_data(&self, compressed_data: &[u8]) -> Result<Vec<u8>, DomainError>;

    /// Determine if a file should be compressed based on its MIME type and size.
    fn should_compress(&self, mime_type: &str, size: u64) -> bool;
}
