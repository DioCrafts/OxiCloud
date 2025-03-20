use std::sync::Arc;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

use crate::application::dtos::file_dto::FileDto;
use crate::common::errors::DomainError;

/// Puerto primario para operaciones de subida de archivos
#[async_trait]
pub trait FileUploadUseCase: Send + Sync + 'static {
    /// Sube un nuevo archivo desde bytes
    async fn upload_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
        user_id: Option<String>,
    ) -> Result<FileDto, DomainError>;
}

/// Puerto primario para operaciones de recuperación de archivos
#[async_trait]
pub trait FileRetrievalUseCase: Send + Sync + 'static {
    /// Obtiene un archivo por su ID
    async fn get_file(&self, id: &str) -> Result<FileDto, DomainError>;
    
    /// Lista archivos en una carpeta
    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<FileDto>, DomainError>;
    
    /// Obtiene contenido de archivo como bytes (para archivos pequeños)
    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError>;
    
    /// Obtiene contenido de archivo como stream (para archivos grandes)
    async fn get_file_stream(&self, id: &str) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError>;
}

/// Puerto primario para operaciones de gestión de archivos
#[async_trait]
pub trait FileManagementUseCase: Send + Sync + 'static {
    /// Mueve un archivo a otra carpeta
    async fn move_file(&self, file_id: &str, folder_id: Option<String>) -> Result<FileDto, DomainError>;
    
    /// Elimina un archivo
    async fn delete_file(&self, id: &str) -> Result<(), DomainError>;
}

/// Factory para crear implementaciones de casos de uso de archivos
pub trait FileUseCaseFactory: Send + Sync + 'static {
    fn create_file_upload_use_case(&self) -> Arc<dyn FileUploadUseCase>;
    fn create_file_retrieval_use_case(&self) -> Arc<dyn FileRetrievalUseCase>;
    fn create_file_management_use_case(&self) -> Arc<dyn FileManagementUseCase>;
}