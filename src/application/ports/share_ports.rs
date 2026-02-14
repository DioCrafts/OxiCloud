use async_trait::async_trait;

use crate::{
    application::dtos::{
        pagination::PaginatedResponseDto,
        share_dto::{CreateShareDto, ShareDto, UpdateShareDto},
    },
    common::errors::DomainError,
    domain::entities::share::ShareItemType,
};

#[async_trait]
pub trait ShareUseCase: Send + Sync + 'static {
    /// Create a new shared link for a file or folder
    async fn create_shared_link(
        &self,
        user_id: &str,
        dto: CreateShareDto,
    ) -> Result<ShareDto, DomainError>;

    /// Get a shared link by its ID
    async fn get_shared_link(&self, id: &str) -> Result<ShareDto, DomainError>;

    /// Get a shared link by its token (for access by non-users)
    async fn get_shared_link_by_token(&self, token: &str) -> Result<ShareDto, DomainError>;

    /// Get all shared links for a specific item
    async fn get_shared_links_for_item(
        &self,
        item_id: &str,
        item_type: &ShareItemType,
    ) -> Result<Vec<ShareDto>, DomainError>;

    /// Update a shared link
    async fn update_shared_link(
        &self,
        id: &str,
        dto: UpdateShareDto,
    ) -> Result<ShareDto, DomainError>;

    /// Delete a shared link
    async fn delete_shared_link(&self, id: &str) -> Result<(), DomainError>;

    /// Get all shared links created by a specific user
    async fn get_user_shared_links(
        &self,
        user_id: &str,
        page: usize,
        per_page: usize,
    ) -> Result<PaginatedResponseDto<ShareDto>, DomainError>;

    /// Verify a password for a password-protected shared link
    async fn verify_shared_link_password(
        &self,
        token: &str,
        password: &str,
    ) -> Result<bool, DomainError>;

    /// Register an access to a shared link
    async fn register_shared_link_access(&self, token: &str) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ShareStoragePort: Send + Sync + 'static {
    async fn save_share(
        &self,
        share: &crate::domain::entities::share::Share,
    ) -> Result<crate::domain::entities::share::Share, DomainError>;

    async fn find_share_by_id(
        &self,
        id: &str,
    ) -> Result<crate::domain::entities::share::Share, DomainError>;

    async fn find_share_by_token(
        &self,
        token: &str,
    ) -> Result<crate::domain::entities::share::Share, DomainError>;

    async fn find_shares_by_item(
        &self,
        item_id: &str,
        item_type: &ShareItemType,
    ) -> Result<Vec<crate::domain::entities::share::Share>, DomainError>;

    async fn update_share(
        &self,
        share: &crate::domain::entities::share::Share,
    ) -> Result<crate::domain::entities::share::Share, DomainError>;

    async fn delete_share(&self, id: &str) -> Result<(), DomainError>;

    async fn find_shares_by_user(
        &self,
        user_id: &str,
        offset: usize,
        limit: usize,
    ) -> Result<(Vec<crate::domain::entities::share::Share>, usize), DomainError>;
}
