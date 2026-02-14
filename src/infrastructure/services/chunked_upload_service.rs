//! Chunked Upload Service - TUS-like Protocol for Large File Uploads
//! 
//! Enables parallel chunk uploads for files >10MB with:
//! - Resumable uploads (persist progress)
//! - Parallel chunk transfers (up to 6 concurrent)
//! - Automatic reassembly
//! - Expiration cleanup (24h)
//!
//! Protocol:
//! 1. POST /api/uploads     → Create upload session, get upload_id
//! 2. PATCH /api/uploads/:id → Upload chunks (parallel OK)
//! 3. HEAD /api/uploads/:id  → Check progress
//! 4. POST /api/uploads/:id/complete → Finalize and assemble

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs::{self, File, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use uuid::Uuid;
use async_trait::async_trait;

use crate::application::ports::chunked_upload_ports::{
    ChunkedUploadPort,
    CreateUploadResponseDto,
    ChunkUploadResponseDto,
    UploadStatusResponseDto,
};
use crate::domain::errors::{DomainError, ErrorKind};

/// Minimum file size to use chunked upload (10MB)
pub const CHUNKED_UPLOAD_THRESHOLD: usize = 10 * 1024 * 1024;

/// Default chunk size (5MB) - optimized for parallel transfers
pub const DEFAULT_CHUNK_SIZE: usize = 5 * 1024 * 1024;

/// Maximum concurrent chunks per upload
pub const MAX_PARALLEL_CHUNKS: usize = 6;

/// Upload session expiration time
const SESSION_EXPIRATION: Duration = Duration::from_secs(24 * 60 * 60); // 24 hours

/// Chunk status
#[derive(Debug, Clone, PartialEq)]
pub enum ChunkStatus {
    Pending,
    Uploading,
    Complete,
    Failed(String),
}

/// Individual chunk metadata
#[derive(Debug, Clone)]
pub struct ChunkInfo {
    pub index: usize,
    pub offset: u64,
    pub size: usize,
    pub status: ChunkStatus,
    pub checksum: Option<String>,
}

/// Upload session state
#[derive(Debug, Clone)]
pub struct UploadSession {
    pub id: String,
    pub filename: String,
    pub folder_id: Option<String>,
    pub content_type: String,
    pub total_size: u64,
    pub chunk_size: usize,
    pub chunks: Vec<ChunkInfo>,
    pub created_at: Instant,
    pub last_activity: Instant,
    pub temp_dir: PathBuf,
    pub bytes_received: u64,
}

impl UploadSession {
    /// Calculate number of chunks needed
    pub fn calculate_chunk_count(total_size: u64, chunk_size: usize) -> usize {
        (total_size as usize).div_ceil(chunk_size).max(1)
    }
    
    /// Get upload progress (0.0 - 1.0)
    pub fn progress(&self) -> f64 {
        if self.total_size == 0 {
            return 1.0;
        }
        self.bytes_received as f64 / self.total_size as f64
    }
    
    /// Check if all chunks are complete
    pub fn is_complete(&self) -> bool {
        self.chunks.iter().all(|c| c.status == ChunkStatus::Complete)
    }
    
    /// Get pending chunk indices
    pub fn pending_chunks(&self) -> Vec<usize> {
        self.chunks
            .iter()
            .enumerate()
            .filter(|(_, c)| c.status == ChunkStatus::Pending)
            .map(|(i, _)| i)
            .collect()
    }
    
    /// Check if session has expired
    pub fn is_expired(&self) -> bool {
        self.last_activity.elapsed() > SESSION_EXPIRATION
    }
}

/// Response for upload session creation
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreateUploadResponse {
    pub upload_id: String,
    pub chunk_size: usize,
    pub total_chunks: usize,
    pub expires_at: u64,
}

/// Response for chunk upload
#[derive(Debug, Clone, serde::Serialize)]
pub struct ChunkUploadResponse {
    pub chunk_index: usize,
    pub bytes_received: u64,
    pub progress: f64,
    pub is_complete: bool,
}

/// Response for upload status
#[derive(Debug, Clone, serde::Serialize)]
pub struct UploadStatusResponse {
    pub upload_id: String,
    pub filename: String,
    pub total_size: u64,
    pub bytes_received: u64,
    pub progress: f64,
    pub total_chunks: usize,
    pub completed_chunks: usize,
    pub pending_chunks: Vec<usize>,
    pub is_complete: bool,
}

/// Chunked Upload Service
pub struct ChunkedUploadService {
    sessions: Arc<RwLock<HashMap<String, UploadSession>>>,
    temp_base_dir: PathBuf,
}

impl ChunkedUploadService {
    /// Create new service with temp directory for chunks
    pub fn new(temp_base_dir: PathBuf) -> Self {
        let service = Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            temp_base_dir,
        };
        
        // Start cleanup task
        let sessions_clone = service.sessions.clone();
        let temp_dir_clone = service.temp_base_dir.clone();
        tokio::spawn(async move {
            Self::cleanup_loop(sessions_clone, temp_dir_clone).await;
        });
        
        service
    }
    
    /// Background task to clean expired sessions
    async fn cleanup_loop(
        sessions: Arc<RwLock<HashMap<String, UploadSession>>>,
        temp_base_dir: PathBuf,
    ) {
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Every hour
        
        loop {
            interval.tick().await;
            
            let expired: Vec<String> = {
                let sessions = sessions.read().await;
                sessions
                    .iter()
                    .filter(|(_, s)| s.is_expired())
                    .map(|(id, _)| id.clone())
                    .collect()
            };
            
            for id in expired {
                let mut sessions = sessions.write().await;
                if let Some(session) = sessions.remove(&id) {
                    // Clean up temp files
                    if let Err(e) = fs::remove_dir_all(&session.temp_dir).await {
                        tracing::warn!("Failed to cleanup expired upload {}: {}", id, e);
                    } else {
                        tracing::info!("🧹 Cleaned expired upload session: {}", id);
                    }
                }
            }
            
            // Also clean orphaned temp directories
            if let Ok(mut entries) = fs::read_dir(&temp_base_dir).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    if path.is_dir() {
                        let dir_name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        
                        // Check if this directory belongs to an active session
                        let sessions = sessions.read().await;
                        if !sessions.contains_key(dir_name) {
                            // Check if directory is old (>24h)
                            if let Ok(metadata) = fs::metadata(&path).await
                                && let Ok(modified) = metadata.modified()
                                    && modified.elapsed().unwrap_or_default() > SESSION_EXPIRATION {
                                        let _ = fs::remove_dir_all(&path).await;
                                        tracing::info!("🧹 Cleaned orphaned upload dir: {:?}", path);
                                    }
                        }
                    }
                }
            }
        }
    }
    
    /// Create a new upload session
    pub async fn create_session(
        &self,
        filename: String,
        folder_id: Option<String>,
        content_type: String,
        total_size: u64,
        chunk_size: Option<usize>,
    ) -> Result<CreateUploadResponse, String> {
        let upload_id = Uuid::new_v4().to_string();
        let chunk_size = chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE);
        let chunk_count = UploadSession::calculate_chunk_count(total_size, chunk_size);
        
        // Create temp directory for chunks
        let temp_dir = self.temp_base_dir.join(&upload_id);
        fs::create_dir_all(&temp_dir).await
            .map_err(|e| format!("Failed to create temp directory: {}", e))?;
        
        // Initialize chunk metadata
        let mut chunks = Vec::with_capacity(chunk_count);
        let mut offset: u64 = 0;
        
        for i in 0..chunk_count {
            let size = if i == chunk_count - 1 {
                // Last chunk may be smaller
                (total_size - offset) as usize
            } else {
                chunk_size
            };
            
            chunks.push(ChunkInfo {
                index: i,
                offset,
                size,
                status: ChunkStatus::Pending,
                checksum: None,
            });
            
            offset += size as u64;
        }
        
        let now = Instant::now();
        let session = UploadSession {
            id: upload_id.clone(),
            filename,
            folder_id,
            content_type,
            total_size,
            chunk_size,
            chunks,
            created_at: now,
            last_activity: now,
            temp_dir,
            bytes_received: 0,
        };
        
        let expires_at = SESSION_EXPIRATION.as_secs();
        
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(upload_id.clone(), session);
        }
        
        tracing::info!(
            "📤 Created chunked upload session: {} ({} chunks, {} bytes each)", 
            upload_id, chunk_count, chunk_size
        );
        
        Ok(CreateUploadResponse {
            upload_id,
            chunk_size,
            total_chunks: chunk_count,
            expires_at,
        })
    }
    
    /// Upload a single chunk
    pub async fn upload_chunk(
        &self,
        upload_id: &str,
        chunk_index: usize,
        data: bytes::Bytes,
        checksum: Option<String>,
    ) -> Result<ChunkUploadResponse, String> {
        // Validate session exists and chunk index is valid
        let (chunk_path, expected_size) = {
            let sessions = self.sessions.read().await;
            let session = sessions.get(upload_id)
                .ok_or_else(|| format!("Upload session not found: {}", upload_id))?;
            
            if chunk_index >= session.chunks.len() {
                return Err(format!("Invalid chunk index: {} (max: {})", 
                    chunk_index, session.chunks.len() - 1));
            }
            
            let chunk = &session.chunks[chunk_index];
            if chunk.status == ChunkStatus::Complete {
                return Err(format!("Chunk {} already uploaded", chunk_index));
            }
            
            (session.temp_dir.join(format!("chunk_{:06}", chunk_index)), chunk.size)
        };
        
        // Validate chunk size
        if data.len() != expected_size {
            return Err(format!(
                "Invalid chunk size: expected {} bytes, got {} bytes",
                expected_size, data.len()
            ));
        }
        
        // Verify checksum if provided
        if let Some(ref expected_checksum) = checksum {
            let actual_checksum = format!("{:x}", md5::compute(&data));
            if &actual_checksum != expected_checksum {
                return Err(format!(
                    "Checksum mismatch: expected {}, got {}",
                    expected_checksum, actual_checksum
                ));
            }
        }
        
        // Write chunk to temp file
        let mut file = File::create(&chunk_path).await
            .map_err(|e| format!("Failed to create chunk file: {}", e))?;
        
        file.write_all(&data).await
            .map_err(|e| format!("Failed to write chunk: {}", e))?;
        
        file.sync_all().await
            .map_err(|e| format!("Failed to sync chunk: {}", e))?;
        
        // Update session state
        let (bytes_received, progress, is_complete) = {
            let mut sessions = self.sessions.write().await;
            let session = sessions.get_mut(upload_id)
                .ok_or_else(|| "Session disappeared".to_string())?;
            
            session.chunks[chunk_index].status = ChunkStatus::Complete;
            session.chunks[chunk_index].checksum = checksum;
            session.bytes_received += data.len() as u64;
            session.last_activity = Instant::now();
            
            (session.bytes_received, session.progress(), session.is_complete())
        };
        
        tracing::debug!(
            "📦 Chunk {}/{} uploaded for {} ({:.1}% complete)",
            chunk_index + 1,
            expected_size,
            upload_id,
            progress * 100.0
        );
        
        Ok(ChunkUploadResponse {
            chunk_index,
            bytes_received,
            progress,
            is_complete,
        })
    }
    
    /// Get upload status
    pub async fn get_status(&self, upload_id: &str) -> Result<UploadStatusResponse, String> {
        let sessions = self.sessions.read().await;
        let session = sessions.get(upload_id)
            .ok_or_else(|| format!("Upload session not found: {}", upload_id))?;
        
        let completed_chunks = session.chunks
            .iter()
            .filter(|c| c.status == ChunkStatus::Complete)
            .count();
        
        Ok(UploadStatusResponse {
            upload_id: session.id.clone(),
            filename: session.filename.clone(),
            total_size: session.total_size,
            bytes_received: session.bytes_received,
            progress: session.progress(),
            total_chunks: session.chunks.len(),
            completed_chunks,
            pending_chunks: session.pending_chunks(),
            is_complete: session.is_complete(),
        })
    }
    
    /// Assemble chunks into final file and return the path
    /// Returns (assembled_file_path, filename, folder_id, content_type, total_size)
    pub async fn complete_upload(
        &self,
        upload_id: &str,
    ) -> Result<(PathBuf, String, Option<String>, String, u64), String> {
        // Get session and validate completion
        let session = {
            let sessions = self.sessions.read().await;
            let session = sessions.get(upload_id)
                .ok_or_else(|| format!("Upload session not found: {}", upload_id))?;
            
            if !session.is_complete() {
                let pending = session.pending_chunks();
                return Err(format!(
                    "Upload not complete. Missing chunks: {:?}",
                    pending
                ));
            }
            
            session.clone()
        };
        
        // Assemble file
        let assembled_path = session.temp_dir.join("assembled");
        let mut output = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&assembled_path)
            .await
            .map_err(|e| format!("Failed to create assembled file: {}", e))?;
        
        // Append chunks in order
        for chunk in &session.chunks {
            let chunk_path = session.temp_dir.join(format!("chunk_{:06}", chunk.index));
            let chunk_data = fs::read(&chunk_path).await
                .map_err(|e| format!("Failed to read chunk {}: {}", chunk.index, e))?;
            
            output.write_all(&chunk_data).await
                .map_err(|e| format!("Failed to write chunk {} to assembled file: {}", chunk.index, e))?;
        }
        
        output.sync_all().await
            .map_err(|e| format!("Failed to sync assembled file: {}", e))?;
        
        // Clean up chunk files (keep assembled)
        for chunk in &session.chunks {
            let chunk_path = session.temp_dir.join(format!("chunk_{:06}", chunk.index));
            let _ = fs::remove_file(&chunk_path).await;
        }
        
        tracing::info!(
            "✅ Assembled chunked upload: {} ({} bytes from {} chunks)",
            session.filename,
            session.total_size,
            session.chunks.len()
        );
        
        Ok((
            assembled_path,
            session.filename.clone(),
            session.folder_id.clone(),
            session.content_type.clone(),
            session.total_size,
        ))
    }
    
    /// Finalize upload: move assembled file to final location and cleanup session
    pub async fn finalize_upload(&self, upload_id: &str) -> Result<(), String> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.remove(upload_id) {
            // Clean up entire temp directory
            if let Err(e) = fs::remove_dir_all(&session.temp_dir).await {
                tracing::warn!("Failed to cleanup upload {}: {}", upload_id, e);
            }
        }
        Ok(())
    }
    
    /// Cancel an upload and cleanup
    pub async fn cancel_upload(&self, upload_id: &str) -> Result<(), String> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.remove(upload_id) {
            if let Err(e) = fs::remove_dir_all(&session.temp_dir).await {
                tracing::warn!("Failed to cleanup cancelled upload {}: {}", upload_id, e);
            }
            tracing::info!("❌ Cancelled chunked upload: {}", upload_id);
        }
        Ok(())
    }
    
    /// Check if file size qualifies for chunked upload
    pub fn should_use_chunked(size: u64) -> bool {
        size as usize >= CHUNKED_UPLOAD_THRESHOLD
    }
    
    /// Get active session count (for monitoring)
    pub async fn active_sessions(&self) -> usize {
        self.sessions.read().await.len()
    }
}

