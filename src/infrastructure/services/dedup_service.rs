//! Content-Addressable Storage with Deduplication
//!
//! Implements hash-based deduplication to eliminate redundant file storage.
//! Files are stored by their SHA-256 hash, and multiple references can point
//! to the same physical blob.
//!
//! Architecture:
//! ```text
//! ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
//! │ User Files      │────▶│ Dedup Index     │────▶│ Blob Store      │
//! │ (references)    │     │ (hash→metadata) │     │ (actual data)   │
//! └─────────────────┘     └─────────────────┘     └─────────────────┘
//! ```
//!
//! Benefits:
//! - 30-50% storage reduction typical
//! - Faster uploads for existing content (instant dedup)
//! - Efficient backups

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::sync::RwLock;
use sha2::{Sha256, Digest};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::application::ports::dedup_ports::{
    DedupPort,
    BlobMetadataDto,
    DedupResultDto,
    DedupStatsDto,
};
use crate::domain::errors::{DomainError, ErrorKind};

/// Chunk size for streaming hash calculation (256KB)
const HASH_CHUNK_SIZE: usize = 256 * 1024;

/// Minimum file size for deduplication (skip tiny files)
const MIN_DEDUP_SIZE: u64 = 4096; // 4KB

/// Blob metadata stored in the dedup index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobMetadata {
    /// SHA-256 hash of the content
    pub hash: String,
    /// Size in bytes
    pub size: u64,
    /// Number of references to this blob
    pub ref_count: u32,
    /// When the blob was first stored
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Original content type (for serving)
    pub content_type: Option<String>,
}

/// Result of a dedup operation
#[derive(Debug, Clone)]
pub enum DedupResult {
    /// New content was stored
    NewBlob {
        hash: String,
        size: u64,
        blob_path: PathBuf,
    },
    /// Content already existed, reference added
    ExistingBlob {
        hash: String,
        size: u64,
        blob_path: PathBuf,
        saved_bytes: u64,
    },
}

impl DedupResult {
    pub fn hash(&self) -> &str {
        match self {
            DedupResult::NewBlob { hash, .. } => hash,
            DedupResult::ExistingBlob { hash, .. } => hash,
        }
    }
    
    pub fn size(&self) -> u64 {
        match self {
            DedupResult::NewBlob { size, .. } => *size,
            DedupResult::ExistingBlob { size, .. } => *size,
        }
    }
    
    pub fn blob_path(&self) -> &Path {
        match self {
            DedupResult::NewBlob { blob_path, .. } => blob_path,
            DedupResult::ExistingBlob { blob_path, .. } => blob_path,
        }
    }
    
    pub fn was_deduplicated(&self) -> bool {
        matches!(self, DedupResult::ExistingBlob { .. })
    }
}

/// Statistics for the dedup service
#[derive(Debug, Clone, Default, Serialize)]
pub struct DedupStats {
    /// Total number of unique blobs
    pub total_blobs: u64,
    /// Total bytes stored (actual disk usage)
    pub total_bytes_stored: u64,
    /// Total bytes referenced (logical size)
    pub total_bytes_referenced: u64,
    /// Bytes saved through deduplication
    pub bytes_saved: u64,
    /// Number of dedup hits
    pub dedup_hits: u64,
    /// Deduplication ratio (referenced / stored)
    pub dedup_ratio: f64,
}

/// Content-Addressable Storage Service
pub struct DedupService {
    /// Root directory for blob storage
    blob_root: PathBuf,
    /// Root directory for temporary files during upload
    temp_root: PathBuf,
    /// In-memory index of blobs (hash -> metadata)
    index: Arc<RwLock<HashMap<String, BlobMetadata>>>,
    /// Path to persistent index file
    index_path: PathBuf,
    /// Statistics
    stats: Arc<RwLock<DedupStats>>,
}

impl DedupService {
    /// Create a new dedup service
    pub fn new(storage_root: &Path) -> Self {
        let blob_root = storage_root.join(".blobs");
        let temp_root = storage_root.join(".dedup_temp");
        let index_path = storage_root.join(".dedup_index.json");
        
        Self {
            blob_root,
            temp_root,
            index: Arc::new(RwLock::new(HashMap::new())),
            index_path,
            stats: Arc::new(RwLock::new(DedupStats::default())),
        }
    }
    
