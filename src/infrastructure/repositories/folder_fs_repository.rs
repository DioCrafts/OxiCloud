use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use tokio::fs;
use tokio::time::timeout;

use crate::domain::entities::folder::{Folder, FolderError};
use crate::infrastructure::repositories::repository_errors::{
    FolderRepositoryError, FolderRepositoryResult
};
use crate::domain::services::path_service::StoragePath;
use crate::infrastructure::services::path_service::PathService;
// use crate::application::ports::outbound::IdMappingPort;
use crate::infrastructure::services::id_mapping_service::{IdMappingService, IdMappingError};
use crate::application::services::storage_mediator::StorageMediator;
use crate::domain::repositories::folder_repository::FolderRepository;
use crate::common::errors::DomainError;

// To be able to use streams in the list_folders function
use tokio_stream;

/// Filesystem implementation of the FolderRepository interface
pub struct FolderFsRepository {
    root_path: PathBuf,
    storage_mediator: Arc<dyn StorageMediator>,
    id_mapping_service: Arc<dyn crate::application::ports::outbound::IdMappingPort>,
    path_service: Arc<PathService>,
}

impl FolderFsRepository {
    /// Creates a new filesystem-based folder repository
    pub fn new(
        root_path: PathBuf,
        storage_mediator: Arc<dyn StorageMediator>,
        id_mapping_service: Arc<dyn crate::application::ports::outbound::IdMappingPort>,
        path_service: Arc<PathService>,
    ) -> Self {
        Self { 
            root_path, 
            storage_mediator, 
            id_mapping_service,
            path_service,
        }
    }
    
    /// Returns the root path of the storage
    pub fn get_root_path(&self) -> &PathBuf {
        &self.root_path
    }
    
    /// Creates a stub repository for initialization purposes
    /// This is used temporarily during dependency injection setup
    pub fn new_stub() -> Self {
        let root_path = PathBuf::from("/tmp");
        let path_service = Arc::new(PathService::new(root_path.clone()));
        
        // Create minimal implementations just to satisfy initialization
        // Since we can't easily block on an async function in a sync context, create with a stub
        let id_mapping_service = Arc::new(
            IdMappingService::new_sync(root_path.clone())
        );
        
        // Create a self-referential stub (only used for initialization)
        let storage_mediator_stub = Arc::new(
            crate::application::services::storage_mediator::StubStorageMediator::new()
        );
        
        Self {
            root_path,
            storage_mediator: storage_mediator_stub,
            id_mapping_service,
            path_service,
        }
    }
    
    /// Gets the count of items in a directory efficiently
    async fn count_directory_items(&self, directory_path: &Path) -> FolderRepositoryResult<usize> {
        use tokio::fs::read_dir;
        
        // Timeout to avoid blocking
        let read_dir_timeout = Duration::from_secs(30);
        let read_dir_result = timeout(
            read_dir_timeout,
            read_dir(directory_path)
        ).await;
        
        match read_dir_result {
            Ok(result) => {
                let mut entries = result.map_err(|e| FolderRepositoryError::StorageError(e.to_string()))?;
                let mut count = 0;
                
                // Count entries manually
                while let Ok(Some(_)) = entries.next_entry().await {
                    count += 1;
                }
                
                Ok(count)
            },
            Err(_) => {
                Err(FolderRepositoryError::Other(
                    format!("Timeout counting items in directory: {}", directory_path.display())
                ))
            }
        }
    }
    
    /// Resolves a domain storage path to an absolute filesystem path
    fn resolve_storage_path(&self, storage_path: &StoragePath) -> PathBuf {
        self.path_service.resolve_path(storage_path)
    }
    
    /// Returns a reference to the ID mapping service
    pub fn id_mapping_service(&self) -> &Arc<dyn crate::application::ports::outbound::IdMappingPort> {
        &self.id_mapping_service
    }

    /// Gets the storage path for a folder by its ID (internal helper)
    async fn _get_folder_storage_path(&self, id: &str) -> FolderRepositoryResult<StoragePath> {
        let storage_path = self.id_mapping_service.get_path_by_id(id).await
            .map_err(FolderRepositoryError::from)?;
        Ok(storage_path)
    }
    
