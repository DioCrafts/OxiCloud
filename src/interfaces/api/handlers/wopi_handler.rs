//! WOPI protocol handler.
//!
//! Implements the WOPI host endpoints called by document editors
//! (Collabora Online, OnlyOffice) to access and modify files.
//!
//! These endpoints use `?access_token=` query parameter auth, NOT the
//! regular JWT auth middleware.
//!
//! Reference: doc/wopi-integration.md

use crate::interfaces::middleware::auth::AuthUser;
use axum::{
    Router,
    body::Body,
    extract::{Path, Query, State},
    http::{HeaderMap, Request, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::application::services::wopi_lock_service::WopiLockService;
use crate::application::services::wopi_token_service::WopiTokenService;
use crate::infrastructure::services::wopi_discovery_service::WopiDiscoveryService;

/// Shared state for WOPI handlers.
#[derive(Clone)]
pub struct WopiState {
    pub token_service: Arc<WopiTokenService>,
    pub lock_service: Arc<WopiLockService>,
    pub discovery_service: Arc<WopiDiscoveryService>,
    pub app_state: Arc<crate::common::di::AppState>,
    /// Public base URL for host page origin and postMessage origin
    pub public_base_url: String,
    /// Base URL used for WOPISrc callbacks from Collabora to OxiCloud
    pub wopi_base_url: String,
}

/// Query parameter for WOPI access token.
#[derive(Deserialize)]
pub struct WopiTokenQuery {
    pub access_token: String,
}

/// CheckFileInfo response (WOPI spec).
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CheckFileInfoResponse {
    pub base_file_name: String,
    pub owner_id: String,
    pub size: u64,
    pub user_id: String,
    pub version: String,
    pub supports_locks: bool,
    pub supports_update: bool,
    pub supports_rename: bool,
    pub user_can_write: bool,
    pub user_friendly_name: String,
    pub post_message_origin: String,
    pub last_modified_time: String,
    pub close_url: String,
}

/// GET /wopi/files/{file_id} — CheckFileInfo
async fn check_file_info(
    Path(file_id): Path<String>,
    Query(token_query): Query<WopiTokenQuery>,
    State(state): State<WopiState>,
) -> Response {
    let claims = match state
        .token_service
        .validate_token(&token_query.access_token)
    {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if claims.file_id != file_id {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    // Fetch file metadata
    let file = match state
        .app_state
        .applications
        .file_retrieval_service
        .get_file(&file_id)
        .await
    {
        Ok(f) => f,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    // Convert u64 timestamp to RFC 3339 string
    let last_modified = chrono::DateTime::from_timestamp(file.modified_at as i64, 0)
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_default();

    let response = CheckFileInfoResponse {
        base_file_name: file.name.clone(),
        owner_id: file.owner_id.clone().unwrap_or_else(|| claims.sub.clone()),
        size: file.size,
        user_id: claims.sub.clone(),
        version: file.modified_at.to_string(),
        supports_locks: true,
        supports_update: claims.can_write,
        supports_rename: false,
        user_can_write: claims.can_write,
        user_friendly_name: claims.username.clone(),
        post_message_origin: state.public_base_url.clone(),
        last_modified_time: last_modified,
        close_url: state.public_base_url.clone(),
    };

    axum::Json(response).into_response()
}

/// GET /wopi/files/{file_id}/contents — GetFile
///
/// Streams the file content to Collabora/OnlyOffice in 64 KB chunks.
/// Memory usage is constant (~64 KB) regardless of file size.
async fn get_file(
    Path(file_id): Path<String>,
    Query(token_query): Query<WopiTokenQuery>,
    State(state): State<WopiState>,
) -> Response {
    let claims = match state
        .token_service
        .validate_token(&token_query.access_token)
    {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if claims.file_id != file_id {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    match state
        .app_state
        .applications
        .file_retrieval_service
        .get_file_stream(&file_id)
        .await
    {
        Ok(stream) => {
            let body = axum::body::Body::from_stream(std::pin::Pin::from(stream));
            (StatusCode::OK, body).into_response()
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

/// POST /wopi/files/{file_id}/contents — PutFile
///
/// **Streaming implementation**: the request body is spooled to a temp file
/// with incremental SHA-256 hashing.  Peak RAM usage is ~256 KB regardless
/// of file size (previously buffered the entire body as `Bytes`).
async fn put_file(
    Path(file_id): Path<String>,
    Query(token_query): Query<WopiTokenQuery>,
    headers: HeaderMap,
    State(state): State<WopiState>,
    req: Request<Body>,
) -> Response {
    use http_body_util::BodyStream;
    use sha2::{Digest, Sha256};
    use tokio::io::AsyncWriteExt;
    use tokio_stream::StreamExt;

    let claims = match state
        .token_service
        .validate_token(&token_query.access_token)
    {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if claims.file_id != file_id || !claims.can_write {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    // Check lock
    let request_lock = headers
        .get("X-WOPI-Lock")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let current_lock = state.lock_service.get_lock(&file_id).await;

    if let Some(ref current) = current_lock {
        match &request_lock {
            Some(req_lock) if req_lock == current => {
                // Lock matches — proceed
            }
            _ => {
                // Lock mismatch
                return (
                    StatusCode::CONFLICT,
                    [("X-WOPI-Lock", current.as_str())],
                    "Lock mismatch",
                )
                    .into_response();
            }
        }
    }

    // Get file metadata for the path
    let file = match state
        .app_state
        .applications
        .file_retrieval_service
        .get_file(&file_id)
        .await
    {
        Ok(f) => f,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    // ── Streaming spool: body → temp file + incremental SHA-256 ──
    let temp_file = match tempfile::NamedTempFile::new() {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("WOPI PutFile: failed to create temp file: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    let temp_path = temp_file.path().to_path_buf();

    let mut file_out = match tokio::fs::File::create(&temp_path).await {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("WOPI PutFile: failed to open temp file: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let content_type = file.mime_type.clone();
    let mut hasher = Sha256::new();
    let mut total_bytes: u64 = 0;
    let mut stream = BodyStream::new(req.into_body());

    while let Some(frame_result) = stream.next().await {
        let frame = match frame_result {
            Ok(f) => f,
            Err(e) => {
                let _ = tokio::fs::remove_file(&temp_path).await;
                tracing::error!("WOPI PutFile: body read error: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        if let Some(chunk) = frame.data_ref() {
            total_bytes += chunk.len() as u64;
            hasher.update(chunk);
            if let Err(e) = file_out.write_all(chunk).await {
                let _ = tokio::fs::remove_file(&temp_path).await;
                tracing::error!("WOPI PutFile: temp write error: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    }
    if let Err(e) = file_out.flush().await {
        let _ = tokio::fs::remove_file(&temp_path).await;
        tracing::error!("WOPI PutFile: flush error: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    drop(file_out);

    let hash = hex::encode(hasher.finalize());

    // ── Atomic store: temp file → dedup blob + DB metadata update ──
    let result = state
        .app_state
        .applications
        .file_upload_service
        .update_file_streaming(
            &file.path,
            &temp_path,
            total_bytes,
            &content_type,
            Some(hash),
        )
        .await;

    // Clean up temp file (may already be moved by dedup, ignore error)
    let _ = tokio::fs::remove_file(&temp_path).await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            tracing::error!("WOPI PutFile failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// POST /wopi/files/{file_id} — Dispatches lock operations based on X-WOPI-Override header
async fn file_operations(
    Path(file_id): Path<String>,
    Query(token_query): Query<WopiTokenQuery>,
    headers: HeaderMap,
    State(state): State<WopiState>,
) -> Response {
    let claims = match state
        .token_service
        .validate_token(&token_query.access_token)
    {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if claims.file_id != file_id {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let override_header = headers
        .get("X-WOPI-Override")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let lock_id = headers
        .get("X-WOPI-Lock")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    match override_header {
        "LOCK" => {
            if lock_id.is_empty() {
                return StatusCode::BAD_REQUEST.into_response();
            }
            match state.lock_service.lock(&file_id, lock_id).await {
                Ok(()) => StatusCode::OK.into_response(),
                Err(conflict) => (
                    StatusCode::CONFLICT,
                    [("X-WOPI-Lock", conflict.existing_lock_id.as_str())],
                    "",
                )
                    .into_response(),
            }
        }
        "UNLOCK" => match state.lock_service.unlock(&file_id, lock_id).await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(conflict) => (
                StatusCode::CONFLICT,
                [("X-WOPI-Lock", conflict.existing_lock_id.as_str())],
                "",
            )
                .into_response(),
        },
        "REFRESH_LOCK" => match state.lock_service.refresh_lock(&file_id, lock_id).await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(conflict) => (
                StatusCode::CONFLICT,
                [("X-WOPI-Lock", conflict.existing_lock_id.as_str())],
                "",
            )
                .into_response(),
        },
        "GET_LOCK" => {
            let current = state.lock_service.get_lock(&file_id).await;
            let lock_val = current.unwrap_or_default();
            (StatusCode::OK, [("X-WOPI-Lock", lock_val.as_str())], "").into_response()
        }
        _ => (StatusCode::NOT_IMPLEMENTED, "Unknown WOPI override").into_response(),
    }
}

/// Parameters for the editor URL API endpoint.
#[derive(Deserialize)]
pub struct EditorUrlParams {
    pub file_id: String,
    #[serde(default = "default_action")]
    pub action: String,
}

fn default_action() -> String {
    "edit".to_string()
}

/// Response from the editor URL API endpoint.
#[derive(Serialize)]
pub struct EditorUrlResponse {
    pub editor_url: String,
    pub access_token: String,
    pub access_token_ttl: i64,
}

/// GET /api/wopi/editor-url — Returns the editor iframe URL + WOPI token.
///
/// This endpoint is behind normal auth middleware. The authenticated user
/// requests a WOPI session for a specific file.
pub async fn get_editor_url(
    AuthUser {
        id: user_id,
        username,
    }: AuthUser,
    Query(params): Query<EditorUrlParams>,
    State(state): State<WopiState>,
) -> Response {
    // Get file info to determine extension
    let file = match state
        .app_state
        .applications
        .file_retrieval_service
        .get_file(&params.file_id)
        .await
    {
        Ok(f) => f,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    // Extract extension from filename
    let extension = file.name.rsplit('.').next().unwrap_or("").to_lowercase();

    // Build WOPISrc
    let wopi_src = format!("{}/wopi/files/{}", state.wopi_base_url, params.file_id);

    // Get editor action URL from discovery
    let editor_url = match state
        .discovery_service
        .get_action_url(&extension, &params.action, &wopi_src)
        .await
    {
        Ok(Some(url)) => url,
        Ok(None) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("No editor available for .{} files", extension),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!("WOPI discovery error: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // Determine write permission: owner can write, others read-only.
    // If no owner_id on the file, default to allowing write.
    let can_write = match &file.owner_id {
        Some(owner) => owner == &user_id,
        None => true,
    };

    // Generate WOPI access token
    let (access_token, access_token_ttl) =
        match state
            .token_service
            .generate_token(&params.file_id, &user_id, &username, can_write)
        {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("Failed to generate WOPI token: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

    axum::Json(EditorUrlResponse {
        editor_url,
        access_token,
        access_token_ttl,
    })
    .into_response()
}

/// GET /wopi/edit/{file_id} — Server-rendered host page for new-tab editing.
///
/// Returns a minimal HTML page that POSTs the access token to the editor iframe.
async fn host_page(
    Path(file_id): Path<String>,
    Query(token_query): Query<WopiTokenQuery>,
    State(state): State<WopiState>,
) -> Response {
    let claims = match state
        .token_service
        .validate_token(&token_query.access_token)
    {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if claims.file_id != file_id {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    // Get file info for extension
    let file = match state
        .app_state
        .applications
        .file_retrieval_service
        .get_file(&file_id)
        .await
    {
        Ok(f) => f,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    let extension = file.name.rsplit('.').next().unwrap_or("").to_lowercase();
    let action = if claims.can_write { "edit" } else { "view" };
    let wopi_src = format!("{}/wopi/files/{}", state.wopi_base_url, file_id);

    let editor_url = match state
        .discovery_service
        .get_action_url(&extension, action, &wopi_src)
        .await
    {
        Ok(Some(url)) => url,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let (token, ttl) = match state.token_service.generate_token(
        &file_id,
        &claims.sub,
        &claims.username,
        claims.can_write,
    ) {
        Ok(t) => t,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // Escape HTML entities in file name
    let safe_name = file
        .name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;");

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{safe_name} - OxiCloud Editor</title>
    <style>
        body {{ margin: 0; overflow: hidden; }}
        iframe {{ width: 100%; height: 100vh; border: none; }}
    </style>
</head>
<body>
    <form id="wopi_form" action="{editor_url}" method="post" target="wopi_frame">
        <input name="access_token" value="{token}" type="hidden"/>
        <input name="access_token_ttl" value="{ttl}" type="hidden"/>
    </form>
    <iframe name="wopi_frame" allowfullscreen
        sandbox="allow-scripts allow-same-origin allow-forms allow-popups allow-top-navigation allow-popups-to-escape-sandbox">
    </iframe>
    <script>document.getElementById('wopi_form').submit();</script>
</body>
</html>"#
    );

    Html(html).into_response()
}

/// GET /wopi/supported-extensions — Returns extensions the editor supports.
///
/// Public endpoint (no auth) so the frontend can dynamically show/hide
/// the "Edit in Office" context menu option.
async fn get_supported_extensions(State(state): State<WopiState>) -> Response {
    match state.discovery_service.get_supported_extensions().await {
        Ok(exts) => axum::Json(exts).into_response(),
        Err(e) => {
            tracing::error!("Failed to get supported extensions: {}", e);
            axum::Json(Vec::<String>::new()).into_response()
        }
    }
}

/// Build all WOPI routes.
///
/// Returns a tuple: (wopi_protocol_router, wopi_api_router)
/// - wopi_protocol_router: mounted at `/wopi` (no auth middleware)
/// - wopi_api_router: mounted at `/api/wopi` (behind auth middleware)
pub fn wopi_routes(
    wopi_state: WopiState,
) -> (
    Router<Arc<crate::common::di::AppState>>,
    Router<Arc<crate::common::di::AppState>>,
) {
    let protocol_router = Router::new()
        // CheckFileInfo
        .route("/files/{file_id}", get(check_file_info))
        // Lock/Unlock/RefreshLock/GetLock
        .route("/files/{file_id}", post(file_operations))
        // GetFile
        .route("/files/{file_id}/contents", get(get_file))
        // PutFile
        .route("/files/{file_id}/contents", post(put_file))
        // Host page for new-tab editing
        .route("/edit/{file_id}", get(host_page))
        // Supported extensions (public, no auth)
        .route("/supported-extensions", get(get_supported_extensions))
        .with_state(wopi_state.clone());

    let api_router = Router::new()
        .route("/editor-url", get(get_editor_url))
        .with_state(wopi_state);

    (protocol_router, api_router)
}
