// Copyright (c) 2013 Arthur Schiwon <blizzz@owncloud.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::user_ldap::lib::{ILDAPWrapper, Proxy};
use crate::user_ldap::USER_LDAP;

/// UserInterface trait defines the interface for user operations
pub trait UserInterface {
    fn implements_actions(&self, actions: u32) -> bool;
    fn get_users(&self, search: &str, limit: i32, offset: i32) -> Vec<String>;
    fn user_exists(&self, uid: &str) -> bool;
    fn check_password(&self, uid: &str, password: &str) -> Option<String>;
    fn get_home(&self, uid: &str) -> Option<String>;
    fn get_display_name(&self, uid: &str) -> Option<String>;
    fn get_display_names(&self, search: &str, limit: Option<i32>, offset: Option<i32>) -> HashMap<String, String>;
    fn delete_user(&self, uid: &str) -> bool;
    fn has_user_listings(&self) -> bool;
}

pub struct UserProxy {
    backends: HashMap<String, USER_LDAP>,
    ref_backend: Option<String>,
    ldap: Arc<dyn ILDAPWrapper>,
}

impl Proxy for UserProxy {
    fn new(ldap: Arc<dyn ILDAPWrapper>) -> Self {
        Self {
            backends: HashMap::new(),
            ref_backend: None,
            ldap,
        }
    }

    fn get_ldap(&self) -> Arc<dyn ILDAPWrapper> {
        self.ldap.clone()
    }
}

impl UserProxy {
    /// Constructor
    ///
    /// # Arguments
    ///
    /// * `server_config_prefixes` - Array containing the config prefixes
    /// * `ldap` - LDAP wrapper implementation
    pub fn new(server_config_prefixes: Vec<String>, ldap: Arc<dyn ILDAPWrapper>) -> Self {
        let mut proxy = Self {
            backends: HashMap::new(),
            ref_backend: None,
            ldap: ldap.clone(),
        };

        for config_prefix in server_config_prefixes {
            let backend = USER_LDAP::new(proxy.get_access(&config_prefix));
            
            if proxy.ref_backend.is_none() {
                proxy.ref_backend = Some(config_prefix.clone());
            }
            
            proxy.backends.insert(config_prefix, backend);
        }

        proxy
    }

    /// Tries the backends one after the other until a positive result is returned from the specified method
    ///
    /// # Arguments
    ///
    /// * `uid` - the uid connected to the request
    /// * `method_fn` - the method to call on the backend
    ///
    /// # Returns
    ///
    /// The result of the method or None
    fn walk_backends<F, R>(&mut self, uid: &str, method_fn: F) -> Option<R>
    where
        F: Fn(&USER_LDAP) -> Option<R>,
    {
        let cache_key = self.get_user_cache_key(uid);
        
        for (config_prefix, backend) in &self.backends {
            if let Some(result) = method_fn(backend) {
                self.write_to_cache(&cache_key, Some(config_prefix.clone()));
                return Some(result);
            }
        }
        
        None
    }

    /// Asks the backend connected to the server that supposedly takes care of the uid from the request.
    ///
    /// # Arguments
    ///
    /// * `uid` - the uid connected to the request
    /// * `method_fn` - the method to call on the backend
    ///
    /// # Returns
    ///
    /// The result of the method or None
    fn call_on_last_seen_on<F, R>(&mut self, uid: &str, method_fn: F) -> Option<R>
    where
        F: Fn(&USER_LDAP) -> Option<R>,
    {
        let cache_key = self.get_user_cache_key(uid);
        let prefix = self.get_from_cache(&cache_key);
        
        // In case the uid has been found in the past, try this stored connection first
        if let Some(prefix) = prefix {
            if let Some(backend) = self.backends.get(&prefix) {
                let result = method_fn(backend);
                
                if result.is_none() {
                    // Not found here, reset cache to null if user vanished
                    // because sometimes methods return None with a reason
                    let user_exists = backend.user_exists(uid);
                    
                    if !user_exists {
                        self.write_to_cache(&cache_key, None);
                    }
                }
                
                return result;
            }
        }
        
        None
    }

    /// Handle a request either via cached backend or by walking through all backends
    ///
    /// # Arguments
    ///
    /// * `uid` - the uid connected to the request
    /// * `method_fn` - the method to call on the backend
    ///
    /// # Returns
    ///
    /// The result of the method or None
    fn handle_request<F, R>(&mut self, uid: &str, method_fn: F) -> Option<R>
    where
        F: Fn(&USER_LDAP) -> Option<R> + Copy,
    {
        // Try use cached backend first
        if let Some(result) = self.call_on_last_seen_on(uid, method_fn) {
            return Some(result);
        }
        
        // If cached lookup fails, walk through all backends
        self.walk_backends(uid, method_fn)
    }
}

