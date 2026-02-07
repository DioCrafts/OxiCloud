use std::sync::Arc;
use axum::{
    extract::{Path, State, Multipart, Query},
    http::{StatusCode, header, HeaderMap, Response},
    response::IntoResponse,
    body::Body,
    Json,
};
use bytes::Bytes;
use serde::Deserialize;
use std::collections::HashMap;
use http_range_header::parse_range_header;

use crate::application::services::file_service::{FileService, FileServiceError};
use crate::infrastructure::services::compression_service::{
    CompressionService, GzipCompressionService, CompressionLevel
};
use crate::common::di::AppState;
use crate::interfaces::middleware::auth::CurrentUserId;

/**
 * Type aliases for dependency injection state.
 * These aliases improve code readability when working with service dependencies.
 */
/// State containing the file service for dependency injection
type FileServiceState = Arc<FileService>;
/// Global application state for dependency injection
type GlobalState = AppState;

/**
 * API handler for file-related operations.
 * 
 * The FileHandler is responsible for processing HTTP requests related to file operations.
 * It handles:
 * 
 * 1. File uploads through multipart form data
 * 2. File downloads with optional compression
 * 3. Listing files in folders
 * 4. Moving files between folders
 * 5. Deleting files (with trash integration)
 * 
 * This component acts as an adapter in the hexagonal architecture, translating
 * between HTTP requests/responses and application service calls. It handles
 * HTTP-specific concerns like status codes, headers, and request parsing while
 * delegating business logic to the application services.
 */
pub struct FileHandler;

/// Threshold for using streaming upload (files >= 1MB use streaming)
const STREAMING_UPLOAD_THRESHOLD: usize = 1 * 1024 * 1024;

/// Threshold for write-behind cache (files < 256KB get instant response)
const WRITE_BEHIND_THRESHOLD: usize = 256 * 1024;

