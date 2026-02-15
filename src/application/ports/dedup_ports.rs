//! Deduplication Port - Application layer abstraction for content-addressable storage.
//!
//! This module defines the port (trait) and DTOs for deduplication operations,
//! keeping the application and interface layers independent of the specific
//! content-addressable storage implementation.

use crate::common::errors::DomainError;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::pin::Pin;

/// Metadata of a stored blob in the dedup system.
#[derive(Debug, Clone, Serialize)]
pub struct BlobMetadataDto {
    /// SHA-256 hash of the content.
    pub hash: String,
    /// Size in bytes.
    pub size: u64,
    /// Number of references to this blob.
    pub ref_count: u32,
    /// Original content type (for serving).
    pub content_type: Option<String>,
}

/// Result of a deduplication store operation.
#[derive(Debug, Clone)]
pub enum DedupResultDto {
    /// New content was stored (first occurrence).
    NewBlob {
        hash: String,
        size: u64,
        blob_path: PathBuf,
    },
    /// Content already existed; a reference was added instead.
    ExistingBlob {
        hash: String,
        size: u64,
        blob_path: PathBuf,
        saved_bytes: u64,
    },
}

impl DedupResultDto {
    pub fn hash(&self) -> &str {
        match self {
            DedupResultDto::NewBlob { hash, .. } => hash,
            DedupResultDto::ExistingBlob { hash, .. } => hash,
        }
    }

    pub fn size(&self) -> u64 {
        match self {
            DedupResultDto::NewBlob { size, .. } => *size,
            DedupResultDto::ExistingBlob { size, .. } => *size,
        }
    }

    pub fn blob_path(&self) -> &Path {
        match self {
            DedupResultDto::NewBlob { blob_path, .. } => blob_path,
            DedupResultDto::ExistingBlob { blob_path, .. } => blob_path,
        }
    }

    pub fn was_deduplicated(&self) -> bool {
        matches!(self, DedupResultDto::ExistingBlob { .. })
    }
}

/// Statistics for the deduplication service.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DedupStatsDto {
    /// Total number of unique blobs.
    pub total_blobs: u64,
    /// Total bytes stored (actual disk usage).
    pub total_bytes_stored: u64,
    /// Total bytes referenced (logical size).
    pub total_bytes_referenced: u64,
    /// Bytes saved through deduplication.
    pub bytes_saved: u64,
    /// Number of deduplication hits.
    pub dedup_hits: u64,
    /// Deduplication ratio (referenced / stored).
    pub dedup_ratio: f64,
}

/// Port for content-addressable deduplication operations.
///
/// Implementations store files by their content hash, eliminating
/// duplicate storage automatically. Multiple file references can
/// point to the same physical blob.
#[async_trait]
pub trait DedupPort: Send + Sync + 'static {
    /// Store content with deduplication (from bytes).
    ///
    /// If content with the same hash already exists, a reference is added
    /// instead of storing a duplicate.
    async fn store_bytes(
        &self,
        content: &[u8],
        content_type: Option<String>,
    ) -> Result<DedupResultDto, DomainError>;

    /// Store content with deduplication (streaming from file).
    ///
    /// If `pre_computed_hash` is provided (e.g. hash-on-write from the handler),
    /// the file will NOT be re-read to calculate the hash — saving one full
    /// sequential read of the file.
    async fn store_from_file(
        &self,
        source_path: &Path,
        content_type: Option<String>,
        pre_computed_hash: Option<String>,
    ) -> Result<DedupResultDto, DomainError>;

    /// Check if a blob with the given hash exists.
    async fn blob_exists(&self, hash: &str) -> bool;

    /// Get metadata for a blob.
    async fn get_blob_metadata(&self, hash: &str) -> Option<BlobMetadataDto>;

    /// Read blob content as raw bytes.
    async fn read_blob(&self, hash: &str) -> Result<Vec<u8>, DomainError>;

    /// Read blob content as `Bytes`.
    async fn read_blob_bytes(&self, hash: &str) -> Result<Bytes, DomainError>;

    /// Stream blob content in chunks (64 KB default) — constant memory usage.
    ///
    /// Unlike `read_blob()`, this never loads the entire file into RAM.
    async fn read_blob_stream(
        &self,
        hash: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>;

    /// Stream a byte range of a blob — only reads the requested portion.
    ///
    /// Uses seek + take so a 1 MB range on a 1 GB file only reads 1 MB from disk.
    async fn read_blob_range_stream(
        &self,
        hash: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>;

    /// Get the size of a blob without reading its content.
    ///
    /// Used by HEAD requests to return Content-Length without loading the file.
    async fn blob_size(&self, hash: &str) -> Result<u64, DomainError>;

    /// Add a reference to a blob (increment ref_count).
    async fn add_reference(&self, hash: &str) -> Result<(), DomainError>;

    /// Remove a reference from a blob.
    ///
    /// Returns `true` if the blob was deleted (ref_count reached 0).
    async fn remove_reference(&self, hash: &str) -> Result<bool, DomainError>;

    /// Calculate SHA-256 hash of in-memory content.
    fn hash_bytes(&self, content: &[u8]) -> String;

    /// Calculate SHA-256 hash of a file (streaming).
    async fn hash_file(&self, path: &Path) -> Result<String, DomainError>;

    /// Get deduplication statistics.
    async fn get_stats(&self) -> DedupStatsDto;

    /// Flush the index to persistent storage.
    async fn flush(&self) -> Result<(), DomainError>;

    /// Verify integrity of all stored blobs.
    ///
    /// Returns a list of issues found (empty if everything is OK).
    async fn verify_integrity(&self) -> Result<Vec<String>, DomainError>;
}
