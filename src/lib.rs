// Export the main project modules
pub mod application;
pub mod common;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;

// Common public re-exports
pub use application::services::folder_service::FolderService;
pub use application::services::i18n_application_service::I18nApplicationService;
pub use domain::services::path_service::StoragePath;
pub use infrastructure::services::compression_service::GzipCompressionService;
pub use infrastructure::services::path_service::PathService;
