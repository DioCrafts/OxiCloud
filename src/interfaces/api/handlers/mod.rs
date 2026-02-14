pub mod admin_handler;
pub mod auth_handler;
pub mod batch_handler;
pub mod caldav_handler;
pub mod carddav_handler;
pub mod chunked_upload_handler;
pub mod dedup_handler;
pub mod favorites_handler;
pub mod file_handler;
pub mod folder_handler;
pub mod i18n_handler;
pub mod recent_handler;
pub mod search_handler;
pub mod share_handler;
pub mod trash_handler;
pub mod webdav_handler;

/// Tipo de resultado para controladores de API
pub type ApiResult<T> = Result<T, (axum::http::StatusCode, String)>;
