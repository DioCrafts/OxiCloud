use std::sync::Arc;
use std::collections::HashMap;
use axum::{
    extract::{Path, State, Query},
    http::{StatusCode, header, HeaderName, HeaderValue, Response},
    response::IntoResponse,
    Json,
};

use crate::application::services::folder_service::FolderService;
use crate::application::dtos::folder_dto::{CreateFolderDto, RenameFolderDto, MoveFolderDto};
use crate::application::dtos::pagination::PaginationRequestDto;
use crate::common::errors::ErrorKind;
use crate::application::ports::inbound::FolderUseCase;
use crate::common::di::AppState as GlobalAppState;
use crate::interfaces::middleware::auth::{OptionalAuthUser, AuthUser};

type AppState = Arc<FolderService>;

/// Handler for folder-related API endpoints
pub struct FolderHandler;

impl FolderHandler {
    /// Creates a new folder
    pub async fn create_folder(
        State(service): State<AppState>,
        Json(dto): Json<CreateFolderDto>,
    ) -> impl IntoResponse {
        match service.create_folder(dto).await {
            Ok(folder) => (StatusCode::CREATED, Json(folder)).into_response(),
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::AlreadyExists => StatusCode::CONFLICT,
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                (status, err.to_string()).into_response()
            }
        }
    }
    
    /// Gets a folder by ID
    pub async fn get_folder(
        State(service): State<AppState>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        match service.get_folder(&id).await {
            Ok(folder) => (StatusCode::OK, Json(folder)).into_response(),
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                (status, err.to_string()).into_response()
            }
        }
    }
    
    /// Lists root folders (no parent ID)
    /// Non-admin users only see their own home folder.
    pub async fn list_root_folders(
        State(service): State<AppState>,
        auth_user: AuthUser,
    ) -> axum::response::Response {
        Self::list_folders_for_user(service, None, &auth_user).await
    }

    /// Lists contents of a specific folder by its ID
    pub async fn list_folder_contents(
        State(service): State<AppState>,
        Path(id): Path<String>,
    ) -> axum::response::Response {
        Self::list_folders_inner(service, Some(&id)).await
    }

    /// Lists root folders with pagination support
    pub async fn list_root_folders_paginated(
        State(service): State<AppState>,
        auth_user: AuthUser,
        _pagination: Query<PaginationRequestDto>,
    ) -> axum::response::Response {
        // For paginated root listing, filter by user as well
        Self::list_folders_for_user(service, None, &auth_user).await
    }

    /// Lists contents of a specific folder with pagination
    pub async fn list_folder_contents_paginated(
        State(service): State<AppState>,
        Path(id): Path<String>,
        pagination: Query<PaginationRequestDto>,
    ) -> axum::response::Response {
        Self::list_folders_paginated_inner(service, pagination, Some(&id)).await
    }

    /// Checks if a folder name matches the user home-folder convention.
    fn is_user_home_folder(folder_name: &str) -> bool {
        folder_name.starts_with("My Folder - ")
    }

    /// Checks if a folder belongs to the given user.
    fn folder_belongs_to_user(folder_name: &str, username: &str) -> bool {
        let expected = format!("My Folder - {}", username);
        folder_name == expected
    }

    /// Lists folders, optionally filtered by parent ID (internal helper)
    async fn list_folders_inner(
        service: AppState,
        parent_id: Option<&str>,
    ) -> axum::response::Response {
        match service.list_folders(parent_id).await {
            Ok(folders) => {
                (StatusCode::OK, Json(folders)).into_response()
            },
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                (status, Json(serde_json::json!({
                    "error": err.to_string()
                }))).into_response()
            }
        }
    }

    /// Lists folders with user-based filtering for root listings.
    /// Non-admin users only see their own home folder at the root level.
    async fn list_folders_for_user(
        service: AppState,
        parent_id: Option<&str>,
        auth_user: &AuthUser,
    ) -> axum::response::Response {
        match service.list_folders(parent_id).await {
            Ok(folders) => {
                // Only filter at root level (parent_id == None)
                let filtered = if parent_id.is_none() {
                    folders.into_iter().filter(|f| {
                        // Skip hidden/system folders
                        if f.name.starts_with('.') {
                            return false;
                        }
                        // If it's a user home folder, only show if it belongs to this user
                        if Self::is_user_home_folder(&f.name) {
                            return Self::folder_belongs_to_user(&f.name, &auth_user.username);
                        }
                        // Non-home folders are visible to everyone
                        true
                    }).collect()
                } else {
                    folders
                };
                (StatusCode::OK, Json(filtered)).into_response()
            },
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                (status, Json(serde_json::json!({
                    "error": err.to_string()
                }))).into_response()
            }
        }
    }
    
    /// Lists folders with pagination support (internal helper)
    async fn list_folders_paginated_inner(
        service: AppState,
        Query(pagination): Query<PaginationRequestDto>,
        parent_id: Option<&str>,
    ) -> axum::response::Response {
        match service.list_folders_paginated(parent_id, &pagination).await {
            Ok(paginated_result) => {
                (StatusCode::OK, Json(paginated_result)).into_response()
            },
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                // Return a JSON error response
                (status, Json(serde_json::json!({
                    "error": err.to_string()
                }))).into_response()
            }
        }
    }
    
    /// Renames a folder
    pub async fn rename_folder(
        State(service): State<AppState>,
        Path(id): Path<String>,
        Json(dto): Json<RenameFolderDto>,
    ) -> impl IntoResponse {
        match service.rename_folder(&id, dto).await {
            Ok(folder) => (StatusCode::OK, Json(folder)).into_response(),
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    ErrorKind::AlreadyExists => StatusCode::CONFLICT,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                // Return a proper JSON error response
                (status, Json(serde_json::json!({
                    "error": err.to_string()
                }))).into_response()
            }
        }
    }
    
    /// Moves a folder to a new parent
    pub async fn move_folder(
        State(service): State<AppState>,
        Path(id): Path<String>,
        Json(dto): Json<MoveFolderDto>,
    ) -> impl IntoResponse {
        match service.move_folder(&id, dto).await {
            Ok(folder) => (StatusCode::OK, Json(folder)).into_response(),
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    ErrorKind::AlreadyExists => StatusCode::CONFLICT,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                (status, err.to_string()).into_response()
            }
        }
    }
    
    /// Deletes a folder (with trash support)
    pub async fn delete_folder(
        State(service): State<AppState>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        // For folder deletion without trash functionality
        match service.delete_folder(&id).await {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(err) => {
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                (status, err.to_string()).into_response()
            }
        }
    }
    
    /// Deletes a folder with trash functionality
    pub async fn delete_folder_with_trash(
        State(state): State<GlobalAppState>,
        OptionalAuthUser(auth_user): OptionalAuthUser,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let user_id = auth_user.as_ref().map(|u| u.id.as_str()).unwrap_or("anonymous");
        // Check if trash service is available
        if let Some(trash_service) = &state.trash_service {
            tracing::info!("Moving folder to trash: {}", id);
            
            // Try to move to trash first
            match trash_service.move_to_trash(&id, "folder", user_id).await {
                Ok(_) => {
                    tracing::info!("Folder successfully moved to trash: {}", id);
                    return StatusCode::NO_CONTENT.into_response();
                },
                Err(err) => {
                    tracing::warn!("Could not move folder to trash, falling back to permanent delete: {}", err);
                    // Fall through to regular delete if trash fails
                }
            }
        }
        
        // Fallback to permanent delete if trash is unavailable or failed
        let folder_service = &state.applications.folder_service;
        match folder_service.delete_folder(&id).await {
            Ok(_) => {
                tracing::info!("Folder permanently deleted: {}", id);
                StatusCode::NO_CONTENT.into_response()
            },
            Err(err) => {
                tracing::error!("Error deleting folder: {}", err);
                
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                (status, Json(serde_json::json!({
                    "error": format!("Error deleting folder: {}", err)
                }))).into_response()
            }
        }
    }
    
    /// Downloads a folder as a ZIP file
    pub async fn download_folder_zip(
        State(state): State<GlobalAppState>,
        Path(id): Path<String>,
        Query(_params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        tracing::info!("Downloading folder as ZIP: {}", id);
        
        // Get folder information first to check it exists and get name
        let folder_service = &state.applications.folder_service;
        
        match folder_service.get_folder(&id).await {
            Ok(folder) => {
                tracing::info!("Preparing ZIP for folder: {} ({})", folder.name, id);
                
                // Use ZIP service from DI container
                let zip_service = &state.core.zip_service;
                
                // Create the ZIP file
                match zip_service.create_folder_zip(&id, &folder.name).await {
                    Ok(zip_data) => {
                        tracing::info!("ZIP file created successfully, size: {} bytes", zip_data.len());
                        
                        // Setup headers for download
                        let filename = format!("{}.zip", folder.name);
                        let content_disposition = format!("attachment; filename=\"{}\"", filename);
                        
                        // Build response with the ZIP data
                        let mut headers = HashMap::new();
                        headers.insert(header::CONTENT_TYPE.to_string(), "application/zip".to_string());
                        headers.insert(header::CONTENT_DISPOSITION.to_string(), content_disposition);
                        headers.insert(header::CONTENT_LENGTH.to_string(), zip_data.len().to_string());
                        
                        // Build the response
                        let mut response = Response::builder()
                            .status(StatusCode::OK)
                            .body(axum::body::Body::from(zip_data))
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
                    Err(err) => {
                        tracing::error!("Error creating ZIP file: {}", err);
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                            "error": format!("Error creating ZIP file: {}", err)
                        }))).into_response()
                    }
                }
            },
            Err(err) => {
                tracing::error!("Folder not found: {}", err);
                let status = match err.kind {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                (status, Json(serde_json::json!({
                    "error": format!("Error finding folder: {}", err)
                }))).into_response()
            }
        }
    }
}