    /// Gets a folder path from the ID mapping service
    pub async fn get_mapped_folder_path(&self, folder_id: &str) -> FolderRepositoryResult<String> {
        let storage_path = self.id_mapping_service.get_path_by_id(folder_id).await
            .map_err(|e| FolderRepositoryError::StorageError(format!("Failed to get folder path: {}", e)))?;
        Ok(storage_path.to_string())
    }
    
    /// Updates a folder path in the ID mapping service
    pub async fn update_mapped_folder_path(&self, folder_id: &str, new_path: &PathBuf) -> FolderRepositoryResult<()> {
        let storage_path = StoragePath::from_string(&new_path.to_string_lossy().to_string());
        self.id_mapping_service.update_path(folder_id, &storage_path).await
            .map_err(|e| FolderRepositoryError::StorageError(format!("Failed to update folder path: {}", e)))
    }
    
    /// Removes a folder ID from the ID mapping service
    pub async fn remove_mapped_folder_id(&self, folder_id: &str) -> FolderRepositoryResult<()> {
        self.id_mapping_service.remove_id(folder_id).await
            .map_err(|e| FolderRepositoryError::StorageError(format!("Failed to remove folder ID: {}", e)))
    }
    
    /// Checks if a folder exists at a given storage path
    async fn check_folder_exists_at_storage_path(&self, storage_path: &StoragePath) -> FolderRepositoryResult<bool> {
        let abs_path = self.resolve_storage_path(storage_path);
        
        // Check if folder exists and is a directory
        let exists = abs_path.exists() && abs_path.is_dir();
        
        tracing::debug!("Checking if folder exists: {} - path: {}", exists, abs_path.display());
        
        Ok(exists)
    }
    
    /// Creates the physical directory on the filesystem
    async fn create_directory(&self, path: &Path) -> Result<(), std::io::Error> {
        fs::create_dir_all(path).await
    }
    
    /// Helper method to create a Folder entity from a storage path and metadata
    async fn create_folder_entity(
        &self,
        id: String,
        name: String,
        storage_path: StoragePath,
        parent_id: Option<String>,
        created_at: Option<u64>,
        modified_at: Option<u64>,
    ) -> FolderRepositoryResult<Folder> {
        // If timestamps are provided, use them; otherwise, let Folder::new create default timestamps
        let folder = if let (Some(created), Some(modified)) = (created_at, modified_at) {
            Folder::with_timestamps(
                id, 
                name, 
                storage_path, 
                parent_id,
                created,
                modified,
            )
        } else {
            Folder::new(
                id, 
                name, 
                storage_path, 
                parent_id,
            )
        };
        
        // Convert domain error to repository error
        folder.map_err(|e| match e {
            FolderError::InvalidFolderName(name) => 
                FolderRepositoryError::ValidationError(format!("Invalid folder name: {}", name)),
            FolderError::ValidationError(msg) =>
                FolderRepositoryError::ValidationError(msg),
        })
    }
    
    /// Extracts folder metadata from a physical path
    async fn get_folder_metadata(&self, abs_path: &PathBuf) -> FolderRepositoryResult<(u64, u64)> {
        let metadata = fs::metadata(&abs_path).await
            .map_err(|e| FolderRepositoryError::StorageError(e.to_string()))?;
            
        // Get creation timestamp
        let created_at = metadata.created()
            .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
            .unwrap_or_else(|_| 0);
            
        // Get modification timestamp
        let modified_at = metadata.modified()
            .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
            .unwrap_or_else(|_| 0);
            
        Ok((created_at, modified_at))
    }
}

// Convert IdMappingError to FolderRepositoryError
impl From<IdMappingError> for FolderRepositoryError {
    fn from(err: IdMappingError) -> Self {
        match err {
            IdMappingError::NotFound(id) => FolderRepositoryError::NotFound(id),
            IdMappingError::IoError(e) => FolderRepositoryError::StorageError(e.to_string()),
            IdMappingError::Timeout(msg) => FolderRepositoryError::StorageError(format!("Timeout: {}", msg)),
            _ => FolderRepositoryError::StorageError(err.to_string()),
        }
    }
}

