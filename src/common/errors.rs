//! Application errors
//!
//! This module re-exports domain errors for compatibility.
//! Infrastructure error conversions (sqlx, serde_json, etc.)
//! are located in infrastructure/adapters/error_adapters.rs, following
//! Clean Architecture principles where the domain should not know about
//! infrastructure details.

// Re-export domain errors for compatibility
pub use crate::domain::errors::{DomainError, ErrorKind, Result};

// Re-export AppError from interfaces for backward compatibility
// NOTE: The canonical location of AppError is now crate::interfaces::errors

// Infrastructure error conversions have been moved to:
// crate::infrastructure::adapters::error_adapters
//
// To convert infrastructure errors to DomainError, use:
// - The IntoDomainError trait for explicit conversions with context
// - Or handle errors in infrastructure repositories/services
//   using map_err() with DomainError::internal_error() or similar methods
