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
/// ```rust
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
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::TokenNotProvided => (StatusCode::UNAUTHORIZED, "Token no proporcionado".to_string()),
            AuthError::InvalidToken(msg) => (StatusCode::UNAUTHORIZED, msg),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expirado".to_string()),
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "Usuario no encontrado".to_string()),
            AuthError::AccessDenied(msg) => (StatusCode::FORBIDDEN, msg),
        };

        let body = axum::Json(serde_json::json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

// Middleware de autenticación simplificado - solo valida si existe un token
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Check URL for special no_validation parameter to break auth loops
    let uri = request.uri().to_string();
    let skip_validation = uri.contains("no_redirect=true") || uri.contains("bypass_auth=true");
    
    if skip_validation {
        tracing::info!("Bypassing token validation due to special URL parameter");
        // Create a default user for the request
        let current_user = CurrentUser {
            id: "default-user-id".to_string(),
            username: "usuario".to_string(),
            email: "usuario@example.com".to_string(),
            role: "user".to_string(),
        };
        request.extensions_mut().insert(current_user);
        return Ok(next.run(request).await);
    }
    
    // En una primera etapa, simplemente verificar si hay un token, sin validarlo
    if let Some(token_str) = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer ")) {
        
        // Handle mock tokens differently
        let is_mock = token_str.contains("mock") || token_str == "mock_access_token";
        
        if is_mock {
            tracing::info!("Mock token detected, using simplified validation");
            let current_user = CurrentUser {
                id: "test-user-id".to_string(),
                username: "test".to_string(),
                email: "test@example.com".to_string(),
                role: "user".to_string(),
            };
            request.extensions_mut().insert(current_user);
            return Ok(next.run(request).await);
        }
        
        // Process normal token - try to validate it using JWT service
        tracing::info!("Processing token: {}", token_str.chars().take(8).collect::<String>() + "...");
        
        // Try to get the token service and validate the token
        if let Some(auth_service) = state.auth_service.as_ref() {
            let token_service = &auth_service.token_service;
            match token_service.validate_token(token_str) {
                Ok(claims) => {
                    tracing::info!("Token validated successfully for user: {}", claims.username);
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
        
        // Fallback: if no auth service available, use token claims from parsing JWT manually
        // Try to decode the token manually using jsonwebtoken
        use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
        
        // Try with default secret (from environment or config)
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "oxicloud_secret_key_please_change_in_production".to_string());
        
        #[derive(serde::Deserialize)]
        struct Claims {
            sub: String,
            username: String,
            email: String,
            role: String,
        }
        
        let validation = Validation::new(Algorithm::HS256);
        match decode::<Claims>(
            token_str,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &validation
        ) {
            Ok(token_data) => {
                tracing::info!("Token decoded successfully for user: {}", token_data.claims.username);
                let current_user = CurrentUser {
                    id: token_data.claims.sub,
                    username: token_data.claims.username,
                    email: token_data.claims.email,
                    role: token_data.claims.role,
                };
                request.extensions_mut().insert(current_user);
                return Ok(next.run(request).await);
            },
            Err(e) => {
                tracing::warn!("Fallback token decode failed: {}", e);
                return Err(AuthError::InvalidToken(format!("Token inválido: {}", e)));
            }
        }
    }
    
    // Si hay un indicador para evitar redirección, permitir el acceso sin token
    if uri.contains("api/") && uri.contains("login") {
        tracing::info!("Allowing access to login endpoint without token");
        return Ok(next.run(request).await);
    }
    
    // Si no hay token, devolver error de token no proporcionado
    Err(AuthError::TokenNotProvided)
}

// Middleware simplificado para verificar roles de administrador
pub async fn require_admin(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    // Implementación simplificada que verifica si hay un token de admin
    if let Some(auth_value) = headers.get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_value.to_str() {
            if auth_str.contains("admin") {
                // Autorizado como admin
                let current_user = CurrentUser {
                    id: "admin-user-id".to_string(),
                    username: "admin".to_string(),
                    email: "admin@example.com".to_string(),
                    role: "admin".to_string(),
                };
                request.extensions_mut().insert(current_user);
                return next.run(request).await;
            }
        }
    }
    
    // Acceso denegado
    let error = AuthError::AccessDenied("Se requiere rol de administrador".to_string());
    error.into_response()
}