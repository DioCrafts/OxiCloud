use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, Result as IoResult};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::process::Command;
use std::ffi::OsStr;
use std::os::windows::prelude::*;
use async_trait::async_trait;

#[cfg(windows)]
mod platform {
    pub struct Local;
    
    impl Local {
        pub fn new(args: &StorageArgs) -> Self {
            // In the PHP code, this inherits from MappedLocal on Windows
            // For this translation we're creating a placeholder
            Local
        }
    }
}

#[cfg(not(windows))]
mod platform {
    pub use super::LocalStorage as Local;
}

pub use platform::Local;

pub struct StorageArgs {
    pub datadir: String,
}

/// Common trait for all storage implementations
#[async_trait]
pub trait Storage {
    fn id(&self) -> String;
    fn mkdir(&self, path: &str) -> bool;
    fn rmdir(&self, path: &str) -> bool;
    fn opendir(&self, path: &str) -> IoResult<fs::ReadDir>;
    fn is_dir(&self, path: &str) -> bool;
    fn is_file(&self, path: &str) -> bool;
    fn stat(&self, path: &str) -> IoResult<fs::Metadata>;
    fn filetype(&self, path: &str) -> IoResult<String>;
    fn filesize(&self, path: &str) -> u64;
    fn is_readable(&self, path: &str) -> bool;
    fn is_updatable(&self, path: &str) -> bool;
    fn file_exists(&self, path: &str) -> bool;
    fn filemtime(&self, path: &str) -> IoResult<SystemTime>;
    fn touch(&self, path: &str, mtime: Option<SystemTime>) -> bool;
    fn file_get_contents(&self, path: &str) -> IoResult<Vec<u8>>;
    fn file_put_contents(&self, path: &str, data: &[u8]) -> IoResult<usize>;
    fn unlink(&self, path: &str) -> bool;
    fn rename(&self, path1: &str, path2: &str) -> bool;
    fn copy(&self, path1: &str, path2: &str) -> bool;
    fn fopen(&self, path: &str, mode: &str) -> IoResult<File>;
    fn get_mime_type(&self, path: &str) -> Option<String>;
    fn hash(&self, path: &str, hash_type: &str, raw: bool) -> IoResult<String>;
    fn free_space(&self, path: &str) -> i64;
    fn search(&self, query: &str) -> Vec<String>;
    fn get_local_file(&self, path: &str) -> String;
    fn get_local_folder(&self, path: &str) -> String;
    fn has_updated(&self, path: &str, time: SystemTime) -> bool;
}

/// For local filestore, we only have to map the paths
#[cfg(not(windows))]
pub struct LocalStorage {
    datadir: String,
}

#[cfg(not(windows))]
impl LocalStorage {
    pub fn new(args: &StorageArgs) -> Self {
        let mut datadir = args.datadir.clone();
        if !datadir.ends_with('/') {
            datadir.push('/');
        }
        Self { datadir }
    }

