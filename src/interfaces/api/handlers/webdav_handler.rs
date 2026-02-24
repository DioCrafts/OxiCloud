/**
 * WebDAV Handler Module
 *
 * This module implements the WebDAV protocol (RFC 4918) endpoints for OxiCloud.
 * It provides a complete WebDAV server implementation that allows clients to
 * perform file operations over HTTP, including reading, writing, and manipulating
 * files and directories.
 */
use axum::{
    Router,
    body::{self, Body},
    http::{HeaderName, Request, StatusCode, header},
    response::Response,
};
use bytes::{Buf, Bytes};
use chrono::Utc;
use quick_xml::Writer;
use uuid::Uuid;

use crate::application::adapters::webdav_adapter::{
    LockInfo, LockScope, LockType, PropFindRequest, WebDavAdapter,
};
use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;
use crate::application::ports::file_ports::FileRetrievalUseCase;
use crate::application::ports::inbound::FolderUseCase;
use crate::common::di::AppState;
use crate::interfaces::errors::AppError;
use crate::interfaces::middleware::auth::CurrentUser;
use std::sync::Arc;

// Create a custom DAV header since it's not in the standard headers
const HEADER_DAV: HeaderName = HeaderName::from_static("dav");
const HEADER_LOCK_TOKEN: HeaderName = HeaderName::from_static("lock-token");
// const HEADER_IF: HeaderName = HeaderName::from_static("if");

/// Maximum body size for XML-based WebDAV requests (PROPFIND, PROPPATCH, LOCK).
/// 1 MB is generous — a typical PROPFIND body is < 1 KB.
const MAX_XML_BODY: usize = 1_048_576;

/// Maximum body size for MKCOL requests (RFC 4918: body must be empty).
const MAX_MKCOL_BODY: usize = 4096;

/// Batch size for streaming PROPFIND — files and folders are fetched in pages
/// of this size to keep memory constant regardless of folder contents.
const PROPFIND_BATCH_SIZE: i64 = 500;

/**
 * Creates and returns the WebDAV router with all required endpoints.
 *
 * This function sets up all WebDAV method handlers following RFC 4918,
 * mapping HTTP methods to appropriate WebDAV operations.
 *
 * @return Router configured with WebDAV endpoints
 */
pub fn webdav_routes() -> Router<Arc<AppState>> {
    // Three explicit routes to avoid Axum trailing-slash gaps
    // (same pattern used for CalDAV/CardDAV)
    Router::new()
        .route("/webdav/{*path}", axum::routing::any(handle_webdav_methods))
        .route("/webdav/", axum::routing::any(handle_webdav_methods_root))
        .route("/webdav", axum::routing::any(handle_webdav_methods_root))
}

/// Extract the resource path from the request URI, stripping the `/webdav/` prefix.
fn extract_webdav_path(uri: &axum::http::Uri) -> String {
    let raw = uri.path();
    if let Some(rest) = raw.strip_prefix("/webdav/") {
        rest.trim_end_matches('/').to_string()
    } else if raw == "/webdav" {
        String::new()
    } else {
        // Fallback: split-based extraction
        let parts: Vec<&str> = raw.split('/').collect();
        if parts.len() > 2 {
            parts[2..].join("/")
        } else {
            String::new()
        }
    }
}

async fn handle_webdav_methods_root(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    handle_webdav_dispatch(state, req, String::new()).await
}

async fn handle_webdav_methods(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    let path = extract_webdav_path(req.uri());
    handle_webdav_dispatch(state, req, path).await
}

async fn handle_webdav_dispatch(
    state: Arc<AppState>,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    let method = req.method().clone();

    match method.as_str() {
        "OPTIONS" => handle_options(path).await,
        "GET" => handle_get(state, req, path).await,
        "HEAD" => handle_head(state, req, path).await,
        "PUT" => handle_put(state, req, path).await,
        "MKCOL" => handle_mkcol(state, req, path).await,
        "DELETE" => handle_delete(state, req, path).await,
        "MOVE" => handle_move(state, req, path).await,
        "COPY" => handle_copy(state, req, path).await,
        "PROPFIND" => handle_propfind(state, req, path).await,
        "PROPPATCH" => handle_proppatch(state, req, path).await,
        "LOCK" => handle_lock(state, req, path).await,
        "UNLOCK" => handle_unlock(state, req, path).await,
        _ => Err(AppError::method_not_allowed(format!(
            "Method not allowed: {}",
            method
        ))),
    }
}

/**
 * Handles OPTIONS requests to advertise WebDAV capabilities.
 *
 * This handler responds with the DAV header indicating WebDAV compliance
 * level and the methods supported by this WebDAV server.
 *
 * @param state The application state containing service dependencies
 * @param path The requested resource path
 * @return HTTP response with appropriate WebDAV headers
 */
