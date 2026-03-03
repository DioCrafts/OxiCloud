use std::sync::Arc;

use crate::application::ports::file_ports::FileUseCaseFactory;
use crate::application::services::file_management_service::FileManagementService;
use crate::application::services::file_retrieval_service::FileRetrievalService;
use crate::application::services::file_upload_service::FileUploadService;
use crate::infrastructure::repositories::pg::file_blob_read_repository::FileBlobReadRepository;
use crate::infrastructure::repositories::pg::file_blob_write_repository::FileBlobWriteRepository;

/// Factory for creating file use case implementations
pub struct AppFileUseCaseFactory {
    file_read_repository: Arc<FileBlobReadRepository>,
    file_write_repository: Arc<FileBlobWriteRepository>,
}

impl AppFileUseCaseFactory {
    /// Creates a new factory for file use cases
    pub fn new(
        file_read_repository: Arc<FileBlobReadRepository>,
        file_write_repository: Arc<FileBlobWriteRepository>,
    ) -> Self {
        Self {
            file_read_repository,
            file_write_repository,
        }
    }
}

impl FileUseCaseFactory for AppFileUseCaseFactory {
    fn create_file_upload_use_case(&self) -> Arc<FileUploadService> {
        Arc::new(FileUploadService::new(self.file_write_repository.clone()))
    }

    fn create_file_retrieval_use_case(&self) -> Arc<FileRetrievalService> {
        Arc::new(FileRetrievalService::new(self.file_read_repository.clone()))
    }

    fn create_file_management_use_case(&self) -> Arc<FileManagementService> {
        Arc::new(FileManagementService::new(
            self.file_write_repository.clone(),
        ))
    }
}
