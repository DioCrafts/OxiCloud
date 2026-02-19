use axum::{
    extract::{FromRequestParts, Request, State},
    http::{HeaderMap, StatusCode, header, request::Parts},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::convert::Infallible;
use std::sync::Arc;

use crate::application::dtos::user_dto::LoginDto;
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

    #[error("Invalid Basic Auth credentials")]
    InvalidBasicAuth,
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
            AuthError::InvalidBasicAuth => (
                StatusCode::UNAUTHORIZED,
                "Invalid username or password".to_string(),
            ),
        };

        let body = axum::Json(serde_json::json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

/// Authenticate using Basic Auth (username:password base64 encoded)
async fn authenticate_basic_auth(
    auth_header: &str,
    state: &Arc<AppState>,
) -> Result<CurrentUser, AuthError> {
    // Strip "Basic " prefix
    let credentials = auth_header
        .strip_prefix("Basic ")
        .ok_or(AuthError::InvalidBasicAuth)?;

    // Decode base64
    let decoded = base64::decode(credentials.trim()).map_err(|_| AuthError::InvalidBasicAuth)?;

    let decoded_str = String::from_utf8(decoded).map_err(|_| AuthError::InvalidBasicAuth)?;

    // Split username:password
    let parts: Vec<&str> = decoded_str.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(AuthError::InvalidBasicAuth);
    }

    let username = parts[0];
    let password = parts[1];

    if username.is_empty() || password.is_empty() {
        return Err(AuthError::InvalidBasicAuth);
    }

    tracing::debug!("Basic Auth attempt for user: {}", username);

    // Get auth service
    let auth_service = state
        .auth_service
        .as_ref()
        .ok_or(AuthError::AuthServiceUnavailable)?;

    // Create login DTO and attempt login
    let login_dto = LoginDto {
        username: username.to_string(),
        password: password.to_string(),
    };

    match auth_service.auth_application_service.login(login_dto).await {
        Ok(auth_response) => {
            tracing::info!("Basic Auth successful for user: {}", username);
            let user = auth_response.user;
            Ok(CurrentUser {
                id: user.id,
                username: user.username,
                email: user.email,
                role: user.role,
            })
        }
        Err(e) => {
            tracing::warn!("Basic Auth failed for user {}: {}", username, e);
            Err(AuthError::InvalidBasicAuth)
        }
    }
}

/// Authenticate using Bearer Token (JWT)
async fn authenticate_bearer_token(
    auth_header: &str,
    state: &Arc<AppState>,
) -> Result<CurrentUser, AuthError> {
    // Strip "Bearer " prefix
    let token_str = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::TokenNotProvided)?
        .trim();

    if token_str.is_empty() {
        return Err(AuthError::TokenNotProvided);
    }

    tracing::debug!("Processing Bearer token");

    // Get auth service
    let auth_service = state
        .auth_service
        .as_ref()
        .ok_or(AuthError::AuthServiceUnavailable)?;

    // Validate token
    match auth_service.token_service.validate_token(token_str) {
        Ok(claims) => {
            tracing::debug!("Token validated successfully for user: {}", claims.username);
            Ok(CurrentUser {
                id: claims.sub,
                username: claims.username,
                email: claims.email,
                role: claims.role,
            })
        }
        Err(e) => {
            tracing::warn!("Token validation failed: {}", e);
            Err(AuthError::InvalidToken(format!("Invalid token: {}", e)))
        }
    }
}

/// Secure authentication middleware.
///
/// Supports both JWT Bearer Token and HTTP Basic Authentication.
/// - Bearer Token: `Authorization: Bearer <jwt_token>`
/// - Basic Auth: `Authorization: Basic <base64(username:password)>`
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Extract Authorization header
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(AuthError::TokenNotProvided)?;

    // Determine auth type and authenticate
    let current_user = if auth_header.starts_with("Bearer ") {
        authenticate_bearer_token(auth_header, &state).await?
    } else if auth_header.starts_with("Basic ") {
        authenticate_basic_auth(auth_header, &state).await?
    } else {
        return Err(AuthError::TokenNotProvided);
    };

    // Insert CurrentUser into request extensions
    request.extensions_mut().insert(current_user);
    Ok(next.run(request).await)
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
