/*
 * Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;

/// Directory stream implementation for virtual directories
pub struct Dir {
    name: String,
    index: usize,
}

// Static directory storage shared across all instances
static DIRS: Lazy<RwLock<HashMap<String, Vec<String>>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

impl Dir {
    /// Opens a directory for reading
    pub fn dir_opendir(&mut self, path: &str) -> bool {
        self.name = path.strip_prefix("fakedir://").unwrap_or(path).to_string();
        self.index = 0;
        
        // Ensure the directory exists in our static storage
        let mut dirs = DIRS.write().unwrap();
        if !dirs.contains_key(&self.name) {
            dirs.insert(self.name.clone(), Vec::new());
        }
        
        true
    }

    /// Reads the next entry from the directory
    pub fn dir_readdir(&mut self) -> Option<String> {
        let dirs = DIRS.read().unwrap();
        
        if let Some(entries) = dirs.get(&self.name) {
            if self.index >= entries.len() {
                return None;
            }
            
            let filename = entries[self.index].clone();
            self.index += 1;
            Some(filename)
        } else {
            None
        }
    }

    /// Closes the directory
    pub fn dir_closedir(&mut self) -> bool {
        self.name = String::new();
        true
    }

    /// Resets the position to the beginning of the directory
    pub fn dir_rewinddir(&mut self) -> bool {
        self.index = 0;
        true
    }

    /// Registers content for a virtual directory path
    pub fn register(path: &str, content: Vec<String>) {
        let mut dirs = DIRS.write().unwrap();
        dirs.insert(path.to_string(), content);
    }
}

impl Default for Dir {
    fn default() -> Self {
        Self {
            name: String::new(),
            index: 0,
        }
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        // Ensure resources are cleaned up properly
        self.dir_closedir();
    }
}