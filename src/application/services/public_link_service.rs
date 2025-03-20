use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{debug, instrument};

use crate::domain::repositories::public_link_repository::{
    PublicLinkRepository,
    PublicLinkRepositoryError,
    PublicLinkRepositoryResult
};
use crate::domain::entities::public_link::PublicLink;
use crate::domain::entities::shared_file::PermissionLevel;
use crate::application::ports::outbound::FileStoragePort;
use crate::common::errors::DomainError;

/// Service for managing public links to files
pub struct PublicLinkService {
    public_link_repository: Arc<dyn PublicLinkRepository>,
    file_repository: Arc<dyn FileStoragePort>,
}

impl PublicLinkService {
    /// Create a new PublicLinkService
    pub fn new(
        public_link_repository: Arc<dyn PublicLinkRepository>,
        file_repository: Arc<dyn FileStoragePort>,
    ) -> Self {
        Self {
            public_link_repository,
            file_repository,
        }
    }

    /// Create a new public link for a file
    #[instrument(skip(self, password))]
    pub async fn create_public_link(
        &self,
        file_id: &str,
        owner_id: &str,
        permission: PermissionLevel,
        password: Option<&str>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<PublicLink, DomainError> {
        debug!("Creating public link for file {} by owner {}", file_id, owner_id);
        
        // Verify the file exists
        self.file_repository.get_file(file_id).await?;
        
        // Create the public link
        let link = self.public_link_repository
            .create_public_link(file_id, owner_id, permission, password, expires_at)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(link)
    }

    /// Get a public link by its ID
    #[instrument(skip(self))]
    pub async fn get_public_link(
        &self,
        link_id: &str,
    ) -> Result<PublicLink, DomainError> {
        debug!("Getting public link {}", link_id);
        
        let link = self.public_link_repository
            .get_public_link(link_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(link)
    }

    /// Update the permission for a public link
    #[instrument(skip(self))]
    pub async fn update_permission(
        &self,
        link_id: &str,
        owner_id: &str,
        permission: PermissionLevel,
    ) -> Result<PublicLink, DomainError> {
        debug!("Updating permission for link {} to {:?}", link_id, permission);
        
        let link = self.public_link_repository
            .update_permission(link_id, owner_id, permission)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(link)
    }

    /// Update the password for a public link
    #[instrument(skip(self, password))]
    pub async fn update_password(
        &self,
        link_id: &str,
        owner_id: &str,
        password: Option<&str>,
    ) -> Result<PublicLink, DomainError> {
        debug!("Updating password for link {}", link_id);
        
        let link = self.public_link_repository
            .update_password(link_id, owner_id, password)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(link)
    }

    /// Update the expiration date for a public link
    #[instrument(skip(self))]
    pub async fn update_expiration(
        &self,
        link_id: &str,
        owner_id: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<PublicLink, DomainError> {
        debug!("Updating expiration for link {}", link_id);
        
        let link = self.public_link_repository
            .update_expiration(link_id, owner_id, expires_at)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(link)
    }

    /// Delete a public link
    #[instrument(skip(self))]
    pub async fn delete_public_link(
        &self,
        link_id: &str,
        owner_id: &str,
    ) -> Result<(), DomainError> {
        debug!("Deleting public link {}", link_id);
        
        self.public_link_repository
            .delete_public_link(link_id, owner_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(())
    }

    /// Get all public links for a file
    #[instrument(skip(self))]
    pub async fn get_links_for_file(
        &self,
        file_id: &str,
        owner_id: &str,
    ) -> Result<Vec<PublicLink>, DomainError> {
        debug!("Getting public links for file {}", file_id);
        
        let links = self.public_link_repository
            .get_links_for_file(file_id, owner_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(links)
    }

    /// Get all public links created by a user
    #[instrument(skip(self))]
    pub async fn get_links_by_user(
        &self,
        owner_id: &str,
    ) -> Result<Vec<PublicLink>, DomainError> {
        debug!("Getting public links created by user {}", owner_id);
        
        let links = self.public_link_repository
            .get_links_by_user(owner_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(links)
    }

    /// Verify a password for a public link
    #[instrument(skip(self, password))]
    pub async fn verify_password(
        &self,
        link_id: &str,
        password: &str,
    ) -> Result<bool, DomainError> {
        debug!("Verifying password for link {}", link_id);
        
        let is_valid = self.public_link_repository
            .verify_password(link_id, password)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(is_valid)
    }

    /// Access a public link - increments the access counter
    #[instrument(skip(self))]
    pub async fn access_link(
        &self,
        link_id: &str,
    ) -> Result<PublicLink, DomainError> {
        debug!("Accessing public link {}", link_id);
        
        let link = self.public_link_repository
            .increment_access_count(link_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(link)
    }

    /// Access a file through a public link, with password verification if needed
    #[instrument(skip(self, password))]
    pub async fn access_file(
        &self,
        link_id: &str,
        password: Option<&str>,
    ) -> Result<PublicLink, DomainError> {
        debug!("Accessing file through public link {}", link_id);
        
        // Get the link first to check if a password is required
        let link = self.public_link_repository
            .get_public_link(link_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        // Check if password is required
        if link.password_hash().is_some() {
            // Password is required
            if let Some(pwd) = password {
                let is_valid = self.public_link_repository
                    .verify_password(link_id, pwd)
                    .await
                    .map_err(Self::map_repository_error)?;
                
                if !is_valid {
                    return Err(DomainError::Unauthorized("Invalid password".to_string()));
                }
            } else {
                return Err(DomainError::Unauthorized("Password required".to_string()));
            }
        }
        
        // Verify the file still exists
        self.file_repository.get_file(&link.file_id()).await?;
        
        // Increment the access counter
        let updated_link = self.public_link_repository
            .increment_access_count(link_id)
            .await
            .map_err(Self::map_repository_error)?;
        
        Ok(updated_link)
    }

    /// Get the file content through a public link, with password verification if needed
    #[instrument(skip(self, password))]
    pub async fn get_file_content(
        &self,
        link_id: &str,
        password: Option<&str>,
    ) -> Result<Vec<u8>, DomainError> {
        debug!("Getting file content through public link {}", link_id);
        
        // First access the link
        let link = self.access_file(link_id, password).await?;
        
        // Then get the file content
        let content = self.file_repository
            .get_file_content(&link.file_id())
            .await?;
        
        Ok(content)
    }

    /// Get the file stream through a public link, with password verification if needed
    #[instrument(skip(self, password))]
    pub async fn get_file_stream(
        &self,
        link_id: &str,
        password: Option<&str>,
    ) -> Result<Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>, DomainError> {
        debug!("Getting file stream through public link {}", link_id);
        
        // First access the link
        let link = self.access_file(link_id, password).await?;
        
        // Then get the file stream
        let stream = self.file_repository
            .get_file_stream(&link.file_id())
            .await?;
        
        Ok(stream)
    }

    /// Map repository errors to domain errors
    fn map_repository_error(error: PublicLinkRepositoryError) -> DomainError {
        match error {
            PublicLinkRepositoryError::NotFound(msg) => DomainError::not_found("public_link", msg),
            PublicLinkRepositoryError::DatabaseError(msg) => DomainError::internal_error("public_link", msg),
            PublicLinkRepositoryError::InvalidPermission(msg) => DomainError::validation_error("public_link", msg),
            PublicLinkRepositoryError::DomainError(err) => err,
            PublicLinkRepositoryError::Expired => DomainError::validation_error("public_link", "Public link is expired"),
            PublicLinkRepositoryError::PasswordRequired => DomainError::access_denied("public_link", "Password required"),
            PublicLinkRepositoryError::InvalidPassword => DomainError::access_denied("public_link", "Invalid password"),
            PublicLinkRepositoryError::Other(msg) => DomainError::internal_error("public_link", msg),
        }
    }
}