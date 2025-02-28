// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::files::cache::Cache;
use crate::files::cache::Scanner;
use crate::files::storage::Storage;
use std::sync::Arc;

/// Check the storage backends for updates and change the cache accordingly
pub struct Watcher {
    /// The storage backend
    storage: Arc<dyn Storage>,
    
    /// The cache for the storage
    cache: Arc<dyn Cache>,
    
    /// The scanner for the storage
    scanner: Arc<dyn Scanner>,
}

impl Watcher {
    /// Create a new watcher for the given storage
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        let cache = storage.get_cache();
        let scanner = storage.get_scanner();
        
        Self {
            storage,
            cache,
            scanner,
        }
    }

    /// Check $path for updates
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check for updates
    ///
    /// # Returns
    ///
    /// `true` if path was updated, `false` otherwise
    pub fn check_update(&self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let cached_entry = self.cache.get(path)?;
        
        if self.storage.has_updated(path, cached_entry.storage_mtime)? {
            if self.storage.is_dir(path)? {
                self.scanner.scan(path, Scanner::SCAN_SHALLOW)?;
            } else {
                self.scanner.scan_file(path)?;
            }
            
            if cached_entry.mimetype == "httpd/unix-directory" {
                self.clean_folder(path)?;
            }
            
            self.cache.correct_folder_size(path)?;
            
            return Ok(true);
        }
        
        Ok(false)
    }

    /// Remove deleted files in $path from the cache
    ///
    /// # Arguments
    ///
    /// * `path` - The folder path to clean
    pub fn clean_folder(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let cached_content = self.cache.get_folder_contents(path)?;
        
        for entry in cached_content {
            if !self.storage.file_exists(&entry.path)? {
                self.cache.remove(&entry.path)?;
            }
        }
        
        Ok(())
    }
}