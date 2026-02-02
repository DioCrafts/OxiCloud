use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use tokio::{fs, io::AsyncWriteExt, time};
use tokio::fs::File as TokioFile;
use tokio_util::codec::{BytesCodec, FramedRead};
use tracing::instrument;
use mime_guess::from_path;
use futures::{Stream, StreamExt};
use bytes::Bytes;
use uuid::Uuid;
use tokio::task;

use crate::infrastructure::services::file_system_utils::FileSystemUtils;

use crate::domain::entities::file::File;
use crate::domain::repositories::file_repository::{
    FileRepository, FileRepositoryError, FileRepositoryResult
};
use crate::application::services::storage_mediator::StorageMediator;
// use crate::application::ports::outbound::IdMappingPort;
use crate::infrastructure::services::id_mapping_service::IdMappingError;
use crate::infrastructure::services::file_metadata_cache::{FileMetadataCache, CacheEntryType};
use crate::domain::services::path_service::StoragePath;
use crate::infrastructure::services::path_service::PathService;
use crate::common::errors::DomainError;
use crate::common::config::AppConfig;
use crate::application::ports::outbound::FileStoragePort;
use crate::infrastructure::repositories::parallel_file_processor::ParallelFileProcessor;

/**
 * Filesystem implementation of the File Repository interface.
 * 
 * This repository provides a concrete implementation of the FileRepository domain interface
 * that interacts with a filesystem-based storage backend. It implements:
 * 
 * 1. File creation, retrieval, and deletion operations
 * 2. File content reading (both in-memory and streaming)
 * 3. Folder organization for files
 * 4. ID-to-path mapping persistence
 * 5. Optimized handling of large files using parallel I/O
 * 6. Metadata caching to reduce filesystem operations
 * 7. Trash operations for file lifecycle management
 * 
 * The implementation follows the hexagonal architecture pattern as a secondary adapter,
 * implementing domain interfaces and ports while isolating the application core from
 * filesystem-specific details.
 */

// Use constants from centralized configuration instead of fixed values
// This is replaced with self.config.concurrency.max_concurrent_files later

/// Filesystem implementation of the FileRepository interface
pub struct FileFsRepository {
    root_path: PathBuf,
    storage_mediator: Arc<dyn StorageMediator>,
    id_mapping_service: Arc<dyn crate::application::ports::outbound::IdMappingPort>,
    path_service: Arc<PathService>,
    metadata_cache: Arc<FileMetadataCache>,
    config: AppConfig,
    parallel_processor: Option<Arc<ParallelFileProcessor>>,
}

impl FileFsRepository {
    /// Creates a new filesystem-based file repository
    pub fn new(
        root_path: PathBuf, 
        storage_mediator: Arc<dyn StorageMediator>,
        id_mapping_service: Arc<dyn crate::application::ports::outbound::IdMappingPort>,
        path_service: Arc<PathService>,
        metadata_cache: Arc<FileMetadataCache>,
    ) -> Self {
        Self { 
            root_path, 
            storage_mediator, 
            id_mapping_service,
            path_service,
            metadata_cache,
            config: AppConfig::default(),
            parallel_processor: None,
        }
    }
    
    /// Creates a new repository with a pre-configured parallel file processor
    pub fn new_with_processor(
        root_path: PathBuf, 
        storage_mediator: Arc<dyn StorageMediator>,
        id_mapping_service: Arc<dyn crate::application::ports::outbound::IdMappingPort>,
        path_service: Arc<PathService>,
        metadata_cache: Arc<FileMetadataCache>,
        parallel_processor: Arc<ParallelFileProcessor>,
    ) -> Self {
        Self { 
            root_path, 
            storage_mediator, 
            id_mapping_service,
            path_service,
            metadata_cache,
            config: AppConfig::default(),
            parallel_processor: Some(parallel_processor),
        }
    }
    
    /// Resolves a domain storage path to an absolute filesystem path
    fn resolve_storage_path(&self, storage_path: &StoragePath) -> PathBuf {
        self.path_service.resolve_path(storage_path)
    }
    
    /// Resolves a legacy PathBuf to an absolute filesystem path
    #[allow(dead_code)]
    fn resolve_legacy_path(&self, relative_path: &std::path::Path) -> PathBuf {
        self.storage_mediator.resolve_path(relative_path)
    }
    
    /// Returns a reference to the ID mapping service
    pub fn id_mapping_service(&self) -> &Arc<dyn crate::application::ports::outbound::IdMappingPort> {
        &self.id_mapping_service
    }
    
    /// Returns a reference to the metadata cache
    pub fn metadata_cache(&self) -> &Arc<FileMetadataCache> {
        &self.metadata_cache
    }
    
    /// Returns a reference to the root path
    pub fn get_root_path(&self) -> &PathBuf {
        &self.root_path
    }
    
    /// Checks if a file exists at a given storage path
    async fn file_exists_at_storage_path(&self, storage_path: &StoragePath) -> FileRepositoryResult<bool> {
        let abs_path = self.resolve_storage_path(storage_path);
        
        // Try to get from advanced cache first
        if let Some(is_file) = self.metadata_cache.is_file(&abs_path).await {
            tracing::debug!("Metadata cache hit for existence check: {} - path: {}", is_file, abs_path.display());
            return Ok(is_file);
        }
        
        // If not in cache, verify directly and update cache
        tracing::debug!("Metadata cache miss for existence check: {}", abs_path.display());
        
        // Use timeout to avoid blocking
        match time::timeout(
            self.config.timeouts.file_timeout(),
            fs::metadata(&abs_path)
        ).await {
            Ok(Ok(metadata)) => {
                let is_file = metadata.is_file();
                
                // Update cache with fresh information
                if let Err(e) = self.metadata_cache.refresh_metadata(&abs_path).await {
                    tracing::warn!("Failed to update cache for {}: {}", abs_path.display(), e);
                }
                
                if is_file {
                    tracing::debug!("File exists and is accessible: {}", abs_path.display());
                    Ok(true)
                } else {
                    tracing::warn!("Path exists but is not a file: {}", abs_path.display());
                    Ok(false)
                }
            },
            Ok(Err(e)) => {
                tracing::warn!("File check failed: {} - {}", abs_path.display(), e);
                
                // Add to cache as non-existent
                let entry_type = CacheEntryType::Unknown;
                let file_metadata = crate::infrastructure::services::file_metadata_cache::FileMetadata::new(
                    abs_path.clone(),
                    false,
                    entry_type,
                    None,
                    None,
                    None,
                    None,
                    Duration::from_millis(self.config.timeouts.file_operation_ms),
                );
                self.metadata_cache.update_cache(file_metadata).await;
                
                Ok(false)
            },
            Err(_) => {
                tracing::warn!("Timeout checking file metadata: {}", abs_path.display());
                return Err(FileRepositoryError::Timeout(format!("Timeout checking file: {}", abs_path.display())));
            }
        }
    }
    
