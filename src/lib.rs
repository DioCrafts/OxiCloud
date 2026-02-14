// Export the main project modules
pub mod application;
pub mod common;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;

// Common public re-exports
pub use application::services::folder_service::FolderService;
pub use application::services::i18n_application_service::I18nApplicationService;
pub use application::services::storage_mediator::{FileSystemStorageMediator, StorageMediator};
pub use domain::services::path_service::StoragePath;
pub use infrastructure::repositories::CompositeFileRepository;
pub use infrastructure::repositories::folder_fs_repository::FolderFsRepository;
pub use infrastructure::repositories::parallel_file_processor::ParallelFileProcessor;
pub use infrastructure::services::buffer_pool::BufferPool;
pub use infrastructure::services::compression_service::GzipCompressionService;
pub use infrastructure::services::path_service::PathService;
