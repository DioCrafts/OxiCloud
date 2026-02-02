use std::sync::Arc;
use async_trait::async_trait;

use crate::application::dtos::file_dto::FileDto;
use crate::application::ports::file_ports::FileManagementUseCase;
use crate::application::ports::storage_ports::FileWritePort;
use crate::common::errors::DomainError;

/// Service for file management operations
pub struct FileManagementService {
    file_repository: Arc<dyn FileWritePort>,
}

impl FileManagementService {
    /// Creates a new file management service
    pub fn new(file_repository: Arc<dyn FileWritePort>) -> Self {
        Self { file_repository }
    }
}

#[async_trait]
impl FileManagementUseCase for FileManagementService {
    async fn move_file(&self, file_id: &str, folder_id: Option<String>) -> Result<FileDto, DomainError> {
        tracing::info!("Moving file with ID: {} to folder: {:?}", file_id, folder_id);
        
        let moved_file = self.file_repository.move_file(file_id, folder_id).await
            .map_err(|e| {
                tracing::error!("Error moving file (ID: {}): {}", file_id, e);
                e
            })?;
        
        tracing::info!("File moved successfully: {} (ID: {}) to folder: {:?}", 
                       moved_file.name(), moved_file.id(), moved_file.folder_id());
        
        Ok(FileDto::from(moved_file))
    }
    
    async fn delete_file(&self, id: &str) -> Result<(), DomainError> {
        self.file_repository.delete_file(id).await
    }
}