//! Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use std::sync::Arc;
use async_trait::async_trait;

/// Interface for cache implementations
#[async_trait]
pub trait Cache: Send + Sync {
    /// Get a value from the cache
    async fn get(&self, key: &str) -> Option<String>;
    
    /// Set a value in the cache
    async fn set(&self, key: &str, value: &str, ttl: u64) -> bool;
    
    /// Check if a key exists in the cache
    async fn has_key(&self, key: &str) -> bool;
    
    /// Remove a key from the cache
    async fn remove(&self, key: &str) -> bool;
    
    /// Clear all keys or with a specific prefix
    async fn clear(&self, prefix: &str);
}

/// Broker for handling multiple cache backends
pub struct Broker {
    /// Fast cache implementation (e.g. memory)
    fast_cache: Arc<dyn Cache>,
    
    /// Slow cache implementation (e.g. file-based)
    slow_cache: Arc<dyn Cache>,
}

impl Broker {
    /// Create a new cache broker with fast and slow backends
    pub fn new(fast_cache: Arc<dyn Cache>, slow_cache: Arc<dyn Cache>) -> Self {
        Self {
            fast_cache,
            slow_cache,
        }
    }
    
    /// Get a value from the cache, trying fast cache first
    pub async fn get(&self, key: &str) -> Option<String> {
        if let Some(value) = self.fast_cache.get(key).await {
            return Some(value);
        }
        self.slow_cache.get(key).await
    }
    
    /// Set a value in the cache
    pub async fn set(&self, key: &str, value: &str, ttl: u64) -> bool {
        if !self.fast_cache.set(key, value, ttl).await {
            if self.fast_cache.has_key(key).await {
                self.fast_cache.remove(key).await;
            }
            return self.slow_cache.set(key, value, ttl).await;
        }
        true
    }
    
    /// Check if a key exists in any cache
    pub async fn has_key(&self, key: &str) -> bool {
        if self.fast_cache.has_key(key).await {
            return true;
        }
        self.slow_cache.has_key(key).await
    }
    
    /// Remove a key from all caches
    pub async fn remove(&self, key: &str) -> bool {
        if self.fast_cache.remove(key).await {
            return true;
        }
        self.slow_cache.remove(key).await
    }
    
    /// Clear all caches with an optional prefix
    pub async fn clear(&self, prefix: &str) {
        self.fast_cache.clear(prefix).await;
        self.slow_cache.clear(prefix).await;
    }
}