// Implement Clone to use in concurrent processing
impl Clone for FolderFsRepository {
    fn clone(&self) -> Self {
        // Clone the Arcs, which only increments the reference counter
        Self {
            root_path: self.root_path.clone(),
            storage_mediator: self.storage_mediator.clone(),
            id_mapping_service: self.id_mapping_service.clone(),
            path_service: self.path_service.clone(),
        }
    }
}

#[async_trait]
impl FolderRepository for FolderFsRepository {
    async fn create_folder(&self, name: String, parent_id: Option<String>) -> Result<Folder, DomainError> {
        // Get the parent folder path (if any)
        let parent_storage_path = match &parent_id {
            Some(id) => {
                match self._get_folder_storage_path(id).await {
                    Ok(path) => {
                        tracing::info!("Using folder path: {:?} for parent_id: {:?}", path.to_string(), id);
                        Some(path)
                    },
                    Err(e) => {
                        tracing::error!("Error getting parent folder: {}", e);
                        return Err(DomainError::from(e));
                    },
                }
            },
            None => None,
        };
        
        // Create the storage path for the new folder
        let folder_storage_path = match parent_storage_path {
            Some(parent) => parent.join(&name),
            None => StoragePath::from_string(&name),
        };
        tracing::info!("Creating folder at path: {:?}", folder_storage_path.to_string());
        
        // Check if folder already exists
        if self.check_folder_exists_at_storage_path(&folder_storage_path).await.map_err(DomainError::from)? {
            return Err(DomainError::already_exists("Folder", folder_storage_path.to_string()));
        }
        
        // Create the physical directory
        let abs_path = self.resolve_storage_path(&folder_storage_path);
        self.create_directory(&abs_path).await
            .map_err(|e| DomainError::internal_error("Folder", e.to_string()))?;
        
        // Create and return the folder entity with a persisted ID
        let id = self.id_mapping_service.get_or_create_id(&folder_storage_path).await
            .map_err(|e| DomainError::internal_error("Folder", e.to_string()))?;
        let folder = self.create_folder_entity(
            id.clone(),
            name.clone(),
            folder_storage_path.clone(),
            parent_id.clone(),
            None,
            None,
        ).await.map_err(DomainError::from)?;
        
        // Ensure ID mapping is persisted
        let save_result = self.id_mapping_service.save_changes().await;
        if let Err(e) = &save_result {
            tracing::error!("Failed to save ID mapping for folder {}: {}", id, e);
        } else {
            tracing::info!("Successfully saved ID mapping for folder ID: {} -> path: {} (name: {})", 
                id, folder_storage_path.to_string(), name);
        }
        save_result?;
        
        tracing::debug!("Created folder with ID: {}", folder.id());
        Ok(folder)
    }
    
    async fn get_folder(&self, id: &str) -> Result<Folder, DomainError> {
        tracing::debug!("Looking for folder with ID: {}", id);
        
        // Find path by ID using the mapping service
        let storage_path = self.id_mapping_service.get_path_by_id(id).await?;
        
        // Check if folder exists physically
        let abs_path = self.resolve_storage_path(&storage_path);
        if !abs_path.exists() || !abs_path.is_dir() {
            tracing::error!("Folder not found at path: {}", abs_path.display());
            return Err(DomainError::not_found("Folder", format!("Folder {} not found at {}", id, storage_path.to_string())));
        }
        
        // Get folder metadata
        let (created_at, modified_at) = self.get_folder_metadata(&abs_path).await.map_err(DomainError::from)?;
        
        // Get folder name from the storage path
        let name = match storage_path.file_name() {
            Some(name) => name,
            None => {
                tracing::error!("Invalid folder path: {}", storage_path.to_string());
                return Err(DomainError::validation_error(format!("Invalid path: {}", storage_path.to_string())));
            }
        };
        
        // Determine parent ID if any
        let parent = storage_path.parent();
        let parent_id: Option<String> = if parent.is_none() || parent.as_ref().unwrap().is_empty() {
            None
        } else {
            match self.id_mapping_service.get_or_create_id(parent.as_ref().unwrap()).await {
                Ok(pid) => Some(pid),
                Err(_) => None,
            }
        };
        
        // Create folder entity
        let folder = self.create_folder_entity(
            id.to_string(),
            name,
            storage_path,
            parent_id,
            Some(created_at),
            Some(modified_at),
        ).await.map_err(DomainError::from)?;
        
        Ok(folder)
    }
    
