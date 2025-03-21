pub mod file_handler;
pub mod folder_handler;
pub mod i18n_handler;
pub mod batch_handler;
pub mod auth_handler;

/// Tipo de resultado para controladores de API
pub type ApiResult<T> = Result<T, (axum::http::StatusCode, String)>;

