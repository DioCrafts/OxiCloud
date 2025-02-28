//! Módulo de API REST para OxiCloud
//! 
//! Este módulo proporciona endpoints REST para acceder
//! a la funcionalidad de OxiCloud desde clientes externos.

pub mod auth;
pub mod files;
pub mod users;

// Re-exportar helpers comunes para APIs
pub use crate::utils::errors::ApiError;

/// Respuesta estandarizada para endpoints de API
#[derive(serde::Serialize)]
pub struct ApiResponse<T>
where
    T: serde::Serialize,
{
    pub status: String,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T>
where
    T: serde::Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            status: "success".to_string(),
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            status: "error".to_string(),
            data: None,
            message: Some(message.into()),
        }
    }
}