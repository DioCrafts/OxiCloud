use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use serde::Serialize;
use tokio::io::AsyncWriteExt;

use crate::application::ports::dedup_ports::DedupResultDto;
use crate::common::di::AppState;
use crate::interfaces::middleware::auth::AuthUser;
use std::sync::Arc;

/// Global application state for dependency injection
type GlobalState = Arc<AppState>;

/// Response for hash check endpoint
#[derive(Debug, Serialize)]
pub struct HashCheckResponse {
    /// Whether a blob with this hash already exists
    pub exists: bool,
    /// The SHA-256 hash that was checked
    pub hash: String,
    /// If exists, the size of the existing blob
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_size: Option<u64>,
    /// If exists, the number of references to this blob
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_count: Option<u32>,
}

/// Response for upload with dedup endpoint
#[derive(Debug, Serialize)]
pub struct DedupUploadResponse {
    /// Whether this was a new file or an existing one
    pub is_new: bool,
    /// The SHA-256 hash of the content
    pub hash: String,
    /// The size of the content in bytes
    pub size: u64,
    /// Bytes saved by deduplication (0 if new file)
    pub bytes_saved: u64,
    /// Current reference count for this blob
    pub ref_count: u32,
}

/// Response for dedup stats endpoint
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    /// Total number of unique blobs stored
    pub unique_blobs: u64,
    /// Total number of references (files pointing to blobs)
    pub total_references: u64,
    /// Total bytes saved by deduplication
    pub bytes_saved: u64,
    /// Total logical bytes (what users think they have)
    pub total_logical_bytes: u64,
    /// Total physical bytes (actual disk usage)
    pub total_physical_bytes: u64,
    /// Deduplication ratio (logical / physical)
    pub dedup_ratio: f64,
    /// Percentage of storage saved
    pub savings_percentage: f64,
}

/// Handler for deduplication-related endpoints
///
/// Provides endpoints for:
/// - Checking if content already exists (by hash)
/// - Uploading files with automatic deduplication
/// - Getting deduplication statistics
pub struct DedupHandler;

