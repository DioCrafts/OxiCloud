use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde_json::json;
use tracing::{debug, error, instrument, warn};

// use crate::application::ports::trash_ports::TrashUseCase;
use crate::common::di::AppState;
use crate::interfaces::middleware::auth::{AuthUser, OptionalAuthUser};
use std::sync::Arc;

/// Gets all items in the trash for the current user
#[instrument(skip_all)]
pub async fn get_trash_items(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> (StatusCode, Json<serde_json::Value>) {
    // SECURITY: Always use the authenticated user's ID from the JWT token.
    // Never allow user ID override via query parameters to prevent
    // privilege escalation attacks.
    let effective_user = auth_user.id.clone();

    debug!("Request to list trash items for user {}", effective_user);

    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (
                StatusCode::NOT_IMPLEMENTED,
                Json(json!({
                    "error": "Trash feature is not enabled"
                })),
            );
        }
    };

    let result = trash_service.get_trash_items(&effective_user).await;

    match result {
        Ok(items) => {
            debug!("Found {} items in trash", items.len());
            (StatusCode::OK, Json(json!(items)))
        }
        Err(e) => {
            error!("Error retrieving trash items: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Error retrieving trash items: {}", e)
                })),
            )
        }
    }
}

/// Moves an item (file or folder) to the trash (generic function, not used directly in routes)
#[instrument(skip_all)]
pub async fn move_to_trash(
    State(state): State<Arc<AppState>>,
    OptionalAuthUser(auth_user): OptionalAuthUser,
    Path((item_type, item_id)): Path<(String, String)>,
) -> (StatusCode, Json<serde_json::Value>) {
    let user_id = auth_user
        .as_ref()
        .map(|u| u.id.as_str())
        .unwrap_or("anonymous");
    debug!(
        "Request to move to trash: type={}, id={}, user={}",
        item_type, item_id, user_id
    );

    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (
                StatusCode::NOT_IMPLEMENTED,
                Json(json!({
                    "error": "Trash feature is not enabled"
                })),
            );
        }
    };
    let result = trash_service
        .move_to_trash(&item_id, &item_type, user_id)
        .await;

    match result {
        Ok(_) => {
            debug!("Item moved to trash successfully");
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": "Item moved to trash successfully"
                })),
            )
        }
        Err(e) => {
            error!("Error moving item to trash: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Error moving item to trash: {}", e)
                })),
            )
        }
    }
}

/// Moves a file to the trash
#[instrument(skip_all)]
pub async fn move_file_to_trash(
    State(state): State<Arc<AppState>>,
    OptionalAuthUser(auth_user): OptionalAuthUser,
    Path(item_id): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    let user_id = auth_user
        .as_ref()
        .map(|u| u.id.as_str())
        .unwrap_or("anonymous");
    debug!(
        "Request to move file to trash: id={}, user={}",
        item_id, user_id
    );

    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (
                StatusCode::NOT_IMPLEMENTED,
                Json(json!({
                    "error": "Trash feature is not enabled"
                })),
            );
        }
    };

    // Specify that it is a file
    let result = trash_service.move_to_trash(&item_id, "file", user_id).await;

    match result {
        Ok(_) => {
            debug!("File moved to trash successfully");
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": "File moved to trash successfully"
                })),
            )
        }
        Err(e) => {
            error!("Error moving file to trash: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Error moving file to trash: {}", e)
                })),
            )
        }
    }
}

/// Moves a folder to the trash
#[instrument(skip_all)]
pub async fn move_folder_to_trash(
    State(state): State<Arc<AppState>>,
    OptionalAuthUser(auth_user): OptionalAuthUser,
    Path(item_id): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    let user_id = auth_user
        .as_ref()
        .map(|u| u.id.as_str())
        .unwrap_or("anonymous");
    debug!(
        "Request to move folder to trash: id={}, user={}",
        item_id, user_id
    );

    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (
                StatusCode::NOT_IMPLEMENTED,
                Json(json!({
                    "error": "Trash feature is not enabled"
                })),
            );
        }
    };

    // Specify that it is a folder
    let result = trash_service
        .move_to_trash(&item_id, "folder", user_id)
        .await;

    match result {
        Ok(_) => {
            debug!("Folder moved to trash successfully");
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": "Folder moved to trash successfully"
                })),
            )
        }
        Err(e) => {
            error!("Error moving folder to trash: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Error moving folder to trash: {}", e)
                })),
            )
        }
    }
}

/// Restores an item from the trash to its original location
#[instrument(skip_all)]
pub async fn restore_from_trash(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(trash_id): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    debug!("Request to restore item {} from trash", trash_id);

    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (
                StatusCode::NOT_IMPLEMENTED,
                Json(json!({
                    "error": "Trash feature is not enabled"
                })),
            );
        }
    };
    let result = trash_service.restore_item(&trash_id, &auth_user.id).await;

    match result {
        Ok(_) => {
            debug!("Item restored successfully");
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": "Item restored successfully"
                })),
            )
        }
        Err(e) => {
            let err_str = format!("{}", e);
            // If item not found, report success (it was already restored or removed)
            if err_str.contains("not found") || err_str.contains("NotFound") {
                warn!(
                    "Item not found in trash, but reporting success: {}",
                    trash_id
                );
                return (
                    StatusCode::OK,
                    Json(json!({
                        "success": true,
                        "message": "Item restored (or was already removed from trash)"
                    })),
                );
            }

            error!("Error restoring item from trash: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Error restoring item from trash: {}", e)
                })),
            )
        }
    }
}

/// Permanently deletes an item from the trash
#[instrument(skip_all)]
pub async fn delete_permanently(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(trash_id): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    debug!("Request to permanently delete item {}", trash_id);

    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (
                StatusCode::NOT_IMPLEMENTED,
                Json(json!({
                    "error": "Trash feature is not enabled"
                })),
            );
        }
    };
    let result = trash_service
        .delete_permanently(&trash_id, &auth_user.id)
        .await;

    match result {
        Ok(_) => {
            debug!("Item permanently deleted");
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": "Item deleted permanently"
                })),
            )
        }
        Err(e) => {
            let err_str = format!("{}", e);
            // If item not found, report success (it was already deleted)
            if err_str.contains("not found") || err_str.contains("NotFound") {
                warn!(
                    "Item not found in trash, but reporting success: {}",
                    trash_id
                );
                return (
                    StatusCode::OK,
                    Json(json!({
                        "success": true,
                        "message": "Item deleted (or was already removed from trash)"
                    })),
                );
            }

            error!("Error permanently deleting item: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Error deleting item permanently: {}", e)
                })),
            )
        }
    }
}

/// Empties the trash completely for the current user
#[instrument(skip_all)]
pub async fn empty_trash(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> (StatusCode, Json<serde_json::Value>) {
    debug!("Request to empty trash for user {}", auth_user.id);

    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (
                StatusCode::NOT_IMPLEMENTED,
                Json(json!({
                    "error": "Trash feature is not enabled"
                })),
            );
        }
    };
    let result = trash_service.empty_trash(&auth_user.id).await;

    match result {
        Ok(_) => {
            debug!("Trash emptied successfully");
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": "Trash emptied successfully"
                })),
            )
        }
        Err(e) => {
            error!("Error emptying trash: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Error emptying trash: {}", e)
                })),
            )
        }
    }
}
