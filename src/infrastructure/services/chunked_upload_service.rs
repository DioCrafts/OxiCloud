//! Chunked Upload Service — TUS-like Protocol for Large File Uploads
//!
//! Enables parallel chunk uploads for files >10 MB with:
//! - **Persistent, resumable uploads** — progress survives server restarts.
//!   A `session.json` (written once on create) + a `progress.bin` bitmask
//!   (updated atomically on each chunk) are stored alongside the chunk files.
//!   On boot the service scans `temp_base_dir` and recovers any active sessions.
//! - Parallel chunk transfers (up to 6 concurrent)
//! - Automatic reassembly with hash-on-write (BLAKE3)
//! - Expiration cleanup (24 h)
//!
//! Protocol:
//! 1. POST /api/uploads     → Create upload session, get upload_id
//! 2. PATCH /api/uploads/:id → Upload chunks (parallel OK)
//! 3. HEAD /api/uploads/:id  → Check progress
//! 4. POST /api/uploads/:id/complete → Finalize and assemble

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::application::ports::chunked_upload_ports::{
    ChunkUploadResponseDto, ChunkedUploadPort, CreateUploadResponseDto, UploadStatusResponseDto,
};
use crate::domain::errors::{DomainError, ErrorKind};

/// Minimum file size to use chunked upload (10 MB)
pub const CHUNKED_UPLOAD_THRESHOLD: usize = 10 * 1024 * 1024;

/// Default chunk size (5 MB) — optimised for parallel transfers
pub const DEFAULT_CHUNK_SIZE: usize = 5 * 1024 * 1024;

/// Maximum concurrent chunks per upload
pub const MAX_PARALLEL_CHUNKS: usize = 6;

/// Upload session expiration time (24 h)
const SESSION_EXPIRATION: Duration = Duration::from_secs(24 * 60 * 60);

/// Sentinel file names inside each session directory
const SESSION_META_FILE: &str = "session.json";
const PROGRESS_FILE: &str = "progress.bin";

// ─── Serialisable types ──────────────────────────────────────────────────────

/// Chunk status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChunkStatus {
    Pending,
    Uploading,
    Complete,
    Failed(String),
}

/// Individual chunk metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    pub index: usize,
    pub offset: u64,
    pub size: usize,
    pub status: ChunkStatus,
    pub checksum: Option<String>,
}

/// Upload session state — fully serialisable for crash recovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadSession {
    pub id: String,
    pub filename: String,
    pub folder_id: Option<String>,
    pub content_type: String,
    pub total_size: u64,
    pub chunk_size: usize,
    pub chunks: Vec<ChunkInfo>,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub temp_dir: PathBuf,
    pub bytes_received: u64,
}

impl UploadSession {
    /// Calculate number of chunks needed
    pub fn calculate_chunk_count(total_size: u64, chunk_size: usize) -> usize {
        (total_size as usize).div_ceil(chunk_size).max(1)
    }

    /// Get upload progress (0.0 – 1.0)
    pub fn progress(&self) -> f64 {
        if self.total_size == 0 {
            return 1.0;
        }
        self.bytes_received as f64 / self.total_size as f64
    }

    /// Check if all chunks are complete
    pub fn is_complete(&self) -> bool {
        self.chunks
            .iter()
            .all(|c| c.status == ChunkStatus::Complete)
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
        let elapsed = Utc::now()
            .signed_duration_since(self.last_activity)
            .to_std()
            .unwrap_or(Duration::ZERO);
        elapsed > SESSION_EXPIRATION
    }

    // ── Persistence helpers ──────────────────────────────────────────────

    /// Build the completed-chunks bitmask (1 bit per chunk).
    fn build_progress_bitmask(&self) -> Vec<u8> {
        let len = self.chunks.len().div_ceil(8);
        let mut bitmask = vec![0u8; len];
        for chunk in &self.chunks {
            if chunk.status == ChunkStatus::Complete {
                bitmask[chunk.index / 8] |= 1 << (chunk.index % 8);
            }
        }
        bitmask
    }

    /// Apply a bitmask read from disk, marking matching chunks as `Complete`
    /// and recalculating `bytes_received`.
    fn apply_progress_bitmask(&mut self, bitmask: &[u8]) {
        self.bytes_received = 0;
        for chunk in &mut self.chunks {
            let byte_idx = chunk.index / 8;
            let bit_idx = chunk.index % 8;
            if byte_idx < bitmask.len() && (bitmask[byte_idx] & (1 << bit_idx)) != 0 {
                chunk.status = ChunkStatus::Complete;
                self.bytes_received += chunk.size as u64;
            }
        }
    }

