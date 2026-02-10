use axum::{
    Router,
    routing::{get, put, post},
    extract::{State, Json},
    http::{StatusCode, HeaderMap, header},
    response::IntoResponse,
};

use crate::common::di::AppState;
use crate::application::dtos::settings_dto::{SaveOidcSettingsDto, TestOidcConnectionDto};
use crate::interfaces::errors::AppError;

/// Admin API routes — all require admin role.
pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/settings/oidc", get(get_oidc_settings))
        .route("/settings/oidc", put(save_oidc_settings))
        .route("/settings/oidc/test", post(test_oidc_connection))
        .route("/settings/general", get(get_general_settings))
}

/// Validate JWT and require admin role. Returns (user_id, role).
async fn admin_guard(state: &AppState, headers: &HeaderMap) -> Result<(String, String), AppError> {
    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::unauthorized("Authorization token required"))?;

    let claims = auth.token_service.validate_token(token)
        .map_err(|e| AppError::unauthorized(&format!("Invalid token: {}", e)))?;

    if claims.role != "admin" {
        return Err(AppError::new(
            StatusCode::FORBIDDEN,
            "Admin access required",
            "Forbidden",
        ));
    }

    Ok((claims.sub, claims.role))
}

/// GET /api/admin/settings/oidc — get OIDC settings for the admin panel
async fn get_oidc_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let svc = state.admin_settings_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Admin settings service not available"))?;

    let settings = svc.get_oidc_settings().await
        .map_err(|e| AppError::internal_error(&format!("Failed to load settings: {}", e)))?;

    Ok(Json(settings))
}

/// PUT /api/admin/settings/oidc — save OIDC settings + hot-reload
async fn save_oidc_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(dto): Json<SaveOidcSettingsDto>,
) -> Result<impl IntoResponse, AppError> {
    let (user_id, _) = admin_guard(&state, &headers).await?;

    let svc = state.admin_settings_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Admin settings service not available"))?;

    svc.save_oidc_settings(dto, &user_id).await
        .map_err(|e| AppError::internal_error(&format!("Failed to save settings: {}", e)))?;

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "OIDC settings saved and applied successfully"
    }))))
}

/// POST /api/admin/settings/oidc/test — test OIDC discovery
async fn test_oidc_connection(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(dto): Json<TestOidcConnectionDto>,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let svc = state.admin_settings_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Admin settings service not available"))?;

    let result = svc.test_oidc_connection(dto).await
        .map_err(|e| AppError::internal_error(&format!("Connection test failed: {}", e)))?;

    Ok(Json(result))
}

/// GET /api/admin/settings/general — system overview
async fn get_general_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    admin_guard(&state, &headers).await?;

    let auth = state.auth_service.as_ref()
        .ok_or_else(|| AppError::internal_error("Auth service not configured"))?;

    let user_count = auth.auth_application_service.count_all_users().await.unwrap_or(0);
    let oidc_configured = auth.auth_application_service.oidc_enabled();

    Ok(Json(serde_json::json!({
        "server_version": env!("CARGO_PKG_VERSION"),
        "auth_enabled": true,
        "total_users": user_count,
        "oidc_configured": oidc_configured,
    })))
}