// ─── Port implementation ─────────────────────────────────────────────────────

#[async_trait]
impl ChunkedUploadPort for ChunkedUploadService {
    async fn create_session(
        &self,
        filename: String,
        folder_id: Option<String>,
        content_type: String,
        total_size: u64,
        chunk_size: Option<usize>,
    ) -> Result<CreateUploadResponseDto, DomainError> {
        let resp = self.create_session(filename, folder_id, content_type, total_size, chunk_size).await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "ChunkedUpload", e))?;
        Ok(CreateUploadResponseDto {
            upload_id: resp.upload_id,
            chunk_size: resp.chunk_size,
            total_chunks: resp.total_chunks,
            expires_at: resp.expires_at,
        })
    }

    async fn upload_chunk(
        &self,
        upload_id: &str,
        chunk_index: usize,
        data: bytes::Bytes,
        checksum: Option<String>,
    ) -> Result<ChunkUploadResponseDto, DomainError> {
        let resp = self.upload_chunk(upload_id, chunk_index, data, checksum).await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "ChunkedUpload", e))?;
        Ok(ChunkUploadResponseDto {
            chunk_index: resp.chunk_index,
            bytes_received: resp.bytes_received,
            progress: resp.progress,
            is_complete: resp.is_complete,
        })
    }

    async fn get_status(
        &self,
        upload_id: &str,
    ) -> Result<UploadStatusResponseDto, DomainError> {
        let resp = self.get_status(upload_id).await
            .map_err(|e| DomainError::new(ErrorKind::NotFound, "ChunkedUpload", e))?;
        Ok(UploadStatusResponseDto {
            upload_id: resp.upload_id,
            filename: resp.filename,
            total_size: resp.total_size,
            bytes_received: resp.bytes_received,
            progress: resp.progress,
            total_chunks: resp.total_chunks,
            completed_chunks: resp.completed_chunks,
            pending_chunks: resp.pending_chunks,
            is_complete: resp.is_complete,
        })
    }

    async fn complete_upload(
        &self,
        upload_id: &str,
    ) -> Result<(PathBuf, String, Option<String>, String, u64), DomainError> {
        self.complete_upload(upload_id).await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "ChunkedUpload", e))
    }

    async fn finalize_upload(
        &self,
        upload_id: &str,
    ) -> Result<(), DomainError> {
        self.finalize_upload(upload_id).await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "ChunkedUpload", e))
    }

    async fn cancel_upload(
        &self,
        upload_id: &str,
    ) -> Result<(), DomainError> {
        self.cancel_upload(upload_id).await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "ChunkedUpload", e))
    }

    fn should_use_chunked(&self, size: u64) -> bool {
        ChunkedUploadService::should_use_chunked(size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chunk_count_calculation() {
        // 10MB file with 5MB chunks = 2 chunks
        assert_eq!(UploadSession::calculate_chunk_count(10 * 1024 * 1024, 5 * 1024 * 1024), 2);
        
        // 11MB file with 5MB chunks = 3 chunks
        assert_eq!(UploadSession::calculate_chunk_count(11 * 1024 * 1024, 5 * 1024 * 1024), 3);
        
        // 1 byte file = 1 chunk
        assert_eq!(UploadSession::calculate_chunk_count(1, 5 * 1024 * 1024), 1);
        
        // 0 byte file = 1 chunk
        assert_eq!(UploadSession::calculate_chunk_count(0, 5 * 1024 * 1024), 1);
    }
    
    #[test]
    fn test_should_use_chunked() {
        assert!(!ChunkedUploadService::should_use_chunked(9 * 1024 * 1024));
        assert!(ChunkedUploadService::should_use_chunked(10 * 1024 * 1024));
        assert!(ChunkedUploadService::should_use_chunked(100 * 1024 * 1024));
    }
}
