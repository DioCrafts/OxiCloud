use std::path::PathBuf;
use std::collections::HashMap;
use tokio::sync::{RwLock, Mutex};
use tokio::fs;
use tokio::time;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::domain::services::path_service::StoragePath;
use crate::common::errors::{DomainError, ErrorKind};
use crate::application::ports::outbound::IdMappingPort;
use crate::common::config::TimeoutConfig;

/// Specific error for the ID mapping service
#[derive(Debug, thiserror::Error)]
pub enum IdMappingError {
    #[error("ID not found: {0}")]
    NotFound(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}

// Implement conversion from IdMappingError to DomainError
impl From<IdMappingError> for DomainError {
    fn from(err: IdMappingError) -> Self {
        match err {
            IdMappingError::NotFound(id) => DomainError::not_found("IdMapping", id),
            IdMappingError::IoError(e) => DomainError::new(
                ErrorKind::InternalError,
                "IdMapping",
                format!("IO error: {}", e)
            ).with_source(e),
            IdMappingError::Timeout(msg) => DomainError::timeout(
                "IdMapping",
                format!("Timeout: {}", msg)
            ),
            IdMappingError::SerializationError(e) => DomainError::new(
                ErrorKind::InternalError,
                "IdMapping",
                format!("Serialization error: {}", e)
            ).with_source(e),
            IdMappingError::Other(msg) => DomainError::new(
                ErrorKind::InternalError,
                "IdMapping",
                format!("Other error: {}", msg)
            ),
        }
    }
}

/// Structure to store IDs mapped to their paths
#[derive(Serialize, Deserialize, Debug, Default)]
struct IdMap {
    path_to_id: HashMap<String, String>,
    id_to_path: HashMap<String, String>, // Field for efficient bidirectional lookup
    version: u32, // Version to detect changes
}

/// Service to manage mappings between paths and unique IDs
pub struct IdMappingService {
    map_path: PathBuf,
    id_map: RwLock<IdMap>,
    save_mutex: Mutex<()>, // To prevent multiple concurrent saves
    timeouts: TimeoutConfig,
    pending_save: RwLock<bool>, // Indicates if there are pending changes
}

impl IdMappingService {
    /// Creates a new ID mapping service
    pub async fn new(map_path: PathBuf) -> Result<Self, DomainError> {
        let timeouts = TimeoutConfig::default();
        let id_map = Self::load_id_map(&map_path, &timeouts).await?;
        
        Ok(Self {
            map_path,
            id_map: RwLock::new(id_map),
            save_mutex: Mutex::new(()),
            timeouts,
            pending_save: RwLock::new(false),
        })
    }
    
    /// Creates an in-memory ID mapping service (for testing)
    ///
    /// Similar functionality as new_in_memory but with a simpler signature for dummy use
    pub fn dummy() -> Self {
        Self {
            map_path: PathBuf::from("/tmp/dummy_id_map.json"),
            id_map: RwLock::new(IdMap::default()),
            save_mutex: Mutex::new(()),
            timeouts: TimeoutConfig::default(),
            pending_save: RwLock::new(false),
        }
    }
    
    /// Creates an in-memory ID mapping service (for testing - original version)
    pub fn new_in_memory() -> Self {
        Self {
            map_path: PathBuf::from("memory"),
            id_map: RwLock::new(IdMap::default()),
            save_mutex: Mutex::new(()),
            timeouts: TimeoutConfig::default(),
            pending_save: RwLock::new(false),
        }
    }
    
