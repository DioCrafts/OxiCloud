// Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::memcache::Cache;
use lazy_static::lazy_static;
use memcached_rs::{Client, MemcachedError};
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref CACHE: Arc<Mutex<Option<Client>>> = Arc::new(Mutex::new(None));
}

pub struct Memcached {
    prefix: String,
}

impl Memcached {
    pub fn new(prefix: &str) -> Result<Self, MemcachedError> {
        let instance = Self {
            prefix: prefix.to_string(),
        };

        let mut cache_guard = CACHE.lock().unwrap();
        if cache_guard.is_none() {
            let (host, port) = crate::config::get_value::<(String, u16)>(
                "memcached_server",
                ("localhost".to_string(), 11211),
            );
            let client = Client::connect(&[(host, port)])?;
            *cache_guard = Some(client);
        }

        Ok(instance)
    }

    /// Entries in Memcached get namespaced to prevent collisions between owncloud instances and users
    fn get_namespace(&self) -> &str {
        &self.prefix
    }

    pub fn is_available() -> bool {
        // In Rust we'll assume memcached client library availability
        // rather than checking for PHP extension
        true
    }
}

impl Cache for Memcached {
    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        let namespace_key = format!("{}{}", self.get_namespace(), key);
        
        let cache_guard = CACHE.lock().unwrap();
        if let Some(client) = &*cache_guard {
            match client.get::<T>(&namespace_key) {
                Ok(value) => Some(value),
                Err(MemcachedError::KeyNotFound) => None,
                Err(_) => None, // Other errors also return None
            }
        } else {
            None
        }
    }

    fn set<T: serde::Serialize>(&self, key: &str, value: T, ttl: u32) -> bool {
        let namespace_key = format!("{}{}", self.get_namespace(), key);
        
        let cache_guard = CACHE.lock().unwrap();
        if let Some(client) = &*cache_guard {
            if ttl > 0 {
                client.set(&namespace_key, &value, ttl).is_ok()
            } else {
                client.set(&namespace_key, &value, 0).is_ok()
            }
        } else {
            false
        }
    }

    fn has_key(&self, key: &str) -> bool {
        let namespace_key = format!("{}{}", self.get_namespace(), key);
        
        let cache_guard = CACHE.lock().unwrap();
        if let Some(client) = &*cache_guard {
            !matches!(client.get::<()>(&namespace_key), Err(MemcachedError::KeyNotFound))
        } else {
            false
        }
    }

    fn remove(&self, key: &str) -> bool {
        let namespace_key = format!("{}{}", self.get_namespace(), key);
        
        let cache_guard = CACHE.lock().unwrap();
        if let Some(client) = &*cache_guard {
            client.delete(&namespace_key).is_ok()
        } else {
            false
        }
    }

    fn clear(&self, prefix: &str) -> bool {
        let namespace_prefix = format!("{}{}", self.get_namespace(), prefix);
        
        let cache_guard = CACHE.lock().unwrap();
        if let Some(client) = &*cache_guard {
            if let Ok(all_keys) = client.get_keys() {
                let keys_to_delete: Vec<String> = all_keys
                    .into_iter()
                    .filter(|key| key.starts_with(&namespace_prefix))
                    .collect();
                
                if !keys_to_delete.is_empty() {
                    for key in keys_to_delete {
                        let _ = client.delete(&key);
                    }
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}