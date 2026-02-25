use std::sync::Arc;

use async_trait::async_trait;
use thiserror::Error;
use tokio::sync::Semaphore;

use crate::{
    application::{
        dtos::{
            pagination::PaginatedResponseDto,
            share_dto::{CreateShareDto, ShareDto, UpdateShareDto},
        },
        ports::{
            auth_ports::PasswordHasherPort,
            outbound::FolderStoragePort,
            share_ports::{ShareStoragePort, ShareUseCase},
            storage_ports::FileReadPort,
        },
    },
    common::{config::AppConfig, errors::DomainError},
    domain::entities::share::{Share, ShareItemType, SharePermissions},
};

#[derive(Debug, Error)]
pub enum ShareServiceError {
    #[error("Share not found: {0}")]
    NotFound(String),
    #[error("Item not found: {0}")]
    ItemNotFound(String),
    #[error("Access denied: {0}")]
    AccessDenied(String),
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    #[error("Share expired")]
    Expired,
    #[error("Repository error: {0}")]
    Repository(String),
    #[error("Invalid item type: {0}")]
    InvalidItemType(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<ShareServiceError> for DomainError {
    fn from(error: ShareServiceError) -> Self {
        match error {
            ShareServiceError::NotFound(s) => DomainError::not_found("Share", s),
            ShareServiceError::ItemNotFound(s) => DomainError::not_found("Item", s),
            ShareServiceError::AccessDenied(s) => DomainError::access_denied("Share", s),
            ShareServiceError::InvalidPassword(s) => DomainError::access_denied("Share", s),
            ShareServiceError::Expired => {
                DomainError::access_denied("Share", "Share has expired".to_string())
            }
            ShareServiceError::Repository(s) => DomainError::internal_error("Share", s),
            ShareServiceError::InvalidItemType(s) => DomainError::validation_error(s),
            ShareServiceError::Validation(s) => DomainError::validation_error(s),
        }
    }
}

/// Maximum number of concurrent Argon2 hashing operations.
///
/// Each Argon2id hash consumes ~19 MB of RAM and ~300 ms of CPU.
/// Limiting concurrency prevents RAM exhaustion and thread-pool saturation
/// under burst traffic (e.g. many share-creation requests with passwords).
const MAX_CONCURRENT_HASHES: usize = 2;

pub struct ShareService {
    config: Arc<AppConfig>,
    share_repository: Arc<dyn ShareStoragePort>,
    file_repository: Arc<dyn FileReadPort>,
    folder_repository: Arc<dyn FolderStoragePort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    /// Bounds the number of in-flight Argon2 password hashes to avoid
    /// saturating the blocking thread pool and consuming excessive RAM.
    hash_semaphore: Arc<Semaphore>,
}

impl ShareService {
    pub fn new(
        config: Arc<AppConfig>,
        share_repository: Arc<dyn ShareStoragePort>,
        file_repository: Arc<dyn FileReadPort>,
        folder_repository: Arc<dyn FolderStoragePort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
    ) -> Self {
        Self {
            config,
            share_repository,
            file_repository,
            folder_repository,
            password_hasher,
            hash_semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_HASHES)),
        }
    }

    /// Verifies that the item to share exists
    async fn verify_item_exists(
        &self,
        item_id: &str,
        item_type: &ShareItemType,
    ) -> Result<(), ShareServiceError> {
        match item_type {
            ShareItemType::File => {
                self.file_repository
                    .get_file(item_id) // Using the correct method from the FileStoragePort trait
                    .await
                    .map_err(|_| {
                        ShareServiceError::ItemNotFound(format!(
                            "File with ID {} not found",
                            item_id
                        ))
                    })?;
            }
            ShareItemType::Folder => {
                self.folder_repository
                    .get_folder(item_id) // Using the correct method from the FolderStoragePort trait
                    .await
                    .map_err(|_| {
                        ShareServiceError::ItemNotFound(format!(
                            "Folder with ID {} not found",
                            item_id
                        ))
                    })?;
            }
        }
        Ok(())
    }

    /// Hash a password via the injected `PasswordHasherPort`, bounded by a
    /// semaphore so at most `MAX_CONCURRENT_HASHES` Argon2 operations run
    /// concurrently. This keeps RAM usage predictable (~19 MB × 2 = ~38 MB max)
    /// and avoids starving the Tokio blocking thread pool.
    async fn hash_password_async(&self, password: &str) -> Result<String, DomainError> {
        let _permit = self.hash_semaphore.acquire().await
            .map_err(|_| DomainError::internal_error("ShareService", "Hash semaphore closed".to_string()))?;
        self.password_hasher.hash_password(password).await
    }
}

