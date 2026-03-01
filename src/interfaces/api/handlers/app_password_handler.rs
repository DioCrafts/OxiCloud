//! HTTP handlers for App Password management.
//!
//! All endpoints require JWT authentication (the user must be logged in to
//! create/list/revoke their app passwords).

use crate::application::dtos::app_password_dto::CreateAppPasswordRequestDto;
use crate::common::di::AppState;
use crate::interfaces::errors::AppError;
use crate::interfaces::middleware::auth::CurrentUser;
use axum::extract::State;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use std::sync::Arc;

/// Protected routes — require JWT auth middleware.
pub fn app_password_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/app-passwords", post(create_app_password))
        .route("/app-passwords", get(list_app_passwords))
        .route("/app-passwords/{id}", delete(revoke_app_password))
}

/// POST /api/auth/app-passwords — Create a new app password.
///
/// Returns the plain-text password ONCE. The user must copy it immediately.
async fn create_app_password(
    State(state): State<Arc<AppState>>,
    axum::Extension(user): axum::Extension<CurrentUser>,
    Json(request): Json<CreateAppPasswordRequestDto>,
) -> Result<Json<crate::application::dtos::app_password_dto::AppPasswordCreatedResponseDto>, AppError>
{
    let service = state
        .app_password_service
        .as_ref()
        .ok_or_else(|| AppError::internal_error("App password service not configured"))?;

    let response = service
        .create(&user.id, request)
        .await
        .map_err(|e| AppError::from(e))?;

    Ok(Json(response))
}

/// GET /api/auth/app-passwords — List all app passwords for the current user.
///
/// Never returns plain-text passwords (only prefix + metadata).
async fn list_app_passwords(
    State(state): State<Arc<AppState>>,
    axum::Extension(user): axum::Extension<CurrentUser>,
) -> Result<Json<crate::application::dtos::app_password_dto::AppPasswordListResponseDto>, AppError>
{
    let service = state
        .app_password_service
        .as_ref()
        .ok_or_else(|| AppError::internal_error("App password service not configured"))?;

    let response = service
        .list(&user.id)
        .await
        .map_err(|e| AppError::from(e))?;

    Ok(Json(response))
}

/// DELETE /api/auth/app-passwords/:id — Revoke an app password.
async fn revoke_app_password(
    State(state): State<Arc<AppState>>,
    axum::Extension(user): axum::Extension<CurrentUser>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<crate::application::dtos::app_password_dto::AppPasswordRevokeResponseDto>, AppError>
{
    let service = state
        .app_password_service
        .as_ref()
        .ok_or_else(|| AppError::internal_error("App password service not configured"))?;

    let response = service
        .revoke(&user.id, &id)
        .await
        .map_err(|e| AppError::from(e))?;

    Ok(Json(response))
}