    /// Initialize the service (create directories, load index)
    pub async fn initialize(&self) -> std::io::Result<()> {
        // Create directories
        fs::create_dir_all(&self.blob_root).await?;
        fs::create_dir_all(&self.temp_root).await?;
        
        // Create hash prefix directories (00-ff)
        for i in 0..=255u8 {
            let prefix = format!("{:02x}", i);
            fs::create_dir_all(self.blob_root.join(&prefix)).await?;
        }
        
        // Load existing index
        self.load_index().await?;
        
        tracing::info!(
            "🔗 Dedup service initialized: {} blobs, {} bytes stored",
            self.stats.read().await.total_blobs,
            self.stats.read().await.total_bytes_stored
        );
        
        Ok(())
    }
    
    /// Load index from disk
    async fn load_index(&self) -> std::io::Result<()> {
        if !self.index_path.exists() {
            return Ok(());
        }
        
        let content = fs::read_to_string(&self.index_path).await?;
        let entries: Vec<BlobMetadata> = serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        
        let mut index = self.index.write().await;
        let mut stats = self.stats.write().await;
        
        for entry in entries {
            stats.total_blobs += 1;
            stats.total_bytes_stored += entry.size;
            stats.total_bytes_referenced += entry.size * entry.ref_count as u64;
            
            index.insert(entry.hash.clone(), entry);
        }
        
        stats.bytes_saved = stats.total_bytes_referenced.saturating_sub(stats.total_bytes_stored);
        if stats.total_bytes_stored > 0 {
            stats.dedup_ratio = stats.total_bytes_referenced as f64 / stats.total_bytes_stored as f64;
        }
        
        Ok(())
    }
    
    /// Save index to disk
    async fn save_index(&self) -> std::io::Result<()> {
        let index = self.index.read().await;
        let entries: Vec<&BlobMetadata> = index.values().collect();
        let content = serde_json::to_string_pretty(&entries)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        
        // Write atomically
        let temp_path = self.index_path.with_extension("json.tmp");
        fs::write(&temp_path, content).await?;
        fs::rename(&temp_path, &self.index_path).await?;
        
        Ok(())
    }
    
    /// Get the blob path for a given hash
    pub fn blob_path(&self, hash: &str) -> PathBuf {
        // Use first 2 chars as directory prefix for better filesystem distribution
        let prefix = &hash[0..2];
        self.blob_root.join(prefix).join(format!("{}.blob", hash))
    }
    
    /// Calculate SHA-256 hash of content
    pub fn hash_bytes(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        hex::encode(hasher.finalize())
    }
    
    /// Calculate SHA-256 hash of a file (streaming)
    pub async fn hash_file(path: &Path) -> std::io::Result<String> {
        let file = File::open(path).await?;
        let mut reader = BufReader::with_capacity(HASH_CHUNK_SIZE, file);
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; HASH_CHUNK_SIZE];
        
        loop {
            let bytes_read = reader.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        
        Ok(hex::encode(hasher.finalize()))
    }
    
    /// Check if a blob exists
    pub async fn blob_exists(&self, hash: &str) -> bool {
        let index = self.index.read().await;
        index.contains_key(hash)
    }
    
    /// Get blob metadata
    pub async fn get_blob_metadata(&self, hash: &str) -> Option<BlobMetadata> {
        let index = self.index.read().await;
        index.get(hash).cloned()
    }
    
    /// Store content with deduplication (from bytes)
    pub async fn store_bytes(
        &self,
        content: &[u8],
        content_type: Option<String>,
    ) -> Result<DedupResult, String> {
        let size = content.len() as u64;
        
        // Skip dedup for tiny files
        if size < MIN_DEDUP_SIZE {
            return self.store_new_blob_from_bytes(content, content_type).await;
        }
        
        // Calculate hash
        let hash = Self::hash_bytes(content);
        
        // Check if already exists
        if self.blob_exists(&hash).await {
            // Increment reference count
            self.increment_ref_count(&hash).await?;
            
            let blob_path = self.blob_path(&hash);
            
            // Update stats
            {
                let mut stats = self.stats.write().await;
                stats.dedup_hits += 1;
                stats.bytes_saved += size;
                stats.total_bytes_referenced += size;
                if stats.total_bytes_stored > 0 {
                    stats.dedup_ratio = stats.total_bytes_referenced as f64 / stats.total_bytes_stored as f64;
                }
            }
            
            tracing::info!("🔗 DEDUP HIT: {} ({} bytes saved)", &hash[..12], size);
            
            return Ok(DedupResult::ExistingBlob {
                hash,
                size,
                blob_path,
                saved_bytes: size,
            });
        }
        
        // Store new blob
        self.store_new_blob_from_bytes_with_hash(content, content_type, hash).await
    }
    
