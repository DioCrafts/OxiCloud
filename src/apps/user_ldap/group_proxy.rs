// Copyright (c) 2013 Arthur Schiwon blizzz@owncloud.com
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
use std::sync::Arc;

use crate::lib::{ILDAPWrapper, Proxy};
use crate::GROUP_LDAP;

/// Trait defining group interface operations
pub trait GroupInterface {
    fn in_group(&self, uid: &str, gid: &str) -> bool;
    fn get_user_groups(&self, uid: &str) -> Vec<String>;
    fn users_in_group(&self, gid: &str, search: &str, limit: i32, offset: i32) -> Vec<String>;
    fn display_names_in_group(&self, gid: &str, search: &str, limit: i32, offset: i32) -> HashMap<String, String>;
    fn get_groups(&self, search: &str, limit: i32, offset: i32) -> Vec<String>;
    fn group_exists(&self, gid: &str) -> bool;
    fn implements_actions(&self, actions: i32) -> bool;
}

pub struct GroupProxy {
    backends: HashMap<String, GROUP_LDAP>,
    ref_backend: Option<String>,
    proxy: Proxy,
}

impl GroupProxy {
    /// Constructor
    /// 
    /// # Arguments
    /// * `server_config_prefixes` - Array containing the config prefixes
    /// * `ldap` - LDAP wrapper implementation
    pub fn new(server_config_prefixes: Vec<String>, ldap: Arc<dyn ILDAPWrapper>) -> Self {
        let mut proxy = Proxy::new(ldap);
        let mut backends = HashMap::new();
        let mut ref_backend = None;

        for config_prefix in server_config_prefixes {
            let access = proxy.get_access(&config_prefix);
            let backend = GROUP_LDAP::new(access);
            
            if ref_backend.is_none() {
                ref_backend = Some(config_prefix.clone());
            }
            
            backends.insert(config_prefix, backend);
        }

        GroupProxy {
            backends,
            ref_backend,
            proxy,
        }
    }

    /// Tries the backends one after the other until a positive result is returned from the specified method
    /// 
    /// # Arguments
    /// * `gid` - The gid connected to the request
    /// * `method` - Function to call on backends
    /// 
    /// # Returns
    /// * `Option<T>` - The result of the method or None
    fn walk_backends<F, T>(&self, gid: &str, method: F) -> Option<T>
    where
        F: Fn(&GROUP_LDAP) -> Option<T>,
    {
        let cache_key = self.get_group_cache_key(gid);
        
        for (config_prefix, backend) in &self.backends {
            if let Some(result) = method(backend) {
                self.proxy.write_to_cache(&cache_key, Some(config_prefix.clone()));
                return Some(result);
            }
        }
        
        None
    }

    /// Asks the backend connected to the server that supposedly takes care of the gid from the request
    /// 
    /// # Arguments
    /// * `gid` - The gid connected to the request
    /// * `method` - Function to call on backend
    /// 
    /// # Returns
    /// * `Option<T>` - The result of the method or None
    fn call_on_last_seen_on<F, T>(&self, gid: &str, method: F, group_exists_check: bool) -> Option<T>
    where
        F: Fn(&GROUP_LDAP) -> Option<T>,
    {
        let cache_key = self.get_group_cache_key(gid);
        let prefix = self.proxy.get_from_cache(&cache_key);
        
        // In case the gid has been found in the past, try this stored connection first
        if let Some(prefix) = prefix {
            if let Some(backend) = self.backends.get(&prefix) {
                let result = method(backend);
                
                if result.is_none() && group_exists_check {
                    // Not found here, reset cache to null if group vanished
                    // because sometimes methods return None with a reason
                    let group_exists = backend.group_exists(gid);
                    if !group_exists {
                        self.proxy.write_to_cache(&cache_key, None);
                    }
                }
                
                return result;
            }
        }
        
        None
    }

    /// Handles a request by trying the cached connection first, then walking all backends
    /// 
    /// # Arguments
    /// * `gid` - The gid connected to the request
    /// * `method` - Function to call on backends
    /// * `group_exists_check` - Whether to check if the group exists when result is None
    /// 
    /// # Returns
    /// * `Option<T>` - The result of the method or None
    fn handle_request<F, T>(&self, gid: &str, method: F, group_exists_check: bool) -> Option<T>
    where
        F: Fn(&GROUP_LDAP) -> Option<T> + Copy,
    {
        let result = self.call_on_last_seen_on(gid, method, group_exists_check);
        
        if result.is_some() {
            return result;
        }
        
        self.walk_backends(gid, method)
    }