    /// Legacy method for checking file existence with PathBuf
    #[allow(dead_code)]
    pub async fn file_exists(&self, path: &std::path::Path) -> FileRepositoryResult<bool> {
        let abs_path = self.resolve_legacy_path(path);
        
        // Try to get from advanced cache first
        if let Some(is_file) = self.metadata_cache.is_file(&abs_path).await {
            tracing::debug!("Metadata cache hit for legacy existence check: {} - path: {}", is_file, abs_path.display());
            return Ok(is_file);
        }
        
        // If not in cache, verify directly
        tracing::info!("Checking if file exists: {} - path: {}", abs_path.exists(), abs_path.display());
        
        match time::timeout(
            self.config.timeouts.file_timeout(),
            fs::metadata(&abs_path)
        ).await {
            Ok(Ok(metadata)) => {
                let is_file = metadata.is_file();
                
                // Update cache with fresh information
                if let Err(e) = self.metadata_cache.refresh_metadata(&abs_path).await {
                    tracing::warn!("Failed to update cache for {}: {}", abs_path.display(), e);
                }
                
                if is_file {
                    tracing::info!("File exists and is accessible: {}", abs_path.display());
                    return Ok(true);
                } else {
                    tracing::warn!("Path exists but is not a file: {}", abs_path.display());
                    return Ok(false);
                }
            },
            Ok(Err(e)) => {
                tracing::warn!("File exists but metadata check failed: {} - {}", abs_path.display(), e);
                return Ok(false);
            },
            Err(_) => {
                tracing::warn!("Timeout checking file metadata: {}", abs_path.display());
                return Err(FileRepositoryError::Timeout(format!("Timeout checking file: {}", abs_path.display())));
            }
        }
    }
    
    /// Helper method to create a File entity from a storage path and metadata
    async fn create_file_entity(
        &self,
        id: String,
        name: String,
        storage_path: StoragePath,
        size: u64,
        mime_type: String,
        folder_id: Option<String>,
        created_at: Option<u64>,
        modified_at: Option<u64>,
    ) -> FileRepositoryResult<File> {
        // If timestamps are provided, use them; otherwise, let File::new create default timestamps
        if let (Some(created), Some(modified)) = (created_at, modified_at) {
            File::with_timestamps(
                id, 
                name, 
                storage_path, 
                size, 
                mime_type, 
                folder_id,
                created,
                modified,
            )
            .map_err(|e| FileRepositoryError::Other(e.to_string()))
        } else {
            File::new(
                id, 
                name, 
                storage_path, 
                size, 
                mime_type, 
                folder_id,
            )
            .map_err(|e| FileRepositoryError::Other(e.to_string()))
        }
    }
    
    /// Extracts file metadata from a physical path with timeout and cache
    async fn get_file_metadata(&self, abs_path: &PathBuf) -> FileRepositoryResult<(u64, u64, u64)> {
        // Try to get from cache first
        if let Some(cached_metadata) = self.metadata_cache.get_metadata(abs_path).await {
            if let (Some(size), Some(created_at), Some(modified_at)) = 
                (cached_metadata.size, cached_metadata.created_at, cached_metadata.modified_at) {
                tracing::debug!("Using cached metadata for: {}", abs_path.display());
                return Ok((size, created_at, modified_at));
            }
        }
        
        // If not in cache or incomplete metadata, load from filesystem
        let metadata = match time::timeout(
            self.config.timeouts.file_timeout(),
            fs::metadata(&abs_path)
        ).await {
            Ok(Ok(metadata)) => metadata,
            Ok(Err(e)) => return Err(FileRepositoryError::IoError(e)),
            Err(_) => return Err(FileRepositoryError::Timeout(
                format!("Timeout getting metadata for: {}", abs_path.display())
            )),
        };
        
        let size = metadata.len();
        
        // Get creation timestamp
        let created_at = metadata.created()
            .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
            .unwrap_or_else(|_| 0);
            
        // Get modification timestamp
        let modified_at = metadata.modified()
            .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
            .unwrap_or_else(|_| 0);
        
        // Update cache if possible
        if let Err(e) = self.metadata_cache.refresh_metadata(abs_path).await {
            tracing::warn!("Failed to update metadata cache for {}: {}", abs_path.display(), e);
        }
            
        Ok((size, created_at, modified_at))
    }
    
    /// Creates parent directories if needed with timeout and fsync
    async fn ensure_parent_directory(&self, abs_path: &PathBuf) -> FileRepositoryResult<()> {
        if let Some(parent) = abs_path.parent() {
            time::timeout(
                self.config.timeouts.dir_timeout(),
                FileSystemUtils::create_dir_with_sync(parent)
            ).await
            .map_err(|_| FileRepositoryError::Timeout(
                format!("Timeout creating parent directory: {}", parent.display())
            ))?
            .map_err(FileRepositoryError::IoError)?;
        }
        Ok(())
    }
    
    /// Check if a file is large based on size threshold from config
    async fn is_large_file(&self, abs_path: &PathBuf) -> FileRepositoryResult<bool> {
        if !abs_path.exists() {
            return Ok(false);
        }
        
        let metadata = time::timeout(
            self.config.timeouts.file_timeout(),
            fs::metadata(&abs_path)
        ).await
        .map_err(|_| FileRepositoryError::Timeout(format!("Timeout checking file size: {}", abs_path.display())))?
        .map_err(FileRepositoryError::IoError)?;
            
        // Use the ResourceConfig method to determine if it's a large file
        Ok(self.config.resources.is_large_file(metadata.len()))
    }
    
    /// Non-blocking file deletion for large files
    async fn delete_file_non_blocking(&self, abs_path: PathBuf) -> FileRepositoryResult<()> {
        // Check if file is large enough to warrant spawn_blocking
        let is_large = self.is_large_file(&abs_path).await?;
        
        if is_large {
            tracing::info!("Using non-blocking deletion for large file: {}", abs_path.display());
            
            // Use spawn_blocking for large files to prevent blocking the runtime
            task::spawn_blocking(move || {
                // Use standard library's blocking remove_file
                match std::fs::remove_file(&abs_path) {
                    Ok(_) => tracing::info!("Successfully deleted large file: {}", abs_path.display()),
                    Err(e) => tracing::error!("Failed to delete large file: {} - {}", abs_path.display(), e),
                }
            }).await
            .map_err(|e| FileRepositoryError::Other(format!("Join error in spawn_blocking: {}", e)))?;
        } else {
            // For smaller files use tokio's async version
            time::timeout(
                self.config.timeouts.file_timeout(),
                fs::remove_file(&abs_path)
            ).await
            .map_err(|_| FileRepositoryError::Timeout(format!("Timeout deleting file: {}", abs_path.display())))?
            .map_err(FileRepositoryError::IoError)?;
        }
        
        Ok(())
    }
}

// Convert IdMappingError to FileRepositoryError
impl From<IdMappingError> for FileRepositoryError {
    fn from(err: IdMappingError) -> Self {
        match err {
            IdMappingError::NotFound(id) => FileRepositoryError::NotFound(id),
            IdMappingError::IoError(e) => FileRepositoryError::IoError(e),
            IdMappingError::Timeout(msg) => FileRepositoryError::Timeout(msg),
            _ => FileRepositoryError::Other(err.to_string()),
        }
    }
}

// Errors are already defined by the FileRepositoryError interface