    /// Store new blob from bytes (no dedup check)
    async fn store_new_blob_from_bytes(
        &self,
        content: &[u8],
        content_type: Option<String>,
    ) -> Result<DedupResult, String> {
        let hash = Self::hash_bytes(content);
        self.store_new_blob_from_bytes_with_hash(content, content_type, hash).await
    }
    
    /// Store new blob from bytes with known hash
    async fn store_new_blob_from_bytes_with_hash(
        &self,
        content: &[u8],
        content_type: Option<String>,
        hash: String,
    ) -> Result<DedupResult, String> {
        let size = content.len() as u64;
        let blob_path = self.blob_path(&hash);
        
        // Ensure parent directory exists
        if let Some(parent) = blob_path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| format!("Failed to create blob directory: {}", e))?;
        }
        
        // Write blob atomically
        let temp_path = self.temp_root.join(format!("{}.tmp", uuid::Uuid::new_v4()));
        fs::write(&temp_path, content).await
            .map_err(|e| format!("Failed to write temp blob: {}", e))?;
        
        fs::rename(&temp_path, &blob_path).await
            .map_err(|e| format!("Failed to move blob to final location: {}", e))?;
        
        // Register in index
        let metadata = BlobMetadata {
            hash: hash.clone(),
            size,
            ref_count: 1,
            created_at: chrono::Utc::now(),
            content_type,
        };
        
        {
            let mut index = self.index.write().await;
            index.insert(hash.clone(), metadata);
        }
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_blobs += 1;
            stats.total_bytes_stored += size;
            stats.total_bytes_referenced += size;
            if stats.total_bytes_stored > 0 {
                stats.dedup_ratio = stats.total_bytes_referenced as f64 / stats.total_bytes_stored as f64;
            }
        }
        
        // Save index periodically (every 100 new blobs or async)
        let save_index = self.stats.read().await.total_blobs % 100 == 0;
        if save_index {
            let _ = self.save_index().await;
        }
        
        tracing::info!("💾 NEW BLOB: {} ({} bytes)", &hash[..12], size);
        
        Ok(DedupResult::NewBlob {
            hash,
            size,
            blob_path,
        })
    }
    
    /// Store content with deduplication (streaming from file)
    pub async fn store_from_file(
        &self,
        source_path: &Path,
        content_type: Option<String>,
    ) -> Result<DedupResult, String> {
        let file_size = fs::metadata(source_path).await
            .map_err(|e| format!("Failed to get file metadata: {}", e))?
            .len();
        
        // Skip dedup for tiny files
        if file_size < MIN_DEDUP_SIZE {
            let content = fs::read(source_path).await
                .map_err(|e| format!("Failed to read file: {}", e))?;
            return self.store_new_blob_from_bytes(&content, content_type).await;
        }
        
        // Calculate hash (streaming)
        let hash = Self::hash_file(source_path).await
            .map_err(|e| format!("Failed to hash file: {}", e))?;
        
        // Check if already exists
        if self.blob_exists(&hash).await {
            // Increment reference count
            self.increment_ref_count(&hash).await?;
            
            let blob_path = self.blob_path(&hash);
            
            // Update stats
            {
                let mut stats = self.stats.write().await;
                stats.dedup_hits += 1;
                stats.bytes_saved += file_size;
                stats.total_bytes_referenced += file_size;
                if stats.total_bytes_stored > 0 {
                    stats.dedup_ratio = stats.total_bytes_referenced as f64 / stats.total_bytes_stored as f64;
                }
            }
            
            // Delete source file since we don't need it
            let _ = fs::remove_file(source_path).await;
            
            tracing::info!("🔗 DEDUP HIT (file): {} ({} bytes saved)", &hash[..12], file_size);
            
            return Ok(DedupResult::ExistingBlob {
                hash,
                size: file_size,
                blob_path,
                saved_bytes: file_size,
            });
        }
        
        // Move file to blob store
        let blob_path = self.blob_path(&hash);
        
        if let Some(parent) = blob_path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| format!("Failed to create blob directory: {}", e))?;
        }
        
        fs::rename(source_path, &blob_path).await
            .map_err(|e| format!("Failed to move file to blob store: {}", e))?;
        
        // Register in index
        let metadata = BlobMetadata {
            hash: hash.clone(),
            size: file_size,
            ref_count: 1,
            created_at: chrono::Utc::now(),
            content_type,
        };
        
        {
            let mut index = self.index.write().await;
            index.insert(hash.clone(), metadata);
        }
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_blobs += 1;
            stats.total_bytes_stored += file_size;
            stats.total_bytes_referenced += file_size;
            if stats.total_bytes_stored > 0 {
                stats.dedup_ratio = stats.total_bytes_referenced as f64 / stats.total_bytes_stored as f64;
            }
        }
        
        tracing::info!("💾 NEW BLOB (file): {} ({} bytes)", &hash[..12], file_size);
        
        Ok(DedupResult::NewBlob {
            hash,
            size: file_size,
            blob_path,
        })
    }
    
    /// Increment reference count for a blob
    async fn increment_ref_count(&self, hash: &str) -> Result<(), String> {
        let mut index = self.index.write().await;
        
        if let Some(metadata) = index.get_mut(hash) {
            metadata.ref_count += 1;
            Ok(())
        } else {
            Err(format!("Blob not found: {}", hash))
        }
    }
    
    /// Add a reference to a blob (used when creating file references)
    pub async fn add_reference(&self, hash: &str) -> Result<(), String> {
        self.increment_ref_count(hash).await?;
        
        // Update stats
        if let Some(metadata) = self.get_blob_metadata(hash).await {
            let mut stats = self.stats.write().await;
            stats.total_bytes_referenced += metadata.size;
            if stats.total_bytes_stored > 0 {
                stats.dedup_ratio = stats.total_bytes_referenced as f64 / stats.total_bytes_stored as f64;
            }
        }
        
        Ok(())
    }
    
    /// Remove a reference to a blob, delete blob if ref_count reaches 0
    pub async fn remove_reference(&self, hash: &str) -> Result<bool, String> {
        let should_delete = {
            let mut index = self.index.write().await;
            
            if let Some(metadata) = index.get_mut(hash) {
                metadata.ref_count = metadata.ref_count.saturating_sub(1);
                
                // Update stats
                {
                    let mut stats = self.stats.write().await;
                    stats.total_bytes_referenced = stats.total_bytes_referenced.saturating_sub(metadata.size);
                    stats.bytes_saved = stats.bytes_saved.saturating_sub(metadata.size);
                    if stats.total_bytes_stored > 0 {
                        stats.dedup_ratio = stats.total_bytes_referenced as f64 / stats.total_bytes_stored as f64;
                    }
                }
                
                metadata.ref_count == 0
            } else {
                return Ok(false);
            }
        };
        
        if should_delete {
            // Remove from index
            let removed_metadata = {
                let mut index = self.index.write().await;
                index.remove(hash)
            };
            
            if let Some(metadata) = removed_metadata {
                // Delete blob file
                let blob_path = self.blob_path(hash);
                if let Err(e) = fs::remove_file(&blob_path).await {
                    tracing::warn!("Failed to delete blob {}: {}", hash, e);
                }
                
                // Update stats
                {
                    let mut stats = self.stats.write().await;
                    stats.total_blobs = stats.total_blobs.saturating_sub(1);
                    stats.total_bytes_stored = stats.total_bytes_stored.saturating_sub(metadata.size);
                    if stats.total_bytes_stored > 0 {
                        stats.dedup_ratio = stats.total_bytes_referenced as f64 / stats.total_bytes_stored as f64;
                    } else {
                        stats.dedup_ratio = 1.0;
                    }
                }
                
                tracing::info!("🗑️ BLOB DELETED: {} (no more references)", &hash[..12]);
            }
            
            Ok(true)
        } else {
            tracing::debug!("📎 Reference removed from blob {}", &hash[..12]);
            Ok(false)
        }
    }
    
    /// Read blob content
    pub async fn read_blob(&self, hash: &str) -> Result<Vec<u8>, String> {
        let blob_path = self.blob_path(hash);
        
        if !blob_path.exists() {
            return Err(format!("Blob not found: {}", hash));
        }
        
        fs::read(&blob_path).await
            .map_err(|e| format!("Failed to read blob: {}", e))
    }
    
    /// Read blob as Bytes
    pub async fn read_blob_bytes(&self, hash: &str) -> Result<Bytes, String> {
        self.read_blob(hash).await.map(Bytes::from)
    }
    
    /// Get statistics
    pub async fn get_stats(&self) -> DedupStats {
        self.stats.read().await.clone()
    }
    
    /// Flush index to disk
    pub async fn flush(&self) -> std::io::Result<()> {
        self.save_index().await
    }
    
    /// Verify integrity of all blobs
    pub async fn verify_integrity(&self) -> Result<Vec<String>, String> {
        let mut corrupted = Vec::new();
        let index = self.index.read().await;
        
        for (hash, metadata) in index.iter() {
            let blob_path = self.blob_path(hash);
            
            // Check file exists
            if !blob_path.exists() {
                corrupted.push(format!("{}: file missing", hash));
                continue;
            }
            
            // Verify hash
            match Self::hash_file(&blob_path).await {
                Ok(actual_hash) => {
                    if actual_hash != *hash {
                        corrupted.push(format!("{}: hash mismatch (actual: {})", hash, actual_hash));
                    }
                },
                Err(e) => {
                    corrupted.push(format!("{}: read error ({})", hash, e));
                }
            }
            
            // Check size
            if let Ok(file_meta) = fs::metadata(&blob_path).await
                && file_meta.len() != metadata.size {
                    corrupted.push(format!(
                        "{}: size mismatch (expected: {}, actual: {})",
                        hash, metadata.size, file_meta.len()
                    ));
                }
        }
        
        if corrupted.is_empty() {
            tracing::info!("✅ Integrity check passed for {} blobs", index.len());
        } else {
            tracing::warn!("⚠️ Integrity check found {} issues", corrupted.len());
        }
        
        Ok(corrupted)
    }
    
    /// Garbage collect orphaned blobs (blobs with ref_count=0)
    pub async fn garbage_collect(&self) -> Result<(u64, u64), String> {
        let orphans: Vec<(String, u64)> = {
            let index = self.index.read().await;
            index.iter()
                .filter(|(_, m)| m.ref_count == 0)
                .map(|(h, m)| (h.clone(), m.size))
                .collect()
        };
        
        let mut deleted_count = 0u64;
        let mut deleted_bytes = 0u64;
        
        for (hash, size) in orphans {
            if self.remove_reference(&hash).await.is_ok() {
                deleted_count += 1;
                deleted_bytes += size;
            }
        }
        
        if deleted_count > 0 {
            let _ = self.save_index().await;
            tracing::info!(
                "🧹 Garbage collected {} blobs ({} bytes)",
                deleted_count, deleted_bytes
            );
        }
        
        Ok((deleted_count, deleted_bytes))
    }
}

