use tokio::fs::{self, OpenOptions, File};
use tokio::io::AsyncWriteExt;
use std::path::Path;
use std::io::Error as IoError;
use tempfile::NamedTempFile;
use tracing::{warn, error};

/// Utility functions for file system operations with proper synchronization
pub struct FileSystemUtils;

impl FileSystemUtils {
    /// Writes data to a file with fsync to ensure durability
    /// Uses a safe atomic write pattern: write to temp file, fsync, rename
    pub async fn atomic_write<P: AsRef<Path>>(path: P, contents: &[u8]) -> Result<(), IoError> {
        let path = path.as_ref();
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        // Create a temporary file in the same directory
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let temp_file = match NamedTempFile::new_in(dir) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to create temporary file in {}: {}", dir.display(), e);
                return Err(IoError::other(format!("Failed to create temporary file: {}", e)));
            }
        };
        
        let temp_path = temp_file.path().to_path_buf();
        
        // Convert to tokio file and write contents
        let std_file = temp_file.as_file().try_clone()?;
        let mut file = File::from_std(std_file);
        file.write_all(contents).await?;
        
        // Ensure data is synced to disk
        file.flush().await?;
        file.sync_all().await?;
        
        // Rename the temporary file to the target path (atomic operation on most filesystems)
        fs::rename(&temp_path, path).await?;
        
        // Sync the directory to ensure the rename is persisted
        if let Some(parent) = path.parent() {
            match Self::sync_directory(parent).await {
                Ok(_) => {},
                Err(e) => {
                    warn!("Failed to sync directory {}: {}. File was written but directory entry might not be durable.", 
                          parent.display(), e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Creates or appends to a file with fsync
    pub async fn write_with_sync<P: AsRef<Path>>(path: P, contents: &[u8], append: bool) -> Result<(), IoError> {
        let path = path.as_ref();
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        // Open file with appropriate options
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(!append)
            .append(append)
            .open(path)
            .await?;
        
        // Write contents
        file.write_all(contents).await?;
        
        // Ensure data is synced to disk
        file.flush().await?;
        file.sync_all().await?;
        
        Ok(())
    }
    
    /// Creates directories with fsync
    pub async fn create_dir_with_sync<P: AsRef<Path>>(path: P) -> Result<(), IoError> {
        let path = path.as_ref();
        
        // Create directory
        fs::create_dir_all(path).await?;
        
        // Sync the directory
        Self::sync_directory(path).await?;
        
        // Sync parent directory to ensure directory creation is persisted
        if let Some(parent) = path.parent() {
            match Self::sync_directory(parent).await {
                Ok(_) => {},
                Err(e) => {
                    warn!("Failed to sync parent directory {}: {}. Directory was created but entry might not be durable.", 
                          parent.display(), e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Renames a file or directory with proper syncing
    pub async fn rename_with_sync<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<(), IoError> {
        let from = from.as_ref();
        let to = to.as_ref();
        
        // Ensure parent directory of destination exists
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        // Perform rename
        fs::rename(from, to).await?;
        
        // Sync parent directories to ensure rename is persisted
        if let Some(from_parent) = from.parent() {
            match Self::sync_directory(from_parent).await {
                Ok(_) => {},
                Err(e) => {
                    warn!("Failed to sync source directory {}: {}. Rename completed but might not be durable.", 
                          from_parent.display(), e);
                }
            }
        }
        
        if let Some(to_parent) = to.parent() {
            match Self::sync_directory(to_parent).await {
                Ok(_) => {},
                Err(e) => {
                    warn!("Failed to sync destination directory {}: {}. Rename completed but might not be durable.", 
                          to_parent.display(), e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Removes a file with directory syncing
    pub async fn remove_file_with_sync<P: AsRef<Path>>(path: P) -> Result<(), IoError> {
        let path = path.as_ref();
        
        // Remove file
        fs::remove_file(path).await?;
        
        // Sync parent directory to ensure removal is persisted
        if let Some(parent) = path.parent() {
            match Self::sync_directory(parent).await {
                Ok(_) => {},
                Err(e) => {
                    warn!("Failed to sync directory after file removal {}: {}. File was removed but entry might not be durable.", 
                          parent.display(), e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Removes a directory with parent directory syncing
    pub async fn remove_dir_with_sync<P: AsRef<Path>>(path: P, recursive: bool) -> Result<(), IoError> {
        let path = path.as_ref();
        
        // Remove directory
        if recursive {
            fs::remove_dir_all(path).await?;
        } else {
            fs::remove_dir(path).await?;
        }
        
        // Sync parent directory to ensure removal is persisted
        if let Some(parent) = path.parent() {
            match Self::sync_directory(parent).await {
                Ok(_) => {},
                Err(e) => {
                    warn!("Failed to sync directory after directory removal {}: {}. Directory was removed but entry might not be durable.", 
                          parent.display(), e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Syncs a directory to ensure its contents are durable
    async fn sync_directory<P: AsRef<Path>>(path: P) -> Result<(), IoError> {
        let path = path.as_ref();
        
        // Open directory with read permissions
        let dir_file = match OpenOptions::new()
            .read(true)
            .open(path)
            .await {
                Ok(file) => file,
                Err(e) => {
                    warn!("Failed to open directory for syncing {}: {}", path.display(), e);
                    return Err(e);
                }
            };
        
        // Sync the directory
        dir_file.sync_all().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;
    use tokio::io::AsyncReadExt;
    
    #[tokio::test]
    async fn test_atomic_write() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        // Write data atomically
        FileSystemUtils::atomic_write(&file_path, b"Hello, world!").await.unwrap();
        
        // Read back the data
        let mut file = fs::File::open(&file_path).await.unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.unwrap();
        
        assert_eq!(contents, "Hello, world!");
    }
    
    #[tokio::test]
    async fn test_write_with_sync() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        // Write data with sync
        FileSystemUtils::write_with_sync(&file_path, b"First line\n", false).await.unwrap();
        
        // Append data
        FileSystemUtils::write_with_sync(&file_path, b"Second line", true).await.unwrap();
        
        // Read back the data
        let mut file = fs::File::open(&file_path).await.unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.unwrap();
        
        assert_eq!(contents, "First line\nSecond line");
    }
    
    #[tokio::test]
    async fn test_rename_with_sync() {
        let temp_dir = tempdir().unwrap();
        let source_path = temp_dir.path().join("source.txt");
        let dest_path = temp_dir.path().join("dest.txt");
        
        // Create source file
        FileSystemUtils::write_with_sync(&source_path, b"Test content", false).await.unwrap();
        
        // Rename file
        FileSystemUtils::rename_with_sync(&source_path, &dest_path).await.unwrap();
        
        // Verify source doesn't exist
        assert!(!source_path.exists());
        
        // Verify destination exists
        let mut file = fs::File::open(&dest_path).await.unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.unwrap();
        
        assert_eq!(contents, "Test content");
    }
}