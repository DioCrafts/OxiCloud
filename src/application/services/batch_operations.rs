use futures::{Future, future::join_all};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Semaphore;
use tracing::info;

use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::{FolderDto, MoveFolderDto};
use crate::application::ports::file_ports::{FileManagementUseCase, FileRetrievalUseCase};
use crate::application::ports::inbound::FolderUseCase;
use crate::application::ports::trash_ports::TrashUseCase;
use crate::application::ports::zip_ports::ZipPort;
use crate::application::services::folder_service::FolderService;
use crate::common::config::AppConfig;
use crate::common::errors::DomainError;

/// Specific errors for batch operations
#[derive(Debug, Error)]
pub enum BatchOperationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Operation cancelled: {0}")]
    Cancelled(String),

    #[error("Concurrency limit exceeded: {0}")]
    ConcurrencyLimit(String),

    #[error("Batch operation error: {0} ({1} of {2} completed)")]
    PartialFailure(String, usize, usize),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result of a batch operation with statistics
#[derive(Debug, Clone)]
pub struct BatchResult<T> {
    /// Successful results
    pub successful: Vec<T>,
    /// Failed operations with their errors
    pub failed: Vec<(String, String)>,
    /// Operation statistics
    pub stats: BatchStats,
}

/// Statistics of a batch operation
#[derive(Debug, Clone, Default)]
pub struct BatchStats {
    /// Total number of operations
    pub total: usize,
    /// Number of successful operations
    pub successful: usize,
    /// Number of failed operations
    pub failed: usize,
    /// Total execution time in milliseconds
    pub execution_time_ms: u128,
    /// Maximum concurrency reached
    pub max_concurrency: usize,
}

/// Batch operations service
pub struct BatchOperationService {
    file_retrieval: Arc<dyn FileRetrievalUseCase>,
    file_management: Arc<dyn FileManagementUseCase>,
    folder_service: Arc<FolderService>,
    trash_service: Option<Arc<dyn TrashUseCase>>,
    zip_service: Option<Arc<dyn ZipPort>>,
    config: AppConfig,
    semaphore: Arc<Semaphore>,
}

impl BatchOperationService {
    /// Creates a new instance of the batch operations service
    pub fn new(
        file_retrieval: Arc<dyn FileRetrievalUseCase>,
        file_management: Arc<dyn FileManagementUseCase>,
        folder_service: Arc<FolderService>,
        config: AppConfig,
    ) -> Self {
        // Limit concurrency based on configuration
        let max_concurrency = config.concurrency.max_concurrent_files;

        Self {
            file_retrieval,
            file_management,
            folder_service,
            trash_service: None,
            zip_service: None,
            config,
            semaphore: Arc::new(Semaphore::new(max_concurrency)),
        }
    }

    /// Creates a new instance with default configuration
    pub fn default(
        file_retrieval: Arc<dyn FileRetrievalUseCase>,
        file_management: Arc<dyn FileManagementUseCase>,
        folder_service: Arc<FolderService>,
    ) -> Self {
        Self::new(
            file_retrieval,
            file_management,
            folder_service,
            AppConfig::default(),
        )
    }

    /// Set the optional trash service (enables batch trash operations)
    pub fn with_trash_service(mut self, trash_service: Arc<dyn TrashUseCase>) -> Self {
        self.trash_service = Some(trash_service);
        self
    }

    /// Set the optional zip service (enables batch download)
    pub fn with_zip_service(mut self, zip_service: Arc<dyn ZipPort>) -> Self {
        self.zip_service = Some(zip_service);
        self
    }

