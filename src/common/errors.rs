//! Errores de la aplicación
//!
//! Este módulo re-exporta los errores del dominio y define utilidades
//! para conversión de errores de infraestructura.

// Re-exportar errores del dominio para compatibilidad
pub use crate::domain::errors::{DomainError, ErrorKind, Result};

// Re-exportar AppError desde interfaces para compatibilidad hacia atrás
// NOTA: El lugar canónico de AppError es ahora crate::interfaces::errors

// Macro para convertir errores específicos de infraestructura a DomainError
#[macro_export]
macro_rules! impl_from_error {
    ($error_type:ty, $entity_type:expr) => {
        impl From<$error_type> for crate::domain::errors::DomainError {
            fn from(err: $error_type) -> Self {
                crate::domain::errors::DomainError {
                    kind: crate::domain::errors::ErrorKind::InternalError,
                    entity_type: $entity_type,
                    entity_id: None,
                    message: format!("{}", err),
                    source: Some(Box::new(err)),
                }
            }
        }
    };
}

// Implementaciones para errores de infraestructura (sqlx, serde_json)
impl_from_error!(serde_json::Error, "Serialization");
impl_from_error!(sqlx::Error, "Database");
