use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// OxiCloud - Cloud Storage Platform
///
/// OxiCloud is a NextCloud-like file storage system built in Rust with a focus on
/// performance, security, and clean architecture. The system provides:
///
/// - File and folder management with rich metadata
/// - User authentication and authorization
/// - File trash system with automatic cleanup
/// - Efficient handling of large files through parallel processing
/// - Compression capabilities for bandwidth optimization
/// - RESTful API and web interface
///
/// The architecture follows the Clean/Hexagonal Architecture pattern with:
///
/// - Domain Layer: Core business entities and repository interfaces (domain/*)
/// - Application Layer: Use cases and service orchestration (application/*)
/// - Infrastructure Layer: Technical implementations of repositories (infrastructure/*)
/// - Interface Layer: API endpoints and web controllers (interfaces/*)
///
/// Dependencies are managed through dependency inversion, with high-level modules
/// defining interfaces (ports) that low-level modules implement (adapters).
///
/// @author OxiCloud Development Team
use oxicloud::common;
use oxicloud::infrastructure;
use oxicloud::interfaces;

use common::di::AppServiceFactory;
use infrastructure::db::create_database_pool;
use interfaces::{create_api_routes, create_public_api_routes, web::create_web_routes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if present (for local development)
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment variables
    let config = common::config::AppConfig::from_env();

    // Ensure storage and locales directories exist
    let storage_path = config.storage_path.clone();
    if !storage_path.exists() {
        std::fs::create_dir_all(&storage_path).expect("Failed to create storage directory");
    }
    let locales_path = PathBuf::from("./static/locales");
    if !locales_path.exists() {
        std::fs::create_dir_all(&locales_path).expect("Failed to create locales directory");
    }

    // Initialize database pool if auth is enabled
    let db_pool = if config.features.enable_auth {
        match create_database_pool(&config).await {
            Ok(pool) => {
                tracing::info!("PostgreSQL database pool initialized successfully");
                Some(Arc::new(pool))
            }
            Err(e) => {
                // SECURITY: fail-closed. If auth is required but the database
                // is unreachable, the server MUST NOT start in public mode.
                panic!(
                    "FATAL: enable_auth=true but database connection failed: {}. \
                     Refusing to start without authentication.",
                    e
                );
            }
        }
    } else {
        None
    };

    // Build all services via the factory
    let factory = AppServiceFactory::with_config(storage_path, locales_path, config.clone());

    let app_state = factory.build_app_state(db_pool).await
        .expect("Failed to build application state. If running in Docker, ensure the storage volume is writable by the oxicloud user (UID 1001)");

    // Build application router
    let api_routes = create_api_routes(&app_state);
    let public_api_routes = create_public_api_routes(&app_state);
    let web_routes = create_web_routes();

    let mut app;

    // Build CalDAV / CardDAV / WebDAV protocol routers (merged at top-level, not under /api)
    use oxicloud::interfaces::api::handlers::caldav_handler;
    use oxicloud::interfaces::api::handlers::carddav_handler;
    use oxicloud::interfaces::api::handlers::webdav_handler;
    let caldav_router = caldav_handler::caldav_routes();
    let carddav_router = carddav_handler::carddav_routes();
    let webdav_router = webdav_handler::webdav_routes();

    // Build WOPI routes if enabled
    use oxicloud::interfaces::api::handlers::wopi_handler;
    let wopi_routes = if config.wopi.enabled {
        if let (Some(token_svc), Some(lock_svc), Some(discovery_svc)) = (
            &app_state.wopi_token_service,
            &app_state.wopi_lock_service,
            &app_state.wopi_discovery_service,
        ) {
            let wopi_base_url = std::env::var("OXICLOUD_WOPI_BASE_URL")
                .map(|v| v.trim_end_matches('/').to_string())
                .ok()
                .filter(|v| !v.is_empty())
                .unwrap_or_else(|| config.base_url());

            let wopi_state = wopi_handler::WopiState {
                token_service: token_svc.clone(),
                lock_service: lock_svc.clone(),
                discovery_service: discovery_svc.clone(),
                app_state: app_state.clone(),
                public_base_url: config.base_url(),
                wopi_base_url,
            };

            let (protocol, api) = wopi_handler::wopi_routes(wopi_state);
            Some((protocol, api))
        } else {
            None
        }
    } else {
        None
    };

    // Apply auth middleware to protected API routes when auth is enabled
    if config.features.enable_auth {
        // SECURITY: if auth is required, auth_service MUST be present at this
        // point.  The earlier guards in di.rs and main.rs guarantee this, but
        // add a defensive check so a future refactor cannot silently degrade.
        assert!(
            app_state.auth_service.is_some(),
            "FATAL: enable_auth=true but auth_service is None. \
             This should have been caught during initialization."
        );
    }
    if config.features.enable_auth {
        use interfaces::api::handlers::auth_handler::auth_routes;
        use oxicloud::interfaces::middleware::auth::auth_middleware;

        let app_state_arc = Arc::new(app_state.clone());
        let auth_router = auth_routes().with_state(app_state_arc.clone());

        // Protected API routes — require valid JWT token
        let protected_api = api_routes.layer(axum::middleware::from_fn_with_state(
            app_state_arc.clone(),
            auth_middleware,
        ));

        // CalDAV/CardDAV/WebDAV with auth middleware (merged, not nested)
        let caldav_protected = caldav_router.layer(axum::middleware::from_fn_with_state(
            app_state_arc.clone(),
            auth_middleware,
        ));
        let carddav_protected = carddav_router.layer(axum::middleware::from_fn_with_state(
            app_state_arc.clone(),
            auth_middleware,
        ));
        let webdav_protected = webdav_router.layer(axum::middleware::from_fn_with_state(
            app_state_arc,
            auth_middleware,
        ));

        app = Router::new()
            // Auth endpoints (login, register, refresh) are public — no middleware
            .nest("/api/auth", auth_router)
            // Public API routes (share access, i18n) — no auth required
            .nest("/api", public_api_routes)
            // All other API routes are protected by auth middleware
            .nest("/api", protected_api)
            // CalDAV/CardDAV/WebDAV protocols merged at top-level for client compatibility
            .merge(caldav_protected)
            .merge(carddav_protected)
            .merge(webdav_protected)
            .merge(web_routes)
            .layer(TraceLayer::new_for_http());

        // Mount WOPI routes (protocol routes use own token auth, API routes behind auth middleware)
        if let Some((wopi_protocol, wopi_api)) = wopi_routes {
            let wopi_api_protected = wopi_api.layer(axum::middleware::from_fn_with_state(
                Arc::new(app_state.clone()),
                auth_middleware,
            ));
            app = app
                .nest("/wopi", wopi_protocol)
                .nest("/api/wopi", wopi_api_protected);
        }
    } else {
        // Auth disabled — no middleware applied
        tracing::warn!("Authentication is DISABLED — all API routes are publicly accessible");
        app = Router::new()
            .nest("/api", public_api_routes)
            .nest("/api", api_routes)
            // CalDAV/CardDAV/WebDAV protocols merged at top-level
            .merge(caldav_router)
            .merge(carddav_router)
            .merge(webdav_router)
            .merge(web_routes)
            .layer(TraceLayer::new_for_http());

        // Mount WOPI routes (no auth middleware when auth is disabled)
        if let Some((wopi_protocol, wopi_api)) = wopi_routes {
            app = app.nest("/wopi", wopi_protocol).nest("/api/wopi", wopi_api);
        }
    }

    // Apply the redirect middleware for legacy routes
    use oxicloud::interfaces::middleware::redirect::redirect_middleware;
    app = app.layer(axum::middleware::from_fn(redirect_middleware));

    // Increase the default body limit to 10 GB to allow large file uploads.
    // Without this Axum caps Multipart bodies at 2 MB.
    app = app.layer(DefaultBodyLimit::max(10 * 1024 * 1024 * 1024));

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8086));
    tracing::info!("Starting OxiCloud server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Provide the fully-built state to the router
    let app = app.with_state(app_state);

    axum::serve(listener, app).await?;
    tracing::info!("Server shutdown completed");

    Ok(())
}
