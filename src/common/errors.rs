//! Errores de la aplicación
//!
//! Este módulo re-exporta los errores del dominio para compatibilidad.
//! Las conversiones de errores de infraestructura (sqlx, serde_json, etc.)
//! se encuentran en infrastructure/adapters/error_adapters.rs, siguiendo
//! los principios de Clean Architecture donde el dominio no debe conocer
//! detalles de infraestructura.

// Re-exportar errores del dominio para compatibilidad
pub use crate::domain::errors::{DomainError, ErrorKind, Result};

// Re-exportar AppError desde interfaces para compatibilidad hacia atrás
// NOTA: El lugar canónico de AppError es ahora crate::interfaces::errors

// Las conversiones de errores de infraestructura se han movido a:
// crate::infrastructure::adapters::error_adapters
//
// Para convertir errores de infraestructura a DomainError, use:
// - El trait IntoDomainError para conversiones explícitas con contexto
// - O maneje los errores en los repositorios/servicios de infraestructura
//   usando map_err() con DomainError::internal_error() o métodos similares