    /// Persist the full session metadata once (on create).
    async fn persist_metadata(&self) -> Result<(), String> {
        let path = self.temp_dir.join(SESSION_META_FILE);
        let json =
            serde_json::to_vec(self).map_err(|e| format!("Failed to serialise session: {e}"))?;
        // Atomic write: write to .tmp then rename
        let tmp = self.temp_dir.join("session.json.tmp");
        fs::write(&tmp, &json)
            .await
            .map_err(|e| format!("Failed to write session metadata: {e}"))?;
        fs::rename(&tmp, &path)
            .await
            .map_err(|e| format!("Failed to rename session metadata: {e}"))?;
        Ok(())
    }

    /// Persist the lightweight progress bitmask (on each chunk upload).
    async fn persist_progress(&self) -> Result<(), String> {
        let bitmask = self.build_progress_bitmask();
        let path = self.temp_dir.join(PROGRESS_FILE);
        // Bitmask is tiny (< 512 B for up to 4 096 chunks).
        // A single write() of < 512 B is atomic on POSIX.
        fs::write(&path, &bitmask)
            .await
            .map_err(|e| format!("Failed to write progress bitmask: {e}"))?;
        Ok(())
    }
}

// ─── Service ─────────────────────────────────────────────────────────────────

/// Chunked Upload Service
///
/// Uses `DashMap` (sharded concurrent map) instead of a global `RwLock<HashMap>`
/// so that operations on independent upload sessions never contend with each
/// other.  Disk I/O (temp-dir cleanup) is always performed **outside** any
/// map lock to avoid blocking concurrent uploads.
pub struct ChunkedUploadService {
    sessions: Arc<DashMap<String, UploadSession>>,
    temp_base_dir: PathBuf,
}

impl ChunkedUploadService {
    /// Create the service, recover any persisted sessions, and start the
    /// background cleanup task.
    pub async fn new(temp_base_dir: PathBuf) -> Self {
        // Ensure the base directory exists
        let _ = fs::create_dir_all(&temp_base_dir).await;

        // Recover sessions that survived a restart
        let recovered = Self::recover_sessions(&temp_base_dir).await;
        let recovered_count = recovered.len();

        let service = Self {
            sessions: Arc::new(DashMap::from_iter(recovered)),
            temp_base_dir,
        };

        if recovered_count > 0 {
            tracing::info!("♻️  Recovered {recovered_count} chunked-upload session(s) from disk");
        }

        // Start cleanup task
        let sessions_clone = service.sessions.clone();
        let temp_dir_clone = service.temp_base_dir.clone();
        tokio::spawn(async move {
            Self::cleanup_loop(sessions_clone, temp_dir_clone).await;
        });

        service
    }