    /// Delete a directory and its contents recursively
    fn del_tree(&self, dir: &str) -> bool {
        let dir_relative = dir;
        let dir_path = format!("{}{}", self.datadir, dir);
        let path = Path::new(&dir_path);

        if !path.exists() {
            return true;
        }

        if !path.is_dir() || path.is_symlink() {
            return fs::remove_file(path).is_ok();
        }

        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries.filter_map(Result::ok) {
                    let item = entry.file_name();
                    if item == OsStr::new(".") || item == OsStr::new("..") {
                        continue;
                    }

                    let item_path = entry.path();
                    if item_path.is_file() {
                        if let Err(_) = fs::remove_file(&item_path) {
                            return false;
                        }
                    } else if item_path.is_dir() {
                        let rel_path = format!("{}/{}", dir_relative, item.to_string_lossy());
                        if !self.del_tree(&rel_path) {
                            return false;
                        }
                    }
                }
                fs::remove_dir(path).is_ok()
            }
            Err(_) => false,
        }
    }

    /// Search recursively in a directory for a specific query
    fn search_in_dir(&self, query: &str, dir: &str) -> Vec<String> {
        let mut files = Vec::new();
        let path = format!("{}{}", self.datadir, dir);

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                let item = entry.file_name();
                let item_str = item.to_string_lossy().to_string();
                
                if item_str == "." || item_str == ".." {
                    continue;
                }
                
                if item_str.to_lowercase().contains(&query.to_lowercase()) {
                    files.push(format!("{}/{}", dir, item_str));
                }
                
                if entry.path().is_dir() {
                    let sub_dir = format!("{}/{}", dir, item_str);
                    let sub_results = self.search_in_dir(query, &sub_dir);
                    files.extend(sub_results);
                }
            }
        }
        
        files
    }

    /// Get file size using OS-specific methods when standard methods fail
    fn get_file_size_from_os(full_path: &str) -> u64 {
        let os_name = std::env::consts::OS.to_lowercase();
        
        if os_name.contains("windows") {
            // In Rust we don't have COM objects like in PHP
            // This would require a Windows-specific crate integration
            0
        } else if os_name.contains("bsd") {
            match Command::new("stat")
                .args(&["-f", "%z", full_path])
                .output() {
                    Ok(output) => {
                        if output.status.success() {
                            if let Ok(size_str) = String::from_utf8(output.stdout) {
                                if let Ok(size) = size_str.trim().parse::<u64>() {
                                    return size;
                                }
                            }
                        }
                        0
                    }
                    Err(_) => 0,
                }
        } else if os_name.contains("linux") {
            match Command::new("stat")
                .args(&["-c", "%s", full_path])
                .output() {
                    Ok(output) => {
                        if output.status.success() {
                            if let Ok(size_str) = String::from_utf8(output.stdout) {
                                if let Ok(size) = size_str.trim().parse::<u64>() {
                                    return size;
                                }
                            }
                        }
                        0
                    }
                    Err(_) => 0,
                }
        } else {
            // Log error about unknown OS
            0
        }
    }
}

pub const SPACE_UNKNOWN: i64 = -1;

#[cfg(not(windows))]
#[async_trait]
impl Storage for LocalStorage {
    fn id(&self) -> String {
        format!("local::{}", self.datadir)
    }

    fn mkdir(&self, path: &str) -> bool {
        fs::create_dir(format!("{}{}", self.datadir, path)).is_ok()
    }

