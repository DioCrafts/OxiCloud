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
        dtos::public_link_dto::{
            CreatePublicLinkDto, PublicFileAccessDto, PublicLinkDto, 
            UpdateExpirationDto, UpdatePasswordDto, UpdatePermissionDto, VerifyPasswordDto,
        },
        services::public_link_service::PublicLinkService,
    },
    interfaces::middleware::auth::CurrentUser,
};

use super::ApiResult;

/// Handler for public file sharing operations
pub struct PublicLinkHandler;

/// Type alias for application state
type AppState = Arc<PublicLinkService>;

impl PublicLinkHandler {
    /// Create a new public link for a file
    pub async fn create_public_link(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Json(request): Json<CreatePublicLinkDto>,
    ) -> impl IntoResponse {
        debug!("Create public link request: {:?}", request);
        
        match PermissionLevel::from_string(&request.permission) {
            Ok(permission_level) => {
                match service.create_public_link(
                    &request.file_id,
                    &current_user.id,
                    permission_level,
                    request.password.as_deref(),
                    request.expires_at,
                ).await {
                    Ok(link) => {
                        info!("Public link created successfully for file: {}", request.file_id);
                        (StatusCode::CREATED, Json(link)).into_response()
                    }
                    Err(e) => {
                        error!("Failed to create public link: {}", e);
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

    /// Get public link by ID
    pub async fn get_public_link(
        State(service): State<AppState>,
        Extension(_current_user): Extension<CurrentUser>,
        Path(link_id): Path<String>,
    ) -> impl IntoResponse {
        debug!("Get public link request: {}", link_id);
        
        match service.get_public_link(&link_id).await {
            Ok(link) => {
                debug!("Retrieved public link: {}", link_id);
                (StatusCode::OK, Json(link)).into_response()
            }
            Err(e) => {
                error!("Failed to get public link: {}", e);
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

    /// Get all public links for a file
    pub async fn get_links_for_file(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Path(file_id): Path<String>,
    ) -> impl IntoResponse {
        debug!("Get public links for file: {}", file_id);
        
        match service.get_links_for_file(&file_id, &current_user.id).await {
            Ok(links) => {
                debug!("Retrieved {} public links for file {}", links.len(), file_id);
                (StatusCode::OK, Json(links)).into_response()
            }
            Err(e) => {
                error!("Failed to get public links for file: {}", e);
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

    /// Get all public links created by current user
    pub async fn get_links_by_user(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
    ) -> impl IntoResponse {
        debug!("Get public links for user: {}", current_user.id);
        
        match service.get_links_by_user(&current_user.id).await {
            Ok(links) => {
                debug!("Retrieved {} public links for user {}", links.len(), current_user.id);
                (StatusCode::OK, Json(links)).into_response()
            }
            Err(e) => {
                error!("Failed to get public links for user: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }

    /// Update public link permission
    pub async fn update_link_permission(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Path(link_id): Path<String>,
        Json(request): Json<UpdatePermissionDto>,
    ) -> impl IntoResponse {
        debug!("Update public link permission: {} to {:?}", link_id, request);
        
        match request.to_permission_level() {
            Ok(permission_level) => {
                match service.update_permission(&link_id, &current_user.id, permission_level).await {
                    Ok(link) => {
                        info!("Public link permission updated successfully: {}", link_id);
                        (StatusCode::OK, Json(link)).into_response()
                    }
                    Err(e) => {
                        error!("Failed to update public link permission: {}", e);
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

    /// Update public link password
    pub async fn update_link_password(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Path(link_id): Path<String>,
        Json(request): Json<UpdatePasswordDto>,
    ) -> impl IntoResponse {
        debug!("Update public link password: {}", link_id);
        
        match service.update_password(&link_id, &current_user.id, request.password.as_deref()).await {
            Ok(link) => {
                info!("Public link password updated successfully: {}", link_id);
                (StatusCode::OK, Json(link)).into_response()
            }
            Err(e) => {
                error!("Failed to update public link password: {}", e);
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

    /// Update public link expiration
    pub async fn update_link_expiration(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Path(link_id): Path<String>,
        Json(request): Json<UpdateExpirationDto>,
    ) -> impl IntoResponse {
        debug!("Update public link expiration: {}", link_id);
        
        match service.update_expiration(&link_id, &current_user.id, request.expires_at).await {
            Ok(link) => {
                info!("Public link expiration updated successfully: {}", link_id);
                (StatusCode::OK, Json(link)).into_response()
            }
            Err(e) => {
                error!("Failed to update public link expiration: {}", e);
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

    /// Delete public link
    pub async fn delete_public_link(
        State(service): State<AppState>,
        Extension(current_user): Extension<CurrentUser>,
        Path(link_id): Path<String>,
    ) -> impl IntoResponse {
        debug!("Delete public link request: {}", link_id);
        
        match service.delete_public_link(&link_id, &current_user.id).await {
            Ok(_) => {
                info!("Public link deleted successfully: {}", link_id);
                StatusCode::NO_CONTENT.into_response()
            }
            Err(e) => {
                error!("Failed to delete public link: {}", e);
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

    /// Access public file (public endpoint - no auth required)
    pub async fn access_public_file(
        State(service): State<AppState>,
        Path(link_id): Path<String>,
        Json(request): Json<Option<VerifyPasswordDto>>,
    ) -> impl IntoResponse {
        debug!("Access public file request: {}", link_id);
        
        let password = request.map(|dto| dto.password);
        
        match service.access_file(&link_id, password.as_deref()).await {
            Ok(file_access) => {
                info!("Public file accessed successfully: {}", link_id);
                (StatusCode::OK, Json(file_access)).into_response()
            }
            Err(e) => {
                error!("Failed to access public file: {}", e);
                match e {
                    _ if e.to_string().contains("not found") => {
                        (StatusCode::NOT_FOUND, e.to_string()).into_response()
                    }
                    _ if e.to_string().contains("password") => {
                        (StatusCode::UNAUTHORIZED, "Invalid password".to_string()).into_response()
                    }
                    _ if e.to_string().contains("expired") => {
                        (StatusCode::GONE, "Link has expired".to_string()).into_response()
                    }
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
                }
            }
        }
    }
}