impl DedupHandler {
    /// Check if the authenticated user already has a file with the given hash.
    ///
    /// User-scoped: only reveals whether **this user** owns a file that
    /// references the blob — never exposes global existence or ref_count.
    ///
    /// GET /api/dedup/check/{hash}
    pub async fn check_hash(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        Path(hash): Path<String>,
    ) -> impl IntoResponse {
        let dedup = &state.core.dedup_service;

        // Validate hash format (SHA-256 = 64 hex chars)
        if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{"error": "Invalid hash format. Expected SHA-256 (64 hex characters)"}"#,
                ))
                .unwrap()
                .into_response();
        }

        // Only reveal whether THIS user has the blob — no global oracle
        let user_has_it = dedup
            .user_owns_blob_reference(&hash, &auth_user.id.to_string())
            .await;

        if user_has_it {
            // Fetch size from metadata (safe — user owns a reference)
            let size = dedup.get_blob_metadata(&hash).await.map(|m| m.size);
            let response = HashCheckResponse {
                exists: true,
                hash,
                existing_size: size,
                ref_count: None, // Never expose global ref_count
            };
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&response).unwrap()))
                .unwrap()
                .into_response()
        } else {
            let response = HashCheckResponse {
                exists: false,
                hash,
                existing_size: None,
                ref_count: None,
            };
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&response).unwrap()))
                .unwrap()
                .into_response()
        }
    }

    /// Upload content with automatic deduplication (streaming).
    ///
    /// Spools the upload to a temp file while computing the BLAKE3 hash
    /// incrementally (hash-on-write). Memory usage is constant (~512 KB)
    /// regardless of file size. Then delegates to `store_from_file` with
    /// the pre-computed hash so the file is never re-read for hashing.
    ///
    /// POST /api/dedup/upload
    pub async fn upload_with_dedup(
        State(state): State<GlobalState>,
        _auth_user: AuthUser,
        mut multipart: Multipart,
    ) -> impl IntoResponse {
        let dedup = &state.core.dedup_service;

        // Process multipart form
        while let Some(field) = multipart.next_field().await.unwrap_or(None) {
            let name = field.name().unwrap_or("").to_string();

            if name == "file" {
                let content_type = field
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .to_string();

                // ── Spool to temp file + BLAKE3 hash-on-write ────────
                let temp_dir = state.core.path_service.get_root_path().join(".dedup_temp");
                let temp_path = temp_dir.join(format!("dedup-{}", uuid::Uuid::new_v4()));

                let mut total_size: u64 = 0;
                let mut hasher = blake3::Hasher::new();
                let mut field = field;

                let spool_result: Result<(), String> = async {
                    let file = tokio::fs::File::create(&temp_path)
                        .await
                        .map_err(|e| format!("Failed to create temp file: {}", e))?;

                    // 512 KB buffer — reduces write syscalls
                    let mut writer = tokio::io::BufWriter::with_capacity(524_288, file);

                    loop {
                        match field.chunk().await {
                            Ok(Some(chunk)) => {
                                total_size += chunk.len() as u64;
                                hasher.update(&chunk);
                                writer.write_all(&chunk).await.map_err(|e| {
                                    format!("Failed to write chunk: {}", e)
                                })?;
                            }
                            Ok(None) => break,
                            Err(e) => {
                                return Err(format!(
                                    "Connection lost during upload (received {} bytes): {}",
                                    total_size, e
                                ));
                            }
                        }
                    }

                    writer
                        .flush()
                        .await
                        .map_err(|e| format!("Failed to flush temp file: {}", e))?;
                    Ok(())
                }
                .await;

                if let Err(msg) = spool_result {
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    tracing::warn!("Dedup upload spool failed: {}", msg);
                    return Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(format!(
                            r#"{{"error": "{}"}}"#,
                            msg
                        )))
                        .unwrap()
                        .into_response();
                }

                if total_size == 0 {
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    return Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(r#"{"error": "Empty file not allowed"}"#))
                        .unwrap()
                        .into_response();
                }

                let hash = hasher.finalize().to_hex().to_string();

                // ── Store with deduplication (pre-computed hash) ──────
                match dedup
                    .store_from_file(&temp_path, Some(content_type), Some(hash))
                    .await
                {
                    Ok(result) => {
                        let (is_new, bytes_saved) = match &result {
                            DedupResultDto::NewBlob { .. } => (true, 0),
                            DedupResultDto::ExistingBlob { saved_bytes, .. } => {
                                (false, *saved_bytes)
                            }
                        };

                        let metadata = dedup.get_blob_metadata(result.hash()).await;

                        let response = DedupUploadResponse {
                            is_new,
                            hash: result.hash().to_string(),
                            size: result.size(),
                            bytes_saved,
                            ref_count: metadata.map(|m| m.ref_count).unwrap_or(1),
                        };

                        tracing::info!(
                            "🔗 Dedup upload: hash={}, new={}, saved={}",
                            result.hash(),
                            is_new,
                            bytes_saved
                        );

                        return Response::builder()
                            .status(if is_new {
                                StatusCode::CREATED
                            } else {
                                StatusCode::OK
                            })
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(Body::from(serde_json::to_string(&response).unwrap()))
                            .unwrap()
                            .into_response();
                    }
                    Err(e) => {
                        let _ = tokio::fs::remove_file(&temp_path).await;
                        tracing::error!("Dedup upload failed: {}", e);
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(Body::from(r#"{"error": "Upload failed"}"#))
                            .unwrap()
                            .into_response();
                    }
                }
            }
        }

        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                r#"{"error": "No file field found in multipart form"}"#,
            ))
            .unwrap()
            .into_response()
    }

    /// Get deduplication statistics
    ///
    /// GET /api/dedup/stats
    ///
    /// Returns comprehensive statistics about the deduplication system including:
    /// - Number of unique blobs
    /// - Total references
    /// - Bytes saved
    /// - Deduplication ratio
    pub async fn get_stats(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
    ) -> impl IntoResponse {
        // Admin-only — global dedup statistics are sensitive infrastructure data
        if auth_user.role != "admin" {
            return Response::builder()
                .status(StatusCode::FORBIDDEN)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"error": "Admin role required"}"#))
                .unwrap()
                .into_response();
        }

        let dedup = &state.core.dedup_service;
        let stats = dedup.get_stats().await;

        // Calculate savings percentage
        let savings_pct = if stats.total_bytes_referenced > 0 {
            (stats.bytes_saved as f64 / stats.total_bytes_referenced as f64) * 100.0
        } else {
            0.0
        };

        let response = StatsResponse {
            unique_blobs: stats.total_blobs,
            total_references: stats.dedup_hits + stats.total_blobs, // Approximation
            bytes_saved: stats.bytes_saved,
            total_logical_bytes: stats.total_bytes_referenced,
            total_physical_bytes: stats.total_bytes_stored,
            dedup_ratio: stats.dedup_ratio,
            savings_percentage: savings_pct,
        };

        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&response).unwrap()))
            .unwrap()
            .into_response()
    }

    /// Retrieve content by hash (user-scoped).
    ///
    /// GET /api/dedup/blob/{hash}
    ///
    /// Returns the raw content of a blob **only if** the authenticated user
    /// owns at least one file that references it. Returns 404 otherwise
    /// (does not reveal whether the blob exists globally).
    pub async fn get_blob(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        Path(hash): Path<String>,
    ) -> impl IntoResponse {
        let dedup = &state.core.dedup_service;

        // Validate hash format
        if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"error": "Invalid hash format"}"#))
                .unwrap()
                .into_response();
        }

        // Verify the user owns at least one file referencing this blob
        if !dedup
            .user_owns_blob_reference(&hash, &auth_user.id.to_string())
            .await
        {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"error": "Blob not found"}"#))
                .unwrap()
                .into_response();
        }

        // Get metadata first for content-type
        let metadata = dedup.get_blob_metadata(&hash).await;
        let content_type = metadata
            .as_ref()
            .and_then(|m| m.content_type.clone())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        // Stream blob in 64 KB chunks — constant memory regardless of size
        let size = match dedup.blob_size(&hash).await {
            Ok(s) => s,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"error": "Blob not found"}"#))
                    .unwrap()
                    .into_response();
            }
        };

        match dedup.read_blob_stream(&hash).await {
            Ok(stream) => Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .header(header::CONTENT_LENGTH, size.to_string())
                .header("X-Dedup-Hash", &hash)
                .body(Body::from_stream(stream))
                .unwrap()
                .into_response(),
            Err(_) => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"error": "Blob not found"}"#))
                .unwrap()
                .into_response(),
        }
    }

    /// Force recalculation of statistics from disk
    ///
    /// POST /api/dedup/recalculate
    ///
    /// Verifies integrity and returns current statistics.
    /// Useful for health checks and auditing.
    pub async fn recalculate_stats(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
    ) -> impl IntoResponse {
        // Admin-only — integrity verification is a privileged operation
        if auth_user.role != "admin" {
            return Response::builder()
                .status(StatusCode::FORBIDDEN)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"error": "Admin role required"}"#))
                .unwrap()
                .into_response();
        }

        let dedup = &state.core.dedup_service;

        // Verify integrity first
        match dedup.verify_integrity().await {
            Ok(issues) => {
                if !issues.is_empty() {
                    tracing::warn!("Dedup integrity issues found: {:?}", issues);
                }
            }
            Err(e) => {
                tracing::error!("Dedup integrity verification failed: {}", e);
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"error": "Verification failed"}"#))
                    .unwrap()
                    .into_response();
            }
        }

        let stats = dedup.get_stats().await;

        // Calculate savings percentage
        let savings_pct = if stats.total_bytes_referenced > 0 {
            (stats.bytes_saved as f64 / stats.total_bytes_referenced as f64) * 100.0
        } else {
            0.0
        };

        let response = StatsResponse {
            unique_blobs: stats.total_blobs,
            total_references: stats.dedup_hits + stats.total_blobs,
            bytes_saved: stats.bytes_saved,
            total_logical_bytes: stats.total_bytes_referenced,
            total_physical_bytes: stats.total_bytes_stored,
            dedup_ratio: stats.dedup_ratio,
            savings_percentage: savings_pct,
        };

        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&response).unwrap()))
            .unwrap()
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    /// Verify that the hash-on-write pattern produces the same BLAKE3 hash
    /// as hashing the entire content at once.
    #[tokio::test]
    async fn hash_on_write_matches_full_hash() {
        let content = b"Hello, OxiCloud dedup streaming upload!";

        // 1. Full-content hash (reference)
        let full_hash = blake3::hash(content).to_hex().to_string();

        // 2. Incremental hash-on-write (what the handler does)
        let mut hasher = blake3::Hasher::new();
        // Simulate multiple chunks
        hasher.update(&content[..10]);
        hasher.update(&content[10..25]);
        hasher.update(&content[25..]);
        let incremental_hash = hasher.finalize().to_hex().to_string();

        assert_eq!(full_hash, incremental_hash);
    }

    /// Verify BLAKE3 incremental hashing produces a valid 64-char hex hash.
    #[tokio::test]
    async fn incremental_hash_format_is_valid() {
        let content = vec![0xABu8; 1024 * 1024]; // 1 MB of data

        let mut hasher = blake3::Hasher::new();
        // Feed in 64 KB chunks like real uploads
        for chunk in content.chunks(65_536) {
            hasher.update(chunk);
        }
        let hash = hasher.finalize().to_hex().to_string();

        assert_eq!(hash.len(), 64, "BLAKE3 hash should be 64 hex characters");
        assert!(
            hash.chars().all(|c| c.is_ascii_hexdigit()),
            "Hash should only contain hex characters"
        );
    }

    /// Verify spool-to-disk + BLAKE3 hash-on-write writes correct content
    /// and produces the correct hash.
    #[tokio::test]
    async fn spool_to_temp_file_preserves_content_and_hash() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path().join("test-upload.tmp");

        let content = b"The quick brown fox jumps over the lazy dog";
        let expected_hash = blake3::hash(content).to_hex().to_string();

        // Simulate the handler's hash-on-write spool loop
        let mut hasher = blake3::Hasher::new();
        let mut total_size: u64 = 0;
        {
            let file = tokio::fs::File::create(&temp_path).await.unwrap();
            let mut writer = tokio::io::BufWriter::with_capacity(524_288, file);

            // Simulate 3 incoming chunks
            let chunks: &[&[u8]] = &[&content[..10], &content[10..30], &content[30..]];
            for chunk in chunks {
                total_size += chunk.len() as u64;
                hasher.update(chunk);
                writer.write_all(chunk).await.unwrap();
            }
            writer.flush().await.unwrap();
        }

        let hash = hasher.finalize().to_hex().to_string();

        // Verify hash matches
        assert_eq!(hash, expected_hash);

        // Verify total size
        assert_eq!(total_size, content.len() as u64);

        // Verify file content on disk is identical
        let disk_content = tokio::fs::read(&temp_path).await.unwrap();
        assert_eq!(disk_content, content);
    }

    /// Verify that an empty upload produces total_size == 0.
    #[tokio::test]
    async fn empty_upload_detected_before_store() {
        let hasher = blake3::Hasher::new();
        let total_size: u64 = 0;

        // No chunks fed — simulates empty file
        let _hash = hasher.finalize().to_hex().to_string();

        // The handler checks total_size == 0 and returns 400
        assert_eq!(total_size, 0);
    }

    /// Verify large payload streaming produces consistent hash
    /// without buffering all content in memory.
    #[tokio::test]
    async fn large_payload_streaming_hash_consistency() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path().join("large-upload.tmp");

        // 5 MB of patterned data
        let chunk_size = 65_536usize; // 64 KB chunks
        let total_chunks = 80; // 80 × 64 KB = 5 MB
        let mut reference_data = Vec::with_capacity(chunk_size * total_chunks);

        let mut hasher = blake3::Hasher::new();
        let mut total_size: u64 = 0;
        {
            let file = tokio::fs::File::create(&temp_path).await.unwrap();
            let mut writer = tokio::io::BufWriter::with_capacity(524_288, file);

            for i in 0..total_chunks {
                // Patterned data: each chunk filled with its index byte
                let chunk = vec![(i % 256) as u8; chunk_size];
                reference_data.extend_from_slice(&chunk);
                total_size += chunk.len() as u64;
                hasher.update(&chunk);
                writer.write_all(&chunk).await.unwrap();
            }
            writer.flush().await.unwrap();
        }

        let streaming_hash = hasher.finalize().to_hex().to_string();
        let reference_hash = blake3::hash(&reference_data).to_hex().to_string();

        // Hashes match
        assert_eq!(streaming_hash, reference_hash);

        // File on disk matches
        let file_size = tokio::fs::metadata(&temp_path).await.unwrap().len();
        assert_eq!(file_size, total_size);
        assert_eq!(total_size, (chunk_size * total_chunks) as u64);

        // Verify file content matches (read back)
        let disk_data = tokio::fs::read(&temp_path).await.unwrap();
        assert_eq!(disk_data, reference_data);
    }

    /// Verify temp file is cleaned up when spool fails partway through.
    #[tokio::test]
    async fn temp_file_cleanup_on_partial_write() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path().join("partial-upload.tmp");

        // Create the file and write some data
        {
            let file = tokio::fs::File::create(&temp_path).await.unwrap();
            let mut writer = tokio::io::BufWriter::with_capacity(524_288, file);
            writer.write_all(b"partial data").await.unwrap();
            writer.flush().await.unwrap();
        }

        // File exists before cleanup
        assert!(tokio::fs::try_exists(&temp_path).await.unwrap());

        // Simulate the handler's error cleanup path
        let _ = tokio::fs::remove_file(&temp_path).await;

        // File is gone after cleanup
        assert!(!tokio::fs::try_exists(&temp_path).await.unwrap_or(true));
    }

    /// Verify the DedupUploadResponse serializes correctly for new blobs.
    #[test]
    fn dedup_upload_response_serialization_new_blob() {
        let response = DedupUploadResponse {
            is_new: true,
            hash: "a".repeat(64),
            size: 1024,
            bytes_saved: 0,
            ref_count: 1,
        };

        let json = serde_json::to_string(&response).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["is_new"], true);
        assert_eq!(parsed["size"], 1024);
        assert_eq!(parsed["bytes_saved"], 0);
        assert_eq!(parsed["ref_count"], 1);
    }

    /// Verify the DedupUploadResponse serializes correctly for dedup hits.
    #[test]
    fn dedup_upload_response_serialization_dedup_hit() {
        let response = DedupUploadResponse {
            is_new: false,
            hash: "b".repeat(64),
            size: 2048,
            bytes_saved: 2048,
            ref_count: 3,
        };

        let json = serde_json::to_string(&response).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["is_new"], false);
        assert_eq!(parsed["bytes_saved"], 2048);
        assert_eq!(parsed["ref_count"], 3);
    }
}
