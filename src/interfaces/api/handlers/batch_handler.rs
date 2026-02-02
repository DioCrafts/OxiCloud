use std::sync::Arc;
use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::application::services::batch_operations::{
    BatchOperationService, BatchResult, BatchStats
};
use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;
use crate::interfaces::api::handlers::ApiResult;

/// Estado compartido para el handler de batch
#[derive(Clone)]
pub struct BatchHandlerState {
    pub batch_service: Arc<BatchOperationService>,
}

/// DTO para las solicitudes de operaciones en lote de archivos
#[derive(Debug, Deserialize)]
pub struct BatchFileOperationRequest {
    /// IDs de los archivos a procesar
    pub file_ids: Vec<String>,
    /// ID de la carpeta destino (opcional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_folder_id: Option<String>,
}

/// DTO para las solicitudes de operaciones en lote de carpetas
#[derive(Debug, Deserialize)]
pub struct BatchFolderOperationRequest {
    /// IDs de las carpetas a procesar
    pub folder_ids: Vec<String>,
    /// Si la operación debe ser recursiva
    #[serde(default)]
    pub recursive: bool,
    /// ID de la carpeta destino (opcional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_folder_id: Option<String>,
}

/// DTO para las solicitudes de creación en lote de carpetas
#[derive(Debug, Deserialize)]
pub struct BatchCreateFoldersRequest {
    /// Detalles de las carpetas a crear
    pub folders: Vec<CreateFolderDetail>,
}

/// Detalle para creación de una carpeta
#[derive(Debug, Deserialize)]
pub struct CreateFolderDetail {
    /// Nombre de la carpeta
    pub name: String,
    /// ID de la carpeta padre (opcional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

/// DTO para los resultados de operaciones en lote
#[derive(Debug, Serialize)]
pub struct BatchOperationResponse<T> {
    /// Entidades procesadas exitosamente
    pub successful: Vec<T>,
    /// Operaciones fallidas con sus mensajes de error
    pub failed: Vec<FailedOperation>,
    /// Estadísticas de la operación
    pub stats: BatchOperationStats,
}

/// Operación fallida en un lote
#[derive(Debug, Serialize)]
pub struct FailedOperation {
    /// Identificador de la entidad que falló
    pub id: String,
    /// Mensaje de error
    pub error: String,
}

/// Estadísticas de una operación por lotes
#[derive(Debug, Serialize)]
pub struct BatchOperationStats {
    /// Número total de operaciones
    pub total: usize,
    /// Número de operaciones exitosas
    pub successful: usize,
    /// Número de operaciones fallidas
    pub failed: usize,
    /// Tiempo total de ejecución en milisegundos
    pub execution_time_ms: u128,
}

/// Convierte BatchStats del dominio a DTO
impl From<BatchStats> for BatchOperationStats {
    fn from(stats: BatchStats) -> Self {
        Self {
            total: stats.total,
            successful: stats.successful,
            failed: stats.failed,
            execution_time_ms: stats.execution_time_ms,
        }
    }
}

/// Convierte BatchResult<T> del dominio a DTO
impl<T, U> From<BatchResult<T>> for BatchOperationResponse<U>
where
    U: From<T>,
{
    fn from(result: BatchResult<T>) -> Self {
        let successful = result.successful.into_iter().map(U::from).collect();
        
        let failed = result.failed.into_iter()
            .map(|(id, error)| FailedOperation { id, error })
            .collect();
        
        Self {
            successful,
            failed,
            stats: result.stats.into(),
        }
    }
}

/// Handler para mover múltiples archivos en lote
pub async fn move_files_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchFileOperationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verificar que hay archivos para procesar
    if request.file_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No file IDs provided"
            }))
        ).into_response());
    }
    
    // Ejecutar operación de lote
    let result = state.batch_service
        .move_files(request.file_ids, request.target_folder_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Convertir resultado a DTO
    let response: BatchOperationResponse<FileDto> = result.into();
    
    // Determinar código de estado basado en los resultados
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Algunas operaciones exitosas, otras fallidas
        } else {
            StatusCode::BAD_REQUEST // Todas fallaron
        }
    } else {
        StatusCode::OK // Todas exitosas
    };
    
    Ok((status_code, Json(response)).into_response())
}

