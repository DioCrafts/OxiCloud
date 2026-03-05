use axum::{
    Router,
    body::Body,
    extract::{Path, State},
    http::{Request, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    routing::{any, delete, get, post},
};
use std::sync::Arc;

use crate::common::di::AppState;
use crate::interfaces::middleware::auth::CurrentUser;
use crate::interfaces::middleware::rate_limit::{RateLimiter, rate_limit_login};
use crate::interfaces::nextcloud::avatar_handler;
use crate::interfaces::nextcloud::basic_auth_middleware::basic_auth_middleware;
use crate::interfaces::nextcloud::login_v2_handler;
use crate::interfaces::nextcloud::ocs_handler;
use crate::interfaces::nextcloud::preview_handler;
use crate::interfaces::nextcloud::status_handler;
use crate::interfaces::nextcloud::trashbin_handler;
use crate::interfaces::nextcloud::uploads_handler;
use crate::interfaces::nextcloud::webdav_handler;

/// Build Nextcloud routes with a pre-built `Arc<AppState>` for the middleware layer.
///
/// This is the preferred entry point — pass the real state so the Basic Auth
/// middleware can look up app passwords from the database.
pub fn nextcloud_routes_with_state(state: Arc<AppState>) -> Router<Arc<AppState>> {
    // Rate limiter for NC login submit (reuses auth config values)
    let nc_login_limiter = {
        let rl = &state.core.config.auth.rate_limit;
        Arc::new(RateLimiter::new(
            rl.login_max_requests,
            rl.login_window_secs,
            100_000,
        ))
    };

    // Public routes — no auth required.
    let public = Router::new()
        .route("/status.php", get(status_handler::handle_status))
        .route(
            "/index.php/login/v2",
            post(login_v2_handler::handle_login_initiate),
        )
        .route(
            "/login/v2/flow/{token}",
            get(login_v2_handler::handle_login_page)
                .post(login_v2_handler::handle_login_submit)
                .layer(axum::middleware::from_fn_with_state(
                    nc_login_limiter,
                    rate_limit_login,
                )),
        )
        // OIDC initiation from Nextcloud login page
        .route(
            "/login/v2/flow/{token}/oidc",
            get(login_v2_handler::handle_login_oidc),
        )
        .route(
            "/index.php/login/v2/poll",
            post(login_v2_handler::handle_login_poll),
        )
        .route("/login/v2/poll", post(login_v2_handler::handle_login_poll))
        // Capabilities are public — iOS app fetches them before having credentials.
        .route(
            "/ocs/v1.php/cloud/capabilities",
            get(ocs_handler::handle_capabilities_v1),
        )
        .route(
            "/ocs/v2.php/cloud/capabilities",
            get(ocs_handler::handle_capabilities_v2),
        );

    // Protected routes — require Basic Auth via app passwords.
    let protected = Router::new()
        .route("/ocs/v2.php/cloud/user", get(ocs_handler::handle_user_info))
        .route(
            "/ocs/v1.php/cloud/users/{userid}",
            get(ocs_handler::handle_user_provisioning_v1),
        )
        .route(
            "/ocs/v2.php/cloud/users/{userid}",
            get(ocs_handler::handle_user_provisioning_v2),
        )
        .route(
            "/ocs/v2.php/core/apppassword",
            delete(ocs_handler::handle_revoke_apppassword),
        )
        .route(
            "/ocs/v2.php/apps/notifications/api/v2/notifications",
            get(ocs_handler::handle_notifications_list),
        )
        .route(
            "/ocs/v2.php/apps/notifications/api/v2/push",
            post(ocs_handler::handle_notifications_push),
        )
        .route(
            "/ocs/v2.php/apps/files_sharing/api/v1/sharees",
            get(ocs_handler::handle_sharees_search),
        )
        // Unified Search
        .route(
            "/ocs/v2.php/search/providers",
            get(ocs_handler::handle_search_providers),
        )
        .route(
            "/ocs/v2.php/search/providers/{provider_id}/search",
            get(ocs_handler::handle_search),
        )
        .route(
            "/index.php/core/preview",
            get(preview_handler::handle_preview),
        )
        .route(
            "/index.php/avatar/{user}/{size}",
            get(avatar_handler::handle_avatar),
        )
        .route(
            "/remote.php/dav/files/{user}/{*subpath}",
            any(handle_dav_files),
        )
        .route("/remote.php/dav/files/{user}/", any(handle_dav_files_root))
        .route("/remote.php/dav/files/{user}", any(handle_dav_files_root))
        .route(
            "/remote.php/dav/uploads/{user}/{upload_id}/{*rest}",
            any(handle_dav_uploads),
        )
        .route(
            "/remote.php/dav/uploads/{user}/{upload_id}",
            any(handle_dav_uploads_root),
        )
        // Trashbin WebDAV
        .route(
            "/remote.php/dav/trashbin/{user}/{*subpath}",
            any(handle_dav_trashbin),
        )
        .route(
            "/remote.php/dav/trashbin/{user}/",
            any(handle_dav_trashbin_root),
        )
        .route(
            "/remote.php/dav/trashbin/{user}",
            any(handle_dav_trashbin_root),
        )
        .route("/remote.php/webdav/{*subpath}", any(handle_legacy_webdav))
        .route("/remote.php/webdav/", any(handle_legacy_webdav_root))
        .route("/remote.php/webdav", any(handle_legacy_webdav_root))
        .layer(middleware::from_fn_with_state(state, basic_auth_middleware));

    Router::new().merge(public).merge(protected)
}

// ──────────────── Handler glue ────────────────

/// Reject requests where the URL `{user}` doesn't match the authenticated user.
#[allow(clippy::result_large_err)]
fn verify_url_user(url_user: &str, auth_user: &CurrentUser) -> Result<(), Response> {
    if url_user != auth_user.username {
        Err(StatusCode::FORBIDDEN.into_response())
    } else {
        Ok(())
    }
}

async fn handle_dav_files(
    State(state): State<Arc<AppState>>,
    Path((url_user, subpath)): Path<(String, String)>,
    user_ext: CurrentUser,
    req: Request<Body>,
) -> Result<Response, Response> {
    verify_url_user(&url_user, &user_ext)?;
    webdav_handler::handle_nc_webdav(state, req, user_ext, subpath)
        .await
        .map_err(|e| e.into_response())
}

async fn handle_dav_files_root(
    State(state): State<Arc<AppState>>,
    Path(url_user): Path<String>,
    user_ext: CurrentUser,
    req: Request<Body>,
) -> Result<Response, Response> {
    verify_url_user(&url_user, &user_ext)?;
    webdav_handler::handle_nc_webdav(state, req, user_ext, String::new())
        .await
        .map_err(|e| e.into_response())
}

async fn handle_dav_uploads(
    State(state): State<Arc<AppState>>,
    Path((url_user, upload_id, rest)): Path<(String, String, String)>,
    user_ext: CurrentUser,
    req: Request<Body>,
) -> Result<Response, Response> {
    verify_url_user(&url_user, &user_ext)?;
    uploads_handler::handle_nc_uploads(state, req, user_ext, upload_id, rest)
        .await
        .map_err(|e| e.into_response())
}

async fn handle_dav_uploads_root(
    State(state): State<Arc<AppState>>,
    Path((url_user, upload_id)): Path<(String, String)>,
    user_ext: CurrentUser,
    req: Request<Body>,
) -> Result<Response, Response> {
    verify_url_user(&url_user, &user_ext)?;
    uploads_handler::handle_nc_uploads(state, req, user_ext, upload_id, String::new())
        .await
        .map_err(|e| e.into_response())
}

/// Legacy /remote.php/webdav/* — redirect to /remote.php/dav/files/{user}/*
async fn handle_legacy_webdav(Path(subpath): Path<String>, user_ext: CurrentUser) -> Response {
    let location = format!("/remote.php/dav/files/{}/{}", user_ext.username, subpath);
    Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header("location", location)
        .body(Body::empty())
        .unwrap()
}

async fn handle_legacy_webdav_root(user_ext: CurrentUser) -> Response {
    let location = format!("/remote.php/dav/files/{}/", user_ext.username);
    Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header("location", location)
        .body(Body::empty())
        .unwrap()
}

async fn handle_dav_trashbin(
    State(state): State<Arc<AppState>>,
    Path((url_user, subpath)): Path<(String, String)>,
    user_ext: CurrentUser,
    req: Request<Body>,
) -> Result<Response, Response> {
    verify_url_user(&url_user, &user_ext)?;
    trashbin_handler::handle_nc_trashbin(state, req, user_ext, subpath)
        .await
        .map_err(|e| e.into_response())
}

async fn handle_dav_trashbin_root(
    State(state): State<Arc<AppState>>,
    Path(url_user): Path<String>,
    user_ext: CurrentUser,
    req: Request<Body>,
) -> Result<Response, Response> {
    verify_url_user(&url_user, &user_ext)?;
    trashbin_handler::handle_nc_trashbin(state, req, user_ext, String::new())
        .await
        .map_err(|e| e.into_response())
}
