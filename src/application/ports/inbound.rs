use async_trait::async_trait;

use crate::application::dtos::folder_dto::{
    CreateFolderDto, FolderDto, MoveFolderDto, RenameFolderDto,
};
use crate::application::dtos::search_dto::{SearchCriteriaDto, SearchResultsDto};
use crate::common::errors::DomainError;

/// Primary port for folder operations
#[async_trait]
pub trait FolderUseCase: Send + Sync + 'static {
    /// Creates a new folder
    async fn create_folder(&self, dto: CreateFolderDto) -> Result<FolderDto, DomainError>;

    /// Gets a folder by its ID
    async fn get_folder(&self, id: &str) -> Result<FolderDto, DomainError>;

    /// Gets a folder by its path
    async fn get_folder_by_path(&self, path: &str) -> Result<FolderDto, DomainError>;

    /// Lists folders within a parent folder
    async fn list_folders(&self, parent_id: Option<&str>) -> Result<Vec<FolderDto>, DomainError>;

    /// Lists folders with pagination
    async fn list_folders_paginated(
        &self,
        parent_id: Option<&str>,
        pagination: &crate::application::dtos::pagination::PaginationRequestDto,
    ) -> Result<crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>, DomainError>;

    /// Renames a folder
    async fn rename_folder(&self, id: &str, dto: RenameFolderDto)
    -> Result<FolderDto, DomainError>;

    /// Moves a folder to another parent
    async fn move_folder(&self, id: &str, dto: MoveFolderDto) -> Result<FolderDto, DomainError>;

    /// Deletes a folder
    async fn delete_folder(&self, id: &str) -> Result<(), DomainError>;
}

/**
 * Primary port for file and folder search
 *
 * Defines the operations related to advanced search of
 * files and folders based on various criteria.
 */
#[async_trait]
pub trait SearchUseCase: Send + Sync + 'static {
    /**
     * Performs a search based on the specified criteria
     *
     * @param criteria Search criteria including text, dates, sizes, etc.
     * @return Search results containing matching files and folders
     */
    async fn search(&self, criteria: SearchCriteriaDto) -> Result<SearchResultsDto, DomainError>;

    /**
     * Clears the search results cache
     *
     * @return Result indicating success or error
     */
    async fn clear_search_cache(&self) -> Result<(), DomainError>;
}
