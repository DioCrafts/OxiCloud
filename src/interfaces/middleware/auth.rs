use std::sync::Arc;
use axum::{
    extract::{State, Request},
    http::{StatusCode, HeaderMap, header},
    middleware::Next,
    response::{Response, IntoResponse},
    body::Body,
};

use crate::common::di::AppState;

// Extensión para almacenar datos del usuario autenticado
#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

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
    
    // More permissive bypass conditions, especially for file uploads
    let skip_validation = uri.contains("no_redirect=true") || 
                         uri.contains("bypass_auth=true") || 
                         uri.contains("files/upload");
    
    if skip_validation {
        tracing::info!("Bypassing token validation for: {} (special URL pattern)", uri);
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
    
    // Examine headers for debugging
    let has_auth_header = headers.contains_key(header::AUTHORIZATION);
    let content_type = headers.get(header::CONTENT_TYPE).and_then(|v| v.to_str().ok()).unwrap_or("none");
    tracing::debug!("Request auth status: has_auth_header={}, content_type={}", has_auth_header, content_type);
    
    // Check for multipart content type (special handling for file uploads)
    if content_type.contains("multipart/form-data") {
        tracing::info!("Detected multipart form data, relaxing auth for file upload");
        let current_user = CurrentUser {
            id: "upload-user-id".to_string(),
            username: "upload-user".to_string(),
            email: "upload@example.com".to_string(),
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
        
        // Accept any token for now in development mode
        tracing::info!("Valid token detected: {}", token_str.chars().take(8).collect::<String>() + "...");
        
        // For regular tokens, create a test user (this will be replaced with real validation)
        let current_user = CurrentUser {
            id: "auth-user-id".to_string(), 
            username: "auth-user".to_string(),
            email: "auth@example.com".to_string(),
            role: "user".to_string(),
        };
        
        // Añadir usuario a la request
        request.extensions_mut().insert(current_user);
        return Ok(next.run(request).await);
    }
    
    // Special endpoints that should work without authentication
    let public_endpoints = [
        "api/login", 
        "api/register", 
        "api/i18n",
        "api/s/", // Public shares
    ];
    
    // Check if the URI contains any of the public endpoints
    if public_endpoints.iter().any(|endpoint| uri.contains(endpoint)) {
        tracing::info!("Allowing access to public endpoint without token: {}", uri);
        // Add a minimal user context for public endpoints
        let current_user = CurrentUser {
            id: "public-user-id".to_string(),
            username: "public".to_string(),
            email: "public@example.com".to_string(),
            role: "public".to_string(),
        };
        request.extensions_mut().insert(current_user);
        return Ok(next.run(request).await);
    }
    
    // Development mode - enable this to bypass all auth in development
    #[cfg(debug_assertions)]
    {
        tracing::warn!("⚠️ Development mode: bypassing authentication for all requests");
        let current_user = CurrentUser {
            id: "dev-user-id".to_string(),
            username: "developer".to_string(),
            email: "dev@example.com".to_string(),
            role: "admin".to_string(),
        };
        request.extensions_mut().insert(current_user);
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