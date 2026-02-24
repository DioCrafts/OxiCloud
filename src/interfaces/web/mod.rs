use crate::common::config::AppConfig;
use crate::common::di::AppState;
use axum::http::header::{CACHE_CONTROL, HeaderValue};
use axum::{Router, response::Html, routing::get};
use std::sync::Arc;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

/// Creates web routes for serving static files
pub fn create_web_routes() -> Router<Arc<AppState>> {
    // Get config to access static path
    let config = AppConfig::from_env();
    let static_path = config.static_path.clone();

    // Static assets (JS, CSS, JSON, SVG, ICO) served via ServeDir
    // with brotli + gzip compression and aggressive caching (7 days).
    // HTML pages are served via explicit routes (include_str!) and
    // do NOT pass through these layers.
    let static_service = ServeDir::new(static_path);

    Router::new()
        // Add specific routes for clean URLs (without .html)
        .route("/login", get(serve_login_page))
        .route("/profile", get(serve_profile_page))
        .route("/admin", get(serve_admin_page))
        // Serve static files with compression + cache headers
        .fallback_service(static_service)
        .layer(CompressionLayer::new().br(true).gzip(true))
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=604800, stale-while-revalidate=86400"),
        ))
}

/// Serve the login page
async fn serve_login_page() -> Html<&'static str> {
    Html(include_str!("../../../static/login.html"))
}

/// Serve the profile page
async fn serve_profile_page() -> Html<&'static str> {
    Html(include_str!("../../../static/profile.html"))
}

/// Serve the admin page
async fn serve_admin_page() -> Html<&'static str> {
    Html(include_str!("../../../static/admin.html"))
}
