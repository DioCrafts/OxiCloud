// Módulos generados automáticamente

pub mod broker;
pub mod fileglobalgc;
pub mod fileglobal;
pub mod usercache;
pub mod file;

// Contenido fusionado desde src/lib/private/cache.rs
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;
use std::path::Path;
use std::fs;
use crypto::md5::Md5;
use crypto::digest::Digest;

mod cache {
    pub mod file;
    pub mod file_global;
}

use crate::cache::file::File;
use crate::cache::file_global::FileGlobal;

/// Cache implementation for OC
pub struct Cache;

/// A trait defining the cache interface
pub trait CacheInterface {
    fn get<T: Clone>(&self, key: &str) -> Option<T>;
    fn set<T: Clone>(&self, key: &str, value: T, ttl: u64) -> bool;
    fn has_key(&self, key: &str) -> bool;
    fn remove(&self, key: &str) -> bool;
    fn clear(&self, prefix: &str) -> bool;
}

static USER_CACHE_INIT: Once = Once::new();
static GLOBAL_CACHE_INIT: Once = Once::new();

static mut USER_CACHE: Option<Arc<Mutex<File>>> = None;
static mut GLOBAL_CACHE: Option<Arc<Mutex<FileGlobal>>> = None;

impl Cache {
    /// Get the global cache
    ///
    /// Returns a reference to the global cache instance
    pub fn get_global_cache() -> Arc<Mutex<FileGlobal>> {
        unsafe {
            GLOBAL_CACHE_INIT.call_once(|| {
                GLOBAL_CACHE = Some(Arc::new(Mutex::new(FileGlobal::new())));
            });
            GLOBAL_CACHE.clone().unwrap()
        }
    }

    /// Get the user cache
    ///
    /// Returns a reference to the user cache instance
    pub fn get_user_cache() -> Arc<Mutex<File>> {
        unsafe {
            USER_CACHE_INIT.call_once(|| {
                USER_CACHE = Some(Arc::new(Mutex::new(File::new())));
            });
            USER_CACHE.clone().unwrap()
        }
    }

    /// Get a value from the user cache
    ///
    /// # Arguments
    ///
    /// * `key` - The key to get the value for
    ///
    /// # Returns
    ///
    /// The value if found, None otherwise
    pub fn get<T: Clone>(key: &str) -> Option<T> {
        let user_cache = Self::get_user_cache();
        let cache = user_cache.lock().unwrap();
        cache.get(key)
    }

    /// Set a value in the user cache
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set
    /// * `value` - The value to store
    /// * `ttl` - Time to live in seconds (0 for no expiration)
    ///
    /// # Returns
    ///
    /// true if successful, false otherwise
    pub fn set<T: Clone>(key: &str, value: T, ttl: u64) -> bool {
        if key.is_empty() {
            return false;
        }
        
        let user_cache = Self::get_user_cache();
        let mut cache = user_cache.lock().unwrap();
        cache.set(key, value, ttl)
    }

    /// Check if a value is set in the user cache
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check
    ///
    /// # Returns
    ///
    /// true if the key exists, false otherwise
    pub fn has_key(key: &str) -> bool {
        let user_cache = Self::get_user_cache();
        let cache = user_cache.lock().unwrap();
        cache.has_key(key)
    }

    /// Remove an item from the user cache
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove
    ///
    /// # Returns
    ///
    /// true if successful, false otherwise
    pub fn remove(key: &str) -> bool {
        let user_cache = Self::get_user_cache();
        let mut cache = user_cache.lock().unwrap();
        cache.remove(key)
    }

    /// Clear the user cache of all entries starting with a prefix
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix to match (optional)
    ///
    /// # Returns
    ///
    /// true if successful, false otherwise
    pub fn clear(prefix: &str) -> bool {
        let user_cache = Self::get_user_cache();
        let mut cache = user_cache.lock().unwrap();
        cache.clear(prefix)
    }

    /// Creates cache key based on the files given
    ///
    /// # Arguments
    ///
    /// * `files` - List of file paths to generate the key from
    ///
    /// # Returns
    ///
    /// MD5 hash string of the concatenated file metadata
    pub fn generate_cache_key_from_files(files: &[String]) -> Result<String, std::io::Error> {
        let mut sorted_files = files.to_vec();
        sorted_files.sort();
        
        let mut key = String::new();
        for file in sorted_files {
            let path = Path::new(&file);
            let metadata = fs::metadata(path)?;
            key.push_str(&file);
            key.push_str(&metadata.modified()?.duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs()
                .to_string());
            key.push_str(&metadata.len().to_string());
        }
        
        let mut hasher = Md5::new();
        hasher.input_str(&key);
        Ok(hasher.result_str())
    }
}