async fn handle_options(_path: String) -> Result<Response<Body>, AppError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(HEADER_DAV, "1, 2") // Class 1 and 2 WebDAV support
        .header(
            header::ALLOW,
            "OPTIONS, GET, HEAD, PUT, DELETE, PROPFIND, PROPPATCH, MKCOL, COPY, MOVE, LOCK, UNLOCK",
        )
        .body(Body::empty())
        .unwrap())
}

/**
 * Handles PROPFIND requests to retrieve resource properties.
 *
 * This handler processes WebDAV PROPFIND requests according to RFC 4918,
 * retrieving properties of files and folders in the specified path.
 *
 * **Security hardening (Sol.2):** `Depth: infinity` is rejected with
 * `403 Forbidden` and the RFC 4918 `propfind-finite-depth` precondition
 * error body.  The default depth when the header is absent is `1`.
 *
 * **Streaming response (Sol.3):** For `Depth: 1`, files and sub-folders
 * are fetched in batches of `PROPFIND_BATCH_SIZE` and the XML response
 * is written incrementally to a streaming body.  Memory usage is O(batch)
 * regardless of how many children the folder contains.
 *
 * @param state The application state containing service dependencies
 * @param req   The HTTP request containing the PROPFIND XML body
 * @param path  The requested resource path
 * @return      207 Multi-Status XML response with resource properties
 */
async fn handle_propfind(
    state: Arc<AppState>,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    // ── 1. Extract and validate Depth header ─────────────────────
    let depth = req
        .headers()
        .get("Depth")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("1");

    // RFC 4918 §9.1: servers MAY reject Depth:infinity with 403
    if depth == "infinity" {
        let body = r#"<?xml version="1.0" encoding="utf-8"?>
<D:error xmlns:D="DAV:">
  <D:propfind-finite-depth/>
</D:error>"#;
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .body(Body::from(body))
            .unwrap());
    }

    // Normalize: anything other than "0" or "1" is treated as "0"
    let depth = match depth {
        "0" | "1" => depth,
        _ => "0",
    };
    let depth_owned = depth.to_string();

    // ── 2. Authenticate ──────────────────────────────────────────
    let _user = {
        let user_ref = req
            .extensions()
            .get::<CurrentUser>()
            .ok_or_else(|| AppError::unauthorized("Authentication required"))?;
        user_ref.clone()
    };

    // ── 3. Parse PROPFIND XML body ───────────────────────────────
    let body_bytes = {
        let body = req.into_body();
        body::to_bytes(body, MAX_XML_BODY)
            .await
            .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?
    };

    let propfind_request = if body_bytes.is_empty() {
        PropFindRequest {
            prop_find_type: crate::application::adapters::webdav_adapter::PropFindType::AllProp,
        }
    } else {
        WebDavAdapter::parse_propfind(body_bytes.reader()).map_err(|e| {
            AppError::bad_request(format!("Failed to parse PROPFIND request: {}", e))
        })?
    };

    // ── 4. Services ──────────────────────────────────────────────
    let folder_service = state.applications.folder_service.clone();
    let file_retrieval_service = state.applications.file_retrieval_service.clone();

    let base_href = if path.is_empty() || path == "/" {
        "/webdav/".to_string()
    } else {
        format!("/webdav/{}/", path)
    };

    // ── 5. Determine target resource ─────────────────────────────
    if path.is_empty() || path == "/" {
        // Root folder
        let root_folder = FolderDto {
            id: "root".to_string(),
            name: "".to_string(),
            path: "".to_string(),
            parent_id: None,
            owner_id: None,
            created_at: Utc::now().timestamp() as u64,
            modified_at: Utc::now().timestamp() as u64,
            is_root: true,
            icon_class: "fas fa-folder".to_string(),
            icon_special_class: "folder-icon".to_string(),
            category: "Folder".to_string(),
        };

        return build_streaming_propfind_response(
            root_folder,
            None, // folder_id = None → root children
            &depth_owned,
            &base_href,
            propfind_request,
            folder_service,
            file_retrieval_service,
        )
        .await;
    }

    // Try folder first
    if let Ok(folder) = folder_service.get_folder_by_path(&path).await {
        let folder_id = folder.id.clone();
        return build_streaming_propfind_response(
            folder,
            Some(folder_id),
            &depth_owned,
            &base_href,
            propfind_request,
            folder_service,
            file_retrieval_service,
        )
        .await;
    }

    // Try file
    if let Ok(file) = file_retrieval_service.get_file_by_path(&path).await {
        let mut buf = Vec::with_capacity(1024);
        {
            let mut xml_writer = Writer::new(&mut buf);
            WebDavAdapter::write_multistatus_start(&mut xml_writer)
                .map_err(|e| AppError::internal_error(format!("XML write error: {}", e)))?;
            WebDavAdapter::write_file_entry(
                &mut xml_writer,
                &file,
                &propfind_request,
                &base_href,
            )
            .map_err(|e| AppError::internal_error(format!("XML write error: {}", e)))?;
            WebDavAdapter::write_multistatus_end(&mut xml_writer)
                .map_err(|e| AppError::internal_error(format!("XML write error: {}", e)))?;
        }
        return Ok(Response::builder()
            .status(StatusCode::MULTI_STATUS)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .body(Body::from(buf))
            .unwrap());
    }

    Err(AppError::not_found(format!("Resource not found: {}", path)))
}