impl FileHandler {
    /// Uploads a file with TRUE STREAMING support and Write-Behind Cache
    /// 
    /// Three-tier upload strategy:
    /// 
    /// 1. INSTANT (<256KB): Write-behind cache
    ///    - Store in RAM, respond immediately
    ///    - Flush to disk asynchronously
    ///    - User perceives ~0ms latency
    /// 
    /// 2. BUFFERED (256KB - 1MB): In-memory processing
    ///    - Fast for medium files
    ///    - Direct write to disk before response
    /// 
    /// 3. STREAMING (≥1MB): Direct disk writes
    ///    - Constant memory regardless of file size
    ///    - Uses atomic rename for crash safety
    pub async fn upload_file(
        State(service): State<FileServiceState>,
        mut multipart: Multipart,
    ) -> impl IntoResponse {
        use futures::stream;
        use std::pin::Pin;
        
        let mut folder_id: Option<String> = None;
        
        tracing::debug!("📤 Processing file upload request");
        
        while let Some(field) = multipart.next_field().await.unwrap_or(None) {
            let name = field.name().unwrap_or("").to_string();
            
            if name == "folder_id" {
                let folder_id_value = field.text().await.unwrap_or_default();
                if !folder_id_value.is_empty() {
                    folder_id = Some(folder_id_value);
                }
                continue;
            }
            
            if name == "file" {
                let filename = field.file_name().unwrap_or("unnamed").to_string();
                let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
                
                tracing::info!("📤 UPLOAD START: {} (folder: {:?})", filename, folder_id);
                
                // Collect all chunks from the field - we need to consume the field completely
                // to avoid borrow issues with multipart
                let mut chunks: Vec<Bytes> = Vec::new();
                let mut total_size: usize = 0;
                let mut field = field;
                
                while let Ok(Some(chunk)) = field.chunk().await {
                    total_size += chunk.len();
                    chunks.push(chunk);
                    
                    // Log progress every 10MB
                    if total_size > 0 && total_size % (10 * 1024 * 1024) < chunks.last().map(|c| c.len()).unwrap_or(0) {
                        tracing::debug!(
                            "📥 Upload receiving: {} - {}MB", 
                            filename, 
                            total_size / (1024 * 1024)
                        );
                    }
                }
                
                // Empty file check
                if chunks.is_empty() {
                    return Self::upload_empty_file(service, filename, folder_id, content_type).await;
                }
                
                // Decide upload strategy based on total size
                if total_size >= STREAMING_UPLOAD_THRESHOLD {
                    // ═══════════════════════════════════════════════════════════════
                    // STREAMING UPLOAD - For large files
                    // Create a stream from collected chunks and write to disk
                    // ═══════════════════════════════════════════════════════════════
                    tracing::info!(
                        "📡 STREAMING UPLOAD: {} ({} MB, {} chunks)", 
                        filename, 
                        total_size / (1024 * 1024),
                        chunks.len()
                    );
                    
                    // Convert chunks to a stream
                    let chunk_stream = stream::iter(
                        chunks.into_iter().map(|c| Ok::<_, std::io::Error>(c))
                    );
                    let pinned_stream: Pin<Box<dyn futures::Stream<Item = Result<Bytes, std::io::Error>> + Send>> = 
                        Box::pin(chunk_stream);
                    
                    // Use streaming upload - writes directly to disk
                    match service.upload_file_from_stream(
                        filename.clone(),
                        folder_id.clone(),
                        content_type.clone(),
                        pinned_stream,
                    ).await {
                        Ok(file) => {
                            tracing::info!(
                                "✅ STREAMING UPLOAD COMPLETE: {} ({} MB, ID: {})", 
                                filename, 
                                total_size / (1024 * 1024),
                                file.id
                            );
                            return Response::builder()
                                .status(StatusCode::CREATED)
                                .header(header::CONTENT_TYPE, "application/json")
                                .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
                                .body(axum::body::Body::from(serde_json::to_string(&file).unwrap()))
                                .unwrap()
                                .into_response();
                        },
                        Err(err) => {
                            tracing::error!("❌ STREAMING UPLOAD FAILED: {} - {}", filename, err);
                            return Self::error_response(err);
                        }
                    }
                } else {
                    // ═══════════════════════════════════════════════════════════════
                    // BUFFERED UPLOAD - For small files (<1MB)
                    // Faster for small files as we avoid temp file overhead
                    // ═══════════════════════════════════════════════════════════════
                    tracing::debug!("💨 BUFFERED UPLOAD: {} ({} bytes)", filename, total_size);
                    
                    // Combine chunks efficiently
                    let data = if chunks.len() == 1 {
                        chunks.into_iter().next().unwrap().to_vec()
                    } else {
                        let mut combined = Vec::with_capacity(total_size);
                        for chunk in chunks {
                            combined.extend_from_slice(&chunk);
                        }
                        combined
                    };
                    
                    match service.upload_file_from_bytes(
                        filename.clone(),
                        folder_id.clone(),
                        content_type.clone(),
                        data,
                    ).await {
                        Ok(file) => {
                            tracing::info!("✅ BUFFERED UPLOAD COMPLETE: {} (ID: {})", filename, file.id);
                            return Response::builder()
                                .status(StatusCode::CREATED)
                                .header(header::CONTENT_TYPE, "application/json")
                                .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
                                .body(axum::body::Body::from(serde_json::to_string(&file).unwrap()))
                                .unwrap()
                                .into_response();
                        },
                        Err(err) => {
                            tracing::error!("❌ BUFFERED UPLOAD FAILED: {} - {}", filename, err);
                            return Self::error_response(err);
                        }
                    }
                }
            }
        }
        
        tracing::warn!("Upload request missing file field");
        (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "No file provided"
        }))).into_response()
    }
    
    /// Handle empty file uploads
    async fn upload_empty_file(
        service: Arc<FileService>,
        filename: String,
        folder_id: Option<String>,
        content_type: String,
    ) -> Response<axum::body::Body> {
        tracing::debug!("Uploading empty file: {}", filename);
        match service.upload_file_from_bytes(filename.clone(), folder_id, content_type, vec![]).await {
            Ok(file) => {
                Response::builder()
                    .status(StatusCode::CREATED)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(serde_json::to_string(&file).unwrap()))
                    .unwrap()
            },
            Err(err) => Self::error_response(err),
        }
    }
    
    /// Uploads a file with Write-Behind Cache for instant response
    /// 
    /// This version uses AppState to access the write-behind cache,
    /// enabling ~0ms perceived latency for small files (<256KB).
    /// 
    /// Flow for small files:
    /// 1. Generate file ID and metadata
    /// 2. Store content in write-behind cache (+ dedup blob store)
    /// 3. Respond immediately with 201 Created
    /// 4. Background worker flushes to disk
    /// 
    /// Deduplication:
    /// - All uploads are stored in the dedup blob store
    /// - Duplicate content is detected by SHA-256 hash
    /// - Only one copy of identical content is stored
    pub async fn upload_file_with_cache(
        State(state): State<GlobalState>,
        mut multipart: Multipart,
    ) -> impl IntoResponse {
        use futures::stream;
        use std::pin::Pin;
        use crate::infrastructure::services::write_behind_cache::WriteBehindCache;
        
        let service = &state.applications.file_service_concrete;
        let write_behind = &state.core.write_behind_cache;
        let dedup_service = &state.core.dedup_service;
        
        let mut folder_id: Option<String> = None;
        
        tracing::debug!("📤 Processing file upload request (with write-behind cache + dedup)");
        
        while let Some(field) = multipart.next_field().await.unwrap_or(None) {
            let name = field.name().unwrap_or("").to_string();
            
            if name == "folder_id" {
                let folder_id_value = field.text().await.unwrap_or_default();
                if !folder_id_value.is_empty() {
                    folder_id = Some(folder_id_value);
                }
                continue;
            }
            
            if name == "file" {
                let filename = field.file_name().unwrap_or("unnamed").to_string();
                let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
                
                // Collect chunks
                let mut chunks: Vec<Bytes> = Vec::new();
                let mut total_size: usize = 0;
                let mut field = field;
                
                while let Ok(Some(chunk)) = field.chunk().await {
                    total_size += chunk.len();
                    chunks.push(chunk);
                }
                
                // Empty file - handle separately
                if chunks.is_empty() {
                    return Self::upload_empty_file(service.clone(), filename, folder_id, content_type).await.into_response();
                }
                
                // ═══════════════════════════════════════════════════════════════
                // DEDUPLICATION: Store content in blob store for dedup tracking
                // This runs in parallel with normal upload to track duplicates
                // ═══════════════════════════════════════════════════════════════
                let dedup_data: Vec<u8> = {
                    let mut combined = Vec::with_capacity(total_size);
                    for chunk in &chunks {
                        combined.extend_from_slice(chunk);
                    }
                    combined
                };
                
                // Store in dedup blob store (async, non-blocking for response)
                let dedup_result = dedup_service.store_bytes(&dedup_data, Some(content_type.clone())).await;
                match &dedup_result {
                    Ok(result) => {
                        if result.was_deduplicated() {
                            tracing::info!(
                                "🔗 DEDUP: {} - content already exists (hash: {}, saved {} bytes)", 
                                filename,
                                &result.hash()[..12],
                                result.size()
                            );
                        } else {
                            tracing::info!(
                                "💾 DEDUP: {} - new content stored (hash: {})", 
                                filename,
                                &result.hash()[..12]
                            );
                        }
                    },
                    Err(e) => {
                        // Dedup failure is not fatal - continue with normal upload
                        tracing::warn!("⚠️ DEDUP: Failed to store in blob store: {}", e);
                    }
                }
                
                tracing::info!(
                    "📤 UPLOAD: {} ({} bytes, folder: {:?}, strategy: {})", 
                    filename, 
                    total_size,
                    folder_id,
                    if total_size < WRITE_BEHIND_THRESHOLD { "WRITE-BEHIND" }
                    else if total_size < STREAMING_UPLOAD_THRESHOLD { "BUFFERED" }
                    else { "STREAMING" }
                );
                
                // ═══════════════════════════════════════════════════════════════
                // TIER 1: WRITE-BEHIND PURO (<256KB) - Zero latency upload
                // 1. Register metadata (~0.1ms)
                // 2. Cache content in RAM
                // 3. Respond 201 IMMEDIATELY
                // 4. Background: flush to disk
                // ═══════════════════════════════════════════════════════════════
                if total_size < WRITE_BEHIND_THRESHOLD && WriteBehindCache::is_eligible(total_size) {
                    // Combine chunks
                    let data: Bytes = if chunks.len() == 1 {
                        chunks.into_iter().next().unwrap()
                    } else {
                        let mut combined = Vec::with_capacity(total_size);
                        for chunk in chunks {
                            combined.extend_from_slice(&chunk);
                        }
                        combined.into()
                    };
                    
                    // Register file metadata WITHOUT writing to disk
                    match service.register_file_deferred(
                        filename.clone(),
                        folder_id.clone(),
                        content_type.clone(),
                        total_size as u64,
                    ).await {
                        Ok((file, target_path)) => {
                            // Put content in write-behind cache for:
                            // 1. Immediate reads (before flush completes)
                            // 2. Background flush to disk
                            if let Err(e) = write_behind.put_pending(
                                file.id.clone(),
                                data,
                                target_path,
                            ).await {
                                tracing::error!("❌ Write-behind cache failed: {} - {}", file.id, e);
                                // Fallback: return error (file exists in metadata but not on disk)
                                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                                    "error": format!("Failed to queue file write: {}", e)
                                }))).into_response();
                            }
                            
                            tracing::info!("⚡ WRITE-BEHIND UPLOAD: {} (ID: {}, ~0ms latency)", filename, file.id);
                            return Response::builder()
                                .status(StatusCode::CREATED)
                                .header(header::CONTENT_TYPE, "application/json")
                                .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
                                .body(axum::body::Body::from(serde_json::to_string(&file).unwrap()))
                                .unwrap()
                                .into_response();
                        },
                        Err(err) => {
                            tracing::error!("❌ WRITE-BEHIND REGISTRATION FAILED: {} - {}", filename, err);
                            return Self::error_response(err).into_response();
                        }
                    }
                }
                
                // ═══════════════════════════════════════════════════════════════
                // TIER 2: STREAMING UPLOAD (≥1MB) - Direct disk writes
                // ═══════════════════════════════════════════════════════════════
                if total_size >= STREAMING_UPLOAD_THRESHOLD {
                    let chunk_stream = stream::iter(
                        chunks.into_iter().map(|c| Ok::<_, std::io::Error>(c))
                    );
                    let pinned_stream: Pin<Box<dyn futures::Stream<Item = Result<Bytes, std::io::Error>> + Send>> = 
                        Box::pin(chunk_stream);
                    
                    match service.upload_file_from_stream(
                        filename.clone(),
                        folder_id.clone(),
                        content_type.clone(),
                        pinned_stream,
                    ).await {
                        Ok(file) => {
                            tracing::info!(
                                "✅ STREAMING UPLOAD COMPLETE: {} ({} MB, ID: {})", 
                                filename, 
                                total_size / (1024 * 1024),
                                file.id
                            );
                            return Response::builder()
                                .status(StatusCode::CREATED)
                                .header(header::CONTENT_TYPE, "application/json")
                                .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
                                .body(axum::body::Body::from(serde_json::to_string(&file).unwrap()))
                                .unwrap()
                                .into_response();
                        },
                        Err(err) => {
                            tracing::error!("❌ STREAMING UPLOAD FAILED: {} - {}", filename, err);
                            return Self::error_response(err).into_response();
                        }
                    }
                }
                
                // ═══════════════════════════════════════════════════════════════
                // TIER 3: BUFFERED UPLOAD (256KB - 1MB)
                // ═══════════════════════════════════════════════════════════════
                let data = if chunks.len() == 1 {
                    chunks.into_iter().next().unwrap().to_vec()
                } else {
                    let mut combined = Vec::with_capacity(total_size);
                    for chunk in chunks {
                        combined.extend_from_slice(&chunk);
                    }
                    combined
                };
                
                match service.upload_file_from_bytes(
                    filename.clone(),
                    folder_id.clone(),
                    content_type.clone(),
                    data,
                ).await {
                    Ok(file) => {
                        tracing::info!("✅ BUFFERED UPLOAD COMPLETE: {} (ID: {})", filename, file.id);
                        return Response::builder()
                            .status(StatusCode::CREATED)
                            .header(header::CONTENT_TYPE, "application/json")
                            .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
                            .body(axum::body::Body::from(serde_json::to_string(&file).unwrap()))
                            .unwrap()
                            .into_response();
                    },
                    Err(err) => {
                        tracing::error!("❌ BUFFERED UPLOAD FAILED: {} - {}", filename, err);
                        return Self::error_response(err).into_response();
                    }
                }
            }
        }
        
        (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "No file provided"
        }))).into_response()
    }

    /// Build error response for upload failures
    fn error_response(err: FileServiceError) -> Response<axum::body::Body> {
        let status = match &err {
            FileServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            FileServiceError::AccessError(_) => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        Response::builder()
            .status(status)
            .header(header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(serde_json::json!({
                "error": format!("Error uploading file: {}", err)
            }).to_string()))
            .unwrap()
    }
    
    /// Get a thumbnail for an image file
    /// 
    /// Supports three sizes:
    /// - icon: 150x150 (for file listings)
    /// - preview: 400x400 (for gallery view)
    /// - large: 800x800 (for detail view)
    /// 
    /// Thumbnails are:
    /// - Generated on-demand if not cached
    /// - Stored as WebP for smaller size
    /// - Cached in memory for fast repeated access
    pub async fn get_thumbnail(
        State(state): State<GlobalState>,
        Path((id, size)): Path<(String, String)>,
    ) -> impl IntoResponse {
        use crate::infrastructure::services::thumbnail_service::{ThumbnailService, ThumbnailSize};
        
        let service = &state.applications.file_service_concrete;
        let thumbnail_service = &state.core.thumbnail_service;
        
        // Parse size parameter
        let thumb_size = match size.as_str() {
            "icon" => ThumbnailSize::Icon,
            "preview" => ThumbnailSize::Preview,
            "large" => ThumbnailSize::Large,
            _ => {
                return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Invalid thumbnail size. Use: icon, preview, or large"
                }))).into_response();
            }
        };
        
        // Get file info
        let file = match service.get_file(&id).await {
            Ok(f) => f,
            Err(err) => {
                return (StatusCode::NOT_FOUND, Json(serde_json::json!({
                    "error": format!("File not found: {}", err)
                }))).into_response();
            }
        };
        
        // Check if file is an image
        if !ThumbnailService::is_supported_image(&file.mime_type) {
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "File is not a supported image type"
            }))).into_response();
        }
        
        // Build the file path from storage root + relative path
        let storage_root = state.core.path_service.get_root_path();
        let file_path = storage_root.join(&file.path);
        
        // Get or generate thumbnail
        match thumbnail_service.get_thumbnail(&id, thumb_size, &file_path).await {
            Ok(data) => {
                // Generate ETag for caching
                let etag = format!("\"thumb-{}-{:?}\"", id, thumb_size);
                
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "image/webp")
                    .header(header::CONTENT_LENGTH, data.len())
                    .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable")
                    .header(header::ETAG, etag)
                    .body(Body::from(data))
                    .unwrap()
                    .into_response()
            },
            Err(err) => {
                tracing::error!("Thumbnail generation failed: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "error": format!("Failed to generate thumbnail: {}", err)
                }))).into_response()
            }
        }
    }
    
    /// Downloads a file with optimized 3-tier caching strategy
    /// 
    /// Architecture for maximum performance:
    /// 1. HOT CACHE (RAM): LRU cache for files <10MB - latency ~0.1ms
    /// 2. STREAMING: Direct file streaming for files ≥10MB - no RAM overhead
    /// 
    /// Also supports:
    /// - ETag-based caching with 304 Not Modified responses
    /// - Optional compression (disabled for streaming to preserve speed)
    /// - Automatic WebP transcoding for images (30-50% smaller)
    pub async fn download_file(
        State(state): State<GlobalState>,
        Path(id): Path<String>,
        Query(params): Query<HashMap<String, String>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let service = &state.applications.file_service_concrete;
        let content_cache = &state.core.file_content_cache;
        
        // Get file info first to check it exists and get metadata
        let file = match service.get_file(&id).await {
            Ok(f) => f,
            Err(err) => {
                let status = match &err {
                    FileServiceError::NotFound(_) => StatusCode::NOT_FOUND,
                    FileServiceError::AccessError(_) => StatusCode::SERVICE_UNAVAILABLE,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                return (status, Json(serde_json::json!({
                    "error": err.to_string()
                }))).into_response();
            }
        };
        
        // Generate ETag based on file ID and modification time
        let etag = format!("\"{}-{}\"", id, file.modified_at);
        
        // Check If-None-Match header for ETag validation (304 Not Modified)
        if let Some(if_none_match) = headers.get(header::IF_NONE_MATCH) {
            if let Ok(client_etag) = if_none_match.to_str() {
                if client_etag == etag || client_etag == "*" {
                    tracing::debug!("ETag match for file {}, returning 304", file.name);
                    return Response::builder()
                        .status(StatusCode::NOT_MODIFIED)
                        .header(header::ETAG, &etag)
                        .body(Body::empty())
                        .unwrap()
                        .into_response();
                }
            }
        }
        
        // ═══════════════════════════════════════════════════════════════════════
        // RANGE REQUESTS - For video seeking and resumable downloads
        // ═══════════════════════════════════════════════════════════════════════
        if let Some(range_header) = headers.get(header::RANGE) {
            if let Ok(range_str) = range_header.to_str() {
                if let Ok(ranges) = parse_range_header(range_str) {
                    // Validate and get the first range (we only support single ranges)
                    let validated = ranges.validate(file.size);
                    
                    if let Ok(valid_ranges) = validated {
                        if let Some(range) = valid_ranges.first() {
                            let start = *range.start();
                            let end = *range.end();
                            let range_length = end - start + 1;
                            
                            tracing::info!(
                                "📡 RANGE REQUEST: {} bytes {}-{}/{}", 
                                file.name, start, end, file.size
                            );
                            
                            // Determine content disposition for range request
                            let force_inline = params.get("inline").map_or(false, |v| v == "true" || v == "1");
                            let disposition = if force_inline || 
                                             file.mime_type.starts_with("image/") || 
                                             file.mime_type == "application/pdf" ||
                                             file.mime_type.starts_with("video/") ||
                                             file.mime_type.starts_with("audio/") {
                                format!("inline; filename=\"{}\"", file.name)
                            } else {
                                format!("attachment; filename=\"{}\"", file.name)
                            };
                            
                            // Use range streaming
                            match service.get_file_range_stream(&id, start, Some(end + 1)).await {
                                Ok(stream) => {
                                    let pinned_stream = Box::into_pin(stream);
                                    
                                    return Response::builder()
                                        .status(StatusCode::PARTIAL_CONTENT)
                                        .header(header::CONTENT_TYPE, &file.mime_type)
                                        .header(header::CONTENT_DISPOSITION, &disposition)
                                        .header(header::CONTENT_LENGTH, range_length)
                                        .header(header::CONTENT_RANGE, format!("bytes {}-{}/{}", start, end, file.size))
                                        .header(header::ACCEPT_RANGES, "bytes")
                                        .header(header::ETAG, &etag)
                                        .header(header::CACHE_CONTROL, "private, max-age=3600, must-revalidate")
                                        .body(Body::from_stream(pinned_stream))
                                        .unwrap()
                                        .into_response();
                                },
                                Err(err) => {
                                    tracing::error!("Error creating range stream: {}", err);
                                    // Fall through to normal download on error
                                }
                            }
                        }
                    } else {
                        // Range not satisfiable
                        tracing::warn!("Range not satisfiable: {} for file size {}", range_str, file.size);
                        return Response::builder()
                            .status(StatusCode::RANGE_NOT_SATISFIABLE)
                            .header(header::CONTENT_RANGE, format!("bytes */{}", file.size))
                            .body(Body::empty())
                            .unwrap()
                            .into_response();
                    }
                }
            }
        }
        
        // Determine content disposition
        let force_inline = params.get("inline").map_or(false, |v| v == "true" || v == "1");
        let disposition = if force_inline || 
                         file.mime_type.starts_with("image/") || 
                         file.mime_type == "application/pdf" ||
                         file.mime_type.starts_with("video/") ||
                         file.mime_type.starts_with("audio/") {
            format!("inline; filename=\"{}\"", file.name)
        } else {
            format!("attachment; filename=\"{}\"", file.name)
        };
        
        // File size threshold for streaming (10MB)
        const CACHE_THRESHOLD: u64 = 10 * 1024 * 1024;
        // Threshold for mmap vs streaming (100MB)
        const MMAP_THRESHOLD: u64 = 100 * 1024 * 1024;
        
        // ═══════════════════════════════════════════════════════════════════════
        // TIER 0: WRITE-BEHIND CACHE - For recently uploaded small files
        // Serves content directly from RAM if file was just uploaded
        // ═══════════════════════════════════════════════════════════════════════
        let write_behind = &state.core.write_behind_cache;
        if let Some(pending_content) = write_behind.get_pending(&id).await {
            tracing::debug!("⚡ TIER 0 Write-Behind HIT: {} ({} bytes)", file.name, pending_content.len());
            
            return Self::build_cached_response(
                pending_content,
                &file.mime_type,
                &disposition,
                &etag,
                file.size,
                &params,
            ).await;
        }
        
        // ═══════════════════════════════════════════════════════════════════════
        // TIER 1: HOT CACHE - For small files (<10MB)
        // With automatic WebP transcoding for images (30-50% smaller)
        // ═══════════════════════════════════════════════════════════════════════
        if file.size < CACHE_THRESHOLD {
            use crate::infrastructure::services::image_transcode_service::{
                ImageTranscodeService, BrowserCapabilities
            };
            
            // Check if browser supports WebP and image is transcodable
            let accept_header = headers.get(header::ACCEPT)
                .and_then(|v| v.to_str().ok());
            let browser_caps = BrowserCapabilities::from_accept_header(accept_header);
            let should_transcode = browser_caps.supports_webp 
                && ImageTranscodeService::should_transcode(&file.mime_type, file.size)
                && params.get("original").map_or(true, |v| v != "true" && v != "1");
            
            // Check cache first
            if let Some((cached_content, _cached_etag, _cached_content_type)) = content_cache.get(&id).await {
                tracing::debug!("🔥 TIER 1 Cache HIT: {} ({} bytes)", file.name, cached_content.len());
                
                // Try WebP transcoding for cached content
                if should_transcode {
                    let transcode_service = &state.core.image_transcode_service;
                    if let Some(format) = browser_caps.best_format() {
                        match transcode_service.get_transcoded(
                            &id,
                            &cached_content,
                            &file.mime_type,
                            format,
                        ).await {
                            Ok((transcoded, webp_mime, was_transcoded)) => {
                                if was_transcoded {
                                    tracing::debug!("🖼️ WebP transcode: {} -> {} bytes ({:.0}% smaller)", 
                                        cached_content.len(), transcoded.len(),
                                        (1.0 - transcoded.len() as f64 / cached_content.len() as f64) * 100.0
                                    );
                                    return Self::build_cached_response(
                                        transcoded,
                                        &webp_mime,
                                        &disposition,
                                        &etag,
                                        file.size,
                                        &params,
                                    ).await;
                                }
                            },
                            Err(e) => {
                                tracing::debug!("WebP transcode failed, serving original: {}", e);
                            }
                        }
                    }
                }
                
                return Self::build_cached_response(
                    cached_content,
                    &file.mime_type,
                    &disposition,
                    &etag,
                    file.size,
                    &params,
                ).await;
            }
            
            // Cache miss - load from disk and cache
            tracing::debug!("💾 TIER 1 Cache MISS: {} - loading from disk", file.name);
            
            match service.get_file_content(&id).await {
                Ok(content) => {
                    let content_bytes = Bytes::from(content);
                    
                    // Store in cache for next time
                    content_cache.put(
                        id.clone(),
                        content_bytes.clone(),
                        etag.clone(),
                        file.mime_type.clone()
                    ).await;
                    
                    // Try WebP transcoding
                    if should_transcode {
                        let transcode_service = &state.core.image_transcode_service;
                        if let Some(format) = browser_caps.best_format() {
                            match transcode_service.get_transcoded(
                                &id,
                                &content_bytes,
                                &file.mime_type,
                                format,
                            ).await {
                                Ok((transcoded, webp_mime, was_transcoded)) => {
                                    if was_transcoded {
                                        tracing::info!("🖼️ WebP transcode: {} {} -> {} bytes ({:.0}% smaller)", 
                                            file.name, content_bytes.len(), transcoded.len(),
                                            (1.0 - transcoded.len() as f64 / content_bytes.len() as f64) * 100.0
                                        );
                                        return Self::build_cached_response(
                                            transcoded,
                                            &webp_mime,
                                            &disposition,
                                            &etag,
                                            file.size,
                                            &params,
                                        ).await;
                                    }
                                },
                                Err(e) => {
                                    tracing::debug!("WebP transcode failed, serving original: {}", e);
                                }
                            }
                        }
                    }
                    
                    return Self::build_cached_response(
                        content_bytes,
                        &file.mime_type,
                        &disposition,
                        &etag,
                        file.size,
                        &params,
                    ).await;
                },
                Err(err) => {
                    tracing::error!("Error reading file content: {}", err);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                        "error": format!("Error reading file: {}", err)
                    }))).into_response();
                }
            }
        }
        
        // ═══════════════════════════════════════════════════════════════════════
        // TIER 2: MMAP - For medium files (10-100MB)
        // Zero-copy kernel memory mapping for optimal performance
        // ═══════════════════════════════════════════════════════════════════════
        if file.size < MMAP_THRESHOLD {
            tracing::info!("🗺️ TIER 2 MMAP: {} ({} MB)", file.name, file.size / (1024 * 1024));
            
            match service.get_file_mmap(&id).await {
                Ok(mmap_content) => {
                    return Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, &file.mime_type)
                        .header(header::CONTENT_DISPOSITION, &disposition)
                        .header(header::CONTENT_LENGTH, file.size)
                        .header(header::ETAG, &etag)
                        .header(header::CACHE_CONTROL, "private, max-age=3600, must-revalidate")
                        .header(header::ACCEPT_RANGES, "bytes")
                        .body(Body::from(mmap_content))
                        .unwrap()
                        .into_response();
                },
                Err(err) => {
                    tracing::warn!("MMAP failed, falling back to streaming: {}", err);
                    // Fall through to streaming
                }
            }
        }
        
        // ═══════════════════════════════════════════════════════════════════════
        // TIER 3: STREAMING - For very large files (≥100MB)
        // Chunk-based streaming directly from disk to network
        // ═══════════════════════════════════════════════════════════════════════
        tracing::info!("📡 TIER 3 STREAMING: {} ({} MB)", file.name, file.size / (1024 * 1024));
        
        match service.get_file_stream(&id).await {
            Ok(stream) => {
                // The stream is already Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>
                // axum's Body::from_stream accepts streams that yield Result<impl Into<Bytes>, Error>
                // We need to pin the boxed stream for use with Body::from_stream
                let pinned_stream = Box::into_pin(stream);
                
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, &file.mime_type)
                    .header(header::CONTENT_DISPOSITION, &disposition)
                    .header(header::CONTENT_LENGTH, file.size)
                    .header(header::ETAG, &etag)
                    .header(header::CACHE_CONTROL, "private, max-age=3600, must-revalidate")
                    .header(header::ACCEPT_RANGES, "bytes")
                    .body(Body::from_stream(pinned_stream))
                    .unwrap()
                    .into_response()
            },
            Err(err) => {
                tracing::error!("Error creating file stream: {}", err);
                
                // Fallback to regular content loading if streaming fails
                tracing::warn!("Falling back to content-based download for: {}", file.name);
                
                match service.get_file_content(&id).await {
                    Ok(content) => {
                        Response::builder()
                            .status(StatusCode::OK)
                            .header(header::CONTENT_TYPE, &file.mime_type)
                            .header(header::CONTENT_DISPOSITION, &disposition)
                            .header(header::CONTENT_LENGTH, content.len())
                            .header(header::ETAG, &etag)
                            .body(Body::from(content))
                            .unwrap()
                            .into_response()
                    },
                    Err(content_err) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                            "error": format!("Error reading file: {}", content_err)
                        }))).into_response()
                    }
                }
            }
        }
    }
    
    /// Build response for cached/small files with optional compression
    async fn build_cached_response(
        content: Bytes,
        mime_type: &str,
        disposition: &str,
        etag: &str,
        file_size: u64,
        params: &HashMap<String, String>,
    ) -> Response<Body> {
        // Check if compression is requested
        let compression_param = params.get("compress").map(|v| v.as_str());
        let force_compress = compression_param == Some("true") || compression_param == Some("1");
        let force_no_compress = compression_param == Some("false") || compression_param == Some("0");
        
        let compression_service = GzipCompressionService::new();
        let should_compress = if force_no_compress {
            false
        } else if force_compress {
            true
        } else {
            compression_service.should_compress(mime_type, file_size)
        };
        
        let compression_level = match params.get("compression_level").map(|v| v.as_str()) {
            Some("fast") => CompressionLevel::Fast,
            Some("best") => CompressionLevel::Best,
            _ => CompressionLevel::Default,
        };
        
        let builder = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_DISPOSITION, disposition)
            .header(header::ETAG, etag)
            .header(header::CACHE_CONTROL, "private, max-age=3600, must-revalidate")
            .header(header::VARY, "Accept-Encoding");
        
        if should_compress {
            match compression_service.compress_data(&content.to_vec(), compression_level).await {
                Ok(compressed) => {
                    tracing::debug!(
                        "Compressed {}KB → {}KB ({:.1}x)", 
                        content.len() / 1024, 
                        compressed.len() / 1024,
                        content.len() as f64 / compressed.len().max(1) as f64
                    );
                    
                    builder
                        .header(header::CONTENT_TYPE, mime_type)
                        .header(header::CONTENT_ENCODING, "gzip")
                        .header(header::CONTENT_LENGTH, compressed.len())
                        .body(Body::from(compressed))
                        .unwrap()
                },
                Err(_) => {
                    builder
                        .header(header::CONTENT_TYPE, mime_type)
                        .header(header::CONTENT_LENGTH, content.len())
                        .body(Body::from(content))
                        .unwrap()
                }
            }
        } else {
            builder
                .header(header::CONTENT_TYPE, mime_type)
                .header(header::CONTENT_LENGTH, content.len())
                .body(Body::from(content))
                .unwrap()
        }
    }
    
    /// Lists files, optionally filtered by folder ID
    pub async fn list_files(
        State(service): State<FileServiceState>,
        folder_id: Option<&str>,
    ) -> impl IntoResponse {
        tracing::info!("Listing files with folder_id: {:?}", folder_id);
        
        // Simply use the file service to list files
        match service.list_files(folder_id).await {
            Ok(files) => {
                // Log success for debugging purposes
                tracing::info!("Found {} files through the service", files.len());
                
                if !files.is_empty() {
                    tracing::info!("First file in service list: {} (ID: {})", 
                        files[0].name, files[0].id);
                } else {
                    tracing::info!("No files found in folder through service");
                }
                
                // Devolver respuesta con cabeceras para evitar caché del navegador
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header("Cache-Control", "no-cache, no-store, must-revalidate")
                    .header("Pragma", "no-cache")
                    .header("Expires", "0")
                    .body(axum::body::Body::from(serde_json::to_string(&files).unwrap()))
                    .unwrap();
                
                response
            },
            Err(err) => {
                tracing::error!("Error listing files through service: {}", err);
                
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                
                // Return a JSON error response
                (status, Json(serde_json::json!({
                    "error": err.to_string()
                }))).into_response()
            }
        }
    }
    
    /// Deletes a file (with trash support and dedup reference counting)
    /// 
    /// When a file is deleted:
    /// 1. Try to move to trash (soft delete)
    /// 2. If trash fails, do permanent delete
    /// 3. Decrement dedup reference count for the content hash
    pub async fn delete_file(
        State(state): State<GlobalState>,
        CurrentUserId(user_id): CurrentUserId,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let dedup_service = &state.core.dedup_service;
        
        // Get file info first to calculate content hash for dedup
        let file_info = state.applications.file_service.get_file(&id).await.ok();
        let content_hash: Option<String> = if file_info.is_some() {
            // Try to read file content and calculate hash for dedup tracking
            match state.applications.file_service.get_file_content(&id).await {
                Ok(content) => {
                    let hash = crate::infrastructure::services::dedup_service::DedupService::hash_bytes(&content);
                    tracing::debug!("🔗 DEDUP: File {} has content hash: {}", id, &hash[..12]);
                    Some(hash)
                },
                Err(e) => {
                    tracing::debug!("Could not read file content for dedup: {}", e);
                    None
                }
            }
        } else {
            None
        };
        
        // Check if trash service is available
        if let Some(trash_service) = &state.trash_service {
            tracing::info!("Moving file to trash: {}", id);
            
            // Debug logs to track trash components
            tracing::debug!("Trash service type: {}", std::any::type_name_of_val(&*trash_service));
            // User ID extracted from authenticated token via CurrentUserId
            tracing::info!("Using authenticated user ID: {}", user_id);
            
            // Try to move to trash first - add more detailed logging
            tracing::info!("About to call trash_service.move_to_trash with id={}, type=file", id);
            match trash_service.move_to_trash(&id, "file", &user_id).await {
                Ok(_) => {
                    tracing::info!("File successfully moved to trash: {}", id);
                    
                    // Decrement dedup reference count (file is in trash but content might be shared)
                    if let Some(hash) = &content_hash {
                        match dedup_service.remove_reference(hash).await {
                            Ok(deleted) => {
                                if deleted {
                                    tracing::info!("🗑️ DEDUP: Blob {} deleted (no more references)", &hash[..12]);
                                } else {
                                    tracing::debug!("🔗 DEDUP: Reference removed from blob {}", &hash[..12]);
                                }
                            },
                            Err(e) => {
                                tracing::warn!("⚠️ DEDUP: Failed to decrement reference: {}", e);
                            }
                        }
                    }
                    
                    // Note: Use 204 No Content for consistency with DELETE operations
                    return StatusCode::NO_CONTENT.into_response();
                },
                Err(err) => {
                    tracing::error!("Could not move file to trash: {:?}", err);
                    tracing::error!("Error kind: {:?}, Error details: {}", err.kind, err);
                    tracing::warn!("Could not move file to trash, falling back to permanent delete: {}", err);
                    // Fall through to regular delete if trash fails
                }
            }
        } else {
            tracing::warn!("Trash service not available, using permanent delete");
        }
        
        // Fallback to permanent delete if trash is unavailable or failed
        tracing::warn!("Falling back to permanent delete for file: {}", id);
        let file_service = &state.applications.file_service;
        match file_service.delete_file(&id).await {
            Ok(_) => {
                tracing::info!("File permanently deleted: {}", id);
                
                // Decrement dedup reference count for permanent delete
                if let Some(hash) = &content_hash {
                    match dedup_service.remove_reference(hash).await {
                        Ok(deleted) => {
                            if deleted {
                                tracing::info!("🗑️ DEDUP: Blob {} deleted (no more references)", &hash[..12]);
                            } else {
                                tracing::debug!("🔗 DEDUP: Reference removed from blob {}", &hash[..12]);
                            }
                        },
                        Err(e) => {
                            tracing::warn!("⚠️ DEDUP: Failed to decrement reference: {}", e);
                        }
                    }
                }
                
                // CRITICAL FIX: Return status code that matches the API expectations (204 No Content)
                // This ensures the client knows the operation was successful
                StatusCode::NO_CONTENT.into_response()
            },
            Err(err) => {
                tracing::error!("Error deleting file: {}", err);
                
                let status = match err.kind {
                    crate::common::errors::ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                (status, Json(serde_json::json!({
                    "error": format!("Error deleting file: {}", err)
                }))).into_response()
            }
        }
    }
    
    /// Moves a file to a different folder
    pub async fn move_file(
        State(service): State<FileServiceState>,
        Path(id): Path<String>,
        Json(payload): Json<MoveFilePayload>,
    ) -> impl IntoResponse {
        tracing::info!("API request: Moving file with ID: {} to folder: {:?}", id, payload.folder_id);
        
        // First verify if the file exists
        match service.get_file(&id).await {
            Ok(file) => {
                tracing::info!("File found: {} (ID: {}), proceeding with move operation", file.name, id);
                
                // For target folders, we trust that the move operation will verify their existence
                if let Some(folder_id) = &payload.folder_id {
                    tracing::info!("Will attempt to move to folder: {}", folder_id);
                }
                
                // Proceed with the move operation
                match service.move_file(&id, payload.folder_id).await {
                    Ok(file) => {
                        tracing::info!("File moved successfully: {} (ID: {})", file.name, file.id);
                        (StatusCode::OK, Json(file)).into_response()
                    },
                    Err(err) => {
                        // Simplify error handling
                        let status = StatusCode::INTERNAL_SERVER_ERROR;
                        tracing::error!("Error moving file: {}", err);
                        
                        (status, Json(serde_json::json!({
                            "error": format!("Error moving file: {}", err)
                        }))).into_response()
                    }
                }
            },
            Err(err) => {
                tracing::error!("Error finding file to move - does not exist: {} (ID: {})", err, id);
                (StatusCode::NOT_FOUND, Json(serde_json::json!({
                    "error": format!("The file with ID: {} does not exist", id),
                    "code": StatusCode::NOT_FOUND.as_u16()
                }))).into_response()
            }
        }
    }
}

/// Payload for moving a file
#[derive(Debug, Deserialize)]
pub struct MoveFilePayload {
    /// Target folder ID (None means root)
    pub folder_id: Option<String>,
}