use crate::common::config::AppConfig;
use crate::common::di::AppState;
use axum::{Router, response::Html, routing::get};
use tower_http::services::ServeDir;

/// Creates web routes for serving static files
pub fn create_web_routes() -> Router<AppState> {
    // Get config to access static path
    let config = AppConfig::from_env();
    let static_path = config.static_path.clone();

    Router::new()
        // Add specific routes for clean URLs (without .html)
        .route("/login", get(serve_login_page))
        .route("/profile", get(serve_profile_page))
        .route("/admin", get(serve_admin_page))
        .route("/shared", get(serve_shared_page))
        // Serve static files
        .fallback_service(ServeDir::new(static_path))
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

/// Serve the shared page
async fn serve_shared_page() -> Html<&'static str> {
    Html(include_str!("../../../static/shared.html"))
}
