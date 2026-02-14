use std::path::PathBuf;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::{fs, time};
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use futures::{Stream, StreamExt};
use bytes::Bytes;
use mime_guess::from_path;
use tokio::task;

use crate::domain::entities::file::File;
use crate::application::ports::storage_ports::FileWritePort;
use crate::common::errors::DomainError;
use crate::infrastructure::repositories::repository_errors::{FileRepositoryResult, FileRepositoryError};
use crate::infrastructure::services::file_system_utils::FileSystemUtils;
use crate::application::ports::cache_ports::MetadataCachePort;
use crate::infrastructure::repositories::parallel_file_processor::ParallelFileProcessor;
use crate::application::services::storage_mediator::StorageMediator;
use crate::infrastructure::services::path_service::PathService;
use crate::domain::services::path_service::StoragePath;
use crate::common::config::AppConfig;

/// Repository implementation for file **write** operations.
///
/// Implements `FileWritePort`:
/// save_file, save_file_from_stream, move_file, delete_file,
/// update_file_content, register_file_deferred.
pub struct FileFsWriteRepository {
    root_path: PathBuf,
    storage_mediator: Arc<dyn StorageMediator>,
    id_mapping_service: Arc<dyn crate::application::ports::outbound::IdMappingPort>,
    path_service: Arc<PathService>,
    metadata_cache: Arc<dyn MetadataCachePort>,
    config: AppConfig,
    parallel_processor: Option<Arc<ParallelFileProcessor>>,
}

impl FileFsWriteRepository {
    /// Full constructor with all dependencies.
    pub fn new(
        root_path: PathBuf,
        storage_mediator: Arc<dyn StorageMediator>,
        id_mapping_service: Arc<dyn crate::application::ports::outbound::IdMappingPort>,
        path_service: Arc<PathService>,
        metadata_cache: Arc<dyn MetadataCachePort>,
        config: AppConfig,
        parallel_processor: Option<Arc<ParallelFileProcessor>>,
    ) -> Self {
        Self { root_path, storage_mediator, id_mapping_service, path_service, metadata_cache, config, parallel_processor }
    }

    /// Stub for testing (does not perform real I/O).
    pub fn default_stub() -> Self {
        Self {
            root_path: PathBuf::from("./storage"),
            storage_mediator: Arc::new(
                crate::application::services::storage_mediator::FileSystemStorageMediator::new_stub(),
            ),
            id_mapping_service: Arc::new(crate::common::stubs::StubIdMappingPort),
            path_service: Arc::new(PathService::new(PathBuf::from("./storage"))),
            metadata_cache: Arc::new(
                crate::infrastructure::services::file_metadata_cache::FileMetadataCache::default()
            ) as Arc<dyn MetadataCachePort>,
            config: AppConfig::default(),
            parallel_processor: None,
        }
    }

    // ─── helpers ─────────────────────────────────────────────

    fn resolve_storage_path(&self, storage_path: &StoragePath) -> PathBuf {
        self.path_service.resolve_path(storage_path)
    }

    async fn ensure_parent_directory(&self, abs_path: &PathBuf) -> FileRepositoryResult<()> {
        if let Some(parent) = abs_path.parent() {
            time::timeout(
                self.config.timeouts.dir_timeout(),
                FileSystemUtils::create_dir_with_sync(parent),
            ).await
            .map_err(|_| FileRepositoryError::StorageError(format!("Timeout creating dir: {}", parent.display())))?
            .map_err(|e| FileRepositoryError::StorageError(e.to_string()))?;
        }
        Ok(())
    }

    async fn file_exists_at_storage_path(&self, storage_path: &StoragePath) -> FileRepositoryResult<bool> {
        let abs = self.resolve_storage_path(storage_path);
        if let Some(is_file) = self.metadata_cache.is_file(&abs).await {
            return Ok(is_file);
        }
        match time::timeout(self.config.timeouts.file_timeout(), fs::metadata(&abs)).await {
            Ok(Ok(m)) => {
                let _ = self.metadata_cache.refresh_metadata(&abs).await;
                Ok(m.is_file())
            }
            Ok(Err(_)) => Ok(false),
            Err(_) => Err(FileRepositoryError::StorageError(format!("Timeout: {}", abs.display()))),
        }
    }

