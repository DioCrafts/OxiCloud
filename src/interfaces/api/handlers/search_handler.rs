use axum::{
    extract::{State, Query, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde_json::json;
use tracing::{info, error};

use crate::application::dtos::search_dto::SearchCriteriaDto;
use crate::common::di::AppState;

/**
 * Handler for search operations through the API.
 * 
 * This handler exposes endpoints related to search functionality,
 * allowing users to search for files and folders using various criteria.
 */
pub struct SearchHandler;

impl SearchHandler {
    /**
     * Performs a search based on the criteria provided as query parameters.
     * 
     * This endpoint allows simple searches directly with URL parameters.
     * 
     * @param state Application state with services
     * @param query_params Search parameters as query string
     * @return HTTP response with the search results
     */
    pub async fn search_files_get(
        State(state): State<AppState>,
        Query(params): Query<SearchParams>,
    ) -> impl IntoResponse {
        info!("API: File search with parameters: {:?}", params);
        
        // Extract the search service or return error if not available
        let search_service = match &state.applications.search_service {
            Some(service) => service,
            None => {
                error!("Search service not available");
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(json!({
                        "error": "Search service is not available"
                    }))
                ).into_response();
            }
        };
        
        // Convert search parameters to DTO
        let search_criteria = SearchCriteriaDto {
            name_contains: params.query,
            file_types: params.type_filter.map(|t| t.split(',').map(|s| s.trim().to_string()).collect()),
            created_after: params.created_after,
            created_before: params.created_before,
            modified_after: params.modified_after,
            modified_before: params.modified_before,
            min_size: params.min_size,
            max_size: params.max_size,
            folder_id: params.folder_id,
            recursive: params.recursive.unwrap_or(true),
            limit: params.limit.unwrap_or(100),
            offset: params.offset.unwrap_or(0),
        };
        
        // Perform the search
        match search_service.search(search_criteria).await {
            Ok(results) => {
                info!("Search completed, {} files and {} folders found", 
                     results.files.len(), results.folders.len());
                (StatusCode::OK, Json(results)).into_response()
            },
            Err(err) => {
                error!("Search error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": format!("Search error: {}", err)
                    }))
                ).into_response()
            }
        }
    }
    
    /**
     * Performs an advanced search based on a complete JSON criteria object.
     * 
     * This endpoint allows more complex searches with all possible criteria
     * provided in the request body.
     * 
     * @param state Application state with services
     * @param criteria Complete search criteria
     * @return HTTP response with the search results
     */
    pub async fn search_files_post(
        State(state): State<AppState>,
        Json(criteria): Json<SearchCriteriaDto>,
    ) -> impl IntoResponse {
        info!("API: Advanced file search");
        
        // Extract the search service or return error if not available
        let search_service = match &state.applications.search_service {
            Some(service) => service,
            None => {
                error!("Search service not available");
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(json!({
                        "error": "Search service is not available"
                    }))
                ).into_response();
            }
        };
        
        // Perform the search
        match search_service.search(criteria).await {
            Ok(results) => {
                info!("Search completed, {} files and {} folders found", 
                     results.files.len(), results.folders.len());
                (StatusCode::OK, Json(results)).into_response()
            },
            Err(err) => {
                error!("Search error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": format!("Search error: {}", err)
                    }))
                ).into_response()
            }
        }
    }
    
    /**
     * Clears the search results cache.
     * 
     * This endpoint is useful for forcing fresh searches after significant
     * changes in the file system.
     * 
     * @param state Application state with services
     * @return HTTP response indicating success or error
     */
    pub async fn clear_search_cache(
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        info!("API: Clearing search cache");
        
        // Extract the search service or return error if not available
        let search_service = match &state.applications.search_service {
            Some(service) => service,
            None => {
                error!("Search service not available");
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(json!({
                        "error": "Search service is not available"
                    }))
                ).into_response();
            }
        };
        
        // Clear the cache
        match search_service.clear_search_cache().await {
            Ok(_) => {
                info!("Search cache cleared successfully");
                (
                    StatusCode::OK,
                    Json(json!({
                        "message": "Search cache cleared successfully"
                    }))
                ).into_response()
            },
            Err(err) => {
                error!("Error clearing search cache: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": format!("Error clearing search cache: {}", err)
                    }))
                ).into_response()
            }
        }
    }
}

/// Search parameters for the GET endpoint
#[derive(Debug, serde::Deserialize)]
pub struct SearchParams {
    /// Text to search for in file and folder names
    pub query: Option<String>,
    
    /// Filter by file types (comma-separated extensions)
    #[serde(rename = "type")]
    pub type_filter: Option<String>,
    
    /// Filter items created after this date (timestamp)
    pub created_after: Option<u64>,
    
    /// Filter items created before this date (timestamp)
    pub created_before: Option<u64>,
    
    /// Filter items modified after this date (timestamp)
    pub modified_after: Option<u64>,
    
    /// Filter items modified before this date (timestamp)
    pub modified_before: Option<u64>,
    
    /// Minimum size in bytes
    pub min_size: Option<u64>,
    
    /// Maximum size in bytes
    pub max_size: Option<u64>,
    
    /// Folder ID to limit the search scope
    pub folder_id: Option<String>,
    
    /// Recursive search in subfolders
    pub recursive: Option<bool>,
    
    /// Result limit for pagination
    pub limit: Option<usize>,
    
    /// Offset for pagination
    pub offset: Option<usize>,
}