    async fn get_folder_by_path(&self, storage_path: &StoragePath) -> Result<Folder, DomainError> {
        // Check if the physical directory exists
        let abs_path = self.resolve_storage_path(storage_path);
        if !abs_path.exists() || !abs_path.is_dir() {
            return Err(DomainError::not_found("Folder", storage_path.to_string()));
        }
        
        // Extract folder name from storage path
        let name = match storage_path.file_name() {
            Some(name) => name,
            None => {
                return Err(DomainError::validation_error(format!("Invalid path: {}", storage_path.to_string())));
            }
        };
        
        // Determine parent ID if any
        let parent = storage_path.parent();
        let parent_id: Option<String> = if parent.is_none() || parent.as_ref().unwrap().is_empty() {
            None
        } else {
            match self.id_mapping_service.get_or_create_id(parent.as_ref().unwrap()).await {
                Ok(pid) => Some(pid),
                Err(_) => None,
            }
        };
        
        // Get folder metadata
        let (created_at, modified_at) = self.get_folder_metadata(&abs_path).await.map_err(DomainError::from)?;
        
        // Get or create an ID for this path
        let id = self.id_mapping_service.get_or_create_id(storage_path).await?;
        tracing::debug!("Found folder with path: {:?}, assigned ID: {}", storage_path.to_string(), id);
        
        // Create folder entity
        let folder = self.create_folder_entity(
            id,
            name,
            storage_path.clone(),
            parent_id,
            Some(created_at),
            Some(modified_at),
        ).await.map_err(DomainError::from)?;
        
        // Ensure ID mapping is persisted
        self.id_mapping_service.save_changes().await?;
        
        Ok(folder)
    }
    
    async fn list_folders(&self, parent_id: Option<&str>) -> Result<Vec<Folder>, DomainError> {
        use futures::stream::StreamExt;
        use tokio::time::{timeout, Duration};
        
        tracing::info!("Listing folders in parent_id: {:?}", parent_id);
        
        // Get the parent storage path
        let parent_storage_path = match parent_id {
            Some(id) => {
                match self._get_folder_storage_path(id).await {
                    Ok(path) => {
                        tracing::info!("Found parent folder with path: {:?}", path.to_string());
                        path
                    },
                    Err(e) => {
                        tracing::error!("Error getting parent folder by ID: {}: {}", id, e);
                        return Ok(Vec::new());
                    },
                }
            },
            None => StoragePath::root(),
        };
        
        // Get the absolute folder path
        let abs_parent_path = self.resolve_storage_path(&parent_storage_path);
        tracing::info!("Absolute parent path: {:?}", &abs_parent_path);
        
        // Ensure the directory exists
        if !abs_parent_path.exists() || !abs_parent_path.is_dir() {
            tracing::error!("Directory does not exist or is not a directory: {:?}", &abs_parent_path);
            return Ok(Vec::new());
        }
        
        // Read the directory with a timeout
        let read_dir_timeout = Duration::from_secs(30);
        let read_dir_result = match timeout(
            read_dir_timeout,
            fs::read_dir(&abs_parent_path)
        ).await {
            Ok(result) => result.map_err(|e| DomainError::internal_error("Folder", e.to_string()))?,
            Err(_) => {
                return Err(DomainError::internal_error("Folder",
                    format!("Timeout reading directory: {}", abs_parent_path.display())
                ));
            }
        };
        
        let mut folders = Vec::new();
        let mut entries = tokio_stream::wrappers::ReadDirStream::new(read_dir_result);
        
        while let Some(entry_result) = entries.next().await {
            let entry = match entry_result {
                Ok(e) => e,
                Err(err) => {
                    tracing::error!("Error reading directory entry: {}", err);
                    continue;
                }
            };
            
            let metadata = match entry.metadata().await {
                Ok(m) => m,
                Err(err) => {
                    tracing::error!("Error getting metadata for {}: {}", entry.path().display(), err);
                    continue;
                }
            };
            
            if !metadata.is_dir() {
                continue;
            }
            
            let folder_name = entry.file_name().to_string_lossy().to_string();
            let folder_storage_path = parent_storage_path.join(&folder_name);
            
            let get_folder_timeout = Duration::from_secs(5);
            let folder_result = timeout(
                get_folder_timeout,
                self.get_folder_by_path(&folder_storage_path)
            ).await;
            
            match folder_result {
                Ok(result) => {
                    match result {
                        Ok(folder) => {
                            tracing::debug!("Found folder: {}", folder.name());
                            folders.push(folder);
                        },
                        Err(e) => {
                            tracing::warn!("Could not get folder entity for {}: {}", folder_name, e);
                        }
                    }
                },
                Err(_) => {
                    tracing::warn!("Timeout getting folder entity for {}", folder_name);
                }
            }
        }
        
        if let Err(e) = self.id_mapping_service.save_changes().await {
            tracing::error!("Failed to save ID mappings: {}", e);
        }
        
        tracing::info!("Found {} folders in parent {:?}", folders.len(), parent_id);
        Ok(folders)
    }
    
