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
use bytes::Buf;
use chrono::Utc;
use uuid::Uuid;

use crate::application::adapters::webdav_adapter::{
    LockInfo, LockScope, LockType, PropFindRequest, WebDavAdapter,
};
use crate::application::dtos::folder_dto::FolderDto;
use crate::common::di::AppState;
use crate::interfaces::errors::AppError;
use crate::interfaces::middleware::auth::CurrentUser;

// Create a custom DAV header since it's not in the standard headers
const HEADER_DAV: HeaderName = HeaderName::from_static("dav");
const HEADER_LOCK_TOKEN: HeaderName = HeaderName::from_static("lock-token");
// const HEADER_IF: HeaderName = HeaderName::from_static("if");

/// Maximum body size for XML-based WebDAV requests (PROPFIND, PROPPATCH, LOCK).
/// 1 MB is generous — a typical PROPFIND body is < 1 KB.
const MAX_XML_BODY: usize = 1_048_576;

/// Maximum body size for MKCOL requests (RFC 4918: body must be empty).
const MAX_MKCOL_BODY: usize = 4096;

/**
 * Creates and returns the WebDAV router with all required endpoints.
 *
 * This function sets up all WebDAV method handlers following RFC 4918,
 * mapping HTTP methods to appropriate WebDAV operations.
 *
 * @return Router configured with WebDAV endpoints
 */
pub fn webdav_routes() -> Router<AppState> {
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
    axum::extract::State(state): axum::extract::State<AppState>,
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    handle_webdav_dispatch(state, req, String::new()).await
}

async fn handle_webdav_methods(
    axum::extract::State(state): axum::extract::State<AppState>,
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    let path = extract_webdav_path(req.uri());
    handle_webdav_dispatch(state, req, path).await
}

