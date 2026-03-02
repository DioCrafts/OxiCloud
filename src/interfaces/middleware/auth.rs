use axum::{
    extract::{FromRequestParts, Request, State},
    http::{HeaderMap, StatusCode, header, request::Parts},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::convert::Infallible;
use std::sync::Arc;

use crate::common::di::AppState;

// Re-export CurrentUser from application layer for use in handlers
pub use crate::application::dtos::user_dto::CurrentUser;

// Structure for use in Axum extractors
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
}

/// Reusable extractor that gets the user_id of the authenticated user.
/// Automatically extracted from the `CurrentUser` inserted by the auth middleware.
///
/// Usage in handlers:
/// ```ignore
/// async fn my_handler(CurrentUserId(user_id): CurrentUserId) -> impl IntoResponse { ... }
/// ```
#[derive(Clone, Debug)]
pub struct CurrentUserId(pub String);

// Implement FromRequestParts for AuthUser — allows using `auth_user: AuthUser` in handlers
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<CurrentUser>()
            .map(|cu| AuthUser {
                id: cu.id.clone(),
                username: cu.username.clone(),
            })
            .ok_or(AuthError::UserNotFound)
    }
}

// Implement FromRequestParts for CurrentUserId — lightweight extractor for user_id only
impl<S> FromRequestParts<S> for CurrentUserId
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<CurrentUser>()
            .map(|cu| CurrentUserId(cu.id.clone()))
            .ok_or(AuthError::UserNotFound)
    }
}

/// Optional user ID extractor – never fails.
/// Yields `Some(id)` when auth middleware ran, `None` otherwise.
#[derive(Clone, Debug)]
pub struct OptionalUserId(pub Option<String>);

impl<S> FromRequestParts<S> for OptionalUserId
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(OptionalUserId(
            parts
                .extensions
                .get::<CurrentUser>()
                .map(|cu| cu.id.clone()),
        ))
    }
}

/// Optional auth user extractor – never fails.
/// Yields `Some(AuthUser)` when auth middleware ran, `None` otherwise.
#[derive(Clone, Debug)]
pub struct OptionalAuthUser(pub Option<AuthUser>);

impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(OptionalAuthUser(parts.extensions.get::<CurrentUser>().map(
            |cu| AuthUser {
                id: cu.id.clone(),
                username: cu.username.clone(),
            },
        )))
    }
}

// Error for authentication operations
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Token not provided")]
    TokenNotProvided,

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("Access denied: {0}")]
    AccessDenied(String),

    #[error("Authentication service unavailable")]
    AuthServiceUnavailable,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::TokenNotProvided => {
                (StatusCode::UNAUTHORIZED, "Token not provided".to_string())
            }
            AuthError::InvalidToken(msg) => (StatusCode::UNAUTHORIZED, msg),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired".to_string()),
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "User not found".to_string()),
            AuthError::AccessDenied(msg) => (StatusCode::FORBIDDEN, msg),
            AuthError::AuthServiceUnavailable => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Authentication service unavailable".to_string(),
            ),
        };

        let body = axum::Json(serde_json::json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

/// Secure authentication middleware.
///
/// Supports two authentication methods:
/// 1. **Bearer JWT** — standard token in `Authorization: Bearer <token>`
/// 2. **Basic Auth with App Passwords** — for DAV clients (DAVx⁵, Thunderbird, rclone)
///    that send `Authorization: Basic base64(username:app_password)`
///
/// Bearer is tried first; if no Bearer header is found, Basic is attempted.
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    // ── 1. Try Bearer JWT ────────────────────────────────────────
    if let Some(header_value) = auth_header {
        if let Some(token_str) = header_value.strip_prefix("Bearer ") {
            let token_str = token_str.trim();
            if !token_str.is_empty() {
                tracing::debug!("Processing Bearer authentication token");

                if let Some(auth_service) = state.auth_service.as_ref() {
                    let token_service = &auth_service.token_service;
                    match token_service.validate_token(token_str) {
                        Ok(claims) => {
                            tracing::debug!(
                                "Token validated successfully for user: {}",
                                claims.username
                            );
                            let current_user = CurrentUser {
                                id: claims.sub,
                                username: claims.username,
                                email: claims.email,
                                role: claims.role,
                            };
                            request.extensions_mut().insert(current_user);
                            return Ok(next.run(request).await);
                        }
                        Err(e) => {
                            tracing::warn!("Bearer token validation failed: {}", e);
                            return Err(AuthError::InvalidToken(format!(
                                "Invalid token: {}",
                                e
                            )));
                        }
                    }
                }
            }
        }

        // ── 2. Try Basic Auth with App Passwords ─────────────────
        if let Some(basic_encoded) = header_value.strip_prefix("Basic ") {
            let basic_encoded = basic_encoded.trim();
            if !basic_encoded.is_empty() {
                tracing::debug!("Processing Basic authentication (app password)");

                // Decode base64(username:password)
                use base64::Engine;
                let decoded = base64::engine::general_purpose::STANDARD
                    .decode(basic_encoded)
                    .map_err(|_| {
                        AuthError::InvalidToken("Invalid Basic auth encoding".to_string())
                    })?;
                let credentials = String::from_utf8(decoded).map_err(|_| {
                    AuthError::InvalidToken("Invalid Basic auth encoding".to_string())
                })?;

                let (username, password) = credentials.split_once(':').ok_or_else(|| {
                    AuthError::InvalidToken("Invalid Basic auth format".to_string())
                })?;

                if let Some(app_pw_service) = state.app_password_service.as_ref() {
                    match app_pw_service.verify_basic_auth(username, password).await {
                        Ok((user_id, uname, email, role)) => {
                            tracing::debug!(
                                "App password authentication successful for user: {}",
                                uname
                            );
                            let current_user = CurrentUser {
                                id: user_id,
                                username: uname,
                                email,
                                role,
                            };
                            request.extensions_mut().insert(current_user);
                            return Ok(next.run(request).await);
                        }
                        Err(e) => {
                            tracing::warn!("App password verification failed: {}", e);
                            return Err(AuthError::InvalidToken(
                                "Invalid username or app password".to_string(),
                            ));
                        }
                    }
                } else {
                    tracing::warn!("Basic auth attempted but app password service not configured");
                    return Err(AuthError::InvalidToken(
                        "App passwords are not enabled".to_string(),
                    ));
                }
            }
        }
    }

    // No valid Authorization header found
    if state.auth_service.is_none() {
        tracing::error!("Auth middleware invoked but auth service is not configured");
        return Err(AuthError::AuthServiceUnavailable);
    }

    Err(AuthError::TokenNotProvided)
}

/// Middleware to verify that the authenticated user has an admin role.
///
/// Must be applied AFTER auth_middleware, as it depends on
/// `CurrentUser` being present in the request extensions.
pub async fn require_admin(request: Request, next: Next) -> Response {
    // Get the CurrentUser inserted by auth_middleware
    if let Some(current_user) = request.extensions().get::<CurrentUser>() {
        if current_user.role == "admin" {
            tracing::debug!("Admin access granted for user: {}", current_user.username);
            return next.run(request).await;
        }
        tracing::warn!(
            "Admin access denied for user: {} (role: {})",
            current_user.username,
            current_user.role
        );
    } else {
        tracing::warn!("Admin check failed: no authenticated user in request");
    }

    // Access denied
    let error = AuthError::AccessDenied("Admin role required".to_string());
    error.into_response()
}
