use async_trait::async_trait;
use crate::domain::entities::file::File;
use crate::domain::services::path_service::StoragePath;
use crate::common::errors::DomainError;
use futures::Stream;
use bytes::Bytes;

/**
 * Comprehensive error types for file repository operations.
 * 
 * This enum represents all possible error conditions that can occur during file repository 
 * operations, providing detailed context for error handling across the application.
 */
#[derive(Debug, thiserror::Error)]
pub enum FileRepositoryError {
    /// Returned when a requested file cannot be found by ID or path
    #[error("File not found: {0}")]
    NotFound(String),
    
    /// Returned when attempting to create a file at a location where one already exists
    #[error("File already exists: {0}")]
    AlreadyExists(String),
    
    /// Returned when a provided file path is invalid or malformed
    #[error("Invalid file path: {0}")]
    InvalidPath(String),
    
    /// Returned when an operation is not supported by the current implementation
    #[error("Operation not supported: {0}")]
    OperationNotSupported(String),
    
    /// Wraps standard I/O errors from the filesystem
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Indicates errors in the path-to-ID mapping system
    #[error("Mapping error: {0}")]
    MappingError(String),
    
    /// Specific errors related to ID mapping operations
    #[error("ID Mapping error: {0}")]
    IdMappingError(String),
    
    /// Returned when an operation exceeds its timeout threshold
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    /// Propagates domain model errors to the repository layer
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    
    /// Catch-all for other unspecified errors
    #[error("Other error: {0}")]
    Other(String),
}

/**
 * Type alias for results of file repository operations.
 * 
 * Provides a consistent return type for all repository methods, containing
 * either a successful value or a FileRepositoryError.
 */
pub type FileRepositoryResult<T> = Result<T, FileRepositoryError>;

/**
 * Repository interface defining all file storage operations.
 * 
 * This trait represents the primary port for file operations in the domain model,
 * following the hexagonal architecture pattern. It defines the contract that any
 * file storage implementation must fulfill, abstracting away implementation details
 * like filesystem specifics, cloud storage, or database operations.
 * 
 * All implementations must be thread-safe (Send + Sync) and have a 'static lifetime
 * to support the async operations in the system.
 */