// Enable cloning for concurrent operations
impl Clone for FileFsRepository {
    fn clone(&self) -> Self {
        Self {
            root_path: self.root_path.clone(),
            storage_mediator: self.storage_mediator.clone(),
            id_mapping_service: self.id_mapping_service.clone(),
            path_service: self.path_service.clone(),
            metadata_cache: self.metadata_cache.clone(),
            config: self.config.clone(),
            parallel_processor: self.parallel_processor.clone(),
        }
    }
}

#[async_trait]
impl FileStoragePort for FileFsRepository {
    async fn save_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<File, DomainError> {
        self.save_file_from_bytes(name, folder_id, content_type, content)
            .await
            .map_err(|e| DomainError::internal_error("FileStorage", format!("Failed to save file: {}", e)))
    }
    
    async fn get_file(&self, id: &str) -> Result<File, DomainError> {
        self.get_file_by_id(id)
            .await
            .map_err(|e| DomainError::internal_error("FileStorage", format!("Failed to get file with ID: {}: {}", id, e)))
    }
    
    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<File>, DomainError> {
        FileRepository::list_files(self, folder_id)
            .await
            .map_err(|e| DomainError::internal_error("FileStorage", format!("Failed to list files in folder: {:?}: {}", folder_id, e)))
    }
    
    async fn delete_file(&self, id: &str) -> Result<(), DomainError> {
        FileRepository::delete_file(self, id)
            .await
            .map_err(|e| DomainError::internal_error("FileStorage", format!("Failed to delete file with ID: {}: {}", id, e)))
    }
    
    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError> {
        FileRepository::get_file_content(self, id)
            .await
            .map_err(|e| DomainError::internal_error("FileStorage", format!("Failed to get content for file with ID: {}: {}", id, e)))
    }
    
    async fn get_file_stream(&self, id: &str) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        FileRepository::get_file_stream(self, id)
            .await
            .map_err(|e| DomainError::internal_error("FileStorage", format!("Failed to get stream for file with ID: {}: {}", id, e)))
    }
    
    async fn move_file(&self, file_id: &str, target_folder_id: Option<String>) -> Result<File, DomainError> {
        // Clone target_folder_id before passing to avoid ownership issues
        let cloned_target = target_folder_id.clone();
        let result = FileRepository::move_file(self, file_id, target_folder_id)
            .await;
            
        result.map_err(|e| DomainError::internal_error("FileStorage", format!("Failed to move file with ID: {} to folder: {:?}: {}", file_id, cloned_target, e)))
    }
    
    async fn get_file_path(&self, id: &str) -> Result<StoragePath, DomainError> {
        FileRepository::get_file_path(self, id)
            .await
            .map_err(|e| DomainError::internal_error("FileStorage", format!("Failed to get path for file with ID: {}: {}", id, e)))
    }
    
    async fn get_parent_folder_id(&self, path: &str) -> Result<String, DomainError> {
        // Convert path string to StoragePath
        let storage_path = StoragePath::from_string(path);
        
        // Get parent path
        let parent_path = match storage_path.parent() {
            Some(parent) => parent,
            None => return Ok("root".to_string()), // Root folder
        };
        
        // If it's an empty path (root), return root ID
        if parent_path.is_empty() {
            return Ok("root".to_string());
        }
        
        // Try to get the ID for the parent path from the ID mapping service
        let parent_id = self.id_mapping_service.get_or_create_id(&parent_path).await
            .map_err(|e| DomainError::internal_error("FileStorage", 
                format!("Failed to get parent folder ID for path: {}: {}", path, e)))?;
                
        Ok(parent_id)
    }
    
    async fn update_file_content(&self, file_id: &str, content: Vec<u8>) -> Result<(), DomainError> {
        // First get the file to make sure it exists and to get its path
        let file = self.get_file_by_id(file_id).await
            .map_err(|e| DomainError::internal_error("FileStorage", 
                format!("Failed to get file for update: {}: {}", file_id, e)))?;
        
        // Get the file path for writing
        let file_path = FileStoragePort::get_file_path(self, file_id).await
            .map_err(|e| DomainError::internal_error("FileStorage", 
                format!("Failed to get file path for update: {}: {}", file_id, e)))?;
        
        // Resolve to actual filesystem path
        let physical_path = self.storage_mediator.resolve_storage_path(&file_path);
        
        // Write the content to the file with fsync
        FileSystemUtils::atomic_write(&physical_path, &content)
            .await
            .map_err(|e| DomainError::internal_error("FileStorage", 
                format!("Failed to write updated content to file: {}: {}", file_id, e)))?;
                
        // Get the metadata and add it to cache if available
        if let Some(metadata) = std::fs::metadata(&physical_path).ok() {
            // Create a FileMetadata instance and update the cache
            use crate::infrastructure::services::file_metadata_cache::FileMetadata;
            use crate::infrastructure::services::file_metadata_cache::CacheEntryType;
            use std::time::UNIX_EPOCH;
            use std::time::Duration;
            
            // Get modified and created times
            let created_at = metadata.created()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs());
                
            let modified_at = metadata.modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs());
                
            // Default TTL
            let ttl = Duration::from_secs(60); // 1 minute
            
            // Create FileMetadata instance
            let file_metadata = FileMetadata::new(
                physical_path.clone(),
                true,  // exists
                CacheEntryType::File,
                Some(metadata.len()),
                Some(file.mime_type().to_string()),
                created_at,
                modified_at,
                ttl,
            );
            
            // Update the cache
            self.metadata_cache.update_cache(file_metadata).await;
        }
        
        Ok(())
    }
}

#[async_trait]
impl FileRepository for FileFsRepository {
    #[instrument(skip(self))]
    async fn move_to_trash(&self, file_id: &str) -> FileRepositoryResult<()> {
        tracing::info!("FileRepository::move_to_trash called for file ID: {}", file_id);
        // Call the internal implementation for trash handling
        match self._trash_move_to_trash(file_id).await {
            Ok(_) => {
                tracing::info!("File successfully moved to trash: {}", file_id);
                Ok(())
            },
            Err(e) => {
                tracing::error!("Failed to move file to trash: {} - {}", file_id, e);
                Err(e)
            }
        }
    }
    
    #[instrument(skip(self))]
    async fn restore_from_trash(&self, file_id: &str, original_path: &str) -> FileRepositoryResult<()> {
        tracing::info!("FileRepository::restore_from_trash called for file ID: {} to path: {}", file_id, original_path);
        match self._trash_restore_from_trash(file_id, original_path).await {
            Ok(_) => {
                tracing::info!("File successfully restored from trash: {}", file_id);
                Ok(())
            },
            Err(e) => {
                tracing::error!("Failed to restore file from trash: {} - {}", file_id, e);
                Err(e)
            }
        }
    }
    
    #[instrument(skip(self))]
    async fn delete_file_permanently(&self, file_id: &str) -> FileRepositoryResult<()> {
        tracing::info!("FileRepository::delete_file_permanently called for file ID: {}", file_id);
        match self._trash_delete_file_permanently(file_id).await {
            Ok(_) => {
                tracing::info!("File permanently deleted successfully: {}", file_id);
                Ok(())
            },
            Err(e) => {
                tracing::error!("Failed to delete file permanently: {} - {}", file_id, e);
                Err(e)
            }
        }
    }
    
