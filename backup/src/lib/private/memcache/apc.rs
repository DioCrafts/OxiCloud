// Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::sync::Arc;
use async_trait::async_trait;
use apcu::{ApcuCache, ApcuEntry};

/// APC implementation of the cache interface
pub struct Apc {
    prefix: String,
    cache: Arc<ApcuCache>,
}

impl Apc {
    /// Create a new APC cache
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            cache: Arc::new(ApcuCache::new()),
        }
    }

    /// entries in APC gets namespaced to prevent collisions between owncloud instances and users
    fn get_namespace(&self) -> &str {
        &self.prefix
    }
}

#[async_trait]
impl super::Cache for Apc {
    async fn get<T: serde::de::DeserializeOwned + Send + Sync>(&self, key: &str) -> Option<T> {
        let namespaced_key = format!("{}{}", self.get_namespace(), key);
        match self.cache.get::<T>(&namespaced_key) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    async fn set<T: serde::Serialize + Send + Sync>(&self, key: &str, value: T, ttl: u64) -> bool {
        let namespaced_key = format!("{}{}", self.get_namespace(), key);
        self.cache.set(&namespaced_key, &value, ttl as i32).is_ok()
    }

    async fn has_key(&self, key: &str) -> bool {
        let namespaced_key = format!("{}{}", self.get_namespace(), key);
        self.cache.exists(&namespaced_key).unwrap_or(false)
    }

    async fn remove(&self, key: &str) -> bool {
        let namespaced_key = format!("{}{}", self.get_namespace(), key);
        self.cache.delete(&namespaced_key).is_ok()
    }

    async fn clear(&self, prefix: &str) -> bool {
        let ns = format!("{}{}", self.get_namespace(), prefix);
        
        // Get all cache entries
        if let Ok(entries) = self.cache.cache_info() {
            for entry in entries {
                if let ApcuEntry { info, .. } = entry {
                    if info.starts_with(&ns) {
                        let _ = self.cache.delete(&info);
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

impl Apc {
    /// Check if APC is available
    pub fn is_available() -> bool {
        if cfg!(not(feature = "apcu")) {
            return false;
        }
        
        // Check if APC is enabled in CLI mode
        if std::env::var("CLI").unwrap_or_default() == "1" {
            match std::env::var("APC_ENABLE_CLI") {
                Ok(val) => val == "1" || val.to_lowercase() == "on" || val.to_lowercase() == "true",
                Err(_) => false,
            }
        } else {
            true
        }
    }
}