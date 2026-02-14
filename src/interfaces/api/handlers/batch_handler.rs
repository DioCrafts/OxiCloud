use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;
use crate::application::services::batch_operations::{
    BatchOperationService, BatchResult, BatchStats,
};
use crate::interfaces::api::handlers::ApiResult;

/// Shared state for the batch handler
#[derive(Clone)]
pub struct BatchHandlerState {
    pub batch_service: Arc<BatchOperationService>,
}

/// DTO for batch file operation requests
#[derive(Debug, Deserialize)]
pub struct BatchFileOperationRequest {
    /// IDs of the files to process
    pub file_ids: Vec<String>,
    /// Target folder ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_folder_id: Option<String>,
}

/// DTO for batch folder operation requests
#[derive(Debug, Deserialize)]
pub struct BatchFolderOperationRequest {
    /// IDs of the folders to process
    pub folder_ids: Vec<String>,
    /// Whether the operation should be recursive
    #[serde(default)]
    pub recursive: bool,
    /// Target folder ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_folder_id: Option<String>,
}

/// DTO for batch folder creation requests
#[derive(Debug, Deserialize)]
pub struct BatchCreateFoldersRequest {
    /// Details of the folders to create
    pub folders: Vec<CreateFolderDetail>,
}

/// Detail for folder creation
#[derive(Debug, Deserialize)]
pub struct CreateFolderDetail {
    /// Folder name
    pub name: String,
    /// Parent folder ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

/// DTO for batch operation results
#[derive(Debug, Serialize)]
pub struct BatchOperationResponse<T> {
    /// Successfully processed entities
    pub successful: Vec<T>,
    /// Failed operations with their error messages
    pub failed: Vec<FailedOperation>,
    /// Operation statistics
    pub stats: BatchOperationStats,
}

/// Failed operation in a batch
#[derive(Debug, Serialize)]
pub struct FailedOperation {
    /// Identifier of the entity that failed
    pub id: String,
    /// Error message
    pub error: String,
}

/// Statistics for a batch operation
#[derive(Debug, Serialize)]
pub struct BatchOperationStats {
    /// Total number of operations
    pub total: usize,
    /// Number of successful operations
    pub successful: usize,
    /// Number of failed operations
    pub failed: usize,
    /// Total execution time in milliseconds
    pub execution_time_ms: u128,
}

/// Converts domain BatchStats to DTO
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

/// Converts domain BatchResult<T> to DTO
impl<T, U> From<BatchResult<T>> for BatchOperationResponse<U>
where
    U: From<T>,
{
    fn from(result: BatchResult<T>) -> Self {
        let successful = result.successful.into_iter().map(U::from).collect();

        let failed = result
            .failed
            .into_iter()
            .map(|(id, error)| FailedOperation { id, error })
            .collect();

        Self {
            successful,
            failed,
            stats: result.stats.into(),
        }
    }
}

/// Handler for moving multiple files in batch
pub async fn move_files_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchFileOperationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verify there are files to process
    if request.file_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No file IDs provided"
            })),
        )
            .into_response());
    }

    // Execute batch operation
    let result = state
        .batch_service
        .move_files(request.file_ids, request.target_folder_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Convert result to DTO
    let response: BatchOperationResponse<FileDto> = result.into();

    // Determine status code based on results
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Some operations successful, others failed
        } else {
            StatusCode::BAD_REQUEST // All failed
        }
    } else {
        StatusCode::OK // All successful
    };

    Ok((status_code, Json(response)).into_response())
}

/// Handler for copying multiple files in batch
pub async fn copy_files_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchFileOperationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verify there are files to process
    if request.file_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No file IDs provided"
            })),
        )
            .into_response());
    }

    // Execute batch operation
    let result = state
        .batch_service
        .copy_files(request.file_ids, request.target_folder_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Convert result to DTO
    let response: BatchOperationResponse<FileDto> = result.into();

    // Determine status code based on results
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Some operations successful, others failed
        } else {
            StatusCode::BAD_REQUEST // All failed
        }
    } else {
        StatusCode::OK // All successful
    };

    Ok((status_code, Json(response)).into_response())
}

