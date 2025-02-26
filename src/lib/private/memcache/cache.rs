//! Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use std::ops::{Index, IndexMut};

/// Generic cache trait that can be used as a key-value store
pub trait Cache: Index<String, Output = Option<Vec<u8>>> + IndexMut<String> {
    /// Get the prefix used for all keys in this cache
    fn get_prefix(&self) -> &str;

    /// Get a value from the cache
    fn get(&self, key: &str) -> Option<Vec<u8>>;

    /// Set a value in the cache
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set
    /// * `value` - The value to set
    /// * `ttl` - Time to live in seconds, 0 for unlimited
    fn set(&mut self, key: &str, value: Vec<u8>, ttl: u64) -> bool;

    /// Check if a key exists in the cache
    fn has_key(&self, key: &str) -> bool;

    /// Remove a value from the cache
    fn remove(&mut self, key: &str) -> bool;

    /// Clear the cache
    ///
    /// # Arguments
    /// * `prefix` - Optional prefix to clear only a subset of the cache
    fn clear(&mut self, prefix: &str) -> bool;
}

/// Base implementation for cache backends
pub struct CacheBase {
    prefix: String,
}

impl CacheBase {
    /// Create a new cache with the given prefix
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix to use for all keys
    pub fn new(prefix: &str) -> Self {
        // In a real implementation, we would fetch the instance ID from somewhere
        let instance_id = get_instance_id();
        
        Self {
            prefix: format!("{}/{}", instance_id, prefix),
        }
    }

    /// Get the prefix used for all keys
    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }
}

// This would be implemented in another module
fn get_instance_id() -> String {
    // This is a placeholder for OC_Util::getInstanceId()
    "instance_id".to_string()
}

// Example implementation for a concrete cache
pub struct MemoryCache {
    base: CacheBase,
    // In a real implementation, we would have a storage backend here
}

impl MemoryCache {
    pub fn new(prefix: &str) -> Self {
        Self {
            base: CacheBase::new(prefix),
        }
    }
}

impl Cache for MemoryCache {
    fn get_prefix(&self) -> &str {
        self.base.get_prefix()
    }

    fn get(&self, key: &str) -> Option<Vec<u8>> {
        // Implementation would go here
        None
    }

    fn set(&mut self, key: &str, value: Vec<u8>, ttl: u64) -> bool {
        // Implementation would go here
        true
    }

    fn has_key(&self, key: &str) -> bool {
        // Implementation would go here
        false
    }

    fn remove(&mut self, key: &str) -> bool {
        // Implementation would go here
        true
    }

    fn clear(&mut self, prefix: &str) -> bool {
        // Implementation would go here
        true
    }
}

impl Index<String> for MemoryCache {
    type Output = Option<Vec<u8>>;

    fn index(&self, key: String) -> &Self::Output {
        // This would return a reference to the cached value
        // In a real implementation, this would access the storage
        &None
    }
}

impl IndexMut<String> for MemoryCache {
    fn index_mut(&mut self, key: String) -> &mut Self::Output {
        // This would return a mutable reference to the cached value
        // In a real implementation, this would access the storage
        // Note: Rust's ownership model makes this challenging to implement properly
        // A more idiomatic approach would be to avoid IndexMut altogether
        unimplemented!("IndexMut is not typically implemented for caches in Rust")
    }
}