#[async_trait]
impl ShareUseCase for ShareService {
    async fn create_shared_link(
        &self,
        user_id: &str,
        dto: CreateShareDto,
    ) -> Result<ShareDto, DomainError> {
        // Convert the item type
        let item_type = ShareItemType::try_from(dto.item_type.as_str())
            .map_err(|e| ShareServiceError::InvalidItemType(e.to_string()))?;

        // Verify that the item exists
        self.verify_item_exists(&dto.item_id, &item_type).await?;

        // Convert the permissions DTO if it exists
        let permissions = dto.permissions.map(|p| p.to_entity());

        // Hash the password if provided (async, semaphore-bounded)
        let password_hash = match dto.password {
            Some(p) => Some(self.hash_password_async(&p).await?),
            None => None,
        };

        // Create the Share entity
        let share = Share::new(
            dto.item_id.clone(),
            dto.item_name.clone(),
            item_type,
            user_id.to_string(),
            permissions,
            password_hash,
            dto.expires_at,
        )
        .map_err(|e| ShareServiceError::Validation(e.to_string()))?;

        // Save to the repository
        let saved_share = self
            .share_repository
            .save_share(&share)
            .await
            .map_err(|e| ShareServiceError::Repository(e.to_string()))?;

        // Convert the entity to DTO for the response
        Ok(ShareDto::from_entity(&saved_share, &self.config.base_url()))
    }

    async fn get_shared_link(&self, id: &str) -> Result<ShareDto, DomainError> {
        // Find the shared link by its ID
        let share = self
            .share_repository
            .find_share_by_id(id)
            .await
            .map_err(|e| {
                ShareServiceError::NotFound(format!("Share with ID {} not found: {}", id, e))
            })?;

        // Check if it has expired
        if share.is_expired() {
            return Err(ShareServiceError::Expired.into());
        }

        // Convert the entity to DTO for the response
        Ok(ShareDto::from_entity(&share, &self.config.base_url()))
    }

    async fn get_shared_link_by_token(&self, token: &str) -> Result<ShareDto, DomainError> {
        // Find the shared link by its token
        let share = self
            .share_repository
            .find_share_by_token(token)
            .await
            .map_err(|e| {
                ShareServiceError::NotFound(format!("Share with token {} not found: {}", token, e))
            })?;

        // Check if it has expired
        if share.is_expired() {
            return Err(ShareServiceError::Expired.into());
        }

        // Convert the entity to DTO for the response
        Ok(ShareDto::from_entity(&share, &self.config.base_url()))
    }

    async fn get_shared_links_for_item(
        &self,
        item_id: &str,
        item_type: &ShareItemType,
    ) -> Result<Vec<ShareDto>, DomainError> {
        // Find all shared links for the item
        let shares = self
            .share_repository
            .find_shares_by_item(item_id, item_type)
            .await
            .map_err(|e| ShareServiceError::Repository(e.to_string()))?;

        // Filter out expired links
        let active_shares: Vec<Share> = shares.into_iter().filter(|s| !s.is_expired()).collect();

        // Convert the entities to DTOs for the response
        let share_dtos = active_shares
            .iter()
            .map(|s| ShareDto::from_entity(s, &self.config.base_url()))
            .collect();

        Ok(share_dtos)
    }

    async fn update_shared_link(
        &self,
        id: &str,
        dto: UpdateShareDto,
    ) -> Result<ShareDto, DomainError> {
        // Find the existing shared link
        let mut share = self
            .share_repository
            .find_share_by_id(id)
            .await
            .map_err(|e| {
                ShareServiceError::NotFound(format!("Share with ID {} not found: {}", id, e))
            })?;

        // Update permissions if provided
        if let Some(permissions_dto) = dto.permissions {
            let permissions = SharePermissions::new(
                permissions_dto.read,
                permissions_dto.write,
                permissions_dto.reshare,
            );
            share = share.with_permissions(permissions);
        }

        // Update password if provided (async, semaphore-bounded)
        if let Some(password) = dto.password {
            let password_hash = if password.is_empty() {
                None
            } else {
                Some(self.hash_password_async(&password).await?)
            };
            share = share.with_password(password_hash);
        }

        // Update expiration date if provided
        if dto.expires_at.is_some() {
            share = share.with_expiration(dto.expires_at);
        }

        // Save the changes
        let updated_share = self
            .share_repository
            .update_share(&share)
            .await
            .map_err(|e| ShareServiceError::Repository(e.to_string()))?;

        // Convert the entity to DTO for the response
        Ok(ShareDto::from_entity(
            &updated_share,
            &self.config.base_url(),
        ))
    }

