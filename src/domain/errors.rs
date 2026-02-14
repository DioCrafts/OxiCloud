//! Domain errors
//!
//! This module contains domain-specific error types.
//! DomainError is the base error used throughout the domain layer.

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error;

/// Common Result type for the domain with DomainError as the standard error
pub type Result<T> = std::result::Result<T, DomainError>;

/// Domain error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Entity not found
    NotFound,
    /// Entity already exists
    AlreadyExists,
    /// Invalid input or failed validation
    InvalidInput,
    /// Access or permissions error
    AccessDenied,
    /// Timeout expired
    Timeout,
    /// Internal system error
    InternalError,
    /// Functionality not implemented
    NotImplemented,
    /// Unsupported operation
    UnsupportedOperation,
    /// Database error
    DatabaseError,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ErrorKind::NotFound => write!(f, "Not Found"),
            ErrorKind::AlreadyExists => write!(f, "Already Exists"),
            ErrorKind::InvalidInput => write!(f, "Invalid Input"),
            ErrorKind::AccessDenied => write!(f, "Access Denied"),
            ErrorKind::Timeout => write!(f, "Timeout"),
            ErrorKind::InternalError => write!(f, "Internal Error"),
            ErrorKind::NotImplemented => write!(f, "Not Implemented"),
            ErrorKind::UnsupportedOperation => write!(f, "Unsupported Operation"),
            ErrorKind::DatabaseError => write!(f, "Database Error"),
        }
    }
}

/// Base domain error that provides detailed context
#[derive(Error, Debug)]
#[error("{kind}: {message}")]
pub struct DomainError {
    /// Error type
    pub kind: ErrorKind,
    /// Affected entity type (e.g.: "File", "Folder")
    pub entity_type: &'static str,
    /// Entity identifier if available
    pub entity_id: Option<String>,
    /// Descriptive error message
    pub message: String,
    /// Source error (optional)
    #[source]
    pub source: Option<Box<dyn StdError + Send + Sync>>,
}

impl DomainError {
    /// Creates a new domain error
    pub fn new<S: Into<String>>(kind: ErrorKind, entity_type: &'static str, message: S) -> Self {
        Self {
            kind,
            entity_type,
            entity_id: None,
            message: message.into(),
            source: None,
        }
    }

    /// Creates an entity not found error
    pub fn not_found<S: Into<String>>(entity_type: &'static str, entity_id: S) -> Self {
        let id = entity_id.into();
        Self {
            kind: ErrorKind::NotFound,
            entity_type,
            entity_id: Some(id.clone()),
            message: format!("{} not found: {}", entity_type, id),
            source: None,
        }
    }

    /// Creates an entity already exists error
    pub fn already_exists<S: Into<String>>(entity_type: &'static str, entity_id: S) -> Self {
        let id = entity_id.into();
        Self {
            kind: ErrorKind::AlreadyExists,
            entity_type,
            entity_id: Some(id.clone()),
            message: format!("{} already exists: {}", entity_type, id),
            source: None,
        }
    }

    /// Creates an error for unsupported operations
    pub fn operation_not_supported<S: Into<String>>(entity_type: &'static str, message: S) -> Self {
        Self::new(ErrorKind::UnsupportedOperation, entity_type, message)
    }

    /// Creates a timeout error
    pub fn timeout<S: Into<String>>(entity_type: &'static str, message: S) -> Self {
        Self {
            kind: ErrorKind::Timeout,
            entity_type,
            entity_id: None,
            message: message.into(),
            source: None,
        }
    }

    /// Creates an internal error
    pub fn internal_error<S: Into<String>>(entity_type: &'static str, message: S) -> Self {
        Self {
            kind: ErrorKind::InternalError,
            entity_type,
            entity_id: None,
            message: message.into(),
            source: None,
        }
    }

    /// Creates an access denied error
    pub fn access_denied<S: Into<String>>(entity_type: &'static str, message: S) -> Self {
        Self {
            kind: ErrorKind::AccessDenied,
            entity_type,
            entity_id: None,
            message: message.into(),
            source: None,
        }
    }

    /// Alias for access_denied to maintain compatibility
    pub fn unauthorized<S: Into<String>>(message: S) -> Self {
        Self {
            kind: ErrorKind::AccessDenied,
            entity_type: "Authorization",
            entity_id: None,
            message: message.into(),
            source: None,
        }
    }

    /// Creates a database error
    pub fn database_error<S: Into<String>>(message: S) -> Self {
        Self {
            kind: ErrorKind::DatabaseError,
            entity_type: "Database",
            entity_id: None,
            message: message.into(),
            source: None,
        }
    }

    /// Creates a validation error
    pub fn validation_error<S: Into<String>>(message: S) -> Self {
        Self {
            kind: ErrorKind::InvalidInput,
            entity_type: "Validation",
            entity_id: None,
            message: message.into(),
            source: None,
        }
    }

    /// Creates a not implemented error
    pub fn not_implemented<S: Into<String>>(entity_type: &'static str, message: S) -> Self {
        Self {
            kind: ErrorKind::NotImplemented,
            entity_type,
            entity_id: None,
            message: message.into(),
            source: None,
        }
    }

    /// Sets the entity ID
    pub fn with_id<S: Into<String>>(mut self, entity_id: S) -> Self {
        self.entity_id = Some(entity_id.into());
        self
    }

    /// Sets the source error
    pub fn with_source<E: StdError + Send + Sync + 'static>(mut self, source: E) -> Self {
        self.source = Some(Box::new(source));
        self
    }
}

/// Trait for adding context to errors
pub trait ErrorContext<T, E> {
    fn with_context<C, F>(self, context: F) -> std::result::Result<T, DomainError>
    where
        C: Into<String>,
        F: FnOnce() -> C;

    fn with_error_kind(
        self,
        kind: ErrorKind,
        entity_type: &'static str,
    ) -> std::result::Result<T, DomainError>;
}

impl<T, E: StdError + Send + Sync + 'static> ErrorContext<T, E> for std::result::Result<T, E> {
    fn with_context<C, F>(self, context: F) -> std::result::Result<T, DomainError>
    where
        C: Into<String>,
        F: FnOnce() -> C,
    {
        self.map_err(|e| DomainError {
            kind: ErrorKind::InternalError,
            entity_type: "Unknown",
            entity_id: None,
            message: context().into(),
            source: Some(Box::new(e)),
        })
    }

    fn with_error_kind(
        self,
        kind: ErrorKind,
        entity_type: &'static str,
    ) -> std::result::Result<T, DomainError> {
        self.map_err(|e| DomainError {
            kind,
            entity_type,
            entity_id: None,
            message: format!("{}", e),
            source: Some(Box::new(e)),
        })
    }
}

// From implementations for standard errors (without external infrastructure dependencies)
impl From<std::io::Error> for DomainError {
    fn from(err: std::io::Error) -> Self {
        DomainError {
            kind: ErrorKind::InternalError,
            entity_type: "IO",
            entity_id: None,
            message: format!("{}", err),
            source: Some(Box::new(err)),
        }
    }
}

impl From<uuid::Error> for DomainError {
    fn from(err: uuid::Error) -> Self {
        DomainError {
            kind: ErrorKind::InvalidInput,
            entity_type: "UUID",
            entity_id: None,
            message: format!("{}", err),
            source: Some(Box::new(err)),
        }
    }
}
