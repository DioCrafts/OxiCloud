use axum::{
    routing::get,
    Router,
    response::Html,
    extract::Path,
    http::{Uri, StatusCode, Request},
    response::{IntoResponse, Response, Redirect},
    middleware::{self, Next},
};
use tower_http::services::ServeDir;
use std::path::PathBuf;
use std::sync::Arc;
use crate::common::di::AppState;
use crate::common::config::AppConfig;

/// Special middleware to handle any URL that starts with /login
async fn login_redirect_middleware(request: Request<axum::body::Body>, next: Next) -> Response {
    let path = request.uri().path();
    
    // If the path starts with /login but is not exactly /login
    if path.starts_with("/login") && path != "/login" {
        tracing::info!("Redirecting from {} to /login", path);
        // Simply redirect to /login
        return Redirect::to("/login").into_response();
    }
    
    // Otherwise continue with the request
    next.run(request).await
}

/// Creates web routes for serving static files
pub fn create_web_routes() -> Router<Arc<AppState>> {
    // Get config to access static path
    let config = AppConfig::from_env();
    let static_path = config.static_path.clone();
    
    // Create a router with specific routes and middleware
    Router::new()
        // Apply our login middleware to all routes
        .layer(middleware::from_fn(login_redirect_middleware))
        // Login page with dedicated handler function
        .route("/login", get(serve_login_page))
        // Public file page
        .route("/public-file", get(serve_public_file_page))
        // Simply use a fallback for static files
        .fallback_service(ServeDir::new(static_path))
}

/// Serve the login page
async fn serve_login_page() -> Html<&'static str> {
    // Use include_str! with a static lifetime to include the login page content
    Html(include_str!("../../../static/login.html"))
}

/// Serve the public file access page
async fn serve_public_file_page() -> Html<&'static str> {
    // Just serve the static HTML file, token will be extracted from query params
    Html(include_str!("../../../static/public-file.html"))
}