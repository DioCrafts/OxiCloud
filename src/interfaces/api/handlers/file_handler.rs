use axum::{
    Json,
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{HeaderMap, Response, StatusCode, header},
    response::IntoResponse,
};
use bytes::Bytes;
use http_range_header::parse_range_header;
use serde::Deserialize;
use std::collections::HashMap;

use crate::application::ports::file_ports::OptimizedFileContent;
use crate::common::di::AppState;
use crate::interfaces::middleware::auth::{AuthUser, OptionalUserId};
use std::sync::Arc;

/**
 * Type aliases for dependency injection state.
 */
/// Global application state for dependency injection
type GlobalState = Arc<AppState>;

/**
 * API handler for file-related operations.
 *
 * Acts as a thin HTTP adapter in the hexagonal architecture: it parses requests,
 * delegates business logic to application services, and maps results to HTTP
 * responses.  No infrastructure or strategy logic lives here.
 */
pub struct FileHandler;

impl FileHandler {
    // ═══════════════════════════════════════════════════════════════════════
    //  UPLOAD
    // ═══════════════════════════════════════════════════════════════════════

    /// Streaming file upload — constant ~64 KB RAM regardless of file size.
    ///
    /// **Hash-on-Write**: SHA-256 is computed while spooling the multipart
    /// body to the temp file. This eliminates the second sequential read
    /// that dedup_service would otherwise need, cutting total I/O in half.
    pub async fn upload_file(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        multipart: Multipart,
    ) -> impl IntoResponse {
        match Self::upload_file_inner(&state, &auth_user, multipart).await {
            Ok(file) => Self::created_json_response(&file).into_response(),
            Err(response) => response.into_response(),
        }
    }

