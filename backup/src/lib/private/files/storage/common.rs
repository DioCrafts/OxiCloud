// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::io::{Read, Write, Result as IoResult};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashSet;

/// Storage backend trait for providing common filesystem operation methods
pub trait Storage: Send + Sync {
    fn stat(&self, path: &str) -> Option<FileStat>;
    fn filetype(&self, path: &str) -> Option<String>;
    fn file_exists(&self, path: &str) -> bool;
    fn opendir(&self, path: &str) -> Option<Box<dyn Iterator<Item = String> + Send>>;
    fn mkdir(&self, path: &str) -> bool;
    fn rmdir(&self, path: &str) -> bool;
    fn unlink(&self, path: &str) -> bool;
    fn fopen(&self, path: &str, mode: &str) -> Option<Box<dyn StorageFile>>;
    fn touch(&self, path: &str, mtime: Option<u64>) -> bool;
    fn is_readable(&self, path: &str) -> bool;
    fn is_updatable(&self, path: &str) -> bool;
}

pub trait StorageFile: Read + Write + Send + Sync {}

#[derive(Debug, Clone)]
pub struct FileStat {
    pub size: u64,
    pub mtime: u64,
}

pub const PERMISSION_CREATE: u8 = 0x04;
pub const PERMISSION_READ: u8 = 0x01;
pub const PERMISSION_UPDATE: u8 = 0x02;
pub const PERMISSION_DELETE: u8 = 0x08;
pub const PERMISSION_SHARE: u8 = 0x10;

pub const SPACE_UNKNOWN: i64 = -1;

/// Storage backend class for providing common filesystem operation methods
/// which are not storage-backend specific.
///
/// Common is never used directly; it is extended by all other
/// storage backends, where its methods may be overridden, and additional
/// (backend-specific) methods are defined.
///
/// Some Common methods call functions which are first defined
/// in classes which extend it, e.g. self.stat() .
pub struct Common {
    cache: Option<Cache>,
    scanner: Option<Scanner>,
    permission_cache: Option<PermissionsCache>,
    watcher: Option<Watcher>,
    storage_cache: Option<StorageCache>,
}

pub struct Cache {}
pub struct Scanner {}
pub struct PermissionsCache {}
pub struct Watcher {}
pub struct StorageCache {}

impl Common {
    pub fn new(_parameters: &[(&str, &str)]) -> Self {
        Self {
            cache: None,
            scanner: None,
            permission_cache: None,
            watcher: None,
            storage_cache: None,
        }
    }

    pub fn is_dir(&self, path: &str) -> bool {
        if let Some(filetype) = self.filetype(path) {
            return filetype == "dir";
        }
        false
    }

    pub fn is_file(&self, path: &str) -> bool {
        if let Some(filetype) = self.filetype(path) {
            return filetype == "file";
        }
        false
    }

    pub fn filesize(&self, path: &str) -> u64 {
        if self.is_dir(path) {
            return 0; // by definition
        } else if let Some(stat) = self.stat(path) {
            return stat.size;
        }
        0
    }

    pub fn is_creatable(&self, path: &str) -> bool {
        self.is_dir(path) && self.is_updatable(path)
    }

    pub fn is_deletable(&self, path: &str) -> bool {
        self.is_updatable(path)
    }

    pub fn is_sharable(&self, path: &str) -> bool {
        self.is_readable(path)
    }

    pub fn get_permissions(&self, path: &str) -> u8 {
        let mut permissions = 0;
        if self.is_creatable(path) {
            permissions |= PERMISSION_CREATE;
        }
        if self.is_readable(path) {
            permissions |= PERMISSION_READ;
        }
        if self.is_updatable(path) {
            permissions |= PERMISSION_UPDATE;
        }
        if self.is_deletable(path) {
            permissions |= PERMISSION_DELETE;
        }
        if self.is_sharable(path) {
            permissions |= PERMISSION_SHARE;
        }
        permissions
    }

    pub fn filemtime(&self, path: &str) -> u64 {
        if let Some(stat) = self.stat(path) {
            return stat.mtime;
        }
        0
    }

    pub fn file_get_contents(&self, path: &str) -> Option<Vec<u8>> {
        let mut handle = self.fopen(path, "r")?;
        let size = self.filesize(path) as usize;
        if size == 0 {
            return Some(Vec::new());
        }
        
        let mut buffer = vec![0u8; size];
        match handle.read_exact(&mut buffer) {
            Ok(_) => Some(buffer),
            Err(_) => None,
        }
    }

