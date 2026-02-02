use std::sync::Arc;

use crate::application::ports::file_ports::{FileUploadUseCase, FileRetrievalUseCase, FileManagementUseCase, FileUseCaseFactory};
use crate::application::services::file_upload_service::FileUploadService;
use crate::application::services::file_retrieval_service::FileRetrievalService;
use crate::application::services::file_management_service::FileManagementService;
use crate::application::ports::storage_ports::{FileReadPort, FileWritePort};

/// Factory para crear implementaciones de casos de uso de archivos
pub struct AppFileUseCaseFactory {
    file_read_repository: Arc<dyn FileReadPort>,
    file_write_repository: Arc<dyn FileWritePort>,
}

impl AppFileUseCaseFactory {
    /// Crea una nueva factory para casos de uso de archivos
    pub fn new(
        file_read_repository: Arc<dyn FileReadPort>,
        file_write_repository: Arc<dyn FileWritePort>
    ) -> Self {
        Self {
            file_read_repository,
            file_write_repository,
        }
    }
}

impl FileUseCaseFactory for AppFileUseCaseFactory {
    fn create_file_upload_use_case(&self) -> Arc<dyn FileUploadUseCase> {
        Arc::new(FileUploadService::new(self.file_write_repository.clone()))
    }
    
    fn create_file_retrieval_use_case(&self) -> Arc<dyn FileRetrievalUseCase> {
        Arc::new(FileRetrievalService::new(self.file_read_repository.clone()))
    }
    
    fn create_file_management_use_case(&self) -> Arc<dyn FileManagementUseCase> {
        Arc::new(FileManagementService::new(self.file_write_repository.clone()))
    }
}