use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;
use tracing::{debug, error, warn, instrument};

// use crate::application::ports::trash_ports::TrashUseCase;
use crate::common::di::AppState;
use crate::interfaces::middleware::auth::AuthUser;

/// Obtiene todos los elementos en la papelera para el usuario actual
#[instrument(skip_all)]
pub async fn get_trash_items(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> (StatusCode, Json<serde_json::Value>) {
    // SECURITY: Always use the authenticated user's ID from the JWT token.
    // Never allow user ID override via query parameters to prevent
    // privilege escalation attacks.
    let effective_user = auth_user.id.clone();

    debug!("Solicitud para listar elementos en papelera para usuario {}", effective_user);
    
    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Trash feature is not enabled"
            })));
        }
    };
    
    let result = trash_service.get_trash_items(&effective_user).await;
    
    match result {
        Ok(items) => {
            debug!("Encontrados {} elementos en la papelera", items.len());
            (StatusCode::OK, Json(json!(items)))
        },
        Err(e) => {
            error!("Error al obtener elementos de la papelera: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Error retrieving trash items: {}", e)
            })))
        }
    }
}

/// Mueve un elemento (archivo o carpeta) a la papelera (función genérica, no usada directamente en rutas)
#[instrument(skip_all)]
pub async fn move_to_trash(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((item_type, item_id)): Path<(String, String)>,
) -> (StatusCode, Json<serde_json::Value>) {
    debug!("Solicitud para mover a papelera: tipo={}, id={}, usuario={}", 
           item_type, item_id, auth_user.id);
    
    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Trash feature is not enabled"
            })));
        }
    };
    let result = trash_service.move_to_trash(&item_id, &item_type, &auth_user.id).await;
    
    match result {
        Ok(_) => {
            debug!("Elemento movido a papelera con éxito");
            (StatusCode::OK, Json(json!({
                "success": true,
                "message": "Item moved to trash successfully"
            })))
        },
        Err(e) => {
            error!("Error al mover elemento a papelera: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Error moving item to trash: {}", e)
            })))
        }
    }
}

/// Mueve un archivo a la papelera
#[instrument(skip_all)]
pub async fn move_file_to_trash(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(item_id): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    debug!("Solicitud para mover archivo a papelera: id={}, usuario={}", 
           item_id, auth_user.id);
    
    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Trash feature is not enabled"
            })));
        }
    };
    
    // Especificar que es un archivo
    let result = trash_service.move_to_trash(&item_id, "file", &auth_user.id).await;
    
    match result {
        Ok(_) => {
            debug!("Archivo movido a papelera con éxito");
            (StatusCode::OK, Json(json!({
                "success": true,
                "message": "File moved to trash successfully"
            })))
        },
        Err(e) => {
            error!("Error al mover archivo a papelera: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Error moving file to trash: {}", e)
            })))
        }
    }
}

/// Mueve una carpeta a la papelera
#[instrument(skip_all)]
pub async fn move_folder_to_trash(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(item_id): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    debug!("Solicitud para mover carpeta a papelera: id={}, usuario={}", 
           item_id, auth_user.id);
    
    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Trash feature is not enabled"
            })));
        }
    };
    
    // Especificar que es una carpeta
    let result = trash_service.move_to_trash(&item_id, "folder", &auth_user.id).await;
    
    match result {
        Ok(_) => {
            debug!("Carpeta movida a papelera con éxito");
            (StatusCode::OK, Json(json!({
                "success": true,
                "message": "Folder moved to trash successfully"
            })))
        },
        Err(e) => {
            error!("Error al mover carpeta a papelera: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Error moving folder to trash: {}", e)
            })))
        }
    }
}

/// Restaura un elemento desde la papelera a su ubicación original
#[instrument(skip_all)]
pub async fn restore_from_trash(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(trash_id): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    debug!("Solicitud para restaurar elemento {} de papelera", trash_id);
    
    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Trash feature is not enabled"
            })));
        }
    };
    let result = trash_service.restore_item(&trash_id, &auth_user.id).await;
    
    match result {
        Ok(_) => {
            debug!("Elemento restaurado con éxito");
            (StatusCode::OK, Json(json!({
                "success": true,
                "message": "Item restored successfully"
            })))
        },
        Err(e) => {
            let err_str = format!("{}", e);
            // If item not found, report success (it was already restored or removed)
            if err_str.contains("not found") || err_str.contains("NotFound") {
                warn!("Item not found in trash, but reporting success: {}", trash_id);
                return (StatusCode::OK, Json(json!({
                    "success": true,
                    "message": "Item restored (or was already removed from trash)"
                })));
            }

            error!("Error al restaurar elemento de papelera: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Error restoring item from trash: {}", e)
            })))
        }
    }
}

/// Elimina permanentemente un elemento de la papelera
#[instrument(skip_all)]
pub async fn delete_permanently(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(trash_id): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    debug!("Solicitud para eliminar permanentemente elemento {}", trash_id);
    
    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Trash feature is not enabled"
            })));
        }
    };
    let result = trash_service.delete_permanently(&trash_id, &auth_user.id).await;
    
    match result {
        Ok(_) => {
            debug!("Elemento eliminado permanentemente");
            (StatusCode::OK, Json(json!({
                "success": true,
                "message": "Item deleted permanently"
            })))
        },
        Err(e) => {
            let err_str = format!("{}", e);
            // If item not found, report success (it was already deleted)
            if err_str.contains("not found") || err_str.contains("NotFound") {
                warn!("Item not found in trash, but reporting success: {}", trash_id);
                return (StatusCode::OK, Json(json!({
                    "success": true,
                    "message": "Item deleted (or was already removed from trash)"
                })));
            }

            error!("Error al eliminar permanentemente elemento: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Error deleting item permanently: {}", e)
            })))
        }
    }
}

/// Vacía la papelera completamente para el usuario actual
#[instrument(skip_all)]
pub async fn empty_trash(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> (StatusCode, Json<serde_json::Value>) {
    debug!("Solicitud para vaciar papelera del usuario {}", auth_user.id);
    
    let trash_service = match state.trash_service.as_ref() {
        Some(service) => service,
        None => {
            return (StatusCode::NOT_IMPLEMENTED, Json(json!({
                "error": "Trash feature is not enabled"
            })));
        }
    };
    let result = trash_service.empty_trash(&auth_user.id).await;
    
    match result {
        Ok(_) => {
            debug!("Papelera vaciada con éxito");
            (StatusCode::OK, Json(json!({
                "success": true,
                "message": "Trash emptied successfully"
            })))
        },
        Err(e) => {
            error!("Error al vaciar papelera: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": format!("Error emptying trash: {}", e)
            })))
        }
    }
}