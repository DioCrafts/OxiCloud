use std::sync::Arc;
use axum::{
    extract::{State, Request, FromRequestParts},
    http::{StatusCode, HeaderMap, header, request::Parts},
    middleware::Next,
    response::{Response, IntoResponse},
};

use crate::common::di::AppState;

// Re-export CurrentUser from application layer for use in handlers
pub use crate::application::dtos::user_dto::CurrentUser;

// Estructura para usar en extractores de Axum
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
}

/// Extractor reutilizable que obtiene el user_id del usuario autenticado.
/// Se extrae automáticamente del `CurrentUser` insertado por el auth middleware.
///
/// Uso en handlers:
/// ```ignore
/// async fn my_handler(CurrentUserId(user_id): CurrentUserId) -> impl IntoResponse { ... }
/// ```
#[derive(Clone, Debug)]
pub struct CurrentUserId(pub String);

// Implementar FromRequestParts para AuthUser — permite usar `auth_user: AuthUser` en handlers
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

// Implementar FromRequestParts para CurrentUserId — extractor ligero solo para el user_id
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

// Error para las operaciones de autenticación
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Token no proporcionado")]
    TokenNotProvided,
    
    #[error("Token inválido: {0}")]
    InvalidToken(String),
    
    #[error("Token expirado")]
    TokenExpired,
    
    #[error("Usuario no encontrado")]
    UserNotFound,
    
    #[error("Acceso denegado: {0}")]
    AccessDenied(String),
    
    #[error("Servicio de autenticación no disponible")]
    AuthServiceUnavailable,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::TokenNotProvided => (StatusCode::UNAUTHORIZED, "Token no proporcionado".to_string()),
            AuthError::InvalidToken(msg) => (StatusCode::UNAUTHORIZED, msg),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expirado".to_string()),
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "Usuario no encontrado".to_string()),
            AuthError::AccessDenied(msg) => (StatusCode::FORBIDDEN, msg),
            AuthError::AuthServiceUnavailable => (StatusCode::INTERNAL_SERVER_ERROR, "Servicio de autenticación no disponible".to_string()),
        };

        let body = axum::Json(serde_json::json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

/// Middleware de autenticación seguro.
///
/// Valida el token JWT contra el servicio de autenticación configurado.
/// No acepta bypasses, tokens mock, ni parámetros de URL para saltar validación.
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Extraer el token Bearer del header Authorization
    let token_str = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(AuthError::TokenNotProvided)?;
    
    // Validar que el token no esté vacío
    let token_str = token_str.trim();
    if token_str.is_empty() {
        return Err(AuthError::TokenNotProvided);
    }
    
    tracing::debug!("Processing authentication token");
    
    // Validar el token usando el servicio de autenticación
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
                return Err(AuthError::InvalidToken(format!("Token inválido: {}", e)));
            }
        }
    }
    
    // Si no hay servicio de autenticación disponible, denegar acceso
    tracing::error!("Auth middleware invoked but auth service is not configured");
    Err(AuthError::AuthServiceUnavailable)
}

/// Middleware para verificar que el usuario autenticado tiene rol de administrador.
///
/// Debe aplicarse DESPUÉS del auth_middleware, ya que depende de que
/// `CurrentUser` esté presente en las extensiones de la request.
pub async fn require_admin(
    request: Request,
    next: Next,
) -> Response {
    // Obtener el CurrentUser insertado por auth_middleware
    if let Some(current_user) = request.extensions().get::<CurrentUser>() {
        if current_user.role == "admin" {
            tracing::debug!("Admin access granted for user: {}", current_user.username);
            return next.run(request).await;
        }
        tracing::warn!("Admin access denied for user: {} (role: {})", current_user.username, current_user.role);
    } else {
        tracing::warn!("Admin check failed: no authenticated user in request");
    }
    
    // Acceso denegado
    let error = AuthError::AccessDenied("Se requiere rol de administrador".to_string());
    error.into_response()
}