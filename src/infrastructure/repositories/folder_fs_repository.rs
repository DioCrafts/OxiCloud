use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use tokio::fs;
use tokio::time::timeout;
use tracing::instrument;

use crate::domain::entities::folder::{Folder, FolderError};
use crate::domain::repositories::folder_repository::{
    FolderRepository, FolderRepositoryError, FolderRepositoryResult
};
use crate::domain::services::path_service::{StoragePath, PathService};
// use crate::application::ports::outbound::IdMappingPort;
use crate::infrastructure::services::id_mapping_service::{IdMappingService, IdMappingError};
use crate::application::services::storage_mediator::StorageMediator;
use crate::application::ports::outbound::FolderStoragePort;
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
    #[allow(dead_code)]
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
                let mut entries = result.map_err(FolderRepositoryError::IoError)?;
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
    
    /// Resolves a legacy PathBuf to an absolute filesystem path
    fn resolve_legacy_path(&self, relative_path: &std::path::Path) -> PathBuf {
        self.storage_mediator.resolve_path(relative_path)
    }
    
    /// Returns a reference to the ID mapping service
    pub fn id_mapping_service(&self) -> &Arc<dyn crate::application::ports::outbound::IdMappingPort> {
        &self.id_mapping_service
    }
    
    /// Gets a folder path from the ID mapping service
    pub async fn get_mapped_folder_path(&self, folder_id: &str) -> FolderRepositoryResult<String> {
        let storage_path = self.id_mapping_service.get_path_by_id(folder_id).await
            .map_err(|e| FolderRepositoryError::MappingError(format!("Failed to get folder path: {}", e)))?;
        Ok(storage_path.to_string())
    }
    
    /// Updates a folder path in the ID mapping service
    pub async fn update_mapped_folder_path(&self, folder_id: &str, new_path: &PathBuf) -> FolderRepositoryResult<()> {
        let storage_path = StoragePath::from_string(&new_path.to_string_lossy().to_string());
        self.id_mapping_service.update_path(folder_id, &storage_path).await
            .map_err(|e| FolderRepositoryError::MappingError(format!("Failed to update folder path: {}", e)))
    }
    
    /// Removes a folder ID from the ID mapping service
    pub async fn remove_mapped_folder_id(&self, folder_id: &str) -> FolderRepositoryResult<()> {
        self.id_mapping_service.remove_id(folder_id).await
            .map_err(|e| FolderRepositoryError::MappingError(format!("Failed to remove folder ID: {}", e)))
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
            .map_err(FolderRepositoryError::IoError)?;
            
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
            IdMappingError::IoError(e) => FolderRepositoryError::IoError(e),
            IdMappingError::Timeout(msg) => FolderRepositoryError::Other(format!("Timeout: {}", msg)),
            _ => FolderRepositoryError::MappingError(err.to_string()),
        }
    }
}

