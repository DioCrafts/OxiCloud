use std::sync::Arc;
use std::convert::Infallible;
use axum::{
    extract::{State, Request, FromRequestParts},
    http::{StatusCode, HeaderMap, header, request::Parts},
    middleware::Next,
    response::{Response, IntoResponse},
};

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
            parts.extensions.get::<CurrentUser>().map(|cu| cu.id.clone()),
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
        Ok(OptionalAuthUser(
            parts.extensions.get::<CurrentUser>().map(|cu| AuthUser {
                id: cu.id.clone(),
                username: cu.username.clone(),
            }),
        ))
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
            AuthError::TokenNotProvided => (StatusCode::UNAUTHORIZED, "Token not provided".to_string()),
            AuthError::InvalidToken(msg) => (StatusCode::UNAUTHORIZED, msg),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired".to_string()),
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "User not found".to_string()),
            AuthError::AccessDenied(msg) => (StatusCode::FORBIDDEN, msg),
            AuthError::AuthServiceUnavailable => (StatusCode::INTERNAL_SERVER_ERROR, "Authentication service unavailable".to_string()),
        };

        let body = axum::Json(serde_json::json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

/// Secure authentication middleware.
///
/// Validates the JWT token against the configured authentication service.
/// Does not accept bypasses, mock tokens, or URL parameters to skip validation.
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Extract the Bearer token from the Authorization header
    let token_str = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(AuthError::TokenNotProvided)?;
    
    // Validate that the token is not empty
    let token_str = token_str.trim();
    if token_str.is_empty() {
        return Err(AuthError::TokenNotProvided);
    }
    
    tracing::debug!("Processing authentication token");
    
    // Validate the token using the authentication service
    if let Some(auth_service) = state.auth_service.as_ref() {
        let token_service = &auth_service.token_service;
        match token_service.validate_token(token_str) {
            Ok(claims) => {
                tracing::debug!("Token validated successfully for user: {}", claims.username);
                let current_user = CurrentUser {
                    id: claims.sub,
                    username: claims.username,
                    email: claims.email,
                    role: claims.role,
                };
                request.extensions_mut().insert(current_user);
                return Ok(next.run(request).await);
            },
            Err(e) => {
                tracing::warn!("Token validation failed: {}", e);
                return Err(AuthError::InvalidToken(format!("Invalid token: {}", e)));
            }
        }
    }
    
    // If no authentication service is available, deny access
    tracing::error!("Auth middleware invoked but auth service is not configured");
    Err(AuthError::AuthServiceUnavailable)
}

/// Middleware to verify that the authenticated user has an admin role.
///
/// Must be applied AFTER auth_middleware, as it depends on
/// `CurrentUser` being present in the request extensions.
pub async fn require_admin(
    request: Request,
    next: Next,
) -> Response {
    // Get the CurrentUser inserted by auth_middleware
    if let Some(current_user) = request.extensions().get::<CurrentUser>() {
        if current_user.role == "admin" {
            tracing::debug!("Admin access granted for user: {}", current_user.username);
            return next.run(request).await;
        }
        tracing::warn!("Admin access denied for user: {} (role: {})", current_user.username, current_user.role);
    } else {
        tracing::warn!("Admin check failed: no authenticated user in request");
    }
    
    // Access denied
    let error = AuthError::AccessDenied("Admin role required".to_string());
    error.into_response()
}