    async fn get_file_metadata_raw(&self, abs_path: &PathBuf) -> FileRepositoryResult<(u64, u64, u64)> {
        if let Some(cached) = self.metadata_cache.get_metadata(abs_path).await
            && let (Some(s), Some(c), Some(m)) = (cached.size, cached.created_at, cached.modified_at) {
                return Ok((s, c, m));
            }
        let meta = time::timeout(self.config.timeouts.file_timeout(), fs::metadata(abs_path))
            .await
            .map_err(|_| FileRepositoryError::StorageError(format!("Timeout: {}", abs_path.display())))?
            .map_err(|e| FileRepositoryError::StorageError(e.to_string()))?;
        let s = meta.len();
        let c = meta.created().map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()).unwrap_or(0);
        let m = meta.modified().map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()).unwrap_or(0);
        let _ = self.metadata_cache.refresh_metadata(abs_path).await;
        Ok((s, c, m))
    }

    /// Resolve folder to StoragePath
    async fn resolve_folder_path(&self, folder_id: &Option<String>) -> StoragePath {
        match folder_id {
            Some(id) => match self.storage_mediator.get_folder_path(id).await {
                Ok(path) => {
                    let lossy = path.to_string_lossy().to_string();
                    let folder_name = path.file_name().and_then(|f| f.to_str()).unwrap_or(&lossy);
                    StoragePath::from_string(folder_name)
                }
                Err(_) => StoragePath::root(),
            },
            None => StoragePath::root(),
        }
    }

    /// Generate unique file path avoiding name collisions.
    async fn unique_file_path(
        &self,
        folder_path: &StoragePath,
        name: &str,
    ) -> FileRepositoryResult<(StoragePath, String)> {
        let mut file_path = folder_path.join(name);
        let mut actual_name = name.to_string();
        let mut counter = 1;
        while self.file_exists_at_storage_path(&file_path).await? {
            let (stem, ext) = if let Some(dot) = name.rfind('.') {
                (name[..dot].to_string(), name[dot..].to_string())
            } else {
                (name.to_string(), String::new())
            };
            actual_name = format!("{}_{}{}", stem, counter, ext);
            file_path = folder_path.join(&actual_name);
            counter += 1;
        }
        Ok((file_path, actual_name))
    }

    async fn delete_file_non_blocking(&self, abs_path: PathBuf) -> FileRepositoryResult<()> {
        let file_size = match fs::metadata(&abs_path).await {
            Ok(m) => m.len(),
            Err(_) => 0,
        };
        if self.config.resources.is_large_file(file_size) {
            task::spawn_blocking(move || { let _ = std::fs::remove_file(&abs_path); })
                .await
                .map_err(|e| FileRepositoryError::Other(e.to_string()))?;
        } else {
            time::timeout(self.config.timeouts.file_timeout(), fs::remove_file(&abs_path))
                .await
                .map_err(|_| FileRepositoryError::StorageError("Timeout deleting file".into()))?
                .map_err(|e| FileRepositoryError::StorageError(e.to_string()))?;
        }
        Ok(())
    }

    /// Persist ID mapping with retry + verification.
    async fn persist_id_mapping(&self, id: &str, expected_path: &str) -> FileRepositoryResult<()> {
        for attempt in 1..=3 {
            match self.id_mapping_service.save_changes().await {
                Ok(_) => {
                    if let Ok(verified) = self.id_mapping_service.get_path_by_id(id).await
                        && verified.to_string() == expected_path {
                            return Ok(());
                        }
                    if attempt == 3 {
                        return Err(FileRepositoryError::Other("Failed to verify ID mapping after 3 attempts".into()));
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
                Err(e) if attempt < 3 => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    tracing::warn!("ID mapping save retry {}: {}", attempt, e);
                }
                Err(e) => return Err(FileRepositoryError::Other(format!("Save ID mapping failed: {}", e))),
            }
        }
        Ok(())
    }
}