    /// Loads the ID map from disk with robust error handling
    async fn load_id_map(map_path: &PathBuf, timeouts: &TimeoutConfig) -> Result<IdMap, DomainError> {
        if map_path.exists() {
            // Try to read with timeout to avoid indefinite blocking
            let read_result = time::timeout(
                timeouts.lock_timeout(),
                fs::read_to_string(map_path)
            ).await
            .map_err(|_| DomainError::timeout("IdMapping", format!("Timeout reading ID map from {}", map_path.display())))?;
            
            let content = read_result.map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to read ID map from {}: {}", map_path.display(), e)))?;
            
            // Parse the JSON
            match serde_json::from_str::<IdMap>(&content) {
                Ok(mut map) => {
                    // Rebuild the inverse map if necessary
                    if map.id_to_path.is_empty() && !map.path_to_id.is_empty() {
                        let mut rebuild_count = 0;
                        for (path, id) in &map.path_to_id {
                            map.id_to_path.insert(id.clone(), path.clone());
                            rebuild_count += 1;
                        }
                        tracing::info!("Rebuilt inverse mapping with {} entries", rebuild_count);
                    }
                    
                    tracing::info!("Loaded ID map with {} entries (version: {})", 
                                   map.path_to_id.len(), map.version);
                    return Ok(map);
                },
                Err(e) => {
                    tracing::error!("Error parsing ID map: {}", e);
                    // Try to backup the corrupted file
                    let backup_path = map_path.with_extension("json.bak");
                    if let Err(copy_err) = tokio::fs::copy(map_path, &backup_path).await {
                        tracing::error!("Failed to backup corrupted map file: {}", copy_err);
                    } else {
                        tracing::info!("Backed up corrupted ID map to {}", backup_path.display());
                    }
                    
                    tracing::info!("Creating new empty map after error");
                    return Ok(IdMap {
                        path_to_id: HashMap::new(),
                        id_to_path: HashMap::new(),
                        version: 1, // Start with version 1
                    });
                }
            }
        }
        
        // Return an empty map if the file doesn't exist and create the file
        tracing::info!("No existing ID map found, creating new empty map");
        let empty_map = IdMap {
            path_to_id: HashMap::new(),
            id_to_path: HashMap::new(),
            version: 1, // Start with version 1
        };
        
        // Ensure directory exists
        if let Some(parent) = map_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent).await {
                    tracing::error!("Failed to create directory for ID map: {}", e);
                }
            }
        }
        
        // Write empty map to file (best-effort: the in-memory map is valid even if disk write fails)
        match serde_json::to_string_pretty(&empty_map) {
            Ok(json) => {
                if let Err(e) = fs::write(map_path, json).await {
                    tracing::warn!("Could not write initial empty ID map (will retry on next save): {}", e);
                } else {
                    tracing::info!("Created initial empty ID map at {}", map_path.display());
                }
            },
            Err(e) => {
                tracing::error!("Failed to serialize empty ID map: {}", e);
            }
        }
        
        Ok(empty_map)
    }
    
    /// Saves the ID map to disk safely
    async fn save_id_map(&self) -> Result<(), DomainError> {
        // Acquire exclusive lock for saving
        let _lock = time::timeout(
            self.timeouts.lock_timeout(),
            self.save_mutex.lock()
        ).await
        .map_err(|_| DomainError::timeout("IdMapping", "Timeout acquiring save lock for ID mapping"))?;
        
        // Create JSON with read lock to minimize lock hold time
        let json = {
            let mut map = time::timeout(
                self.timeouts.lock_timeout(),
                self.id_map.write()
            ).await
            .map_err(|_| DomainError::timeout("IdMapping", "Timeout acquiring write lock for ID mapping"))?;
            
            // Increment version only if there are pending changes to save
            let pending = *self.pending_save.read().await;
            if pending {
                map.version += 1;
                tracing::debug!("Incrementing ID map version to {}", map.version);
            }
            
            // Use serde with reasonably safe defaults
            serde_json::to_string_pretty(&*map)
                .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to serialize ID map to JSON: {}", e)))?
        };
        
        // Write to a temporary file first to avoid corruption
        let temp_path = self.map_path.with_extension("json.tmp");
        fs::write(&temp_path, &json).await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to write temporary ID map to {}: {}", temp_path.display(), e)))?;
        
        // Perform the atomic rename
        fs::rename(&temp_path, &self.map_path).await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to rename temporary ID map to {}: {}", self.map_path.display(), e)))?;
        
        // Reset pending flag
        {
            let mut pending = self.pending_save.write().await;
            *pending = false;
        }
        
        tracing::info!("Saved ID map successfully to {}", self.map_path.display());
        Ok(())
    }
    
    /// Generates a unique ID
    fn generate_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
    
    /// Marks changes as pending
    async fn mark_pending(&self) {
        let mut pending = self.pending_save.write().await;
        *pending = true;
    }
    
    /// Gets the ID for a path or generates a new one if it doesn't exist
    pub async fn get_or_create_id(&self, path: &StoragePath) -> Result<String, IdMappingError> {
        let path_str = path.to_string();
        
        // First attempt with read lock (more efficient)
        {
            let read_result = match time::timeout(
                self.timeouts.lock_timeout(), 
                self.id_map.read()
            ).await {
                Ok(guard) => guard,
                Err(_) => return Err(IdMappingError::Timeout("Timeout acquiring read lock for ID mapping".to_string())),
            };
            
            if let Some(id) = read_result.path_to_id.get(&path_str) {
                return Ok(id.clone());
            }
        }
        
        // If not found, acquire write lock
        let write_result = match time::timeout(
            self.timeouts.lock_timeout(),
            self.id_map.write()
        ).await {
            Ok(guard) => guard,
            Err(_) => return Err(IdMappingError::Timeout("Timeout acquiring write lock for ID mapping".to_string())),
        };
        
        let mut map = write_result;
        
        // Check again (it could have been added while we were waiting for the lock)
        if let Some(id) = map.path_to_id.get(&path_str) {
            return Ok(id.clone());
        }
        
        // Generate a new ID and store it
        let id = self.generate_id();
        map.path_to_id.insert(path_str.clone(), id.clone());
        map.id_to_path.insert(id.clone(), path_str);
        
        // Mark as pending for saving
        drop(map); // Release the write lock before acquiring another
        self.mark_pending().await;
        
        tracing::debug!("Created new ID mapping: {} -> {}", path.to_string(), id);
        
        Ok(id)
    }
    
    /// Gets a path by its ID with timeout handling
    pub async fn get_path_by_id(&self, id: &str) -> Result<StoragePath, IdMappingError> {
        let read_result = match time::timeout(
            self.timeouts.lock_timeout(), 
            self.id_map.read()
        ).await {
            Ok(guard) => guard,
            Err(_) => return Err(IdMappingError::Timeout("Timeout acquiring read lock for ID lookup".to_string())),
        };
        
        if let Some(path_str) = read_result.id_to_path.get(id) {
            return Ok(StoragePath::from_string(path_str));
        }
        
        Err(IdMappingError::NotFound(id.to_string()))
    }
    
    /// Updates the mapping of an existing ID to a new path
    pub async fn update_path(&self, id: &str, new_path: &StoragePath) -> Result<(), IdMappingError> {
        let write_result = match time::timeout(
            self.timeouts.lock_timeout(),
            self.id_map.write()
        ).await {
            Ok(guard) => guard,
            Err(_) => return Err(IdMappingError::Timeout("Timeout acquiring write lock for ID update".to_string())),
        };
        
        let mut map = write_result;
        
        // Find the previous path to remove it
        if let Some(old_path) = map.id_to_path.get(id).cloned() {
            map.path_to_id.remove(&old_path);
            
            // Register the new path
            let new_path_str = new_path.to_string();
            map.path_to_id.insert(new_path_str.clone(), id.to_string());
            map.id_to_path.insert(id.to_string(), new_path_str);
            
            // Mark as pending
            drop(map); // Release the write lock before acquiring another
            self.mark_pending().await;
            
            tracing::debug!("Updated path mapping for ID {}: {} -> {}", 
                id, old_path, new_path.to_string());
            
            Ok(())
        } else {
            Err(IdMappingError::NotFound(id.to_string()))
        }
    }
    
    /// Removes an ID from the map
    pub async fn remove_id(&self, id: &str) -> Result<(), IdMappingError> {
        let write_result = match time::timeout(
            self.timeouts.lock_timeout(),
            self.id_map.write()
        ).await {
            Ok(guard) => guard,
            Err(_) => return Err(IdMappingError::Timeout("Timeout acquiring write lock for ID removal".to_string())),
        };
        
        let mut map = write_result;
        
        // Find the path to remove it
        if let Some(path) = map.id_to_path.remove(id) {
            map.path_to_id.remove(&path);
            
            // Mark as pending
            drop(map); // Release the write lock before acquiring another
            self.mark_pending().await;
            
            tracing::debug!("Removed ID mapping: {} -> {}", id, path);
            Ok(())
        } else {
            Err(IdMappingError::NotFound(id.to_string()))
        }
    }
    
    /// Saves pending changes to disk immediately, without debounce
    pub async fn save_pending_changes(&self) -> Result<(), IdMappingError> {
        // Check if there are pending changes
        {
            let pending = self.pending_save.read().await;
            if !*pending {
                return Ok(());
            }
        }
        
        // Save immediately (without debounce or spawn)
        match self.save_id_map().await {
            Ok(_) => {
                tracing::info!("ID mappings saved successfully to disk at {}", self.map_path.display());
                
                // Explicitly verify that the file exists and has size
                match std::fs::metadata(&self.map_path) {
                    Ok(metadata) => {
                        if metadata.len() > 0 {
                            tracing::info!("Verified saved map file exists with size: {} bytes", metadata.len());
                        } else {
                            tracing::warn!("Map file exists but has zero size - this might cause issues");
                        }
                    },
                    Err(e) => {
                        tracing::error!("Failed to verify saved map file: {}", e);
                        // Try a second save if verification fails
                        if let Err(retry_err) = self.save_id_map().await {
                            tracing::error!("Second save attempt also failed: {}", retry_err);
                            return Err(IdMappingError::IoError(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("Failed to verify and retry save: {}", retry_err)
                            )));
                        }
                        tracing::info!("Second save attempt succeeded");
                    }
                }
                
                Ok(())
            },
            Err(e) => {
                tracing::error!("Failed to save ID map to {}: {}", self.map_path.display(), e);
                // Try a second save with delay in case of error
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                match self.save_id_map().await {
                    Ok(_) => {
                        tracing::info!("Second save attempt succeeded after initial failure");
                        Ok(())
                    },
                    Err(retry_e) => {
                        tracing::error!("Second save attempt also failed: {}", retry_e);
                        Err(IdMappingError::IoError(std::io::Error::new(
                            std::io::ErrorKind::Other, 
                            format!("Failed to save ID mappings after retry: {}", retry_e)
                        )))
                    }
                }
            }
        }
    }
}

