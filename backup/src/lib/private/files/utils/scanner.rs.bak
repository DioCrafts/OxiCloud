// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::sync::{Arc, Weak};
use async_trait::async_trait;

/// Hooks available in scope \OC\Utils\Scanner
///  - scan_file(string $absolute_path)
///  - scan_folder(string $absolute_path)
pub struct Scanner {
    /// The username
    user: String,
    
    /// Event emitter
    emitter: Arc<PublicEmitter>,
}

impl Scanner {
    /// Create a new scanner for the given user
    pub fn new(user: String) -> Self {
        Self {
            user,
            emitter: Arc::new(PublicEmitter::new()),
        }
    }

    /// Get all storages for the given directory
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory to get mounts for
    ///
    /// # Returns
    ///
    /// A vector of mounts
    async fn get_mounts(&self, dir: &str) -> Result<Vec<Arc<Mount>>, FileSystemError> {
        // TODO: move to the node based fileapi once that's done
        Util::tear_down_fs().await?;
        Util::setup_fs(&self.user).await?;
        
        let view = Filesystem::get_view().await?;
        let absolute_path = view.get_absolute_path(dir).await?;

        let mount_manager = Filesystem::get_mount_manager().await?;
        let mut mounts = mount_manager.find_in(&absolute_path).await?;
        let dir_mount = mount_manager.find(&absolute_path).await?;
        mounts.push(dir_mount);
        
        // Start with the mount of $dir
        mounts.reverse();
        
        Ok(mounts)
    }

    /// Attach listeners to the scanner
    ///
    /// # Arguments
    ///
    /// * `mount` - The mount to attach listeners to
    async fn attach_listener(&self, mount: Arc<Mount>) -> Result<(), FileSystemError> {
        let storage = match mount.get_storage().await? {
            Some(storage) => storage,
            None => return Ok(()),
        };
        
        let scanner = storage.get_scanner().await?;
        let emitter_weak = Arc::downgrade(&self.emitter);
        let mount_weak = Arc::downgrade(&mount);
        
        // Setup scan_file listener
        scanner.listen("\\OC\\Files\\Cache\\Scanner".to_string(), "scanFile".to_string(), 
            Box::new(move |path: String| {
                let emitter = match emitter_weak.upgrade() {
                    Some(e) => e,
                    None => return,
                };
                let mount = match mount_weak.upgrade() {
                    Some(m) => m,
                    None => return,
                };
                
                let full_path = format!("{}{}", mount.get_mount_point(), path);
                emitter.emit(
                    "\\OC\\Files\\Utils\\Scanner".to_string(),
                    "scanFile".to_string(), 
                    vec![full_path]
                );
            })
        ).await?;
        
        // Setup scan_folder listener
        let emitter_weak = Arc::downgrade(&self.emitter);
        let mount_weak = Arc::downgrade(&mount);
        
        scanner.listen("\\OC\\Files\\Cache\\Scanner".to_string(), "scanFolder".to_string(), 
            Box::new(move |path: String| {
                let emitter = match emitter_weak.upgrade() {
                    Some(e) => e,
                    None => return,
                };
                let mount = match mount_weak.upgrade() {
                    Some(m) => m,
                    None => return,
                };
                
                let full_path = format!("{}{}", mount.get_mount_point(), path);
                emitter.emit(
                    "\\OC\\Files\\Utils\\Scanner".to_string(),
                    "scanFolder".to_string(), 
                    vec![full_path]
                );
            })
        ).await?;
        
        Ok(())
    }

    /// Perform a background scan of the given directory
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory to scan
    pub async fn background_scan(&self, dir: &str) -> Result<(), FileSystemError> {
        let mounts = self.get_mounts(dir).await?;
        
        for mount in mounts {
            let storage = match mount.get_storage().await? {
                Some(storage) => storage,
                None => continue,
            };
            
            let scanner = storage.get_scanner().await?;
            self.attach_listener(mount).await?;
            scanner.background_scan().await?;
        }
        
        Ok(())
    }