/// Handler for deleting multiple files in batch
pub async fn delete_files_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchFileOperationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verify there are files to process
    if request.file_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No file IDs provided"
            })),
        )
            .into_response());
    }

    // Execute batch operation
    let result = state
        .batch_service
        .delete_files(request.file_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Create custom response for string IDs
    let response = BatchOperationResponse {
        successful: result.successful,
        failed: result
            .failed
            .into_iter()
            .map(|(id, error)| FailedOperation { id, error })
            .collect(),
        stats: result.stats.into(),
    };

    // Determine status code based on results
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Some operations successful, others failed
        } else {
            StatusCode::BAD_REQUEST // All failed
        }
    } else {
        StatusCode::OK // All successful
    };

    Ok((status_code, Json(response)).into_response())
}

/// Handler for deleting multiple folders in batch
pub async fn delete_folders_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchFolderOperationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verify there are folders to process
    if request.folder_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No folder IDs provided"
            })),
        )
            .into_response());
    }

    // Execute batch operation
    let result = state
        .batch_service
        .delete_folders(request.folder_ids, request.recursive)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Create custom response for string IDs
    let response = BatchOperationResponse {
        successful: result.successful,
        failed: result
            .failed
            .into_iter()
            .map(|(id, error)| FailedOperation { id, error })
            .collect(),
        stats: result.stats.into(),
    };

    // Determine status code based on results
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Some operations successful, others failed
        } else {
            StatusCode::BAD_REQUEST // All failed
        }
    } else {
        StatusCode::OK // All successful
    };

    Ok((status_code, Json(response)).into_response())
}

/// Handler for creating multiple folders in batch
pub async fn create_folders_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchCreateFoldersRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verify there are folders to process
    if request.folders.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No folders provided"
            })),
        )
            .into_response());
    }

    // Transform the format for the service
    let folders = request
        .folders
        .into_iter()
        .map(|detail| (detail.name, detail.parent_id))
        .collect();

    // Execute batch operation
    let result = state
        .batch_service
        .create_folders(folders)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Convert result to DTO
    let response: BatchOperationResponse<FolderDto> = result.into();

    // Determine status code based on results
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Some operations successful, others failed
        } else {
            StatusCode::BAD_REQUEST // All failed
        }
    } else {
        StatusCode::CREATED // All successful
    };

    Ok((status_code, Json(response)).into_response())
}

/// Handler for getting multiple files in batch
pub async fn get_files_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchFileOperationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verify there are files to process
    if request.file_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No file IDs provided"
            })),
        )
            .into_response());
    }

    // Execute batch operation
    let result = state
        .batch_service
        .get_multiple_files(request.file_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Convert result to DTO
    let response: BatchOperationResponse<FileDto> = result.into();

    // Determine status code based on results
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Some operations successful, others failed
        } else {
            StatusCode::BAD_REQUEST // All failed
        }
    } else {
        StatusCode::OK // All successful
    };

    Ok((status_code, Json(response)).into_response())
}

/// Handler for getting multiple folders in batch
pub async fn get_folders_batch(
    State(state): State<BatchHandlerState>,
    Json(request): Json<BatchFolderOperationRequest>,
) -> ApiResult<impl IntoResponse> {
    // Verify there are folders to process
    if request.folder_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No folder IDs provided"
            })),
        )
            .into_response());
    }

    // Execute batch operation
    let result = state
        .batch_service
        .get_multiple_folders(request.folder_ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Convert result to DTO
    let response: BatchOperationResponse<FolderDto> = result.into();

    // Determine status code based on results
    let status_code = if response.stats.failed > 0 {
        if response.stats.successful > 0 {
            StatusCode::PARTIAL_CONTENT // Some operations successful, others failed
        } else {
            StatusCode::BAD_REQUEST // All failed
        }
    } else {
        StatusCode::OK // All successful
    };

    Ok((status_code, Json(response)).into_response())
}