    async fn list_folders_paginated(
        &self, 
        parent_id: Option<&str>,
        offset: usize,
        limit: usize,
        include_total: bool
    ) -> Result<(Vec<Folder>, Option<usize>), DomainError> {
        use futures::stream::StreamExt;
        use tokio::time::{timeout, Duration};
        
        tracing::info!("Listing folders in parent_id: {:?} with pagination (offset={}, limit={})", 
            parent_id, offset, limit);
        
        let parent_storage_path = match parent_id {
            Some(id) => {
                match self._get_folder_storage_path(id).await {
                    Ok(path) => path,
                    Err(e) => {
                        tracing::error!("Error getting parent folder by ID: {}: {}", id, e);
                        return Ok((Vec::new(), Some(0)));
                    },
                }
            },
            None => StoragePath::root(),
        };
        
        let abs_parent_path = self.resolve_storage_path(&parent_storage_path);
        
        if !abs_parent_path.exists() || !abs_parent_path.is_dir() {
            return Ok((Vec::new(), Some(0)));
        }
        
        let total_count = if include_total {
            match self.count_directory_items(&abs_parent_path).await {
                Ok(count) => Some(count),
                Err(e) => {
                    tracing::warn!("Error counting directory items: {}", e);
                    None
                }
            }
        } else {
            None
        };
        
        let read_dir_timeout = Duration::from_secs(30);
        let read_dir_result = match timeout(
            read_dir_timeout,
            fs::read_dir(&abs_parent_path)
        ).await {
            Ok(result) => result.map_err(|e| DomainError::internal_error("Folder", e.to_string()))?,
            Err(_) => {
                return Err(DomainError::internal_error("Folder",
                    format!("Timeout reading directory: {}", abs_parent_path.display())
                ));
            }
        };
        
        let mut entries = tokio_stream::wrappers::ReadDirStream::new(read_dir_result);
        let mut folders = Vec::new();
        let mut current_idx = 0;
        
        while let Some(entry_result) = entries.next().await {
            if current_idx < offset {
                current_idx += 1;
                continue;
            }
            
            if folders.len() >= limit {
                break;
            }
            
            let entry = match entry_result {
                Ok(e) => e,
                Err(err) => {
                    tracing::error!("Error reading directory entry: {}", err);
                    current_idx += 1;
                    continue;
                }
            };
            
            let file_type = match entry.file_type().await {
                Ok(ft) => ft,
                Err(e) => {
                    tracing::error!("Error getting file type: {}", e);
                    current_idx += 1;
                    continue;
                }
            };
            
            if !file_type.is_dir() {
                current_idx += 1;
                continue;
            }
            
            let path = entry.path();
            let rel_path = match path.strip_prefix(&self.root_path) {
                Ok(rel) => StoragePath::from(rel.to_path_buf()),
                Err(_) => {
                    tracing::error!("Error stripping prefix from path: {}", path.display());
                    current_idx += 1;
                    continue;
                }
            };
            
            let folder_result = timeout(
                Duration::from_secs(10),
                self.get_folder_by_path(&rel_path)
            ).await;
            
            match folder_result {
                Ok(result) => match result {
                    Ok(folder) => {
                        folders.push(folder);
                    },
                    Err(e) => {
                        tracing::error!("Error getting folder by path: {}: {}", rel_path.to_string(), e);
                    }
                },
                Err(_) => {
                    tracing::error!("Timeout getting folder by path: {}", rel_path.to_string());
                }
            }
            
            current_idx += 1;
        }
        
        if !folders.is_empty() {
            if let Err(e) = self.id_mapping_service.save_changes().await {
                tracing::error!("Error saving ID mappings: {}", e);
            }
        }
        
        Ok((folders, total_count))
    }
    