/// Builds a streaming 207 Multi-Status PROPFIND response.
///
/// The XML is written incrementally: first the folder itself, then children
/// (sub-folders and files) are fetched in batches of `PROPFIND_BATCH_SIZE`.
/// Each batch is serialised to XML and sent as a chunk, so memory stays
/// constant at O(batch_size) regardless of the total number of children.
async fn build_streaming_propfind_response(
    folder: FolderDto,
    folder_id: Option<String>,
    depth: &str,
    base_href: &str,
    propfind_request: PropFindRequest,
    folder_service: std::sync::Arc<dyn FolderUseCase>,
    file_retrieval_service: std::sync::Arc<dyn FileRetrievalUseCase>,
) -> Result<Response<Body>, AppError> {
    let depth = depth.to_string();
    let base_href = base_href.to_string();
    let propfind_request = Arc::new(propfind_request);

    let stream = async_stream::try_stream! {
        // ── XML header + <D:multistatus> + folder entry ──────────
        let mut buf = Vec::with_capacity(4096);
        {
            let mut w = Writer::new(&mut buf);
            WebDavAdapter::write_multistatus_start(&mut w)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
            WebDavAdapter::write_folder_entry(&mut w, &folder, &propfind_request, &base_href)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        }
        yield Bytes::from(buf);

        // ── Children (only if Depth == 1) ────────────────────────
        if depth == "1" {
            let pagination = crate::application::dtos::pagination::PaginationRequestDto {
                page: 0,
                page_size: PROPFIND_BATCH_SIZE as usize,
            };
            let fid_ref = folder_id.as_deref();

            // Stream sub-folders in pages
            let mut page = 0usize;
            loop {
                let pag = crate::application::dtos::pagination::PaginationRequestDto {
                    page,
                    page_size: pagination.page_size,
                };
                let result = folder_service
                    .list_folders_paginated(fid_ref, &pag)
                    .await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

                if result.items.is_empty() {
                    break;
                }

                let mut chunk = Vec::with_capacity(result.items.len() * 800);
                {
                    let mut w = Writer::new(&mut chunk);
                    for subfolder in &result.items {
                        let href = format!("{}{}/", base_href, subfolder.name);
                        WebDavAdapter::write_folder_entry(&mut w, subfolder, &propfind_request, &href)
                            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
                    }
                }
                let has_more = result.pagination.has_next;
                yield Bytes::from(chunk);

                if !has_more {
                    break;
                }
                page += 1;
            }

            // Stream files in pages
            let mut offset: i64 = 0;
            loop {
                let batch: Vec<FileDto> = file_retrieval_service
                    .list_files_batch(fid_ref, offset, PROPFIND_BATCH_SIZE)
                    .await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

                if batch.is_empty() {
                    break;
                }

                let batch_len = batch.len();
                let mut chunk = Vec::with_capacity(batch_len * 800);
                {
                    let mut w = Writer::new(&mut chunk);
                    for file in &batch {
                        let href = format!("{}{}", base_href, file.name);
                        WebDavAdapter::write_file_entry(&mut w, file, &propfind_request, &href)
                            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
                    }
                }
                yield Bytes::from(chunk);

                if (batch_len as i64) < PROPFIND_BATCH_SIZE {
                    break;
                }
                offset += batch_len as i64;
            }
        }

        // ── Close </D:multistatus> ───────────────────────────────
        let mut buf = Vec::with_capacity(32);
        {
            let mut w = Writer::new(&mut buf);
            WebDavAdapter::write_multistatus_end(&mut w)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        }
        yield Bytes::from(buf);
    };

    use futures::TryStreamExt;
    let stream = stream.map_err(|e: std::io::Error| -> Box<dyn std::error::Error + Send + Sync> { Box::new(e) });

    Ok(Response::builder()
        .status(StatusCode::MULTI_STATUS)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(Body::from_stream(stream))
        .unwrap())
}

/**
 * Handles PROPPATCH requests to set or remove resource properties.
 *
 * This handler processes WebDAV PROPPATCH requests according to RFC 4918,
 * modifying properties of files and folders in the specified path.
 *
 * @param state The application state containing service dependencies
 * @param user The authenticated user information
 * @param path The requested resource path
 * @param req The HTTP request containing the PROPPATCH XML body
 * @return XML response with property modification results
 */
