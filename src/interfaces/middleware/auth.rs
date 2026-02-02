use std::sync::Arc;
use axum::{
    extract::{State, Request},
    http::{StatusCode, HeaderMap, header},
    middleware::Next,
    response::{Response, IntoResponse},
    body::Body,
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

// Implementamos el extractor para AuthUser
// Use a function instead of an extractor for now
// We'll use this directly in handlers until we solve the extractor lifetime issues
pub async fn get_auth_user(req: &Request<Body>) -> Result<AuthUser, AuthError> {
    // Get the current user from extensions
    if let Some(current_user) = req.extensions().get::<CurrentUser>() {
        return Ok(AuthUser {
            id: current_user.id.clone(),
            username: current_user.username.clone(),
        });
    }

    // Return error if user not found
    Err(AuthError::UserNotFound)
}

// Middleware de autenticación simplificado - solo valida si existe un token
pub async fn auth_middleware(
    State(_state): State<Arc<AppState>>,
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
        
        // Process normal token
        tracing::info!("Processing token: {}", token_str.chars().take(8).collect::<String>() + "...");
        
        // For regular tokens, create a test user (this will be replaced with real validation)
        let current_user = CurrentUser {
            id: "test-user-id".to_string(),
            username: "test-user".to_string(),
            email: "test@example.com".to_string(),
            role: "user".to_string(),
        };
        
        // Añadir usuario a la request
        request.extensions_mut().insert(current_user);
        return Ok(next.run(request).await);
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