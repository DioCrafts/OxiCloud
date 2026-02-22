//! Domain persistence port for the Folder entity.
//!
//! Defines the contract that any folder storage implementation
//! must fulfill. This trait lives in the domain because Folder is a core entity
//! of the system and its persistence contracts belong to the domain layer,
//! following the principles of Clean/Hexagonal Architecture.
//!
//! Concrete implementations (filesystem, PostgreSQL, S3, etc.) live in
//! the infrastructure layer.

use async_trait::async_trait;

use crate::common::errors::DomainError;
use crate::domain::entities::folder::Folder;
use crate::domain::services::path_service::StoragePath;

/// Domain port for folder persistence.
///
/// Defines the CRUD and management operations required for
/// the Folder entity in the storage system.
#[async_trait]
pub trait FolderRepository: Send + Sync + 'static {
    /// Creates a new folder
    async fn create_folder(
        &self,
        name: String,
        parent_id: Option<String>,
    ) -> Result<Folder, DomainError>;

    /// Gets a folder by its ID
    async fn get_folder(&self, id: &str) -> Result<Folder, DomainError>;

    /// Gets a folder by its storage path
    async fn get_folder_by_path(&self, storage_path: &StoragePath) -> Result<Folder, DomainError>;

    /// Lists folders within a parent folder
    async fn list_folders(&self, parent_id: Option<&str>) -> Result<Vec<Folder>, DomainError>;

    /// Lists root-level folders owned by a specific user.
    /// For non-root queries (parent_id is Some), ownership is implicit
    /// because the parent already belongs to the user.
    async fn list_folders_by_owner(
        &self,
        parent_id: Option<&str>,
        owner_id: &str,
    ) -> Result<Vec<Folder>, DomainError>;

    /// Lists folders with pagination
    async fn list_folders_paginated(
        &self,
        parent_id: Option<&str>,
        offset: usize,
        limit: usize,
        include_total: bool,
    ) -> Result<(Vec<Folder>, Option<usize>), DomainError>;

    /// Lists folders with pagination, scoped to a specific owner.
    /// Combines the owner filtering of `list_folders_by_owner` with
    /// the pagination of `list_folders_paginated`.
    async fn list_folders_by_owner_paginated(
        &self,
        parent_id: Option<&str>,
        owner_id: &str,
        offset: usize,
        limit: usize,
        include_total: bool,
    ) -> Result<(Vec<Folder>, Option<usize>), DomainError>;

    /// Renames a folder
    async fn rename_folder(&self, id: &str, new_name: String) -> Result<Folder, DomainError>;

    /// Moves a folder to another parent
    async fn move_folder(
        &self,
        id: &str,
        new_parent_id: Option<&str>,
    ) -> Result<Folder, DomainError>;

    /// Deletes a folder
    async fn delete_folder(&self, id: &str) -> Result<(), DomainError>;

    /// Checks if a folder exists at the given path
    async fn folder_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError>;

    /// Gets the path of a folder
    async fn get_folder_path(&self, id: &str) -> Result<StoragePath, DomainError>;

    // ── Trash operations ──

    /// Moves a folder to the trash
    async fn move_to_trash(&self, folder_id: &str) -> Result<(), DomainError>;

    /// Restores a folder from the trash to its original location
    async fn restore_from_trash(
        &self,
        folder_id: &str,
        original_path: &str,
    ) -> Result<(), DomainError>;

    /// Permanently deletes a folder (used by the trash)
    async fn delete_folder_permanently(&self, folder_id: &str) -> Result<(), DomainError>;

    /// Creates a root-level home folder for a user.
    /// This is used during user registration to create the user's personal folder.
    async fn create_home_folder(&self, user_id: &str, name: String) -> Result<Folder, DomainError>;

    /// Lists all descendant folders in a subtree (ltree-based).
    ///
    /// Returns all folders whose lpath is a descendant of the given folder's
    /// lpath. Used for recursive search — O(1) SQL via GiST index instead
    /// of O(N) recursive traversal.
    ///
    /// The default implementation returns an empty vec (stubs / mocks).
    async fn list_descendant_folders(
        &self,
        folder_id: &str,
        name_contains: Option<&str>,
        user_id: &str,
    ) -> Result<Vec<Folder>, DomainError> {
        let _ = (folder_id, name_contains, user_id);
        Ok(Vec::new())
    }
}
