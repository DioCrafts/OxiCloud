use futures::{Future, future::join_all};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Semaphore;
use tracing::info;

use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;
use crate::application::ports::file_ports::{FileManagementUseCase, FileRetrievalUseCase};
use crate::application::ports::inbound::FolderUseCase;
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

            async move {
                // Acquire semaphore permit
                let permit = semaphore.acquire().await.unwrap();

                // For both recursive and non-recursive, use the standard delete_folder method
                // since FolderUseCase only has a single delete_folder method
                let delete_result = folder_service.delete_folder(&folder_id).await;

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
