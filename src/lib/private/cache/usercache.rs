// Copyright (c) 2013 Thomas Tanghus (thomas@tanghus.net)
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::cache::file::File;

/// This interface defines method for accessing the file based user cache.
pub struct UserCache {
    /// The underlying file cache implementation
    user_cache: File,
}

/// Implementation of ICache trait for UserCache
impl crate::public::cache::ICache for UserCache {
    /// Get a value from the user cache
    ///
    /// # Arguments
    ///
    /// * `key` - The key to retrieve
    ///
    /// # Returns
    ///
    /// The value associated with the key, or None if not found
    fn get(&self, key: &str) -> Option<String> {
        self.user_cache.get(key)
    }

    /// Set a value in the user cache
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set
    /// * `value` - The value to store
    /// * `ttl` - Time To Live in seconds. Defaults to 0 (no expiration)
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    fn set(&self, key: &str, value: &str, ttl: u64) -> bool {
        if key.is_empty() {
            return false;
        }
        self.user_cache.set(key, value, ttl)
    }

    /// Check if a value is set in the user cache
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check
    ///
    /// # Returns
    ///
    /// true if the key exists in the cache, false otherwise
    fn has_key(&self, key: &str) -> bool {
        self.user_cache.has_key(key)
    }

    /// Remove an item from the user cache
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove
    ///
    /// # Returns
    ///
    /// true if the key was removed, false otherwise
    fn remove(&self, key: &str) -> bool {
        self.user_cache.remove(key)
    }

    /// Clear the user cache of all entries starting with a prefix
    ///
    /// # Arguments
    ///
    /// * `prefix` - Optional prefix to filter entries to be cleared
    ///
    /// # Returns
    ///
    /// true if the operation was successful, false otherwise
    fn clear(&self, prefix: &str) -> bool {
        self.user_cache.clear(prefix)
    }
}

impl UserCache {
    /// Creates a new UserCache instance
    pub fn new() -> Self {
        UserCache {
            user_cache: File::new(),
        }
    }
}

impl Default for UserCache {
    fn default() -> Self {
        Self::new()
    }
}