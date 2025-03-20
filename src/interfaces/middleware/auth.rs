use std::sync::Arc;
use axum::{
    extract::{State, Request, FromRequestParts},
    http::{StatusCode, request::Parts, HeaderMap, header},
    middleware::Next,
    response::{Response, IntoResponse},
    body::Body,
    RequestPartsExt,
};
use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::common::di::AppState;
use crate::common::errors::AppError;
use crate::domain::entities::user::UserRole;

// Extensión para almacenar datos del usuario autenticado
#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
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

// Middleware de autenticación que verifica el token JWT
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Verificar si hay un token 
    if let Some(token_str) = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer ")) {
        
        // Aquí deberías validar el token JWT y extraer la información del usuario
        // En una implementación real, necesitaríamos verificar la firma, expiración, etc.
        
        // Por ahora, usaremos un enfoque simplificado para desarrollo
        // TODO: Implementar validación JWT completa
        if let Some(auth_service) = &state.auth_service {
            match auth_service.auth_service.validate_token(token_str) {
                Ok(claims) => {
                    // Construir el usuario a partir de las claims del token
                    let current_user = CurrentUser {
                        id: claims.sub,
                        username: claims.username,
                        email: claims.email,
                        role: claims.role,
                    };
                    
                    // Añadir usuario a la request
                    request.extensions_mut().insert(current_user);
                    return Ok(next.run(request).await);
                },
                Err(e) => {
                    tracing::warn!("Error validando token: {}", e);
                    return Err(AuthError::InvalidToken(e.to_string()));
                }
            }
        } else {
            // Fallback para entornos de desarrollo sin servicio de autenticación
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
    }
    
    // Si no hay token, devolver error de token no proporcionado
    Err(AuthError::TokenNotProvided)
}

// Middleware para verificar roles de administrador
pub async fn require_admin(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    // Verificar si el usuario ya está autenticado (desde middleware anterior)
    if let Some(current_user) = request.extensions().get::<CurrentUser>() {
        // Verificar si el usuario tiene rol de administrador
        if current_user.role == "admin" {
            // Autorizado como admin
            return next.run(request).await;
        } else {
            // Usuario autenticado pero no es admin
            let error = AuthError::AccessDenied(format!(
                "El usuario {} no tiene permisos de administrador", current_user.username
            ));
            return error.into_response();
        }
    }
    
    // Si llegamos aquí, no hay usuario autenticado en el contexto
    // Esto no debería ocurrir si auth_middleware se ejecuta antes que este middleware
    let error = AuthError::TokenNotProvided;
    error.into_response()
}