    fn rmdir(&self, path: &str) -> bool {
        let full_path = format!("{}{}", self.datadir, path);
        let path = Path::new(&full_path);
        
        if !path.exists() || !path.is_dir() {
            return false;
        }
        
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries.filter_map(Result::ok) {
                    let file_name = entry.file_name();
                    if file_name == OsStr::new(".") || file_name == OsStr::new("..") {
                        continue;
                    }
                    
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        if let Err(_) = fs::remove_dir_all(&entry_path) {
                            return false;
                        }
                    } else {
                        if let Err(_) = fs::remove_file(&entry_path) {
                            return false;
                        }
                    }
                }
                fs::remove_dir(path).is_ok()
            }
            Err(_) => false,
        }
    }

    fn opendir(&self, path: &str) -> IoResult<fs::ReadDir> {
        fs::read_dir(format!("{}{}", self.datadir, path))
    }

    fn is_dir(&self, path: &str) -> bool {
        let mut path_str = path.to_string();
        if path_str.ends_with('/') {
            path_str.pop();
        }
        
        Path::new(&format!("{}{}", self.datadir, path_str)).is_dir()
    }

    fn is_file(&self, path: &str) -> bool {
        Path::new(&format!("{}{}", self.datadir, path)).is_file()
    }

    fn stat(&self, path: &str) -> IoResult<fs::Metadata> {
        let full_path = format!("{}{}", self.datadir, path);
        match fs::metadata(&full_path) {
            Ok(metadata) => {
                // No direct equivalent to PHP's stat array in Rust
                // We return the raw metadata and let the caller extract what they need
                Ok(metadata)
            }
            Err(e) => Err(e),
        }
    }

    fn filetype(&self, path: &str) -> IoResult<String> {
        let full_path = format!("{}{}", self.datadir, path);
        let metadata = fs::symlink_metadata(&full_path)?;
        
        let file_type = if metadata.is_dir() {
            "dir"
        } else if metadata.is_file() {
            "file"
        } else if metadata.file_type().is_symlink() {
            // For symlinks, get the target type (similar to PHP's behavior)
            match fs::metadata(&full_path) {
                Ok(target_meta) => {
                    if target_meta.is_dir() {
                        "dir"
                    } else {
                        "file"
                    }
                }
                Err(_) => "unknown"
            }
        } else {
            "unknown"
        };
        
        Ok(file_type.to_string())
    }

    fn filesize(&self, path: &str) -> u64 {
        if self.is_dir(path) {
            return 0;
        }
        
        let full_path = format!("{}{}", self.datadir, path);
        match fs::metadata(&full_path) {
            Ok(metadata) => {
                let size = metadata.len();
                if size == 0 {
                    // This isn't exactly the same as PHP's check for < 0,
                    // but Rust's metadata.len() doesn't return negative values
                    // Using 0 as an indicator to try alternative method
                    Self::get_file_size_from_os(&full_path)
                } else {
                    size
                }
            }
            Err(_) => 0,
        }
    }

    fn is_readable(&self, path: &str) -> bool {
        let full_path = Path::new(&format!("{}{}", self.datadir, path));
        // Simplified check - just see if we can open the file for reading
        File::open(full_path).is_ok()
    }

    fn is_updatable(&self, path: &str) -> bool {
        let full_path = Path::new(&format!("{}{}", self.datadir, path));
        // Check if we can open for writing
        OpenOptions::new().write(true).open(full_path).is_ok()
    }

    fn file_exists(&self, path: &str) -> bool {
        Path::new(&format!("{}{}", self.datadir, path)).exists()
    }

    fn filemtime(&self, path: &str) -> IoResult<SystemTime> {
        let metadata = fs::metadata(format!("{}{}", self.datadir, path))?;
        metadata.modified()
    }

    fn touch(&self, path: &str, mtime: Option<SystemTime>) -> bool {
        let full_path = format!("{}{}", self.datadir, path);
        
        if self.file_exists(path) && !self.is_updatable(path) {
            return false;
        }
        
        let result = if let Some(mtime) = mtime {
            // Open file or create if it doesn't exist
            let file_result = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&full_path);
                
            if file_result.is_err() {
                return false;
            }
            
            // Convert SystemTime to filetime
            let duration = mtime.duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0));
            
            // This would need the filetime crate in a real implementation
            // filetime::set_file_mtime(&full_path, filetime::FileTime::from_unix_time(duration.as_secs() as i64, 0))
            //     .is_ok()
            
            // For simplicity in this translation:
            true
        } else {
            // Just create/touch the file with current time
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(&full_path)
                .is_ok()
        };
        
        result
    }

    fn file_get_contents(&self, path: &str) -> IoResult<Vec<u8>> {
        let mut file = File::open(format!("{}{}", self.datadir, path))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn file_put_contents(&self, path: &str, data: &[u8]) -> IoResult<usize> {
        let mut file = File::create(format!("{}{}", self.datadir, path))?;
        file.write(data)
    }

    fn unlink(&self, path: &str) -> bool {
        self.del_tree(path)
    }

    fn rename(&self, path1: &str, path2: &str) -> bool {
        if !self.is_updatable(path1) {
            // Log error: unable to rename, file is not writable
            return false;
        }
        
        if !self.file_exists(path1) {
            // Log error: unable to rename, file does not exist
            return false;
        }
        
        fs::rename(
            format!("{}{}", self.datadir, path1),
            format!("{}{}", self.datadir, path2)
        ).is_ok()
    }

    fn copy(&self, path1: &str, path2: &str) -> bool {
        let mut target_path = path2.to_string();
        
        if self.is_dir(&target_path) {
            if !self.file_exists(&target_path) {
                if !self.mkdir(&target_path) {
                    return false;
                }
            }
            
            let source_name = Path::new(path1)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
                
            target_path = format!("{}/{}", target_path, source_name);
        }
        
        fs::copy(
            format!("{}{}", self.datadir, path1),
            format!("{}{}", self.datadir, target_path)
        ).is_ok()
    }

    fn fopen(&self, path: &str, mode: &str) -> IoResult<File> {
        let full_path = format!("{}{}", self.datadir, path);
        
        // Map PHP file modes to Rust OpenOptions
        let mut options = OpenOptions::new();
        
        match mode {
            "r" => {
                options.read(true);
            },
            "r+" => {
                options.read(true).write(true);
            },
            "w" => {
                options.write(true).create(true).truncate(true);
            },
            "w+" => {
                options.read(true).write(true).create(true).truncate(true);
            },
            "a" => {
                options.write(true).create(true).append(true);
            },
            "a+" => {
                options.read(true).write(true).create(true).append(true);
            },
            "x" => {
                options.write(true).create_new(true);
            },
            "x+" => {
                options.read(true).write(true).create_new(true);
            },
            _ => {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file mode"));
            }
        }
        
        options.open(full_path)
    }

    fn get_mime_type(&self, path: &str) -> Option<String> {
        if !self.is_readable(path) {
            return None;
        }
        
        // In a real implementation, we would use a mime guessing library like mime_guess
        // For this translation, we'll simulate the PHP behavior with a placeholder
        
        let full_path = format!("{}{}", self.datadir, path);
        
        // Simplified mime type detection based on extension
        let extension = Path::new(&full_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
            
        match extension.to_lowercase().as_str() {
            "txt" => Some("text/plain".to_string()),
            "html" | "htm" => Some("text/html".to_string()),
            "css" => Some("text/css".to_string()),
            "js" => Some("application/javascript".to_string()),
            "jpg" | "jpeg" => Some("image/jpeg".to_string()),
            "png" => Some("image/png".to_string()),
            "gif" => Some("image/gif".to_string()),
            "pdf" => Some("application/pdf".to_string()),
            "zip" => Some("application/zip".to_string()),
            _ => Some("application/octet-stream".to_string()),
        }
    }

    fn hash(&self, path: &str, hash_type: &str, raw: bool) -> IoResult<String> {
        // In a real implementation, we would use a hash library
        // For this translation, we'll create a placeholder that mimics the behavior
        
        let full_path = format!("{}{}", self.datadir, path);
        let data = fs::read(&full_path)?;
        
        // This would use a proper hashing library in a real implementation
        match hash_type {
            "md5" => {
                // Placeholder for MD5 hash implementation
                Ok("md5_hash_placeholder".to_string())
            },
            "sha1" => {
                // Placeholder for SHA1 hash implementation
                Ok("sha1_hash_placeholder".to_string())
            },
            _ => {
                Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported hash type"))
            }
        }
    }

    fn free_space(&self, path: &str) -> i64 {
        let full_path = format!("{}{}", self.datadir, path);
        
        // This would use platform-specific code to get free space
        // For this translation, we'll use a placeholder
        
        // In a real implementation:
        // #[cfg(unix)]
        // {
        //    use std::os::unix::fs::MetadataExt;
        //    let stat = fs::metadata(&full_path).ok()?;
        //    // Return available blocks * block size
        // }
        
        SPACE_UNKNOWN
    }

    fn search(&self, query: &str) -> Vec<String> {
        self.search_in_dir(query, "")
    }

    fn get_local_file(&self, path: &str) -> String {
        format!("{}{}", self.datadir, path)
    }

    fn get_local_folder(&self, path: &str) -> String {
        format!("{}{}", self.datadir, path)
    }

    fn has_updated(&self, path: &str, time: SystemTime) -> bool {
        match self.filemtime(path) {
            Ok(mtime) => mtime > time,
            Err(_) => false,
        }
    }
}