    #[instrument(skip(self, content))]
    async fn update_file_content(&self, file_id: &str, content: Vec<u8>) -> FileRepositoryResult<()> {
        tracing::info!("FileRepository::update_file_content called for file ID: {}", file_id);
        
        // Get the file info to verify it exists and get its path
        let file = self.get_file_by_id(file_id).await?;
        
        // Get the file path
        let storage_path = FileRepository::get_file_path(self, file_id).await?;
        let physical_path = self.path_service.resolve_path(&storage_path);
        
        // Write the content to the file with fsync
        FileSystemUtils::atomic_write(&physical_path, &content)
            .await
            .map_err(|e| FileRepositoryError::IoError(e))?;
            
        // Get the metadata and add it to cache if available
        if let Some(metadata) = std::fs::metadata(&physical_path).ok() {
            // Create a FileMetadata instance and update the cache
            use crate::infrastructure::services::file_metadata_cache::FileMetadata;
            use crate::infrastructure::services::file_metadata_cache::CacheEntryType;
            use std::time::UNIX_EPOCH;
            use std::time::Duration;
            
            // Get modified and created times
            let created_at = metadata.created()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs());
                
            let modified_at = metadata.modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs());
                
            // Default TTL
            let ttl = Duration::from_secs(60); // 1 minute
            
            // Create FileMetadata instance
            let file_metadata = FileMetadata::new(
                physical_path.clone(),
                true,  // exists
                CacheEntryType::File,
                Some(metadata.len()),
                Some(file.mime_type().to_string()),
                created_at,
                modified_at,
                ttl,
            );
            
