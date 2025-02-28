//! Gestión de errores centralizada

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use derive_more::{Display, Error};
use log::error;
use serde::Serialize;
use std::fmt;

use crate::api::ApiResponse;

/// Errores de la API REST
#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display("Error de validación: {}", _0)]
    #[error(ignore)]
    ValidationError(String),
    
    #[display("Error de autenticación: {}", _0)]
    #[error(ignore)]
    Unauthorized(String),
    
    #[display("Acceso prohibido: {}", _0)]
    #[error(ignore)]
    Forbidden(String),
    
    #[display("No encontrado: {}", _0)]
    #[error(ignore)]
    NotFound(String),
    
    #[display("Solicitud inválida: {}", _0)]
    #[error(ignore)]
    BadRequest(String),
    
    #[display("Error interno del servidor")]
    InternalError,
    
    #[display("Error del servidor: {}", _0)]
    #[error(ignore)]
    ServerError(String),
}


impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        
        // Construir respuesta JSON con formato consistente
        let response = ApiResponse::<()>::error(self.to_string());
        
        // Loguear errores internos del servidor
        if status_code == StatusCode::INTERNAL_SERVER_ERROR {
            error!("Error interno del servidor: {}", self);
        }
        
        HttpResponse::build(status_code)
            .json(response)
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// Conversiones comunes de errores
impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        error!("Error de base de datos: {:?}", error);
        ApiError::ServerError("Error de base de datos".to_string())
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        error!("Error anyhow: {:?}", error);
        ApiError::ServerError("Error interno del servidor".to_string())
    }
}

impl From<std::io::Error> for ApiError {
    fn from(error: std::io::Error) -> Self {
        error!("Error de I/O: {:?}", error);
        ApiError::ServerError("Error de I/O".to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        error!("Error de JSON: {:?}", error);
        ApiError::BadRequest("Formato JSON inválido".to_string())
    }
}

impl From<actix_web::Error> for ApiError {
    fn from(error: actix_web::Error) -> Self {
        error!("Error de Actix-web: {:?}", error);
        ApiError::ServerError("Error interno del servidor".to_string())
    }
}

impl From<actix_web::http::header::ToStrError> for ApiError {
    fn from(error: actix_web::http::header::ToStrError) -> Self {
        error!("Error de header: {:?}", error);
        ApiError::BadRequest("Formato de header inválido".to_string())
    }
}

/// Error personalizado para la aplicación
#[derive(Debug, Serialize)]
pub struct AppError {
    pub code: u16,
    pub message: String,
    #[serde(skip)]
    pub cause: Option<String>,
}

impl AppError {
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            cause: None,
        }
    }
    
    pub fn with_cause(mut self, cause: impl fmt::Display) -> Self {
        self.cause = Some(cause.to_string());
        self
    }
    
    /// Devuelve un error de "no encontrado"
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(404, message)
    }
    
    /// Devuelve un error de "no autorizado"
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(401, message)
    }
    
    /// Devuelve un error de "prohibido"
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(403, message)
    }
    
    /// Devuelve un error de "solicitud inválida"
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(400, message)
    }
    
    /// Devuelve un error de "error interno del servidor"
    pub fn internal_error() -> Self {
        Self::new(500, "Error interno del servidor")
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        error!("Error de base de datos: {:?}", error);
        Self::new(500, "Error de base de datos").with_cause(error)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        error!("Error anyhow: {:?}", error);
        Self::new(500, "Error interno").with_cause(error)
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        error!("Error de I/O: {:?}", error);
        Self::new(500, "Error de I/O").with_cause(error)
    }
}