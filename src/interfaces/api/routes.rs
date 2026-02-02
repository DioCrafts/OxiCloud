use std::sync::Arc;
use std::collections::HashMap;
use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::{State, Query, Path},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use tower_http::{
    compression::CompressionLayer, 
    trace::TraceLayer,
};
use serde_json::json;
use crate::common::config::AppConfig;
use crate::common::di::AppState;

use crate::interfaces::middleware::cache::{HttpCache, start_cache_cleanup_task};

use crate::application::services::folder_service::FolderService;
use crate::application::services::file_service::FileService;
use crate::application::services::i18n_application_service::I18nApplicationService;
use crate::application::services::batch_operations::BatchOperationService;
use crate::application::ports::trash_ports::TrashUseCase;
use crate::application::ports::inbound::SearchUseCase;
use crate::application::ports::share_ports::ShareUseCase;
use crate::application::ports::favorites_ports::FavoritesUseCase;
use crate::application::ports::recent_ports::RecentItemsUseCase;

use crate::interfaces::api::handlers::folder_handler::FolderHandler;
use crate::interfaces::api::handlers::file_handler::FileHandler;
use crate::interfaces::api::handlers::i18n_handler::I18nHandler;
// Eliminamos la importación de ShareHandler ya que ahora usamos directamente el servicio
use crate::interfaces::api::handlers::batch_handler::{
    self, BatchHandlerState
};
use crate::application::dtos::pagination::PaginationRequestDto;

