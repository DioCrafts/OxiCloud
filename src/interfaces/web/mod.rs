use crate::common::config::AppConfig;
use crate::common::di::AppState;
use axum::http::header::{CACHE_CONTROL, HeaderValue};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::Router;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::OnceLock;
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_header::SetResponseHeaderLayer;

const ASSETS_START_MARKER: &str = "<!-- OXICLOUD_ASSETS_START -->";
const ASSETS_END_MARKER: &str = "<!-- OXICLOUD_ASSETS_END -->";

#[derive(Debug, Clone, Deserialize)]
struct PageManifestEntry {
    js: Vec<String>,
    css: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct BuildManifest {
    pages: BTreeMap<String, PageManifestEntry>,
}

static BUILD_MANIFEST: OnceLock<Option<BuildManifest>> = OnceLock::new();

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

fn no_cache_html_response_owned(content: String) -> Response {
    (
        [(
            CACHE_CONTROL,
            HeaderValue::from_static("no-cache, max-age=0, must-revalidate"),
        )],
        Html(content),
    )
        .into_response()
}

fn build_manifest() -> Option<&'static BuildManifest> {
    BUILD_MANIFEST
        .get_or_init(|| {
            serde_json::from_str::<BuildManifest>(include_str!("../../../static/dist/manifest.json")).ok()
        })
        .as_ref()
}

fn render_index_with_dist_assets() -> String {
    let original = include_str!("../../../static/index.html");
    let Some(manifest) = build_manifest() else {
        return original.to_string();
    };

    let Some(page) = manifest.pages.get("index.html") else {
        return original.to_string();
    };

    let mut block = String::new();
    block.push('\n');
    block.push_str(ASSETS_START_MARKER);
    block.push('\n');
    for href in &page.css {
        block.push_str("<link rel=\"stylesheet\" href=\"");
        block.push_str(href);
        block.push_str("\">\n");
    }
    for src in &page.js {
        block.push_str("<script defer src=\"");
        block.push_str(src);
        block.push_str("\"></script>\n");
    }
    block.push_str(ASSETS_END_MARKER);
    block.push('\n');

    let with_managed_block = if let (Some(start), Some(end)) = (
        original.find(ASSETS_START_MARKER),
        original.find(ASSETS_END_MARKER),
    ) {
        let mut out = String::with_capacity(original.len() + 256);
        out.push_str(&original[..start]);
        out.push_str(&block);
        out.push_str(&original[end + ASSETS_END_MARKER.len()..]);
        out
    } else {
        original.to_string()
    };

    if with_managed_block.contains("/dist/") {
        return with_managed_block;
    }

    if let Some(head_idx) = with_managed_block.find("</head>") {
        let mut out = String::with_capacity(with_managed_block.len() + 256);
        out.push_str(&with_managed_block[..head_idx]);
        out.push_str(&block);
        out.push_str(&with_managed_block[head_idx..]);
        return out;
    }

    with_managed_block
}

/// Serve the index page
async fn serve_index_page() -> Response {
    no_cache_html_response_owned(render_index_with_dist_assets())
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
