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

    /// Lists every folder in a subtree rooted at `folder_id` (inclusive).
    ///
    /// Uses ltree `<@` for a single GiST-indexed scan.  The result is
    /// ordered by `path` so callers can iterate in directory order.
    ///
    /// Default: falls back to `list_folders` (one level only).
    async fn list_subtree_folders(&self, folder_id: &str) -> Result<Vec<Folder>, DomainError> {
        let _ = folder_id;
        Ok(Vec::new())
    }

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

    /// Search folders with SQL-level filtering by name, user, and scope.
    ///
    /// - **Non-recursive** (`recursive = false`): searches direct children of
    ///   `parent_id` (or root folders when `None`).
    /// - **Recursive with `parent_id`**: delegates to `list_descendant_folders`
    ///   (ltree GiST-indexed scan).
    /// - **Recursive without `parent_id`**: searches ALL folders owned by
    ///   `user_id` with optional name filter in SQL.
    ///
    /// The default implementation falls back to `list_folders` + in-memory
    /// filter so that stubs and mocks compile without changes.
    async fn search_folders(
        &self,
        parent_id: Option<&str>,
        name_contains: Option<&str>,
        user_id: &str,
        recursive: bool,
    ) -> Result<Vec<Folder>, DomainError> {
        // Recursive with folder_id → use optimised ltree scan
        if recursive && let Some(fid) = parent_id {
            return self
                .list_descendant_folders(fid, name_contains, user_id)
                .await;
        }
        // Fallback: load + filter in memory (stubs / mocks)
        let all = self.list_folders(parent_id).await?;
        match name_contains {
            Some(q) if !q.is_empty() => {
                let q = q.to_lowercase();
                Ok(all
                    .into_iter()
                    .filter(|f| f.name().to_lowercase().contains(&q))
                    .collect())
            }
            _ => Ok(all),
        }
    }

    /// Return up to `limit` folders whose name contains `query` (case-insensitive).
    ///
    /// Results are ordered by relevance (exact > starts-with > contains) for
    /// autocomplete suggestions.
    ///
    /// The default implementation falls back to `list_folders` + in-memory
    /// filter so that stubs and mocks compile without changes.
    async fn suggest_folders_by_name(
        &self,
        parent_id: Option<&str>,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Folder>, DomainError> {
        let all = self.list_folders(parent_id).await?;
        let q = query.to_lowercase();
        let mut matched: Vec<Folder> = all
            .into_iter()
            .filter(|f| f.name().to_lowercase().contains(&q))
            .collect();
        matched.truncate(limit);
        Ok(matched)
    }
}