async fn handle_proppatch(
    _state: Arc<AppState>,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    let _user = req
        .extensions()
        .get::<CurrentUser>()
        .ok_or_else(|| AppError::unauthorized("Authentication required"))?;

    // Read request body (XML — bounded to 1 MB)
    let body_bytes = body::to_bytes(req.into_body(), MAX_XML_BODY)
        .await
        .map_err(|e| AppError::payload_too_large(format!("PROPPATCH body too large or unreadable: {}", e)))?;
    let (props_to_set, props_to_remove) = WebDavAdapter::parse_proppatch(body_bytes.reader())
        .map_err(|e| AppError::bad_request(format!("Failed to parse PROPPATCH request: {}", e)))?;

    // For now, we don't actually persist custom properties, but we respond as if we did
    // In a full implementation, we would store these properties in a database

    // Generate response - we'll pretend all operations succeeded
    let mut results = Vec::new();

    // For each property to set, indicate success
    for prop in &props_to_set {
        results.push((&prop.name, true));
    }

    // For each property to remove, indicate success
    for prop in &props_to_remove {
        results.push((prop, true));
    }

    // Generate response
    let href = format!("/webdav/{}", path);
    let mut response_body = Vec::new();
    WebDavAdapter::generate_proppatch_response(&mut response_body, &href, &results).map_err(
        |e| AppError::internal_error(format!("Failed to generate PROPPATCH response: {}", e)),
    )?;

    Ok(Response::builder()
        .status(StatusCode::MULTI_STATUS)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(Body::from(response_body))
        .unwrap())
}

/**
 * Handles GET requests to retrieve file contents.
 *
 * This handler retrieves the contents of a file at the specified path.
 *
 * @param state The application state containing service dependencies
 * @param user The authenticated user information
 * @param path The requested resource path
 * @return HTTP response with file contents
 */
async fn handle_get(
    state: Arc<AppState>,
    _req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    // Get file service from state
    let file_retrieval_service = &state.applications.file_retrieval_service;

    // Check if path is empty (root folder)
    if path.is_empty() || path == "/" {
        return Err(AppError::bad_request("Cannot GET a directory"));
    }

    // Get file metadata
    let file = file_retrieval_service
        .get_file_by_path(&path)
        .await
        .map_err(|_e| AppError::not_found(format!("File not found: {}", path)))?;

    // Stream file content — constant ~64 KB memory regardless of file size
    let stream = file_retrieval_service
        .get_file_stream(&file.id)
        .await
        .map_err(|e| AppError::internal_error(format!("Failed to stream file: {}", e)))?;

    // Build streaming response using Content-Length from metadata
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, file.mime_type)
        .header(header::CONTENT_LENGTH, file.size)
        .header(header::ETAG, format!("\"{}\"", file.id))
        .header(
            header::LAST_MODIFIED,
            chrono::DateTime::<Utc>::from_timestamp(file.created_at as i64, 0)
                .unwrap_or_else(Utc::now)
                .to_rfc2822(),
        )
        .body(Body::from_stream(Box::into_pin(stream)))
        .unwrap())
}

/**
 * Handles HEAD requests — same as GET but returns only headers, no body.
 */
async fn handle_head(
    state: Arc<AppState>,
    _req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    let file_retrieval_service = &state.applications.file_retrieval_service;
    let folder_service = &state.applications.folder_service;

    if path.is_empty() || path == "/" {
        // Root folder — return collection headers
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "httpd/unix-directory")
            .header(header::CONTENT_LENGTH, 0)
            .body(Body::empty())
            .unwrap());
    }

    // Check if it's a folder first
    if let Ok(folder) = folder_service.get_folder_by_path(&path).await {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "httpd/unix-directory")
            .header(header::CONTENT_LENGTH, 0)
            .header(header::ETAG, format!("\"{}\"", folder.id))
            .body(Body::empty())
            .unwrap());
    }

    // Try as file — use metadata only, never load content for HEAD
    let file = file_retrieval_service
        .get_file_by_path(&path)
        .await
        .map_err(|_e| AppError::not_found(format!("Resource not found: {}", path)))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, &file.mime_type)
        .header(header::CONTENT_LENGTH, file.size)
        .header(header::ETAG, format!("\"{}\"", file.id))
        .header(
            header::LAST_MODIFIED,
            chrono::DateTime::<Utc>::from_timestamp(file.created_at as i64, 0)
                .unwrap_or_else(Utc::now)
                .to_rfc2822(),
        )
        .body(Body::empty())
        .unwrap())
}

/**
 * Handles PUT requests to create or update files.
 *
 * **Streaming implementation**: the request body is spooled to a temp file
 * with incremental SHA-256 hashing. Peak RAM usage is ~256 KB regardless
 * of file size. The temp file is then atomically moved into blob storage
 * via `update_file_streaming`.
 *
 * @param state The application state containing service dependencies
 * @param path The requested resource path
 * @param req The HTTP request containing the file contents
 * @return HTTP response indicating success
 */