    pub fn file_put_contents(&self, path: &str, data: &[u8]) -> Option<usize> {
        let mut handle = self.fopen(path, "w")?;
        match handle.write(data) {
            Ok(written) => Some(written),
            Err(_) => None,
        }
    }

    pub fn rename(&self, path1: &str, path2: &str) -> bool {
        if self.copy(path1, path2) {
            return self.unlink(path1);
        }
        false
    }

    pub fn copy(&self, path1: &str, path2: &str) -> bool {
        let source = match self.fopen(path1, "r") {
            Some(file) => file,
            None => return false,
        };
        
        let target = match self.fopen(path2, "w") {
            Some(file) => file,
            None => return false,
        };
        
        match stream_copy(source, target) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Deletes all files and folders recursively within a directory
    ///
    /// # Arguments
    ///
    /// * `directory` - The directory whose contents will be deleted
    /// * `empty` - Flag indicating whether directory will be emptied
    ///
    /// # Note
    ///
    /// By default the directory specified by `directory` will be
    /// deleted together with its contents. To avoid this set `empty` to true
    pub fn delete_all(&self, directory: &str, empty: bool) -> bool {
        let directory = directory.trim_start_matches('/');
        
        if !self.is_dir(directory) || !self.is_readable(directory) {
            return false;
        }
        
        let dir_handle = match self.opendir(directory) {
            Some(handle) => handle,
            None => return false,
        };
        
        for contents in dir_handle {
            if is_ignored_dir(&contents) {
                continue;
            }
            
            let path = format!("{}/{}", directory, contents);
            if self.is_dir(&path) {
                if !self.delete_all(&path, false) {
                    return false;
                }
            } else if !self.unlink(&path) {
                return false;
            }
        }
        
        if !empty && !self.rmdir(directory) {
            return false;
        }
        
        true
    }

    pub fn get_mime_type(&self, path: &str) -> Option<String> {
        if !self.file_exists(path) {
            return None;
        }
        
        if self.is_dir(path) {
            return Some("httpd/unix-directory".to_string());
        }
        
        let source = self.fopen(path, "r")?;
        let mut head = vec![0u8; 8192]; // 8kb should suffice to determine a mimetype
        
        // Read up to 8kb of data
        match source.read(&mut head) {
            Ok(bytes_read) => {
                head.truncate(bytes_read);
            }
            Err(_) => return None,
        }
        
        let extension = if let Some(pos) = path.rfind('.') {
            &path[pos..]
        } else {
            ""
        };
        
        let tmp_file = tmp_file(extension);
        if let Err(_) = std::fs::write(&tmp_file, &head) {
            return None;
        }
        
        let mime = get_mime_type(&tmp_file);
        let _ = std::fs::remove_file(tmp_file); // Ignore errors on cleanup
        
        Some(mime)
    }

    pub fn hash(&self, hash_type: &str, path: &str, raw: bool) -> Option<String> {
        let tmp_file = self.get_local_file(path)?;
        let hash = calculate_hash(hash_type, &tmp_file, raw)?;
        let _ = std::fs::remove_file(tmp_file); // Ignore errors on cleanup
        Some(hash)
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        self.search_in_dir(query, "")
    }

    pub fn get_local_file(&self, path: &str) -> Option<PathBuf> {
        self.to_tmp_file(path)
    }

    fn to_tmp_file(&self, path: &str) -> Option<PathBuf> {
        let source = self.fopen(path, "r")?;
        
        let extension = if let Some(pos) = path.rfind('.') {
            &path[pos..]
        } else {
            ""
        };
        
        let tmp_file = tmp_file(extension);
        let mut target = File::create(&tmp_file).ok()?;
        
        if let Err(_) = stream_copy_std(source, &mut target) {
            return None;
        }
        
        Some(tmp_file)
    }

    pub fn get_local_folder(&self, path: &str) -> Option<PathBuf> {
        let base_dir = tmp_folder()?;
        self.add_local_folder(path, &base_dir).ok()?;
        Some(base_dir)
    }

    fn add_local_folder(&self, path: &str, target: &Path) -> IoResult<()> {
        let dir_handle = self.opendir(path)?;
        
        for file in dir_handle {
            if file == "." || file == ".." {
                continue;
            }
            
            let source_path = format!("{}/{}", path, file);
            let target_path = target.join(&file);
            
            if self.is_dir(&source_path) {
                fs::create_dir_all(&target_path)?;
                self.add_local_folder(&source_path, &target_path)?;
            } else {
                if let Some(tmp) = self.to_tmp_file(&source_path) {
                    fs::rename(tmp, target_path)?;
                }
            }
        }
        
        Ok(())
    }

    fn search_in_dir(&self, query: &str, dir: &str) -> Vec<String> {
        let mut files = Vec::new();
        
        let dir_handle = match self.opendir(dir) {
            Some(handle) => handle,
            None => return files,
        };
        
        let query_lower = query.to_lowercase();
        
        for item in dir_handle {
            if item == "." || item == ".." {
                continue;
            }
            
            let path = if dir.is_empty() {
                item.clone()
            } else {
                format!("{}/{}", dir, item)
            };
            
            if item.to_lowercase().contains(&query_lower) {
                files.push(path.clone());
            }
            
            if self.is_dir(&path) {
                let subdir_files = self.search_in_dir(query, &path);
                files.extend(subdir_files);
            }
        }
        
        files
    }

    /// Check if a file or folder has been updated since a certain time
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check
    /// * `time` - Unix timestamp to compare against
    pub fn has_updated(&self, path: &str, time: u64) -> bool {
        self.filemtime(path) > time
    }

    pub fn get_cache(&mut self, _path: &str) -> &mut Cache {
        if self.cache.is_none() {
            self.cache = Some(Cache {});
        }
        self.cache.as_mut().unwrap()
    }

    pub fn get_scanner(&mut self, _path: &str) -> &mut Scanner {
        if self.scanner.is_none() {
            self.scanner = Some(Scanner {});
        }
        self.scanner.as_mut().unwrap()
    }

    pub fn get_permissions_cache(&mut self, _path: &str) -> &mut PermissionsCache {
        if self.permission_cache.is_none() {
            self.permission_cache = Some(PermissionsCache {});
        }
        self.permission_cache.as_mut().unwrap()
    }

    pub fn get_watcher(&mut self, _path: &str) -> &mut Watcher {
        if self.watcher.is_none() {
            self.watcher = Some(Watcher {});
        }
        self.watcher.as_mut().unwrap()
    }

    pub fn get_storage_cache(&mut self) -> &mut StorageCache {
        if self.storage_cache.is_none() {
            self.storage_cache = Some(StorageCache {});
        }
        self.storage_cache.as_mut().unwrap()
    }

    /// Get the owner of a path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to get the owner
    ///
    /// # Returns
    ///
    /// Username or None
    pub fn get_owner(&self, _path: &str) -> Option<String> {
        get_current_user()
    }

    /// Get the ETag for a file or folder
    ///
    /// # Arguments
    ///
    /// * `path` - The path to get the ETag for
    pub fn get_etag(&self, path: &str) -> String {
        if let Some(etag_fn) = get_etag_function() {
            etag_fn(path)
        } else {
            generate_unique_id()
        }
    }

    /// Clean a path, i.e. remove all redundant '.' and '..'
    /// making sure that it can't point higher than '/'
    ///
    /// # Arguments
    ///
    /// * `path` - The path to clean
    ///
    /// # Returns
    ///
    /// Cleaned path
    pub fn clean_path(&self, path: &str) -> String {
        let path = if path.is_empty() || !path.starts_with('/') {
            format!("/{}", path)
        } else {
            path.to_string()
        };
        
        let mut output = Vec::new();
        
        for chunk in path.split('/') {
            match chunk {
                ".." => {
                    if !output.is_empty() {
                        output.pop();
                    }
                },
                "." | "" => {},
                _ => output.push(chunk),
            }
        }
        
        format!("/{}", output.join("/"))
    }

    pub fn test(&self) -> bool {
        self.stat("").is_some()
    }

    /// Get the free space in the storage
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check
    pub fn free_space(&self, _path: &str) -> i64 {
        SPACE_UNKNOWN
    }
}

impl Storage for Common {
    fn stat(&self, _path: &str) -> Option<FileStat> {
        // Implementation must be provided by the storage backend
        None
    }

