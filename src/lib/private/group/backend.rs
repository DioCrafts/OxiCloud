//! ownCloud
//!
//! @author Frank Karlitschek
//! @copyright 2012 Frank Karlitschek frank@owncloud.org
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;

/// Error code for functions not provided by the group backend
pub const GROUP_BACKEND_NOT_IMPLEMENTED: i32 = -501;

/// Actions that user backends can define
pub const GROUP_BACKEND_CREATE_GROUP: u32 = 0x00000001;
pub const GROUP_BACKEND_DELETE_GROUP: u32 = 0x00000010;
pub const GROUP_BACKEND_ADD_TO_GROUP: u32 = 0x00000100;
pub const GROUP_BACKEND_REMOVE_FROM_GROUP: u32 = 0x00001000;
pub const GROUP_BACKEND_GET_DISPLAYNAME: u32 = 0x00010000;

/// Interface for group management
pub trait GroupInterface {
    /// Get all supported actions
    fn get_supported_actions(&self) -> u32;
    
    /// Check if backend implements actions
    fn implements_actions(&self, actions: u32) -> bool;
    
    /// Check if user is in group
    fn in_group(&self, uid: &str, gid: &str) -> bool;
    
    /// Get all groups a user belongs to
    fn get_user_groups(&self, uid: &str) -> Vec<String>;
    
    /// Get a list of all groups
    fn get_groups(&self, search: &str, limit: i32, offset: i32) -> Vec<String>;
    
    /// Check if a group exists
    fn group_exists(&self, gid: &str) -> bool;
    
    /// Get a list of all users in a group
    fn users_in_group(&self, gid: &str, search: &str, limit: i32, offset: i32) -> Vec<String>;
    
    /// Get a list of all display names in a group
    fn display_names_in_group(&self, gid: &str, search: &str, limit: i32, offset: i32) -> HashMap<String, String>;
}

/// Abstract base class for group management
pub struct GroupBackend {
    possible_actions: HashMap<u32, &'static str>,
}

impl Default for GroupBackend {
    fn default() -> Self {
        let mut possible_actions = HashMap::new();
        possible_actions.insert(GROUP_BACKEND_CREATE_GROUP, "create_group");
        possible_actions.insert(GROUP_BACKEND_DELETE_GROUP, "delete_group");
        possible_actions.insert(GROUP_BACKEND_ADD_TO_GROUP, "add_to_group");
        possible_actions.insert(GROUP_BACKEND_REMOVE_FROM_GROUP, "remove_from_group");
        possible_actions.insert(GROUP_BACKEND_GET_DISPLAYNAME, "display_names_in_group");
        
        Self { possible_actions }
    }
}

impl GroupBackend {
    /// Create a new GroupBackend
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Helper method to check if a method exists
    fn method_exists(&self, method_name: &str) -> bool {
        // In Rust, this would typically be determined by trait implementation
        // This is a placeholder that would need to be replaced with actual implementation
        match method_name {
            "create_group" | "delete_group" | "add_to_group" | 
            "remove_from_group" | "display_names_in_group" => true,
            _ => false,
        }
    }
}

impl GroupInterface for GroupBackend {
    /// Get all supported actions
    fn get_supported_actions(&self) -> u32 {
        let mut actions = 0;
        for (&action, method_name) in &self.possible_actions {
            if self.method_exists(method_name) {
                actions |= action;
            }
        }
        actions
    }
    
    /// Check if backend implements actions
    fn implements_actions(&self, actions: u32) -> bool {
        (self.get_supported_actions() & actions) != 0
    }
    
    /// Check if user is in group
    fn in_group(&self, uid: &str, gid: &str) -> bool {
        self.get_user_groups(uid).contains(&gid.to_string())
    }
    
    /// Get all groups a user belongs to
    fn get_user_groups(&self, _uid: &str) -> Vec<String> {
        Vec::new()
    }
    
    /// Get a list of all groups
    fn get_groups(&self, _search: &str, _limit: i32, _offset: i32) -> Vec<String> {
        Vec::new()
    }
    
    /// Check if a group exists
    fn group_exists(&self, gid: &str) -> bool {
        self.get_groups(gid, 1, 0).contains(&gid.to_string())
    }
    
    /// Get a list of all users in a group
    fn users_in_group(&self, _gid: &str, _search: &str, _limit: i32, _offset: i32) -> Vec<String> {
        Vec::new()
    }
    
    /// Get a list of all display names in a group
    fn display_names_in_group(&self, gid: &str, search: &str, limit: i32, offset: i32) -> HashMap<String, String> {
        let mut display_names = HashMap::new();
        let users = self.users_in_group(gid, search, limit, offset);
        
        for user in users {
            display_names.insert(user.clone(), user);
        }
        
        display_names
    }
}