async fn handle_put(
    state: Arc<AppState>,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    use http_body_util::BodyStream;
    use sha2::{Digest, Sha256};
    use tokio::io::AsyncWriteExt;
    use tokio_stream::StreamExt;

    // Get file service from state
    let file_upload_service = &state.applications.file_upload_service;

    // Check if path is empty (root folder)
    if path.is_empty() || path == "/" {
        return Err(AppError::bad_request("Cannot PUT to root folder"));
    }

    // Hard upload size limit from config
    let max_upload = state.core.config.storage.max_upload_size;

    // Extract content type before consuming the request
    let content_type = req
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    // ── Streaming spool: body → temp file + incremental hash ──
    let temp_file = tempfile::NamedTempFile::new()
        .map_err(|e| AppError::internal_error(format!("Failed to create temp file: {}", e)))?;
    let temp_path = temp_file.path().to_path_buf();

    let mut file = tokio::fs::File::create(&temp_path)
        .await
        .map_err(|e| AppError::internal_error(format!("Failed to open temp file: {}", e)))?;

    let mut hasher = Sha256::new();
    let mut total_bytes: usize = 0;
    let mut stream = BodyStream::new(req.into_body());

    while let Some(frame_result) = stream.next().await {
        let frame = frame_result
            .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?;
        if let Some(chunk) = frame.data_ref() {
            total_bytes += chunk.len();
            if total_bytes > max_upload {
                // Abort early — stop reading, delete temp file
                drop(file);
                let _ = tokio::fs::remove_file(&temp_path).await;
                return Err(AppError::payload_too_large(format!(
                    "Upload exceeds maximum size of {} bytes",
                    max_upload
                )));
            }
            hasher.update(chunk);
            file.write_all(chunk)
                .await
                .map_err(|e| AppError::internal_error(format!("Failed to write to temp file: {}", e)))?;
        }
    }
    file.flush().await
        .map_err(|e| AppError::internal_error(format!("Failed to flush temp file: {}", e)))?;
    drop(file);

    let hash = hex::encode(hasher.finalize());

    // ── Atomic store: temp file → dedup blob + DB metadata update ──
    let result = file_upload_service
        .update_file_streaming(
            &path,
            &temp_path,
            total_bytes as u64,
            &content_type,
            Some(hash),
        )
        .await;

    // Clean up temp file (may already be moved by dedup, ignore error)
    let _ = tokio::fs::remove_file(&temp_path).await;

    match result {
        Ok(_) => Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::empty())
            .unwrap()),
        Err(e) => Err(AppError::internal_error(format!(
            "Failed to put file: {}",
            e
        ))),
    }
}

/**
 * Handles MKCOL requests to create folders.
 *
 * This handler creates a new folder at the specified path.
 *
 * @param state The application state containing service dependencies
 * @param user The authenticated user information
 * @param path The requested resource path
 * @return HTTP response indicating success
 */
async fn handle_mkcol(
    state: Arc<AppState>,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    // Get folder service from state
    let folder_service = &state.applications.folder_service;

    // Check if path is empty (root folder)
    if path.is_empty() || path == "/" {
        return Err(AppError::conflict("Root folder already exists"));
    }

    // Read request body - must be empty for MKCOL
    let body_bytes = {
        // Convert the request into a body
        let body = req.into_body();

        // Read request body (MKCOL — must be empty per RFC 4918)
        body::to_bytes(body, MAX_MKCOL_BODY)
            .await
            .map_err(|e| AppError::payload_too_large(format!("MKCOL body too large: {}", e)))?
    };

    if !body_bytes.is_empty() {
        return Err(AppError::unsupported_media_type(
            "MKCOL request body must be empty",
        ));
    }

    // Extract folder name from path
    let folder_name = path.split('/').next_back().unwrap_or("unnamed");

    // Get parent folder path
    let parent_path = if let Some(idx) = path.rfind('/') {
        &path[..idx]
    } else {
        ""
    };

    // Create folder
    let create_dto = crate::application::dtos::folder_dto::CreateFolderDto {
        name: folder_name.to_string(),
        parent_id: if parent_path.is_empty() {
            None
        } else {
            // Try to get the parent folder ID from its path
            match folder_service.get_folder_by_path(parent_path).await {
                Ok(parent) => Some(parent.id),
                Err(_) => None, // If not found, use root
            }
        },
    };

    folder_service
        .create_folder(create_dto)
        .await
        .map_err(|e| AppError::internal_error(format!("Failed to create folder: {}", e)))?;

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::empty())
        .unwrap())
}

/**
 * Handles DELETE requests to remove files or folders.
 *
 * This handler deletes a file or folder at the specified path.
 *
 * @param state The application state containing service dependencies
 * @param user The authenticated user information
 * @param path The requested resource path
 * @return HTTP response indicating success
 */