// ─── Port implementation ─────────────────────────────────────────────────────

/// Convert infra DedupResult to port DedupResultDto.
impl From<DedupResult> for DedupResultDto {
    fn from(result: DedupResult) -> Self {
        match result {
            DedupResult::NewBlob { hash, size, blob_path } => {
                DedupResultDto::NewBlob { hash, size, blob_path }
            }
            DedupResult::ExistingBlob { hash, size, blob_path, saved_bytes } => {
                DedupResultDto::ExistingBlob { hash, size, blob_path, saved_bytes }
            }
        }
    }
}

/// Convert infra BlobMetadata to port BlobMetadataDto.
impl From<BlobMetadata> for BlobMetadataDto {
    fn from(m: BlobMetadata) -> Self {
        BlobMetadataDto {
            hash: m.hash,
            size: m.size,
            ref_count: m.ref_count,
            content_type: m.content_type,
        }
    }
}

/// Convert infra DedupStats to port DedupStatsDto.
impl From<DedupStats> for DedupStatsDto {
    fn from(s: DedupStats) -> Self {
        DedupStatsDto {
            total_blobs: s.total_blobs,
            total_bytes_stored: s.total_bytes_stored,
            total_bytes_referenced: s.total_bytes_referenced,
            bytes_saved: s.bytes_saved,
            dedup_hits: s.dedup_hits,
            dedup_ratio: s.dedup_ratio,
        }
    }
}

