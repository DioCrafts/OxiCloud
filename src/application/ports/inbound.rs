use async_trait::async_trait;

use crate::application::dtos::folder_dto::{
    CreateFolderDto, FolderDto, MoveFolderDto, RenameFolderDto,
};
use crate::application::dtos::search_dto::{
    SearchCriteriaDto, SearchResultsDto, SearchSuggestionsDto,
};
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

    /// Lists folders scoped to a specific owner (for user-facing endpoints).
    /// At root level, only returns folders belonging to this user.
    async fn list_folders_for_owner(
        &self,
        parent_id: Option<&str>,
        owner_id: &str,
    ) -> Result<Vec<FolderDto>, DomainError>;

    /// Lists folders with pagination
    async fn list_folders_paginated(
        &self,
        parent_id: Option<&str>,
        pagination: &crate::application::dtos::pagination::PaginationRequestDto,
    ) -> Result<crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>, DomainError>;

    /// Lists folders with pagination, scoped to a specific owner.
    async fn list_folders_for_owner_paginated(
        &self,
        parent_id: Option<&str>,
        owner_id: &str,
        pagination: &crate::application::dtos::pagination::PaginationRequestDto,
    ) -> Result<crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>, DomainError>;

    /// Renames a folder (ownership verified against caller_id)
    async fn rename_folder(&self, id: &str, dto: RenameFolderDto, caller_id: &str)
    -> Result<FolderDto, DomainError>;

    /// Moves a folder to another parent (ownership verified against caller_id)
    async fn move_folder(&self, id: &str, dto: MoveFolderDto, caller_id: &str) -> Result<FolderDto, DomainError>;

    /// Deletes a folder (ownership verified against caller_id)
    async fn delete_folder(&self, id: &str, caller_id: &str) -> Result<(), DomainError>;

    /// Creates a root-level home folder for a user during registration.
    async fn create_home_folder(&self, user_id: &str, name: String) -> Result<FolderDto, DomainError>;
}

/**
 * Primary port for file and folder search.
 *
 * All search processing (filtering, scoring, sorting, categorization)
 * is handled server-side in Rust for maximum efficiency.
 */
#[async_trait]
pub trait SearchUseCase: Send + Sync + 'static {
    /// Performs a full search based on the specified criteria.
    async fn search(&self, criteria: SearchCriteriaDto) -> Result<SearchResultsDto, DomainError>;

    /// Returns quick suggestions for autocomplete (lightweight, fast).
    async fn suggest(
        &self,
        query: &str,
        folder_id: Option<&str>,
        limit: usize,
    ) -> Result<SearchSuggestionsDto, DomainError>;

    /// Clears the search results cache.
    async fn clear_search_cache(&self) -> Result<(), DomainError>;
}