    fn filetype(&self, _path: &str) -> Option<String> {
        // Implementation must be provided by the storage backend
        None
    }
    
    fn file_exists(&self, _path: &str) -> bool {
        // Implementation must be provided by the storage backend
        false
    }
    
    fn opendir(&self, _path: &str) -> Option<Box<dyn Iterator<Item = String> + Send>> {
        // Implementation must be provided by the storage backend
        None
    }
    
    fn mkdir(&self, _path: &str) -> bool {
        // Implementation must be provided by the storage backend
        false
    }
    
    fn rmdir(&self, _path: &str) -> bool {
        // Implementation must be provided by the storage backend
        false
    }
    
    fn unlink(&self, _path: &str) -> bool {
        // Implementation must be provided by the storage backend
        false
    }
    
    fn fopen(&self, _path: &str, _mode: &str) -> Option<Box<dyn StorageFile>> {
        // Implementation must be provided by the storage backend
        None
    }
    
    fn touch(&self, _path: &str, _mtime: Option<u64>) -> bool {
        // Implementation must be provided by the storage backend
        false
    }
    
    fn is_readable(&self, _path: &str) -> bool {
        // Implementation must be provided by the storage backend
        false
    }
    
    fn is_updatable(&self, _path: &str) -> bool {
        // Implementation must be provided by the storage backend
        false
    }
}

// Helper functions

fn is_ignored_dir(dir: &str) -> bool {
    // This should be implemented based on OC\Files\Filesystem::isIgnoredDir
    dir == "." || dir == ".."
}

fn stream_copy<R: Read, W: Write>(mut source: R, mut target: W) -> IoResult<(u64, bool)> {
    let mut buffer = [0; 8192];
    let mut total_read = 0;
    
    loop {
        match source.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(bytes_read) => {
                total_read += bytes_read as u64;
                target.write_all(&buffer[..bytes_read])?;
            },
            Err(e) => return Err(e),
        }
    }
    
