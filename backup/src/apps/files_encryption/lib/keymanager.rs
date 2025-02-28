// Key management for the encryption system
//
// This module handles storage and retrieval of encryption keys.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use async_trait::async_trait;
use glob::glob;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyManagerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Path error: {0}")]
    PathError(String),
    
    #[error("Key not found for {0}")]
    KeyNotFound(String),
    
    #[error("Failed to retrieve content: {0}")]
    ContentRetrievalError(String),
    
    #[error("Failed to set key: {0}")]
    KeySetError(String),
    
    #[error("Failed to delete key: {0}")]
    KeyDeleteError(String),
    
    #[error("Glob pattern error: {0}")]
    GlobError(String),
}

pub type KeyManagerResult<T> = Result<T, KeyManagerError>;

/// Filesystem view trait - equivalent to OC_FilesystemView in PHP
#[async_trait]
pub trait FilesystemView: Send + Sync {
    async fn file_exists(&self, path: &str) -> bool;
    async fn is_dir(&self, path: &str) -> bool;
    async fn file_get_contents(&self, path: &str) -> KeyManagerResult<Vec<u8>>;
    async fn file_put_contents(&self, path: &str, contents: &[u8]) -> KeyManagerResult<usize>;
    async fn mkdir(&self, path: &str) -> KeyManagerResult<()>;
    async fn unlink(&self, path: &str) -> KeyManagerResult<bool>;
    fn get_local_folder(&self, path: &str) -> PathBuf;
    fn get_local_file(&self, path: &str) -> PathBuf;
}

/// Utility for encryption operations
pub struct Util {
    view: Arc<dyn FilesystemView>,
    user: String,
}

impl Util {
    pub fn new(view: Arc<dyn FilesystemView>, user: String) -> Self {
        Self { view, user }
    }
    
    pub async fn get_uid_and_filename(&self, path: &str) -> (String, String) {
        // Simplified implementation - in real code would need to handle actual logic
        // for determining owner and filename
        let owner = self.user.clone();
        let filename = path.trim_start_matches('/').to_string();
        (owner, filename)
    }
    
    pub async fn is_system_wide_mount_point(&self, filename: &str) -> bool {
        // Simplified implementation - would need actual logic
        false
    }
}

/// Helper utilities for encryption
pub struct Helper;

impl Helper {
    pub fn is_partial_file_path(path: &str) -> bool {
        path.ends_with(".part")
    }
    
    pub fn strip_partial_file_extension(path: &str) -> String {
        if Self::is_partial_file_path(path) {
            path.trim_end_matches(".part").to_string()
        } else {
            path.to_string()
        }
    }
    
    pub fn escape_glob_pattern(pattern: &Path) -> String {
        // Simple implementation - actual implementation would need to handle all glob metachars
        pattern.to_string_lossy()
            .replace("[", "\\[")
            .replace("]", "\\]")
            .replace("*", "\\*")
            .replace("?", "\\?")
    }
}

/// Manager for encryption keys
pub struct KeyManager;

impl KeyManager {
    /// Retrieve the ENCRYPTED private key from a user
    ///
    /// The key returned by this method must be decrypted before use
    pub async fn get_private_key(
        view: Arc<dyn FilesystemView>,
        user: &str
    ) -> KeyManagerResult<Vec<u8>> {
        let path = format!("/{}/files_encryption/{}.private.key", user, user);
        
        if view.file_exists(&path).await {
            view.file_get_contents(&path).await
        } else {
            Err(KeyManagerError::KeyNotFound(format!("Private key for user {}", user)))
        }
    }
    
    /// Retrieve public key for a specified user
    pub async fn get_public_key(
        view: Arc<dyn FilesystemView>,
        user_id: &str
    ) -> KeyManagerResult<Vec<u8>> {
        let path = format!("/public-keys/{}.public.key", user_id);
        view.file_get_contents(&path).await
    }
    
    /// Retrieve a user's public and private key
    pub async fn get_user_keys(
        view: Arc<dyn FilesystemView>,
        user_id: &str
    ) -> KeyManagerResult<HashMap<String, Vec<u8>>> {
        let public_key = Self::get_public_key(view.clone(), user_id).await?;
        let private_key = Self::get_private_key(view.clone(), user_id).await?;
        
        let mut keys = HashMap::new();
        keys.insert("publicKey".to_string(), public_key);
        keys.insert("privateKey".to_string(), private_key);
        
        Ok(keys)
    }
    
