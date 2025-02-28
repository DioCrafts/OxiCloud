// ownCloud – LDAP Backend Proxy
//
// @author Arthur Schiwon
// @copyright 2013 Arthur Schiwon blizzz@owncloud.com
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use md5::{Md5, Digest};
use base64::{Engine as _, engine::general_purpose};
use serde::{Serialize, Deserialize};

use crate::connection::Connection;
use crate::access::Access;
use crate::cache::Cache;

pub trait ILDAPWrapper: Send + Sync {}

static ACCESSES: Lazy<Mutex<HashMap<String, Arc<Access>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub trait ProxyTrait {
    fn call_on_last_seen_on(&self, id: &str, method: &str, parameters: &[&dyn std::any::Any]) -> Option<Box<dyn std::any::Any>>;
    fn walk_backends(&self, id: &str, method: &str, parameters: &[&dyn std::any::Any]) -> Option<Box<dyn std::any::Any>>;
}

pub struct Proxy {
    ldap: Arc<dyn ILDAPWrapper>,
    cache: Arc<dyn Cache>,
}

impl Proxy {
    pub fn new(ldap: Arc<dyn ILDAPWrapper>, cache: Arc<dyn Cache>) -> Self {
        Proxy {
            ldap,
            cache,
        }
    }

    fn add_access(&self, config_prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
        let connector = Connection::new(self.ldap.clone(), config_prefix.to_string());
        let access = Arc::new(Access::new(connector, self.ldap.clone()));
        
        let mut accesses = ACCESSES.lock().unwrap();
        accesses.insert(config_prefix.to_string(), access);
        
        Ok(())
    }

    fn get_access(&self, config_prefix: &str) -> Result<Arc<Access>, Box<dyn std::error::Error>> {
        let mut accesses = ACCESSES.lock().unwrap();
        
        if !accesses.contains_key(config_prefix) {
            drop(accesses);
            self.add_access(config_prefix)?;
            accesses = ACCESSES.lock().unwrap();
        }
        
        Ok(accesses.get(config_prefix).unwrap().clone())
    }

    fn get_user_cache_key(&self, uid: &str) -> String {
        format!("user-{}-lastSeenOn", uid)
    }

    fn get_group_cache_key(&self, gid: &str) -> String {
        format!("group-{}-lastSeenOn", gid)
    }

    /// Takes care of the request to the User backend
    ///
    /// # Arguments
    ///
    /// * `id` - The uid connected to the request
    /// * `method` - The method of the user backend that shall be called
    /// * `parameters` - An array of parameters to be passed
    ///
    /// # Returns
    ///
    /// The result of the specified method
    fn handle_request(&self, id: &str, method: &str, parameters: &[&dyn std::any::Any]) -> Option<Box<dyn std::any::Any>> {
        if let Some(result) = self.call_on_last_seen_on(id, method, parameters) {
            return Some(result);
        }
        
        self.walk_backends(id, method, parameters)
    }

    fn get_cache_key(&self, key: Option<&str>) -> String {
        let prefix = "LDAP-Proxy-";
        
        match key {
            None => prefix.to_string(),
            Some(k) => {
                let mut hasher = Md5::new();
                hasher.update(k.as_bytes());
                let result = hasher.finalize();
                format!("{}{:x}", prefix, result)
            }
        }
    }

    pub fn get_from_cache<T: 'static + for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        if !self.is_cached(key) {
            return None;
        }
        
        let cache_key = self.get_cache_key(Some(key));
        let cached_value = self.cache.get(&cache_key)?;
        
        let decoded = general_purpose::STANDARD.decode(cached_value).ok()?;
        bincode::deserialize(&decoded).ok()
    }

    pub fn is_cached(&self, key: &str) -> bool {
        let cache_key = self.get_cache_key(Some(key));
        self.cache.has_key(&cache_key)
    }

    pub fn write_to_cache<T: Serialize>(&self, key: &str, value: &T) -> Result<(), Box<dyn std::error::Error>> {
        let cache_key = self.get_cache_key(Some(key));
        let serialized = bincode::serialize(value)?;
        let encoded = general_purpose::STANDARD.encode(serialized);
        
        self.cache.set(&cache_key, &encoded, 2592000)?;
        Ok(())
    }

    pub fn clear_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cache.clear(&self.get_cache_key(None))?;
        Ok(())
    }
}

impl ProxyTrait for Proxy {
    fn call_on_last_seen_on(&self, id: &str, method: &str, parameters: &[&dyn std::any::Any]) -> Option<Box<dyn std::any::Any>> {
        // Implementation to be provided by the child class
        None
    }

    fn walk_backends(&self, id: &str, method: &str, parameters: &[&dyn std::any::Any]) -> Option<Box<dyn std::any::Any>> {
        // Implementation to be provided by the child class
        None
    }
}

pub trait Cache: Send + Sync {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, value: &str, ttl: u64) -> Result<(), Box<dyn std::error::Error>>;
    fn has_key(&self, key: &str) -> bool;
    fn clear(&self, prefix: &str) -> Result<(), Box<dyn std::error::Error>>;
}