    /// Core upload logic shared by [`Self::upload_file`] and
    /// [`Self::upload_file_with_thumbnails`].
    ///
    /// Returns the typed `FileDto` on success so callers can use it
    /// directly (e.g. for thumbnail generation) without re-parsing JSON.
    async fn upload_file_inner(
        state: &GlobalState,
        auth_user: &AuthUser,
        mut multipart: Multipart,
    ) -> Result<crate::application::dtos::file_dto::FileDto, Response<Body>> {
        use sha2::{Digest, Sha256};

        let upload_service = &state.applications.file_upload_service;
        let mut folder_id: Option<String> = None;

        tracing::debug!("📤 Processing streaming file upload (hash-on-write)");

        while let Some(field) = multipart.next_field().await.unwrap_or(None) {
            let name = field.name().unwrap_or("").to_string();

            if name == "folder_id" {
                let v = field.text().await.unwrap_or_default();
                if !v.is_empty() {
                    folder_id = Some(v);
                }
                continue;
            }

            if name == "file" {
                let raw_filename = field.file_name().unwrap_or("unnamed").to_string();
                // Browsers send the full relative path (e.g. "Screenshots/file.png")
                // as the filename for folder uploads via webkitRelativePath.
                // Strip path components to get the basename only.
                // This also prevents path-traversal attacks.
                let filename = raw_filename
                    .rsplit('/')
                    .next()
                    .unwrap_or(&raw_filename)
                    .rsplit('\\')
                    .next()
                    .unwrap_or(&raw_filename)
                    .to_string();
                let content_type = field
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .to_string();

                // ── Early quota check (before spooling to disk) ──────
                if let Some(storage_svc) = state.storage_usage_service.as_ref() {
                    let estimated_size = field
                        .headers()
                        .get(header::CONTENT_LENGTH)
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(0);
                    if let Err(err) = storage_svc
                        .check_storage_quota(&auth_user.id, estimated_size)
                        .await
                    {
                        tracing::warn!(
                            "⛔ UPLOAD REJECTED (early quota): user={}, file={}, est_size={}",
                            auth_user.username,
                            filename,
                            estimated_size
                        );
                        return Err(Self::quota_error_response(err));
                    }
                }

                // ── Spool multipart field to temp file + hash-on-write ──
                let temp_dir = state.core.path_service.get_root_path().join(".dedup_temp");
                let _ = tokio::fs::create_dir_all(&temp_dir).await;
                let temp_path = temp_dir.join(format!("upload-{}", uuid::Uuid::new_v4()));

                let mut total_size: u64 = 0;
                let mut hasher = Sha256::new();
                let spool_result: Result<(), String> = async {
                    let file = tokio::fs::File::create(&temp_path)
                        .await
                        .map_err(|e| format!("Failed to create temp file: {}", e))?;

                    // Pre-allocate if Content-Length is known (reduces fragmentation)
                    let hint = field
                        .headers()
                        .get(axum::http::header::CONTENT_LENGTH)
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok());
                    if let Some(len) = hint {
                        let _ = file.set_len(len).await; // best-effort
                    }

                    // 512 KB buffer — 8× fewer write syscalls than 64 KB
                    let mut writer = tokio::io::BufWriter::with_capacity(524_288, file);
                    let mut field = field;
                    while let Ok(Some(chunk)) = field.chunk().await {
                        total_size += chunk.len() as u64;
                        hasher.update(&chunk);
                        tokio::io::AsyncWriteExt::write_all(&mut writer, &chunk)
                            .await
                            .map_err(|e| format!("Failed to write chunk: {}", e))?;
                    }
                    tokio::io::AsyncWriteExt::flush(&mut writer)
                        .await
                        .map_err(|e| format!("Failed to flush temp file: {}", e))?;
                    Ok(())
                }
                .await;

                if let Err(e) = spool_result {
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    tracing::error!("❌ UPLOAD SPOOL FAILED: {} - {}", filename, e);
                    return Err(Self::domain_error_response(
                        crate::common::errors::DomainError::internal_error("FileUpload", e),
                    ));
                }

                // Empty file — use in-memory path
                if total_size == 0 {
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    return upload_service
                        .upload_file(filename, folder_id, content_type, vec![])
                        .await
                        .map_err(Self::domain_error_response);
                }

                // Finalize hash
                let hash = hex::encode(hasher.finalize());

                // ── Quota enforcement ────────────────────────────────
                if let Some(storage_svc) = state.storage_usage_service.as_ref()
                    && let Err(err) = storage_svc
                        .check_storage_quota(&auth_user.id, total_size)
                        .await
                {
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    tracing::warn!(
                        "⛔ UPLOAD REJECTED (quota): user={}, file={}, size={}",
                        auth_user.username,
                        filename,
                        total_size
                    );
                    return Err(Self::quota_error_response(err));
                }

                // ── Streaming upload (temp file → blob store, hash pre-computed) ─
                match upload_service
                    .upload_file_streaming(
                        filename.clone(),
                        folder_id,
                        content_type,
                        &temp_path,
                        total_size,
                        Some(hash),
                    )
                    .await
                {
                    Ok(file) => {
                        tracing::info!(
                            "✅ STREAMING UPLOAD: {} ({} bytes, ID: {})",
                            filename,
                            total_size,
                            file.id
                        );
                        return Ok(file);
                    }
                    Err(err) => {
                        let _ = tokio::fs::remove_file(&temp_path).await;
                        tracing::error!("❌ UPLOAD FAILED: {} - {}", filename, err);
                        return Err(Self::domain_error_response(err));
                    }
                }
            }
        }

        Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No file provided"
            })),
        )
            .into_response())
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  THUMBNAILS
    // ═══════════════════════════════════════════════════════════════════════

    /// Get a thumbnail for an image file.
    ///
    /// Thumbnail orchestration (path resolution, generation, caching) stays here
    /// because it is tightly coupled to HTTP response headers.
    pub async fn get_thumbnail(
        State(state): State<GlobalState>,
        Path((id, size)): Path<(String, String)>,
    ) -> impl IntoResponse {
        use crate::application::ports::thumbnail_ports::ThumbnailSize;

        let file_retrieval_service = &state.applications.file_retrieval_service;
        let thumbnail_service = &state.core.thumbnail_service;

        let thumb_size = match size.as_str() {
            "icon" => ThumbnailSize::Icon,
            "preview" => ThumbnailSize::Preview,
            "large" => ThumbnailSize::Large,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "Invalid thumbnail size. Use: icon, preview, or large"
                    })),
                )
                    .into_response();
            }
        };

        let file = match file_retrieval_service.get_file(&id).await {
            Ok(f) => f,
            Err(err) => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "error": format!("File not found: {}", err)
                    })),
                )
                    .into_response();
            }
        };

        if !thumbnail_service.is_supported_image(&file.mime_type) {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "File is not a supported image type"
                })),
            )
                .into_response();
        }

        let storage_root = state.core.path_service.get_root_path();
        let file_path = storage_root.join(&file.path);

        match thumbnail_service
            .get_thumbnail(&id, thumb_size, &file_path)
            .await
        {
            Ok(data) => {
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
            }
            Err(err) => {
                tracing::error!("Thumbnail generation failed: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": format!("Failed to generate thumbnail: {}", err)
                    })),
                )
                    .into_response()
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  DOWNLOAD
    // ═══════════════════════════════════════════════════════════════════════

    /// Downloads a file with optimized multi-tier strategy.
    ///
    /// The tier selection (write-behind → hot cache → WebP transcode → mmap →
    /// streaming) is fully handled by `FileRetrievalUseCase::get_file_optimized`.
    /// This handler only deals with HTTP concerns: ETag, Range, Content-Disposition,
    /// and optional compression.
    pub async fn download_file(
        State(state): State<GlobalState>,
        Path(id): Path<String>,
        Query(params): Query<HashMap<String, String>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let retrieval = &state.applications.file_retrieval_service;

        // ── Get file metadata ────────────────────────────────────────
        let file_dto = match retrieval.get_file(&id).await {
            Ok(f) => f,
            Err(err) => {
                let status = if err.to_string().contains("not found")
                    || err.to_string().contains("NotFound")
                {
                    StatusCode::NOT_FOUND
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                };
                return (
                    status,
                    Json(serde_json::json!({
                        "error": err.to_string()
                    })),
                )
                    .into_response();
            }
        };

        // ── Metadata-only request ────────────────────────────────────
        if params
            .get("metadata")
            .is_some_and(|v| v == "true" || v == "1")
        {
            return (
                StatusCode::OK,
                Json(serde_json::json!({
                    "id": file_dto.id,
                    "name": file_dto.name,
                    "path": file_dto.path,
                    "size": file_dto.size,
                    "mime_type": file_dto.mime_type,
                    "folder_id": file_dto.folder_id,
                    "created_at": file_dto.created_at,
                    "modified_at": file_dto.modified_at
                })),
            )
                .into_response();
        }

        let etag = format!("\"{}-{}\"", id, file_dto.modified_at);

        // ── ETag (304 Not Modified) ──────────────────────────────────
        if let Some(inm) = headers.get(header::IF_NONE_MATCH)
            && let Ok(client_etag) = inm.to_str()
            && (client_etag == etag || client_etag == "*")
        {
            return Response::builder()
                .status(StatusCode::NOT_MODIFIED)
                .header(header::ETAG, &etag)
                .body(Body::empty())
                .unwrap()
                .into_response();
        }

        // ── Range Requests ───────────────────────────────────────────
        if let Some(range_header) = headers.get(header::RANGE)
            && let Ok(range_str) = range_header.to_str()
            && let Ok(ranges) = parse_range_header(range_str)
        {
            let validated = ranges.validate(file_dto.size);
            if let Ok(valid_ranges) = validated {
                if let Some(range) = valid_ranges.first() {
                    let start = *range.start();
                    let end = *range.end();
                    let range_length = end - start + 1;
                    let disposition =
                        Self::content_disposition(&file_dto.name, &file_dto.mime_type, &params);

                    match retrieval
                        .get_file_range_stream(&id, start, Some(end + 1))
                        .await
                    {
                        Ok(stream) => {
                            return Response::builder()
                                .status(StatusCode::PARTIAL_CONTENT)
                                .header(header::CONTENT_TYPE, &file_dto.mime_type)
                                .header(header::CONTENT_DISPOSITION, &disposition)
                                .header(header::CONTENT_LENGTH, range_length)
                                .header(
                                    header::CONTENT_RANGE,
                                    format!("bytes {}-{}/{}", start, end, file_dto.size),
                                )
                                .header(header::ACCEPT_RANGES, "bytes")
                                .header(header::ETAG, &etag)
                                .header(
                                    header::CACHE_CONTROL,
                                    "private, max-age=3600, must-revalidate",
                                )
                                .body(Body::from_stream(Box::into_pin(stream)))
                                .unwrap()
                                .into_response();
                        }
                        Err(err) => {
                            tracing::error!("Error creating range stream: {}", err);
                            // fall through to normal download
                        }
                    }
                }
            } else {
                return Response::builder()
                    .status(StatusCode::RANGE_NOT_SATISFIABLE)
                    .header(header::CONTENT_RANGE, format!("bytes */{}", file_dto.size))
                    .body(Body::empty())
                    .unwrap()
                    .into_response();
            }
        }

        // ── Normal download (delegated to service) ───────────────────
        let disposition = Self::content_disposition(&file_dto.name, &file_dto.mime_type, &params);

        let accept_webp = headers
            .get(header::ACCEPT)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|a| a.contains("image/webp"));
        let prefer_original = params
            .get("original")
            .is_some_and(|v| v == "true" || v == "1");

        match retrieval
            .get_file_optimized_preloaded(&id, file_dto.clone(), accept_webp, prefer_original)
            .await
        {
            Ok((_file, content)) => match content {
                OptimizedFileContent::Bytes {
                    data, mime_type, ..
                } => Self::build_cached_response(
                    data,
                    &mime_type,
                    &disposition,
                    &etag,
                )
                .into_response(),
                OptimizedFileContent::Mmap(mmap_data) => Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, &file_dto.mime_type)
                    .header(header::CONTENT_DISPOSITION, &disposition)
                    .header(header::CONTENT_LENGTH, mmap_data.len())
                    .header(header::ETAG, &etag)
                    .header(
                        header::CACHE_CONTROL,
                        "private, max-age=3600, must-revalidate",
                    )
                    .header(header::ACCEPT_RANGES, "bytes")
                    .body(Body::from(mmap_data))
                    .unwrap()
                    .into_response(),
                OptimizedFileContent::Stream(pinned_stream) => Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, &file_dto.mime_type)
                    .header(header::CONTENT_DISPOSITION, &disposition)
                    .header(header::CONTENT_LENGTH, file_dto.size)
                    .header(header::ETAG, &etag)
                    .header(
                        header::CACHE_CONTROL,
                        "private, max-age=3600, must-revalidate",
                    )
                    .header(header::ACCEPT_RANGES, "bytes")
                    .body(Body::from_stream(pinned_stream))
                    .unwrap()
                    .into_response(),
            },
            Err(err) => {
                tracing::error!("Error downloading file: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": format!("Error reading file: {}", err)
                    })),
                )
                    .into_response()
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  LIST
    // ═══════════════════════════════════════════════════════════════════════

    /// Lists files, extracting `folder_id` from query parameters.
    ///
    /// Axum-compatible handler wrapper around [`Self::list_files`].
    pub async fn list_files_query(
        State(state): State<GlobalState>,
        headers: HeaderMap,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let folder_id = params.get("folder_id").map(|id| id.as_str());
        tracing::info!("API: Listing files with folder_id: {:?}", folder_id);

        let retrieval = &state.applications.file_retrieval_service;
        match retrieval.list_files(folder_id).await {
            Ok(files) => {
                // Compute lightweight ETag from max modified_at + count
                let max_mod = files.iter().map(|f| f.modified_at).max().unwrap_or(0);
                let count = files.len();
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                std::hash::Hash::hash(&max_mod, &mut hasher);
                std::hash::Hash::hash(&count, &mut hasher);
                let etag = format!("\"{:x}\"", std::hash::Hasher::finish(&hasher));

                // 304 Not Modified if client already has this version
                if let Some(inm) = headers.get(header::IF_NONE_MATCH)
                    && let Ok(client_etag) = inm.to_str()
                    && client_etag == etag
                {
                    return Response::builder()
                        .status(StatusCode::NOT_MODIFIED)
                        .header(header::ETAG, &etag)
                        .body(Body::empty())
                        .unwrap()
                        .into_response();
                }

                tracing::info!("Found {} files", files.len());
                let mut resp = (StatusCode::OK, Json(files)).into_response();
                resp.headers_mut().insert(
                    header::ETAG,
                    header::HeaderValue::from_str(&etag).unwrap(),
                );
                resp
            }
            Err(err) => {
                tracing::error!("Error listing files: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": format!("Error listing files: {}", err)
                    })),
                )
                    .into_response()
            }
        }
    }

    /// Uploads a file and generates thumbnails in the background for images.
    ///
    /// Delegates to [`Self::upload_file_inner`] and, on success, spawns
    /// a background task to generate all thumbnail sizes before serialising
    /// the `FileDto` once.
    pub async fn upload_file_with_thumbnails(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        multipart: Multipart,
    ) -> impl IntoResponse {
        let file = match Self::upload_file_inner(&state, &auth_user, multipart).await {
            Ok(f) => f,
            Err(response) => return response.into_response(),
        };

        // Generate thumbnails for supported images in background
        if state.core.thumbnail_service.is_supported_image(&file.mime_type) {
            let file_id = file.id.clone();
            let file_path_rel = file.path.clone();
            let thumbnail_service = state.core.thumbnail_service.clone();
            let path_service = state.core.path_service.clone();

            tokio::spawn(async move {
                let file_path = path_service.get_root_path().join(&file_path_rel);
                tracing::info!("🖼️ Generating thumbnails for: {}", file_id);
                thumbnail_service.generate_all_sizes_background(file_id, file_path);
            });
        }

        Self::created_json_response(&file).into_response()
    }

    /// Lists files, optionally filtered by folder ID
    pub async fn list_files(
        State(state): State<GlobalState>,
        folder_id: Option<&str>,
    ) -> impl IntoResponse {
        tracing::info!("Listing files with folder_id: {:?}", folder_id);

        let retrieval = &state.applications.file_retrieval_service;
        match retrieval.list_files(folder_id).await {
            Ok(files) => {
                tracing::info!("Found {} files through the service", files.len());
                Response::builder()
                    .status(StatusCode::OK)
                    .header("Cache-Control", "no-cache, no-store, must-revalidate")
                    .header("Pragma", "no-cache")
                    .header("Expires", "0")
                    .body(Body::from(serde_json::to_string(&files).unwrap()))
                    .unwrap()
            }
            Err(err) => {
                tracing::error!("Error listing files: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": err.to_string()
                    })),
                )
                    .into_response()
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  DELETE
    // ═══════════════════════════════════════════════════════════════════════

    /// Deletes a file (trash-first with dedup cleanup).
    ///
    /// All logic (trash fallback, dedup ref-count, hash computation) is handled
    /// by `FileManagementUseCase::delete_with_cleanup`.
    ///
    /// When auth is available, uses trash-first deletion; otherwise falls back
    /// to permanent delete so the endpoint works with or without auth.
    pub async fn delete_file(
        State(state): State<GlobalState>,
        OptionalUserId(user_id): OptionalUserId,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let mgmt = &state.applications.file_management_service;

        let result = if let Some(uid) = user_id {
            // Auth available: trash-first with dedup cleanup
            mgmt.delete_with_cleanup(&id, &uid)
                .await
                .map(|was_trashed| {
                    if was_trashed {
                        tracing::info!("File moved to trash: {}", id);
                    } else {
                        tracing::info!("File permanently deleted: {}", id);
                    }
                })
        } else {
            // No auth: permanent delete
            tracing::warn!("No auth context – permanently deleting file: {}", id);
            mgmt.delete_file(&id).await.map(|_| {
                tracing::info!("File permanently deleted (no auth): {}", id);
            })
        };

        match result {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(err) => {
                tracing::error!("Error deleting file: {}", err);
                let status = if err.to_string().contains("not found")
                    || err.to_string().contains("NotFound")
                {
                    StatusCode::NOT_FOUND
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                };
                (
                    status,
                    Json(serde_json::json!({
                        "error": format!("Error deleting file: {}", err)
                    })),
                )
                    .into_response()
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  MOVE
    // ═══════════════════════════════════════════════════════════════════════

    /// Renames a file
    pub async fn rename_file(
        State(state): State<GlobalState>,
        Path(id): Path<String>,
        Json(payload): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let new_name = match payload.get("name").and_then(|v| v.as_str()) {
            Some(name) if !name.trim().is_empty() => name.trim().to_string(),
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "Missing or empty 'name' field"
                    })),
                )
                    .into_response();
            }
        };

        tracing::info!("Renaming file {} to \"{}\"", id, new_name);
        let mgmt = &state.applications.file_management_service;
        match mgmt.rename_file(&id, &new_name).await {
            Ok(file_dto) => (StatusCode::OK, Json(file_dto)).into_response(),
            Err(err) => {
                tracing::error!("Error renaming file: {}", err);
                let status = if err.to_string().contains("not found")
                    || err.to_string().contains("NotFound")
                {
                    StatusCode::NOT_FOUND
                } else if err.to_string().contains("already exists") {
                    StatusCode::CONFLICT
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                };
                (
                    status,
                    Json(serde_json::json!({
                        "error": format!("Error renaming file: {}", err)
                    })),
                )
                    .into_response()
            }
        }
    }

    /// Moves a file to a different folder
    pub async fn move_file(
        State(state): State<GlobalState>,
        Path(id): Path<String>,
        Json(payload): Json<MoveFilePayload>,
    ) -> impl IntoResponse {
        tracing::info!("Moving file {} to folder {:?}", id, payload.folder_id);

        let retrieval = &state.applications.file_retrieval_service;
        let mgmt = &state.applications.file_management_service;

        match retrieval.get_file(&id).await {
            Ok(_) => match mgmt.move_file(&id, payload.folder_id).await {
                Ok(file) => (StatusCode::OK, Json(file)).into_response(),
                Err(err) => {
                    tracing::error!("Error moving file: {}", err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("Error moving file: {}", err)
                        })),
                    )
                        .into_response()
                }
            },
            Err(err) => {
                tracing::error!("File not found for move: {}", err);
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "error": format!("File with ID {} does not exist", id)
                    })),
                )
                    .into_response()
            }
        }
    }

    /// Moves a file to a different folder (simplified payload accepting generic JSON)
    pub async fn move_file_simple(
        State(state): State<GlobalState>,
        Path(id): Path<String>,
        Json(payload): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let folder_id = payload
            .get("folder_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mgmt = &state.applications.file_management_service;
        match mgmt.move_file(&id, folder_id).await {
            Ok(file_dto) => (StatusCode::OK, Json(file_dto)).into_response(),
            Err(err) => {
                tracing::error!("Error moving file: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": format!("Error moving file: {}", err)
                    })),
                )
                    .into_response()
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  PRIVATE HELPERS
    // ═══════════════════════════════════════════════════════════════════════

    /// Build a Content-Disposition header value.
    fn content_disposition(name: &str, mime: &str, params: &HashMap<String, String>) -> String {
        let force_inline = params
            .get("inline")
            .is_some_and(|v| v == "true" || v == "1");
        if force_inline
            || mime.starts_with("image/")
            || mime == "application/pdf"
            || mime.starts_with("video/")
            || mime.starts_with("audio/")
        {
            format!("inline; filename=\"{}\"", name)
        } else {
            format!("attachment; filename=\"{}\"", name)
        }
    }

    /// Build a 201 Created JSON response.
    fn created_json_response(file: &crate::application::dtos::file_dto::FileDto) -> Response<Body> {
        Response::builder()
            .status(StatusCode::CREATED)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
            .body(Body::from(serde_json::to_string(file).unwrap()))
            .unwrap()
    }

    /// Build error response for DomainError.
    fn domain_error_response(err: crate::common::errors::DomainError) -> Response<Body> {
        let status = match err.kind {
            crate::common::errors::ErrorKind::NotFound => StatusCode::NOT_FOUND,
            crate::common::errors::ErrorKind::QuotaExceeded => StatusCode::INSUFFICIENT_STORAGE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Response::builder()
            .status(status)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::json!({ "error": format!("Error: {}", err) }).to_string(),
            ))
            .unwrap()
    }

    /// Build a quota-specific error response with 507 status and structured body.
    fn quota_error_response(err: crate::common::errors::DomainError) -> Response<Body> {
        Response::builder()
            .status(StatusCode::INSUFFICIENT_STORAGE)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::json!({
                    "error": err.message,
                    "error_type": "QuotaExceeded"
                })
                .to_string(),
            ))
            .unwrap()
    }

    /// Build response for cached/small files.
    ///
    /// Compression is handled uniformly by `CompressionLayer` (tower-http)
    /// which negotiates `Accept-Encoding` and applies gzip/brotli in streaming
    /// mode. No manual compression is done here to avoid double-encoding.
    fn build_cached_response(
        content: Bytes,
        mime_type: &str,
        disposition: &str,
        etag: &str,
    ) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime_type)
            .header(header::CONTENT_DISPOSITION, disposition)
            .header(header::ETAG, etag)
            .header(
                header::CACHE_CONTROL,
                "private, max-age=3600, must-revalidate",
            )
            .header(header::VARY, "Accept-Encoding")
            .header(header::CONTENT_LENGTH, content.len())
            .body(Body::from(content))
            .unwrap()
    }
}

/// Payload for moving a file
#[derive(Debug, Deserialize)]
pub struct MoveFilePayload {
    /// Target folder ID (None means root)
    pub folder_id: Option<String>,
}
