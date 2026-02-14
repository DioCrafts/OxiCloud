use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use tracing::{error, info};

use crate::application::ports::favorites_ports::FavoritesUseCase;
use crate::interfaces::middleware::auth::AuthUser;

/// Handler for favorite-related API endpoints
pub async fn get_favorites(
    State(favorites_service): State<Arc<dyn FavoritesUseCase>>,
    auth_user: AuthUser,
) -> impl IntoResponse {
    let user_id = &auth_user.id;

    match favorites_service.get_favorites(user_id).await {
        Ok(favorites) => {
            info!("Retrieved {} favorites for user", favorites.len());
            (StatusCode::OK, Json(serde_json::json!(favorites))).into_response()
        }
        Err(err) => {
            error!("Error retrieving favorites: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to retrieve favorites: {}", err)
                })),
            )
                .into_response()
        }
    }
}

/// Add an item to user's favorites
pub async fn add_favorite(
    State(favorites_service): State<Arc<dyn FavoritesUseCase>>,
    auth_user: AuthUser,
    Path((item_type, item_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;

    // Validate item_type
    if item_type != "file" && item_type != "folder" {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Item type must be 'file' or 'folder'"
            })),
        );
    }

    match favorites_service
        .add_to_favorites(user_id, &item_id, &item_type)
        .await
    {
        Ok(_) => {
            info!("Added {} '{}' to favorites", item_type, item_id);
            (
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "message": "Item added to favorites"
                })),
            )
        }
        Err(err) => {
            error!("Error adding to favorites: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to add to favorites: {}", err)
                })),
            )
        }
    }
}

/// Remove an item from user's favorites
pub async fn remove_favorite(
    State(favorites_service): State<Arc<dyn FavoritesUseCase>>,
    auth_user: AuthUser,
    Path((item_type, item_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;

    match favorites_service
        .remove_from_favorites(user_id, &item_id, &item_type)
        .await
    {
        Ok(removed) => {
            if removed {
                info!("Removed {} '{}' from favorites", item_type, item_id);
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "message": "Item removed from favorites"
                    })),
                )
            } else {
                info!("Item {} '{}' was not in favorites", item_type, item_id);
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "message": "Item was not in favorites"
                    })),
                )
            }
        }
        Err(err) => {
            error!("Error removing from favorites: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to remove from favorites: {}", err)
                })),
            )
        }
    }
}