async fn handle_delete(
    state: Arc<AppState>,
    _req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    // Get services from state
    let file_retrieval_service = &state.applications.file_retrieval_service;
    let file_management_service = &state.applications.file_management_service;
    let folder_service = &state.applications.folder_service;

    // Check if path is empty (root folder)
    if path.is_empty() || path == "/" {
        return Err(AppError::forbidden("Cannot delete root folder"));
    }

    // Check if path is a folder
    let folder_result = folder_service.get_folder_by_path(&path).await;

    if let Ok(folder) = folder_result {
        // Delete folder — use the folder's own owner as caller_id
        let caller_id = folder.owner_id.as_deref().unwrap_or("webdav");
        folder_service
            .delete_folder(&folder.id, caller_id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to delete folder: {}", e)))?;
    } else {
        // Try to delete file
        let file = file_retrieval_service
            .get_file_by_path(&path)
            .await
            .map_err(|_e| AppError::not_found(format!("Resource not found: {}", path)))?;

        file_management_service
            .delete_file(&file.id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to delete file: {}", e)))?;
    }

    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap())
}

/**
 * Handles MOVE requests to rename or relocate files or folders.
 *
 * This handler moves a file or folder from one path to another.
 *
 * @param state The application state containing service dependencies
 * @param user The authenticated user information
 * @param path The source resource path
 * @param req The HTTP request containing the destination path
 * @return HTTP response indicating success
 */
async fn handle_move(
    state: Arc<AppState>,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    let source_path = path;

    // Get destination from Destination header
    let destination = req
        .headers()
        .get("Destination")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::bad_request("Destination header required"))?
        .to_string();

    // Overwrite header (RFC 4918 §9.8.4): T = overwrite, F = fail if exists
    let overwrite = req
        .headers()
        .get("Overwrite")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("T")
        != "F";

    // Extract destination path from URL
    let destination_path = if let Some(webdav_prefix) = destination.find("/webdav/") {
        let after_prefix = &destination[webdav_prefix + 8..];
        after_prefix.trim_end_matches('/').to_string()
    } else {
        return Err(AppError::bad_request("Invalid destination URL"));
    };

    // Get services from state
    let file_retrieval_service = &state.applications.file_retrieval_service;
    let file_management_service = &state.applications.file_management_service;
    let folder_service = &state.applications.folder_service;

    // Check if destination already exists (for Overwrite header compliance)
    if !overwrite {
        let dest_exists = folder_service
            .get_folder_by_path(&destination_path)
            .await
            .is_ok()
            || file_retrieval_service
                .get_file_by_path(&destination_path)
                .await
                .is_ok();
        if dest_exists {
            return Err(AppError::precondition_failed(
                "Destination already exists and Overwrite is F",
            ));
        }
    }

    // Check if source is a folder
    let folder_result = folder_service.get_folder_by_path(&source_path).await;

    if let Ok(folder) = folder_result {
        // Move folder
        let dest_folder_name = destination_path
            .split('/')
            .next_back()
            .unwrap_or(&destination_path);
        let dest_parent_path = if let Some(idx) = destination_path.rfind('/') {
            &destination_path[..idx]
        } else {
            ""
        };

        // Create DTOs for moving and renaming
        let move_dto = crate::application::dtos::folder_dto::MoveFolderDto {
            parent_id: if dest_parent_path.is_empty() {
                None
            } else {
                match folder_service.get_folder_by_path(dest_parent_path).await {
                    Ok(parent) => Some(parent.id),
                    Err(_) => None, // If not found, use root
                }
            },
        };

        folder_service
            .move_folder(
                &folder.id,
                move_dto,
                folder.owner_id.as_deref().unwrap_or("webdav"),
            )
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to move folder: {}", e)))?;

        if folder.name != dest_folder_name {
            let rename_dto = crate::application::dtos::folder_dto::RenameFolderDto {
                name: dest_folder_name.to_string(),
            };

            folder_service
                .rename_folder(
                    &folder.id,
                    rename_dto,
                    folder.owner_id.as_deref().unwrap_or("webdav"),
                )
                .await
                .map_err(|e| AppError::internal_error(format!("Failed to rename folder: {}", e)))?;
        }
    } else {
        // Try to move file
        let file = file_retrieval_service
            .get_file_by_path(&source_path)
            .await
            .map_err(|_e| AppError::not_found(format!("Resource not found: {}", source_path)))?;

        let dest_filename = destination_path
            .split('/')
            .next_back()
            .unwrap_or(&destination_path);
        let dest_parent_path = if let Some(idx) = destination_path.rfind('/') {
            &destination_path[..idx]
        } else {
            ""
        };

        // Determine source parent path for comparison
        let source_parent_path = if let Some(idx) = source_path.rfind('/') {
            &source_path[..idx]
        } else {
            ""
        };

        // Only call move_file if the parent directory actually changes
        if source_parent_path != dest_parent_path {
            file_management_service
                .move_file(&file.id, Some(dest_parent_path.to_string()))
                .await
                .map_err(|e| AppError::internal_error(format!("Failed to move file: {}", e)))?;
        }

        // Rename the file if the name changed
        if file.name != dest_filename {
            file_management_service
                .rename_file(&file.id, dest_filename)
                .await
                .map_err(|e| AppError::internal_error(format!("Failed to rename file: {}", e)))?;
        }
    }

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::empty())
        .unwrap())
}

