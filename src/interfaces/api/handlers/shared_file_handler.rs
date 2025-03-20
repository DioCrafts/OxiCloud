use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    Json,
};
use axum::http::StatusCode;
use std::sync::Arc;
use tracing::{debug, error, info};
use crate::domain::entities::shared_file::PermissionLevel;

use crate::{
    application::{
        dtos::shared_file_dto::{FileAccessDto, ShareFileRequestDto, SharedFileDto, UpdatePermissionDto},
        services::sharing_service::SharingService,
    },
    interfaces::middleware::auth::CurrentUser,
};

use super::ApiResult;

/// Handler for file sharing operations
pub struct SharedFileHandler;

/// Type alias for application state
type AppState = Arc<SharingService>;

impl SharedFileHandler {
    /// Share a file with another user
    pub async fn share_file(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Json(request): Json<ShareFileRequestDto>,
    ) -> impl IntoResponse {
        debug!("Share file request: {:?}", request);
        
        match PermissionLevel::from_string(&request.permission) {
            Ok(permission_level) => {
                match service.share_file(
                    &request.file_id,
                    &current_user.id,
                    &request.user_id,
                    permission_level,
                ).await {
                    Ok(shared_file) => {
                        info!("File shared successfully: {} with user {}", request.file_id, request.user_id);
                        (StatusCode::CREATED, Json(shared_file)).into_response()
                    }
                    Err(e) => {
                        error!("Failed to share file: {}", e);
                        match e {
                            _ if e.to_string().contains("not found") => {
                                (StatusCode::NOT_FOUND, e.to_string()).into_response()
                            }
                            _ if e.to_string().contains("permission") => {
                                (StatusCode::FORBIDDEN, e.to_string()).into_response()
                            }
                            _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
                        }
                    }
                }
            }
            Err(e) => {
                error!("Invalid permission format: {}", e);
                (StatusCode::BAD_REQUEST, format!("Invalid permission format: {}", e)).into_response()
            }
        }
    }

    /// Update permission for a shared file
    pub async fn update_permission(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Path(file_id): Path<String>,
        Json(request): Json<UpdatePermissionDto>,
    ) -> impl IntoResponse {
        debug!("Update permission request for file {}: {:?}", file_id, request);
        
        match request.to_permission_level() {
            Ok(permission_level) => {
                match service.update_permission(
                    &file_id,
                    &current_user.id,
                    &request.user_id,
                    permission_level,
                ).await {
                    Ok(shared_file) => {
                        info!("Permission updated successfully for file {} with user {}", 
                              file_id, request.user_id);
                        (StatusCode::OK, Json(shared_file)).into_response()
                    }
                    Err(e) => {
                        error!("Failed to update permission: {}", e);
                        match e {
                            _ if e.to_string().contains("not found") => {
                                (StatusCode::NOT_FOUND, e.to_string()).into_response()
                            }
                            _ if e.to_string().contains("permission") => {
                                (StatusCode::FORBIDDEN, e.to_string()).into_response()
                            }
                            _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
                        }
                    }
                }
            }
            Err(e) => {
                error!("Invalid permission format: {}", e);
                (StatusCode::BAD_REQUEST, format!("Invalid permission format: {}", e)).into_response()
            }
        }
    }

    /// Unshare a file with a user
    pub async fn unshare_file(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Path((file_id, user_id)): Path<(String, String)>,
    ) -> impl IntoResponse {
        debug!("Unshare file request: {} from user {}", file_id, user_id);
        
        match service.unshare_file(&file_id, &current_user.id, &user_id).await {
            Ok(_) => {
                info!("File unshared successfully: {} from user {}", file_id, user_id);
                StatusCode::NO_CONTENT.into_response()
            }
            Err(e) => {
                error!("Failed to unshare file: {}", e);
                match e {
                    _ if e.to_string().contains("not found") => {
                        (StatusCode::NOT_FOUND, e.to_string()).into_response()
                    }
                    _ if e.to_string().contains("permission") => {
                        (StatusCode::FORBIDDEN, e.to_string()).into_response()
                    }
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
                }
            }
        }
    }

    /// Get files shared with current user
    pub async fn get_files_shared_with_me(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
    ) -> impl IntoResponse {
        debug!("Get files shared with user: {}", current_user.id);
        
        match service.get_files_shared_with_user(&current_user.id).await {
            Ok(files) => {
                debug!("Retrieved {} shared files for user {}", files.len(), current_user.id);
                (StatusCode::OK, Json(files)).into_response()
            }
            Err(e) => {
                error!("Failed to get shared files: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }

    /// Get files shared by current user
    pub async fn get_files_shared_by_me(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
    ) -> impl IntoResponse {
        debug!("Get files shared by user: {}", current_user.id);
        
        match service.get_files_shared_by_user(&current_user.id).await {
            Ok(files) => {
                debug!("Retrieved {} files shared by user {}", files.len(), current_user.id);
                (StatusCode::OK, Json(files)).into_response()
            }
            Err(e) => {
                error!("Failed to get files shared by user: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }

    /// Get users with access to a file
    pub async fn get_users_with_access(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Path(file_id): Path<String>,
    ) -> impl IntoResponse {
        debug!("Get users with access to file: {}", file_id);
        
        match service.get_users_with_access(&file_id, &current_user.id).await {
            Ok(users) => {
                debug!("Retrieved {} users with access to file {}", 
                      users.len(), file_id);
                
                // Create a wrapper with users list
                let access_data = FileAccessDto {
                    file_id: file_id.clone(),
                    file_name: "".to_string(), // We don't have the file name here
                    users: users.into_iter().map(|sf| sf.into()).collect(),
                };
                
                (StatusCode::OK, Json(access_data)).into_response()
            }
            Err(e) => {
                error!("Failed to get users with access: {}", e);
                match e {
                    _ if e.to_string().contains("not found") => {
                        (StatusCode::NOT_FOUND, e.to_string()).into_response()
                    }
                    _ if e.to_string().contains("permission") => {
                        (StatusCode::FORBIDDEN, e.to_string()).into_response()
                    }
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
                }
            }
        }
    }

    /// Check if user has access to file (used by file handlers)
    pub async fn check_file_access(
        service: &SharingService,
        file_id: &str,
        user_id: &str,
    ) -> Result<bool, (StatusCode, String)> {
        match service.check_user_has_access(file_id, user_id).await {
            Ok(has_access) => Ok(has_access.is_some()),
            Err(e) => {
                error!("Failed to check file access: {}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }
    }
}