    async fn delete_shared_link(&self, id: &str) -> Result<(), DomainError> {
        // Delete the shared link
        self.share_repository
            .delete_share(id)
            .await
            .map_err(|e| ShareServiceError::Repository(e.to_string()))?;

        Ok(())
    }

    async fn get_user_shared_links(
        &self,
        user_id: &str,
        page: usize,
        per_page: usize,
    ) -> Result<PaginatedResponseDto<ShareDto>, DomainError> {
        // Calculate offset for pagination
        let offset = (page - 1) * per_page;

        // Find the user's shared links
        let (shares, total) = self
            .share_repository
            .find_shares_by_user(user_id, offset, per_page)
            .await
            .map_err(|e| ShareServiceError::Repository(e.to_string()))?;

        // Convert the entities to DTOs
        let share_dtos: Vec<ShareDto> = shares
            .iter()
            .map(|s| ShareDto::from_entity(s, &self.config.base_url()))
            .collect();

        // Create the paginated result
        let paginated = PaginatedResponseDto::new(share_dtos, page, per_page, total);

        Ok(paginated)
    }

    async fn verify_shared_link_password(
        &self,
        token: &str,
        password: &str,
    ) -> Result<bool, DomainError> {
        // Find the shared link by its token
        let share = self
            .share_repository
            .find_share_by_token(token)
            .await
            .map_err(|e| {
                ShareServiceError::NotFound(format!("Share with token {} not found: {}", token, e))
            })?;

        // Check if it has expired
        if share.is_expired() {
            return Err(ShareServiceError::Expired.into());
        }

        // Verify the password using the infrastructure port
        match share.password_hash() {
            Some(hash) => self.password_hasher.verify_password(password, hash).await,
            None => Ok(true), // No password required
        }
    }

