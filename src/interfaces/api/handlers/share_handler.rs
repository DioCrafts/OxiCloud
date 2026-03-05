use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;

use crate::application::services::share_service::ShareService;
use crate::{
    application::{
        dtos::share_dto::{CreateShareDto, UpdateShareDto},
        ports::share_ports::ShareUseCase,
    },
    common::errors::ErrorKind,
    domain::entities::share::ShareItemType,
    interfaces::errors::AppError,
    interfaces::middleware::auth::AuthUser,
};

#[derive(Debug, Deserialize)]
pub struct GetSharesQuery {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub item_id: Option<String>,
    pub item_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyPasswordRequest {
    pub password: String,
}

/// Create a new shared link
pub async fn create_shared_link(
    State(share_use_case): State<Arc<ShareService>>,
    auth_user: AuthUser,
    Json(dto): Json<CreateShareDto>,
) -> impl IntoResponse {
    match share_use_case
        .create_shared_link(&auth_user.id, dto)
        .await
    {
        Ok(share) => (StatusCode::CREATED, Json(share)).into_response(),
        Err(err) => AppError::from(err).into_response()
    }
}

/// Get information about a specific shared link by ID
pub async fn get_shared_link(
    State(share_use_case): State<Arc<ShareService>>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match share_use_case.get_shared_link(&id, &auth_user.id).await {
        Ok(share) => (StatusCode::OK, Json(share)).into_response(),
        Err(err) => AppError::from(err).into_response()
    }
}

/// Get all shared links created by the current user.
/// Supports optional filtering by item_id + item_type query params.
pub async fn get_user_shares(
    State(share_use_case): State<Arc<ShareService>>,
    auth_user: AuthUser,
    Query(query): Query<GetSharesQuery>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;

    // If both item_id and item_type are provided, return shares for that specific item
    if let (Some(item_id), Some(item_type_str)) = (&query.item_id, &query.item_type) {
        let item_type = match ShareItemType::try_from(item_type_str.as_str()) {
            Ok(t) => t,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": format!("Invalid item_type: {}", item_type_str) })),
                )
                    .into_response();
            }
        };
        return match share_use_case
            .get_shared_links_for_item(item_id, &item_type, user_id)
            .await
        {
            Ok(shares) => (StatusCode::OK, Json(shares)).into_response(),
            Err(err) => AppError::from(err).into_response(),
        };
    }

    // Default: paginated list of all user shares
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    match share_use_case
        .get_user_shared_links(user_id, page, per_page)
        .await
    {
        Ok(shares) => (StatusCode::OK, Json(shares)).into_response(),
        Err(err) => AppError::from(err).into_response(),
    }
}

/// Update a shared link's properties
pub async fn update_shared_link(
    State(share_use_case): State<Arc<ShareService>>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(dto): Json<UpdateShareDto>,
) -> impl IntoResponse {
    match share_use_case
        .update_shared_link(&id, &auth_user.id, dto)
        .await
    {
        Ok(share) => (StatusCode::OK, Json(share)).into_response(),
        Err(err) => AppError::from(err).into_response()
    }
}

/// Delete a shared link
pub async fn delete_shared_link(
    State(share_use_case): State<Arc<ShareService>>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match share_use_case
        .delete_shared_link(&id, &auth_user.id)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => AppError::from(err).into_response()
    }
}

/// Access a shared item via its token
pub async fn access_shared_item(
    State(share_use_case): State<Arc<ShareService>>,
    Path(token): Path<String>,
) -> impl IntoResponse {
    // Register the access
    let _ = share_use_case.register_shared_link_access(&token).await;

    // Get the shared link
    match share_use_case.get_shared_link_by_token(&token).await {
        Ok(item) => (StatusCode::OK, Json(item)).into_response(),
        Err(err) => {
            // Special handling for share access errors
            if err.kind == ErrorKind::AccessDenied {
                if err.message.contains("password") {
                    return (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "error": "Password required",
                            "requiresPassword": true
                        })),
                    )
                        .into_response();
                }
                if err.message.contains("expired") {
                    return AppError::new(StatusCode::GONE, err.message, "Expired")
                        .into_response();
                }
            }
            AppError::from(err).into_response()
        }
    }
}

/// Verify password for a password-protected shared item
pub async fn verify_shared_item_password(
    State(share_use_case): State<Arc<ShareService>>,
    Path(token): Path<String>,
    Json(req): Json<VerifyPasswordRequest>,
) -> impl IntoResponse {
    match share_use_case
        .verify_shared_link_password(&token, &req.password)
        .await
    {
        Ok(item) => (StatusCode::OK, Json(item)).into_response(),
        Err(err) => {
            if err.kind == ErrorKind::AccessDenied {
                if err.message.contains("expired") {
                    return AppError::new(StatusCode::GONE, err.message, "Expired")
                        .into_response();
                }
                if err.message.contains("password") {
                    return AppError::unauthorized("Invalid password").into_response();
                }
            }
            AppError::from(err).into_response()
        }
    }
}