            // Update the cache
            self.metadata_cache.update_cache(file_metadata).await;
        }
        
        tracing::info!("File content updated successfully: {}", file_id);
        Ok(())
    }
    async fn save_file_from_bytes(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> FileRepositoryResult<File>
    {
        // Get the folder path from the mediator
        let folder_path = match &folder_id {
            Some(id) => {
                match self.storage_mediator.get_folder_path(id).await {
                    Ok(path) => {
                        tracing::info!("Using folder path: {:?} for folder_id: {:?}", path, id);
                        // Convert to StoragePath - use just the folder name to avoid path duplication
                        // Get just the folder name to avoid path duplication
                        let lossy = path.to_string_lossy().to_string();
                        let folder_name = path.file_name()
                            .and_then(|f| f.to_str())
                            .unwrap_or_else(|| &lossy);
                        tracing::info!("Using folder name: {} for StoragePath", folder_name);
                        StoragePath::from_string(folder_name)
                    },
                    Err(e) => {
                        tracing::error!("Error getting folder: {}", e);
                        // Root path
                        StoragePath::root()
                    },
                }
            },
            None => StoragePath::root(),
        };
        
        // Create the storage path for the file
        let mut file_storage_path = folder_path.join(&name);
        tracing::info!("Created file path: {:?}", file_storage_path.to_string());
        
        // Check if file already exists and generate a unique name if needed
        let mut exists = self.file_exists_at_storage_path(&file_storage_path).await?;
        tracing::info!("File exists check: {} for path: {:?}", exists, file_storage_path.to_string());
        
        // If file exists, generate a unique name by adding a suffix
        let mut original_name = name.clone();
        let mut counter = 1;
        
        while exists {
            // Extract filename and extension
            let file_stem;
            let extension;
            
            if let Some(dot_pos) = original_name.rfind('.') {
                file_stem = original_name[..dot_pos].to_string();
                extension = original_name[dot_pos..].to_string();
            } else {
                file_stem = original_name.clone();
                extension = "".to_string();
            }
            
            // Create new name with counter
            let new_name = format!("{}_{}{}", file_stem, counter, extension);
            
            // Update the storage path with the new name
            let new_file_storage_path = folder_path.join(&new_name);
            
            // Check if the new path exists
            exists = self.file_exists_at_storage_path(&new_file_storage_path).await?;
            
            if !exists {
                // Update variables for the new path
                tracing::info!("Generated unique name for duplicate file: {} -> {}", original_name, new_name);
                original_name = new_name.clone();
                file_storage_path = new_file_storage_path;
            } else {
                // Try next counter
                counter += 1;
            }
        }
        
        // Create parent directories if they don't exist
        let abs_path = self.resolve_storage_path(&file_storage_path);
        self.ensure_parent_directory(&abs_path).await?;
        
        // Calculate file size
        let content_size = content.len() as u64;
        
        // Verificar si el archivo es muy grande para el procesamiento paralelo de escritura
        if self.config.resources.needs_parallel_processing(content_size, &self.config.concurrency) {
            // Para archivos muy grandes, usar procesador paralelo
            tracing::info!("Using parallel file processor for large file write: {} ({} bytes)", 
                         abs_path.display(), content_size);
            
            // Usar el procesador pre-configurado si está disponible o crear uno nuevo
            let result = if let Some(processor) = &self.parallel_processor {
                tracing::debug!("Using pre-configured parallel processor with buffer pool");
                processor.write_file_parallel(&abs_path, &content).await
            } else {
                tracing::debug!("Creating on-demand parallel processor");
                // Importar y crear el procesador paralelo
                use crate::infrastructure::repositories::parallel_file_processor::ParallelFileProcessor;
                let processor = ParallelFileProcessor::new(self.config.clone());
                
                // Escribir archivo en paralelo
                processor.write_file_parallel(&abs_path, &content).await
            };
            
            // Manejar resultado
            result?;
            
            tracing::info!("Successfully wrote {}MB file using parallel chunks", content_size / (1024 * 1024));
        } else if content_size > self.config.resources.large_file_threshold_mb * 1024 * 1024 {
            // Para archivos grandes pero no tanto como para paralelizar, usar chunking
            let file_creation_result = time::timeout(
                self.config.timeouts.file_timeout(), 
                TokioFile::create(&abs_path)
            ).await
            .map_err(|_| FileRepositoryError::Timeout(format!("Timeout creating file: {}", abs_path.display())))?
            .map_err(FileRepositoryError::IoError)?;
            
            let mut file = file_creation_result;
            
            // Define el tamaño del chunk usando la configuración
            let chunk_size = self.config.resources.chunk_size_bytes;
            
            tracing::info!("Using chunked writing with size {} bytes for file: {} ({} bytes)", 
                         chunk_size, abs_path.display(), content_size);
            
            // Divide el contenido en chunks y escribe cada uno con timeout
            for (i, chunk) in content.chunks(chunk_size).enumerate() {
                let _write_result = time::timeout(
                    self.config.timeouts.file_timeout(),
                    file.write_all(chunk)
                ).await
                .map_err(|_| FileRepositoryError::Timeout(
                    format!("Timeout writing chunk {} to file: {}", i, abs_path.display())
                ))?
                .map_err(FileRepositoryError::IoError)?;
                
                tracing::debug!("Written chunk {} ({} bytes) to file {}", i, chunk.len(), abs_path.display());
            }
            
            // Ensure file is properly flushed and closed
            let _flush_result = time::timeout(
                self.config.timeouts.file_timeout(),
                file.flush()
            ).await
            .map_err(|_| FileRepositoryError::Timeout(format!("Timeout flushing file: {}", abs_path.display())))?
            .map_err(FileRepositoryError::IoError)?;
        } else {
            // Para archivos pequeños, escritura simple
            let file_creation_result = time::timeout(
                self.config.timeouts.file_timeout(), 
                TokioFile::create(&abs_path)
            ).await
            .map_err(|_| FileRepositoryError::Timeout(format!("Timeout creating file: {}", abs_path.display())))?
            .map_err(FileRepositoryError::IoError)?;
            
            let mut file = file_creation_result;
            
            // Para archivos pequeños, escribe todo el contenido de una vez
            let _write_result = time::timeout(
                self.config.timeouts.file_timeout(),
                file.write_all(&content)
            ).await
            .map_err(|_| FileRepositoryError::Timeout(format!("Timeout writing to file: {}", abs_path.display())))?
            .map_err(FileRepositoryError::IoError)?;
            
            // Ensure file is properly flushed and closed
            let _flush_result = time::timeout(
                self.config.timeouts.file_timeout(),
                file.flush()
            ).await
            .map_err(|_| FileRepositoryError::Timeout(format!("Timeout flushing file: {}", abs_path.display())))?
            .map_err(FileRepositoryError::IoError)?;
        }
        
        // Get file metadata
        let (size, created_at, modified_at) = self.get_file_metadata(&abs_path).await?;
            
        // Determine the MIME type
        let mime_type = if content_type.is_empty() {
            from_path(&abs_path)
                .first_or_octet_stream()
                .to_string()
        } else {
            content_type
        };
        
        // Create and return the file entity with a persistent ID
        let id = self.id_mapping_service.get_or_create_id(&file_storage_path).await?;
        
        // Keep a string representation of the path for logging
        let path_string = file_storage_path.to_string();
        
        let file = self.create_file_entity(
            id.clone(), // Clone ID for use in logging
            original_name, // Use the potentially modified name with counter suffix
            file_storage_path,
            size,
            mime_type,
            folder_id,
            Some(created_at),
            Some(modified_at),
        ).await?;
        
        // Ensure ID mapping is persisted - this is critical for later retrieval
        // Ejecutar múltiples intentos de guardado con verificación para garantizar persistencia
        for attempt in 1..=3 {
            match self.id_mapping_service.save_changes().await {
                Ok(_) => {
                    tracing::info!("Successfully saved ID mapping for file ID: {} -> path: {} (attempt {})", id, path_string, attempt);
                    
                    // Verificar que el mapeo se puede recuperar después de guardado
                    if let Ok(verified_path) = self.id_mapping_service.get_path_by_id(&id).await {
                        if verified_path.to_string() == path_string {
                            tracing::info!("Verified ID mapping is retrievable after save: {} -> {}", id, path_string);
                            break; // Guaradado correcto y verificado, salir del bucle
                        } else {
                            tracing::error!("Mapping verification failed: expected {} but got {}", path_string, verified_path.to_string());
                            if attempt < 3 {
                                tracing::info!("Will retry saving ID mapping (attempt {}/3)", attempt + 1);
                                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                continue;
                            } else {
                                return Err(FileRepositoryError::Other(
                                    format!("Failed to verify ID mapping for file: {} after 3 attempts", id)
                                ));
                            }
                        }
                    } else {
                        tracing::error!("Cannot verify mapping, ID {} not found after save", id);
                        if attempt < 3 {
                            tracing::info!("Will retry saving ID mapping (attempt {}/3)", attempt + 1);
                            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                            continue;
                        } else {
                            return Err(FileRepositoryError::Other(
                                format!("Failed to verify ID mapping for file: {} after 3 attempts", id)
                            ));
                        }
                    }
                },
                Err(e) => {
                    tracing::error!("Failed to save ID mapping for file {}: {} (attempt {})", id, e, attempt);
                    if attempt < 3 {
                        tracing::info!("Will retry saving ID mapping (attempt {}/3)", attempt + 1);
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        continue;
                    } else {
                        return Err(FileRepositoryError::Other(
                            format!("Failed to save ID mapping for file: {} after 3 attempts - {}", id, e)
                        ));
                    }
                }
            }
        }
        
        // Invalidate any directory cache entries for the parent folders
        // to ensure directory listings show the new file
        if let Some(parent_dir) = abs_path.parent() {
            self.metadata_cache.invalidate_directory(parent_dir).await;
        }
        
        tracing::info!("Saved file: {} with ID: {}", path_string, file.id());
        Ok(file)
    }
    
    async fn save_file_with_id(
        &self,
        id: String,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> FileRepositoryResult<File>
    {
        // Get the folder path from the mediator
        let folder_path = match &folder_id {
            Some(fid) => {
                match self.storage_mediator.get_folder_path(fid).await {
                    Ok(path) => {
                        tracing::info!("Using folder path: {:?} for folder_id: {:?}", path, fid);
                        // Convert to StoragePath - use just the folder name to avoid path duplication
                        // Get just the folder name to avoid path duplication
                        let lossy = path.to_string_lossy().to_string();
                        let folder_name = path.file_name()
                            .and_then(|f| f.to_str())
                            .unwrap_or_else(|| &lossy);
                        tracing::info!("Using folder name: {} for StoragePath", folder_name);
                        StoragePath::from_string(folder_name)
                    },
                    Err(e) => {
                        tracing::error!("Error getting folder: {}", e);
                        // Root path
                        StoragePath::root()
                    },
                }
            },
            None => StoragePath::root(),
        };
        
        // Create the storage path for the file
        let file_storage_path = folder_path.join(&name);
        tracing::info!("Created file path with ID: {:?} for file: {}", file_storage_path.to_string(), id);
        
        // Check if file already exists (and handle overwrites if needed)
        let exists = self.file_exists_at_storage_path(&file_storage_path).await?;
        tracing::info!("File exists check: {} for path: {:?}", exists, file_storage_path.to_string());
        
        // For save_file_with_id, force overwrite if needed
        let abs_path = self.resolve_storage_path(&file_storage_path);
        if exists {
            tracing::warn!("File already exists at path: {:?} - will overwrite", file_storage_path.to_string());
            // Delete the existing file with non-blocking approach
            self.delete_file_non_blocking(abs_path.clone()).await?;
        }
        
        // Create parent directories if they don't exist
        self.ensure_parent_directory(&abs_path).await?;
        
        // Write the file with timeout
        let file_creation_result = time::timeout(
            self.config.timeouts.file_timeout(), 
            TokioFile::create(&abs_path)
        ).await
        .map_err(|_| FileRepositoryError::Timeout(format!("Timeout creating file: {}", abs_path.display())))?
        .map_err(FileRepositoryError::IoError)?;
        
        let mut file = file_creation_result;
        
        let _write_result = time::timeout(
            self.config.timeouts.file_timeout(),
            file.write_all(&content)
        ).await
        .map_err(|_| FileRepositoryError::Timeout(format!("Timeout writing to file: {}", abs_path.display())))?
        .map_err(FileRepositoryError::IoError)?;
        
        // Ensure file is properly flushed and closed
        let _flush_result = time::timeout(
            self.config.timeouts.file_timeout(),
            file.flush()
        ).await
        .map_err(|_| FileRepositoryError::Timeout(format!("Timeout flushing file: {}", abs_path.display())))?
        .map_err(FileRepositoryError::IoError)?;
        
        // Get file metadata
        let (size, created_at, modified_at) = self.get_file_metadata(&abs_path).await?;
            
        // Determine the MIME type
        let mime_type = if content_type.is_empty() {
            from_path(&abs_path)
                .first_or_octet_stream()
                .to_string()
        } else {
            content_type
        };
        
        // Update the ID mapping for this path
        self.id_mapping_service.update_path(&id, &file_storage_path).await
            .map_err(|e| {
                // Domain errors should be mapped to appropriate FileRepositoryError
                if e.kind == crate::common::errors::ErrorKind::NotFound {
                    // If no previous mapping exists, treat this as a new mapping
                    tracing::info!("No existing ID mapping found for {}, creating new mapping", id);
                    FileRepositoryError::Other("ID not found in mapping, but continuing with new mapping".to_string())
                } else {
                    FileRepositoryError::from(e)
                }
            })?;
        
        // Keep a string representation of the path for logging
        let path_string = file_storage_path.to_string();
        
        // Create the file entity with the provided ID
        let file = self.create_file_entity(
            id.clone(),
            name,
            file_storage_path,
            size,
            mime_type,
            folder_id,
            Some(created_at),
            Some(modified_at),
        ).await?;
        
        // Save changes to mapping service
        self.id_mapping_service.save_changes().await?;
        
        tracing::info!("Saved file with specific ID: {} at path: {}", id, path_string);
        Ok(file)
    }
    
    async fn get_file_by_id(&self, id: &str) -> FileRepositoryResult<File> {
        // Find path by ID using the mapping service
        let storage_path = self.id_mapping_service.get_path_by_id(id).await
            .map_err(FileRepositoryError::from)?;
        
        // Check if file exists physically
        let abs_path = self.resolve_storage_path(&storage_path);
        if !abs_path.exists() || !abs_path.is_file() {
            tracing::error!("File not found at path: {}", abs_path.display());
            return Err(FileRepositoryError::NotFound(format!("File {} not found at {}", id, storage_path.to_string())));
        }
        
        // Get file metadata
        let (size, created_at, modified_at) = self.get_file_metadata(&abs_path).await?;
        
        // Get file name from the storage path
        let name = match storage_path.file_name() {
            Some(name) => name,
            None => {
                tracing::error!("Invalid file path: {}", storage_path.to_string());
                return Err(FileRepositoryError::InvalidPath(storage_path.to_string()));
            }
        };
        
        // Determine parent folder ID - we need to handle this based on storage path
        // This is a simplification - in a real system we might need to look up the folder ID
        let parent = storage_path.parent();
        let folder_id: Option<String> = if parent.is_none() || parent.as_ref().unwrap().is_empty() {
            None // Root folder
        } else {
            // For simplicity, we'll leave this as None for now
            // In a real implementation, you would look up the parent folder ID
            None
        };
        
        // Determine MIME type
        let mime_type = from_path(&abs_path)
            .first_or_octet_stream()
            .to_string();
        
        // Create file entity
        let file = self.create_file_entity(
            id.to_string(),
            name,
            storage_path,
            size,
            mime_type,
            folder_id,
            Some(created_at),
            Some(modified_at),
        ).await?;
        
        Ok(file)
    }
    
    async fn list_files(&self, folder_id: Option<&str>) -> FileRepositoryResult<Vec<File>> {
        tracing::info!("Listing files in folder_id: {:?}", folder_id);
        
        // Si estamos en modo desarrollo, listamos todos los archivos del directorio raíz
        // para facilitar el testing
        let base_storage_path = self.root_path.clone();
        let is_dev_mode = true; // Hard-code development mode para debugging
        
        if is_dev_mode && folder_id.is_none() {
            tracing::info!("Modo desarrollo activado: listando todos los archivos en el directorio raíz");
            
            let mut files_result = Vec::new();
            
            // Listar archivos en el directorio raíz
            match fs::read_dir(&base_storage_path).await {
                Ok(mut entries) => {
                    while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
                        let path = entry.path();
                        
                        // Skip if not a file or if it's a hidden/special file
                        if !path.is_file() {
                            continue;
                        }
                        
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        if file_name.starts_with('.') || file_name == "folder_ids.json" || file_name == "file_ids.json" {
                            continue;
                        }
                        
                        // Get file metadata
                        let metadata = match fs::metadata(&path).await {
                            Ok(m) => m,
                            Err(e) => {
                                tracing::error!("Error getting metadata for {:?}: {}", path, e);
                                continue;
                            }
                        };
                        
                        // Generate consistent ID for the file based on name
                        let storage_path = StoragePath::from_string(&file_name);
                        let id = Uuid::new_v4().to_string();
                        
                        // Extract file properties
                        let size = metadata.len();
                        let created_at = metadata.created()
                            .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
                            .unwrap_or(0);
                        let modified_at = metadata.modified()
                            .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
                            .unwrap_or(0);
                        
                        // Determine MIME type
                        let mime_type = from_path(&path)
                            .first_or_octet_stream()
                            .to_string();
                        
                        // Create file entity
                        let file = File::with_timestamps(
                            id,
                            file_name,
                            storage_path,
                            size,
                            mime_type,
                            None, // No folder ID
                            created_at,
                            modified_at,
                        ).unwrap();
                        
                        files_result.push(file);
                    }
                },
                Err(e) => {
                    tracing::error!("Error reading directory {:?}: {}", base_storage_path, e);
                }
            }
            
            tracing::info!("Modo desarrollo: se encontraron {} archivos en el directorio raíz", files_result.len());
            return Ok(files_result);
        }
        
        // Si no estamos en modo desarrollo o se especificó un folder_id, seguimos la lógica normal
        // Get the folder storage path
        let folder_storage_path = match folder_id {
            Some(id) => {
                match self.storage_mediator.get_folder_path(id).await {
                    Ok(path) => {
                        tracing::info!("Found folder with path: {:?}", path);
                        // Convert to StoragePath - use just the folder name to avoid path duplication
                        // Get just the folder name to avoid path duplication
                        let lossy = path.to_string_lossy().to_string();
                        let folder_name = path.file_name()
                            .and_then(|f| f.to_str())
                            .unwrap_or_else(|| &lossy);
                        tracing::info!("Using folder name: {} for StoragePath", folder_name);
                        StoragePath::from_string(folder_name)
                    },
                    Err(e) => {
                        tracing::error!("Error getting folder by ID: {}: {}", id, e);
                        return Ok(Vec::new());
                    },
                }
            },
            None => StoragePath::root(),
        };
        
        // Get the absolute folder path without duplicate ./storage prefix
        let abs_folder_path = self.path_service.resolve_path(&folder_storage_path);
        tracing::info!("Absolute folder path: {:?}", abs_folder_path);
        
        // Check if the directory exists
        if !abs_folder_path.exists() || !abs_folder_path.is_dir() {
            tracing::error!("Directory does not exist or is not a directory: {:?}", abs_folder_path);
            return Ok(Vec::new());
        }
        
        // Read directory entries
        let mut files_result = Vec::new();
        
        // Read the directory entries
        match fs::read_dir(&abs_folder_path).await {
            Ok(mut entries) => {
                while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
                    let path = entry.path();
                    
                    // Skip if not a file
                    if !path.is_file() {
                        continue;
                    }
                    
                    // Skip special files
                    let file_name_lossy = entry.file_name().to_string_lossy().to_string();
                    if file_name_lossy.starts_with('.') || file_name_lossy == "folder_ids.json" || file_name_lossy == "file_ids.json" {
                        continue;
                    }
                    
                    // Get file metadata
                    let metadata = match fs::metadata(&path).await {
                        Ok(m) => m,
                        Err(e) => {
                            tracing::error!("Error getting metadata for {:?}: {}", path, e);
                            continue;
                        }
                    };
                    
                    let file_name = file_name_lossy;
                    let file_storage_path = folder_storage_path.join(&file_name);
                    
                    // Get or create an ID for this file
                    let id = match self.id_mapping_service.get_or_create_id(&file_storage_path).await {
                        Ok(id) => id,
                        Err(e) => {
                            tracing::error!("Error getting ID for file: {}", e);
                            continue;
                        }
                    };
                    
                    // Extract metadata
                    let size = metadata.len();
                    
                    // Get creation timestamp
                    let created_at = metadata.created()
                        .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
                        .unwrap_or_else(|_| 0);
                        
                    // Get modification timestamp
                    let modified_at = metadata.modified()
                        .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
                        .unwrap_or_else(|_| 0);
                    
                    // Determine MIME type
                    let mime_type = from_path(&path)
                        .first_or_octet_stream()
                        .to_string();
                    
                    // Create file entity
                    match File::with_timestamps(
                        id,
                        file_name.clone(),
                        file_storage_path,
                        size,
                        mime_type,
                        folder_id.map(String::from),
                        created_at,
                        modified_at,
                    ) {
                        Ok(file) => {
                            tracing::info!("Added file to result list: {}", file.name());
                            files_result.push(file);
                        },
                        Err(e) => {
                            tracing::error!("Error creating file entity for {}: {}", file_name, e);
                            continue;
                        }
                    }
                }
            },
            Err(e) => {
                tracing::error!("Error reading directory {:?}: {}", abs_folder_path, e);
                return Err(FileRepositoryError::IoError(e));
            }
        }
        
        // Persist any new ID mappings that were created
        if !files_result.is_empty() {
            if let Err(e) = self.id_mapping_service.save_changes().await {
                tracing::error!("Error saving ID mappings: {}", e);
            }
        }
        
        tracing::info!("Found {} files in folder {:?}", files_result.len(), folder_id);
        Ok(files_result)
    }
    
    async fn delete_file(&self, id: &str) -> FileRepositoryResult<()> {
        // Get the file first to check if it exists
        let file = self.get_file_by_id(id).await?;
        
        // Delete the physical file with non-blocking approach
        let abs_path = self.resolve_storage_path(file.storage_path());
        tracing::info!("Deleting physical file: {}", abs_path.display());
        
        // Invalidate metadata cache for this file
        self.metadata_cache.invalidate(&abs_path).await;
        
        // Also invalidate any parent directory caches
        if let Some(parent_dir) = abs_path.parent() {
            self.metadata_cache.invalidate_directory(parent_dir).await;
        }
        
        self.delete_file_non_blocking(abs_path).await?;
        
        tracing::info!("Physical file deleted successfully: {}", file.storage_path().to_string());    
        Ok(())
    }
    
    async fn delete_file_entry(&self, id: &str) -> FileRepositoryResult<()> {
        // Get the file to make sure it exists
        let file = self.get_file_by_id(id).await?;
        
        // Delete the physical file
        let abs_path = self.resolve_storage_path(file.storage_path());
        tracing::info!("Deleting physical file and entry for ID: {}", id);
        
        // Try to delete the file with non-blocking approach, but continue even if it fails
        let delete_result = self.delete_file_non_blocking(abs_path).await;
        match &delete_result {
            Ok(_) => tracing::info!("Physical file deleted successfully: {}", file.storage_path().to_string()),
            Err(e) => tracing::warn!("Failed to delete physical file: {} - {}", file.storage_path().to_string(), e),
        };
        
        // Remove the ID mapping
        self.id_mapping_service.remove_id(id).await
            .map_err(FileRepositoryError::from)?;
        
        // Save the updated mappings
        self.id_mapping_service.save_changes().await?;
        
        // Return success even if file deletion failed - we've removed the mapping
        Ok(())
    }
    
    async fn get_file_content(&self, id: &str) -> FileRepositoryResult<Vec<u8>> {
        // Get the file first to check if it exists and get the path
        let file = self.get_file_by_id(id).await?;
        
        // Read the file content with timeout
        let abs_path = self.resolve_storage_path(file.storage_path());
        
        // Obtener el tamaño del archivo antes de leerlo
        let metadata = time::timeout(
            self.config.timeouts.file_timeout(),
            fs::metadata(&abs_path)
        ).await
        .map_err(|_| FileRepositoryError::Timeout(format!("Timeout getting metadata: {}", abs_path.display())))?
        .map_err(FileRepositoryError::IoError)?;
        
        let file_size = metadata.len();
        
        // Check if this can be loaded in memory
        let can_load_in_memory = self.config.resources.can_load_in_memory(file_size);
        
        tracing::info!("File size: {} bytes, can load in memory: {}", file_size, can_load_in_memory);
        
        if !can_load_in_memory {
            return Err(FileRepositoryError::Other(
                format!("File too large to load in memory: {} MB (max: {} MB)", 
                       file_size / (1024 * 1024), 
                       self.config.resources.max_in_memory_file_size_mb)
            ));
        }
        
        // Verificar si el archivo necesita procesamiento paralelo
        if self.config.resources.needs_parallel_processing(file_size, &self.config.concurrency) {
            // Para archivos muy grandes, usar el procesador paralelo
            tracing::info!("Using parallel file processor for large file: {}", abs_path.display());
            
            // Usar el procesador pre-configurado si está disponible o crear uno nuevo
            let content = if let Some(processor) = &self.parallel_processor {
                tracing::debug!("Using pre-configured parallel processor with buffer pool for reading");
                processor.read_file_parallel(&abs_path).await?
            } else {
                tracing::debug!("Creating on-demand parallel processor for reading");
                // Importar el procesador paralelo
                use crate::infrastructure::repositories::parallel_file_processor::ParallelFileProcessor;
                
                // Crear procesador con la configuración actual
                let processor = ParallelFileProcessor::new(self.config.clone());
                
                // Realizar lectura en paralelo
                processor.read_file_parallel(&abs_path).await?
            };
            
            tracing::info!("Successfully read {}MB file in parallel chunks", file_size / (1024 * 1024));
            return Ok(content);
        } else if self.config.resources.is_large_file(file_size) {
            // Para archivos grandes (pero no tanto como para paralelizar), usar spawn_blocking
            tracing::info!("Using spawn_blocking for large file: {}", abs_path.display());
            
            // Use spawn_blocking to prevent blocking the runtime
            let abs_path_clone = abs_path.clone();
            let chunk_size = self.config.resources.chunk_size_bytes;
            
            // Implementación para leer archivos grandes de forma optimizada:
            // 1. Creamos un buffer del tamaño exacto del archivo para evitar realocaciones
            // 2. Leemos el archivo en chunks dentro del spawn_blocking
            let content = task::spawn_blocking(move || -> std::io::Result<Vec<u8>> {
                use std::io::{Read, BufReader};
                use std::fs::File;
                
                // Abre el archivo de forma bloqueante
                let file = File::open(&abs_path_clone)?;
                let mut reader = BufReader::with_capacity(chunk_size, file);
                
                // Crea un buffer del tamaño exacto del archivo
                let mut buffer = Vec::with_capacity(file_size as usize);
                
                // Lee todo el contenido y devuelve el buffer
                reader.read_to_end(&mut buffer)?;
                Ok(buffer)
            }).await
            .map_err(|e| FileRepositoryError::Other(format!("Join error in spawn_blocking: {}", e)))?
            .map_err(FileRepositoryError::IoError)?;
            
            return Ok(content);
        } else {
            // Para archivos pequeños, usar tokio's async version con timeout
            let content = time::timeout(
                self.config.timeouts.file_timeout(),
                fs::read(&abs_path)
            ).await
            .map_err(|_| FileRepositoryError::Timeout(format!("Timeout reading file: {}", abs_path.display())))?
            .map_err(FileRepositoryError::IoError)?;
            
            return Ok(content);
        }
    }
    
    async fn get_file_stream(&self, id: &str) -> FileRepositoryResult<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>> {
        // Get the file first to check if it exists and get the path
        let file = self.get_file_by_id(id).await?;
        
        // Open the file for reading with timeout
        let abs_path = self.resolve_storage_path(file.storage_path());
        
        // Obtenemos el tamaño del archivo para definir el tamaño óptimo de los chunks
        let metadata = time::timeout(
            self.config.timeouts.file_timeout(),
            fs::metadata(&abs_path)
        ).await
        .map_err(|_| FileRepositoryError::Timeout(format!("Timeout getting metadata for stream: {}", abs_path.display())))?
        .map_err(FileRepositoryError::IoError)?;
        
        let file_size = metadata.len();
        let is_large = self.config.resources.is_large_file(file_size);
        
        // Abrimos el archivo con timeout
        let file = time::timeout(
            self.config.timeouts.file_timeout(),
            TokioFile::open(&abs_path)
        ).await
        .map_err(|_| FileRepositoryError::Timeout(format!("Timeout opening file stream for: {}", file.storage_path().to_string())))?
        .map_err(FileRepositoryError::IoError)?;
        
        // Definir tamaño de chunk óptimo según el tamaño del archivo
        let chunk_size = if is_large {
            // Para archivos grandes usamos el tamaño de chunk configurado
            self.config.resources.chunk_size_bytes
        } else {
            // Para archivos pequeños usamos un tamaño menor para maximizar eficiencia
            4096 // 4KB standard para archivos pequeños
        };
        
        tracing::info!("Streaming file {} (size: {} bytes) with chunk size: {}", 
                     abs_path.display(), file_size, chunk_size);
        
        // Creamos un codec con el tamaño de chunk optimizado
        let codec = BytesCodec::new();
        
        // Create a stream from the file, map BytesMut to Bytes, and box it
        let stream = FramedRead::with_capacity(file, codec, chunk_size)
            .map(|result| {
                result.map(|bytes_mut| {
                    // Convert BytesMut to Bytes (freeze)
                    bytes_mut.freeze()
                })
            });
            
        Ok(Box::new(stream))
    }
    
    async fn move_file(&self, id: &str, target_folder_id: Option<String>) -> FileRepositoryResult<File> {
        // Get the original file
        let original_file = self.get_file_by_id(id).await?;
        
        // If the target folder is the same as the current one, no need to move
        if original_file.folder_id() == target_folder_id.as_deref() {
            tracing::info!("File is already in the target folder, no need to move");
            return Ok(original_file);
        }
        
        // Get the target folder path
        let target_folder_path = match &target_folder_id {
            Some(folder_id) => {
                match self.storage_mediator.get_folder_path(folder_id).await {
                    Ok(path) => {
                        // Convert to StoragePath - use just the folder name to avoid path duplication
                        // Get just the folder name to avoid path duplication
                        let lossy = path.to_string_lossy().to_string();
                        let folder_name = path.file_name()
                            .and_then(|f| f.to_str())
                            .unwrap_or_else(|| &lossy);
                        tracing::info!("Target folder name: {} for StoragePath", folder_name);
                        StoragePath::from_string(folder_name)
                    },
                    Err(e) => {
                        return Err(FileRepositoryError::Other(
                            format!("Could not get target folder: {}", e)
                        ));
                    }
                }
            },
            None => StoragePath::root()
        };
        
        // Create the new file path
        let new_storage_path = target_folder_path.join(original_file.name());
        
        // Check if a file already exists at the destination
        if self.file_exists_at_storage_path(&new_storage_path).await? {
            return Err(FileRepositoryError::AlreadyExists(
                format!("File already exists at destination: {}", new_storage_path.to_string())
            ));
        }
        
        // Get absolute paths
        let old_abs_path = self.resolve_storage_path(original_file.storage_path());
        let new_abs_path = self.resolve_storage_path(&new_storage_path);
        
        // Ensure the target directory exists
        self.ensure_parent_directory(&new_abs_path).await?;
        
        // Move the file physically with fsync (efficient rename operation) with timeout
        time::timeout(
            self.config.timeouts.file_timeout(),
            FileSystemUtils::rename_with_sync(&old_abs_path, &new_abs_path)
        ).await
        .map_err(|_| FileRepositoryError::Timeout(format!("Timeout moving file from {} to {}", 
                                                        old_abs_path.display(), new_abs_path.display())))?
        .map_err(FileRepositoryError::IoError)?;
            
        tracing::info!("File moved successfully from {:?} to {:?}", old_abs_path, new_abs_path);
        
        // Update the ID mapping
        self.id_mapping_service.update_path(id, &new_storage_path).await
            .map_err(FileRepositoryError::from)?;
        
        // Save the updated mappings
        self.id_mapping_service.save_changes().await?;
        
        // Create and return the updated file entity
        // Create an immutable new version of the file with the updated folder
        let moved_file = original_file.with_folder(target_folder_id, Some(target_folder_path))
            .map_err(|e| FileRepositoryError::Other(e.to_string()))?;
        
        Ok(moved_file)
    }
    
    async fn get_file_path(&self, id: &str) -> FileRepositoryResult<StoragePath> {
        // Use the ID mapping service to get the storage path
        let storage_path = self.id_mapping_service.get_path_by_id(id).await
            .map_err(FileRepositoryError::from)?;
        
        Ok(storage_path)
    }
}