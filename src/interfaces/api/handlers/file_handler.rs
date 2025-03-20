use std::sync::Arc;
use axum::{
    extract::{Path, State, Multipart, Query, Extension},
    http::{StatusCode, header, HeaderName, HeaderValue, Response},
    response::IntoResponse,
    Json,
};
use crate::interfaces::middleware::auth::CurrentUser;
use serde::Deserialize;
use std::collections::HashMap;
use futures::Stream;
use std::task::{Context, Poll};
use std::pin::Pin;
use tracing::{debug, error, info, warn};

use crate::application::services::file_service::{FileService, FileServiceError};
use crate::application::services::sharing_service::SharingService;
use crate::domain::entities::shared_file::PermissionLevel;
use crate::infrastructure::services::compression_service::{
    CompressionService, GzipCompressionService, CompressionLevel
};

type AppState = Arc<FileService>;

/// Handler for file-related API endpoints
pub struct FileHandler;

// Simpler approach to make streams Unpin - use Pin<Box<dyn Stream>> directly
struct BoxedStream<T> {
    inner: Pin<Box<dyn Stream<Item = T> + Send + 'static>>,
}

impl<T> Stream for BoxedStream<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Accessing the field directly is safe because BoxedStream is not a structural pinning type
        unsafe { self.get_unchecked_mut().inner.as_mut().poll_next(cx) }
    }
}

// This is safe because BoxedStream's inner field is already Pin<Box<dyn Stream>>
impl<T> Unpin for BoxedStream<T> {}

impl<T> BoxedStream<T> {
    #[allow(dead_code)]
    fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = T> + Send + 'static,
    {
        BoxedStream {
            inner: Box::pin(stream),
        }
    }
}

/// Extended app state with sharing service
pub struct ExtendedAppState {
    file_service: Arc<FileService>,
    sharing_service: Option<Arc<SharingService>>,
}