// Convert FolderRepositoryError to DomainError
impl From<FolderRepositoryError> for DomainError {
    fn from(err: FolderRepositoryError) -> Self {
        match err {
            FolderRepositoryError::NotFound(id) => {
                DomainError::not_found("Folder", id)
            },
            FolderRepositoryError::AlreadyExists(path) => {
                DomainError::already_exists("Folder", path)
            },
            FolderRepositoryError::InvalidPath(path) => {
                DomainError::validation_error("Folder", format!("Invalid path: {}", path))
            },
            FolderRepositoryError::IoError(e) => {
                DomainError::internal_error("Folder", format!("IO error: {}", e))
                    .with_source(e)
            },
            FolderRepositoryError::ValidationError(msg) => {
                DomainError::validation_error("Folder", msg)
            },
            FolderRepositoryError::MappingError(msg) => {
                DomainError::internal_error("Folder", format!("Mapping error: {}", msg))
            },
            FolderRepositoryError::Other(msg) => {
                DomainError::internal_error("Folder", msg)
            },
            FolderRepositoryError::OperationNotSupported(msg) => {
                DomainError::operation_not_supported("Folder", msg)
            },
            FolderRepositoryError::DomainError(e) => e,
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
impl FolderStoragePort for FolderFsRepository {
    async fn create_folder(&self, name: String, parent_id: Option<String>) -> Result<Folder, DomainError> {
        FolderRepository::create_folder(self, name, parent_id).await.map_err(DomainError::from)
    }
    
    async fn get_folder(&self, id: &str) -> Result<Folder, DomainError> {
        FolderRepository::get_folder_by_id(self, id).await.map_err(DomainError::from)
    }
    
    async fn get_folder_by_path(&self, storage_path: &StoragePath) -> Result<Folder, DomainError> {
        FolderRepository::get_folder_by_storage_path(self, storage_path).await.map_err(DomainError::from)
    }
    
    async fn list_folders(&self, parent_id: Option<&str>) -> Result<Vec<Folder>, DomainError> {
        FolderRepository::list_folders(self, parent_id).await.map_err(DomainError::from)
    }
    
    async fn rename_folder(&self, id: &str, new_name: String) -> Result<Folder, DomainError> {
        FolderRepository::rename_folder(self, id, new_name).await.map_err(DomainError::from)
    }
    
    async fn move_folder(&self, id: &str, new_parent_id: Option<&str>) -> Result<Folder, DomainError> {
        FolderRepository::move_folder(self, id, new_parent_id).await.map_err(DomainError::from)
    }
    
    async fn delete_folder(&self, id: &str) -> Result<(), DomainError> {
        FolderRepository::delete_folder(self, id).await.map_err(DomainError::from)
    }
    
    async fn folder_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError> {
        FolderRepository::folder_exists_at_storage_path(self, storage_path).await.map_err(DomainError::from)
    }
    
    async fn get_folder_path(&self, id: &str) -> Result<StoragePath, DomainError> {
        FolderRepository::get_folder_storage_path(self, id).await.map_err(DomainError::from)
    }
    
    async fn list_folders_paginated(
        &self, 
        parent_id: Option<&str>,
        offset: usize,
        limit: usize,
        include_total: bool
    ) -> Result<(Vec<Folder>, Option<usize>), DomainError> {
        FolderRepository::list_folders_paginated(self, parent_id, offset, limit, include_total)
            .await
            .map_err(DomainError::from)
    }
}

#[async_trait]
impl FolderRepository for FolderFsRepository {
    #[instrument(skip(self))]
    async fn move_to_trash(&self, folder_id: &str) -> FolderRepositoryResult<()> {
        // Use the private implementation from folder_fs_repository_trash.rs
        self._trash_move_to_trash(folder_id).await
    }
    
    #[instrument(skip(self))]
    async fn restore_from_trash(&self, folder_id: &str, original_path: &str) -> FolderRepositoryResult<()> {
        // Use the private implementation from folder_fs_repository_trash.rs
        self._trash_restore_from_trash(folder_id, original_path).await
    }
    
    #[instrument(skip(self))]
    async fn delete_folder_permanently(&self, folder_id: &str) -> FolderRepositoryResult<()> {
        // Use the private implementation from folder_fs_repository_trash.rs
        self._trash_delete_folder_permanently(folder_id).await
    }
    async fn create_folder(&self, name: String, parent_id: Option<String>) -> FolderRepositoryResult<Folder> {
        // Get the parent folder path (if any)
        let parent_storage_path = match &parent_id {
            Some(id) => {
                match self.get_folder_storage_path(id).await {
                    Ok(path) => {
                        tracing::info!("Using folder path: {:?} for parent_id: {:?}", path.to_string(), id);
                        Some(path)
                    },
                    Err(e) => {
                        tracing::error!("Error getting parent folder: {}", e);
                        return Err(e);
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
        if self.folder_exists_at_storage_path(&folder_storage_path).await? {
            return Err(FolderRepositoryError::AlreadyExists(folder_storage_path.to_string()));
        }
        
        // Create the physical directory
        let abs_path = self.resolve_storage_path(&folder_storage_path);
        self.create_directory(&abs_path).await
            .map_err(FolderRepositoryError::IoError)?;
        
        // Create and return the folder entity with a persisted ID
        let id = self.id_mapping_service.get_or_create_id(&folder_storage_path).await?;
        let folder = self.create_folder_entity(
            id.clone(), // Clone for logging
            name.clone(), // Clone name for logging
            folder_storage_path.clone(), // Clone for logging
            parent_id.clone(), // Clone for logging
            None,
            None,
        ).await?;
        
        // Ensure ID mapping is persisted - this is critical for later retrieval
        // Attempt to save changes and retry if necessary
        let mut save_result = self.id_mapping_service.save_changes().await;
        
        // Si hay un error, intentamos un segundo intento con mensaje explícito
        if let Err(e) = &save_result {
            tracing::error!("Failed to save ID mapping for folder {}: {} - will retry", id, e);
            
            // Esperar un poco antes de reintentar
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            // Reintentar guardado
            save_result = self.id_mapping_service.save_changes().await;
            
            match &save_result {
                Ok(_) => {
                    tracing::info!("Successfully saved ID mapping for folder on retry: {} -> path: {} (name: {})", 
                        id, folder_storage_path.to_string(), name);
                },
                Err(e) => {
                    tracing::error!("Failed to save ID mapping for folder on retry {}: {}", id, e);
                }
            }
        } else {
            tracing::info!("Successfully saved ID mapping for folder ID: {} -> path: {} (name: {})", 
                id, folder_storage_path.to_string(), name);
        }
        
        // Propagate error if both attempts failed
        save_result?;
        
        tracing::debug!("Created folder with ID: {}", folder.id());
        Ok(folder)
    }
    
    async fn get_folder_by_id(&self, id: &str) -> FolderRepositoryResult<Folder> {
        tracing::debug!("Looking for folder with ID: {}", id);
        
        // Find path by ID using the mapping service
        let storage_path = self.id_mapping_service.get_path_by_id(id).await
            .map_err(FolderRepositoryError::from)?;
        
        // Check if folder exists physically
        let abs_path = self.resolve_storage_path(&storage_path);
        if !abs_path.exists() || !abs_path.is_dir() {
            tracing::error!("Folder not found at path: {}", abs_path.display());
            return Err(FolderRepositoryError::NotFound(format!("Folder {} not found at {}", id, storage_path.to_string())));
        }
        
        // Get folder metadata
        let (created_at, modified_at) = self.get_folder_metadata(&abs_path).await?;
        
        // Get folder name from the storage path
        let name = match storage_path.file_name() {
            Some(name) => name,
            None => {
                tracing::error!("Invalid folder path: {}", storage_path.to_string());
                return Err(FolderRepositoryError::InvalidPath(storage_path.to_string()));
            }
        };
        
        // Determine parent ID if any
        let parent = storage_path.parent();
        let parent_id: Option<String> = if parent.is_none() || parent.as_ref().unwrap().is_empty() {
            None // Root folder
        } else {
            // Try to get the parent ID from the mapping service
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
        ).await?;
        
        Ok(folder)
    }
    
    async fn get_folder_by_storage_path(&self, storage_path: &StoragePath) -> FolderRepositoryResult<Folder> {
        // Check if the physical directory exists
        let abs_path = self.resolve_storage_path(storage_path);
        if !abs_path.exists() || !abs_path.is_dir() {
            return Err(FolderRepositoryError::NotFound(storage_path.to_string()));
        }
        
        // Extract folder name from storage path
        let name = match storage_path.file_name() {
            Some(name) => name,
            None => {
                return Err(FolderRepositoryError::InvalidPath(storage_path.to_string()));
            }
        };
        
        // Determine parent ID if any
        let parent = storage_path.parent();
        let parent_id: Option<String> = if parent.is_none() || parent.as_ref().unwrap().is_empty() {
            None // Root folder
        } else {
            // Try to get the parent ID from the mapping service
            match self.id_mapping_service.get_or_create_id(parent.as_ref().unwrap()).await {
                Ok(pid) => Some(pid),
                Err(_) => None,
            }
        };
        
        // Get folder metadata
        let (created_at, modified_at) = self.get_folder_metadata(&abs_path).await?;
        
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
        ).await?;
        
        // Ensure ID mapping is persisted
        self.id_mapping_service.save_changes().await?;
        
        Ok(folder)
    }
    
    async fn list_folders(&self, parent_id: Option<&str>) -> FolderRepositoryResult<Vec<Folder>> {
        use futures::stream::{StreamExt};
        use tokio::time::{timeout, Duration};
        
        tracing::info!("Listing folders in parent_id: {:?}", parent_id);
        
        // Get the parent storage path
        let parent_storage_path = match parent_id {
            Some(id) => {
                match self.get_folder_storage_path(id).await {
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
        
        // Read the directory with a timeout to avoid indefinite blocking
        let read_dir_timeout = Duration::from_secs(30);
        let read_dir_result = match timeout(
            read_dir_timeout,
            fs::read_dir(&abs_parent_path)
        ).await {
            Ok(result) => result.map_err(FolderRepositoryError::IoError)?,
            Err(_) => {
                return Err(FolderRepositoryError::Other(
                    format!("Timeout reading directory: {}", abs_parent_path.display())
                ));
            }
        };
        
        // Process each entry sequentially to avoid async block type issues
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
                    tracing::error!("Error getting metadata for {}: {}", 
                                   entry.path().display(), err);
                    continue;
                }
            };
            
            // Skip if not a directory
            if !metadata.is_dir() {
                continue;
            }
            
            let folder_name = entry.file_name().to_string_lossy().to_string();
            
            // Create the storage path for this folder
            let folder_storage_path = parent_storage_path.join(&folder_name);
            
            // Try to get the folder by its storage path with timeout
            let get_folder_timeout = Duration::from_secs(5);
            let folder_result = timeout(
                get_folder_timeout,
                self.get_folder_by_storage_path(&folder_storage_path)
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
        
        // Persist any new ID mappings that were created
        if !folders.is_empty() {
            // Attempt to save changes with retry
            let mut save_result = self.id_mapping_service.save_changes().await;
            
            // Si hay un error, intentamos un segundo intento con mensaje explícito
            if let Err(e) = &save_result {
                tracing::error!("Failed to save ID mappings in list_folders: {} - will retry", e);
                
                // Esperar un poco antes de reintentar
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                
                // Reintentar guardado
                save_result = self.id_mapping_service.save_changes().await;
                
                match &save_result {
                    Ok(_) => {
                        tracing::info!("Successfully saved ID mappings in list_folders on retry");
                    },
                    Err(e) => {
                        tracing::error!("Failed to save ID mappings in list_folders on retry: {}", e);
                    }
                }
            } else {
                tracing::info!("Successfully saved ID mappings in list_folders");
            }
            
            // Log error but don't propagate since this is not critical for listing
            if let Err(e) = save_result {
                tracing::error!("Failed to save ID mappings after retries: {}", e);
            }
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
    ) -> FolderRepositoryResult<(Vec<Folder>, Option<usize>)> {
        use futures::stream::StreamExt;
        use tokio::time::{timeout, Duration};
        
        tracing::info!("Listing folders in parent_id: {:?} with pagination (offset={}, limit={})", 
            parent_id, offset, limit);
        
        // Get the parent storage path
        let parent_storage_path = match parent_id {
            Some(id) => {
                match self.get_folder_storage_path(id).await {
                    Ok(path) => {
                        tracing::info!("Found parent folder with path: {:?}", path.to_string());
                        path
                    },
                    Err(e) => {
                        tracing::error!("Error getting parent folder by ID: {}: {}", id, e);
                        return Ok((Vec::new(), Some(0)));
                    },
                }
            },
            None => StoragePath::root(),
        };
        
        // Get the absolute folder path
        let abs_parent_path = self.resolve_storage_path(&parent_storage_path);
        tracing::info!("Absolute parent path: {:?}", abs_parent_path);
        
        // Ensure the directory exists
        if !abs_parent_path.exists() || !abs_parent_path.is_dir() {
            tracing::error!("Directory does not exist or is not a directory: {:?}", abs_parent_path);
            return Ok((Vec::new(), Some(0)));
        }
        
        // Get total count if requested
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
        
        // Read the directory with a timeout to avoid indefinite blocking
        let read_dir_timeout = Duration::from_secs(30);
        let read_dir_result = match timeout(
            read_dir_timeout,
            fs::read_dir(&abs_parent_path)
        ).await {
            Ok(result) => result.map_err(FolderRepositoryError::IoError)?,
            Err(_) => {
                return Err(FolderRepositoryError::Other(
                    format!("Timeout reading directory: {}", abs_parent_path.display())
                ));
            }
        };
        
        // Process entries sequentially to avoid async block typing issues
        let mut entries = tokio_stream::wrappers::ReadDirStream::new(read_dir_result);
        let mut folders = Vec::new();
        let mut current_idx = 0;
        
        // Loop through entries, applying pagination manually
        while let Some(entry_result) = entries.next().await {
            // Skip entries before offset
            if current_idx < offset {
                current_idx += 1;
                continue;
            }
            
            // Stop after reaching limit
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
            
            // Check if it's a directory
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
            
            // Get the path and convert to StoragePath
            let path = entry.path();
            let rel_path = match path.strip_prefix(&self.root_path) {
                Ok(rel) => StoragePath::from(rel.to_path_buf()),
                Err(_) => {
                    tracing::error!("Error stripping prefix from path: {}", path.display());
                    current_idx += 1;
                    continue;
                }
            };
            
            // Get the folder entity with timeout
            let folder_result = timeout(
                Duration::from_secs(10),
                self.get_folder_by_storage_path(&rel_path)
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
        
        // Save ID mappings
        if !folders.is_empty() {
            if let Err(e) = self.id_mapping_service.save_changes().await {
                tracing::error!("Error saving ID mappings: {}", e);
            }
        }
        
        tracing::info!("Found {} folders in paginated request", folders.len());
        
        Ok((folders, total_count))
    }
    
    async fn rename_folder(&self, id: &str, new_name: String) -> FolderRepositoryResult<Folder> {
        // Get the original folder
        let original_folder = self.get_folder_by_id(id).await?;
        tracing::debug!("Renaming folder with ID: {}, Name: {}", id, original_folder.name());
        
        // Create an immutable new version of the folder with updated name
        let renamed_folder = original_folder.with_name(new_name)
            .map_err(|e| FolderRepositoryError::ValidationError(e.to_string()))?;
        
        // Check if target already exists
        if self.folder_exists_at_storage_path(renamed_folder.storage_path()).await? {
            return Err(FolderRepositoryError::AlreadyExists(renamed_folder.storage_path().to_string()));
        }
        
        // Rename the physical directory
        let abs_old_path = self.resolve_storage_path(original_folder.storage_path());
        let abs_new_path = self.resolve_storage_path(renamed_folder.storage_path());
        
        fs::rename(&abs_old_path, &abs_new_path).await
            .map_err(FolderRepositoryError::IoError)?;
            
        // Update the ID mapping
        self.id_mapping_service.update_path(id, renamed_folder.storage_path()).await
            .map_err(FolderRepositoryError::from)?;
        
        // Save the updated mappings
        self.id_mapping_service.save_changes().await?;
        
        tracing::debug!("Folder renamed successfully: ID={}, New name={}", id, renamed_folder.name());
        Ok(renamed_folder)
    }
    
    async fn move_folder(&self, id: &str, new_parent_id: Option<&str>) -> FolderRepositoryResult<Folder> {
        // Get the original folder
        let original_folder = self.get_folder_by_id(id).await?;
        tracing::debug!("Moving folder with ID: {}, Name: {}", id, original_folder.name());
        
        // If the target parent is the same as current, no need to move
        if original_folder.parent_id() == new_parent_id {
            tracing::info!("Folder is already in the target parent, no need to move");
            return Ok(original_folder);
        }
        
        // Get the target parent path
        let target_parent_storage_path = match new_parent_id {
            Some(parent_id) => {
                match self.get_folder_storage_path(parent_id).await {
                    Ok(path) => Some(path),
                    Err(e) => {
                        return Err(FolderRepositoryError::Other(
                            format!("Could not get target folder: {}", e)
                        ));
                    }
                }
            },
            None => None
        };
        
        // Create an immutable new version of the folder with updated parent
        let new_parent_id_option = new_parent_id.map(String::from);
        let moved_folder = original_folder.with_parent(new_parent_id_option, target_parent_storage_path)
            .map_err(|e| FolderRepositoryError::ValidationError(e.to_string()))?;
        
        // Check if target already exists
        if self.folder_exists_at_storage_path(moved_folder.storage_path()).await? {
            return Err(FolderRepositoryError::AlreadyExists(
                format!("Folder already exists at destination: {}", moved_folder.storage_path().to_string())
            ));
        }
        
        // Move the physical directory
        let old_abs_path = self.resolve_storage_path(original_folder.storage_path());
        let new_abs_path = self.resolve_storage_path(moved_folder.storage_path());
        
        // Ensure the target directory exists
        if let Some(parent) = new_abs_path.parent() {
            fs::create_dir_all(parent).await
                .map_err(FolderRepositoryError::IoError)?;
        }
        
        // Move the directory physically (efficient rename operation)
        fs::rename(&old_abs_path, &new_abs_path).await
            .map_err(FolderRepositoryError::IoError)?;
            
        tracing::info!("Folder moved successfully from {:?} to {:?}", old_abs_path, new_abs_path);
        
        // Update the ID mapping
        self.id_mapping_service.update_path(id, moved_folder.storage_path()).await
            .map_err(FolderRepositoryError::from)?;
        
        // Save the updated mappings
        self.id_mapping_service.save_changes().await?;
        
        tracing::debug!("Folder moved successfully: ID={}, New path={:?}", id, moved_folder.storage_path().to_string());
        Ok(moved_folder)
    }
    
    async fn delete_folder(&self, id: &str) -> FolderRepositoryResult<()> {
        use tokio::time::{timeout, Duration};
        
        // Get the folder first to check if it exists
        let folder = self.get_folder_by_id(id).await?;
        let folder_name = folder.name().to_string();
        let storage_path = folder.storage_path().clone();
        
        tracing::info!("Deleting folder with ID: {}, Name: {}", id, folder_name);
        
        // Para carpetas grandes, eliminar puede tomar tiempo
        // Lo manejamos en un task separado para no bloquear
        let abs_path = self.resolve_storage_path(&storage_path);
        
        // Si la carpeta contiene muchos archivos, remove_dir_all puede tardar
        // usamos tokio::spawn para hacerlo en un task separado
        let path_for_display = abs_path.display().to_string();
        let path_for_deletion = abs_path.clone();
        
        let delete_task = tokio::spawn(async move {
            tracing::debug!("Starting removal of folder: {}", path_for_display);
            
            // Verificar si la carpeta existe y tiene muchas entradas
            let path_for_counting = path_for_deletion.clone();
            let entry_count = tokio::task::spawn_blocking(move || {
                let mut count = 0;
                if let Ok(entries) = std::fs::read_dir(&path_for_counting) {
                    for _ in entries {
                        count += 1;
                        if count > 1000 {
                            break; // Solo necesitamos saber si es grande
                        }
                    }
                }
                count
            }).await.unwrap_or(0);
            
            // Para carpetas muy grandes, usar remove_dir_all puede causar bloqueos
            // Para carpetas pequeñas, usamos la versión asíncrona estándar
            if entry_count > 1000 {
                tracing::info!("Large folder detected with >1000 entries, using blocking removal");
                
                // Para carpetas muy grandes, usamos spawn_blocking para no bloquear el runtime de tokio
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
                tracing::debug!("Using async removal for folder with {} entries", entry_count);
                fs::remove_dir_all(&path_for_deletion).await
            }
        });
        
        // Esperar a que termine la eliminación con timeout
        const DELETE_TIMEOUT_SECS: u64 = 60; // 1 minuto máximo para eliminar
        
        let delete_result = timeout(
            Duration::from_secs(DELETE_TIMEOUT_SECS), 
            delete_task
        ).await;
        
        match delete_result {
            Ok(task_result) => {
                match task_result {
                    Ok(fs_result) => {
                        if let Err(e) = fs_result {
                            return Err(FolderRepositoryError::IoError(e));
                        }
                    },
                    Err(join_err) => {
                        return Err(FolderRepositoryError::Other(
                            format!("Task panicked during folder deletion: {}", join_err)
                        ));
                    }
                }
            },
            Err(_) => {
                // El timeout ocurrió, pero la tarea sigue ejecutándose en segundo plano
                tracing::warn!("Timeout waiting for folder deletion, continuing with ID removal");
                // No retornamos error, continuamos con la eliminación del ID
            }
        }
        
        // Incluso si la eliminación física puede estar en progreso (timeout),
        // procedemos a eliminar la entrada del mapping
        // En el peor caso, si la eliminación física falla pero el ID se elimina,
        // la carpeta se quedará huérfana, pero no afectará al sistema
        
        // Remove the ID mapping con timeout
        const MAPPING_TIMEOUT_SECS: u64 = 5;
        let remove_id_result = timeout(
            Duration::from_secs(MAPPING_TIMEOUT_SECS),
            self.id_mapping_service.remove_id(id)
        ).await;
        
        match remove_id_result {
            Ok(result) => result.map_err(FolderRepositoryError::from)?,
            Err(_) => {
                return Err(FolderRepositoryError::Other(
                    "Timeout removing folder ID from mapping".to_string()
                ));
            }
        }
        
        // Save the updated mappings (asíncrono, no esperamos)
        let _ = self.id_mapping_service.save_changes().await;
        
        tracing::info!("Folder deleted successfully: ID={}, Name={}", id, folder_name);
        Ok(())
    }
    
    async fn folder_exists_at_storage_path(&self, storage_path: &StoragePath) -> FolderRepositoryResult<bool> {
        self.check_folder_exists_at_storage_path(storage_path).await
    }
    
    async fn get_folder_storage_path(&self, id: &str) -> FolderRepositoryResult<StoragePath> {
        // Use the ID mapping service to get the storage path
        let storage_path = self.id_mapping_service.get_path_by_id(id).await
            .map_err(FolderRepositoryError::from)?;
        
        Ok(storage_path)
    }
    
    // Legacy method implementations
    
    async fn folder_exists(&self, path: &std::path::PathBuf) -> FolderRepositoryResult<bool> {
        let abs_path = self.resolve_legacy_path(path);
        Ok(abs_path.exists() && abs_path.is_dir())
    }
    
    async fn get_folder_by_path(&self, path: &std::path::PathBuf) -> FolderRepositoryResult<Folder> {
        // Convert PathBuf to StoragePath
        let path_str = path.to_string_lossy().to_string();
        let storage_path = StoragePath::from_string(&path_str);
        
        // Use the new method
        self.get_folder_by_storage_path(&storage_path).await
    }
}