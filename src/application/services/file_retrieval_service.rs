use std::sync::Arc;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

use crate::application::dtos::file_dto::FileDto;
use crate::application::ports::file_ports::FileRetrievalUseCase;
use crate::application::ports::storage_ports::FileReadPort;
use crate::common::errors::DomainError;

/// Servicio para operaciones de recuperación de archivos
pub struct FileRetrievalService {
    file_repository: Arc<dyn FileReadPort>,
}

impl FileRetrievalService {
    /// Crea un nuevo servicio de recuperación de archivos
    pub fn new(file_repository: Arc<dyn FileReadPort>) -> Self {
        Self { file_repository }
    }
}

#[async_trait]
impl FileRetrievalUseCase for FileRetrievalService {
    async fn get_file(&self, id: &str) -> Result<FileDto, DomainError> {
        let file = self.file_repository.get_file(id).await?;
        Ok(FileDto::from(file))
    }
    
    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<FileDto>, DomainError> {
        let files = self.file_repository.list_files(folder_id).await?;
        Ok(files.into_iter().map(FileDto::from).collect())
    }
    
    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError> {
        self.file_repository.get_file_content(id).await
    }
    
    async fn get_file_stream(&self, id: &str) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        self.file_repository.get_file_stream(id).await
    }
}