use async_trait::async_trait;

use crate::application::dtos::folder_dto::{CreateFolderDto, FolderDto, MoveFolderDto, RenameFolderDto};
use crate::application::dtos::search_dto::{SearchCriteriaDto, SearchResultsDto};
use crate::common::errors::DomainError;

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

/**
 * Puerto primario para búsqueda de archivos y carpetas
 * 
 * Define las operaciones relacionadas con la búsqueda avanzada de
 * archivos y carpetas basándose en diversos criterios.
 */
#[async_trait]
pub trait SearchUseCase: Send + Sync + 'static {
    /**
     * Realiza una búsqueda basada en los criterios especificados
     * 
     * @param criteria Criterios de búsqueda que incluyen texto, fechas, tamaños, etc.
     * @return Resultados de la búsqueda que contienen archivos y carpetas coincidentes
     */
    async fn search(&self, criteria: SearchCriteriaDto) -> Result<SearchResultsDto, DomainError>;
    
    /**
     * Limpia la caché de resultados de búsqueda
     * 
     * @return Resultado indicando éxito o error
     */
    async fn clear_search_cache(&self) -> Result<(), DomainError>;
}