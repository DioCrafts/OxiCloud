use crate::application::dtos::folder_dto::{
    CreateFolderDto, FolderDto, MoveFolderDto, RenameFolderDto,
};
use crate::application::ports::inbound::FolderUseCase;
use crate::application::ports::outbound::FolderStoragePort;
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::services::path_service::StoragePath;
use async_trait::async_trait;
use std::sync::Arc;

/// Implementation of the use case for folder operations
pub struct FolderService {
    folder_storage: Arc<dyn FolderStoragePort>,
}

impl FolderService {
    /// Creates a new folder service
    pub fn new(folder_storage: Arc<dyn FolderStoragePort>) -> Self {
        Self { folder_storage }
    }

    /// Creates a stub implementation for testing and middleware
    pub fn new_stub() -> impl FolderUseCase {
        struct FolderServiceStub;

        #[async_trait]
        impl FolderUseCase for FolderServiceStub {
            async fn create_folder(&self, _dto: CreateFolderDto) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn get_folder(&self, _id: &str) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn get_folder_by_path(&self, _path: &str) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn list_folders(
                &self,
                _parent_id: Option<&str>,
            ) -> Result<Vec<FolderDto>, DomainError> {
                Ok(vec![])
            }

            async fn list_folders_for_owner(
                &self,
                _parent_id: Option<&str>,
                _owner_id: &str,
            ) -> Result<Vec<FolderDto>, DomainError> {
                Ok(vec![])
            }

            async fn list_folders_paginated(
                &self,
                _parent_id: Option<&str>,
                _pagination: &crate::application::dtos::pagination::PaginationRequestDto,
            ) -> Result<
                crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>,
                DomainError,
            > {
                Ok(
                    crate::application::dtos::pagination::PaginatedResponseDto::new(
                        vec![],
                        0,
                        10,
                        0,
                    ),
                )
            }

            async fn list_folders_for_owner_paginated(
                &self,
                _parent_id: Option<&str>,
                _owner_id: &str,
                _pagination: &crate::application::dtos::pagination::PaginationRequestDto,
            ) -> Result<
                crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>,
                DomainError,
            > {
                Ok(
                    crate::application::dtos::pagination::PaginatedResponseDto::new(
                        vec![],
                        0,
                        10,
                        0,
                    ),
                )
            }

            async fn rename_folder(
                &self,
                _id: &str,
                _dto: RenameFolderDto,
                _caller_id: &str,
            ) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn move_folder(
                &self,
                _id: &str,
                _dto: MoveFolderDto,
                _caller_id: &str,
            ) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn delete_folder(&self, _id: &str, _caller_id: &str) -> Result<(), DomainError> {
                Ok(())
            }

            async fn create_home_folder(
                &self,
                _user_id: &str,
                _name: String,
            ) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }
        }

        FolderServiceStub
    }
}

