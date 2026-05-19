//! HTTP/API Error types for the interfaces layer.
//!
//! This module contains error types specific to the HTTP/API layer.
//! These errors handle the conversion from domain errors to HTTP responses.

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

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
    pub error_code: Option<&'static str>,
}

/// JSON response structure for errors.
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: String,
    pub error: String,
    pub error_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<&'static str>,
}

impl AppError {
    /// Create a new AppError with custom status code, message and error type.
    pub fn new(
        status_code: StatusCode,
        message: impl Into<String>,
        error_type: impl Into<String>,
    ) -> Self {
        Self {
            status_code,
            message: message.into(),
            error_type: error_type.into(),
            error_code: None,
        }
    }

    /// Create a 400 Bad Request error.
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message, "bad_request")
    }

    /// Create a 401 Unauthorized error.
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message, "unauthorized")
    }

    /// Create a 403 Forbidden error.
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, message, "forbidden")
    }

    /// Create a 404 Not Found error.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message, "not_found")
    }

    /// Create a 500 Internal Server Error.
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message, "internal_error")
    }

    /// Create a 405 Method Not Allowed error.
    pub fn method_not_allowed(message: impl Into<String>) -> Self {
        Self::new(
            StatusCode::METHOD_NOT_ALLOWED,
            message,
            "method_not_allowed",
        )
    }

    /// Create a 409 Conflict error.
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, message, "conflict")
    }

    /// Create a 423 Locked error (WebDAV).
    pub fn locked(message: impl Into<String>) -> Self {
        Self::new(StatusCode::LOCKED, message, "locked")
    }

    /// Create a 415 Unsupported Media Type error.
    pub fn unsupported_media_type(message: impl Into<String>) -> Self {
        Self::new(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            message,
            "unsupported_media_type",
        )
    }

    /// Create a 412 Precondition Failed error.
    pub fn precondition_failed(message: impl Into<String>) -> Self {
        Self::new(
            StatusCode::PRECONDITION_FAILED,
            message,
            "precondition_failed",
        )
    }

    /// Create a 413 Payload Too Large error.
    pub fn payload_too_large(message: impl Into<String>) -> Self {
        Self::new(StatusCode::PAYLOAD_TOO_LARGE, message, "payload_too_large")
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
            ErrorKind::QuotaExceeded => StatusCode::INSUFFICIENT_STORAGE,
        };

        Self {
            status_code,
            message: err.message,
            error_type: err.kind.to_string(),
            error_code: err.error_code,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code;

        // Sanitize 500 Internal Server Error to prevent information leakage.
        // Log the full error server-side for debugging, return a generic
        // message to the client. Other status codes (including 5xx like
        // 501, 503, 507) keep their intentionally user-facing messages.
        let client_message = if status == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!(
                error_type = %self.error_type,
                "Internal server error: {}",
                self.message
            );
            "An internal error occurred. Please try again later.".to_string()
        } else {
            if status.is_client_error() {
                tracing::warn!("{}", self.message);
            }
            self.message
        };

        let error_response = ErrorResponse {
            status: status.to_string(),
            error: client_message,
            error_type: self.error_type,
            error_code: self.error_code,
        };

        let body = Json(error_response);
        (status, body).into_response()
    }
}
