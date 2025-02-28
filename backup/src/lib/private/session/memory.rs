// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;

/// Trait defining session operations
pub trait Session {
    fn set(&mut self, key: &str, value: String);
    fn get(&self, key: &str) -> Option<String>;
    fn exists(&self, key: &str) -> bool;
    fn remove(&mut self, key: &str);
    fn clear(&mut self);
}

/// Store session data in an in-memory HashMap, not persistence
pub struct Memory {
    data: HashMap<String, String>,
}

impl Memory {
    pub fn new(_name: &str) -> Self {
        // No need to use name since all data is already scoped to this instance
        Self {
            data: HashMap::new(),
        }
    }
}

impl Session for Memory {
    /// Set a value in the session
    fn set(&mut self, key: &str, value: String) {
        self.data.insert(key.to_string(), value);
    }

    /// Get a value from the session
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    /// Check if a key exists in the session
    fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Remove a key from the session
    fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    /// Clear the session
    fn clear(&mut self) {
        self.data.clear();
    }
}