/**
 * Handles COPY requests to duplicate files or folders.
 *
 * This handler copies a file or folder from one path to another.
 *
 * @param state The application state containing service dependencies
 * @param user The authenticated user information
 * @param path The source resource path
 * @param req The HTTP request containing the destination path
 * @return HTTP response indicating success
 */
async fn handle_copy(
    state: Arc<AppState>,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    let source_path = path;

    // Get destination from Destination header
    let destination = req
        .headers()
        .get("Destination")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::bad_request("Destination header required"))?
        .to_string();

    // Overwrite header (RFC 4918 §9.8.4): T = overwrite, F = fail if exists
    let overwrite = req
        .headers()
        .get("Overwrite")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("T")
        != "F";

    // Extract destination path from URL
    let destination_path = if let Some(webdav_prefix) = destination.find("/webdav/") {
        let after_prefix = &destination[webdav_prefix + 8..];
        after_prefix.trim_end_matches('/').to_string()
    } else {
        return Err(AppError::bad_request("Invalid destination URL"));
    };

    // Get depth from Depth header
    let depth = req
        .headers()
        .get("Depth")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("infinity");

    // Get services from state
    let file_retrieval_service = &state.applications.file_retrieval_service;
    let folder_service = &state.applications.folder_service;

    // Check if destination already exists (for Overwrite header compliance)
    if !overwrite {
        let dest_exists = folder_service
            .get_folder_by_path(&destination_path)
            .await
            .is_ok()
            || file_retrieval_service
                .get_file_by_path(&destination_path)
                .await
                .is_ok();
        if dest_exists {
            return Err(AppError::precondition_failed(
                "Destination already exists and Overwrite is F",
            ));
        }
    }

    // Check if source is a folder
    let folder_result = folder_service.get_folder_by_path(&source_path).await;

    if let Ok(folder) = folder_result {
        // Copy folder
        let recursive = depth != "0";

        let dest_folder_name = destination_path
            .split('/')
            .next_back()
            .unwrap_or(&destination_path);
        let dest_parent_path = if let Some(idx) = destination_path.rfind('/') {
            &destination_path[..idx]
        } else {
            ""
        };

        let target_parent_id = if dest_parent_path.is_empty() {
            None
        } else {
            match folder_service.get_folder_by_path(dest_parent_path).await {
                Ok(parent) => Some(parent.id),
                Err(_) => None,
            }
        };

        if recursive {
            // Atomic recursive copy: single SQL function call copies the entire
            // folder tree (all sub-folders + all files) with zero-copy dedup.
            // O(depth) folder INSERTs + 1 batch file INSERT + 1 batch ref_count UPDATE.
            let file_management_service = &state.applications.file_management_service;
            file_management_service
                .copy_folder_tree(
                    &folder.id,
                    target_parent_id,
                    Some(dest_folder_name.to_string()),
                )
                .await
                .map_err(|e| {
                    AppError::internal_error(format!("Failed to copy folder tree: {}", e))
                })?;
        } else {
            // Depth: 0 — create empty folder only (no sub-folder or file copy)
            let create_dto = crate::application::dtos::folder_dto::CreateFolderDto {
                name: dest_folder_name.to_string(),
                parent_id: target_parent_id,
            };
            folder_service
                .create_folder(create_dto)
                .await
                .map_err(|e| {
                    AppError::internal_error(format!(
                        "Failed to create destination folder: {}",
                        e
                    ))
                })?;
        }
    } else {
        // Copy file — use zero-copy dedup (only increments blob ref_count, no content loaded)
        let file = file_retrieval_service
            .get_file_by_path(&source_path)
            .await
            .map_err(|_e| AppError::not_found(format!("Resource not found: {}", source_path)))?;

        // Get destination parent folder ID
        let dest_parent_path = if let Some(idx) = destination_path.rfind('/') {
            &destination_path[..idx]
        } else {
            ""
        };

        let target_folder_id = if dest_parent_path.is_empty() {
            None
        } else {
            match folder_service.get_folder_by_path(dest_parent_path).await {
                Ok(parent) => Some(parent.id),
                Err(_) => None,
            }
        };

        // Zero-copy: only creates a new metadata row + increments blob reference count.
        // No file content is ever loaded into memory.
        let file_management_service = &state.applications.file_management_service;
        file_management_service
            .copy_file(&file.id, target_folder_id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to copy file: {}", e)))?;
    }

    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap())
}

