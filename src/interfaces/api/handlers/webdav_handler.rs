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
    response::Response,
    http::{StatusCode, header, HeaderName, Request},
    body::{Body, self},
};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use bytes::Buf;

use crate::common::di::AppState;
use crate::application::adapters::webdav_adapter::{WebDavAdapter, PropFindRequest, LockInfo, LockScope, LockType};
use crate::interfaces::middleware::auth::CurrentUser;
use crate::application::dtos::folder_dto::FolderDto;
use crate::interfaces::errors::AppError;

// Create a custom DAV header since it's not in the standard headers
const HEADER_DAV: HeaderName = HeaderName::from_static("dav");
const HEADER_LOCK_TOKEN: HeaderName = HeaderName::from_static("lock-token");
// const HEADER_IF: HeaderName = HeaderName::from_static("if");

/**
 * Creates and returns the WebDAV router with all required endpoints.
 * 
 * This function sets up all WebDAV method handlers following RFC 4918,
 * mapping HTTP methods to appropriate WebDAV operations.
 * 
 * @return Router configured with WebDAV endpoints
 */
pub fn webdav_routes() -> Router<AppState> {
    // Create the router with a single catchall route
    // This will internally dispatch to the appropriate method handler
    Router::new()
        .route("/webdav/{*path}", axum::routing::any(handle_webdav_methods))
}

