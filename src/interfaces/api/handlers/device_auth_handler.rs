//! HTTP handlers for OAuth 2.0 Device Authorization Grant (RFC 8628).
//!
//! Endpoints:
//!   POST /api/auth/device/authorize — Client starts the device flow (public)
//!   GET  /api/auth/device/verify    — Check user_code validity (authenticated)
//!   POST /api/auth/device/verify    — User approves/denies (authenticated)
//!   POST /api/auth/device/token     — Client polls for tokens (public)
//!   GET  /api/auth/device/devices   — List user's authorized devices (authenticated)
//!   DELETE /api/auth/device/devices/{id} — Revoke a device (authenticated)

use axum::{
    Router,
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use std::sync::Arc;

use crate::application::dtos::device_auth_dto::*;
use crate::application::services::device_auth_service::DeviceAuthService;
use crate::common::di::AppState;
use crate::interfaces::errors::AppError;
use crate::interfaces::middleware::auth::AuthUser;

/// Create the device auth router.
///
/// Public endpoints (no auth middleware): authorize, token
/// Protected endpoints (behind auth middleware): verify (GET+POST), devices
pub fn device_auth_public_routes() -> Router<Arc<AppState>> {
    Router::new()
        // Client-facing endpoints (no auth needed — the client doesn't have tokens yet)
        .route("/authorize", post(device_authorize))
        .route("/token", post(device_token))
}

pub fn device_auth_protected_routes() -> Router<Arc<AppState>> {
    Router::new()
        // User-facing endpoints (require valid session)
        .route("/verify", get(device_verify_info))
        .route("/verify", post(device_verify_action))
        .route("/devices", get(list_devices))
        .route("/devices/{id}", delete(revoke_device))
}

// ============================================================================
// POST /api/auth/device/authorize — Client initiates the device flow
// ============================================================================

/// Client sends: `{ "client_name": "rclone", "scope": "webdav" }`
/// Server returns: device_code, user_code, verification_uri, etc.
async fn device_authorize(
    State(state): State<Arc<AppState>>,
    Json(body): Json<DeviceAuthorizeRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    let device_service = get_device_service(&state)?;

    let response = device_service.initiate(body).await.map_err(|e| {
        tracing::error!("Device authorize failed: {}", e);
        AppError::from(e)
    })?;

    Ok((StatusCode::OK, Json(response)))
}

// ============================================================================
// POST /api/auth/device/token — Client polls for tokens
// ============================================================================

/// Client sends: `{ "device_code": "...", "grant_type": "urn:ietf:params:oauth:grant-type:device_code" }`
/// Returns tokens on success, or RFC 8628 error codes while pending.
async fn device_token(
    State(state): State<Arc<AppState>>,
    Json(body): Json<DeviceTokenRequestDto>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let device_service = match get_device_service(&state) {
        Ok(svc) => svc,
        Err(e) => return Err(e.into_response()),
    };

    // Validate grant_type if provided (RFC compliance)
    if !body.grant_type.is_empty()
        && body.grant_type != "urn:ietf:params:oauth:grant-type:device_code"
    {
        let error_body = serde_json::json!({
            "error": "unsupported_grant_type",
            "error_description": "grant_type must be urn:ietf:params:oauth:grant-type:device_code"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_body)).into_response());
    }

    match device_service.poll(&body.device_code).await {
        Ok(tokens) => Ok((StatusCode::OK, Json(tokens)).into_response()),
        Err(poll_err) => {
            let status = StatusCode::from_u16(poll_err.http_status())
                .unwrap_or(StatusCode::BAD_REQUEST);
            let error_body = serde_json::json!({
                "error": poll_err.error_code(),
                "error_description": poll_err.description()
            });
            Err((status, Json(error_body)).into_response())
        }
    }
}

// ============================================================================
// GET /api/auth/device/verify?code=ABCD-1234 — Check if user_code is valid
// ============================================================================

#[derive(serde::Deserialize)]
pub struct VerifyQuery {
    #[serde(default)]
    pub code: String,
}

async fn device_verify_info(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Query(query): Query<VerifyQuery>,
) -> Result<impl IntoResponse, AppError> {
    let device_service = get_device_service(&state)?;

    let info = device_service
        .verify_user_code(&query.code)
        .await
        .map_err(|e| {
            tracing::warn!("Device verify lookup failed: {}", e);
            AppError::from(e)
        })?;

    Ok((StatusCode::OK, Json(info)))
}

// ============================================================================
// POST /api/auth/device/verify — User approves or denies
// ============================================================================

async fn device_verify_action(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(body): Json<DeviceVerifyRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    let device_service = get_device_service(&state)?;

    match body.action.to_lowercase().as_str() {
        "approve" | "allow" | "accept" => {
            device_service
                .approve(&body.user_code, &auth_user.id)
                .await
                .map_err(|e| {
                    tracing::error!("Device approve failed: {}", e);
                    AppError::from(e)
                })?;
            Ok((
                StatusCode::OK,
                Json(serde_json::json!({ "status": "approved" })),
            ))
        }
        "deny" | "reject" | "cancel" => {
            device_service.deny(&body.user_code).await.map_err(|e| {
                tracing::error!("Device deny failed: {}", e);
                AppError::from(e)
            })?;
            Ok((
                StatusCode::OK,
                Json(serde_json::json!({ "status": "denied" })),
            ))
        }
        _ => Err(AppError::bad_request(
            "action must be 'approve' or 'deny'",
        )),
    }
}

// ============================================================================
// GET /api/auth/device/devices — List user's authorized devices
// ============================================================================

async fn list_devices(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let device_service = get_device_service(&state)?;

    let devices = device_service
        .list_user_devices(&auth_user.id)
        .await
        .map_err(|e| {
            tracing::error!("List devices failed: {}", e);
            AppError::from(e)
        })?;

    Ok((StatusCode::OK, Json(devices)))
}

// ============================================================================
// DELETE /api/auth/device/devices/{id} — Revoke a device authorization
// ============================================================================

async fn revoke_device(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(device_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let device_service = get_device_service(&state)?;

    device_service
        .revoke_device(&device_id, &auth_user.id)
        .await
        .map_err(|e| {
            tracing::error!("Revoke device failed: {}", e);
            AppError::from(e)
        })?;

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Helper
// ============================================================================

fn get_device_service(state: &AppState) -> Result<&Arc<DeviceAuthService>, AppError> {
    state
        .device_auth_service
        .as_ref()
        .ok_or_else(|| AppError::internal_error("Device authorization service not configured"))
}