    Ok((total_read, true))
}

fn stream_copy_std(mut source: Box<dyn StorageFile>, target: &mut File) -> IoResult<(u64, bool)> {
    let mut buffer = [0; 8192];
    let mut total_read = 0;
    
    loop {
        match source.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(bytes_read) => {
                total_read += bytes_read as u64;
                target.write_all(&buffer[..bytes_read])?;
            },
            Err(e) => return Err(e),
        }
    }
    
    Ok((total_read, true))
}

fn tmp_file(extension: &str) -> PathBuf {
    // This is a simplified version - in a real implementation 
    // you'd want to use a proper temporary file mechanism
    let mut path = std::env::temp_dir();
    let filename = format!("oc_tmp_{}{}", generate_unique_id(), extension);
    path.push(filename);
    path
}

fn tmp_folder() -> Option<PathBuf> {
    let mut path = std::env::temp_dir();
    let dirname = format!("oc_tmp_folder_{}", generate_unique_id());
    path.push(dirname);
    
    match fs::create_dir_all(&path) {
        Ok(_) => Some(path),
        Err(_) => None,
    }
}

fn get_mime_type(path: &Path) -> String {
    // This would be a more complex implementation in a real app
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    
    match extension {
        "txt" => "text/plain".to_string(),
        "html" | "htm" => "text/html".to_string(),
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "pdf" => "application/pdf".to_string(),
        "zip" => "application/zip".to_string(),
        "json" => "application/json".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

fn calculate_hash(hash_type: &str, path: &Path, raw: bool) -> Option<String> {
    use sha2::{Sha256, Digest};
    
    let data = std::fs::read(path).ok()?;
    
    match hash_type {
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(&data);
            let result = hasher.finalize();
            
            if raw {
                Some(String::from_utf8_lossy(&result).to_string())
            } else {
                Some(format!("{:x}", result))
            }
        },
        // Add more hash algorithms as needed
        _ => None,
    }
}

fn get_current_user() -> Option<String> {
    // Simplified implementation - in a real app, this would be 
    // more complex and involve a user session management system
    Some("admin".to_string())
}

fn get_etag_function() -> Option<fn(&str) -> String> {
    // In a real application, this would be set up based on config
    None
}

fn generate_unique_id() -> String {
    // Simple unique ID generation - in a real application you might want to use UUIDs
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    
    format!("{:x}", now)
}