#[async_trait]
impl DedupPort for DedupService {
    async fn store_bytes(
        &self,
        content: &[u8],
        content_type: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        self.store_bytes(content, content_type).await
            .map(Into::into)
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "Dedup", e))
    }

    async fn store_from_file(
        &self,
        source_path: &Path,
        content_type: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        self.store_from_file(source_path, content_type).await
            .map(Into::into)
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "Dedup", e))
    }

    async fn blob_exists(&self, hash: &str) -> bool {
        self.blob_exists(hash).await
    }

    async fn get_blob_metadata(&self, hash: &str) -> Option<BlobMetadataDto> {
        self.get_blob_metadata(hash).await.map(Into::into)
    }

    async fn read_blob(&self, hash: &str) -> Result<Vec<u8>, DomainError> {
        self.read_blob(hash).await
            .map_err(|e| DomainError::new(ErrorKind::NotFound, "Blob", e))
    }

    async fn read_blob_bytes(&self, hash: &str) -> Result<Bytes, DomainError> {
        self.read_blob_bytes(hash).await
            .map_err(|e| DomainError::new(ErrorKind::NotFound, "Blob", e))
    }

    async fn add_reference(&self, hash: &str) -> Result<(), DomainError> {
        self.add_reference(hash).await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "Blob", e))
    }

    async fn remove_reference(&self, hash: &str) -> Result<bool, DomainError> {
        self.remove_reference(hash).await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "Blob", e))
    }

    fn hash_bytes(&self, content: &[u8]) -> String {
        DedupService::hash_bytes(content)
    }

    async fn hash_file(&self, path: &Path) -> Result<String, DomainError> {
        DedupService::hash_file(path).await.map_err(DomainError::from)
    }

    async fn get_stats(&self) -> DedupStatsDto {
        self.get_stats().await.into()
    }

    async fn flush(&self) -> Result<(), DomainError> {
        self.flush().await.map_err(DomainError::from)
    }

    async fn verify_integrity(&self) -> Result<Vec<String>, DomainError> {
        self.verify_integrity().await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "Dedup", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_dedup_identical_content() {
        let temp_dir = TempDir::new().unwrap();
        let service = DedupService::new(temp_dir.path());
        service.initialize().await.unwrap();
        
        // Content must be >= MIN_DEDUP_SIZE (4096 bytes) for dedup to kick in
        let content = &b"Hello, World! This is test content for dedup. ".repeat(100);
        
        // First store
        let result1 = service.store_bytes(content, None).await.unwrap();
        assert!(!result1.was_deduplicated());
        
        // Second store (same content)
        let result2 = service.store_bytes(content, None).await.unwrap();
        assert!(result2.was_deduplicated());
        assert_eq!(result1.hash(), result2.hash());
        
        // Check stats
        let stats = service.get_stats().await;
        assert_eq!(stats.total_blobs, 1);
        assert_eq!(stats.dedup_hits, 1);
    }
    
    #[tokio::test]
    async fn test_reference_counting() {
        let temp_dir = TempDir::new().unwrap();
        let service = DedupService::new(temp_dir.path());
        service.initialize().await.unwrap();
        
        // Content must be >= MIN_DEDUP_SIZE (4096 bytes) for dedup to kick in
        let content = &b"Test content for reference counting. ".repeat(120);
        
        // Store twice
        let result1 = service.store_bytes(content, None).await.unwrap();
        let _result2 = service.store_bytes(content, None).await.unwrap();
        
        let hash = result1.hash().to_string();
        
        // Check ref count
        let metadata = service.get_blob_metadata(&hash).await.unwrap();
        assert_eq!(metadata.ref_count, 2);
        
        // Remove one reference
        let deleted = service.remove_reference(&hash).await.unwrap();
        assert!(!deleted);
        
        // Remove second reference (should delete)
        let deleted = service.remove_reference(&hash).await.unwrap();
        assert!(deleted);
        
        // Blob should be gone
        assert!(!service.blob_exists(&hash).await);
    }
}