    async fn rename_folder(&self, id: &str, new_name: String) -> Result<Folder, DomainError> {
        let original_folder = self.get_folder(id).await?;
        tracing::debug!("Renaming folder with ID: {}, Name: {}", id, original_folder.name());
        
        let renamed_folder = original_folder.with_name(new_name)
            .map_err(|e| DomainError::validation_error(e.to_string()))?;
        
        if self.check_folder_exists_at_storage_path(renamed_folder.storage_path()).await.map_err(DomainError::from)? {
            return Err(DomainError::already_exists("Folder", renamed_folder.storage_path().to_string()));
        }
        
        let abs_old_path = self.resolve_storage_path(original_folder.storage_path());
        let abs_new_path = self.resolve_storage_path(renamed_folder.storage_path());
        
        fs::rename(&abs_old_path, &abs_new_path).await
            .map_err(|e| DomainError::internal_error("Folder", e.to_string()))?;
            
        self.id_mapping_service.update_path(id, renamed_folder.storage_path()).await?;
        self.id_mapping_service.save_changes().await?;
        
        tracing::debug!("Folder renamed successfully: ID={}, New name={}", id, renamed_folder.name());
        Ok(renamed_folder)
    }
    
    async fn move_folder(&self, id: &str, new_parent_id: Option<&str>) -> Result<Folder, DomainError> {
        let original_folder = self.get_folder(id).await?;
        tracing::debug!("Moving folder with ID: {}, Name: {}", id, original_folder.name());
        
        if original_folder.parent_id() == new_parent_id {
            tracing::info!("Folder is already in the target parent, no need to move");
            return Ok(original_folder);
        }
        
        let target_parent_storage_path = match new_parent_id {
            Some(parent_id) => {
                match self._get_folder_storage_path(parent_id).await {
                    Ok(path) => Some(path),
                    Err(e) => {
                        return Err(DomainError::internal_error("Folder",
                            format!("Could not get target folder: {}", e)
                        ));
                    }
                }
            },
            None => None
        };
        
        let new_parent_id_option = new_parent_id.map(String::from);
        let moved_folder = original_folder.with_parent(new_parent_id_option, target_parent_storage_path)
            .map_err(|e| DomainError::validation_error(e.to_string()))?;
        
        if self.check_folder_exists_at_storage_path(moved_folder.storage_path()).await.map_err(DomainError::from)? {
            return Err(DomainError::already_exists("Folder",
                format!("Folder already exists at destination: {}", moved_folder.storage_path().to_string())
            ));
        }
        
        let old_abs_path = self.resolve_storage_path(original_folder.storage_path());
        let new_abs_path = self.resolve_storage_path(moved_folder.storage_path());
        
        if let Some(parent) = new_abs_path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| DomainError::internal_error("Folder", e.to_string()))?;
        }
        
        fs::rename(&old_abs_path, &new_abs_path).await
            .map_err(|e| DomainError::internal_error("Folder", e.to_string()))?;
            
        self.id_mapping_service.update_path(id, moved_folder.storage_path()).await?;
        self.id_mapping_service.save_changes().await?;
        
        tracing::debug!("Folder moved successfully: ID={}, New path={:?}", id, moved_folder.storage_path().to_string());
        Ok(moved_folder)
    }
    
    async fn delete_folder(&self, id: &str) -> Result<(), DomainError> {
        use tokio::time::{timeout, Duration};
        
        let folder = self.get_folder(id).await?;
        let folder_name = folder.name().to_string();
        let storage_path = folder.storage_path().clone();
        
        tracing::info!("Deleting folder with ID: {}, Name: {}", id, folder_name);
        
        let abs_path = self.resolve_storage_path(&storage_path);
        let path_for_display = abs_path.display().to_string();
        let path_for_deletion = abs_path.clone();
        
        let delete_task = tokio::spawn(async move {
            tracing::debug!("Starting removal of folder: {}", path_for_display);
            
            let path_for_counting = path_for_deletion.clone();
            let entry_count = tokio::task::spawn_blocking(move || {
                let mut count = 0;
                if let Ok(entries) = std::fs::read_dir(&path_for_counting) {
                    for _ in entries {
                        count += 1;
                        if count > 1000 {
                            break;
                        }
                    }
                }
                count
            }).await.unwrap_or(0);
            
            if entry_count > 1000 {
                tracing::info!("Large folder detected with >1000 entries, using blocking removal");
                let path_for_large_removal = path_for_deletion.clone();
                tokio::task::spawn_blocking(move || {
                    if let Err(e) = std::fs::remove_dir_all(&path_for_large_removal) {
                        tracing::error!("Error removing large directory: {}", e);
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other, 
                            format!("Failed to remove large directory: {}", e)
                        ));
                    }
                    Ok(())
                }).await.unwrap_or_else(|e| {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Task panicked during directory removal: {}", e)
                    ))
                })
            } else {
                fs::remove_dir_all(&path_for_deletion).await
            }
        });
        
        const DELETE_TIMEOUT_SECS: u64 = 60;
        
        let delete_result = timeout(
            Duration::from_secs(DELETE_TIMEOUT_SECS), 
            delete_task
        ).await;
        
        match delete_result {
            Ok(task_result) => {
                match task_result {
                    Ok(fs_result) => {
                        if let Err(e) = fs_result {
                            return Err(DomainError::internal_error("Folder", e.to_string()));
                        }
                    },
                    Err(join_err) => {
                        return Err(DomainError::internal_error("Folder",
                            format!("Task panicked during folder deletion: {}", join_err)
                        ));
                    }
                }
            },
            Err(_) => {
                tracing::warn!("Timeout waiting for folder deletion, continuing with ID removal");
            }
        }
        
        const MAPPING_TIMEOUT_SECS: u64 = 5;
        let remove_id_result = timeout(
            Duration::from_secs(MAPPING_TIMEOUT_SECS),
            self.id_mapping_service.remove_id(id)
        ).await;
        
        match remove_id_result {
            Ok(result) => result?,
            Err(_) => {
                return Err(DomainError::internal_error("Folder",
                    "Timeout removing folder ID from mapping".to_string()
                ));
            }
        }
        
        let _ = self.id_mapping_service.save_changes().await;
        
        tracing::info!("Folder deleted successfully: ID={}, Name={}", id, folder_name);
        Ok(())
    }
    
    async fn folder_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError> {
        self.check_folder_exists_at_storage_path(storage_path).await.map_err(DomainError::from)
    }
    
    async fn get_folder_path(&self, id: &str) -> Result<StoragePath, DomainError> {
        self._get_folder_storage_path(id).await.map_err(DomainError::from)
    }

    async fn move_to_trash(&self, folder_id: &str) -> Result<(), DomainError> {
        self._trash_move_to_trash(folder_id).await.map_err(DomainError::from)
    }

    async fn restore_from_trash(&self, folder_id: &str, original_path: &str) -> Result<(), DomainError> {
        self._trash_restore_from_trash(folder_id, original_path).await.map_err(DomainError::from)
    }

    async fn delete_folder_permanently(&self, folder_id: &str) -> Result<(), DomainError> {
        self._trash_delete_folder_permanently(folder_id).await.map_err(DomainError::from)
    }
}