#[async_trait]
impl IdMappingPort for IdMappingService {
    /// Gets the ID for a path or generates a new one if it doesn't exist
    async fn get_or_create_id(&self, path: &StoragePath) -> Result<String, DomainError> {
        self.get_or_create_id(path).await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to get or create ID for path: {}: {}", path.to_string(), e)))
    }
    
    /// Gets a path by its ID with timeout handling
    async fn get_path_by_id(&self, id: &str) -> Result<StoragePath, DomainError> {
        self.get_path_by_id(id).await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to get path for ID: {}: {}", id, e)))
    }
    
    /// Updates the mapping of an existing ID to a new path
    async fn update_path(&self, id: &str, new_path: &StoragePath) -> Result<(), DomainError> {
        self.update_path(id, new_path).await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to update path for ID: {} to {}: {}", id, new_path.to_string(), e)))
    }
    
    /// Removes an ID from the map
    async fn remove_id(&self, id: &str) -> Result<(), DomainError> {
        self.remove_id(id).await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to remove ID: {}: {}", id, e)))
    }
    
    /// Saves pending changes to disk
    async fn save_changes(&self) -> Result<(), DomainError> {
        self.save_pending_changes().await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to save pending ID mapping changes: {}", e)))
    }
}

// The extension methods were moved to the IdMappingPort trait as default implementations

