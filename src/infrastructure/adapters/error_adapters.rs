//! Infrastructure Error Adapters
//!
//! This module contains error conversion adapters for infrastructure-specific errors.
//! These adapters bridge the gap between infrastructure errors (sqlx, serde_json, etc.)
//! and domain errors, keeping the domain layer clean of infrastructure knowledge.
//!
//! Following Clean Architecture principles, these conversions are placed in the
//! infrastructure layer rather than the common/domain layers.

use crate::domain::errors::{DomainError, ErrorKind};

/// Macro to create From implementations for infrastructure errors to DomainError.
///
/// This macro is intended for use ONLY within the infrastructure layer.
/// The domain layer should not depend on specific infrastructure error types.
///
/// # Example
///
/// ```ignore
/// // In infrastructure code:
/// impl_infra_error_to_domain!(serde_json::Error, "Serialization");
/// impl_infra_error_to_domain!(sqlx::Error, "Database");
/// ```
#[macro_export]
macro_rules! impl_infra_error_to_domain {
    ($error_type:ty, $entity_type:expr) => {
        impl From<$error_type> for $crate::domain::errors::DomainError {
            fn from(err: $error_type) -> Self {
                $crate::domain::errors::DomainError {
                    kind: $crate::domain::errors::ErrorKind::InternalError,
                    entity_type: $entity_type,
                    entity_id: None,
                    message: format!("{}", err),
                    source: Some(Box::new(err)),
                }
            }
        }
    };
}

// Note: We intentionally DO NOT create global From implementations for sqlx::Error
// or serde_json::Error here. Each repository/service should handle its own error
// conversions with proper context. This prevents the domain from depending on
// infrastructure error types.

/// Helper trait for converting infrastructure errors to DomainError with context.
///
/// This trait provides a more explicit way to convert infrastructure errors
/// to domain errors, requiring the caller to provide context about the entity
/// being operated on.
pub trait IntoDomainError {
    /// Convert the error to a DomainError with the given entity type context.
    fn into_domain_error(self, entity_type: &'static str) -> DomainError;
}

impl IntoDomainError for std::io::Error {
    fn into_domain_error(self, entity_type: &'static str) -> DomainError {
        DomainError::new(
            ErrorKind::InternalError,
            entity_type,
            format!("IO error: {}", self),
        ).with_source(self)
    }
}

impl IntoDomainError for serde_json::Error {
    fn into_domain_error(self, entity_type: &'static str) -> DomainError {
        DomainError::new(
            ErrorKind::InternalError,
            entity_type,
            format!("Serialization error: {}", self),
        ).with_source(self)
    }
}

impl IntoDomainError for sqlx::Error {
    fn into_domain_error(self, entity_type: &'static str) -> DomainError {
        match &self {
            sqlx::Error::RowNotFound => {
                DomainError::not_found(entity_type, "Record not found")
            }
            sqlx::Error::Database(db_err) => {
                // Handle specific PostgreSQL error codes
                if db_err.code().is_some_and(|c| c == "23505") {
                    DomainError::already_exists(entity_type, "Record already exists")
                } else {
                    DomainError::new(
                        ErrorKind::DatabaseError,
                        entity_type,
                        format!("Database error: {}", db_err),
                    ).with_source(self)
                }
            }
            _ => DomainError::new(
                ErrorKind::InternalError,
                entity_type,
                format!("Database error: {}", self),
            ).with_source(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let domain_error = io_error.into_domain_error("File");
        
        assert_eq!(domain_error.entity_type, "File");
        assert!(domain_error.message.contains("IO error"));
    }

    #[test]
    fn test_serde_json_error_conversion() {
        let json_str = "{ invalid json }";
        let serde_error: serde_json::Error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
        let domain_error = serde_error.into_domain_error("Config");
        
        assert_eq!(domain_error.entity_type, "Config");
        assert!(domain_error.message.contains("Serialization error"));
    }
}
