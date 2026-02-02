// Exportar los módulos principales del proyecto
pub mod common;
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;

// Re-exportaciones públicas comunes
pub use application::services::folder_service::FolderService;
pub use application::services::file_service::FileService;
pub use application::services::i18n_application_service::I18nApplicationService;
pub use application::services::storage_mediator::{StorageMediator, FileSystemStorageMediator};
pub use infrastructure::services::path_service::PathService;
pub use domain::services::path_service::StoragePath;
pub use infrastructure::repositories::folder_fs_repository::FolderFsRepository;
pub use infrastructure::repositories::file_fs_repository::FileFsRepository;
pub use infrastructure::repositories::parallel_file_processor::ParallelFileProcessor;
pub use infrastructure::services::buffer_pool::BufferPool;
pub use infrastructure::services::compression_service::GzipCompressionService;