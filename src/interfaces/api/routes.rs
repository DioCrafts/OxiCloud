use std::sync::Arc;
use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::{State, Query, Path},
    middleware,
    Extension,
};
use tower_http::{
    compression::CompressionLayer, 
    trace::TraceLayer,
};
use crate::interfaces::middleware::auth::CurrentUser;
use crate::common::config::AppConfig;
use crate::interfaces::middleware::auth::auth_middleware;

use crate::interfaces::middleware::cache::{HttpCache, start_cache_cleanup_task};

use crate::application::services::folder_service::FolderService;
use crate::application::services::file_service::FileService;
use crate::application::services::i18n_application_service::I18nApplicationService;
use crate::application::services::batch_operations::BatchOperationService;
use crate::application::services::sharing_service::SharingService;
use crate::application::services::public_link_service::PublicLinkService;

use crate::interfaces::api::handlers::folder_handler::FolderHandler;
use crate::interfaces::api::handlers::file_handler::FileHandler;
use crate::interfaces::api::handlers::i18n_handler::I18nHandler;
use crate::interfaces::api::handlers::batch_handler::{
    self, BatchHandlerState
};
use crate::interfaces::api::handlers::shared_file_handler::SharedFileHandler;
use crate::interfaces::api::handlers::public_link_handler::PublicLinkHandler;
use crate::application::dtos::pagination::PaginationRequestDto;

/// Creates API routes for the application
pub fn create_api_routes(
    folder_service: Arc<FolderService>, 
    file_service: Arc<FileService>,
    i18n_service: Option<Arc<I18nApplicationService>>,
    sharing_service: Option<Arc<SharingService>>,
    public_link_service: Option<Arc<PublicLinkService>>,
) -> Router<Arc<crate::common::di::AppState>> {
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
    
    let folders_router = Router::new()
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
        .route("/{id}", delete(FolderHandler::delete_folder))
        .with_state(folder_service);
        
    let files_router = Router::new()
        .route("/", get(|
            State(service): State<Arc<FileService>>,
            Extension(current_user): Extension<CurrentUser>,
            axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
        | async move {
            // Get folder_id from query parameter if present
            let folder_id = params.get("folder_id").map(|id| id.as_str());
            tracing::info!("API: Listando archivos con folder_id: {:?}", folder_id);
            FileHandler::list_files(State(service), Extension(current_user), folder_id).await
        }))
        .route("/upload", post(FileHandler::upload_file))
        .route("/{id}", get(FileHandler::download_file))
        .route("/{id}", delete(FileHandler::delete_file))
        .route("/{id}/move", put(FileHandler::move_file))
        .with_state(file_service);
    
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
    
    // Create a router without the i18n routes
    let mut router = Router::new()
        .nest("/folders", folders_router)
        .nest("/files", files_router)
        .nest("/batch", batch_router);
    
    // Add sharing routes if the service is provided
    if let Some(sharing_service) = sharing_service {
        let sharing_router = Router::new()
            // User-to-user sharing endpoints
            .route("/", post(SharedFileHandler::share_file))
            .route("/shared-with-me", get(SharedFileHandler::get_files_shared_with_me))
            .route("/shared-by-me", get(SharedFileHandler::get_files_shared_by_me))
            .route("/{file_id}/users", get(SharedFileHandler::get_users_with_access))
            .route("/{file_id}/permission", put(SharedFileHandler::update_permission))
            .route("/{file_id}/user/{user_id}", delete(SharedFileHandler::unshare_file))
            .with_state(sharing_service);
        
        router = router.nest("/sharing", sharing_router);
    }
    
    // Add public link routes if the service is provided
    if let Some(public_link_service) = public_link_service.clone() {
        let public_link_auth_router = Router::new()
            // Endpoints requiring authentication
            .route("/", post(PublicLinkHandler::create_public_link))
            .route("/my-links", get(PublicLinkHandler::get_links_by_user))
            .route("/{link_id}", get(PublicLinkHandler::get_public_link))
            .route("/file/{file_id}", get(PublicLinkHandler::get_links_for_file))
            .route("/{link_id}/permission", put(PublicLinkHandler::update_link_permission))
            .route("/{link_id}/password", put(PublicLinkHandler::update_link_password))
            .route("/{link_id}/expiration", put(PublicLinkHandler::update_link_expiration))
            .route("/{link_id}", delete(PublicLinkHandler::delete_public_link))
            .with_state(public_link_service.clone());
        
        router = router.nest("/public-links", public_link_auth_router);
    }
    
    // Add public access endpoint (no auth required)
    if let Some(public_link_service) = public_link_service {
        let public_access_router = Router::new()
            .route("/{link_id}", post(PublicLinkHandler::access_public_file))
            .with_state(public_link_service);
        
        // Note: This route deliberately doesn't have auth middleware
        router = router.nest("/public", public_access_router);
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
    let config = AppConfig::from_env();
    
    // For now, just use the router as is - we'll properly implement the auth middleware later
    // when all implementation details are fixed
    let router = router;
    
    // Apply compression and tracing layers
    router
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        // HTTP caching is disabled temporarily due to compatibility issues
        // .layer(HttpCacheLayer::new(http_cache.clone()).with_max_age(folders_ttl))
}