use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use bytes::Bytes;
use serde::Serialize;

use crate::application::ports::dedup_ports::DedupResultDto;
use crate::common::di::AppState;
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
    /// Check if a blob with the given hash already exists
    ///
    /// This endpoint allows clients to check if uploading a file is necessary
    /// by pre-computing the hash client-side and checking against the server.
    ///
    /// GET /api/dedup/check/{hash}
    pub async fn check_hash(
        State(state): State<GlobalState>,
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

        match dedup.get_blob_metadata(&hash).await {
            Some(metadata) => {
                let response = HashCheckResponse {
                    exists: true,
                    hash,
                    existing_size: Some(metadata.size),
                    ref_count: Some(metadata.ref_count),
                };
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(serde_json::to_string(&response).unwrap()))
                    .unwrap()
                    .into_response()
            }
            None => {
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
    }

    /// Upload content with automatic deduplication
    ///
    /// This endpoint calculates the SHA-256 hash of the uploaded content
    /// and either creates a new blob or increments the reference count
    /// of an existing blob.
    ///
    /// POST /api/dedup/upload
    ///
    /// Returns information about whether the content was new or deduplicated.
    pub async fn upload_with_dedup(
        State(state): State<GlobalState>,
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

                // Collect all chunks
                let mut chunks: Vec<Bytes> = Vec::new();
                let mut total_size: usize = 0;
                let mut field = field;

                while let Ok(Some(chunk)) = field.chunk().await {
                    total_size += chunk.len();
                    chunks.push(chunk);
                }

                if chunks.is_empty() {
                    return Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(r#"{"error": "Empty file not allowed"}"#))
                        .unwrap()
                        .into_response();
                }

                // Combine chunks
                let data: Vec<u8> = if chunks.len() == 1 {
                    chunks.into_iter().next().unwrap().to_vec()
                } else {
                    let mut combined = Vec::with_capacity(total_size);
                    for chunk in chunks {
                        combined.extend_from_slice(&chunk);
                    }
                    combined
                };

                // Store with deduplication
                match dedup.store_bytes(&data, Some(content_type)).await {
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
                        tracing::error!("❌ Dedup upload failed: {}", e);
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(Body::from(format!(
                                r#"{{"error": "Upload failed: {}"}}"#,
                                e
                            )))
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
    pub async fn get_stats(State(state): State<GlobalState>) -> impl IntoResponse {
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

    /// Retrieve content by hash
    ///
    /// GET /api/dedup/blob/{hash}
    ///
    /// Returns the raw content of a blob identified by its SHA-256 hash.
    /// Useful for retrieving deduplicated content.
    pub async fn get_blob(
        State(state): State<GlobalState>,
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

    /// Remove a reference to a blob
    ///
    /// DELETE /api/dedup/blob/{hash}
    ///
    /// Decrements the reference count for a blob. If the reference count
    /// reaches zero, the blob is deleted from storage.
    pub async fn remove_reference(
        State(state): State<GlobalState>,
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

        match dedup.remove_reference(&hash).await {
            Ok(deleted) => {
                let message = if deleted {
                    format!(
                        r#"{{"success": true, "deleted": true, "message": "Blob {} was deleted (ref_count reached 0)"}}"#,
                        hash
                    )
                } else {
                    format!(
                        r#"{{"success": true, "deleted": false, "message": "Reference removed from blob {}"}}"#,
                        hash
                    )
                };

                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(message))
                    .unwrap()
                    .into_response()
            }
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(format!(r#"{{"error": "{}"}}"#, e)))
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
    pub async fn recalculate_stats(State(state): State<GlobalState>) -> impl IntoResponse {
        let dedup = &state.core.dedup_service;

        // Verify integrity first
        match dedup.verify_integrity().await {
            Ok(issues) => {
                if !issues.is_empty() {
                    tracing::warn!("Dedup integrity issues found: {:?}", issues);
                }
            }
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(format!(
                        r#"{{"error": "Verification failed: {}"}}"#,
                        e
                    )))
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
