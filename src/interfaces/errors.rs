//! HTTP/API Error types for the interfaces layer.
//!
//! This module contains error types specific to the HTTP/API layer.
//! These errors handle the conversion from domain errors to HTTP responses.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

use crate::domain::errors::{DomainError, ErrorKind};

/// Error type for HTTP/API responses.
/// 
/// This struct represents errors that will be returned to HTTP clients.
/// It contains the HTTP status code, a user-friendly message, and an error type identifier.
#[derive(Debug)]
pub struct AppError {
    pub status_code: StatusCode,
    pub message: String,
    pub error_type: String,
}

/// JSON response structure for errors.
#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    pub error_type: String,
}

impl AppError {
    /// Create a new AppError with custom status code, message and error type.
    pub fn new(status_code: StatusCode, message: impl Into<String>, error_type: impl Into<String>) -> Self {
        Self {
            status_code,
            message: message.into(),
            error_type: error_type.into(),
        }
    }
    
    /// Create a 400 Bad Request error.
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message, "BadRequest")
    }
    
    /// Create a 401 Unauthorized error.
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message, "Unauthorized")
    }
    
    /// Create a 403 Forbidden error.
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, message, "Forbidden")
    }
    
    /// Create a 404 Not Found error.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message, "NotFound")
    }
    
    /// Create a 500 Internal Server Error.
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message, "InternalError")
    }
    
    /// Create a 405 Method Not Allowed error.
    pub fn method_not_allowed(message: impl Into<String>) -> Self {
        Self::new(StatusCode::METHOD_NOT_ALLOWED, message, "MethodNotAllowed")
    }
    
    /// Create a 409 Conflict error.
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, message, "Conflict")
    }
    
    /// Create a 415 Unsupported Media Type error.
    pub fn unsupported_media_type(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNSUPPORTED_MEDIA_TYPE, message, "UnsupportedMediaType")
    }
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        let status_code = match err.kind {
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            ErrorKind::AlreadyExists => StatusCode::CONFLICT,
            ErrorKind::InvalidInput => StatusCode::BAD_REQUEST,
            ErrorKind::AccessDenied => StatusCode::FORBIDDEN,
            ErrorKind::Timeout => StatusCode::REQUEST_TIMEOUT,
            ErrorKind::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            ErrorKind::UnsupportedOperation => StatusCode::METHOD_NOT_ALLOWED,
            ErrorKind::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        Self {
            status_code,
            message: err.message,
            error_type: err.kind.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code;
        let error_response = ErrorResponse {
            status: status.to_string(),
            message: self.message,
            error_type: self.error_type,
        };
        
        let body = Json(error_response);
        (status, body).into_response()
    }
}
