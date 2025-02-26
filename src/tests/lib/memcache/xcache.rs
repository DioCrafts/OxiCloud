/**
 * Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::memcache::cache::Cache;
use crate::memcache::xcache::XCache as MemcacheXCache;
use uuid::Uuid;

pub struct XCache {
    instance: Option<MemcacheXCache>,
}

impl Default for XCache {
    fn default() -> Self {
        Self { instance: None }
    }
}

impl Cache for XCache {
    fn set_up(&mut self) -> Result<(), String> {
        if !MemcacheXCache::is_available() {
            return Err("The xcache extension is not available.".to_string());
        }
        
        let unique_id = Uuid::new_v4().to_string();
        self.instance = Some(MemcacheXCache::new(&unique_id));
        Ok(())
    }
    
    fn get_instance(&self) -> Option<&MemcacheXCache> {
        self.instance.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_xcache() {
        let mut cache = XCache::default();
        if let Err(e) = cache.set_up() {
            println!("Test skipped: {}", e);
            return;
        }
        
        // Test implementation goes here
    }
}