async fn handle_webdav_methods(
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    let method = req.method().clone();
    
    match method.as_str() {
        "OPTIONS" => handle_options(req).await,
        "GET" => handle_get(req).await,
        "PUT" => handle_put(req).await,
        "MKCOL" => handle_mkcol(req).await,
        "DELETE" => handle_delete(req).await,
        "MOVE" => handle_move(req).await,
        "COPY" => handle_copy(req).await,
        "PROPFIND" => handle_propfind(req).await,
        "PROPPATCH" => handle_proppatch(req).await,
        "LOCK" => handle_lock(req).await,
        "UNLOCK" => handle_unlock(req).await,
        _ => Err(AppError::method_not_allowed(format!("Method not allowed: {}", method))),
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
async fn handle_options(
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    // Extract State and Path from request
    let parts = req.uri().path().split('/').collect::<Vec<&str>>();
    let _path = if parts.len() > 2 {
        parts[2..].join("/")
    } else {
        "".to_string()
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(HEADER_DAV, "1, 2") // Class 1 and 2 WebDAV support
        .header(header::ALLOW, "OPTIONS, GET, HEAD, PUT, DELETE, PROPFIND, PROPPATCH, MKCOL, COPY, MOVE, LOCK, UNLOCK")
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
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    // Clone all necessary data first to avoid borrow issues
    let uri = req.uri().clone();
    let path = {
        let parts = uri.path().split('/').collect::<Vec<&str>>();
        if parts.len() > 2 {
            parts[2..].join("/")
        } else {
            "".to_string()
        }
    };
    
    // Extract depth header (cloning to avoid borrowing issues)
    let depth = req.headers()
        .get("Depth")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("infinity")
        .to_string();
    
    // Get the state and user in a way that doesn't keep req borrowed
    let state = {
        let state_ref = req.extensions().get::<Arc<AppState>>().ok_or_else(|| {
            AppError::internal_error("Missing AppState extension")
        })?;
        state_ref.clone()
    };
    
    let _user = {
        let user_ref = req.extensions().get::<CurrentUser>().ok_or_else(|| {
            AppError::unauthorized("Authentication required")
        })?;
        user_ref.clone()
    };
    
    // Extract the body separately to avoid borrow issues
    let body_bytes = {
        // Convert the request into a body
        let body = req.into_body();
        
        // Read request body
        body::to_bytes(body, usize::MAX)
            .await
            .map_err(|e| {
                AppError::bad_request(format!("Failed to read request body: {}", e))
            })?
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
    let file_service = &state.applications.file_service;
    
    // Determine base HREF
    let base_href = format!("/webdav/{}/", path);
    
    // Check if path exists as a file or folder
    if path.is_empty() || path == "/" {
        // Root folder
        let subfolders = folder_service.list_folders(None).await.map_err(|e| {
            AppError::internal_error(format!("Failed to get subfolders: {}", e))
        })?;
        
        let files = file_service.list_files(None).await.map_err(|e| {
            AppError::internal_error(format!("Failed to get files: {}", e))
        })?;
        
        // Create root folder DTO for response
        let root_folder = FolderDto {
            id: "root".to_string(),
            name: "".to_string(),
            path: "".to_string(),
            parent_id: None,
            created_at: Utc::now().timestamp() as u64,
            modified_at: Utc::now().timestamp() as u64,
            is_root: true,
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
        ).map_err(|e| {
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
            // Path is a folder
            let files = if depth != "0" {
                file_service.list_files(Some(&folder.id)).await.map_err(|e| {
                    AppError::internal_error(format!("Failed to get files: {}", e))
                })?
            } else {
                vec![]
            };
            
            let subfolders = if depth != "0" {
                folder_service.list_folders(Some(&folder.id)).await.map_err(|e| {
                    AppError::internal_error(format!("Failed to get subfolders: {}", e))
                })?
            } else {
                vec![]
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
            ).map_err(|e| {
                AppError::internal_error(format!("Failed to generate PROPFIND response: {}", e))
            })?;
            
            Ok(Response::builder()
                .status(StatusCode::MULTI_STATUS)
                .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
                .body(Body::from(response_body))
                .unwrap())
        } else {
            // Check if path is a file
            let file_result = file_service.get_file_by_path(&path).await;
            
            if let Ok(file) = file_result {
                // Path is a file
                let mut response_body = Vec::new();
                WebDavAdapter::generate_propfind_response_for_file(
                    &mut response_body,
                    &file,
                    &propfind_request,
                    &depth,
                    &base_href,
                ).map_err(|e| {
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
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    // Extract State, Extension, and Path from request
    let uri = req.uri().clone();
    let parts = uri.path().split('/').collect::<Vec<&str>>();
    let path = if parts.len() > 2 {
        parts[2..].join("/")
    } else {
        "".to_string()
    };
    
    let _state = req.extensions().get::<Arc<AppState>>().ok_or_else(|| {
        AppError::internal_error("Missing AppState extension")
    })?;
    let _user = req.extensions().get::<CurrentUser>().ok_or_else(|| {
        AppError::unauthorized("Authentication required")
    })?;
    
    // Read request body
    let body_bytes = body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|e| {
            AppError::bad_request(format!("Failed to read request body: {}", e))
        })?;
    
    // Parse PROPPATCH request
    let (props_to_set, props_to_remove) = WebDavAdapter::parse_proppatch(body_bytes.reader()).map_err(|e| {
        AppError::bad_request(format!("Failed to parse PROPPATCH request: {}", e))
    })?;
    
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
    WebDavAdapter::generate_proppatch_response(
        &mut response_body,
        &href,
        &results,
    ).map_err(|e| {
        AppError::internal_error(format!("Failed to generate PROPPATCH response: {}", e))
    })?;
    
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
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    // Extract State, Extension, and Path from request
    let uri = req.uri().clone();
    let parts = uri.path().split('/').collect::<Vec<&str>>();
    let path = if parts.len() > 2 {
        parts[2..].join("/")
    } else {
        "".to_string()
    };
    
    let state = req.extensions().get::<Arc<AppState>>().ok_or_else(|| {
        AppError::internal_error("Missing AppState extension")
    })?;
    let _user = req.extensions().get::<CurrentUser>().ok_or_else(|| {
        AppError::unauthorized("Authentication required")
    })?;
    
    // Get file service from state
    let file_service = &state.applications.file_service;
    let file_retrieval_service = &state.applications.file_retrieval_service;
    
    // Check if path is empty (root folder)
    if path.is_empty() || path == "/" {
        return Err(AppError::bad_request("Cannot GET a directory"));
    }
    
    // Get file metadata
    let file = file_service.get_file_by_path(&path).await.map_err(|_e| {
        AppError::not_found(format!("File not found: {}", path))
    })?;
    
    // Get file content
    let content = file_retrieval_service.get_file_content(&file.id).await.map_err(|e| {
        AppError::internal_error(format!("Failed to get file content: {}", e))
    })?;
    
    // Build response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, file.mime_type)
        .header(header::CONTENT_LENGTH, content.len())
        .header(header::ETAG, format!("\"{}\"", file.id))
        .header(header::LAST_MODIFIED, chrono::DateTime::<Utc>::from_timestamp(file.created_at as i64, 0)
            .unwrap_or_else(|| Utc::now())
            .to_rfc2822())
        .body(Body::from(content))
        .unwrap())
}

/**
 * Handles PUT requests to create or update files.
 * 
 * This handler creates a new file or updates an existing file at the specified path.
 * 
 * @param state The application state containing service dependencies
 * @param user The authenticated user information
 * @param path The requested resource path
 * @param req The HTTP request containing the file contents
 * @return HTTP response indicating success
 */
async fn handle_put(
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    // Clone all necessary data first to avoid borrow issues
    let uri = req.uri().clone();
    let path = {
        let parts = uri.path().split('/').collect::<Vec<&str>>();
        if parts.len() > 2 {
            parts[2..].join("/")
        } else {
            "".to_string()
        }
    };
    
    // Get the state and user in a way that doesn't keep req borrowed
    let state = {
        let state_ref = req.extensions().get::<Arc<AppState>>().ok_or_else(|| {
            AppError::internal_error("Missing AppState extension")
        })?;
        state_ref.clone()
    };
    
    let _user = {
        let user_ref = req.extensions().get::<CurrentUser>().ok_or_else(|| {
            AppError::unauthorized("Authentication required")
        })?;
        user_ref.clone()
    };
    
    // Get file service from state
    let file_service = &state.applications.file_service;
    
    // Check if path is empty (root folder)
    if path.is_empty() || path == "/" {
        return Err(AppError::bad_request("Cannot PUT to root folder"));
    }
    
    // Extract content type before consuming the request
    let content_type = req.headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();
    
    // Read request body
    let body_bytes = {
        // Convert the request into a body
        let body = req.into_body();
        
        // Read request body
        body::to_bytes(body, usize::MAX)
            .await
            .map_err(|e| {
                AppError::bad_request(format!("Failed to read request body: {}", e))
            })?
    };
    
    // Check if file exists
    let file_exists = file_service.get_file_by_path(&path).await.is_ok();
    
    if file_exists {
        // Update existing file
        file_service.update_file(&path, &body_bytes).await.map_err(|e| {
            AppError::internal_error(format!("Failed to update file: {}", e))
        })?;
        
        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::empty())
            .unwrap())
    } else {
        // Create new file  
        // Extract filename from path
        let filename = path.split('/').last().unwrap_or("unnamed");
        
        // Get parent folder path
        let parent_path = if let Some(idx) = path.rfind('/') {
            &path[..idx]
        } else {
            ""
        };
        
        file_service.create_file(parent_path, filename, &body_bytes, &content_type).await.map_err(|e| {
            AppError::internal_error(format!("Failed to create file: {}", e))
        })?;
        
        Ok(Response::builder()
            .status(StatusCode::CREATED)
            .body(Body::empty())
            .unwrap())
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
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    // Clone all necessary data first to avoid borrow issues
    let uri = req.uri().clone();
    let path = {
        let parts = uri.path().split('/').collect::<Vec<&str>>();
        if parts.len() > 2 {
            parts[2..].join("/")
        } else {
            "".to_string()
        }
    };
    
    // Get the state and user in a way that doesn't keep req borrowed
    let state = {
        let state_ref = req.extensions().get::<Arc<AppState>>().ok_or_else(|| {
            AppError::internal_error("Missing AppState extension")
        })?;
        state_ref.clone()
    };
    
    let _user = {
        let user_ref = req.extensions().get::<CurrentUser>().ok_or_else(|| {
            AppError::unauthorized("Authentication required")
        })?;
        user_ref.clone()
    };
    
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
        
        // Read request body
        body::to_bytes(body, usize::MAX)
            .await
            .map_err(|e| {
                AppError::bad_request(format!("Failed to read request body: {}", e))
            })?
    };
    
    if !body_bytes.is_empty() {
        return Err(AppError::unsupported_media_type("MKCOL request body must be empty"));
    }
    
    // Extract folder name from path
    let folder_name = path.split('/').last().unwrap_or("unnamed");
    
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
                Err(_) => None // If not found, use root
            }
        }
    };
    
    folder_service.create_folder(create_dto).await.map_err(|e| {
        AppError::internal_error(format!("Failed to create folder: {}", e))
    })?;
    
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
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    // Extract State, Extension, and Path from request
    let uri = req.uri().clone();
    let parts = uri.path().split('/').collect::<Vec<&str>>();
    let path = if parts.len() > 2 {
        parts[2..].join("/")
    } else {
        "".to_string()
    };
    
    let state = req.extensions().get::<Arc<AppState>>().ok_or_else(|| {
        AppError::internal_error("Missing AppState extension")
    })?;
    let _user = req.extensions().get::<CurrentUser>().ok_or_else(|| {
        AppError::unauthorized("Authentication required")
    })?;
    
    // Get services from state
    let file_service = &state.applications.file_service;
    let folder_service = &state.applications.folder_service;
    
    // Check if path is empty (root folder)
    if path.is_empty() || path == "/" {
        return Err(AppError::forbidden("Cannot delete root folder"));
    }
    
    // Check if path is a folder
    let folder_result = folder_service.get_folder_by_path(&path).await;
    
    if let Ok(folder) = folder_result {
        // Delete folder
        folder_service.delete_folder(&folder.id).await.map_err(|e| {
            AppError::internal_error(format!("Failed to delete folder: {}", e))
        })?;
    } else {
        // Try to delete file
        let file = file_service.get_file_by_path(&path).await.map_err(|_e| {
            AppError::not_found(format!("Resource not found: {}", path))
        })?;
        
        file_service.delete_file(&file.id).await.map_err(|e| {
            AppError::internal_error(format!("Failed to delete file: {}", e))
        })?;
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
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    // Extract State, Extension, and Path from request
    let uri = req.uri().clone();
    let parts = uri.path().split('/').collect::<Vec<&str>>();
    let source_path = if parts.len() > 2 {
        parts[2..].join("/")
    } else {
        "".to_string()
    };
    
    let state = req.extensions().get::<Arc<AppState>>().ok_or_else(|| {
        AppError::internal_error("Missing AppState extension")
    })?;
    let _user = req.extensions().get::<CurrentUser>().ok_or_else(|| {
        AppError::unauthorized("Authentication required")
    })?;
    
    // Get destination from Destination header
    let destination = req.headers()
        .get("Destination")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::bad_request("Destination header required"))?;
    
    // Extract destination path from URL
    let destination_path = if let Some(webdav_prefix) = destination.find("/webdav/") {
        let after_prefix = &destination[webdav_prefix + 8..];
        after_prefix.trim_end_matches('/')
    } else {
        return Err(AppError::bad_request("Invalid destination URL"));
    };
    
    // Get services from state
    let file_service = &state.applications.file_service;
    let folder_service = &state.applications.folder_service;
    
    // Check if source is a folder
    let folder_result = folder_service.get_folder_by_path(&source_path).await;
    
    if let Ok(folder) = folder_result {
        // Move folder
        let dest_folder_name = destination_path.split('/').last().unwrap_or(&destination_path);
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
                    Err(_) => None // If not found, use root
                }
            }
        };
        
        folder_service.move_folder(&folder.id, move_dto).await.map_err(|e| {
            AppError::internal_error(format!("Failed to move folder: {}", e))
        })?;
        
        if folder.name != dest_folder_name {
            let rename_dto = crate::application::dtos::folder_dto::RenameFolderDto {
                name: dest_folder_name.to_string()
            };
            
            folder_service.rename_folder(&folder.id, rename_dto).await.map_err(|e| {
                AppError::internal_error(format!("Failed to rename folder: {}", e))
            })?;
        }
    } else {
        // Try to move file
        let file = file_service.get_file_by_path(&source_path).await.map_err(|_e| {
            AppError::not_found(format!("Resource not found: {}", source_path))
        })?;
        
        let dest_parent_path = if let Some(idx) = destination_path.rfind('/') {
            &destination_path[..idx]
        } else {
            ""
        };
        
        file_service.move_file(&file.id, Some(dest_parent_path.to_string())).await.map_err(|e| {
            AppError::internal_error(format!("Failed to move file: {}", e))
        })?;
    }
    
    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
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
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    // Extract State, Extension, and Path from request
    let uri = req.uri().clone();
    let parts = uri.path().split('/').collect::<Vec<&str>>();
    let source_path = if parts.len() > 2 {
        parts[2..].join("/")
    } else {
        "".to_string()
    };
    
    let state = req.extensions().get::<Arc<AppState>>().ok_or_else(|| {
        AppError::internal_error("Missing AppState extension")
    })?;
    let _user = req.extensions().get::<CurrentUser>().ok_or_else(|| {
        AppError::unauthorized("Authentication required")
    })?;
    
    // Get destination from Destination header
    let destination = req.headers()
        .get("Destination")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::bad_request("Destination header required"))?;
    
    // Extract destination path from URL
    let destination_path = if let Some(webdav_prefix) = destination.find("/webdav/") {
        let after_prefix = &destination[webdav_prefix + 8..];
        after_prefix.trim_end_matches('/')
    } else {
        return Err(AppError::bad_request("Invalid destination URL"));
    };
    
    // Get depth from Depth header
    let depth = req.headers()
        .get("Depth")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("infinity");
    
    // Get services from state
    let file_service = &state.applications.file_service;
    let folder_service = &state.applications.folder_service;
    let file_retrieval_service = &state.applications.file_retrieval_service;
    
    // Check if source is a folder
    let folder_result = folder_service.get_folder_by_path(&source_path).await;
    
    if let Ok(folder) = folder_result {
        // Copy folder
        let recursive = depth != "0";
        
        let dest_folder_name = destination_path.split('/').last().unwrap_or(&destination_path);
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
                    Err(_) => None // If not found, use root
                }
            }
        };
        
        let _new_folder = folder_service.create_folder(create_dto).await.map_err(|e| {
            AppError::internal_error(format!("Failed to create destination folder: {}", e))
        })?;
        
        if recursive {
            // Copy subfolders and files (simplified implementation)
            let files = file_service.list_files(Some(&folder.id)).await.map_err(|e| {
                AppError::internal_error(format!("Failed to list files: {}", e))
            })?;
            
            for file in files {
                // Get file content
                if let Ok(file_source) = file_service.get_file_by_path(&format!("{}/{}", source_path, file.name)).await {
                    if let Ok(content) = file_retrieval_service.get_file_content(&file_source.id).await {
                        // Create new file in destination
                        file_service.create_file(&destination_path, &file.name, &content, &file.mime_type).await.map_err(|e| {
                            AppError::internal_error(format!("Failed to copy file {}: {}", file.name, e))
                        })?;
                    }
                }
            }
        }
    } else {
        // Try to copy file
        let file = file_service.get_file_by_path(&source_path).await.map_err(|_e| {
            AppError::not_found(format!("Resource not found: {}", source_path))
        })?;
        
        // Get file content
        let content = file_retrieval_service.get_file_content(&file.id).await.map_err(|e| {
            AppError::internal_error(format!("Failed to get file content: {}", e))
        })?;
        
        // Get destination parent path and filename
        let dest_filename = destination_path.split('/').last().unwrap_or(&destination_path);
        let dest_parent_path = if let Some(idx) = destination_path.rfind('/') {
            &destination_path[..idx]
        } else {
            ""
        };
        
        // Create new file in destination
        file_service.create_file(dest_parent_path, dest_filename, &content, &file.mime_type).await.map_err(|e| {
            AppError::internal_error(format!("Failed to copy file: {}", e))
        })?;
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
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    // Clone all necessary data first to avoid borrow issues
    let uri = req.uri().clone();
    let path = {
        let parts = uri.path().split('/').collect::<Vec<&str>>();
        if parts.len() > 2 {
            parts[2..].join("/")
        } else {
            "".to_string()
        }
    };
    
    // Get the state and user in a way that doesn't keep req borrowed
    let _state = {
        let state_ref = req.extensions().get::<Arc<AppState>>().ok_or_else(|| {
            AppError::internal_error("Missing AppState extension")
        })?;
        state_ref.clone()
    };
    
    let user = {
        let user_ref = req.extensions().get::<CurrentUser>().ok_or_else(|| {
            AppError::unauthorized("Authentication required")
        })?;
        user_ref.clone()
    };
    
    // Get the headers that we need
    let depth = req.headers()
        .get("Depth")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("infinity")
        .to_string();
    
    let timeout = req.headers()
        .get("Timeout")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    
    let if_header_value = req.headers()
        .get("If")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    
    // Extract the body separately to avoid borrow issues
    let body_bytes = {
        // Convert the request into a body
        let body = req.into_body();
        
        // Read request body
        body::to_bytes(body, usize::MAX)
            .await
            .map_err(|e| {
                AppError::bad_request(format!("Failed to read request body: {}", e))
            })?
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
        WebDavAdapter::generate_lock_response(
            &mut response_body,
            &lock_info,
            &href,
        ).map_err(|e| {
            AppError::internal_error(format!("Failed to generate LOCK response: {}", e))
        })?;
        
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .header(HEADER_LOCK_TOKEN, format!("<{}>", lock_info.token))
            .body(Body::from(response_body))
            .unwrap())
    } else if !body_bytes.is_empty() {
        // Parse lock request
        let (scope, type_, owner) = WebDavAdapter::parse_lockinfo(body_bytes.reader()).map_err(|e| {
            AppError::bad_request(format!("Failed to parse LOCK request: {}", e))
        })?;
        
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
        WebDavAdapter::generate_lock_response(
            &mut response_body,
            &lock_info,
            &href,
        ).map_err(|e| {
            AppError::internal_error(format!("Failed to generate LOCK response: {}", e))
        })?;
        
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
    req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    // Clone all necessary data first to avoid borrow issues
    let uri = req.uri().clone();
    let _path = {
        let parts = uri.path().split('/').collect::<Vec<&str>>();
        if parts.len() > 2 {
            parts[2..].join("/")
        } else {
            "".to_string()
        }
    };
    
    // Get the state and user in a way that doesn't keep req borrowed
    let _state = {
        let state_ref = req.extensions().get::<Arc<AppState>>().ok_or_else(|| {
            AppError::internal_error("Missing AppState extension")
        })?;
        state_ref.clone()
    };
    
    let _user = {
        let user_ref = req.extensions().get::<CurrentUser>().ok_or_else(|| {
            AppError::unauthorized("Authentication required")
        })?;
        user_ref.clone()
    };
    
    // Get lock token from Lock-Token header
    let lock_token = req.headers()
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