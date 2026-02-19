use crate::common::config::AppConfig;
use crate::common::di::AppState;
use axum::http::header::{CACHE_CONTROL, HeaderValue};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::Router;
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_header::SetResponseHeaderLayer;

/// Creates web routes for serving static files
pub fn create_web_routes() -> Router<AppState> {
    // Get config to access static path
    let config = AppConfig::from_env();
    let static_path = config.static_path.clone();

    // Static services split by resource type to apply tailored cache policies.
    let dist_service = ServeDir::new(static_path.join("dist"));
    let locales_service = ServeDir::new(static_path.join("locales"));
    let static_service = ServeDir::new(static_path.clone());

    let html_routes = Router::new()
        .route("/", get(serve_index_page))
        .route("/index.html", get(serve_index_page))
        .route("/login", get(serve_login_page))
        .route("/login.html", get(serve_login_page))
        .route("/profile", get(serve_profile_page))
        .route("/profile.html", get(serve_profile_page))
        .route("/admin", get(serve_admin_page))
        .route("/admin.html", get(serve_admin_page))
        .route("/shared", get(serve_shared_page))
        .route("/shared.html", get(serve_shared_page));

    let sw_routes = Router::new().route_service(
        "/sw.js",
        get_service(ServeFile::new(static_path.join("sw.js"))).layer(
            SetResponseHeaderLayer::if_not_present(
                CACHE_CONTROL,
                HeaderValue::from_static("no-cache, max-age=0, must-revalidate"),
            ),
        ),
    );

    let dist_routes = Router::new()
        .nest_service("/dist", dist_service)
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000, immutable"),
        ));

    let locales_routes = Router::new()
        .nest_service("/locales", locales_service)
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=3600, stale-while-revalidate=86400"),
        ));

    let fallback_routes = Router::new()
        .fallback_service(static_service)
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=3600, stale-while-revalidate=86400"),
        ));

    html_routes
        .merge(sw_routes)
        .merge(dist_routes)
        .merge(locales_routes)
        .merge(fallback_routes)
        .layer(CompressionLayer::new().br(true).gzip(true))
}

fn no_cache_html_response(content: &'static str) -> Response {
    (
        [(
            CACHE_CONTROL,
            HeaderValue::from_static("no-cache, max-age=0, must-revalidate"),
        )],
        Html(content),
    )
        .into_response()
}

/// Serve the index page
async fn serve_index_page() -> Response {
    no_cache_html_response(include_str!("../../../static/index.html"))
}

/// Serve the login page
async fn serve_login_page() -> Response {
    no_cache_html_response(include_str!("../../../static/login.html"))
}

/// Serve the profile page
async fn serve_profile_page() -> Response {
    no_cache_html_response(include_str!("../../../static/profile.html"))
}

/// Serve the admin page
async fn serve_admin_page() -> Response {
    no_cache_html_response(include_str!("../../../static/admin.html"))
}

/// Serve the shared page
async fn serve_shared_page() -> Response {
    no_cache_html_response(include_str!("../../../static/shared.html"))
}
