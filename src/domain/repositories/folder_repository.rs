//! Puerto de persistencia del dominio para la entidad Folder.
//!
//! Define el contrato que cualquier implementación de almacenamiento de carpetas
//! debe cumplir. Este trait vive en el dominio porque Folder es una entidad core
//! del sistema y sus contratos de persistencia pertenecen a la capa de dominio,
//! siguiendo los principios de Clean/Hexagonal Architecture.
//!
//! Las implementaciones concretas (filesystem, PostgreSQL, S3, etc.) viven en
//! la capa de infraestructura.

use async_trait::async_trait;

use crate::domain::entities::folder::Folder;
use crate::domain::services::path_service::StoragePath;
use crate::common::errors::DomainError;

/// Puerto del dominio para persistencia de carpetas.
///
/// Define las operaciones CRUD y de gestión necesarias para
/// la entidad Folder en el sistema de almacenamiento.
#[async_trait]
pub trait FolderRepository: Send + Sync + 'static {
    /// Crea una nueva carpeta
    async fn create_folder(&self, name: String, parent_id: Option<String>) -> Result<Folder, DomainError>;
    
    /// Obtiene una carpeta por su ID
    async fn get_folder(&self, id: &str) -> Result<Folder, DomainError>;
    
    /// Obtiene una carpeta por su ruta de almacenamiento
    async fn get_folder_by_path(&self, storage_path: &StoragePath) -> Result<Folder, DomainError>;
    
    /// Lista carpetas dentro de una carpeta padre
    async fn list_folders(&self, parent_id: Option<&str>) -> Result<Vec<Folder>, DomainError>;
    
    /// Lista carpetas con paginación
    async fn list_folders_paginated(
        &self, 
        parent_id: Option<&str>,
        offset: usize,
        limit: usize,
        include_total: bool
    ) -> Result<(Vec<Folder>, Option<usize>), DomainError>;
    
    /// Renombra una carpeta
    async fn rename_folder(&self, id: &str, new_name: String) -> Result<Folder, DomainError>;
    
    /// Mueve una carpeta a otro padre
    async fn move_folder(&self, id: &str, new_parent_id: Option<&str>) -> Result<Folder, DomainError>;
    
    /// Elimina una carpeta
    async fn delete_folder(&self, id: &str) -> Result<(), DomainError>;
    
    /// Verifica si existe una carpeta en la ruta dada
    async fn folder_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError>;
    
    /// Obtiene la ruta de una carpeta
    async fn get_folder_path(&self, id: &str) -> Result<StoragePath, DomainError>;

    // ── Trash operations ──

    /// Mueve una carpeta a la papelera
    async fn move_to_trash(&self, folder_id: &str) -> Result<(), DomainError>;

    /// Restaura una carpeta desde la papelera a su ubicación original
    async fn restore_from_trash(&self, folder_id: &str, original_path: &str) -> Result<(), DomainError>;

    /// Elimina una carpeta permanentemente (usado por la papelera)
    async fn delete_folder_permanently(&self, folder_id: &str) -> Result<(), DomainError>;
}