impl Clone for FileFsWriteRepository {
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

fn map_repo_err(e: FileRepositoryError) -> DomainError {
    match e {
        FileRepositoryError::NotFound(m) => DomainError::not_found("File", m),
        FileRepositoryError::AlreadyExists(m) => DomainError::already_exists("File", m),
        FileRepositoryError::StorageError(m) => DomainError::internal_error("File", m),
        other => DomainError::internal_error("File", other.to_string()),
    }
}

#[async_trait]
impl FileWritePort for FileFsWriteRepository {
    async fn save_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<File, DomainError> {
        let folder_path = self.resolve_folder_path(&folder_id).await;
        let (file_storage_path, actual_name) = self.unique_file_path(&folder_path, &name).await.map_err(map_repo_err)?;
        let abs_path = self.resolve_storage_path(&file_storage_path);
        self.ensure_parent_directory(&abs_path).await.map_err(map_repo_err)?;

        let content_size = content.len() as u64;

        // Write strategy based on file size
        if self.config.resources.needs_parallel_processing(content_size, &self.config.concurrency) {
            if let Some(proc) = &self.parallel_processor {
                proc.write_file_parallel(&abs_path, &content).await.map_err(map_repo_err)?;
            } else {
                let proc = ParallelFileProcessor::new(self.config.clone());
                proc.write_file_parallel(&abs_path, &content).await.map_err(map_repo_err)?;
            }
        } else if content_size > self.config.resources.large_file_threshold_mb * 1024 * 1024 {
            let mut fh = time::timeout(self.config.timeouts.file_timeout(), TokioFile::create(&abs_path))
                .await
                .map_err(|_| DomainError::internal_error("File", "Timeout creating file"))?
                .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
            let chunk_size = self.config.resources.chunk_size_bytes;
            for chunk in content.chunks(chunk_size) {
                fh.write_all(chunk).await.map_err(|e| DomainError::internal_error("File", e.to_string()))?;
            }
            fh.flush().await.map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        } else {
            let mut fh = time::timeout(self.config.timeouts.file_timeout(), TokioFile::create(&abs_path))
                .await
                .map_err(|_| DomainError::internal_error("File", "Timeout creating file"))?
                .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
            fh.write_all(&content).await.map_err(|e| DomainError::internal_error("File", e.to_string()))?;
            fh.flush().await.map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        }

        let (size, created_at, modified_at) = self.get_file_metadata_raw(&abs_path).await.map_err(map_repo_err)?;
        let mime = if content_type.is_empty() { from_path(&abs_path).first_or_octet_stream().to_string() } else { content_type };
        let id = self.id_mapping_service.get_or_create_id(&file_storage_path).await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        let path_string = file_storage_path.to_string();

        let file = File::with_timestamps(id.clone(), actual_name, file_storage_path, size, mime, folder_id, created_at, modified_at)
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;

        self.persist_id_mapping(&id, &path_string).await.map_err(map_repo_err)?;
        if let Some(parent) = abs_path.parent() {
            self.metadata_cache.invalidate_directory(parent).await;
        }
        Ok(file)
    }

    async fn save_file_from_stream(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        mut stream: std::pin::Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>,
    ) -> Result<File, DomainError> {
        let folder_path = self.resolve_folder_path(&folder_id).await;
        let (file_storage_path, actual_name) = self.unique_file_path(&folder_path, &name).await.map_err(map_repo_err)?;
        let abs_path = self.resolve_storage_path(&file_storage_path);
        self.ensure_parent_directory(&abs_path).await.map_err(map_repo_err)?;

        let temp_path = abs_path.with_extension("tmp.upload");
        let mut fh = time::timeout(self.config.timeouts.file_timeout(), TokioFile::create(&temp_path))
            .await
            .map_err(|_| DomainError::internal_error("File", "Timeout creating temp file"))?
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;

        let mut total_bytes: u64 = 0;
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| DomainError::internal_error("File", e.to_string()))?;
            fh.write_all(&chunk).await.map_err(|e| DomainError::internal_error("File", e.to_string()))?;
            total_bytes += chunk.len() as u64;
        }
        fh.flush().await.map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        fh.sync_all().await.map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        drop(fh);

        // Atomic rename
        fs::rename(&temp_path, &abs_path).await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;

        let (size, created_at, modified_at) = self.get_file_metadata_raw(&abs_path).await.map_err(map_repo_err)?;
        let mime = if content_type.is_empty() { from_path(&abs_path).first_or_octet_stream().to_string() } else { content_type };
        let id = self.id_mapping_service.get_or_create_id(&file_storage_path).await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        let path_string = file_storage_path.to_string();
        let log_name = actual_name.clone();

        let file = File::with_timestamps(id.clone(), actual_name, file_storage_path, size, mime, folder_id, created_at, modified_at)
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;