    /// Retrieve public keys for given users
    pub async fn get_public_keys(
        view: Arc<dyn FilesystemView>,
        user_ids: &[String]
    ) -> KeyManagerResult<HashMap<String, Vec<u8>>> {
        let mut keys = HashMap::new();
        
        for user_id in user_ids {
            match Self::get_public_key(view.clone(), user_id).await {
                Ok(key) => { keys.insert(user_id.clone(), key); },
                Err(e) => return Err(e),
            }
        }
        
        Ok(keys)
    }
    
    /// Store file encryption key
    ///
    /// The keyfile is not encrypted here. Client code must
    /// asymmetrically encrypt the keyfile before passing it to this method
    pub async fn set_file_key(
        view: Arc<dyn FilesystemView>,
        path: &str,
        user_id: &str,
        catfile: &[u8]
    ) -> KeyManagerResult<bool> {
        // Get the currently logged in user and the file owner
        let util = Util::new(view.clone(), user_id.to_string());
        let (owner, filename) = util.get_uid_and_filename(path).await;
        
        // Determine base path based on whether this is a system-wide mount point
        let base_path = if util.is_system_wide_mount_point(&filename).await {
            "/files_encryption/keyfiles".to_string()
        } else {
            format!("/{}/files_encryption/keyfiles", owner)
        };
        
        let target_path = Self::key_set_preparation(view.clone(), &filename, &base_path, &owner).await?;
        
        if !view.is_dir(&format!("{}/{}", base_path, target_path)).await {
            // Create parent directories
            let info_path = PathBuf::from(format!("{}/{}", base_path, target_path));
            let parent_dir = info_path.parent().ok_or_else(|| 
                KeyManagerError::PathError("Cannot get parent directory".to_string())
            )?;
            let key_folder = view.get_local_folder(&parent_dir.to_string_lossy());
            
            if !key_folder.exists() {
                fs::create_dir_all(&key_folder)
                    .map_err(|e| KeyManagerError::Io(e))?;
            }
        }
        
        let file_path = if Helper::is_partial_file_path(&target_path) {
            format!("{}/{}.key", base_path, Helper::strip_partial_file_extension(&target_path))
        } else {
            format!("{}/{}.key", base_path, target_path)
        };
        
        match view.file_put_contents(&file_path, catfile).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
    
    /// Retrieve keyfile for an encrypted file
    pub async fn get_file_key(
        view: Arc<dyn FilesystemView>,
        user_id: &str,
        file_path: &str
    ) -> KeyManagerResult<Vec<u8>> {
        let util = Util::new(view.clone(), user_id.to_string());
        
        let (owner, filename) = util.get_uid_and_filename(file_path).await;
        let filename = Helper::strip_partial_file_extension(&filename);
        let file_path_f = filename.trim_start_matches('/');
        
        let keyfile_path = if util.is_system_wide_mount_point(&filename).await {
            format!("/files_encryption/keyfiles/{}.key", file_path_f)
        } else {
            format!("/{}/files_encryption/keyfiles/{}.key", owner, file_path_f)
        };
        
        if view.file_exists(&keyfile_path).await {
            view.file_get_contents(&keyfile_path).await
        } else {
            Err(KeyManagerError::KeyNotFound(file_path.to_string()))
        }
    }
    
    /// Delete a keyfile
    pub async fn delete_file_key(
        view: Arc<dyn FilesystemView>,
        user_id: &str,
        path: &str
    ) -> KeyManagerResult<bool> {
        let trimmed = path.trim_start_matches('/');
        
        let util = Util::new(view.clone(), user_id.to_string());
        
        let key_path = if util.is_system_wide_mount_point(path).await {
            format!("/files_encryption/keyfiles/{}", trimmed)
        } else {
            format!("/{}/files_encryption/keyfiles/{}", user_id, trimmed)
        };
        
        if view.is_dir(&key_path).await {
            view.unlink(&key_path).await
        } else if view.file_exists(&format!("{}.key", key_path)).await {
            view.unlink(&format!("{}.key", key_path)).await
        } else {
            // Log error that keyfile doesn't exist
            Err(KeyManagerError::KeyNotFound(format!("Keyfile {} does not exist", key_path)))
        }
    }
    
    /// Store private key for the user
    pub async fn set_private_key(
        key: &[u8],
        user: &str
    ) -> KeyManagerResult<usize> {
        // This would need implementation specific to the application's filesystem abstraction
        // Here's a simplified placeholder that shows the concept
        let view_path = format!("/{}/files_encryption", user);
        let view = create_filesystem_view(&view_path)?;
        
        if !view.file_exists("").await {
            view.mkdir("").await?;
        }
        
        view.file_put_contents(&format!("{}.private.key", user), key).await
    }
    
    /// Store share key
    async fn set_share_key(
        view: Arc<dyn FilesystemView>,
        path: &str,
        share_key: &[u8]
    ) -> KeyManagerResult<bool> {
        match view.file_put_contents(path, share_key).await {
            Ok(bytes_written) if bytes_written > 0 => Ok(true),
            Ok(_) => Ok(false),
            Err(e) => Err(e),
        }
    }
    
    /// Store multiple share keys for a single file
    pub async fn set_share_keys(
        view: Arc<dyn FilesystemView>,
        path: &str,
        share_keys: HashMap<String, Vec<u8>>
    ) -> KeyManagerResult<bool> {
        let util = Util::new(view.clone(), "current_user".to_string()); // Would need actual current user
        
        let (owner, filename) = util.get_uid_and_filename(path).await;
        
        let base_path = if util.is_system_wide_mount_point(&filename).await {
            "/files_encryption/share-keys".to_string()
        } else {
            format!("/{}/files_encryption/share-keys", owner)
        };
        
        let share_key_path = Self::key_set_preparation(view.clone(), &filename, &base_path, &owner).await?;
        
        let mut all_successful = true;
        
        for (user_id, share_key) in share_keys {
            let write_path = if Helper::is_partial_file_path(&share_key_path) {
                format!("{}/{}.{}.shareKey", 
                        base_path, 
                        Helper::strip_partial_file_extension(&share_key_path),
                        user_id)
            } else {
                format!("{}/{}.{}.shareKey", base_path, share_key_path, user_id)
            };
            
            if !Self::set_share_key(view.clone(), &write_path, &share_key).await? {
                all_successful = false;
            }
        }
        
        Ok(all_successful)
    }
    
    /// Retrieve shareKey for an encrypted file
    pub async fn get_share_key(
        view: Arc<dyn FilesystemView>,
        user_id: &str,
        file_path: &str
    ) -> KeyManagerResult<Vec<u8>> {
        let util = Util::new(view.clone(), "current_user".to_string()); // Would need actual current user
        
        let (owner, filename) = util.get_uid_and_filename(file_path).await;
        let filename = Helper::strip_partial_file_extension(&filename);
        
        let share_key_path = if util.is_system_wide_mount_point(&filename).await {
            format!("/files_encryption/share-keys/{}.{}.shareKey", filename, user_id)
        } else {
            format!("/{}/files_encryption/share-keys/{}.{}.shareKey", owner, filename, user_id)
        };
        
        if view.file_exists(&share_key_path).await {
            view.file_get_contents(&share_key_path).await
        } else {
            Err(KeyManagerError::KeyNotFound(format!("Share key {} not found", share_key_path)))
        }
    }
    
    /// Delete all share keys of a given file
    pub async fn del_all_share_keys(
        view: Arc<dyn FilesystemView>,
        user_id: &str,
        file_path: &str
    ) -> KeyManagerResult<()> {
        let util = Util::new(view.clone(), user_id.to_string());
        
        let base_dir = if util.is_system_wide_mount_point(file_path).await {
            "/files_encryption/share-keys/".to_string()
        } else {
            format!("{}/files_encryption/share-keys/", user_id)
        };
        
        if view.is_dir(&format!("{}/files/{}", user_id, file_path)).await {
            view.unlink(&format!("{}{}", base_dir, file_path)).await?;
        } else {
            let local_key_path = view.get_local_file(&format!("{}{}", base_dir, file_path));
            let escaped_path = Helper::escape_glob_pattern(&local_key_path);
            
            match glob(&format!("{}*.shareKey", escaped_path)) {
                Ok(paths) => {
                    for entry in paths {
                        match entry {
                            Ok(path) => {
                                if let Err(e) = fs::remove_file(&path) {
                                    return Err(KeyManagerError::Io(e));
                                }
                            },
                            Err(e) => return Err(KeyManagerError::GlobError(e.to_string())),
                        }
                    }
                },
                Err(e) => return Err(KeyManagerError::GlobError(e.to_string())),
            }
        }
        
        Ok(())
    }
    
    /// Delete a single user's shareKey for a single file
    pub async fn del_share_key(
        view: Arc<dyn FilesystemView>,
        user_ids: &[String],
        file_path: &str
    ) -> KeyManagerResult<()> {
        let util = Util::new(view.clone(), "current_user".to_string()); // Would need actual current user
        
        let (owner, filename) = util.get_uid_and_filename(file_path).await;
        
        let share_key_path = if util.is_system_wide_mount_point(&filename).await {
            Path::new("/files_encryption/share-keys/")
                .join(&filename)
                .to_string_lossy()
                .to_string()
        } else {
            Path::new("/")
                .join(&owner)
                .join("files_encryption/share-keys/")
                .join(&filename)
                .to_string_lossy()
                .to_string()
        };
        
        if view.is_dir(&share_key_path).await {
            let local_path = view.get_local_folder(&share_key_path);
            Self::recursive_del_share_keys(&local_path, user_ids)?;
        } else {
            for user_id in user_ids {
                let user_key_path = format!("{}.{}.shareKey", share_key_path, user_id);
                if let Err(e) = view.unlink(&user_key_path).await {
                    // Log that the sharekey couldn't be deleted
                    return Err(e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Recursively delete share keys from given users
    fn recursive_del_share_keys(dir: &Path, user_ids: &[String]) -> KeyManagerResult<()> {
        for user_id in user_ids {
            let extension = format!(".{}.shareKey", user_id);
            let escaped_dir = Helper::escape_glob_pattern(dir);
            let escaped_extension = glob::Pattern::escape(&extension);
            
            let pattern = format!("{}/*{}", escaped_dir, escaped_extension);
            
            match glob(&pattern) {
                Ok(paths) => {
                    for entry in paths {
                        match entry {
                            Ok(path) => {
                                if let Err(e) = fs::remove_file(&path) {
                                    return Err(KeyManagerError::Io(e));
                                }
                            },
                            Err(e) => return Err(KeyManagerError::GlobError(e.to_string())),
                        }
                    }
                },
                Err(e) => return Err(KeyManagerError::GlobError(e.to_string())),
            }
        }
        
        // Process subdirectories
        let escaped_dir = Helper::escape_glob_pattern(dir);
        match glob(&format!("{}/*", escaped_dir)) {
            Ok(paths) => {
                for entry in paths {
                    match entry {
                        Ok(path) => {
                            if path.is_dir() {
                                Self::recursive_del_share_keys(&path, user_ids)?;
                            }
                        },
                        Err(e) => return Err(KeyManagerError::GlobError(e.to_string())),
                    }
                }
            },
            Err(e) => return Err(KeyManagerError::GlobError(e.to_string())),
        }
        
        Ok(())
    }
    
    /// Make preparations to vars and filesystem for saving a keyfile
    pub async fn key_set_preparation(
        view: Arc<dyn FilesystemView>,
        path: &str,
        base_path: &str,
        user_id: &str
    ) -> KeyManagerResult<String> {
        let target_path = path.trim_start_matches('/').to_string();
        
        let path_parts = Path::new(&target_path);
        
        // If the file resides within a subdirectory, create it
        if let Some(dirname) = path_parts.parent() {
            if !dirname.as_os_str().is_empty() && !view.file_exists(&format!("{}/{}", base_path, dirname.to_string_lossy())).await {
                let sub_dirs: Vec<&str> = format!("{}/{}", base_path, dirname.to_string_lossy())
                    .split('/')
                    .filter(|s| !s.is_empty())
                    .collect();
                
                let mut dir = String::new();
                for sub_dir in sub_dirs {
                    dir = format!("{}/{}", dir, sub_dir);
                    if !view.is_dir(&dir).await {
                        view.mkdir(&dir).await?;
                    }
                }
            }
        }
        
        Ok(target_path)
    }
}

// This function is just a placeholder for the example
// In a real implementation, you would have actual FilesystemView implementation
fn create_filesystem_view(_path: &str) -> KeyManagerResult<Arc<dyn FilesystemView>> {
    Err(KeyManagerError::PathError("Filesystem view creation not implemented".to_string()))
}