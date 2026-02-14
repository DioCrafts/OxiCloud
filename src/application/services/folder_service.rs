use crate::application::dtos::folder_dto::{
    CreateFolderDto, FolderDto, MoveFolderDto, RenameFolderDto,
};
use crate::application::ports::inbound::FolderUseCase;
use crate::application::ports::outbound::FolderStoragePort;
use crate::application::transactions::storage_transaction::StorageTransaction;
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

            async fn rename_folder(
                &self,
                _id: &str,
                _dto: RenameFolderDto,
            ) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn move_folder(
                &self,
                _id: &str,
                _dto: MoveFolderDto,
            ) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn delete_folder(&self, _id: &str) -> Result<(), DomainError> {
                Ok(())
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

    /// Lists folders with pagination
    async fn list_folders_paginated(
        &self,
        parent_id: Option<&str>,
        pagination: &crate::application::dtos::pagination::PaginationRequestDto,
    ) -> Result<crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>, DomainError>
    {
        // Validate and adjust pagination
        let pagination = pagination.validate_and_adjust();

        // Get paginated folders and total count
        let (folders, total_items) = self
            .folder_storage
            .list_folders_paginated(
                parent_id,
                pagination.offset(),
                pagination.limit(),
                true, // Always include total for better UX
            )
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

        // The total is needed to calculate pagination
        let total = total_items.unwrap_or(folders.len());

        // Convert to PaginatedResponseDto
        let response = crate::application::dtos::pagination::PaginatedResponseDto::new(
            folders.into_iter().map(FolderDto::from).collect(),
            pagination.page,
            pagination.page_size,
            total,
        );

        Ok(response)
    }

    /// Renames a folder
    async fn rename_folder(
        &self,
        id: &str,
        dto: RenameFolderDto,
    ) -> Result<FolderDto, DomainError> {
        // Input validation
        if dto.name.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Folder",
                "New folder name cannot be empty",
            ));
        }

        // Verify the folder exists
        let existing_folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get folder with ID: {} for renaming: {}", id, e),
            )
        })?;

        // Create transaction for renaming
        let mut transaction = StorageTransaction::new("rename_folder");

        // Main operation: rename folder
        // Clone all values to avoid lifetime issues
        let folder_storage = self.folder_storage.clone();
        let id_owned = id.to_string();
        let name_owned = dto.name.clone();

        // Create future with owned values
        let rename_op = async move {
            folder_storage.rename_folder(&id_owned, name_owned).await?;
            Ok(())
        };
        let rollback_op = {
            let original_name = existing_folder.name().to_string();
            let storage = self.folder_storage.clone();
            let id_clone = id.to_string();

            async move {
                // In case of failure, restore the original name
                storage
                    .rename_folder(&id_clone, original_name)
                    .await
                    .map(|_| ())
                    .map_err(|e| {
                        DomainError::new(
                            ErrorKind::InternalError,
                            "Folder",
                            format!("Failed to rollback folder rename: {}", e),
                        )
                    })
            }
        };

        // Add to the transaction
        transaction.add_operation(rename_op, rollback_op);

        // Execute transaction
        transaction.commit().await?;

        // Get the renamed folder
        let folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get renamed folder with ID: {}: {}", id, e),
            )
        })?;

        Ok(FolderDto::from(folder))
    }

    /// Moves a folder to a new parent
    async fn move_folder(&self, id: &str, dto: MoveFolderDto) -> Result<FolderDto, DomainError> {
        // Verify the source folder exists
        let source_folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get folder with ID: {} for moving: {}", id, e),
            )
        })?;

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

        // Create transaction for moving
        let mut transaction = StorageTransaction::new("move_folder");

        // Main operation: move folder
        // Clone all values to avoid lifetime issues
        let folder_storage = self.folder_storage.clone();
        let id_owned = id.to_string();
        // Get parent ID as owned string or None
        let parent_id_owned = dto.parent_id.as_ref().map(|p| p.to_string());

        // Create future with owned values
        let move_op = async move {
            // Convert Option<String> to Option<&str>
            let parent_ref = parent_id_owned.as_deref();
            folder_storage.move_folder(&id_owned, parent_ref).await?;
            Ok(())
        };
        let rollback_op = {
            let original_parent_id = source_folder.parent_id().map(String::from);
            let storage = self.folder_storage.clone();
            let id_clone = id.to_string();

            async move {
                // In case of failure, restore the original location
                storage
                    .move_folder(&id_clone, original_parent_id.as_deref())
                    .await
                    .map(|_| ())
                    .map_err(|e| {
                        DomainError::new(
                            ErrorKind::InternalError,
                            "Folder",
                            format!("Failed to rollback folder move: {}", e),
                        )
                    })
            }
        };

        // Add to the transaction
        transaction.add_operation(move_op, rollback_op);

        // Execute transaction
        transaction.commit().await?;

        // Get the moved folder
        let folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get moved folder with ID: {}: {}", id, e),
            )
        })?;

        Ok(FolderDto::from(folder))
    }

    /// Deletes a folder
    async fn delete_folder(&self, id: &str) -> Result<(), DomainError> {
        // Verify the folder exists
        let _folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get folder with ID: {} for deletion: {}", id, e),
            )
        })?;

        // In a real implementation, we could verify permissions, dependencies, etc.

        // Delete the folder
        self.folder_storage.delete_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to delete folder with ID: {}: {}", id, e),
            )
        })
    }
}