        self.persist_id_mapping(&id, &path_string).await.map_err(map_repo_err)?;
        if let Some(parent) = abs_path.parent() {
            self.metadata_cache.invalidate_directory(parent).await;
        }
        tracing::info!("✅ STREAMING UPLOAD COMPLETE: {} ({} bytes)", log_name, total_bytes);
        Ok(file)
    }

    async fn move_file(
        &self,
        file_id: &str,
        target_folder_id: Option<String>,
    ) -> Result<File, DomainError> {
        // Get original file
        let original_path = self.id_mapping_service.get_path_by_id(file_id).await?;
        let old_abs = self.resolve_storage_path(&original_path);
        if !old_abs.exists() || !old_abs.is_file() {
            return Err(DomainError::not_found("File", file_id.to_string()));
        }
        let (size, created_at, modified_at) = self.get_file_metadata_raw(&old_abs).await.map_err(map_repo_err)?;
        let name = original_path.file_name()
            .ok_or_else(|| DomainError::internal_error("File", "Invalid path"))?;
        let mime = from_path(&old_abs).first_or_octet_stream().to_string();

        // Build target path
        let target_folder_path = self.resolve_folder_path(&target_folder_id).await;
        let new_storage_path = target_folder_path.join(&name);
        if self.file_exists_at_storage_path(&new_storage_path).await.map_err(map_repo_err)? {
            return Err(DomainError::already_exists("File",
                format!("File already exists at {}", new_storage_path.to_string())));
        }
        let new_abs = self.resolve_storage_path(&new_storage_path);
        self.ensure_parent_directory(&new_abs).await.map_err(map_repo_err)?;

        // Rename
        time::timeout(
            self.config.timeouts.file_timeout(),
            FileSystemUtils::rename_with_sync(&old_abs, &new_abs),
        ).await
        .map_err(|_| DomainError::internal_error("File", "Timeout moving file"))?
        .map_err(|e| DomainError::internal_error("File", e.to_string()))?;

        // Update mapping
        self.id_mapping_service.update_path(file_id, &new_storage_path).await?;
        let _ = self.id_mapping_service.save_changes().await;

        File::with_timestamps(file_id.to_string(), name, new_storage_path, size, mime, target_folder_id, created_at, modified_at)
            .map_err(|e| DomainError::internal_error("File", e.to_string()))
    }

    async fn rename_file(
        &self,
        file_id: &str,
        new_name: &str,
    ) -> Result<File, DomainError> {
        // 1. Get current file info
        let original_path = self.id_mapping_service.get_path_by_id(file_id).await?;
        let old_abs = self.resolve_storage_path(&original_path);
        if !old_abs.exists() || !old_abs.is_file() {
            return Err(DomainError::not_found("File", file_id.to_string()));
        }
        let (size, created_at, modified_at) = self.get_file_metadata_raw(&old_abs).await.map_err(map_repo_err)?;

        // 2. Build new path (same parent directory, different filename)
        let parent = original_path.parent()
            .unwrap_or_else(|| StoragePath::new(vec![]));
        let new_storage_path = parent.join(new_name);
        if self.file_exists_at_storage_path(&new_storage_path).await.map_err(map_repo_err)? {
            return Err(DomainError::already_exists("File",
                format!("File already exists: {}", new_name)));
        }
        let new_abs = self.resolve_storage_path(&new_storage_path);
        let mime = from_path(&new_abs).first_or_octet_stream().to_string();

        // 3. Rename on disk
        time::timeout(
            self.config.timeouts.file_timeout(),
            FileSystemUtils::rename_with_sync(&old_abs, &new_abs),
        ).await
        .map_err(|_| DomainError::internal_error("File", "Timeout renaming file"))?
        .map_err(|e| DomainError::internal_error("File", e.to_string()))?;

        // 4. Update id→path mapping
        self.id_mapping_service.update_path(file_id, &new_storage_path).await?;
        let _ = self.id_mapping_service.save_changes().await;

        File::with_timestamps(
            file_id.to_string(),
            new_name.to_string(),
            new_storage_path,
            size,
            mime,
            None,
            created_at,
            modified_at,
        )
        .map_err(|e| DomainError::internal_error("File", e.to_string()))
    }

    async fn delete_file(&self, id: &str) -> Result<(), DomainError> {
        let storage_path = self.id_mapping_service.get_path_by_id(id).await?;
        let abs_path = self.resolve_storage_path(&storage_path);

        self.metadata_cache.invalidate(&abs_path).await;
        if let Some(parent) = abs_path.parent() {
            self.metadata_cache.invalidate_directory(parent).await;
        }

        self.delete_file_non_blocking(abs_path).await.map_err(map_repo_err)?;

        // Clean up the ID mapping so we don't leave orphaned entries
        if let Err(e) = self.id_mapping_service.remove_id(id).await {
            tracing::warn!("Failed to remove ID mapping for deleted file {}: {}", id, e);
        }
        let _ = self.id_mapping_service.save_changes().await;

        Ok(())
    }

    async fn update_file_content(&self, file_id: &str, content: Vec<u8>) -> Result<(), DomainError> {
        let storage_path = self.id_mapping_service.get_path_by_id(file_id).await?;
        let physical_path = self.resolve_storage_path(&storage_path);

        FileSystemUtils::atomic_write(&physical_path, &content)
            .await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;

        // Refresh cache
        let _ = self.metadata_cache.refresh_metadata(&physical_path).await;
        Ok(())
    }

    async fn register_file_deferred(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        size: u64,
    ) -> Result<(File, PathBuf), DomainError> {
        let folder_path = self.resolve_folder_path(&folder_id).await;
        let (file_storage_path, actual_name) = self.unique_file_path(&folder_path, &name).await.map_err(map_repo_err)?;
        let abs_path = self.resolve_storage_path(&file_storage_path);
        self.ensure_parent_directory(&abs_path).await.map_err(map_repo_err)?;

        let mime = if content_type.is_empty() {
            from_path(&abs_path).first_or_octet_stream().to_string()
        } else {
            content_type
        };
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let id = self.id_mapping_service.get_or_create_id(&file_storage_path).await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        let _ = self.id_mapping_service.save_changes().await;

        let file = File::with_timestamps(id.clone(), actual_name, file_storage_path, size, mime, folder_id, now, now)
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;

        tracing::debug!("⚡ Registered deferred file: {} -> {:?}", id, abs_path);
        Ok((file, abs_path))
    }

    async fn move_to_trash(&self, file_id: &str) -> Result<(), DomainError> {
        // Get the file's current path
        let storage_path = self.id_mapping_service.get_path_by_id(file_id).await?;
        let abs_path = self.resolve_storage_path(&storage_path);

        if !abs_path.exists() || !abs_path.is_file() {
            return Err(DomainError::not_found("File", file_id.to_string()));
        }

        // Create trash directory
        let trash_dir = self.root_path.join(".trash").join("files");
        fs::create_dir_all(&trash_dir).await
            .map_err(|e| DomainError::internal_error("File", format!("Failed to create trash dir: {}", e)))?;

        // Move file to trash
        let trash_path = trash_dir.join(file_id);
        fs::rename(&abs_path, &trash_path).await
            .map_err(|e| DomainError::internal_error("File", format!("Failed to move file to trash: {}", e)))?;

        // Update mapping to trash location
        let trash_storage_path = StoragePath::from_string(&format!(".trash/files/{}", file_id));
        self.id_mapping_service.update_path(file_id, &trash_storage_path).await?;
        let _ = self.id_mapping_service.save_changes().await;

        // Invalidate cache
        self.metadata_cache.invalidate(&abs_path).await;
        if let Some(parent) = abs_path.parent() {
            self.metadata_cache.invalidate_directory(parent).await;
        }

        tracing::debug!("File moved to trash: {} -> {}", file_id, trash_path.display());
        Ok(())
    }

    async fn restore_from_trash(&self, file_id: &str, original_path: &str) -> Result<(), DomainError> {
        // Get current path (should be in trash)
        let current_storage_path = self.id_mapping_service.get_path_by_id(file_id).await?;
        let current_abs_path = self.resolve_storage_path(&current_storage_path);

        if !current_abs_path.exists() {
            return Err(DomainError::not_found("File", format!("File {} not found in trash", file_id)));
        }

        // Ensure parent directory exists for original location
        let original_storage_path = StoragePath::from_string(original_path);
        let original_abs_path = self.resolve_storage_path(&original_storage_path);
        if let Some(parent) = original_abs_path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| DomainError::internal_error("File", format!("Failed to create parent dir: {}", e)))?;
        }

        // Move file back to original location
        fs::rename(&current_abs_path, &original_abs_path).await
            .map_err(|e| DomainError::internal_error("File", format!("Failed to restore file: {}", e)))?;

        // Update mapping back to original path
        self.id_mapping_service.update_path(file_id, &original_storage_path).await?;
        let _ = self.id_mapping_service.save_changes().await;

        tracing::debug!("File restored from trash: {} -> {}", file_id, original_abs_path.display());
        Ok(())
    }

    async fn delete_file_permanently(&self, file_id: &str) -> Result<(), DomainError> {
        // Get current path (could be in trash or original location)
        let storage_path = self.id_mapping_service.get_path_by_id(file_id).await?;
        let abs_path = self.resolve_storage_path(&storage_path);

        // Delete the physical file if it exists
        if abs_path.exists() {
            self.delete_file_non_blocking(abs_path.clone()).await.map_err(map_repo_err)?;
        }

        // Remove ID mapping
        self.id_mapping_service.remove_id(file_id).await?;
        let _ = self.id_mapping_service.save_changes().await;

        // Invalidate cache
        self.metadata_cache.invalidate(&abs_path).await;

        tracing::debug!("File permanently deleted: {}", file_id);
        Ok(())
    }
}