    /// Scan the given directory
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory to scan
    pub async fn scan(&self, dir: &str) -> Result<(), FileSystemError> {
        let mounts = self.get_mounts(dir).await?;
        
        for mount in mounts {
            let storage = match mount.get_storage().await? {
                Some(storage) => storage,
                None => continue,
            };
            
            let scanner = storage.get_scanner().await?;
            self.attach_listener(mount).await?;
            scanner.scan("", ScannerOption::ScanRecursive, ScannerOption::ReuseEtag).await?;
        }
        
        Ok(())
    }
}

// Required types and traits to make the above work

#[async_trait]
pub trait Listener: Send + Sync {
    async fn listen(&self, scope: String, method: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), FileSystemError>;
}

pub struct PublicEmitter {
    // Implementation details
}

impl PublicEmitter {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn emit(&self, scope: String, method: String, args: Vec<String>) {
        // Implementation details
    }
}

pub struct Mount {
    // Implementation details
}

impl Mount {
    pub async fn get_storage(&self) -> Result<Option<Arc<Storage>>, FileSystemError> {
        // Implementation details
        Ok(None)
    }
    
    pub fn get_mount_point(&self) -> &str {
        // Implementation details
        ""
    }
}

pub struct Storage {
    // Implementation details
}

impl Storage {
    pub async fn get_scanner(&self) -> Result<Arc<FileScanner>, FileSystemError> {
        // Implementation details
        unimplemented!()
    }
}

pub struct FileScanner {
    // Implementation details
}

#[async_trait]
impl Listener for FileScanner {
    async fn listen(&self, scope: String, method: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), FileSystemError> {
        // Implementation details
        Ok(())
    }
}

impl FileScanner {
    pub async fn background_scan(&self) -> Result<(), FileSystemError> {
        // Implementation details
        Ok(())
    }
    
    pub async fn scan(&self, path: &str, scan_recursive: ScannerOption, reuse_etag: ScannerOption) -> Result<(), FileSystemError> {
        // Implementation details
        Ok(())
    }
}

pub enum ScannerOption {
    ScanRecursive,
    ReuseEtag,
}

pub struct Filesystem {
    // Implementation details
}

impl Filesystem {
    pub async fn get_view() -> Result<Arc<FilesystemView>, FileSystemError> {
        // Implementation details
        unimplemented!()
    }
    
    pub async fn get_mount_manager() -> Result<Arc<MountManager>, FileSystemError> {
        // Implementation details
        unimplemented!()
    }
}

pub struct FilesystemView {
    // Implementation details
}

impl FilesystemView {
    pub async fn get_absolute_path(&self, path: &str) -> Result<String, FileSystemError> {
        // Implementation details
        unimplemented!()
    }
}

pub struct MountManager {
    // Implementation details
}

impl MountManager {
    pub async fn find_in(&self, path: &str) -> Result<Vec<Arc<Mount>>, FileSystemError> {
        // Implementation details
        unimplemented!()
    }
    
    pub async fn find(&self, path: &str) -> Result<Arc<Mount>, FileSystemError> {
        // Implementation details
        unimplemented!()
    }
}

pub struct Util {
    // Implementation details
}

impl Util {
    pub async fn tear_down_fs() -> Result<(), FileSystemError> {
        // Implementation details
        Ok(())
    }
    
    pub async fn setup_fs(user: &str) -> Result<(), FileSystemError> {
        // Implementation details
        Ok(())
    }
}

#[derive(Debug)]
pub enum FileSystemError {
    // Various error types
    IoError(std::io::Error),
    NotFound,
    PermissionDenied,
    Other(String),
}

impl From<std::io::Error> for FileSystemError {
    fn from(err: std::io::Error) -> Self {
        FileSystemError::IoError(err)
    }
}