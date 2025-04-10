use std::sync::Arc;
use axum::{
    extract::{Path, State, Multipart, Query},
    http::{StatusCode, header, HeaderName, HeaderValue, Response},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use futures::Stream;
// use futures::StreamExt;
use std::task::{Context, Poll};
use std::pin::Pin;

use crate::application::services::file_service::{FileService, FileServiceError};
use crate::infrastructure::services::compression_service::{
    CompressionService, GzipCompressionService, CompressionLevel
};
use crate::common::di::AppState;

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

impl FileHandler {
    /// Uploads a file
    pub async fn upload_file(
        State(service): State<FileServiceState>,
        mut multipart: Multipart,
    ) -> impl IntoResponse {
        // Extract file from multipart request
        let mut file_part = None;
        let mut folder_id = None;
        
        tracing::info!("Processing file upload request");
        
        // More robust multipart field processing
        let mut field_count = 0;
        
        // Process fields until we get None (end of form data)
        loop {
            // Handle errors from next_field() explicitly
            let field_result = match multipart.next_field().await {
                Ok(maybe_field) => maybe_field,
                Err(err) => {
                    tracing::error!("Error reading next multipart field: {}", err);
                    break;
                }
            };
            
            // If we got None, we're done processing the form
            let field = match field_result {
                Some(field) => field,
                None => break,
            };
            
            field_count += 1;
            let name = field.name().unwrap_or("").to_string();
            tracing::info!("Multipart field #{} received: '{}'", field_count, name);
            
            if name == "file" {
                let filename = field.file_name().unwrap_or("unnamed").to_string();
                let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
                tracing::info!("File received: '{}' (type: {})", filename, content_type);
                
                match field.bytes().await {
                    Ok(bytes) => {
                        let size = bytes.len();
                        tracing::info!("Successfully read file '{}' ({} bytes)", filename, size);
                        if size == 0 {
                            tracing::warn!("Warning: File '{}' has zero bytes", filename);
                        }
                        file_part = Some((filename, content_type, bytes));
                    },
                    Err(e) => {
                        tracing::error!("Failed to read file bytes: {}", e);
                    }
                }
            } else if name == "folder_id" {
                match field.text().await {
                    Ok(folder_id_value) => {
                        tracing::info!("folder_id received: '{}'", folder_id_value);
                        
                        if !folder_id_value.is_empty() {
                            folder_id = Some(folder_id_value);
                            tracing::info!("Using folder_id: '{}'", folder_id.as_ref().unwrap());
                        } else {
                            tracing::info!("Empty folder_id received, will use root folder");
                        }
                    },
                    Err(e) => {
                        tracing::error!("Failed to read folder_id: {}", e);
                    }
                }
            } else {
                tracing::warn!("Received unknown field: '{}'", name);
                match field.text().await {
                    Ok(value) => tracing::debug!("Field '{}' value: '{}'", name, value),
                    Err(_) => tracing::debug!("Could not read field '{}' as text", name)
                }
            }
        }
        
        tracing::info!("Multipart processing completed. Got {} fields total", field_count);
        
        // Check if file was provided
        if let Some((filename, content_type, data)) = file_part {
            tracing::info!("Preparing to upload file '{}' ({} bytes) to folder_id: {:?}", 
                filename, data.len(), folder_id);
            
            // Use the proper file service to handle the upload
            let upload_result = service
                .upload_file_from_bytes(filename.clone(), folder_id.clone(), content_type.clone(), data.to_vec())
                .await;
                
            match upload_result {
                Ok(file) => {
                    tracing::info!("File uploaded successfully: '{}' (ID: {})", filename, file.id);
                    
                    // Log additional debugging information
                    tracing::info!("Created file details: folder_id={:?}, size={}, path={}",
                        file.folder_id, file.size, file.path);
                    
                    // Log folder ID information if present
                    if let Some(folder_id) = &file.folder_id {
                        tracing::info!("Archivo guardado en carpeta ID: {}", folder_id);
                    }
                    
                    // Return success response with file information
                    (StatusCode::CREATED, Json(file)).into_response()
                },
                Err(err) => {
                    tracing::error!("Error uploading file '{}' through service: {}", filename, err);
                    
                    // Additional error details
                    match &err {
                        FileServiceError::NotFound(id) => {
                            tracing::error!("Not found error detail: {}", id);
                        },
                        FileServiceError::AccessError(msg) => {
                            tracing::error!("Access error detail: {}", msg);
                        },
                        FileServiceError::InternalError(msg) => {
                            tracing::error!("Internal error detail: {}", msg);
                        },
                        _ => {
                            tracing::error!("Other error type: {:?}", err);
                        }
                    };
                    
                    // Return error response
                    let status = match &err {
                        FileServiceError::NotFound(_) => StatusCode::NOT_FOUND,
                        FileServiceError::AccessError(_) => StatusCode::SERVICE_UNAVAILABLE,
                        _ => StatusCode::INTERNAL_SERVER_ERROR,
                    };
                    
                    (status, Json(serde_json::json!({
                        "error": format!("Error uploading file: {}", err),
                        "error_type": format!("{:?}", err)
                    }))).into_response()
                }
            }
        } else {
            tracing::error!("Error: No file provided in request or file processing failed");
            
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "No file provided or file processing failed",
                "fields_received": field_count
            }))).into_response()
        }
    }
    
    /// Downloads a file with optional compression
    pub async fn download_file(
        State(service): State<FileServiceState>,
        Path(id): Path<String>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
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
        
        // Get file info first to check it exists and get metadata
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
                            
                            // Determine if the file should be displayed inline or downloaded
                            // Images and PDFs should be displayed inline by default, or if inline param is present
                            let force_inline = params.get("inline").map_or(false, |v| v == "true" || v == "1");
                            
                            let disposition = if force_inline || 
                                             file.mime_type.starts_with("image/") || 
                                             file.mime_type == "application/pdf" {
                                format!("inline; filename=\"{}\"", file.name)
                            } else {
                                format!("attachment; filename=\"{}\"", file.name)
                            };
                            
                            headers.insert(header::CONTENT_DISPOSITION.to_string(), disposition);
                            
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
                            
                            // Determine if the file should be displayed inline or downloaded
                            // Images and PDFs should be displayed inline by default, or if inline param is present
                            let force_inline = params.get("inline").map_or(false, |v| v == "true" || v == "1");
                            
                            let disposition = if force_inline || 
                                             file.mime_type.starts_with("image/") || 
                                             file.mime_type == "application/pdf" {
                                format!("inline; filename=\"{}\"", file.name)
                            } else {
                                format!("attachment; filename=\"{}\"", file.name)
                            };
                            
                            headers.insert(header::CONTENT_DISPOSITION.to_string(), disposition);
                            
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
                
                // Return the files as JSON response
                (StatusCode::OK, Json(files)).into_response()
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
    
    /// Deletes a file (with trash support)
    pub async fn delete_file(
        State(state): State<GlobalState>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        // Check if trash service is available
        if let Some(trash_service) = &state.trash_service {
            tracing::info!("Moving file to trash: {}", id);
            
            // Debug logs to track trash components
            tracing::debug!("Trash service type: {}", std::any::type_name_of_val(&*trash_service));
            let default_user_id = "00000000-0000-0000-0000-000000000000".to_string();
            tracing::info!("Using default user ID: {}", default_user_id);
            
            // Try to move to trash first - add more detailed logging
            tracing::info!("About to call trash_service.move_to_trash with id={}, type=file", id);
            match trash_service.move_to_trash(&id, "file", &default_user_id).await {
                Ok(_) => {
                    tracing::info!("File successfully moved to trash: {}", id);
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