use crate::common::di::AppState;
use axum::{
    Router,
    extract::DefaultBodyLimit,
    response::Json as AxumJson,
    routing::{delete, get, post, put},
};
use serde_json::json;
use std::sync::Arc;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

/// Returns the application version from Cargo.toml (compile-time constant)
async fn get_version() -> AxumJson<serde_json::Value> {
    AxumJson(json!({
        "name": "OxiCloud",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

use crate::interfaces::middleware::cache::{HttpCache, start_cache_cleanup_task};

use crate::application::services::batch_operations::BatchOperationService;

use crate::interfaces::api::handlers::admin_handler;
use crate::interfaces::api::handlers::batch_handler::{self, BatchHandlerState};
use crate::interfaces::api::handlers::chunked_upload_handler::ChunkedUploadHandler;
use crate::interfaces::api::handlers::file_handler::FileHandler;
use crate::interfaces::api::handlers::folder_handler::FolderHandler;
use crate::interfaces::api::handlers::i18n_handler::I18nHandler;
use crate::interfaces::api::handlers::trash_handler;

/// Creates public API routes that should NOT require authentication.
///
/// Currently this includes:
/// - `/s/{token}` — public access to shared items via share link
/// - `/s/{token}/verify` — password verification for protected share links
/// - `/i18n/*` — internationalization/translation endpoints
pub fn create_public_api_routes(app_state: &AppState) -> Router<AppState> {
    let share_service = app_state.share_service.clone();
    let i18n_service = Some(app_state.applications.i18n_service.clone());

    let mut router = Router::new();

    // Public share access routes — no auth required
    if let Some(share_service) = share_service {
        use crate::interfaces::api::handlers::share_handler;

        let public_share_router = Router::new()
            .route("/{token}", get(share_handler::access_shared_item))
            .route(
                "/{token}/verify",
                post(share_handler::verify_shared_item_password),
            )
            .with_state(share_service);

        router = router.nest("/s", public_share_router);
    }

    // i18n routes — no auth required (localization should be available before login)
    if let Some(i18n_service) = i18n_service {
        let i18n_router = Router::new()
            .route("/locales", get(I18nHandler::get_locales))
            .route("/translate", get(I18nHandler::translate))
            .route(
                "/locales/{locale_code}",
                get(I18nHandler::get_translations_by_locale),
            )
            .with_state(i18n_service);

        router = router.nest("/i18n", i18n_router);
    }

    // Version endpoint — public, no auth required
    router = router.route("/version", get(get_version));

    router
}

/// Creates protected API routes for the application.
///
/// These routes require authentication when auth is enabled.
/// Receives the fully-assembled `AppState` and extracts all needed services
/// from it, avoiding a long parameter list.
pub fn create_api_routes(app_state: &AppState) -> Router<AppState> {
    // Extract services from the pre-built AppState
    let folder_service = app_state.applications.folder_service_concrete.clone();
    let file_retrieval_service = app_state.applications.file_retrieval_service.clone();
    let file_management_service = app_state.applications.file_management_service.clone();
    let trash_service = app_state.trash_service.clone();
    let search_service = app_state.applications.search_service.clone();
    let share_service = app_state.share_service.clone();
    let favorites_service = app_state.favorites_service.clone();
    let recent_service = app_state.recent_service.clone();

    // Initialize the batch operations service
    let batch_service = Arc::new(BatchOperationService::default(
        file_retrieval_service.clone(),
        file_management_service.clone(),
        folder_service.clone(),
    ));

    // Create state for the batch operations handler
    let batch_handler_state = BatchHandlerState {
        batch_service: batch_service.clone(),
    };

    // Implement HTTP Cache
    let http_cache = HttpCache::new();

    // Define TTL values for different resource types (in seconds)
    let _folders_ttl = 300; // 5 minutes
    let _files_list_ttl = 300; // 5 minutes
    let _i18n_ttl = 3600; // 1 hour

    // Start the cleanup task for HTTP cache
    start_cache_cleanup_task(http_cache.clone());

    // Create the basic folders router with service operations
    let folders_basic_router = Router::new()
        .route("/", post(FolderHandler::create_folder))
        .route("/", get(FolderHandler::list_root_folders))
        .route(
            "/paginated",
            get(FolderHandler::list_root_folders_paginated),
        )
        .route("/{id}", get(FolderHandler::get_folder))
        .route("/{id}/contents", get(FolderHandler::list_folder_contents))
        .route(
            "/{id}/contents/paginated",
            get(FolderHandler::list_folder_contents_paginated),
        )
        .route("/{id}/rename", put(FolderHandler::rename_folder))
        .route("/{id}/move", put(FolderHandler::move_folder))
        .with_state(folder_service.clone());

    // Special route for ZIP download that requires AppState instead of just FolderService
    let folder_zip_router = Router::new()
        .route("/{id}/download", get(FolderHandler::download_folder_zip))
        .with_state(app_state.clone());

    // Create folder operations that use trash (requires full AppState)
    let folders_ops_router =
        Router::new().route("/{id}", delete(FolderHandler::delete_folder_with_trash));

    // Merge the routers
    let folders_router = folders_basic_router
        .merge(folders_ops_router)
        .merge(folder_zip_router);

    // Create file routes for basic operations and trash-enabled delete
    let basic_file_router = Router::new()
        .route("/", get(FileHandler::list_files_query))
        .route("/upload", post(FileHandler::upload_file_with_thumbnails))
        .route("/{id}", get(FileHandler::download_file))
        .route("/{id}/thumbnail/{size}", get(FileHandler::get_thumbnail))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024 * 1024)) // 10 GB for file uploads
        .with_state(app_state.clone());

    // File operations with trash support
    let file_operations_router = Router::new()
        .route("/{id}", delete(FileHandler::delete_file))
        .route("/{id}/move", put(FileHandler::move_file_simple))
        .route("/{id}/rename", put(FileHandler::rename_file));

    // Merge the routers
    let files_router = basic_file_router.merge(file_operations_router);

    // Create routes for batch operations
    let batch_router = Router::new()
        // File operations
        .route("/files/move", post(batch_handler::move_files_batch))
        .route("/files/copy", post(batch_handler::copy_files_batch))
        .route("/files/delete", post(batch_handler::delete_files_batch))
        .route("/files/get", post(batch_handler::get_files_batch))
        // Folder operations
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

    // Direct handler implementations for sharing, without depending on ShareHandler

    // Create routes for shared resources management (requires auth)
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

    // Create a router without the i18n routes
    // Create routes for favorites if the service is available
    let favorites_router = if let Some(favorites_service) = favorites_service.clone() {
        use crate::interfaces::api::handlers::favorites_handler;

        Router::new()
            .route("/", get(favorites_handler::get_favorites))
            .route(
                "/{item_type}/{item_id}",
                post(favorites_handler::add_favorite),
            )
            .route(
                "/{item_type}/{item_id}",
                delete(favorites_handler::remove_favorite),
            )
            .with_state(favorites_service.clone())
    } else {
        Router::new()
    };

    // Create routes for recent items if the service is available
    let recent_router = if let Some(recent_service) = recent_service.clone() {
        use crate::interfaces::api::handlers::recent_handler;

        Router::new()
            .route("/", get(recent_handler::get_recent_items))
            .route(
                "/{item_type}/{item_id}",
                post(recent_handler::record_item_access),
            )
            .route(
                "/{item_type}/{item_id}",
                delete(recent_handler::remove_from_recent),
            )
            .route("/clear", delete(recent_handler::clear_recent_items))
            .with_state(recent_service.clone())
    } else {
        Router::new()
    };

    // Create routes for chunked uploads (large files >10MB)
    let chunked_upload_router = Router::new()
        .route("/", post(ChunkedUploadHandler::create_upload))
        .route(
            "/{upload_id}",
            axum::routing::patch(ChunkedUploadHandler::upload_chunk),
        )
        .route(
            "/{upload_id}",
            axum::routing::head(ChunkedUploadHandler::get_upload_status),
        )
        .route(
            "/{upload_id}/complete",
            post(ChunkedUploadHandler::complete_upload),
        )
        .route("/{upload_id}", delete(ChunkedUploadHandler::cancel_upload))
        .with_state(Arc::new(app_state.clone()));

    // Create routes for deduplication endpoints
    let dedup_router = Router::new()
        .route(
            "/check/{hash}",
            get(super::handlers::dedup_handler::DedupHandler::check_hash),
        )
        .route(
            "/upload",
            post(super::handlers::dedup_handler::DedupHandler::upload_with_dedup),
        )
        .route(
            "/stats",
            get(super::handlers::dedup_handler::DedupHandler::get_stats),
        )
        .route(
            "/blob/{hash}",
            get(super::handlers::dedup_handler::DedupHandler::get_blob),
        )
        .route(
            "/blob/{hash}",
            delete(super::handlers::dedup_handler::DedupHandler::remove_reference),
        )
        .route(
            "/recalculate",
            post(super::handlers::dedup_handler::DedupHandler::recalculate_stats),
        )
        .with_state(app_state.clone());

    let mut router = Router::new()
        .nest("/folders", folders_router)
        .nest("/files", files_router)
        .nest("/uploads", chunked_upload_router)
        .nest("/dedup", dedup_router)
        .nest("/batch", batch_router)
        .nest("/search", search_router)
        .nest("/shares", share_router)
        .nest("/favorites", favorites_router)
        .nest("/recent", recent_router);

    // Re-enable trash routes to make the trash view work
    if let Some(_trash_service_ref) = trash_service.clone() {
        tracing::info!("Setting up trash routes for trash view");

        let trash_router = Router::new()
            .route("/", get(trash_handler::get_trash_items))
            .route("/files/{id}", delete(trash_handler::move_file_to_trash))
            .route("/folders/{id}", delete(trash_handler::move_folder_to_trash))
            .route("/{id}/restore", post(trash_handler::restore_from_trash))
            .route("/{id}", delete(trash_handler::delete_permanently))
            .route("/empty", delete(trash_handler::empty_trash))
            .with_state(app_state.clone());

        router = router.nest("/trash", trash_router);
    } else {
        tracing::warn!("Trash service not available - trash view will not work");
    }

    // NOTE: WebDAV routes are mounted at top-level (/webdav) in main.rs
    // for client compatibility, NOT under /api.

    // NOTE: CalDAV and CardDAV routes are mounted at top-level (/caldav, /carddav)
    // in main.rs for protocol compliance, NOT under /api.

    // Admin settings routes (protected by admin_guard inside the handler)
    let admin_router = admin_handler::admin_routes().with_state(app_state.clone());
    router = router.nest("/admin", admin_router);

    router
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
}