impl FileHandler {
    /// Check if user has access to a file through ownership or sharing
    async fn check_file_access(
        file_service: &FileService,
        sharing_service: Option<&SharingService>,
        file_id: &str,
        user_id: &str,
        required_permission: Option<PermissionLevel>,
    ) -> bool {
        // First check if user is the owner
        let is_owner = file_service.check_file_access(file_id, user_id).await;
        
        if is_owner {
            return true; // Owner has all permissions
        }
        
        // If not owner, check if file is shared with user
        if let Some(sharing_svc) = sharing_service {
            match sharing_svc.check_user_has_access(file_id, user_id).await {
                Ok(Some(shared_file)) => {
                    // If specific permission is required, check it
                    if let Some(required) = required_permission {
                        match required {
                            PermissionLevel::Read => true, // All permission levels include read
                            PermissionLevel::Write => {
                                // Write or Admin permission required
                                matches!(shared_file, PermissionLevel::Write | PermissionLevel::Admin)
                            },
                            PermissionLevel::Admin => {
                                // Only Admin permission is sufficient
                                matches!(shared_file, PermissionLevel::Admin)
                            }
                        }
                    } else {
                        // No specific permission required, any access is sufficient
                        true
                    }
                },
                _ => false, // File not shared with user or error
            }
        } else {
            false // Sharing service not available
        }
    }
    /// Uploads a file
    pub async fn upload_file(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        mut multipart: Multipart,
    ) -> impl IntoResponse {
        // Extract file from multipart request
        let mut file_part = None;
        let mut folder_id = None;
        
        while let Some(field) = multipart.next_field().await.unwrap_or(None) {
            let name = field.name().unwrap_or("").to_string();
            
            if name == "file" {
                file_part = Some((
                    field.file_name().unwrap_or("unnamed").to_string(),
                    field.content_type().unwrap_or("application/octet-stream").to_string(),
                    field.bytes().await.unwrap_or_default(),
                ));
            } else if name == "folder_id" {
                let folder_id_value = field.text().await.unwrap_or_default();
                if !folder_id_value.is_empty() {
                    folder_id = Some(folder_id_value);
                }
            }
        }
        
        // Check if file was provided
        if let Some((filename, content_type, data)) = file_part {
            tracing::info!(
                "User {} is uploading file: {}, size: {} bytes",
                current_user.username,
                filename,
                data.len()
            );
            
            // Upload file from bytes with user_id
            match service.upload_file_from_bytes(filename, folder_id, content_type, data.to_vec(), Some(current_user.id.clone())).await {
                Ok(file) => (StatusCode::CREATED, Json(file)).into_response(),
                Err(err) => {
                    let status = match &err {
                        FileServiceError::Conflict(_) => StatusCode::CONFLICT,
                        FileServiceError::NotFound(_) => StatusCode::NOT_FOUND,
                        FileServiceError::AccessError(_) => StatusCode::FORBIDDEN,
                        _ => StatusCode::INTERNAL_SERVER_ERROR,
                    };
                    
                    (status, Json(serde_json::json!({
                        "error": err.to_string()
                    }))).into_response()
                }
            }
        } else {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "No file provided"
            }))).into_response()
        }
    }
    
    /// Downloads a file with optional compression
    pub async fn download_file(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Path(id): Path<String>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        tracing::info!(
            "User {} is downloading file with ID: {}",
            current_user.username,
            id
        );
        // Initialize compression service
        let compression_service = GzipCompressionService::new();
        
        // Check if compression is explicitly requested or rejected
        let compression_param = params.get("compress").map(|v| v.as_str());
        let force_compress = compression_param == Some("true") || compression_param == Some("1");
        let force_no_compress = compression_param == Some("false") || compression_param == Some("0");
        
        // Determine compression level from query params
        let compression_level = match params.get("compression_level").map(|v| v.as_str()) {
            Some("none") => CompressionLevel::None,
            Some("fast") => CompressionLevel::Fast,
            Some("best") => CompressionLevel::Best,
            _ => CompressionLevel::Default, // Default or unrecognized
        };
        
        // First check if the user has access to this file
        if !service.check_file_access(&id, &current_user.id).await {
            tracing::warn!(
                "User {} attempted to access file {} without permission",
                current_user.username,
                id
            );
            return (
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({
                    "error": "You don't have permission to access this file"
                }))
            ).into_response();
        }
        
        // Get file info to check it exists and get metadata
        match service.get_file(&id).await {
            Ok(file) => {
                // Determine if we should compress based on file type and size
                let should_compress = if force_no_compress {
                    false
                } else if force_compress {
                    true
                } else {
                    compression_service.should_compress(&file.mime_type, file.size)
                };
                
                // Log compression decision for debugging
                tracing::debug!(
                    "Download file: name={}, size={}KB, mime={}, compress={}", 
                    file.name, file.size / 1024, file.mime_type, should_compress
                );
                
                // For large files, use streaming response with potential compression
                if file.size > 10 * 1024 * 1024 { // 10MB threshold for streaming
                    match service.get_file_content(&id).await {
                        Ok(content) => {
                            // Create base headers
                            let mut headers = HashMap::new();
                            headers.insert(
                                header::CONTENT_DISPOSITION.to_string(), 
                                format!("attachment; filename=\"{}\"", file.name)
                            );
                            
                            if should_compress {
                                // Add content-encoding header for compressed response
                                headers.insert(header::CONTENT_ENCODING.to_string(), "gzip".to_string());
                                headers.insert(header::CONTENT_TYPE.to_string(), file.mime_type.clone());
                                headers.insert(header::VARY.to_string(), "Accept-Encoding".to_string());
                                
                                // Compress the content
                                match compression_service.compress_data(&content, compression_level).await {
                                    Ok(compressed_content) => {
                                        tracing::debug!(
                                            "Compressed file: {} from {}KB to {}KB (ratio: {:.2})", 
                                            file.name, 
                                            content.len() / 1024, 
                                            compressed_content.len() / 1024,
                                            content.len() as f64 / compressed_content.len() as f64
                                        );
                                        
                                        // Build a custom response with headers and body
                                        let mut response = Response::builder()
                                            .status(StatusCode::OK)
                                            .body(axum::body::Body::from(compressed_content))
                                            .unwrap();
                                            
                                        // Add headers to response
                                        for (name, value) in headers {
                                            response.headers_mut().insert(
                                                HeaderName::from_bytes(name.as_bytes()).unwrap(),
                                                HeaderValue::from_str(&value).unwrap()
                                            );
                                        }
                                        
                                        response
                                    },
                                    Err(e) => {
                                        tracing::warn!("Compression failed, sending uncompressed: {}", e);
                                        // Fall back to uncompressed
                                        headers.insert(header::CONTENT_TYPE.to_string(), file.mime_type.clone());
                                        
                                        // Build a custom response with headers and body
                                        let mut response = Response::builder()
                                            .status(StatusCode::OK)
                                            .body(axum::body::Body::from(content))
                                            .unwrap();
                                            
                                        // Add headers to response
                                        for (name, value) in headers {
                                            response.headers_mut().insert(
                                                HeaderName::from_bytes(name.as_bytes()).unwrap(),
                                                HeaderValue::from_str(&value).unwrap()
                                            );
                                        }
                                        
                                        response
                                    }
                                }
                            } else {
                                // No compression, return as-is
                                headers.insert(header::CONTENT_TYPE.to_string(), file.mime_type.clone());
                                
                                // Build a custom response with headers and body
                                let mut response = Response::builder()
                                    .status(StatusCode::OK)
                                    .body(axum::body::Body::from(content))
                                    .unwrap();
                                    
                                // Add headers to response
                                for (name, value) in headers {
                                    response.headers_mut().insert(
                                        HeaderName::from_bytes(name.as_bytes()).unwrap(),
                                        HeaderValue::from_str(&value).unwrap()
                                    );
                                }
                                
                                response
                            }
                        },
                        Err(err) => {
                            tracing::error!("Error getting file content: {}", err);
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                                "error": format!("Error reading file: {}", err)
                            }))).into_response()
                        }
                    }
                } else {
                    // For smaller files, load entirely but still potentially compress
                    match service.get_file_content(&id).await {
                        Ok(content) => {
                            // Create base headers
                            let mut headers = HashMap::new();
                            headers.insert(
                                header::CONTENT_DISPOSITION.to_string(), 
                                format!("attachment; filename=\"{}\"", file.name)
                            );
                            
                            if should_compress {
                                // Add content-encoding header for compressed response
                                headers.insert(header::CONTENT_ENCODING.to_string(), "gzip".to_string());
                                headers.insert(header::CONTENT_TYPE.to_string(), file.mime_type.clone());
                                headers.insert(header::VARY.to_string(), "Accept-Encoding".to_string());
                                
                                // Compress the content
                                match compression_service.compress_data(&content, compression_level).await {
                                    Ok(compressed_content) => {
                                        tracing::debug!(
                                            "Compressed file: {} from {}KB to {}KB (ratio: {:.2})", 
                                            file.name, 
                                            content.len() / 1024, 
                                            compressed_content.len() / 1024,
                                            content.len() as f64 / compressed_content.len() as f64
                                        );
                                        
                                        // Build a custom response with headers and body
                                        let mut response = Response::builder()
                                            .status(StatusCode::OK)
                                            .body(axum::body::Body::from(compressed_content))
                                            .unwrap();
                                            
                                        // Add headers to response
                                        for (name, value) in headers {
                                            response.headers_mut().insert(
                                                HeaderName::from_bytes(name.as_bytes()).unwrap(),
                                                HeaderValue::from_str(&value).unwrap()
                                            );
                                        }
                                        
                                        response
                                    },
                                    Err(e) => {
                                        tracing::warn!("Compression failed, sending uncompressed: {}", e);
                                        // Fall back to uncompressed
                                        headers.insert(header::CONTENT_TYPE.to_string(), file.mime_type.clone());
                                        
                                        // Build a custom response with headers and body
                                        let mut response = Response::builder()
                                            .status(StatusCode::OK)
                                            .body(axum::body::Body::from(content))
                                            .unwrap();
                                            
                                        // Add headers to response
                                        for (name, value) in headers {
                                            response.headers_mut().insert(
                                                HeaderName::from_bytes(name.as_bytes()).unwrap(),
                                                HeaderValue::from_str(&value).unwrap()
                                            );
                                        }
                                        
                                        response
                                    }
                                }
                            } else {
                                // No compression, return as-is
                                headers.insert(header::CONTENT_TYPE.to_string(), file.mime_type.clone());
                                
                                // Build a custom response with headers and body
                                let mut response = Response::builder()
                                    .status(StatusCode::OK)
                                    .body(axum::body::Body::from(content))
                                    .unwrap();
                                    
                                // Add headers to response
                                for (name, value) in headers {
                                    response.headers_mut().insert(
                                        HeaderName::from_bytes(name.as_bytes()).unwrap(),
                                        HeaderValue::from_str(&value).unwrap()
                                    );
                                }
                                
                                response
                            }
                        },
                        Err(err) => {
                            tracing::error!("Error getting file content: {}", err);
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                                "error": format!("Error reading file: {}", err)
                            }))).into_response()
                        }
                    }
                }
            },
            Err(err) => {
                let status = match &err {
                    FileServiceError::NotFound(_) => StatusCode::NOT_FOUND,
                    FileServiceError::AccessError(_) => StatusCode::SERVICE_UNAVAILABLE,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                (status, Json(serde_json::json!({
                    "error": err.to_string()
                }))).into_response()
            }
        }
    }
    
    /// Lists files, optionally filtered by folder ID
    pub async fn list_files(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        folder_id: Option<&str>,
    ) -> impl IntoResponse {
        tracing::info!(
            "User {} is listing files in folder: {:?}",
            current_user.username,
            folder_id
        );
        
        // Use the user_id to filter files that belong to the current user
        match service.list_files_by_user(&current_user.id, folder_id).await {
            Ok(files) => {
                // Always return an array even if empty
                tracing::info!("Found {} files for user {}", files.len(), current_user.username);
                (StatusCode::OK, Json(files)).into_response()
            },
            Err(err) => {
                let status = match &err {
                    FileServiceError::NotFound(_) => StatusCode::NOT_FOUND,
                    FileServiceError::AccessError(_) => StatusCode::FORBIDDEN,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                tracing::error!("Error listing files for user {}: {}", current_user.username, err);
                
                // Return a JSON error response
                (status, Json(serde_json::json!({
                    "error": err.to_string()
                }))).into_response()
            }
        }
    }
    
    /// Deletes a file
    pub async fn delete_file(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        tracing::info!(
            "User {} is attempting to delete file with ID: {}",
            current_user.username,
            id
        );
        
        // First check if the user has access to this file
        if !service.check_file_access(&id, &current_user.id).await {
            tracing::warn!(
                "User {} attempted to delete file {} without permission",
                current_user.username,
                id
            );
            return (
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({
                    "error": "You don't have permission to delete this file"
                }))
            ).into_response();
        }
        
        match service.delete_file(&id).await {
            Ok(_) => {
                tracing::info!(
                    "User {} successfully deleted file with ID: {}",
                    current_user.username,
                    id
                );
                StatusCode::NO_CONTENT.into_response()
            },
            Err(err) => {
                let status = match &err {
                    FileServiceError::NotFound(_) => StatusCode::NOT_FOUND,
                    FileServiceError::AccessError(_) => StatusCode::FORBIDDEN,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                tracing::error!(
                    "Error deleting file {} for user {}: {}",
                    id,
                    current_user.username,
                    err
                );
                
                (status, Json(serde_json::json!({
                    "error": err.to_string()
                }))).into_response()
            }
        }
    }
    
    /// Moves a file to a different folder
    pub async fn move_file(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Path(id): Path<String>,
        Json(payload): Json<MoveFilePayload>,
    ) -> impl IntoResponse {
        tracing::info!(
            "User {} is attempting to move file {} to folder: {:?}",
            current_user.username,
            id,
            payload.folder_id
        );
        
        // First check if the user has access to this file
        if !service.check_file_access(&id, &current_user.id).await {
            tracing::warn!(
                "User {} attempted to move file {} without permission",
                current_user.username,
                id
            );
            return (
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({
                    "error": "You don't have permission to move this file",
                    "code": StatusCode::FORBIDDEN.as_u16()
                }))
            ).into_response();
        }
        
        // Verify if the file exists
        match service.get_file(&id).await {
            Ok(file) => {
                tracing::info!(
                    "File found: {} (ID: {}), proceeding with move operation",
                    file.name,
                    id
                );
                
                // For target folders, trust that the move operation will check existence
                if let Some(folder_id) = &payload.folder_id {
                    tracing::info!("Will attempt to move to folder: {}", folder_id);
                    
                    // TODO: If we implement folder ownership, we should check if the user
                    // has access to the target folder as well
                }
                
                // Proceed with the move operation
                match service.move_file(&id, payload.folder_id).await {
                    Ok(file) => {
                        tracing::info!(
                            "User {} successfully moved file: {} (ID: {})",
                            current_user.username,
                            file.name,
                            file.id
                        );
                        (StatusCode::OK, Json(file)).into_response()
                    },
                    Err(err) => {
                        let status = match &err {
                            FileServiceError::NotFound(_) => {
                                tracing::error!("Error moving file - not found: {}", err);
                                StatusCode::NOT_FOUND
                            },
                            FileServiceError::Conflict(_) => {
                                tracing::error!("Error moving file - already exists: {}", err);
                                StatusCode::CONFLICT
                            },
                            FileServiceError::AccessError(_) => {
                                tracing::error!("Error moving file - access denied: {}", err);
                                StatusCode::FORBIDDEN
                            },
                            _ => {
                                tracing::error!("Error moving file: {}", err);
                                StatusCode::INTERNAL_SERVER_ERROR
                            }
                        };
                        
                        (status, Json(serde_json::json!({
                            "error": format!("Error moving file: {}", err.to_string()),
                            "code": status.as_u16(),
                            "details": format!("Error moving file with ID: {} - {}", id, err)
                        }))).into_response()
                    }
                }
            },
            Err(err) => {
                tracing::error!(
                    "Error finding file to move - does not exist: {} (ID: {})",
                    err,
                    id
                );
                (StatusCode::NOT_FOUND, Json(serde_json::json!({
                    "error": format!("File with ID: {} does not exist", id),
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