use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{error, info};

use crate::application::ports::recent_ports::RecentItemsUseCase;
use crate::application::services::recent_service::RecentService;
use crate::interfaces::middleware::auth::AuthUser;

/// Query parameters for getting recent items
#[derive(Deserialize)]
pub struct GetRecentParams {
    #[serde(default)]
    limit: Option<i32>,
}

/// Get user's recent items
#[utoipa::path(
    get,
    path = "/api/recent",
    responses(
        (status = 200, description = "List of recent items", body = Vec<crate::application::dtos::recent_dto::RecentItemDto>)
    ),
    tag = "recent"
)]
pub async fn get_recent_items(
    State(recent_service): State<Arc<RecentService>>,
    auth_user: AuthUser,
    Query(params): Query<GetRecentParams>,
) -> impl IntoResponse {
    let user_id = auth_user.id;

    match recent_service.get_recent_items(user_id, params.limit).await {
        Ok(items) => {
            info!("Retrieved {} recent items for user", items.len());
            (StatusCode::OK, Json(items)).into_response()
        }
        Err(err) => {
            error!("Error retrieving recent items: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to retrieve recent items"
                })),
            )
                .into_response()
        }
    }
}

/// Record access to an item
#[utoipa::path(
    post,
    path = "/api/recent/{item_type}/{item_id}",
    params(
        ("item_type" = String, Path, description = "Item type (file or folder)"),
        ("item_id" = String, Path, description = "Item ID")
    ),
    responses(
        (status = 200, description = "Access recorded"),
        (status = 400, description = "Invalid item type")
    ),
    tag = "recent"
)]
pub async fn record_item_access(
    State(recent_service): State<Arc<RecentService>>,
    auth_user: AuthUser,
    Path((item_type, item_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = auth_user.id;

    // Validate item type
    if item_type != "file" && item_type != "folder" {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Item type must be 'file' or 'folder'"
            })),
        )
            .into_response();
    }

    match recent_service
        .record_item_access(user_id, &item_id, &item_type)
        .await
    {
        Ok(_) => {
            info!("Recorded access to {} '{}' in recents", item_type, item_id);
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "Access recorded successfully"
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("Error recording access in recents: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to record access"
                })),
            )
                .into_response()
        }
    }
}

/// Remove an item from recents
#[utoipa::path(
    delete,
    path = "/api/recent/{item_type}/{item_id}",
    params(
        ("item_type" = String, Path, description = "Item type (file or folder)"),
        ("item_id" = String, Path, description = "Item ID")
    ),
    responses(
        (status = 200, description = "Item removed from recents"),
        (status = 404, description = "Item not in recents")
    ),
    tag = "recent"
)]
pub async fn remove_from_recent(
    State(recent_service): State<Arc<RecentService>>,
    auth_user: AuthUser,
    Path((item_type, item_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = auth_user.id;

    match recent_service
        .remove_from_recent(user_id, &item_id, &item_type)
        .await
    {
        Ok(removed) => {
            if removed {
                info!("Removed {} '{}' from recents", item_type, item_id);
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "message": "Item removed from recents"
                    })),
                )
                    .into_response()
            } else {
                info!("Item {} '{}' was not in recents", item_type, item_id);
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "message": "Item was not in recents"
                    })),
                )
                    .into_response()
            }
        }
        Err(err) => {
            error!("Error removing from recents: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to remove from recents"
                })),
            )
                .into_response()
        }
    }
}

/// Clear all recent items
#[utoipa::path(
    delete,
    path = "/api/recent/clear",
    responses(
        (status = 200, description = "Recent items cleared")
    ),
    tag = "recent"
)]
pub async fn clear_recent_items(
    State(recent_service): State<Arc<RecentService>>,
    auth_user: AuthUser,
) -> impl IntoResponse {
    let user_id = auth_user.id;

    match recent_service.clear_recent_items(user_id).await {
        Ok(_) => {
            info!("Cleared all recent items for user");
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "Recent items cleared successfully"
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("Error clearing recent items: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to clear recent items"
                })),
            )
                .into_response()
        }
    }
}