    async fn register_shared_link_access(&self, token: &str) -> Result<(), DomainError> {
        // Find the shared link by its token
        let share = self
            .share_repository
            .find_share_by_token(token)
            .await
            .map_err(|e| {
                ShareServiceError::NotFound(format!("Share with token {} not found: {}", token, e))
            })?;

        // Check if it has expired
        if share.is_expired() {
            return Err(ShareServiceError::Expired.into());
        }

        // Increment the access counter
        let updated_share = share.increment_access_count();

        // Save the changes
        self.share_repository
            .update_share(&updated_share)
            .await
            .map_err(|e| ShareServiceError::Repository(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dtos::share_dto::SharePermissionsDto;
    use crate::application::ports::auth_ports::PasswordHasherPort;
    use crate::application::ports::share_ports::ShareStoragePort;
    use crate::common::config::AppConfig;
    use crate::domain::repositories::folder_repository::FolderRepository;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;

    struct MockPasswordHasher;

    #[async_trait]
    impl PasswordHasherPort for MockPasswordHasher {
        async fn hash_password(&self, password: &str) -> Result<String, DomainError> {
            Ok(format!("hashed_{}", password))
        }

        async fn verify_password(&self, _password: &str, _hash: &str) -> Result<bool, DomainError> {
            Ok(true)
        }
    }

    struct MockFileRepository;
    struct MockFolderRepository;

    #[async_trait]
    impl FileReadPort for MockFileRepository {
        async fn get_file(
            &self,
            id: &str,
        ) -> Result<crate::domain::entities::file::File, DomainError> {
            if id == "test_file_id" {
                let file = crate::domain::entities::file::File::new(
                    id.to_string(),
                    "test.txt".to_string(),
                    crate::domain::services::path_service::StoragePath::from_string(
                        "/path/to/test.txt",
                    ),
                    123,
                    "text/plain".to_string(),
                    None,
                )
                .unwrap();
                Ok(file)
            } else {
                Err(DomainError::not_found("File", id))
            }
        }

        async fn list_files(
            &self,
            _folder_id: Option<&str>,
        ) -> Result<Vec<crate::domain::entities::file::File>, DomainError> {
            unimplemented!()
        }

        async fn get_file_stream(
            &self,
            _id: &str,
        ) -> Result<
            Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>,
            DomainError,
        > {
            unimplemented!()
        }

        async fn get_file_range_stream(
            &self,
            _id: &str,
            _start: u64,
            _end: Option<u64>,
        ) -> Result<
            Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>,
            DomainError,
        > {
            unimplemented!()
        }

        async fn get_file_path(
            &self,
            _id: &str,
        ) -> Result<crate::domain::services::path_service::StoragePath, DomainError> {
            unimplemented!()
        }

        async fn get_parent_folder_id(&self, _path: &str) -> Result<String, DomainError> {
            unimplemented!()
        }

        async fn get_blob_hash(&self, _file_id: &str) -> Result<String, DomainError> {
            Ok(String::new())
        }

        async fn search_files_paginated(
            &self,
            _folder_id: Option<&str>,
            _criteria: &crate::application::dtos::search_dto::SearchCriteriaDto,
            _user_id: &str,
        ) -> Result<(Vec<crate::domain::entities::file::File>, usize), DomainError> {
            Ok((Vec::new(), 0))
        }

        async fn count_files(
            &self,
            _folder_id: Option<&str>,
            _criteria: &crate::application::dtos::search_dto::SearchCriteriaDto,
            _user_id: &str,
        ) -> Result<usize, DomainError> {
            Ok(0)
        }
    }

    #[async_trait]
    impl FolderRepository for MockFolderRepository {
        async fn create_folder(
            &self,
            _name: String,
            _parent_id: Option<String>,
        ) -> Result<crate::domain::entities::folder::Folder, DomainError> {
            unimplemented!()
        }

        async fn get_folder(
            &self,
            id: &str,
        ) -> Result<crate::domain::entities::folder::Folder, DomainError> {
            if id == "test_folder_id" {
                let folder = crate::domain::entities::folder::Folder::new(
                    id.to_string(),
                    "test".to_string(),
                    crate::domain::services::path_service::StoragePath::from_string(
                        "/path/to/test",
                    ),
                    None,
                )
                .unwrap();
                Ok(folder)
            } else {
                Err(DomainError::not_found("Folder", id))
            }
        }

        async fn get_folder_by_path(
            &self,
            _storage_path: &crate::domain::services::path_service::StoragePath,
        ) -> Result<crate::domain::entities::folder::Folder, DomainError> {
            unimplemented!()
        }

        async fn list_folders(
            &self,
            _parent_id: Option<&str>,
        ) -> Result<Vec<crate::domain::entities::folder::Folder>, DomainError> {
            unimplemented!()
        }

        async fn list_folders_by_owner(
            &self,
            _parent_id: Option<&str>,
            _owner_id: &str,
        ) -> Result<Vec<crate::domain::entities::folder::Folder>, DomainError> {
            unimplemented!()
        }

        async fn list_folders_paginated(
            &self,
            _parent_id: Option<&str>,
            _offset: usize,
            _limit: usize,
            _include_total: bool,
        ) -> Result<(Vec<crate::domain::entities::folder::Folder>, Option<usize>), DomainError>
        {
            unimplemented!()
        }

        async fn list_folders_by_owner_paginated(
            &self,
            _parent_id: Option<&str>,
            _owner_id: &str,
            _offset: usize,
            _limit: usize,
            _include_total: bool,
        ) -> Result<(Vec<crate::domain::entities::folder::Folder>, Option<usize>), DomainError>
        {
            unimplemented!()
        }

        async fn rename_folder(
            &self,
            _id: &str,
            _new_name: String,
        ) -> Result<crate::domain::entities::folder::Folder, DomainError> {
            unimplemented!()
        }

        async fn move_folder(
            &self,
            _id: &str,
            _new_parent_id: Option<&str>,
        ) -> Result<crate::domain::entities::folder::Folder, DomainError> {
            unimplemented!()
        }

        async fn delete_folder(&self, _id: &str) -> Result<(), DomainError> {
            unimplemented!()
        }

        async fn folder_exists(
            &self,
            _storage_path: &crate::domain::services::path_service::StoragePath,
        ) -> Result<bool, DomainError> {
            unimplemented!()
        }

        async fn get_folder_path(
            &self,
            _id: &str,
        ) -> Result<crate::domain::services::path_service::StoragePath, DomainError> {
            unimplemented!()
        }

        async fn move_to_trash(&self, _folder_id: &str) -> Result<(), DomainError> {
            unimplemented!()
        }

        async fn restore_from_trash(
            &self,
            _folder_id: &str,
            _original_path: &str,
        ) -> Result<(), DomainError> {
            unimplemented!()
        }

        async fn delete_folder_permanently(&self, _folder_id: &str) -> Result<(), DomainError> {
            unimplemented!()
        }

        async fn create_home_folder(
            &self,
            _user_id: &str,
            _name: String,
        ) -> Result<crate::domain::entities::folder::Folder, DomainError> {
            unimplemented!()
        }

        async fn bulk_delete_trashed_folders(
            &self,
            _user_id: &str,
        ) -> Result<(u64, Vec<String>), DomainError> {
            // Mock implementation for tests
            Ok((0, Vec::new()))
        }
    }

    struct MockShareRepository {
        shares: Mutex<HashMap<String, Share>>,
        tokens: Mutex<HashMap<String, String>>, // token -> id mapping
    }

    impl MockShareRepository {
        fn new() -> Self {
            Self {
                shares: Mutex::new(HashMap::new()),
                tokens: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl ShareStoragePort for MockShareRepository {
        async fn save_share(&self, share: &Share) -> Result<Share, DomainError> {
            let mut shares = self.shares.lock().unwrap();
            let mut tokens = self.tokens.lock().unwrap();

            shares.insert(share.id().to_string(), share.clone());
            tokens.insert(share.token().to_string(), share.id().to_string());

            Ok(share.clone())
        }

        async fn find_share_by_id(&self, id: &str) -> Result<Share, DomainError> {
            let shares = self.shares.lock().unwrap();

            shares
                .get(id)
                .cloned()
                .ok_or_else(|| DomainError::not_found("Share", id))
        }

        async fn find_share_by_token(&self, token: &str) -> Result<Share, DomainError> {
            let tokens = self.tokens.lock().unwrap();
            let shares = self.shares.lock().unwrap();

            let id = tokens
                .get(token)
                .ok_or_else(|| DomainError::not_found("Share", token))?;

            shares
                .get(id)
                .cloned()
                .ok_or_else(|| DomainError::not_found("Share", id.as_str()))
        }

        async fn find_shares_by_item(
            &self,
            item_id: &str,
            item_type: &ShareItemType,
        ) -> Result<Vec<Share>, DomainError> {
            let shares = self.shares.lock().unwrap();

            let type_str = item_type.to_string();
            let result: Vec<Share> = shares
                .values()
                .filter(|s| s.item_id() == item_id && s.item_type().to_string() == type_str)
                .cloned()
                .collect();

            Ok(result)
        }

        async fn update_share(&self, share: &Share) -> Result<Share, DomainError> {
            let mut shares = self.shares.lock().unwrap();

            let id_str = share.id().to_string();
            if !shares.contains_key(&id_str) {
                return Err(DomainError::not_found("Share", &id_str));
            }

            shares.insert(id_str, share.clone());

            Ok(share.clone())
        }

        async fn delete_share(&self, id: &str) -> Result<(), DomainError> {
            let mut shares = self.shares.lock().unwrap();
            let mut tokens = self.tokens.lock().unwrap();

            // Find the share to get the token
            let share = shares
                .get(id)
                .ok_or_else(|| DomainError::not_found("Share", id))?;

            // Remove token mapping
            tokens.remove(share.token());

            // Remove the share
            shares.remove(id);

            Ok(())
        }

        async fn find_shares_by_user(
            &self,
            user_id: &str,
            offset: usize,
            limit: usize,
        ) -> Result<(Vec<Share>, usize), DomainError> {
            let shares = self.shares.lock().unwrap();

            let user_shares: Vec<Share> = shares
                .values()
                .filter(|s| s.created_by() == user_id)
                .cloned()
                .collect();

            let total = user_shares.len();

            // Apply pagination
            let paginated = user_shares.into_iter().skip(offset).take(limit).collect();

            Ok((paginated, total))
        }
    }

    #[tokio::test]
    async fn test_create_shared_link() {
        let config = Arc::new(AppConfig::default());

        let share_repo = Arc::new(MockShareRepository::new());
        let file_repo = Arc::new(MockFileRepository);
        let folder_repo = Arc::new(MockFolderRepository);
        let password_hasher = Arc::new(MockPasswordHasher);

        let service =
            ShareService::new(config, share_repo, file_repo, folder_repo, password_hasher);

        // Test creating a file share
        let dto = CreateShareDto {
            item_id: "test_file_id".to_string(),
            item_name: Some("test_file.txt".to_string()),
            item_type: "file".to_string(),
            password: Some("secret".to_string()),
            expires_at: None,
            permissions: Some(SharePermissionsDto {
                read: true,
                write: false,
                reshare: false,
            }),
        };

        let result = service.create_shared_link("user123", dto).await;
        assert!(result.is_ok());

        let share_dto = result.unwrap();
        assert_eq!(share_dto.item_id, "test_file_id");
        assert_eq!(share_dto.item_type, "file");
        assert!(share_dto.has_password);
        assert!(share_dto.url.starts_with("http://127.0.0.1:8086/s/"));
    }
}
