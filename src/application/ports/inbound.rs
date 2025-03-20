use std::sync::Arc;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use chrono::{DateTime, Utc};

use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::{CreateFolderDto, FolderDto, MoveFolderDto, RenameFolderDto};
use crate::application::dtos::shared_file_dto::{SharedFileDto, UserAccessDto};
use crate::application::dtos::public_link_dto::{PublicLinkDto, PublicFileAccessDto};
use crate::common::errors::DomainError;

/// Puerto primario para operaciones de archivos
#[async_trait]
pub trait FileUseCase: Send + Sync + 'static {
    /// Sube un nuevo archivo desde bytes
    async fn upload_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
        user_id: Option<String>,
    ) -> Result<FileDto, DomainError>;
    
    /// Obtiene un archivo por su ID
    async fn get_file(&self, id: &str) -> Result<FileDto, DomainError>;
    
    /// Lista archivos en una carpeta
    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<FileDto>, DomainError>;
    
    /// Elimina un archivo
    async fn delete_file(&self, id: &str) -> Result<(), DomainError>;
    
    /// Obtiene contenido de archivo como bytes (para archivos pequeños)
    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError>;
    
    /// Obtiene contenido de archivo como stream (para archivos grandes)
    async fn get_file_stream(&self, id: &str) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError>;
    
    /// Mueve un archivo a otra carpeta
    async fn move_file(&self, file_id: &str, folder_id: Option<String>) -> Result<FileDto, DomainError>;
}

/// Puerto primario para operaciones de carpetas
#[async_trait]
pub trait FolderUseCase: Send + Sync + 'static {
    /// Crea una nueva carpeta
    async fn create_folder(&self, dto: CreateFolderDto) -> Result<FolderDto, DomainError>;
    
    /// Obtiene una carpeta por su ID
    async fn get_folder(&self, id: &str) -> Result<FolderDto, DomainError>;
    
    /// Obtiene una carpeta por su ruta
    async fn get_folder_by_path(&self, path: &str) -> Result<FolderDto, DomainError>;
    
    /// Lista carpetas dentro de una carpeta padre
    async fn list_folders(&self, parent_id: Option<&str>) -> Result<Vec<FolderDto>, DomainError>;
    
    /// Lista carpetas con paginación
    async fn list_folders_paginated(
        &self, 
        parent_id: Option<&str>,
        pagination: &crate::application::dtos::pagination::PaginationRequestDto
    ) -> Result<crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>, DomainError>;
    
    /// Renombra una carpeta
    async fn rename_folder(&self, id: &str, dto: RenameFolderDto) -> Result<FolderDto, DomainError>;
    
    /// Mueve una carpeta a otro padre
    async fn move_folder(&self, id: &str, dto: MoveFolderDto) -> Result<FolderDto, DomainError>;
    
    /// Elimina una carpeta
    async fn delete_folder(&self, id: &str) -> Result<(), DomainError>;
}

/// Puerto primario para operaciones de compartición con usuarios
#[async_trait]
pub trait SharedFileUseCase: Send + Sync + 'static {
    /// Comparte un archivo con un usuario
    async fn share_file_with_user(
        &self,
        file_id: String,
        owner_id: String,
        user_id: String,
        permission: String,
    ) -> Result<SharedFileDto, DomainError>;
    
    /// Actualiza el nivel de permiso de un archivo compartido
    async fn update_permission(
        &self,
        file_id: String,
        owner_id: String,
        user_id: String,
        permission: String,
    ) -> Result<SharedFileDto, DomainError>;
    
    /// Deja de compartir un archivo con un usuario
    async fn unshare_file(
        &self,
        file_id: String,
        owner_id: String,
        user_id: String,
    ) -> Result<(), DomainError>;
    
    /// Obtiene todos los archivos compartidos con un usuario
    async fn get_files_shared_with_user(
        &self,
        user_id: String,
    ) -> Result<Vec<SharedFileDto>, DomainError>;
    
    /// Obtiene todos los usuarios con acceso a un archivo
    async fn get_users_with_access(
        &self,
        file_id: String,
        owner_id: String,
    ) -> Result<Vec<UserAccessDto>, DomainError>;
    
    /// Obtiene todos los archivos que un usuario ha compartido
    async fn get_files_shared_by_user(
        &self,
        owner_id: String,
    ) -> Result<Vec<SharedFileDto>, DomainError>;
    
    /// Verifica si un usuario tiene acceso a un archivo
    async fn check_user_has_access(
        &self,
        file_id: String,
        user_id: String,
    ) -> Result<Option<String>, DomainError>;
}

/// Puerto primario para operaciones con enlaces públicos
#[async_trait]
pub trait PublicLinkUseCase: Send + Sync + 'static {
    /// Crea un nuevo enlace público para un archivo
    async fn create_public_link(
        &self,
        file_id: String,
        owner_id: String,
        permission: String,
        password: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<PublicLinkDto, DomainError>;
    
    /// Obtiene información de un enlace público
    async fn get_public_link(
        &self,
        link_id: String,
        owner_id: Option<String>,
    ) -> Result<PublicLinkDto, DomainError>;
    
    /// Actualiza los permisos de un enlace público
    async fn update_link_permission(
        &self,
        link_id: String,
        owner_id: String,
        permission: String,
    ) -> Result<PublicLinkDto, DomainError>;
    
    /// Actualiza la contraseña de un enlace público
    async fn update_link_password(
        &self,
        link_id: String,
        owner_id: String,
        password: Option<String>,
    ) -> Result<PublicLinkDto, DomainError>;
    
    /// Actualiza la fecha de caducidad de un enlace público
    async fn update_link_expiration(
        &self,
        link_id: String,
        owner_id: String,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<PublicLinkDto, DomainError>;
    
    /// Elimina un enlace público
    async fn delete_public_link(
        &self,
        link_id: String,
        owner_id: String,
    ) -> Result<(), DomainError>;
    
    /// Obtiene todos los enlaces públicos para un archivo
    async fn get_links_for_file(
        &self,
        file_id: String,
        owner_id: String,
    ) -> Result<Vec<PublicLinkDto>, DomainError>;
    
    /// Obtiene todos los enlaces públicos creados por un usuario
    async fn get_links_by_user(
        &self,
        owner_id: String,
    ) -> Result<Vec<PublicLinkDto>, DomainError>;
    
    /// Accede a un archivo a través de un enlace público
    async fn access_public_file(
        &self,
        link_id: String,
        password: Option<String>,
    ) -> Result<PublicFileAccessDto, DomainError>;
}

/// Factory para crear implementaciones de casos de uso
pub trait UseCaseFactory {
    fn create_file_use_case(&self) -> Arc<dyn FileUseCase>;
    fn create_folder_use_case(&self) -> Arc<dyn FolderUseCase>;
    fn create_shared_file_use_case(&self) -> Arc<dyn SharedFileUseCase>;
    fn create_public_link_use_case(&self) -> Arc<dyn PublicLinkUseCase>;
}