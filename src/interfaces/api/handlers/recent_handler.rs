use std::sync::Arc;
use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use tracing::{error, info};

use crate::application::ports::recent_ports::RecentItemsUseCase;
use crate::interfaces::middleware::auth::AuthUser;

/// Query parameters for getting recent items
#[derive(Deserialize)]
pub struct GetRecentParams {
    #[serde(default)]
    limit: Option<i32>,
}

/// Get user's recent items
pub async fn get_recent_items(
    State(recent_service): State<Arc<dyn RecentItemsUseCase>>,
    auth_user: AuthUser,
    Query(params): Query<GetRecentParams>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match recent_service.get_recent_items(user_id, params.limit).await {
        Ok(items) => {
            info!("Retrieved {} recent items for user", items.len());
            (StatusCode::OK, Json(items)).into_response()
        },
        Err(err) => {
            error!("Error retrieving recent items: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(serde_json::json!({
                    "error": format!("Failed to retrieve recent items: {}", err)
                }))
            ).into_response()
        }
    }
}

/// Record access to an item
pub async fn record_item_access(
    State(recent_service): State<Arc<dyn RecentItemsUseCase>>,
    auth_user: AuthUser,
    Path((item_type, item_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    // Validate item type
    if item_type != "file" && item_type != "folder" {
        return (
            StatusCode::BAD_REQUEST, 
            Json(serde_json::json!({
                "error": "Item type must be 'file' or 'folder'"
            }))
        ).into_response();
    }
    
    match recent_service.record_item_access(user_id, &item_id, &item_type).await {
        Ok(_) => {
            info!("Recorded access to {} '{}' in recents", item_type, item_id);
            (
                StatusCode::OK, 
                Json(serde_json::json!({
                    "message": "Access recorded successfully"
                }))
            ).into_response()
        },
        Err(err) => {
            error!("Error recording access in recents: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(serde_json::json!({
                    "error": format!("Failed to record access: {}", err)
                }))
            ).into_response()
        }
    }
}

/// Remove an item from recents
pub async fn remove_from_recent(
    State(recent_service): State<Arc<dyn RecentItemsUseCase>>,
    auth_user: AuthUser,
    Path((item_type, item_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match recent_service.remove_from_recent(user_id, &item_id, &item_type).await {
        Ok(removed) => {
            if removed {
                info!("Removed {} '{}' from recents", item_type, item_id);
                (
                    StatusCode::OK, 
                    Json(serde_json::json!({
                        "message": "Item removed from recents"
                    }))
                ).into_response()
            } else {
                info!("Item {} '{}' was not in recents", item_type, item_id);
                (
                    StatusCode::NOT_FOUND, 
                    Json(serde_json::json!({
                        "message": "Item was not in recents"
                    }))
                ).into_response()
            }
        },
        Err(err) => {
            error!("Error removing from recents: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(serde_json::json!({
                    "error": format!("Failed to remove from recents: {}", err)
                }))
            ).into_response()
        }
    }
}

/// Clear all recent items
pub async fn clear_recent_items(
    State(recent_service): State<Arc<dyn RecentItemsUseCase>>,
    auth_user: AuthUser,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match recent_service.clear_recent_items(user_id).await {
        Ok(_) => {
            info!("Cleared all recent items for user");
            (
                StatusCode::OK, 
                Json(serde_json::json!({
                    "message": "Recent items cleared successfully"
                }))
            ).into_response()
        },
        Err(err) => {
            error!("Error clearing recent items: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(serde_json::json!({
                    "error": format!("Failed to clear recent items: {}", err)
                }))
            ).into_response()
        }
    }
}