impl UserInterface for UserProxy {
    /// Check if backend implements actions
    ///
    /// # Arguments
    ///
    /// * `actions` - bitwise-or'ed actions
    ///
    /// # Returns
    ///
    /// The supported actions as int to be compared with OC_USER_BACKEND_CREATE_USER etc.
    fn implements_actions(&self, actions: u32) -> bool {
        // It's the same across all our user backends obviously
        if let Some(ref_key) = &self.ref_backend {
            if let Some(backend) = self.backends.get(ref_key) {
                return backend.implements_actions(actions);
            }
        }
        false
    }

    /// Get a list of all users
    ///
    /// # Arguments
    ///
    /// * `search` - Optional search term
    /// * `limit` - Optional limit of results
    /// * `offset` - Optional offset for pagination
    ///
    /// # Returns
    ///
    /// Array with all uids
    fn get_users(&self, search: &str, limit: i32, offset: i32) -> Vec<String> {
        // We do it just as the OC_User implementation: 
        // do not play around with limit and offset but ask all backends
        let mut users = Vec::new();
        
        for (_, backend) in &self.backends {
            let backend_users = backend.get_users(search, limit, offset);
            users.extend(backend_users);
        }
        
        users
    }

    /// Check if a user exists
    ///
    /// # Arguments
    ///
    /// * `uid` - the username
    ///
    /// # Returns
    ///
    /// true if user exists, false otherwise
    fn user_exists(&self, uid: &str) -> bool {
        let mut mutable_self = Self {
            backends: self.backends.clone(),
            ref_backend: self.ref_backend.clone(),
            ldap: self.ldap.clone(),
        };
        
        mutable_self.handle_request(uid, |backend| {
            if backend.user_exists(uid) {
                Some(true)
            } else {
                None
            }
        }).unwrap_or(false)
    }

    /// Check if the password is correct
    ///
    /// # Arguments
    ///
    /// * `uid` - The username
    /// * `password` - The password
    ///
    /// # Returns
    ///
    /// Some(uid) if password is correct, None otherwise
    fn check_password(&self, uid: &str, password: &str) -> Option<String> {
        let mut mutable_self = Self {
            backends: self.backends.clone(),
            ref_backend: self.ref_backend.clone(),
            ldap: self.ldap.clone(),
        };
        
        mutable_self.handle_request(uid, |backend| {
            backend.check_password(uid, password)
        })
    }

    /// Get the user's home directory
    ///
    /// # Arguments
    ///
    /// * `uid` - the username
    ///
    /// # Returns
    ///
    /// The home directory or None
    fn get_home(&self, uid: &str) -> Option<String> {
        let mut mutable_self = Self {
            backends: self.backends.clone(),
            ref_backend: self.ref_backend.clone(),
            ldap: self.ldap.clone(),
        };
        
        mutable_self.handle_request(uid, |backend| {
            backend.get_home(uid)
        })
    }

    /// Get display name of the user
    ///
    /// # Arguments
    ///
    /// * `uid` - user ID of the user
    ///
    /// # Returns
    ///
    /// display name or None
    fn get_display_name(&self, uid: &str) -> Option<String> {
        let mut mutable_self = Self {
            backends: self.backends.clone(),
            ref_backend: self.ref_backend.clone(),
            ldap: self.ldap.clone(),
        };
        
        mutable_self.handle_request(uid, |backend| {
            backend.get_display_name(uid)
        })
    }

    /// Get a list of all display names
    ///
    /// # Arguments
    ///
    /// * `search` - Optional search term
    /// * `limit` - Optional limit of results
    /// * `offset` - Optional offset for pagination
    ///
    /// # Returns
    ///
    /// HashMap with all displayNames (value) and the corresponding uids (key)
    fn get_display_names(&self, search: &str, limit: Option<i32>, offset: Option<i32>) -> HashMap<String, String> {
        // We do it just as the OC_User implementation: 
        // do not play around with limit and offset but ask all backends
        let mut users = HashMap::new();
        
        for (_, backend) in &self.backends {
            let backend_users = backend.get_display_names(search, limit, offset);
            users.extend(backend_users);
        }
        
        users
    }

    /// Delete a user
    ///
    /// # Arguments
    ///
    /// * `uid` - The username of the user to delete
    ///
    /// # Returns
    ///
    /// true on success, false otherwise
    fn delete_user(&self, _uid: &str) -> bool {
        // Original implementation just returns false
        false
    }

    /// Check if the backend provides user listings
    ///
    /// # Returns
    ///
    /// true if user listings are supported, false otherwise
    fn has_user_listings(&self) -> bool {
        if let Some(ref_key) = &self.ref_backend {
            if let Some(backend) = self.backends.get(ref_key) {
                return backend.has_user_listings();
            }
        }
        false
    }
}