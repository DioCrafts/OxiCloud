//! Chunked Upload Handler - TUS-like Protocol Endpoints
//!
//! Provides HTTP endpoints for resumable, parallel chunk uploads:
//! - POST   /api/uploads          → Create upload session
//! - PATCH  /api/uploads/:id      → Upload a chunk
//! - HEAD   /api/uploads/:id      → Get upload status
//! - POST   /api/uploads/:id/complete → Assemble and finalize
//! - DELETE /api/uploads/:id      → Cancel upload

use axum::{
    extract::{Path, State, Query},
    http::{StatusCode, header, HeaderMap},
    response::{IntoResponse, Response},
    Json,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::di::AppState;
use crate::infrastructure::services::chunked_upload_service::DEFAULT_CHUNK_SIZE;

/// Request body for creating an upload session
#[derive(Debug, Deserialize)]
pub struct CreateUploadRequest {
    pub filename: String,
    pub folder_id: Option<String>,
    pub content_type: Option<String>,
    pub total_size: u64,
    pub chunk_size: Option<usize>,
}

/// Query params for chunk upload
#[derive(Debug, Deserialize)]
pub struct ChunkUploadParams {
    pub chunk_index: usize,
    pub checksum: Option<String>,
}

/// Final response after completing upload
#[derive(Debug, Serialize)]
pub struct CompleteUploadResponse {
    pub file_id: String,
    pub filename: String,
    pub size: u64,
    pub path: String,
}

/// Chunked Upload Handler
pub struct ChunkedUploadHandler;

impl ChunkedUploadHandler {
    /// POST /api/uploads - Create a new upload session
    /// 
    /// Request body:
    /// ```json
    /// {
    ///   "filename": "large-video.mp4",
    ///   "folder_id": "optional-folder-id",
    ///   "content_type": "video/mp4",
    ///   "total_size": 104857600,
    ///   "chunk_size": 5242880
    /// }
    /// ```
    /// 
    /// Response:
    /// ```json
    /// {
    ///   "upload_id": "uuid",
    ///   "chunk_size": 5242880,
    ///   "total_chunks": 20,
    ///   "expires_at": 86400
    /// }
    /// ```
    pub async fn create_upload(
        State(state): State<Arc<AppState>>,
        Json(request): Json<CreateUploadRequest>,
    ) -> impl IntoResponse {
        let chunked_service = &state.core.chunked_upload_service;
        
        // Validate request
        if request.filename.is_empty() {
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Filename is required"
            }))).into_response();
        }
        
        if request.total_size == 0 {
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Total size must be greater than 0"
            }))).into_response();
        }
        
        // Validate chunk size if provided
        let chunk_size = request.chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE);
        if chunk_size < 1024 * 1024 {
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Chunk size must be at least 1MB"
            }))).into_response();
        }
        
        let content_type = request.content_type
            .unwrap_or_else(|| "application/octet-stream".to_string());
        
        match chunked_service.create_session(
            request.filename,
            request.folder_id,
            content_type,
            request.total_size,
            Some(chunk_size),
        ).await {
            Ok(response) => {
                (StatusCode::CREATED, Json(response)).into_response()
            }
            Err(e) => {
                tracing::error!("Failed to create upload session: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "error": e
                }))).into_response()
            }
        }
    }
    
    /// PATCH /api/uploads/:upload_id - Upload a chunk
    /// 
    /// Query params:
    /// - chunk_index: The index of the chunk (0-based)
    /// - checksum: Optional MD5 checksum for verification
    /// 
    /// Body: Raw bytes of the chunk
    pub async fn upload_chunk(
        State(state): State<Arc<AppState>>,
        Path(upload_id): Path<String>,
        Query(params): Query<ChunkUploadParams>,
        headers: HeaderMap,
        body: Bytes,
    ) -> impl IntoResponse {
        let chunked_service = &state.core.chunked_upload_service;
        
        // Extract checksum from header or query param
        let checksum = params.checksum.or_else(|| {
            headers.get("Content-MD5")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        });
        
        match chunked_service.upload_chunk(
            &upload_id,
            params.chunk_index,
            body,
            checksum,
        ).await {
            Ok(response) => {
                let mut resp = Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("Upload-Offset", response.bytes_received.to_string())
                    .header("Upload-Progress", format!("{:.2}", response.progress * 100.0));
                
                if response.is_complete {
                    resp = resp.header("Upload-Complete", "true");
                }
                
                resp.body(axum::body::Body::from(serde_json::to_string(&response).unwrap()))
                    .unwrap()
                    .into_response()
            }
            Err(e) => {
                let status = if e.contains("not found") {
                    StatusCode::NOT_FOUND
                } else if e.contains("Invalid") || e.contains("already uploaded") {
                    StatusCode::BAD_REQUEST
                } else if e.contains("Checksum") {
                    StatusCode::CONFLICT
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                };
                
                (status, Json(serde_json::json!({
                    "error": e
                }))).into_response()
            }
        }
    }
    
    /// HEAD /api/uploads/:upload_id - Get upload status
    /// 
    /// Returns upload progress and pending chunks
    pub async fn get_upload_status(
        State(state): State<Arc<AppState>>,
        Path(upload_id): Path<String>,
    ) -> impl IntoResponse {
        let chunked_service = &state.core.chunked_upload_service;
        
        match chunked_service.get_status(&upload_id).await {
            Ok(status) => {
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("Upload-Offset", status.bytes_received.to_string())
                    .header("Upload-Length", status.total_size.to_string())
                    .header("Upload-Progress", format!("{:.2}", status.progress * 100.0))
                    .header("Upload-Chunks-Total", status.total_chunks.to_string())
                    .header("Upload-Chunks-Complete", status.completed_chunks.to_string())
                    .body(axum::body::Body::from(serde_json::to_string(&status).unwrap()))
                    .unwrap()
                    .into_response()
            }
            Err(e) => {
                (StatusCode::NOT_FOUND, Json(serde_json::json!({
                    "error": e
                }))).into_response()
            }
        }
    }
    
    /// POST /api/uploads/:upload_id/complete - Finalize upload
    /// 
    /// Assembles all chunks into the final file and creates the file record
    pub async fn complete_upload(
        State(state): State<Arc<AppState>>,
        Path(upload_id): Path<String>,
    ) -> impl IntoResponse {
        let chunked_service = &state.core.chunked_upload_service;
        let file_service = &state.applications.file_service_concrete;
        
        // Assemble chunks
        let (assembled_path, filename, folder_id, content_type, total_size) = 
            match chunked_service.complete_upload(&upload_id).await {
                Ok(result) => result,
                Err(e) => {
                    let status = if e.contains("not found") {
                        StatusCode::NOT_FOUND
                    } else if e.contains("not complete") {
                        StatusCode::CONFLICT
                    } else {
                        StatusCode::INTERNAL_SERVER_ERROR
                    };
                    
                    return (status, Json(serde_json::json!({
                        "error": e
                    }))).into_response();
                }
            };
        
        // Read assembled file and create final file record
        let file_data = match tokio::fs::read(&assembled_path).await {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to read assembled file: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "error": format!("Failed to read assembled file: {}", e)
                }))).into_response();
            }
        };
        
        // Upload via normal service (this handles path resolution, metadata, etc.)
        match file_service.upload_file_from_bytes(
            filename.clone(),
            folder_id.clone(),
            content_type,
            file_data,
        ).await {
            Ok(file) => {
                // Cleanup session
                let _ = chunked_service.finalize_upload(&upload_id).await;
                
                tracing::info!(
                    "✅ CHUNKED UPLOAD COMPLETE: {} (ID: {}, {} bytes)",
                    filename, file.id, total_size
                );
                
                (StatusCode::CREATED, Json(CompleteUploadResponse {
                    file_id: file.id,
                    filename: file.name,
                    size: total_size,
                    path: file.path,
                })).into_response()
            }
            Err(e) => {
                tracing::error!("Failed to create file from assembled upload: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "error": format!("Failed to create file: {:?}", e)
                }))).into_response()
            }
        }
    }
    
    /// DELETE /api/uploads/:upload_id - Cancel upload
    /// 
    /// Cancels an in-progress upload and cleans up temp files
    pub async fn cancel_upload(
        State(state): State<Arc<AppState>>,
        Path(upload_id): Path<String>,
    ) -> impl IntoResponse {
        let chunked_service = &state.core.chunked_upload_service;
        
        match chunked_service.cancel_upload(&upload_id).await {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "error": e
                }))).into_response()
            }
        }
    }
}