async fn handle_webdav_dispatch(
    state: AppState,
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
 * It supports the Depth header to control recursion depth.
 *
 * @param state The application state containing service dependencies
 * @param user The authenticated user information
 * @param path The requested resource path
 * @param req The HTTP request containing the PROPFIND XML body
 * @return XML response with resource properties
 */
async fn handle_propfind(
    state: AppState,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, AppError> {
    // Extract depth header (cloning to avoid borrowing issues)
    let depth = req
        .headers()
        .get("Depth")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("infinity")
        .to_string();

    let _user = {
        let user_ref = req
            .extensions()
            .get::<CurrentUser>()
            .ok_or_else(|| AppError::unauthorized("Authentication required"))?;
        user_ref.clone()
    };

    // Extract the body separately to avoid borrow issues
    let body_bytes = {
        // Convert the request into a body
        let body = req.into_body();

        // Read request body (PROPFIND is XML, 1 MB is more than enough)
        body::to_bytes(body, MAX_XML_BODY)
            .await
            .map_err(|e| AppError::bad_request(format!("Failed to read request body: {}", e)))?
    };

    // Parse PROPFIND request
    let propfind_request = if body_bytes.is_empty() {
        // Empty body means get all properties
        PropFindRequest {
            prop_find_type: crate::application::adapters::webdav_adapter::PropFindType::AllProp,
        }
    } else {
        // Parse XML body
        WebDavAdapter::parse_propfind(body_bytes.reader()).map_err(|e| {
            AppError::bad_request(format!("Failed to parse PROPFIND request: {}", e))
        })?
    };

    // Get folder service from state
    let folder_service = &state.applications.folder_service;
    let file_retrieval_service = &state.applications.file_retrieval_service;

    // Determine base HREF
    let base_href = format!("/webdav/{}/", path);

    // Check if path exists as a file or folder
    if path.is_empty() || path == "/" {
        // Root folder — run both queries concurrently
        let (subfolders_result, files_result) = tokio::join!(
            folder_service.list_folders(None),
            file_retrieval_service.list_files(None)
        );
        let subfolders = subfolders_result
            .map_err(|e| AppError::internal_error(format!("Failed to get subfolders: {}", e)))?;
        let files = files_result
            .map_err(|e| AppError::internal_error(format!("Failed to get files: {}", e)))?;

        // Create root folder DTO for response
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

        // Generate response
        let mut response_body = Vec::new();
        WebDavAdapter::generate_propfind_response(
            &mut response_body,
            Some(&root_folder),
            &files,
            &subfolders,
            &propfind_request,
            &depth,
            &base_href,
        )
        .map_err(|e| {
            AppError::internal_error(format!("Failed to generate PROPFIND response: {}", e))
        })?;

        Ok(Response::builder()
            .status(StatusCode::MULTI_STATUS)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .body(Body::from(response_body))
            .unwrap())
    } else {
        // Check if path is a folder
        let folder_result = folder_service.get_folder_by_path(&path).await;

        if let Ok(folder) = folder_result {
            // Path is a folder — run both queries concurrently
            let (files, subfolders) = if depth != "0" {
                let (files_r, folders_r) = tokio::join!(
                    file_retrieval_service.list_files(Some(&folder.id)),
                    folder_service.list_folders(Some(&folder.id))
                );
                (
                    files_r.map_err(|e| AppError::internal_error(format!("Failed to get files: {}", e)))?,
                    folders_r.map_err(|e| AppError::internal_error(format!("Failed to get subfolders: {}", e)))?,
                )
            } else {
                (vec![], vec![])
            };

            // Generate response
            let mut response_body = Vec::new();
            WebDavAdapter::generate_propfind_response(
                &mut response_body,
                Some(&folder),
                &files,
                &subfolders,
                &propfind_request,
                &depth,
                &base_href,
            )
            .map_err(|e| {
                AppError::internal_error(format!("Failed to generate PROPFIND response: {}", e))
            })?;

            Ok(Response::builder()
                .status(StatusCode::MULTI_STATUS)
                .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
                .body(Body::from(response_body))
                .unwrap())
        } else {
            // Check if path is a file
            let file_result = file_retrieval_service.get_file_by_path(&path).await;

            if let Ok(file) = file_result {
                // Path is a file
                let mut response_body = Vec::new();
                WebDavAdapter::generate_propfind_response_for_file(
                    &mut response_body,
                    &file,
                    &propfind_request,
                    &depth,
                    &base_href,
                )
                .map_err(|e| {
                    AppError::internal_error(format!("Failed to generate PROPFIND response: {}", e))
                })?;

                Ok(Response::builder()
                    .status(StatusCode::MULTI_STATUS)
                    .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
                    .body(Body::from(response_body))
                    .unwrap())
            } else {
                // Path does not exist
                Err(AppError::not_found(format!("Resource not found: {}", path)))
            }
        }
    }
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
    _state: AppState,
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
    state: AppState,
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
    state: AppState,
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
    state: AppState,
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
    state: AppState,
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
    state: AppState,
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
    state: AppState,
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
    state: AppState,
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
    let _file_upload_service = &state.applications.file_upload_service;
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

        // For now, just create a new folder and copy files individually
        // In a real implementation, we would have a dedicated copy_folder service method
        let create_dto = crate::application::dtos::folder_dto::CreateFolderDto {
            name: dest_folder_name.to_string(),
            parent_id: if dest_parent_path.is_empty() {
                None
            } else {
                // Try to get the parent folder ID from its path
                match folder_service.get_folder_by_path(dest_parent_path).await {
                    Ok(parent) => Some(parent.id),
                    Err(_) => None, // If not found, use root
                }
            },
        };

        let _new_folder = folder_service
            .create_folder(create_dto)
            .await
            .map_err(|e| {
                AppError::internal_error(format!("Failed to create destination folder: {}", e))
            })?;

        if recursive {
            // Copy files via zero-copy dedup (only increments blob ref_count)
            let files = file_retrieval_service
                .list_files(Some(&folder.id))
                .await
                .map_err(|e| AppError::internal_error(format!("Failed to list files: {}", e)))?;

            let file_management_service = &state.applications.file_management_service;
            let new_folder_id = Some(_new_folder.id.clone());
            for file in files {
                file_management_service
                    .copy_file(&file.id, new_folder_id.clone())
                    .await
                    .map_err(|e| {
                        AppError::internal_error(format!(
                            "Failed to copy file {}: {}",
                            file.name, e
                        ))
                    })?;
            }
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
    _state: AppState,
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
    _state: AppState,
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