#[async_trait]
pub trait FileRepository: Send + Sync + 'static {
    /**
     * Creates and saves a new file from binary content.
     * 
     * This method handles new file creation with automatic ID generation,
     * content storage, and metadata registration.
     * 
     * @param name The filename with extension
     * @param folder_id Optional ID of parent folder, None for root
     * @param content_type MIME type of the file
     * @param content Binary data of the file
     * @return A File entity with generated metadata on success, error otherwise
     */
    async fn save_file_from_bytes(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> FileRepositoryResult<File>;
    
    /**
     * Creates and saves a new file from a stream of bytes.
     * 
     * STREAMING UPLOAD: Writes chunks directly to disk as they arrive,
     * avoiding memory accumulation for large files. This is the preferred
     * method for handling uploads of any size.
     * 
     * Benefits:
     * - Constant memory usage regardless of file size
     * - Faster time-to-first-byte for large files
     * - Better handling of slow network connections
     * 
     * @param name The filename with extension
     * @param folder_id Optional ID of parent folder, None for root
     * @param content_type MIME type of the file
     * @param stream Async stream of byte chunks
     * @return A File entity with generated metadata on success, error otherwise
     */
    async fn save_file_from_stream(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        stream: std::pin::Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>,
    ) -> FileRepositoryResult<File>;
    
    /**
     * Saves a file with a predetermined ID.
     * 
     * Similar to save_file_from_bytes but allows specifying the ID,
     * useful for restoring files or migrations.
     * 
     * @param id Predefined unique ID for the file
     * @param name The filename with extension
     * @param folder_id Optional ID of parent folder, None for root
     * @param content_type MIME type of the file
     * @param content Binary data of the file
     * @return The created File entity on success, error otherwise
     */
    async fn save_file_with_id(
        &self,
        id: String,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> FileRepositoryResult<File>;
    
    /**
     * Retrieves a file entity by its unique ID.
     * 
     * @param id The unique identifier of the file
     * @return The File entity if found, NotFound error otherwise
     */
    async fn get_file_by_id(&self, id: &str) -> FileRepositoryResult<File>;
    
    /**
     * Lists all files within a specified folder.
     * 
     * @param folder_id Optional folder ID to list files from, None for root
     * @return Vector of File entities in the folder
     */
    async fn list_files(&self, folder_id: Option<&str>) -> FileRepositoryResult<Vec<File>>;
    
    /**
     * Deletes a file by ID.
     * 
     * @param id The unique identifier of the file to delete
     * @return Success or error
     */
    async fn delete_file(&self, id: &str) -> FileRepositoryResult<()>;
    
    /**
     * Deletes a file and removes its mapping entries.
     * 
     * More thorough than delete_file as it also purges ID mappings,
     * useful for permanent deletions.
     * 
     * @param id The unique identifier of the file to delete
     * @return Success or error
     */
    async fn delete_file_entry(&self, id: &str) -> FileRepositoryResult<()>;
    
    /**
     * Retrieves the complete file content as a byte vector.
     * 
     * This method loads the entire file into memory, so it should
     * only be used for reasonably sized files.
     * 
     * @param id The unique identifier of the file
     * @return The file's binary content
     */
    async fn get_file_content(&self, id: &str) -> FileRepositoryResult<Vec<u8>>;
    
    /**
     * Retrieves file content as an asynchronous stream of bytes.
     * 
     * Preferred for large files as it avoids loading everything into memory at once.
     * 
     * @param id The unique identifier of the file
     * @return A stream that yields chunks of file data
     */
    #[allow(clippy::type_complexity)]
    async fn get_file_stream(&self, id: &str) -> FileRepositoryResult<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>;
    
    /**
     * Retrieves a range of file content as an asynchronous stream.
     * 
     * Used for HTTP Range Requests to support:
     * - Video seeking
     * - Resumable downloads
     * - Parallel chunk downloads
     * 
     * @param id The unique identifier of the file
     * @param start Starting byte position (inclusive)
     * @param end Ending byte position (inclusive), None means until EOF
     * @return A stream that yields chunks of file data for the specified range
     */
    #[allow(clippy::type_complexity)]
    async fn get_file_range_stream(
        &self, 
        id: &str, 
        start: u64, 
        end: Option<u64>
    ) -> FileRepositoryResult<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>;
    
    /**
     * Memory-maps a file for zero-copy access.
     * 
     * Uses memory-mapped I/O for efficient reading of medium-sized files (10-100MB).
     * The kernel handles page faults and caching, providing near-zero-copy performance.
     * 
     * IMPORTANT: This is synchronous I/O wrapped in spawn_blocking.
     * Best for files that will be read sequentially in full.
     * 
     * @param id The unique identifier of the file
     * @return Bytes containing the memory-mapped file content
     */
    async fn get_file_mmap(&self, id: &str) -> FileRepositoryResult<Bytes>;
    
    /**
     * Moves a file to a different folder.
     * 
     * @param id The unique identifier of the file to move
     * @param target_folder_id The destination folder ID, None for root
     * @return The updated File entity after the move
     */
    async fn move_file(&self, id: &str, target_folder_id: Option<String>) -> FileRepositoryResult<File>;
    
    /**
     * Retrieves the storage path for a file.
     * 
     * @param id The unique identifier of the file
     * @return The StoragePath object representing the file's location
     */
    async fn get_file_path(&self, id: &str) -> FileRepositoryResult<StoragePath>;
    
    /**
     * Moves a file to the trash system.
     * 
     * Instead of permanent deletion, this marks the file as trashed
     * and relocates it to the trash storage area.
     * 
     * @param file_id The unique identifier of the file to trash
     * @return Success or error
     */
    async fn move_to_trash(&self, file_id: &str) -> FileRepositoryResult<()>;
    
    /**
     * Restores a file from the trash to its original location.
     * 
     * @param file_id The unique identifier of the file to restore
     * @param original_path The original path where the file was located before trashing
     * @return Success or error
     */
    async fn restore_from_trash(&self, file_id: &str, original_path: &str) -> FileRepositoryResult<()>;
    
    /**
     * Permanently deletes a file from the trash system.
     * 
     * This operation is not reversible and removes the file completely.
     * Used primarily by the trash cleanup service.
     * 
     * @param file_id The unique identifier of the file to permanently delete
     * @return Success or error
     */
    async fn delete_file_permanently(&self, file_id: &str) -> FileRepositoryResult<()>;
    
    /**
     * Updates the content of an existing file.
     * 
     * This method replaces the binary content of a file while preserving its
     * metadata like ID, creation timestamp, and location.
     * 
     * @param file_id The unique identifier of the file to update
     * @param content The new binary content for the file
     * @return Success or error
     */
    async fn update_file_content(&self, file_id: &str, content: Vec<u8>) -> FileRepositoryResult<()>;
}