/// Handler para copiar múltiples archivos en lote
pub async fn copy_files_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchFileOperationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verificar que hay archivos para procesar
    if request.file_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No file IDs provided"
            }))
        ).into_response());
    }
    
    // Ejecutar operación de lote
    let result = state.batch_service
        .copy_files(request.file_ids, request.target_folder_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Convertir resultado a DTO
    let response: BatchOperationResponse<FileDto> = result.into();
    
    // Determinar código de estado basado en los resultados
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Algunas operaciones exitosas, otras fallidas
        } else {
            StatusCode::BAD_REQUEST // Todas fallaron
        }
    } else {
        StatusCode::OK // Todas exitosas
    };
    
    Ok((status_code, Json(response)).into_response())
}

/// Handler para eliminar múltiples archivos en lote
pub async fn delete_files_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchFileOperationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verificar que hay archivos para procesar
    if request.file_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No file IDs provided"
            }))
        ).into_response());
    }
    
    // Ejecutar operación de lote
    let result = state.batch_service
        .delete_files(request.file_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Crear respuesta personalizada para IDs de string
    let response = BatchOperationResponse {
        successful: result.successful,
        failed: result.failed.into_iter()
            .map(|(id, error)| FailedOperation { id, error })
            .collect(),
        stats: result.stats.into(),
    };
    
    // Determinar código de estado basado en los resultados
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Algunas operaciones exitosas, otras fallidas
        } else {
            StatusCode::BAD_REQUEST // Todas fallaron
        }
    } else {
        StatusCode::OK // Todas exitosas
    };
    
    Ok((status_code, Json(response)).into_response())
}

/// Handler para eliminar múltiples carpetas en lote
pub async fn delete_folders_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchFolderOperationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verificar que hay carpetas para procesar
    if request.folder_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No folder IDs provided"
            }))
        ).into_response());
    }
    
    // Ejecutar operación de lote
    let result = state.batch_service
        .delete_folders(request.folder_ids, request.recursive)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Crear respuesta personalizada para IDs de string
    let response = BatchOperationResponse {
        successful: result.successful,
        failed: result.failed.into_iter()
            .map(|(id, error)| FailedOperation { id, error })
            .collect(),
        stats: result.stats.into(),
    };
    
    // Determinar código de estado basado en los resultados
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Algunas operaciones exitosas, otras fallidas
        } else {
            StatusCode::BAD_REQUEST // Todas fallaron
        }
    } else {
        StatusCode::OK // Todas exitosas
    };
    
    Ok((status_code, Json(response)).into_response())
}

/// Handler para crear múltiples carpetas en lote
pub async fn create_folders_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchCreateFoldersRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verificar que hay carpetas para procesar
    if request.folders.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No folders provided"
            }))
        ).into_response());
    }
    
    // Transformar el formato para el servicio
    let folders = request.folders
        .into_iter()
        .map(|detail| (detail.name, detail.parent_id))
        .collect();
    
    // Ejecutar operación de lote
    let result = state.batch_service
        .create_folders(folders)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Convertir resultado a DTO
    let response: BatchOperationResponse<FolderDto> = result.into();
    
    // Determinar código de estado basado en los resultados
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Algunas operaciones exitosas, otras fallidas
        } else {
            StatusCode::BAD_REQUEST // Todas fallaron
        }
    } else {
        StatusCode::CREATED // Todas exitosas
    };
    
    Ok((status_code, Json(response)).into_response())
}

/// Handler para obtener múltiples archivos en lote
pub async fn get_files_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchFileOperationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verificar que hay archivos para procesar
    if request.file_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No file IDs provided"
            }))
        ).into_response());
    }
    
    // Ejecutar operación de lote
    let result = state.batch_service
        .get_multiple_files(request.file_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Convertir resultado a DTO
    let response: BatchOperationResponse<FileDto> = result.into();
    
    // Determinar código de estado basado en los resultados
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Algunas operaciones exitosas, otras fallidas
        } else {
            StatusCode::BAD_REQUEST // Todas fallaron
        }
    } else {
        StatusCode::OK // Todas exitosas
    };
    
    Ok((status_code, Json(response)).into_response())
}

/// Handler para obtener múltiples carpetas en lote
pub async fn get_folders_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchFolderOperationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verificar que hay carpetas para procesar
    if request.folder_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No folder IDs provided"
            }))
        ).into_response());
    }
    
    // Ejecutar operación de lote
    let result = state.batch_service
        .get_multiple_folders(request.folder_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Convertir resultado a DTO
    let response: BatchOperationResponse<FolderDto> = result.into();
    
    // Determinar código de estado basado en los resultados
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Algunas operaciones exitosas, otras fallidas
        } else {
            StatusCode::BAD_REQUEST // Todas fallaron
        }
    } else {
        StatusCode::OK // Todas exitosas
    };
    
    Ok((status_code, Json(response)).into_response())
}