// Implement Clone to allow use in tokio::spawn
/// Synchronous helper for contexts where we can't use async
impl IdMappingService {
    /// Create a new service synchronously (only for stubs and initialization)
    pub fn new_sync(map_path: PathBuf) -> Self {
        // Create a minimal implementation for initialization purposes
        Self {
            map_path,
            id_map: RwLock::new(IdMap::default()),
            save_mutex: Mutex::new(()),
            timeouts: TimeoutConfig::default(),
            pending_save: RwLock::new(false),
        }
    }
}

impl Clone for IdMappingService {
    fn clone(&self) -> Self {
        // We cannot directly clone the RwLock/Mutex,
        // but we can create new instances that point to the same internal Arc
        // However, in this case we simply need the map_path
        Self {
            map_path: self.map_path.clone(),
            id_map: RwLock::new(IdMap::default()), // This is not used in the async task
            save_mutex: Mutex::new(()),           // Neither is this
            timeouts: self.timeouts.clone(),
            pending_save: RwLock::new(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_get_or_create_id() {
        let temp_dir = tempdir().unwrap();
        let map_path = temp_dir.path().join("id_map.json");
        
        let service = IdMappingService::new(map_path).await.unwrap();
        
        let path = StoragePath::from_string("/test/file.txt");
        let id = service.get_or_create_id(&path).await.unwrap();
        
        assert!(!id.is_empty(), "ID should not be empty");
        
        // Verify that the same ID is returned for the same path
        let id2 = service.get_or_create_id(&path).await.unwrap();
        assert_eq!(id, id2, "Same path should return same ID");
    }
    
    #[tokio::test]
    async fn test_update_path() {
        let temp_dir = tempdir().unwrap();
        let map_path = temp_dir.path().join("id_map.json");
        
        let service = IdMappingService::new(map_path).await.unwrap();
        
        let old_path = StoragePath::from_string("/test/old.txt");
        let id = service.get_or_create_id(&old_path).await.unwrap();
        
        let new_path = StoragePath::from_string("/test/new.txt");
        service.update_path(&id, &new_path).await.unwrap();
        
        let retrieved_path = service.get_path_by_id(&id).await.unwrap();
        assert_eq!(retrieved_path, new_path, "Path should be updated");
    }
    
    #[tokio::test]
    async fn test_save_and_load() {
        let temp_dir = tempdir().unwrap();
        let map_path = temp_dir.path().join("id_map.json");
        
        // Create and populate the service
        let service = IdMappingService::new(map_path.clone()).await.unwrap();
        
        let path1 = StoragePath::from_string("/test/file1.txt");
        let path2 = StoragePath::from_string("/test/file2.txt");
        let id1 = service.get_or_create_id(&path1).await.unwrap();
        let id2 = service.get_or_create_id(&path2).await.unwrap();
        
        // Save changes
        service.save_pending_changes().await.unwrap();
        
        // Wait to ensure the async save completes
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        // Create a new service that should load the same map
        let service2 = IdMappingService::new(map_path).await.unwrap();
        
        // Verify that the IDs match
        let loaded_id1 = service2.get_or_create_id(&path1).await.unwrap();
        let loaded_id2 = service2.get_or_create_id(&path2).await.unwrap();
        
        assert_eq!(id1, loaded_id1, "ID1 should be preserved");
        assert_eq!(id2, loaded_id2, "ID2 should be preserved");
    }
    
    #[tokio::test]
    async fn test_concurrent_operations() {
        use futures::future::join_all;
        
        let temp_dir = tempdir().unwrap();
        let map_path = temp_dir.path().join("id_map.json");
        
        let service = std::sync::Arc::new(IdMappingService::new(map_path).await.unwrap());
        
        // Create multiple tasks that attempt simultaneous access
        let mut tasks = Vec::new();
        for i in 0..100 {
            let path = StoragePath::from_string(&format!("/test/concurrent/file{}.txt", i));
            let service_clone = service.clone();
            
            tasks.push(tokio::spawn(async move {
                service_clone.get_or_create_id(&path).await
            }));
        }
        
        // Wait for all to finish
        let results = join_all(tasks).await;
        
        // Verify that all succeeded
        for result in results {
            assert!(result.unwrap().is_ok(), "Concurrent operations should succeed");
        }
        
        // Save changes
        service.save_pending_changes().await.unwrap();
    }
}