/// Creates API routes for the application
pub fn create_api_routes(
    folder_service: Arc<FolderService>, 
    file_service: Arc<FileService>,
    i18n_service: Option<Arc<I18nApplicationService>>,
    trash_service: Option<Arc<dyn TrashUseCase>>,
    search_service: Option<Arc<dyn SearchUseCase>>,
    share_service: Option<Arc<dyn ShareUseCase>>,
    favorites_service: Option<Arc<dyn FavoritesUseCase>>,
    recent_service: Option<Arc<dyn RecentItemsUseCase>>,
) -> Router<crate::common::di::AppState> {
    // Create a simplified AppState for the trash view
    // Setup required components for repository construction
    let path_service = Arc::new(crate::infrastructure::services::path_service::PathService::new(std::path::PathBuf::from("./storage")));
    let storage_mediator = Arc::new(crate::application::services::storage_mediator::FileSystemStorageMediator::new_stub());
    let id_mapping_service = Arc::new(crate::infrastructure::services::id_mapping_service::IdMappingService::dummy());
    let path_resolver = Arc::new(crate::infrastructure::repositories::file_path_resolver::FilePathResolver::new(
        path_service.clone(),
        storage_mediator.clone(),
        id_mapping_service.clone()
    ));
    let metadata_cache = Arc::new(crate::infrastructure::services::file_metadata_cache::FileMetadataCache::new(
        crate::common::config::AppConfig::default(),
        1000 // Default max entries
    ));
    
    // Create file and folder repositories
    let file_repository = Arc::new(crate::infrastructure::repositories::file_fs_repository::FileFsRepository::new(
        std::path::PathBuf::from("./storage"),
        storage_mediator.clone(),
        id_mapping_service.clone(),
        path_service.clone(),
        metadata_cache.clone(),
    ));
    
    let folder_repository = Arc::new(crate::infrastructure::repositories::folder_fs_repository::FolderFsRepository::new(
        std::path::PathBuf::from("./storage"),
        storage_mediator.clone(),
        id_mapping_service.clone(),
        path_service.clone(),
    ));
    
    // Create concrete id_mapping_service for optimizer
    let id_mapping_service_concrete = Arc::new(crate::infrastructure::services::id_mapping_service::IdMappingService::dummy());
    let id_mapping_optimizer = Arc::new(crate::infrastructure::services::id_mapping_optimizer::IdMappingOptimizer::new(id_mapping_service_concrete.clone()));
    
    let mut app_state = crate::common::di::AppState {
        core: crate::common::di::CoreServices {
            path_service: path_service.clone(),
            cache_manager: Arc::new(crate::infrastructure::services::cache_manager::StorageCacheManager::default()),
            id_mapping_service: id_mapping_service.clone(),
            file_id_mapping_service: id_mapping_service_concrete.clone(),
            id_mapping_optimizer: id_mapping_optimizer.clone(),
            config: crate::common::config::AppConfig::default(),
        },
        repositories: crate::common::di::RepositoryServices {
            folder_repository: folder_repository.clone(),
            file_repository: file_repository.clone(),
            file_read_repository: Arc::new(crate::infrastructure::repositories::FileFsReadRepository::default_stub()),
            file_write_repository: Arc::new(crate::infrastructure::repositories::FileFsWriteRepository::default_stub()),
            i18n_repository: Arc::new(crate::infrastructure::services::file_system_i18n_service::FileSystemI18nService::dummy()),
            storage_mediator: storage_mediator.clone(),
            metadata_manager: Arc::new(crate::infrastructure::repositories::FileMetadataManager::default()),
            path_resolver: path_resolver.clone(),
            metadata_cache: metadata_cache.clone(),
            trash_repository: None, // This is OK to be None since we use the trash_service directly
        },
        storage_usage_service: None,
        applications: crate::common::di::ApplicationServices {
            folder_service_concrete: folder_service.clone(),
            file_service_concrete: file_service.clone(),
            folder_service: folder_service.clone(),
            file_service: file_service.clone(),
            file_upload_service: Arc::new(crate::application::services::file_upload_service::FileUploadService::new(
                Arc::new(crate::infrastructure::repositories::FileFsWriteRepository::default_stub())
            )),
            file_retrieval_service: Arc::new(crate::application::services::file_retrieval_service::FileRetrievalService::new(
                Arc::new(crate::infrastructure::repositories::FileFsReadRepository::default_stub())
            )),
            file_management_service: Arc::new(crate::application::services::file_management_service::FileManagementService::new(
                Arc::new(crate::infrastructure::repositories::FileFsWriteRepository::default_stub())
            )),
            file_use_case_factory: Arc::new(crate::application::services::file_use_case_factory::AppFileUseCaseFactory::new(
                Arc::new(crate::infrastructure::repositories::FileFsReadRepository::default_stub()),
                Arc::new(crate::infrastructure::repositories::FileFsWriteRepository::default_stub())
            )),
            i18n_service: i18n_service.clone().unwrap_or_else(|| 
                Arc::new(crate::application::services::i18n_application_service::I18nApplicationService::dummy())
            ),
            trash_service: trash_service.clone(), // Include the trash service here too for consistency
            search_service: search_service.clone(), // Include the search service
            share_service: share_service.clone(), // Include the share service
            favorites_service: favorites_service.clone(), // Include the favorites service
            recent_service: recent_service.clone(), // Include the recent service
        },
        db_pool: None,
        auth_service: None,
        trash_service: trash_service.clone(), // This is the important part - include the trash service
        share_service: share_service.clone(), // Include the share service for routes
        favorites_service: favorites_service.clone(), // Include the favorites service for routes
        recent_service: recent_service.clone(), // Include the recent service for routes
        calendar_service: None, // Adding missing field
        contact_service: None   // Adding missing field
    };
    // Inicializar el servicio de operaciones por lotes
    let batch_service = Arc::new(BatchOperationService::default(
        file_service.clone(),
        folder_service.clone()
    ));
    
    // Crear estado para el manejador de operaciones por lotes
    let batch_handler_state = BatchHandlerState {
        batch_service: batch_service.clone(),
    };
    
    // Implement HTTP Cache
    let http_cache = HttpCache::new();
    
    // Define TTL values for different resource types (in seconds)
    let _folders_ttl = 300;      // 5 minutes
    let _files_list_ttl = 300;   // 5 minutes
    let _i18n_ttl = 3600;        // 1 hour
    
    // Start the cleanup task for HTTP cache
    start_cache_cleanup_task(http_cache.clone());
    
    // Create the basic folders router with service operations
    let folders_basic_router = Router::new()
        .route("/", post(FolderHandler::create_folder))
        .route("/", get(|State(service): State<Arc<FolderService>>| async move {
            // No parent ID means list root folders
            FolderHandler::list_folders(State(service), None).await
        }))
        .route("/paginated", get(|
            State(service): State<Arc<FolderService>>,
            pagination: Query<PaginationRequestDto>
        | async move {
            // Paginación para carpetas raíz (sin parent)
            FolderHandler::list_folders_paginated(State(service), pagination, None).await
        }))
        .route("/{id}", get(FolderHandler::get_folder))
        .route("/{id}/contents", get(|
            State(service): State<Arc<FolderService>>,
            Path(id): Path<String>
        | async move {
            // Listar contenido de una carpeta por su ID
            FolderHandler::list_folders(State(service), Some(&id)).await
        }))
        .route("/{id}/contents/paginated", get(|
            State(service): State<Arc<FolderService>>,
            Path(id): Path<String>,
            pagination: Query<PaginationRequestDto>
        | async move {
            // Listar contenido paginado de una carpeta por su ID
            FolderHandler::list_folders_paginated(State(service), pagination, Some(&id)).await
        }))
        .route("/{id}/rename", put(FolderHandler::rename_folder))
        .route("/{id}/move", put(FolderHandler::move_folder))
        .with_state(folder_service.clone());
        
    // Special route for ZIP download that requires AppState instead of just FolderService
    let folder_zip_router = Router::new()
        .route("/{id}/download", get(FolderHandler::download_folder_zip))
        .with_state(app_state.clone());
        
    // Create folder operations that use trash separately
    let folders_ops_router = Router::new()
        .route("/{id}", delete(|
            State(state): State<AppState>,
            Path(id): Path<String>
        | async move {
            // Try to use trash service if available
            if let Some(trash_service) = &state.trash_service {
                tracing::info!("Moving folder to trash: {}", id);
                let default_user = "default".to_string();
                
                match trash_service.move_to_trash(&id, "folder", &default_user).await {
                    Ok(_) => {
                        tracing::info!("Folder successfully moved to trash: {}", id);
                        return StatusCode::NO_CONTENT.into_response();
                    },
                    Err(err) => {
                        tracing::warn!("Could not move folder to trash, falling back to permanent delete: {}", err);
                        // Fall through to regular delete
                    }
                }
            }
            
            // Fallback to permanent delete
            let folder_service = &state.applications.folder_service;
            match folder_service.delete_folder(&id).await {
                Ok(_) => StatusCode::NO_CONTENT.into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }));
        
    // Merge the routers
    let folders_router = folders_basic_router.merge(folders_ops_router).merge(folder_zip_router);
        
    // Create file routes for basic operations and trash-enabled delete
    let basic_file_router = Router::new()
        .route("/", get(|
            State(service): State<Arc<FileService>>,
            axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
        | async move {
            // Get folder_id from query parameter if present
            let folder_id = params.get("folder_id").map(|id| id.as_str());
            tracing::info!("API: Listando archivos con folder_id: {:?}", folder_id);
            // Pass the service directly to the handler
            match service.list_files(folder_id).await {
                Ok(files) => {
                    tracing::info!("Found {} files", files.len());
                    (StatusCode::OK, Json(files)).into_response()
                },
                Err(err) => {
                    tracing::error!("Error listing files: {}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                        "error": format!("Error listing files: {}", err)
                    }))).into_response()
                }
            }
        }))
        .route("/upload", post(FileHandler::upload_file))
        .route("/{id}", get(FileHandler::download_file))
        .with_state(file_service.clone());
    
    // Let's create a router for file operations with trash support
    let file_operations_router = Router::new()
        // CRITICAL FIX: Ensure file deletion route correctly calls FileHandler::delete_file
        // Uses the correct URL pattern
        .route("/{id}", delete(|
            State(state): State<AppState>, 
            Path(id): Path<String>
        | async move {
            tracing::info!("File delete route called explicitly for ID: {}", id);
            FileHandler::delete_file(State(state), Path(id)).await
        }))
        .route("/{id}/move", put(|
            State(state): State<AppState>,
            Path(id): Path<String>,
            Json(payload): Json<serde_json::Value>,
        | async move {
            // Simplified move implementation just to get it working
            let folder_id = payload.get("folder_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
                
            let file_service = &state.applications.file_service;
            match file_service.move_file(&id, folder_id).await {
                Ok(file_dto) => (StatusCode::OK, Json(file_dto)).into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }));
    
    // Merge the routers
    let files_router = basic_file_router.merge(file_operations_router);
    
    // Crear rutas para operaciones por lotes
    let batch_router = Router::new()
        // Operaciones de archivos
        .route("/files/move", post(batch_handler::move_files_batch))
        .route("/files/copy", post(batch_handler::copy_files_batch))
        .route("/files/delete", post(batch_handler::delete_files_batch))
        .route("/files/get", post(batch_handler::get_files_batch))
        // Operaciones de carpetas
        .route("/folders/delete", post(batch_handler::delete_folders_batch))
        .route("/folders/create", post(batch_handler::create_folders_batch))
        .route("/folders/get", post(batch_handler::get_folders_batch))
        .with_state(batch_handler_state);
    
    // Create search routes if the service is available
    let search_router = if search_service.is_some() {
        use crate::interfaces::api::handlers::search_handler::SearchHandler;
        
        Router::new()
            // Simple search with query parameters
            .route("/", get(SearchHandler::search_files_get))
            // Advanced search with full criteria object
            .route("/advanced", post(SearchHandler::search_files_post))
            // Clear search cache
            .route("/cache", delete(SearchHandler::clear_search_cache))
            .with_state(app_state.clone())
    } else {
        Router::new()
    };
    
    // Implementaciones directas de handlers para compartir, sin depender de ShareHandler
    
    // Create routes for shared resources if the service is available
    let share_router = if let Some(share_service) = share_service.clone() {
        use crate::interfaces::api::handlers::share_handler;
        
        Router::new()
            .route("/", post(share_handler::create_shared_link))
            .route("/", get(share_handler::get_user_shares))
            .route("/{id}", get(share_handler::get_shared_link))
            .route("/{id}", put(share_handler::update_shared_link))
            .route("/{id}", delete(share_handler::delete_shared_link))
            .with_state(share_service.clone())
    } else {
        Router::new()
    };
    
    // Public route for accessing shared links
    let public_share_router = if let Some(share_service) = share_service.clone() {
        use crate::interfaces::api::handlers::share_handler;
        
        Router::new()
            .route("/{token}", get(share_handler::access_shared_item))
            .route("/{token}/verify", post(share_handler::verify_shared_item_password))
            .with_state(share_service.clone())
    } else {
        Router::new()
    };

    // Create a router without the i18n routes
    // Create routes for favorites if the service is available
    let favorites_router = if let Some(favorites_service) = favorites_service.clone() {
        use crate::interfaces::api::handlers::favorites_handler;
        
        Router::new()
            .route("/", get(favorites_handler::get_favorites))
            .route("/{item_type}/{item_id}", post(favorites_handler::add_favorite))
            .route("/{item_type}/{item_id}", delete(favorites_handler::remove_favorite))
            .with_state(favorites_service.clone())
    } else {
        Router::new()
    };
    
    // Create routes for recent items if the service is available
    let recent_router = if let Some(recent_service) = recent_service.clone() {
        use crate::interfaces::api::handlers::recent_handler;
        
        Router::new()
            .route("/", get(recent_handler::get_recent_items))
            .route("/{item_type}/{item_id}", post(recent_handler::record_item_access))
            .route("/{item_type}/{item_id}", delete(recent_handler::remove_from_recent))
            .route("/clear", delete(recent_handler::clear_recent_items))
            .with_state(recent_service.clone())
    } else {
        Router::new()
    };

    let mut router = Router::new()
        .nest("/folders", folders_router)
        .nest("/files", files_router)
        .nest("/batch", batch_router)
        .nest("/search", search_router)
        .nest("/shares", share_router)
        .nest("/s", public_share_router)
        .nest("/favorites", favorites_router)
        .nest("/recent", recent_router)
        ;
    
    // Store the share service in app_state for future use
    if let Some(share_service) = share_service.clone() {
        app_state.share_service = Some(share_service);
    }
        
    // Re-enable trash routes to make the trash view work
    if let Some(_trash_service_ref) = trash_service.clone() {
        tracing::info!("Setting up trash routes for trash view");
        
        // Create a router for trash specific endpoints that handles the auth requirements
        // Implement all trash operations needed by the frontend
        let trash_router = Router::new()
            // Get all trash items
            .route("/", get(|
                State(state): State<AppState>, 
                Query(params): Query<HashMap<String, String>>
            | async move {
                tracing::info!("Getting trash items");
                // Use a valid UUID for the default user or from query params
                let default_user = params.get("userId")
                    .unwrap_or(&"00000000-0000-0000-0000-000000000000".to_string())
                    .to_string();
                    
                tracing::info!("Using user ID: {}", default_user);
                // Get the trash service directly
                if let Some(trash_service) = &state.trash_service {
                    // Get trash items for default user
                    match trash_service.get_trash_items(&default_user).await {
                        Ok(items) => {
                            tracing::info!("Found {} items in trash", items.len());
                            let response_data = serde_json::json!(items);
                            tracing::info!("Response data: {:?}", response_data);
                            (StatusCode::OK, Json(response_data)).into_response()
                        },
                        Err(err) => {
                            tracing::error!("Error getting trash items: {}", err);
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                                "error": format!("Error getting trash items: {}", err)
                            }))).into_response()
                        }
                    }
                } else {
                    tracing::error!("Trash service not available");
                    (StatusCode::NOT_IMPLEMENTED, Json(json!({
                        "error": "Trash feature is not enabled"
                    }))).into_response()
                }
            }))
            // Move file to trash
            .route("/files/{id}", delete(|
                State(state): State<AppState>,
                Path(id): Path<String>
            | async move {
                tracing::info!("Moving file to trash: {}", id);
                let default_user = "00000000-0000-0000-0000-000000000000".to_string();
                
                if let Some(trash_service) = &state.trash_service {
                    match trash_service.move_to_trash(&id, "file", &default_user).await {
                        Ok(_) => {
                            tracing::info!("File moved to trash successfully");
                            (StatusCode::OK, Json(json!({
                                "success": true,
                                "message": "File moved to trash successfully"
                            }))).into_response()
                        },
                        Err(err) => {
                            tracing::error!("Error moving file to trash: {}", err);
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                                "error": format!("Error moving file to trash: {}", err)
                            }))).into_response()
                        }
                    }
                } else {
                    tracing::error!("Trash service not available");
                    (StatusCode::NOT_IMPLEMENTED, Json(json!({
                        "error": "Trash feature is not enabled"
                    }))).into_response()
                }
            }))
            // Move folder to trash
            .route("/folders/{id}", delete(|
                State(state): State<AppState>,
                Path(id): Path<String>
            | async move {
                tracing::info!("Moving folder to trash: {}", id);
                let default_user = "00000000-0000-0000-0000-000000000000".to_string();
                
                if let Some(trash_service) = &state.trash_service {
                    match trash_service.move_to_trash(&id, "folder", &default_user).await {
                        Ok(_) => {
                            tracing::info!("Folder moved to trash successfully");
                            (StatusCode::OK, Json(json!({
                                "success": true,
                                "message": "Folder moved to trash successfully"
                            }))).into_response()
                        },
                        Err(err) => {
                            tracing::error!("Error moving folder to trash: {}", err);
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                                "error": format!("Error moving folder to trash: {}", err)
                            }))).into_response()
                        }
                    }
                } else {
                    tracing::error!("Trash service not available");
                    (StatusCode::NOT_IMPLEMENTED, Json(json!({
                        "error": "Trash feature is not enabled"
                    }))).into_response()
                }
            }))
            // Restore item from trash
            .route("/{id}/restore", post(|
                State(state): State<AppState>,
                Path(id): Path<String>
            | async move {
                tracing::info!("Restoring item from trash: {}", id);
                let default_user = "00000000-0000-0000-0000-000000000000".to_string();
                
                if let Some(trash_service) = &state.trash_service {
                    match trash_service.restore_item(&id, &default_user).await {
                        Ok(_) => {
                            tracing::info!("Item restored from trash successfully");
                            (StatusCode::OK, Json(json!({
                                "success": true,
                                "message": "Item restored from trash successfully"
                            }))).into_response()
                        },
                        Err(err) => {
                            let err_str = format!("{}", err);
                            // Check if the error is due to item not being found
                            if err_str.contains("not found") || err_str.contains("NotFound") {
                                tracing::warn!("Item not found in trash, but reporting success: {}", id);
                                // Return success even if the item is not found
                                return (StatusCode::OK, Json(json!({
                                    "success": true,
                                    "message": "Item restored (or was already removed from trash)"
                                }))).into_response();
                            }
                            
                            tracing::error!("Error restoring item from trash: {}", err);
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                                "error": format!("Error restoring item from trash: {}", err)
                            }))).into_response()
                        }
                    }
                } else {
                    tracing::error!("Trash service not available");
                    (StatusCode::NOT_IMPLEMENTED, Json(json!({
                        "error": "Trash feature is not enabled"
                    }))).into_response()
                }
            }))
            // Permanently delete an item from trash
            .route("/{id}", delete(|
                State(state): State<AppState>,
                Path(id): Path<String>
            | async move {
                tracing::info!("Permanently deleting item from trash: {}", id);
                let default_user = "00000000-0000-0000-0000-000000000000".to_string();
                
                if let Some(trash_service) = &state.trash_service {
                    match trash_service.delete_permanently(&id, &default_user).await {
                        Ok(_) => {
                            tracing::info!("Item permanently deleted successfully");
                            (StatusCode::OK, Json(json!({
                                "success": true,
                                "message": "Item permanently deleted"
                            }))).into_response()
                        },
                        Err(err) => {
                            let err_str = format!("{}", err);
                            // Check if the error is due to item not being found
                            if err_str.contains("not found") || err_str.contains("NotFound") {
                                tracing::warn!("Item not found in trash, but reporting success: {}", id);
                                // Return success even if the item is not found
                                return (StatusCode::OK, Json(json!({
                                    "success": true,
                                    "message": "Item deleted (or was already removed from trash)"
                                }))).into_response();
                            }
                            
                            tracing::error!("Error permanently deleting item: {}", err);
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                                "error": format!("Error permanently deleting item: {}", err)
                            }))).into_response()
                        }
                    }
                } else {
                    tracing::error!("Trash service not available");
                    (StatusCode::NOT_IMPLEMENTED, Json(json!({
                        "error": "Trash feature is not enabled"
                    }))).into_response()
                }
            }))
            // Empty trash
            .route("/empty", delete(|
                State(state): State<AppState>
            | async move {
                tracing::info!("Emptying trash");
                let default_user = "00000000-0000-0000-0000-000000000000".to_string();
                
                if let Some(trash_service) = &state.trash_service {
                    match trash_service.empty_trash(&default_user).await {
                        Ok(_) => {
                            tracing::info!("Trash emptied successfully");
                            (StatusCode::OK, Json(json!({
                                "success": true,
                                "message": "Trash emptied successfully"
                            }))).into_response()
                        },
                        Err(err) => {
                            tracing::error!("Error emptying trash: {}", err);
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                                "error": format!("Error emptying trash: {}", err)
                            }))).into_response()
                        }
                    }
                } else {
                    tracing::error!("Trash service not available");
                    (StatusCode::NOT_IMPLEMENTED, Json(json!({
                        "error": "Trash feature is not enabled"
                    }))).into_response()
                }
            }))
            .with_state(app_state.clone());
        
        router = router.nest("/trash", trash_router);
    } else {
        tracing::warn!("Trash service not available - trash view will not work");
    }
    
    // Add i18n routes if the service is provided
    if let Some(i18n_service) = i18n_service {
        let i18n_router = Router::new()
            .route("/locales", get(I18nHandler::get_locales))
            .route("/translate", get(I18nHandler::translate))
            .route("/locales/{locale_code}", get(|
                State(service): State<Arc<I18nApplicationService>>,
                axum::extract::Path(locale_code): axum::extract::Path<String>,
            | async move {
                I18nHandler::get_translations(State(service), locale_code).await
            }))
            .with_state(i18n_service);
        
        router = router.nest("/i18n", i18n_router);
    }
    
    // Get the app configuration
    let _config = AppConfig::from_env();
    
    // For now, just use the router as is - we'll properly implement the auth middleware later
    // when all implementation details are fixed
    let router = router;
    
    // Apply compression and tracing layers
    // Note: We've removed the direct trash endpoints due to handler type compatibility issues
    // These will need to be implemented directly in main.rs or by modifying the file/folder handlers
    if trash_service.is_some() {
        tracing::info!("Trash service is available - trash view is functional");
    }
    
    // Add WebDAV routes if needed
    let webdav_enabled = true; // In production, you'd read this from a config
    let router = if webdav_enabled {
        use crate::interfaces::api::handlers::webdav_handler;
        router.merge(webdav_handler::webdav_routes())
    } else {
        router
    };
    
    // Add CalDAV routes if needed
    let caldav_enabled = true; // In production, you'd read this from a config
    let router = if caldav_enabled {
        use crate::interfaces::api::handlers::caldav_handler;
        router.nest("/caldav", caldav_handler::caldav_routes())
    } else {
        router
    };
    
    // Add CardDAV routes if needed
    let carddav_enabled = true; // In production, you'd read this from a config
    let router = if carddav_enabled {
        // Note: We'll implement carddav_handler in the next phase
        router
    } else {
        router
    };

    router
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        // HTTP caching is disabled temporarily due to compatibility issues
        // .layer(HttpCacheLayer::new(http_cache.clone()).with_max_age(folders_ttl))
}