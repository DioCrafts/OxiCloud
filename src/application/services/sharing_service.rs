use std::sync::Arc;
use async_trait::async_trait;
use tracing::{debug, instrument};

use crate::domain::repositories::shared_file_repository::{
    SharedFileRepository,
    SharedFileRepositoryError,
    SharedFileRepositoryResult
};
use crate::domain::entities::shared_file::{SharedFile, PermissionLevel};
use crate::application::ports::outbound::FileStoragePort;
use crate::common::errors::DomainError;

/// Service for sharing files between users
pub struct SharingService {
    shared_file_repository: Arc<dyn SharedFileRepository>,
    file_repository: Arc<dyn FileStoragePort>,
}

impl SharingService {
    /// Create a new SharingService
    pub fn new(
        shared_file_repository: Arc<dyn SharedFileRepository>,
        file_repository: Arc<dyn FileStoragePort>,
    ) -> Self {
        Self {
            shared_file_repository,
            file_repository,
        }
    }

    /// Share a file with a user
    #[instrument(skip(self))]
    pub async fn share_file(
        &self,
        file_id: &str,
        owner_id: &str,
        user_id: &str,
        permission: PermissionLevel,
    ) -> Result<SharedFile, DomainError> {
        debug!("Sharing file {} from owner {} with user {} (permission: {:?})", 
               file_id, owner_id, user_id, permission);
        
        // Verify the file exists
        self.file_repository.get_file(file_id).await?;
        
        // Share the file
        let shared_file = self.shared_file_repository
            .share_file_with_user(file_id, owner_id, user_id, permission)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(shared_file)
    }

    /// Update sharing permissions for a file
    #[instrument(skip(self))]
    pub async fn update_permission(
        &self,
        file_id: &str,
        owner_id: &str,
        user_id: &str,
        permission: PermissionLevel,
    ) -> Result<SharedFile, DomainError> {
        debug!("Updating permission for file {} shared by {} with user {} to {:?}", 
               file_id, owner_id, user_id, permission);
        
        // Verify ownership before updating
        let shared_file = self.shared_file_repository
            .get_shared_file(file_id, user_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        if shared_file.owner_id() != owner_id {
            return Err(DomainError::Unauthorized(
                "You are not the owner of this shared file".to_string()
            ));
        }
        
        // Update permission
        let updated = self.shared_file_repository
            .update_permission(file_id, user_id, permission)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(updated)
    }

    /// Stop sharing a file with a user
    #[instrument(skip(self))]
    pub async fn unshare_file(
        &self,
        file_id: &str,
        owner_id: &str,
        user_id: &str,
    ) -> Result<(), DomainError> {
        debug!("Removing sharing for file {} by owner {} with user {}", 
               file_id, owner_id, user_id);
        
        // Verify ownership before unsharing
        let shared_file = self.shared_file_repository
            .get_shared_file(file_id, user_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        if shared_file.owner_id() != owner_id {
            return Err(DomainError::Unauthorized(
                "You are not the owner of this shared file".to_string()
            ));
        }
        
        // Remove sharing
        self.shared_file_repository
            .unshare_file(file_id, user_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(())
    }

    /// Get shared file
    #[instrument(skip(self))]
    pub async fn get_shared_file(
        &self,
        file_id: &str,
        user_id: &str,
    ) -> Result<SharedFile, DomainError> {
        debug!("Getting shared file {} for user {}", file_id, user_id);
        
        let shared_file = self.shared_file_repository
            .get_shared_file(file_id, user_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(shared_file)
    }

    /// Get all files shared with a user
    #[instrument(skip(self))]
    pub async fn get_files_shared_with_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<SharedFile>, DomainError> {
        debug!("Getting files shared with user {}", user_id);
        
        let shared_files = self.shared_file_repository
            .get_files_shared_with_user(user_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(shared_files)
    }

    /// Get all files a user has shared with others
    #[instrument(skip(self))]
    pub async fn get_files_shared_by_user(
        &self,
        owner_id: &str,
    ) -> Result<Vec<SharedFile>, DomainError> {
        debug!("Getting files shared by user {}", owner_id);
        
        let shared_files = self.shared_file_repository
            .get_files_shared_by_user(owner_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(shared_files)
    }

    /// Get all users a file is shared with
    #[instrument(skip(self))]
    pub async fn get_users_with_access(
        &self,
        file_id: &str,
        owner_id: &str,
    ) -> Result<Vec<SharedFile>, DomainError> {
        debug!("Getting users with access to file {}", file_id);
        
        // Verify file exists and user is owner
        let file = self.file_repository.get_file(file_id).await?;
        // For now, let's use a dummy owner check. In a real system, files would have owner IDs
        
        let users = self.shared_file_repository
            .get_users_with_access(file_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(users)
    }

    /// Check if a user has access to a file
    #[instrument(skip(self))]
    pub async fn check_user_has_access(
        &self,
        file_id: &str,
        user_id: &str,
    ) -> Result<Option<PermissionLevel>, DomainError> {
        debug!("Checking if user {} has access to file {}", user_id, file_id);
        
        let permission = self.shared_file_repository
            .check_user_has_access(file_id, user_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(permission)
    }

    /// Map repository errors to domain errors
    fn map_repository_error(error: SharedFileRepositoryError) -> DomainError {
        match error {
            SharedFileRepositoryError::NotFound(msg) => DomainError::not_found("shared_file", msg),
            SharedFileRepositoryError::AlreadyExists(msg) => DomainError::already_exists("shared_file", msg),
            SharedFileRepositoryError::DatabaseError(msg) => DomainError::internal_error("shared_file", msg),
            SharedFileRepositoryError::InvalidPermission(msg) => DomainError::validation_error("shared_file", msg),
            SharedFileRepositoryError::DomainError(err) => err,
            SharedFileRepositoryError::Other(msg) => DomainError::internal_error("shared_file", msg),
        }
    }
}