    /// Copies multiple files in parallel
    pub async fn copy_files(
        &self,
        file_ids: Vec<String>,
        target_folder_id: Option<String>,
    ) -> Result<BatchResult<FileDto>, BatchOperationError> {
        info!("Starting batch copy of {} files", file_ids.len());
        let start_time = std::time::Instant::now();

        // Create result structure
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: file_ids.len(),
                ..Default::default()
            },
        };

        // Define the operation to perform for each file
        let operations = file_ids.into_iter().map(|file_id| {
            let mgmt = self.file_management.clone();
            let target_folder = target_folder_id.clone();
            let semaphore = self.semaphore.clone();

            async move {
                // Acquire semaphore permit
                let permit = semaphore.acquire().await.unwrap();

                let copy_result = mgmt.copy_file(&file_id, target_folder.clone()).await;

                // Release the permit explicitly (also released on drop)
                drop(permit);

                // Return the result along with the ID to identify successes/failures
                (file_id, copy_result)
            }
        });

        // Execute all operations in parallel with concurrency control
        let operation_results = join_all(operations).await;

        // Process the results
        for (file_id, operation_result) in operation_results {
            match operation_result {
                Ok(file) => {
                    result.successful.push(file);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((file_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }

        // Complete statistics
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self
            .config
            .concurrency
            .max_concurrent_files
            .min(result.stats.total);

        info!(
            "Batch copy completed: {}/{} successful in {}ms",
            result.stats.successful, result.stats.total, result.stats.execution_time_ms
        );

        Ok(result)
    }

    /// Moves multiple files in parallel
    pub async fn move_files(
        &self,
        file_ids: Vec<String>,
        target_folder_id: Option<String>,
    ) -> Result<BatchResult<FileDto>, BatchOperationError> {
        info!("Starting batch move of {} files", file_ids.len());
        let start_time = std::time::Instant::now();

        // Create result structure
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: file_ids.len(),
                ..Default::default()
            },
        };

        // Define the operation to perform for each file
        let operations = file_ids.into_iter().map(|file_id| {
            let mgmt = self.file_management.clone();
            let target_folder = target_folder_id.clone();
            let semaphore = self.semaphore.clone();

            async move {
                // Acquire semaphore permit
                let permit = semaphore.acquire().await.unwrap();

                let move_result = mgmt.move_file(&file_id, target_folder.clone()).await;

                // Release the permit explicitly
                drop(permit);

                // Return the result along with the ID to identify successes/failures
                (file_id, move_result)
            }
        });

        // Execute all operations in parallel with concurrency control
        let operation_results = join_all(operations).await;

        // Process the results
        for (file_id, operation_result) in operation_results {
            match operation_result {
                Ok(file) => {
                    result.successful.push(file);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((file_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }

        // Complete statistics
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self
            .config
            .concurrency
            .max_concurrent_files
            .min(result.stats.total);

        info!(
            "Batch move completed: {}/{} successful in {}ms",
            result.stats.successful, result.stats.total, result.stats.execution_time_ms
        );

        Ok(result)
    }

    /// Deletes multiple files in parallel
    pub async fn delete_files(
        &self,
        file_ids: Vec<String>,
    ) -> Result<BatchResult<String>, BatchOperationError> {
        info!("Starting batch deletion of {} files", file_ids.len());
        let start_time = std::time::Instant::now();

        // Create result structure
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: file_ids.len(),
                ..Default::default()
            },
        };

        // Define the operation to perform for each file
        let operations = file_ids.into_iter().map(|file_id| {
            let mgmt = self.file_management.clone();
            let semaphore = self.semaphore.clone();
            let id_clone = file_id.clone();

            async move {
                // Acquire semaphore permit
                let permit = semaphore.acquire().await.unwrap();

                let delete_result = mgmt.delete_file(&file_id).await;

                // Release the permit explicitly
                drop(permit);

                // Return the result along with the ID
                (id_clone.clone(), delete_result.map(|_| id_clone))
            }
        });

        // Execute all operations in parallel with concurrency control
        let operation_results = join_all(operations).await;

        // Process the results
        for (file_id, operation_result) in operation_results {
            match operation_result {
                Ok(id) => {
                    result.successful.push(id);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((file_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }

        // Complete statistics
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self
            .config
            .concurrency
            .max_concurrent_files
            .min(result.stats.total);

        info!(
            "Batch deletion completed: {}/{} successful in {}ms",
            result.stats.successful, result.stats.total, result.stats.execution_time_ms
        );

        Ok(result)
    }

    /// Loads multiple files in parallel (data in memory)
    pub async fn get_multiple_files(
        &self,
        file_ids: Vec<String>,
    ) -> Result<BatchResult<FileDto>, BatchOperationError> {
        info!("Starting batch load of {} files", file_ids.len());
        let start_time = std::time::Instant::now();

        // Create result structure
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: file_ids.len(),
                ..Default::default()
            },
        };

        // Define the operation to perform for each file
        let operations = file_ids.into_iter().map(|file_id| {
            let retrieval = self.file_retrieval.clone();
            let semaphore = self.semaphore.clone();

            async move {
                // Acquire semaphore permit
                let permit = semaphore.acquire().await.unwrap();

                let get_result = retrieval.get_file(&file_id).await;

                // Release the permit explicitly
                drop(permit);

                // Return the result along with the ID
                (file_id, get_result)
            }
        });

        // Execute all operations in parallel with concurrency control
        let operation_results = join_all(operations).await;

        // Process the results
        for (file_id, operation_result) in operation_results {
            match operation_result {
                Ok(file) => {
                    result.successful.push(file);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((file_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }

        // Complete statistics
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self
            .config
            .concurrency
            .max_concurrent_files
            .min(result.stats.total);

        info!(
            "Batch load completed: {}/{} successful in {}ms",
            result.stats.successful, result.stats.total, result.stats.execution_time_ms
        );

        Ok(result)
    }

    /// Deletes multiple folders in parallel
    pub async fn delete_folders(
        &self,
        folder_ids: Vec<String>,
        _recursive: bool,
        caller_id: &str,
    ) -> Result<BatchResult<String>, BatchOperationError> {
        info!("Starting batch deletion of {} folders", folder_ids.len());
        let start_time = std::time::Instant::now();

        // Create result structure
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: folder_ids.len(),
                ..Default::default()
            },
        };

        // Define the operation to perform for each folder
        let operations = folder_ids.into_iter().map(|folder_id| {
            let folder_service = self.folder_service.clone();
            let semaphore = self.semaphore.clone();
            let id_clone = folder_id.clone();
            let caller = caller_id.to_string();

            async move {
                // Acquire semaphore permit
                let permit = semaphore.acquire().await.unwrap();

                let delete_result = folder_service.delete_folder(&folder_id, &caller).await;

                // Release the permit explicitly
                drop(permit);

                // Return the result along with the ID
                (id_clone.clone(), delete_result.map(|_| id_clone))
            }
        });

        // Execute all operations in parallel with concurrency control
        let operation_results = join_all(operations).await;

        // Process the results
        for (folder_id, operation_result) in operation_results {
            match operation_result {
                Ok(id) => {
                    result.successful.push(id);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((folder_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }

        // Complete statistics
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self
            .config
            .concurrency
            .max_concurrent_files
            .min(result.stats.total);

        info!(
            "Batch folder deletion completed: {}/{} successful in {}ms",
            result.stats.successful, result.stats.total, result.stats.execution_time_ms
        );

        Ok(result)
    }

    /// Moves multiple files to trash in parallel (soft delete)
    pub async fn trash_files(
        &self,
        file_ids: Vec<String>,
        user_id: &str,
    ) -> Result<BatchResult<String>, BatchOperationError> {
        let trash_service = self
            .trash_service
            .as_ref()
            .ok_or_else(|| BatchOperationError::Internal("Trash service not available".into()))?;

        info!("Starting batch trash of {} files", file_ids.len());
        let start_time = std::time::Instant::now();

        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: file_ids.len(),
                ..Default::default()
            },
        };

        let operations = file_ids.into_iter().map(|file_id| {
            let trash = trash_service.clone();
            let semaphore = self.semaphore.clone();
            let uid = user_id.to_string();
            let id_clone = file_id.clone();

            async move {
                let permit = semaphore.acquire().await.unwrap();
                let trash_result = trash.move_to_trash(&file_id, "file", &uid).await;
                drop(permit);
                (id_clone.clone(), trash_result.map(|_| id_clone))
            }
        });

        let operation_results = join_all(operations).await;

        for (file_id, operation_result) in operation_results {
            match operation_result {
                Ok(id) => {
                    result.successful.push(id);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((file_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }

        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self
            .config
            .concurrency
            .max_concurrent_files
            .min(result.stats.total);

        info!(
            "Batch trash files completed: {}/{} successful in {}ms",
            result.stats.successful, result.stats.total, result.stats.execution_time_ms
        );

        Ok(result)
    }

    /// Moves multiple folders to trash in parallel (soft delete)
    pub async fn trash_folders(
        &self,
        folder_ids: Vec<String>,
        user_id: &str,
    ) -> Result<BatchResult<String>, BatchOperationError> {
        let trash_service = self
            .trash_service
            .as_ref()
            .ok_or_else(|| BatchOperationError::Internal("Trash service not available".into()))?;

        info!("Starting batch trash of {} folders", folder_ids.len());
        let start_time = std::time::Instant::now();

        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: folder_ids.len(),
                ..Default::default()
            },
        };

        let operations = folder_ids.into_iter().map(|folder_id| {
            let trash = trash_service.clone();
            let semaphore = self.semaphore.clone();
            let uid = user_id.to_string();
            let id_clone = folder_id.clone();

            async move {
                let permit = semaphore.acquire().await.unwrap();
                let trash_result = trash.move_to_trash(&folder_id, "folder", &uid).await;
                drop(permit);
                (id_clone.clone(), trash_result.map(|_| id_clone))
            }
        });

        let operation_results = join_all(operations).await;

        for (folder_id, operation_result) in operation_results {
            match operation_result {
                Ok(id) => {
                    result.successful.push(id);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((folder_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }

        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self
            .config
            .concurrency
            .max_concurrent_files
            .min(result.stats.total);

        info!(
            "Batch trash folders completed: {}/{} successful in {}ms",
            result.stats.successful, result.stats.total, result.stats.execution_time_ms
        );

        Ok(result)
    }

    /// Moves multiple folders to a target parent in parallel
    pub async fn move_folders(
        &self,
        folder_ids: Vec<String>,
        target_folder_id: Option<String>,
        caller_id: &str,
    ) -> Result<BatchResult<FolderDto>, BatchOperationError> {
        info!("Starting batch move of {} folders", folder_ids.len());
        let start_time = std::time::Instant::now();

        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: folder_ids.len(),
                ..Default::default()
            },
        };

        let operations = folder_ids.into_iter().map(|folder_id| {
            let folder_service = self.folder_service.clone();
            let target = target_folder_id.clone();
            let semaphore = self.semaphore.clone();
            let caller = caller_id.to_string();

            async move {
                let permit = semaphore.acquire().await.unwrap();
                let dto = MoveFolderDto { parent_id: target };
                let move_result = folder_service.move_folder(&folder_id, dto, &caller).await;
                drop(permit);
                (folder_id, move_result)
            }
        });

        let operation_results = join_all(operations).await;

        for (folder_id, operation_result) in operation_results {
            match operation_result {
                Ok(folder) => {
                    result.successful.push(folder);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((folder_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }

        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self
            .config
            .concurrency
            .max_concurrent_files
            .min(result.stats.total);

        info!(
            "Batch folder move completed: {}/{} successful in {}ms",
            result.stats.successful, result.stats.total, result.stats.execution_time_ms
        );

        Ok(result)
    }

    /// Downloads multiple files/folders as a single ZIP archive
    pub async fn download_zip(
        &self,
        file_ids: Vec<String>,
        folder_ids: Vec<String>,
    ) -> Result<Vec<u8>, BatchOperationError> {
        use std::io::{Cursor, Write};
        use zip::{ZipWriter, write::SimpleFileOptions};

        let zip_service = self.zip_service.as_ref();

        info!(
            "Starting batch download: {} files, {} folders",
            file_ids.len(),
            folder_ids.len()
        );
        let start_time = std::time::Instant::now();

        let buf = Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(buf);
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o644);

        // Add individual files at the root of the ZIP
        for file_id in &file_ids {
            match self.file_retrieval.get_file(file_id).await {
                Ok(file_dto) => {
                    match self.file_retrieval.get_file_content(file_id).await {
                        Ok(content) => {
                            if let Err(e) = zip.start_file(&file_dto.name, options) {
                                info!("Could not start zip entry for {}: {}", file_dto.name, e);
                                continue;
                            }
                            if let Err(e) = zip.write_all(&content) {
                                info!("Could not write zip entry for {}: {}", file_dto.name, e);
                            }
                        }
                        Err(e) => {
                            info!("Could not read file content {}: {}", file_id, e);
                        }
                    }
                }
                Err(e) => {
                    info!("Could not get file metadata {}: {}", file_id, e);
                }
            }
        }

        // Add folders as sub-trees using the existing ZipPort if available
        // Otherwise fall back to manual folder traversal
        if let Some(zip_svc) = zip_service {
            // For each folder, create a separate zip and merge its contents
            // Actually, we need to build the tree ourselves for a single zip
            // Use manual approach for consistency within one archive
            for folder_id in &folder_ids {
                match self.folder_service.get_folder(folder_id).await {
                    Ok(folder) => {
                        self.add_folder_to_zip(&mut zip, folder_id, &folder.name, &options)
                            .await;
                    }
                    Err(e) => {
                        info!("Could not get folder {}: {}", folder_id, e);
                    }
                }
            }
            // Suppress unused variable warning
            let _ = zip_svc;
        } else {
            for folder_id in &folder_ids {
                match self.folder_service.get_folder(folder_id).await {
                    Ok(folder) => {
                        self.add_folder_to_zip(&mut zip, folder_id, &folder.name, &options)
                            .await;
                    }
                    Err(e) => {
                        info!("Could not get folder {}: {}", folder_id, e);
                    }
                }
            }
        }

        let mut zip_buf = zip
            .finish()
            .map_err(|e| BatchOperationError::Internal(format!("ZIP finalize error: {}", e)))?;

        use std::io::Read;
        let mut bytes = Vec::new();
        zip_buf
            .read_to_end(&mut bytes)
            .map_err(|e| BatchOperationError::Internal(format!("ZIP read error: {}", e)))?;

        info!(
            "Batch download ZIP created: {} bytes in {}ms",
            bytes.len(),
            start_time.elapsed().as_millis()
        );

        Ok(bytes)
    }

    /// Recursively add a folder and its contents to a ZipWriter
    async fn add_folder_to_zip(
        &self,
        zip: &mut zip::ZipWriter<std::io::Cursor<Vec<u8>>>,
        folder_id: &str,
        path: &str,
        options: &zip::write::SimpleFileOptions,
    ) {
        use std::io::Write;

        struct PendingFolder {
            id: String,
            path: String,
        }

        let mut queue = vec![PendingFolder {
            id: folder_id.to_string(),
            path: path.to_string(),
        }];

        let mut visited = std::collections::HashSet::new();

        while let Some(current) = queue.pop() {
            if visited.contains(&current.id) {
                continue;
            }
            visited.insert(current.id.clone());

            let dir_path = format!("{}/", current.path);
            let _ = zip.add_directory(&dir_path, *options);

            // Add files
            if let Ok(files) = self.file_retrieval.list_files(Some(&current.id)).await {
                for file in files {
                    let file_path = format!("{}{}", dir_path, file.name);
                    if let Ok(content) = self.file_retrieval.get_file_content(&file.id).await {
                        if zip.start_file(&file_path, *options).is_ok() {
                            let _ = zip.write_all(&content);
                        }
                    }
                }
            }

            // Enqueue subfolders
            if let Ok(subfolders) = self.folder_service.list_folders(Some(&current.id)).await {
                for sub in subfolders {
                    queue.push(PendingFolder {
                        id: sub.id.clone(),
                        path: format!("{}/{}", current.path, sub.name),
                    });
                }
            }
        }
    }

    /// Generic batch operation for any type of async function
    pub async fn generic_batch_operation<T, F, Fut>(
        &self,
        items: Vec<T>,
        operation: F,
    ) -> Result<BatchResult<T>, BatchOperationError>
    where
        T: Clone + Send + 'static + std::fmt::Debug,
        F: Fn(T, Arc<Semaphore>) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = Result<T, DomainError>> + Send + 'static,
    {
        info!(
            "Starting generic batch operation with {} items",
            items.len()
        );
        let start_time = std::time::Instant::now();

        // Create result structure
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: items.len(),
                ..Default::default()
            },
        };

        // Convert each item to a task
        let tasks = items.iter().map(|item| {
            let item_clone = item.clone();
            let op = operation.clone();
            let semaphore = self.semaphore.clone();

            async move {
                // The provided function must handle semaphore acquisition
                let op_result = op(item_clone.clone(), semaphore).await;

                // Return the result along with the original item for identification
                (item_clone, op_result)
            }
        });

        // Execute all tasks in parallel
        let operation_results = join_all(tasks).await;

        // Process results
        for (item, operation_result) in operation_results {
            match operation_result {
                Ok(result_item) => {
                    result.successful.push(result_item);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    // Convert item to string for error reporting
                    result.failed.push((format!("{:?}", item), e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }

        // Complete statistics
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self
            .config
            .concurrency
            .max_concurrent_files
            .min(result.stats.total);

        info!(
            "Generic batch operation completed: {}/{} successful in {}ms",
            result.stats.successful, result.stats.total, result.stats.execution_time_ms
        );

        Ok(result)
    }

    /// Create multiple folders in parallel
    pub async fn create_folders(
        &self,
        folders: Vec<(String, Option<String>)>, // (name, parent_id)
    ) -> Result<BatchResult<FolderDto>, BatchOperationError> {
        info!("Starting batch creation of {} folders", folders.len());
        let start_time = std::time::Instant::now();

        // Create result structure
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: folders.len(),
                ..Default::default()
            },
        };

        // Define the operation for each folder
        let operations = folders.into_iter().map(|(name, parent_id)| {
            let folder_service = self.folder_service.clone();
            let semaphore = self.semaphore.clone();

            async move {
                // Acquire semaphore permit
                let permit = semaphore.acquire().await.unwrap();

                let dto = crate::application::dtos::folder_dto::CreateFolderDto {
                    name: name.clone(),
                    parent_id: parent_id.clone(),
                };
                let create_result = folder_service.create_folder(dto).await;

                // Release the permit explicitly
                drop(permit);

                // Return the result with an identifier for errors
                let id = format!("{}:{}", name, parent_id.unwrap_or_default());
                (id, create_result)
            }
        });

        // Execute all operations in parallel
        let operation_results = join_all(operations).await;

        // Process the results
        for (id, operation_result) in operation_results {
            match operation_result {
                Ok(folder) => {
                    result.successful.push(folder);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }

        // Complete statistics
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self
            .config
            .concurrency
            .max_concurrent_files
            .min(result.stats.total);

        info!(
            "Batch folder creation completed: {}/{} successful in {}ms",
            result.stats.successful, result.stats.total, result.stats.execution_time_ms
        );

        Ok(result)
    }

    /// Get metadata of multiple folders in parallel
    pub async fn get_multiple_folders(
        &self,
        folder_ids: Vec<String>,
    ) -> Result<BatchResult<FolderDto>, BatchOperationError> {
        info!("Starting batch load of {} folders", folder_ids.len());
        let start_time = std::time::Instant::now();

        // Create result structure
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: folder_ids.len(),
                ..Default::default()
            },
        };

        // Define the operation for each folder
        let operations = folder_ids.into_iter().map(|folder_id| {
            let folder_service = self.folder_service.clone();
            let semaphore = self.semaphore.clone();

            async move {
                // Acquire semaphore permit
                let permit = semaphore.acquire().await.unwrap();

                let get_result = folder_service.get_folder(&folder_id).await;

                // Release the permit explicitly
                drop(permit);

                // Return the result with its ID
                (folder_id, get_result)
            }
        });

        // Execute all operations in parallel
        let operation_results = join_all(operations).await;

        // Process the results
        for (folder_id, operation_result) in operation_results {
            match operation_result {
                Ok(folder) => {
                    result.successful.push(folder);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((folder_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }

        // Complete statistics
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self
            .config
            .concurrency
            .max_concurrent_files
            .min(result.stats.total);

        info!(
            "Batch folder load completed: {}/{} successful in {}ms",
            result.stats.successful, result.stats.total, result.stats.execution_time_ms
        );

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::stubs::{StubFileManagementUseCase, StubFileRetrievalUseCase};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_generic_batch_operation() {
        // Create the batch service with stubs
        let batch_service = BatchOperationService::new(
            Arc::new(StubFileRetrievalUseCase),
            Arc::new(StubFileManagementUseCase),
            Arc::new(FolderService::new(Arc::new(
                crate::common::stubs::StubFolderStoragePort,
            ))),
            AppConfig::default(),
        );

        // Define a generic test operation
        let operation = |item: i32, semaphore: Arc<Semaphore>| async move {
            // Acquire and release the semaphore
            let _permit = semaphore.acquire().await.unwrap();

            if item % 2 == 0 {
                // Simulate success for even numbers
                Ok(item * 2)
            } else {
                // Simulate error for odd numbers
                Err(DomainError::validation_error("Odd number not allowed"))
            }
        };

        // Execute the batch operation
        let items = vec![1, 2, 3, 4, 5];

        let result = batch_service
            .generic_batch_operation(items, operation)
            .await
            .unwrap();

        // Verify the results
        assert_eq!(result.stats.total, 5);
        assert_eq!(result.stats.successful, 2);
        assert_eq!(result.stats.failed, 3);

        // Even numbers should be in successes, doubled
        assert!(result.successful.contains(&4)); // 2*2
        assert!(result.successful.contains(&8)); // 4*2

        // Odd numbers should be in failures
        assert_eq!(result.failed.len(), 3);
    }
}