    /// Lightweight constructor that skips recovery and cleanup.
    /// Used only by `AppState::default()` (stub wiring).
    pub fn new_stub(temp_base_dir: PathBuf) -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            temp_base_dir,
        }
    }

    // ── Recovery ─────────────────────────────────────────────────────────

    /// Scan `temp_base_dir` for directories containing `session.json`,
    /// deserialise each session, apply the `progress.bin` bitmask, and
    /// verify the chunk files on disk actually exist for completed chunks.
    async fn recover_sessions(base: &Path) -> HashMap<String, UploadSession> {
        let mut sessions = HashMap::new();
        let mut entries = match fs::read_dir(base).await {
            Ok(e) => e,
            Err(_) => return sessions,
        };

        while let Ok(Some(entry)) = entries.next_entry().await {
            let dir = entry.path();
            if !dir.is_dir() {
                continue;
            }

            let meta_path = dir.join(SESSION_META_FILE);
            let meta_bytes = match fs::read(&meta_path).await {
                Ok(b) => b,
                Err(_) => continue, // no session.json → orphaned dir, skip
            };

            let mut session: UploadSession = match serde_json::from_slice(&meta_bytes) {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!("Skipping corrupt session in {:?}: {e}", dir);
                    continue;
                }
            };

            // Apply progress bitmask if present
            if let Ok(bitmask) = fs::read(dir.join(PROGRESS_FILE)).await {
                session.apply_progress_bitmask(&bitmask);
            } else {
                // No progress file → all chunks still pending (freshly created)
                session.bytes_received = 0;
                for chunk in &mut session.chunks {
                    chunk.status = ChunkStatus::Pending;
                }
            }

            // Verify chunk files on disk — downgrade to Pending if missing
            for chunk in &mut session.chunks {
                if chunk.status == ChunkStatus::Complete {
                    let chunk_path = dir.join(format!("chunk_{:06}", chunk.index));
                    if !chunk_path.exists() {
                        tracing::warn!(
                            "Chunk {} missing on disk for session {}, marking pending",
                            chunk.index,
                            session.id
                        );
                        chunk.status = ChunkStatus::Pending;
                        session.bytes_received =
                            session.bytes_received.saturating_sub(chunk.size as u64);
                    }
                }
            }

            // Skip expired sessions
            if session.is_expired() {
                tracing::info!("Skipping expired recovered session: {}", session.id);
                let _ = fs::remove_dir_all(&dir).await;
                continue;
            }

            tracing::info!(
                "♻️  Recovered session {} — {}/{} chunks ({:.0}%)",
                session.id,
                session
                    .chunks
                    .iter()
                    .filter(|c| c.status == ChunkStatus::Complete)
                    .count(),
                session.chunks.len(),
                session.progress() * 100.0
            );

            sessions.insert(session.id.clone(), session);
        }

        sessions
    }

    // ── Cleanup ──────────────────────────────────────────────────────────

    /// Background task to clean expired sessions
    async fn cleanup_loop(sessions: Arc<DashMap<String, UploadSession>>, temp_base_dir: PathBuf) {
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Every hour

        loop {
            interval.tick().await;

            // Collect expired session ids + temp dirs (lock-free iteration)
            let expired: Vec<(String, PathBuf)> = sessions
                .iter()
                .filter(|entry| entry.value().is_expired())
                .map(|entry| (entry.key().clone(), entry.value().temp_dir.clone()))
                .collect();

            // Remove from map (microseconds per entry) then clean disk OUTSIDE lock
            for (id, temp_dir) in expired {
                sessions.remove(&id);
                if let Err(e) = fs::remove_dir_all(&temp_dir).await {
                    tracing::warn!("Failed to cleanup expired upload {}: {}", id, e);
                } else {
                    tracing::info!("🧹 Cleaned expired upload session: {}", id);
                }
            }

            // Also clean orphaned temp directories (no session.json or very old)
            if let Ok(mut entries) = fs::read_dir(&temp_base_dir).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    if path.is_dir() {
                        let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                        if !sessions.contains_key(dir_name)
                            && let Ok(metadata) = fs::metadata(&path).await
                            && let Ok(modified) = metadata.modified()
                            && modified.elapsed().unwrap_or_default() > SESSION_EXPIRATION
                        {
                            let _ = fs::remove_dir_all(&path).await;
                            tracing::info!("🧹 Cleaned orphaned upload dir: {:?}", path);
                        }
                    }
                }
            }
        }
    }

    // ── Core operations ──────────────────────────────────────────────────

    /// Create a new upload session (persists `session.json` + empty `progress.bin`)
    async fn create_session_inner(
        &self,
        filename: String,
        folder_id: Option<String>,
        content_type: String,
        total_size: u64,
        chunk_size: Option<usize>,
    ) -> Result<CreateUploadResponseDto, String> {
        let upload_id = Uuid::new_v4().to_string();
        let chunk_size = chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE);
        let chunk_count = UploadSession::calculate_chunk_count(total_size, chunk_size);

        // Create temp directory for chunks
        let temp_dir = self.temp_base_dir.join(&upload_id);
        fs::create_dir_all(&temp_dir)
            .await
            .map_err(|e| format!("Failed to create temp directory: {e}"))?;

        // Initialise chunk metadata
        let mut chunks = Vec::with_capacity(chunk_count);
        let mut offset: u64 = 0;

        for i in 0..chunk_count {
            let size = if i == chunk_count - 1 {
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

        let now = Utc::now();
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

        // Persist metadata + empty progress to disk BEFORE inserting into RAM
        session.persist_metadata().await?;
        session.persist_progress().await?;

        self.sessions.insert(upload_id.clone(), session);

        tracing::info!(
            "📤 Created chunked upload session: {} ({} chunks, {} bytes each)",
            upload_id,
            chunk_count,
            chunk_size
        );

        Ok(CreateUploadResponseDto {
            upload_id,
            chunk_size,
            total_chunks: chunk_count,
            expires_at,
        })
    }

    /// Upload a single chunk (persists `progress.bin` after success)
    async fn upload_chunk_inner(
        &self,
        upload_id: &str,
        chunk_index: usize,
        data: bytes::Bytes,
        checksum: Option<String>,
    ) -> Result<ChunkUploadResponseDto, String> {
        // Validate session exists and chunk index is valid
        let (chunk_path, expected_size) = {
            let session = self
                .sessions
                .get(upload_id)
                .ok_or_else(|| format!("Upload session not found: {}", upload_id))?;

            if chunk_index >= session.chunks.len() {
                return Err(format!(
                    "Invalid chunk index: {} (max: {})",
                    chunk_index,
                    session.chunks.len() - 1
                ));
            }

            let chunk = &session.chunks[chunk_index];
            if chunk.status == ChunkStatus::Complete {
                return Err(format!("Chunk {} already uploaded", chunk_index));
            }

            (
                session.temp_dir.join(format!("chunk_{:06}", chunk_index)),
                chunk.size,
            )
        };

        // Validate chunk size
        if data.len() != expected_size {
            return Err(format!(
                "Invalid chunk size: expected {} bytes, got {} bytes",
                expected_size,
                data.len()
            ));
        }

        // Verify checksum if provided — MD5 is CPU-bound (~1.2 ms per 5 MB),
        // so we offload it to the blocking thread-pool to keep the Tokio
        // worker free for other connections.
        if let Some(ref expected_checksum) = checksum {
            let data_clone = data.clone(); // Bytes::clone is O(1) — just an Arc increment
            let actual_checksum =
                tokio::task::spawn_blocking(move || format!("{:x}", md5::compute(&data_clone)))
                    .await
                    .map_err(|e| format!("MD5 checksum task failed: {e}"))?;

            if actual_checksum != *expected_checksum {
                return Err(format!(
                    "Checksum mismatch: expected {}, got {}",
                    expected_checksum, actual_checksum
                ));
            }
        }

        // Write chunk to temp file
        let mut file = File::create(&chunk_path)
            .await
            .map_err(|e| format!("Failed to create chunk file: {e}"))?;

        file.write_all(&data)
            .await
            .map_err(|e| format!("Failed to write chunk: {e}"))?;

        // Update session state — DashMap shard lock held only for RAM updates (~µs).
        // Disk I/O (persist_progress) is done AFTER the ref is dropped so
        // concurrent uploads to other sessions are never blocked.
        let (bytes_received, progress, is_complete, persist_path, persist_bitmask) = {
            let mut session = self
                .sessions
                .get_mut(upload_id)
                .ok_or_else(|| "Session disappeared".to_string())?;

            session.chunks[chunk_index].status = ChunkStatus::Complete;
            session.chunks[chunk_index].checksum = checksum;
            session.bytes_received += data.len() as u64;
            session.last_activity = Utc::now();

            // Build bitmask while under lock (CPU-only, ~microseconds)
            let bitmask = session.build_progress_bitmask();
            let path = session.temp_dir.join(PROGRESS_FILE);

            (
                session.bytes_received,
                session.progress(),
                session.is_complete(),
                path,
                bitmask,
            )
        }; // DashMap shard ref dropped here — held only for RAM updates (~µs)

        // Persist bitmask to disk OUTSIDE the lock — no longer blocks other uploads
        if let Err(e) = fs::write(&persist_path, &persist_bitmask).await {
            tracing::warn!("Failed to persist progress for {upload_id}: {e}");
        }

        tracing::debug!(
            "📦 Chunk {}/{} uploaded for {} ({:.1}% complete)",
            chunk_index + 1,
            expected_size,
            upload_id,
            progress * 100.0
        );

        Ok(ChunkUploadResponseDto {
            chunk_index,
            bytes_received,
            progress,
            is_complete,
        })
    }

    /// Get upload status
    async fn get_status_inner(&self, upload_id: &str) -> Result<UploadStatusResponseDto, String> {
        let session = self
            .sessions
            .get(upload_id)
            .ok_or_else(|| format!("Upload session not found: {}", upload_id))?;

        let completed_chunks = session
            .chunks
            .iter()
            .filter(|c| c.status == ChunkStatus::Complete)
            .count();

        Ok(UploadStatusResponseDto {
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

    /// Assemble chunks into final file and return the path + pre-computed BLAKE3 hash.
    ///
    /// **Hash-on-Write**: BLAKE3 is computed while copying chunks into the
    /// assembled file, eliminating the second sequential read that dedup_service
    /// would otherwise need.
    ///
    /// Returns `(assembled_file_path, filename, folder_id, content_type, total_size, blake3_hash)`.
    async fn complete_upload_inner(
        &self,
        upload_id: &str,
    ) -> Result<(PathBuf, String, Option<String>, String, u64, String), String> {
        // Get session and validate completion.
        // Clone the session data and drop the DashMap ref immediately
        // so the shard is not held during the expensive assembly step.
        let session = {
            let entry = self
                .sessions
                .get(upload_id)
                .ok_or_else(|| format!("Upload session not found: {}", upload_id))?;

            if !entry.is_complete() {
                let pending = entry.pending_chunks();
                return Err(format!(
                    "Upload not complete. Missing chunks: {:?}",
                    pending
                ));
            }

            entry.clone()
        };

        // Assemble file with hash-on-write.
        //
        // The entire loop is offloaded to spawn_blocking because BLAKE3
        // hashing is CPU-bound and would otherwise block a Tokio worker,
        // starving all other connections.
        // Synchronous I/O is used inside the blocking thread — it avoids
        // the async reactor overhead and is actually faster for this
        // sequential workload.
        let assembled_path = session.temp_dir.join("assembled");
        let chunks_meta: Vec<(usize, PathBuf)> = session
            .chunks
            .iter()
            .map(|c| {
                (
                    c.index,
                    session.temp_dir.join(format!("chunk_{:06}", c.index)),
                )
            })
            .collect();
        let total_size = session.total_size;

        let hash = tokio::task::spawn_blocking(move || -> Result<String, String> {
            use std::io::{BufWriter as StdBufWriter, Read, Write};

            let raw_output = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&assembled_path)
                .map_err(|e| format!("Failed to create assembled file: {e}"))?;

            // Pre-allocate assembled file to reduce fragmentation
            let _ = raw_output.set_len(total_size);

            // 512 KB I/O buffers — 8× fewer syscalls than 64 KB
            let mut output = StdBufWriter::with_capacity(524_288, raw_output);
            let mut hasher = blake3::Hasher::new();

            // Single 512 KB read buffer reused across all chunks (avoids N allocations)
            let mut buf = vec![0u8; 524_288];
            for (index, chunk_path) in &chunks_meta {
                let mut chunk_file = std::fs::File::open(chunk_path)
                    .map_err(|e| format!("Failed to open chunk {index}: {e}"))?;
                loop {
                    let n = chunk_file
                        .read(&mut buf)
                        .map_err(|e| format!("Failed to read chunk {index}: {e}"))?;
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buf[..n]);
                    output.write_all(&buf[..n]).map_err(|e| {
                        format!("Failed to write chunk {index} to assembled file: {e}")
                    })?;
                }
            }

            output
                .flush()
                .map_err(|e| format!("Failed to flush assembled file: {e}"))?;

            // Clean up chunk files (keep assembled) — already on a blocking thread
            for (_index, chunk_path) in &chunks_meta {
                let _ = std::fs::remove_file(chunk_path);
            }

            Ok(hasher.finalize().to_hex().to_string())
        })
        .await
        .map_err(|e| format!("Assembly task panicked: {e}"))??;

        let assembled_path = session.temp_dir.join("assembled");

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
            hash,
        ))
    }

    /// Finalize upload: remove session from RAM, then clean disk OUTSIDE lock.
    async fn finalize_upload_inner(&self, upload_id: &str) -> Result<(), String> {
        // Remove from map (~µs) — releases shard immediately
        let removed = self.sessions.remove(upload_id).map(|(_, s)| s);

        // Disk I/O happens with NO lock held
        if let Some(session) = removed
            && let Err(e) = fs::remove_dir_all(&session.temp_dir).await
        {
            tracing::warn!("Failed to cleanup upload {}: {}", upload_id, e);
        }
        Ok(())
    }

    /// Cancel an upload and cleanup — disk I/O outside lock.
    async fn cancel_upload_inner(&self, upload_id: &str) -> Result<(), String> {
        // Remove from map (~µs)
        let removed = self.sessions.remove(upload_id).map(|(_, s)| s);

        // Disk I/O with NO lock held
        if let Some(session) = removed {
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
        self.sessions.len()
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
        self.create_session_inner(filename, folder_id, content_type, total_size, chunk_size)
            .await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "ChunkedUpload", e))
    }

    async fn upload_chunk(
        &self,
        upload_id: &str,
        chunk_index: usize,
        data: bytes::Bytes,
        checksum: Option<String>,
    ) -> Result<ChunkUploadResponseDto, DomainError> {
        self.upload_chunk_inner(upload_id, chunk_index, data, checksum)
            .await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "ChunkedUpload", e))
    }

    async fn get_status(&self, upload_id: &str) -> Result<UploadStatusResponseDto, DomainError> {
        self.get_status_inner(upload_id)
            .await
            .map_err(|e| DomainError::new(ErrorKind::NotFound, "ChunkedUpload", e))
    }

    async fn complete_upload(
        &self,
        upload_id: &str,
    ) -> Result<(PathBuf, String, Option<String>, String, u64, String), DomainError> {
        self.complete_upload_inner(upload_id)
            .await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "ChunkedUpload", e))
    }

    async fn finalize_upload(&self, upload_id: &str) -> Result<(), DomainError> {
        self.finalize_upload_inner(upload_id)
            .await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "ChunkedUpload", e))
    }

    async fn cancel_upload(&self, upload_id: &str) -> Result<(), DomainError> {
        self.cancel_upload_inner(upload_id)
            .await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "ChunkedUpload", e))
    }

    fn should_use_chunked(&self, size: u64) -> bool {
        ChunkedUploadService::should_use_chunked(size)
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_count_calculation() {
        assert_eq!(
            UploadSession::calculate_chunk_count(10 * 1024 * 1024, 5 * 1024 * 1024),
            2
        );
        assert_eq!(
            UploadSession::calculate_chunk_count(11 * 1024 * 1024, 5 * 1024 * 1024),
            3
        );
        assert_eq!(UploadSession::calculate_chunk_count(1, 5 * 1024 * 1024), 1);
        assert_eq!(UploadSession::calculate_chunk_count(0, 5 * 1024 * 1024), 1);
    }

    #[test]
    fn test_should_use_chunked() {
        assert!(!ChunkedUploadService::should_use_chunked(9 * 1024 * 1024));
        assert!(ChunkedUploadService::should_use_chunked(10 * 1024 * 1024));
        assert!(ChunkedUploadService::should_use_chunked(100 * 1024 * 1024));
    }

    #[test]
    fn test_bitmask_roundtrip() {
        let now = Utc::now();
        let mut session = UploadSession {
            id: "test-id".into(),
            filename: "file.bin".into(),
            folder_id: None,
            content_type: "application/octet-stream".into(),
            total_size: 15 * 1024 * 1024,
            chunk_size: 5 * 1024 * 1024,
            chunks: (0..3)
                .map(|i| ChunkInfo {
                    index: i,
                    offset: i as u64 * 5 * 1024 * 1024,
                    size: 5 * 1024 * 1024,
                    status: ChunkStatus::Pending,
                    checksum: None,
                })
                .collect(),
            created_at: now,
            last_activity: now,
            temp_dir: PathBuf::from("/tmp/test-id"),
            bytes_received: 0,
        };

        // Mark chunks 0 and 2 as complete
        session.chunks[0].status = ChunkStatus::Complete;
        session.chunks[2].status = ChunkStatus::Complete;
        session.bytes_received = 10 * 1024 * 1024;

        let bitmask = session.build_progress_bitmask();
        assert_eq!(bitmask, vec![0b00000101]); // bits 0 and 2 set

        // Reset and re-apply
        for chunk in &mut session.chunks {
            chunk.status = ChunkStatus::Pending;
        }
        session.bytes_received = 0;
        session.apply_progress_bitmask(&bitmask);

        assert_eq!(session.chunks[0].status, ChunkStatus::Complete);
        assert_eq!(session.chunks[1].status, ChunkStatus::Pending);
        assert_eq!(session.chunks[2].status, ChunkStatus::Complete);
        assert_eq!(session.bytes_received, 10 * 1024 * 1024);
    }

    #[test]
    fn test_session_serialisation_roundtrip() {
        let now = Utc::now();
        let session = UploadSession {
            id: "abc-123".into(),
            filename: "photo.jpg".into(),
            folder_id: Some("folder-1".into()),
            content_type: "image/jpeg".into(),
            total_size: 1024,
            chunk_size: 512,
            chunks: vec![
                ChunkInfo {
                    index: 0,
                    offset: 0,
                    size: 512,
                    status: ChunkStatus::Complete,
                    checksum: Some("aabb".into()),
                },
                ChunkInfo {
                    index: 1,
                    offset: 512,
                    size: 512,
                    status: ChunkStatus::Pending,
                    checksum: None,
                },
            ],
            created_at: now,
            last_activity: now,
            temp_dir: PathBuf::from("/tmp/abc-123"),
            bytes_received: 512,
        };

        let json = serde_json::to_vec(&session).expect("serialise");
        let restored: UploadSession = serde_json::from_slice(&json).expect("deserialise");

        assert_eq!(restored.id, session.id);
        assert_eq!(restored.filename, session.filename);
        assert_eq!(restored.folder_id, session.folder_id);
        assert_eq!(restored.total_size, session.total_size);
        assert_eq!(restored.chunks.len(), 2);
        assert_eq!(restored.chunks[0].status, ChunkStatus::Complete);
        assert_eq!(restored.chunks[1].status, ChunkStatus::Pending);
        assert_eq!(restored.bytes_received, 512);
    }

    #[test]
    fn test_session_expiry_check() {
        let mut session = UploadSession {
            id: "exp-test".into(),
            filename: "f".into(),
            folder_id: None,
            content_type: "x".into(),
            total_size: 0,
            chunk_size: 1,
            chunks: vec![],
            created_at: Utc::now(),
            last_activity: Utc::now(),
            temp_dir: PathBuf::from("/tmp"),
            bytes_received: 0,
        };

        assert!(!session.is_expired(), "Fresh session must not be expired");

        // Simulate old activity
        session.last_activity = Utc::now() - chrono::Duration::hours(25);
        assert!(session.is_expired(), "25h-old session must be expired");
    }

    #[tokio::test]
    async fn test_persist_and_recover_session() {
        // Use a unique temp dir for this test
        let base = std::env::temp_dir().join(format!("oxicloud_test_{}", Uuid::new_v4()));
        let _ = fs::create_dir_all(&base).await;

        let service = ChunkedUploadService::new(base.clone()).await;

        // Create a session
        let resp = service
            .create_session_inner(
                "bigfile.bin".into(),
                Some("folder-x".into()),
                "application/octet-stream".into(),
                10 * 1024 * 1024,
                Some(5 * 1024 * 1024),
            )
            .await
            .expect("create_session");

        let upload_id = resp.upload_id.clone();

        // Upload first chunk (5 MB of zeros)
        let chunk_data = bytes::Bytes::from(vec![0u8; 5 * 1024 * 1024]);
        service
            .upload_chunk_inner(&upload_id, 0, chunk_data, None)
            .await
            .expect("upload_chunk 0");

        // Verify files exist on disk
        let session_dir = base.join(&upload_id);
        assert!(session_dir.join(SESSION_META_FILE).exists());
        assert!(session_dir.join(PROGRESS_FILE).exists());
        assert!(session_dir.join("chunk_000000").exists());

        // Simulate restart: drop service, recover from disk
        drop(service);

        let recovered = ChunkedUploadService::recover_sessions(&base).await;
        assert_eq!(recovered.len(), 1);
        let session = recovered
            .get(&upload_id)
            .expect("session must be recovered");
        assert_eq!(session.filename, "bigfile.bin");
        assert_eq!(session.folder_id, Some("folder-x".into()));
        assert_eq!(session.chunks[0].status, ChunkStatus::Complete);
        assert_eq!(session.chunks[1].status, ChunkStatus::Pending);
        assert_eq!(session.bytes_received, 5 * 1024 * 1024);

        // Cleanup
        let _ = fs::remove_dir_all(&base).await;
    }

    #[tokio::test]
    async fn test_full_upload_lifecycle() {
        let base = std::env::temp_dir().join(format!("oxicloud_test_{}", Uuid::new_v4()));
        let service = ChunkedUploadService::new(base.clone()).await;

        // 1. Create session (1024 bytes, 512 byte chunks → 2 chunks)
        let resp = service
            .create_session_inner(
                "test.txt".into(),
                None,
                "text/plain".into(),
                1024,
                Some(512),
            )
            .await
            .expect("create");

        assert_eq!(resp.total_chunks, 2);
        assert_eq!(resp.chunk_size, 512);
        let id = resp.upload_id;

        // 2. Upload chunks
        let chunk0 = bytes::Bytes::from(vec![b'A'; 512]);
        let r0 = service
            .upload_chunk_inner(&id, 0, chunk0, None)
            .await
            .expect("chunk 0");
        assert!(!r0.is_complete);

        let chunk1 = bytes::Bytes::from(vec![b'B'; 512]);
        let r1 = service
            .upload_chunk_inner(&id, 1, chunk1, None)
            .await
            .expect("chunk 1");
        assert!(r1.is_complete);
        assert_eq!(r1.bytes_received, 1024);

        // 3. Status check
        let status = service.get_status_inner(&id).await.expect("status");
        assert!(status.is_complete);
        assert_eq!(status.completed_chunks, 2);
        assert!(status.pending_chunks.is_empty());

        // 4. Complete (assemble)
        let (path, filename, _folder, _ct, size, hash) =
            service.complete_upload_inner(&id).await.expect("complete");
        assert_eq!(filename, "test.txt");
        assert_eq!(size, 1024);
        assert!(!hash.is_empty());
        assert!(path.exists());

        // 5. Verify assembled content
        let content = fs::read(&path).await.expect("read assembled");
        assert_eq!(&content[..512], &[b'A'; 512]);
        assert_eq!(&content[512..], &[b'B'; 512]);

        // 6. Finalize
        service.finalize_upload_inner(&id).await.expect("finalize");
        assert_eq!(service.active_sessions().await, 0);

        let _ = fs::remove_dir_all(&base).await;
    }

    #[tokio::test]
    async fn test_cancel_removes_files() {
        let base = std::env::temp_dir().join(format!("oxicloud_test_{}", Uuid::new_v4()));
        let service = ChunkedUploadService::new(base.clone()).await;

        let resp = service
            .create_session_inner(
                "x.bin".into(),
                None,
                "application/octet-stream".into(),
                512,
                Some(512),
            )
            .await
            .expect("create");

        let session_dir = base.join(&resp.upload_id);
        assert!(session_dir.exists());

        service
            .cancel_upload_inner(&resp.upload_id)
            .await
            .expect("cancel");

        assert!(!session_dir.exists(), "temp dir must be removed on cancel");
        assert_eq!(service.active_sessions().await, 0);

        let _ = fs::remove_dir_all(&base).await;
    }

    #[tokio::test]
    async fn test_recovery_skips_expired_sessions() {
        let base = std::env::temp_dir().join(format!("oxicloud_test_{}", Uuid::new_v4()));
        let _ = fs::create_dir_all(&base).await;

        // Manually create an expired session on disk
        let session_dir = base.join("expired-session");
        let _ = fs::create_dir_all(&session_dir).await;

        let expired_session = UploadSession {
            id: "expired-session".into(),
            filename: "old.bin".into(),
            folder_id: None,
            content_type: "application/octet-stream".into(),
            total_size: 1024,
            chunk_size: 1024,
            chunks: vec![ChunkInfo {
                index: 0,
                offset: 0,
                size: 1024,
                status: ChunkStatus::Pending,
                checksum: None,
            }],
            created_at: Utc::now() - chrono::Duration::hours(48),
            last_activity: Utc::now() - chrono::Duration::hours(48),
            temp_dir: session_dir.clone(),
            bytes_received: 0,
        };

        let json = serde_json::to_vec(&expired_session).unwrap();
        fs::write(session_dir.join(SESSION_META_FILE), &json)
            .await
            .unwrap();

        let recovered = ChunkedUploadService::recover_sessions(&base).await;
        assert!(
            recovered.is_empty(),
            "Expired sessions must not be recovered"
        );

        let _ = fs::remove_dir_all(&base).await;
    }

    #[tokio::test]
    async fn test_recovery_downgrades_missing_chunk_files() {
        let base = std::env::temp_dir().join(format!("oxicloud_test_{}", Uuid::new_v4()));
        let _ = fs::create_dir_all(&base).await;

        let session_dir = base.join("partial-session");
        let _ = fs::create_dir_all(&session_dir).await;

        let session = UploadSession {
            id: "partial-session".into(),
            filename: "file.bin".into(),
            folder_id: None,
            content_type: "application/octet-stream".into(),
            total_size: 1024,
            chunk_size: 512,
            chunks: vec![
                ChunkInfo {
                    index: 0,
                    offset: 0,
                    size: 512,
                    status: ChunkStatus::Pending,
                    checksum: None,
                },
                ChunkInfo {
                    index: 1,
                    offset: 512,
                    size: 512,
                    status: ChunkStatus::Pending,
                    checksum: None,
                },
            ],
            created_at: Utc::now(),
            last_activity: Utc::now(),
            temp_dir: session_dir.clone(),
            bytes_received: 0,
        };

        // Write metadata
        let json = serde_json::to_vec(&session).unwrap();
        fs::write(session_dir.join(SESSION_META_FILE), &json)
            .await
            .unwrap();

        // Write progress marking both chunks complete
        let bitmask = vec![0b00000011u8]; // bits 0 and 1
        fs::write(session_dir.join(PROGRESS_FILE), &bitmask)
            .await
            .unwrap();

        // But only create chunk_000000 on disk — chunk_000001 is "missing"
        fs::write(session_dir.join("chunk_000000"), &[0u8; 512])
            .await
            .unwrap();

        let recovered = ChunkedUploadService::recover_sessions(&base).await;
        let s = recovered.get("partial-session").expect("must be recovered");

        assert_eq!(s.chunks[0].status, ChunkStatus::Complete);
        assert_eq!(
            s.chunks[1].status,
            ChunkStatus::Pending,
            "Missing chunk file must be downgraded to Pending"
        );
        assert_eq!(s.bytes_received, 512);

        let _ = fs::remove_dir_all(&base).await;
    }
}