#[async_trait]
impl FolderUseCase for FolderService {
    /// Creates a new folder
    async fn create_folder(&self, dto: CreateFolderDto) -> Result<FolderDto, DomainError> {
        // Input validation
        if dto.name.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Folder",
                "Folder name cannot be empty",
            ));
        }

        // If a parent_id is provided, verify it exists
        if let Some(parent_id) = &dto.parent_id {
            let parent_exists = self.folder_storage.get_folder(parent_id).await.is_ok();
            if !parent_exists {
                return Err(DomainError::not_found("Folder", parent_id));
            }
        }

        // Create the folder
        let folder = self
            .folder_storage
            .create_folder(dto.name, dto.parent_id)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!("Failed to create folder: {}", e),
                )
            })?;

        // Convert to DTO
        Ok(FolderDto::from(folder))
    }

    /// Creates a root-level home folder for a user during registration.
    async fn create_home_folder(
        &self,
        user_id: &str,
        name: String,
    ) -> Result<FolderDto, DomainError> {
        let folder = self
            .folder_storage
            .create_home_folder(user_id, name)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!("Failed to create home folder: {}", e),
                )
            })?;

        Ok(FolderDto::from(folder))
    }

    async fn list_subtree_folders(&self, folder_id: &str) -> Result<Vec<FolderDto>, DomainError> {
        let folders = self.folder_storage.list_subtree_folders(folder_id).await?;
        Ok(folders.into_iter().map(FolderDto::from).collect())
    }

    /// Gets a folder by its ID
    async fn get_folder(&self, id: &str) -> Result<FolderDto, DomainError> {
        let folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get folder with ID: {}: {}", id, e),
            )
        })?;

        Ok(FolderDto::from(folder))
    }

    /// Gets a folder by its path
    async fn get_folder_by_path(&self, path: &str) -> Result<FolderDto, DomainError> {
        // Convert the string path to StoragePath
        let storage_path = StoragePath::from_string(path);

        let folder = self
            .folder_storage
            .get_folder_by_path(&storage_path)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!("Failed to get folder at path: {}: {}", path, e),
                )
            })?;

        Ok(FolderDto::from(folder))
    }

    /// Lists folders within a parent folder
    async fn list_folders(&self, parent_id: Option<&str>) -> Result<Vec<FolderDto>, DomainError> {
        let folders = self
            .folder_storage
            .list_folders(parent_id)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!("Failed to list folders in parent: {:?}: {}", parent_id, e),
                )
            })?;

        // Convert to DTOs
        Ok(folders.into_iter().map(FolderDto::from).collect())
    }

    /// Lists folders scoped to a specific owner.
    /// Self-healing: if listing root folders and none exist, creates a home folder.
    async fn list_folders_for_owner(
        &self,
        parent_id: Option<&str>,
        owner_id: &str,
    ) -> Result<Vec<FolderDto>, DomainError> {
        let folders = self
            .folder_storage
            .list_folders_by_owner(parent_id, owner_id)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!(
                        "Failed to list folders for owner '{}' in parent {:?}: {}",
                        owner_id, parent_id, e
                    ),
                )
            })?;

        // Self-healing: if listing root folders and none exist, create a home folder
        // This ensures the frontend always gets a valid userHomeFolderId
        if parent_id.is_none() && folders.is_empty() {
            tracing::info!(
                "No root folders found for user {}, creating home folder automatically",
                owner_id
            );
            let folder_name = format!("My Folder - {}", &owner_id[..8.min(owner_id.len())]);
            match self
                .folder_storage
                .create_home_folder(owner_id, folder_name.clone())
                .await
            {
                Ok(home_folder) => {
                    tracing::info!(
                        "Created home folder '{}' for user {}",
                        folder_name,
                        owner_id
                    );
                    return Ok(vec![FolderDto::from(home_folder)]);
                }
                Err(e) => {
                    tracing::warn!("Failed to create home folder for user {}: {}", owner_id, e);
                    // Return empty list rather than failing - user might not have storage quota, etc.
                }
            }
        }

        Ok(folders.into_iter().map(FolderDto::from).collect())
    }

    /// Lists folders with pagination
    async fn list_folders_paginated(
        &self,
        parent_id: Option<&str>,
        pagination: &crate::application::dtos::pagination::PaginationRequestDto,
    ) -> Result<crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>, DomainError>
    {
        let pagination = pagination.validate_and_adjust();

        let (folders, total_items) = self
            .folder_storage
            .list_folders_paginated(parent_id, pagination.offset(), pagination.limit(), true)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!(
                        "Failed to list folders with pagination in parent: {:?}: {}",
                        parent_id, e
                    ),
                )
            })?;

        let total = total_items.unwrap_or(folders.len());

        let response = crate::application::dtos::pagination::PaginatedResponseDto::new(
            folders.into_iter().map(FolderDto::from).collect(),
            pagination.page,
            pagination.page_size,
            total,
        );

        Ok(response)
    }

    /// Lists folders with pagination, scoped to a specific owner.
    async fn list_folders_for_owner_paginated(
        &self,
        parent_id: Option<&str>,
        owner_id: &str,
        pagination: &crate::application::dtos::pagination::PaginationRequestDto,
    ) -> Result<crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>, DomainError>
    {
        let pagination = pagination.validate_and_adjust();

        let (folders, total_items) = self
            .folder_storage
            .list_folders_by_owner_paginated(
                parent_id,
                owner_id,
                pagination.offset(),
                pagination.limit(),
                true,
            )
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!(
                        "Failed to list folders for owner '{}' with pagination in parent {:?}: {}",
                        owner_id, parent_id, e
                    ),
                )
            })?;

        let total = total_items.unwrap_or(folders.len());

        let response = crate::application::dtos::pagination::PaginatedResponseDto::new(
            folders.into_iter().map(FolderDto::from).collect(),
            pagination.page,
            pagination.page_size,
            total,
        );

        Ok(response)
    }

    /// Renames a folder after verifying ownership.
    async fn rename_folder(
        &self,
        id: &str,
        dto: RenameFolderDto,
        caller_id: &str,
    ) -> Result<FolderDto, DomainError> {
        // Input validation
        if dto.name.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Folder",
                "New folder name cannot be empty",
            ));
        }

        // Verify the folder exists and belongs to the caller
        let existing_folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get folder with ID: {} for renaming: {}", id, e),
            )
        })?;

        if existing_folder.owner_id() != Some(caller_id) {
            tracing::warn!(
                "rename_folder: user '{}' attempted to rename folder '{}' owned by '{:?}'",
                caller_id,
                id,
                existing_folder.owner_id()
            );
            return Err(DomainError::not_found("Folder", id));
        }

        // Rename folder — UPDATE RETURNING gives us the updated row directly
        let folder = self.folder_storage.rename_folder(id, dto.name).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to rename folder with ID: {}: {}", id, e),
            )
        })?;

        Ok(FolderDto::from(folder))
    }

    /// Moves a folder to a new parent after verifying ownership.
    async fn move_folder(
        &self,
        id: &str,
        dto: MoveFolderDto,
        caller_id: &str,
    ) -> Result<FolderDto, DomainError> {
        // Verify the source folder exists and belongs to the caller
        let source_folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get folder with ID: {} for moving: {}", id, e),
            )
        })?;

        if source_folder.owner_id() != Some(caller_id) {
            tracing::warn!(
                "move_folder: user '{}' attempted to move folder '{}' owned by '{:?}'",
                caller_id,
                id,
                source_folder.owner_id()
            );
            return Err(DomainError::not_found("Folder", id));
        }

        // If a parent_id is specified, verify it exists
        if let Some(parent_id) = &dto.parent_id {
            // Verify we are not trying to move the folder into itself or one of its descendants
            if parent_id == id {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Folder",
                    "Cannot move a folder into itself",
                ));
            }

            // Verify the destination exists
            let parent_exists = self.folder_storage.get_folder(parent_id).await.is_ok();
            if !parent_exists {
                return Err(DomainError::not_found("Folder", parent_id));
            }

            // TODO: Ideally we should verify the entire hierarchy to prevent cycles
        }

        // Move folder — UPDATE RETURNING gives us the updated row directly
        let parent_ref = dto.parent_id.as_deref();
        let folder = self.folder_storage.move_folder(id, parent_ref).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to move folder with ID: {}: {}", id, e),
            )
        })?;

        Ok(FolderDto::from(folder))
    }

    /// Deletes a folder after verifying ownership.
    async fn delete_folder(&self, id: &str, caller_id: &str) -> Result<(), DomainError> {
        // Verify the folder exists and belongs to the caller
        let folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get folder with ID: {} for deletion: {}", id, e),
            )
        })?;

        if folder.owner_id() != Some(caller_id) {
            tracing::warn!(
                "delete_folder: user '{}' attempted to delete folder '{}' owned by '{:?}'",
                caller_id,
                id,
                folder.owner_id()
            );
            return Err(DomainError::not_found("Folder", id));
        }

        // Delete the folder
        self.folder_storage.delete_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to delete folder with ID: {}: {}", id, e),
            )
        })
    }
}
