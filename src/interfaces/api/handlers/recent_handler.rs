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

/// Parámetros de consulta para obtener elementos recientes
#[derive(Deserialize)]
pub struct GetRecentParams {
    #[serde(default)]
    limit: Option<i32>,
}

/// Obtener elementos recientes del usuario
pub async fn get_recent_items(
    State(recent_service): State<Arc<dyn RecentItemsUseCase>>,
    auth_user: AuthUser,
    Query(params): Query<GetRecentParams>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match recent_service.get_recent_items(user_id, params.limit).await {
        Ok(items) => {
            info!("Recuperados {} elementos recientes para usuario", items.len());
            (StatusCode::OK, Json(items)).into_response()
        },
        Err(err) => {
            error!("Error al recuperar elementos recientes: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(serde_json::json!({
                    "error": format!("Fallo al recuperar elementos recientes: {}", err)
                }))
            ).into_response()
        }
    }
}

/// Registrar acceso a un elemento
pub async fn record_item_access(
    State(recent_service): State<Arc<dyn RecentItemsUseCase>>,
    auth_user: AuthUser,
    Path((item_type, item_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    // Validar tipo de elemento
    if item_type != "file" && item_type != "folder" {
        return (
            StatusCode::BAD_REQUEST, 
            Json(serde_json::json!({
                "error": "El tipo de elemento debe ser 'file' o 'folder'"
            }))
        ).into_response();
    }
    
    match recent_service.record_item_access(user_id, &item_id, &item_type).await {
        Ok(_) => {
            info!("Registrado acceso a {} '{}' en recientes", item_type, item_id);
            (
                StatusCode::OK, 
                Json(serde_json::json!({
                    "message": "Acceso registrado correctamente"
                }))
            ).into_response()
        },
        Err(err) => {
            error!("Error al registrar acceso en recientes: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(serde_json::json!({
                    "error": format!("Fallo al registrar acceso: {}", err)
                }))
            ).into_response()
        }
    }
}

/// Eliminar un elemento de recientes
pub async fn remove_from_recent(
    State(recent_service): State<Arc<dyn RecentItemsUseCase>>,
    auth_user: AuthUser,
    Path((item_type, item_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match recent_service.remove_from_recent(user_id, &item_id, &item_type).await {
        Ok(removed) => {
            if removed {
                info!("Eliminado {} '{}' de recientes", item_type, item_id);
                (
                    StatusCode::OK, 
                    Json(serde_json::json!({
                        "message": "Elemento eliminado de recientes"
                    }))
                ).into_response()
            } else {
                info!("Elemento {} '{}' no estaba en recientes", item_type, item_id);
                (
                    StatusCode::NOT_FOUND, 
                    Json(serde_json::json!({
                        "message": "Elemento no estaba en recientes"
                    }))
                ).into_response()
            }
        },
        Err(err) => {
            error!("Error al eliminar de recientes: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(serde_json::json!({
                    "error": format!("Fallo al eliminar de recientes: {}", err)
                }))
            ).into_response()
        }
    }
}

/// Limpiar todos los elementos recientes
pub async fn clear_recent_items(
    State(recent_service): State<Arc<dyn RecentItemsUseCase>>,
    auth_user: AuthUser,
) -> impl IntoResponse {
    let user_id = &auth_user.id;
    
    match recent_service.clear_recent_items(user_id).await {
        Ok(_) => {
            info!("Limpiados todos los elementos recientes para usuario");
            (
                StatusCode::OK, 
                Json(serde_json::json!({
                    "message": "Elementos recientes limpiados correctamente"
                }))
            ).into_response()
        },
        Err(err) => {
            error!("Error al limpiar elementos recientes: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(serde_json::json!({
                    "error": format!("Fallo al limpiar elementos recientes: {}", err)
                }))
            ).into_response()
        }
    }
}