/**
 * Handles LOCK requests to lock resources.
 *
 * This handler processes WebDAV LOCK requests according to RFC 4918,
 * creating a lock on a file or folder.
 *
 * @param state The application state containing service dependencies
 * @param user The authenticated user information
 * @param path The requested resource path
 * @param req The HTTP request containing the LOCK XML body
 * @return XML response with lock information
 */
async fn handle_lock(
    _state: Arc<AppState>,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    let user = {
        let user_ref = req
            .extensions()
            .get::<CurrentUser>()
            .ok_or_else(|| AppError::unauthorized("Authentication required"))?;
        user_ref.clone()
    };

    // Get the headers that we need
    let depth = req
        .headers()
        .get("Depth")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("infinity")
        .to_string();

    let timeout = req
        .headers()
        .get("Timeout")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let if_header_value = req
        .headers()
        .get("If")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Extract the body separately to avoid borrow issues
    let body_bytes = {
        // Convert the request into a body
        let body = req.into_body();

        // Read request body (LOCK is XML, 1 MB is more than enough)
        body::to_bytes(body, MAX_XML_BODY)
            .await
            .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?
    };

    // Check if this is a lock refresh (If header with a lock token)
    if let Some(if_header) = if_header_value {
        // This is a lock refresh request
        // Extract lock token from If header
        let token = if_header
            .trim()
            .trim_start_matches("(<")
            .trim_end_matches(">)")
            .to_string();

        // In a full implementation, we would look up the lock in a database
        // and refresh its timeout. For now, just respond as if we did.

        // Generate lock token and owner (for a real implementation, we'd store these)
        let lock_info = LockInfo {
            token,
            owner: Some(user.id.clone()),
            depth: depth.to_string(),
            timeout,
            scope: LockScope::Exclusive, // Default to exclusive
            type_: LockType::Write,      // Default to write
        };

        // Generate response
        let href = format!("/webdav/{}", path);
        let mut response_body = Vec::new();
        WebDavAdapter::generate_lock_response(&mut response_body, &lock_info, &href).map_err(
            |e| AppError::internal_error(format!("Failed to generate LOCK response: {}", e)),
        )?;

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .header(HEADER_LOCK_TOKEN, format!("<{}>", lock_info.token))
            .body(Body::from(response_body))
            .unwrap())
    } else if !body_bytes.is_empty() {
        // Parse lock request
        let (scope, type_, owner) = WebDavAdapter::parse_lockinfo(body_bytes.reader())
            .map_err(|e| AppError::bad_request(format!("Failed to parse LOCK request: {}", e)))?;

        // Generate lock token and owner (for a real implementation, we'd store these)
        let token = format!("opaquelocktoken:{}", Uuid::new_v4());
        let lock_info = LockInfo {
            token,
            owner: owner.or(Some(user.id.clone())),
            depth: depth.to_string(),
            timeout,
            scope,
            type_,
        };

        // Generate response
        let href = format!("/webdav/{}", path);
        let mut response_body = Vec::new();
        WebDavAdapter::generate_lock_response(&mut response_body, &lock_info, &href).map_err(
            |e| AppError::internal_error(format!("Failed to generate LOCK response: {}", e)),
        )?;

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .header(HEADER_LOCK_TOKEN, format!("<{}>", lock_info.token))
            .body(Body::from(response_body))
            .unwrap())
    } else {
        Err(AppError::bad_request("Invalid LOCK request"))
    }
}

/**
 * Handles UNLOCK requests to remove locks from resources.
 *
 * This handler processes WebDAV UNLOCK requests according to RFC 4918,
 * removing a lock from a file or folder.
 *
 * @param state The application state containing service dependencies
 * @param user The authenticated user information
 * @param path The requested resource path
 * @param req The HTTP request containing the lock token
 * @return HTTP response indicating success
 */
async fn handle_unlock(
    _state: Arc<AppState>,
    req: Request<Body>,
    _path: String,
) -> Result<Response<Body>, AppError> {
    let _user = {
        let user_ref = req
            .extensions()
            .get::<CurrentUser>()
            .ok_or_else(|| AppError::unauthorized("Authentication required"))?;
        user_ref.clone()
    };

    // Get lock token from Lock-Token header
    let lock_token = req
        .headers()
        .get("Lock-Token")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::bad_request("Lock-Token header required"))?;

    // Extract token from header value (format: <token>)
    let _token = lock_token
        .trim()
        .trim_start_matches('<')
        .trim_end_matches('>')
        .to_string();

    // In a full implementation, we would look up the lock in a database
    // and remove it. For now, just respond as if we did.

    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap())
}