    /// Get the cache key for a group
    /// 
    /// # Arguments
    /// * `gid` - The group ID
    /// 
    /// # Returns
    /// * `String` - The cache key
    fn get_group_cache_key(&self, gid: &str) -> String {
        format!("group_{}", gid)
    }
}

impl GroupInterface for GroupProxy {
    /// Is user in group?
    /// 
    /// # Arguments
    /// * `uid` - UID of the user
    /// * `gid` - GID of the group
    /// 
    /// # Returns
    /// * `bool` - True if user is in group, false otherwise
    fn in_group(&self, uid: &str, gid: &str) -> bool {
        self.handle_request(
            gid,
            |backend| Some(backend.in_group(uid, gid)),
            true
        ).unwrap_or(false)
    }

    /// Get all groups a user belongs to
    /// 
    /// # Arguments
    /// * `uid` - Name of the user
    /// 
    /// # Returns
    /// * `Vec<String>` - Array with group names
    fn get_user_groups(&self, uid: &str) -> Vec<String> {
        let mut groups = Vec::new();

        for (_, backend) in &self.backends {
            let backend_groups = backend.get_user_groups(uid);
            groups.extend(backend_groups);
        }

        groups
    }

    /// Get a list of all users in a group
    /// 
    /// # Arguments
    /// * `gid` - Group ID
    /// * `search` - Search term (optional)
    /// * `limit` - Limit the number of results
    /// * `offset` - Offset for results
    /// 
    /// # Returns
    /// * `Vec<String>` - Array with user IDs
    fn users_in_group(&self, gid: &str, search: &str, limit: i32, offset: i32) -> Vec<String> {
        let mut users = Vec::new();

        for (_, backend) in &self.backends {
            let backend_users = backend.users_in_group(gid, search, limit, offset);
            users.extend(backend_users);
        }

        users
    }

    /// Get a list of all display names in a group
    /// 
    /// # Arguments
    /// * `gid` - Group ID
    /// * `search` - Search term
    /// * `limit` - Limit the number of results
    /// * `offset` - Offset for results
    /// 
    /// # Returns
    /// * `HashMap<String, String>` - Map with display names (value) and user IDs (key)
    fn display_names_in_group(&self, gid: &str, search: &str, limit: i32, offset: i32) -> HashMap<String, String> {
        let mut display_names = HashMap::new();

        for (_, backend) in &self.backends {
            let backend_users = backend.display_names_in_group(gid, search, limit, offset);
            display_names.extend(backend_users);
        }

        display_names
    }

    /// Get a list of all groups
    /// 
    /// # Arguments
    /// * `search` - Search term (optional)
    /// * `limit` - Limit the number of results
    /// * `offset` - Offset for results
    /// 
    /// # Returns
    /// * `Vec<String>` - Array with group names
    fn get_groups(&self, search: &str, limit: i32, offset: i32) -> Vec<String> {
        let mut groups = Vec::new();

        for (_, backend) in &self.backends {
            let backend_groups = backend.get_groups(search, limit, offset);
            groups.extend(backend_groups);
        }

        groups
    }

    /// Check if a group exists
    /// 
    /// # Arguments
    /// * `gid` - Group ID
    /// 
    /// # Returns
    /// * `bool` - True if group exists, false otherwise
    fn group_exists(&self, gid: &str) -> bool {
        self.handle_request(
            gid,
            |backend| Some(backend.group_exists(gid)),
            false
        ).unwrap_or(false)
    }

    /// Check if backend implements actions
    /// 
    /// # Arguments
    /// * `actions` - Bitwise-OR'ed actions
    /// 
    /// # Returns
    /// * `bool` - Whether the actions are implemented
    fn implements_actions(&self, actions: i32) -> bool {
        if let Some(ref_key) = &self.ref_backend {
            if let Some(backend) = self.backends.get(ref_key) {
                return backend.implements_actions(actions);
            }
        }
        false
    }
}