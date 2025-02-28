// Copyright (c) 2012 Frank Karlitschek frank@owncloud.org
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

/// Trait that defines the backend interface for group management
pub trait GroupBackend {
    fn create_group(&mut self, gid: &str) -> bool;
    fn delete_group(&mut self, gid: &str) -> bool;
    fn in_group(&self, uid: &str, gid: &str) -> bool;
    fn add_to_group(&mut self, uid: &str, gid: &str) -> bool;
    fn remove_from_group(&mut self, uid: &str, gid: &str) -> bool;
    fn get_user_groups(&self, uid: &str) -> Vec<String>;
    fn get_groups(&self, search: &str, limit: i32, offset: i32) -> Vec<String>;
    fn users_in_group(&self, gid: &str, search: &str, limit: i32, offset: i32) -> Vec<String>;
}

/// Dummy group backend, does not keep state, only for testing use
pub struct GroupDummy {
    groups: HashMap<String, Vec<String>>,
}

impl GroupDummy {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
        }
    }
}

impl Default for GroupDummy {
    fn default() -> Self {
        Self::new()
    }
}

impl GroupBackend for GroupDummy {
    /// Try to create a new group
    ///
    /// Tries to create a new group. If the group name already exists, false will
    /// be returned.
    ///
    /// # Arguments
    /// * `gid` - The name of the group to create
    ///
    /// # Returns
    /// * `true` if group was created successfully
    /// * `false` if group already exists
    fn create_group(&mut self, gid: &str) -> bool {
        if !self.groups.contains_key(gid) {
            self.groups.insert(gid.to_string(), Vec::new());
            true
        } else {
            false
        }
    }

    /// Delete a group
    ///
    /// Deletes a group and removes it from the group_user-table
    ///
    /// # Arguments
    /// * `gid` - gid of the group to delete
    ///
    /// # Returns
    /// * `true` if group was deleted successfully
    /// * `false` if group doesn't exist
    fn delete_group(&mut self, gid: &str) -> bool {
        self.groups.remove(gid).is_some()
    }

    /// Check if user is in group
    ///
    /// Checks whether the user is member of a group or not.
    ///
    /// # Arguments
    /// * `uid` - uid of the user
    /// * `gid` - gid of the group
    ///
    /// # Returns
    /// * `true` if user is in group
    /// * `false` otherwise
    fn in_group(&self, uid: &str, gid: &str) -> bool {
        if let Some(users) = self.groups.get(gid) {
            users.iter().any(|user| user == uid)
        } else {
            false
        }
    }

    /// Add a user to a group
    ///
    /// # Arguments
    /// * `uid` - Name of the user to add to group
    /// * `gid` - Name of the group in which add the user
    ///
    /// # Returns
    /// * `true` if user was added successfully
    /// * `false` if user already in group or group doesn't exist
    fn add_to_group(&mut self, uid: &str, gid: &str) -> bool {
        if let Some(users) = self.groups.get_mut(gid) {
            if !users.iter().any(|user| user == uid) {
                users.push(uid.to_string());
                return true;
            }
        }
        false
    }

    /// Removes a user from a group
    ///
    /// # Arguments
    /// * `uid` - Name of the user to remove from group
    /// * `gid` - Name of the group from which remove the user
    ///
    /// # Returns
    /// * `true` if user was removed successfully
    /// * `false` if user not in group or group doesn't exist
    fn remove_from_group(&mut self, uid: &str, gid: &str) -> bool {
        if let Some(users) = self.groups.get_mut(gid) {
            if let Some(index) = users.iter().position(|user| user == uid) {
                users.remove(index);
                return true;
            }
        }
        false
    }

    /// Get all groups a user belongs to
    ///
    /// This function fetches all groups a user belongs to. It does not check
    /// if the user exists at all.
    ///
    /// # Arguments
    /// * `uid` - Name of the user
    ///
    /// # Returns
    /// * Vector with group names
    fn get_user_groups(&self, uid: &str) -> Vec<String> {
        let mut groups = Vec::new();
        for (group, _) in &self.groups {
            if self.in_group(uid, group) {
                groups.push(group.clone());
            }
        }
        groups
    }

    /// Get a list of all groups
    ///
    /// # Arguments
    /// * `search` - Optional search string
    /// * `limit` - Optional limit
    /// * `offset` - Optional offset
    ///
    /// # Returns
    /// * Vector with group names
    fn get_groups(&self, _search: &str, _limit: i32, _offset: i32) -> Vec<String> {
        self.groups.keys().cloned().collect()
    }

    /// Get a list of all users in a group
    ///
    /// # Arguments
    /// * `gid` - Group ID
    /// * `search` - Optional search string
    /// * `limit` - Optional limit
    /// * `offset` - Optional offset
    ///
    /// # Returns
    /// * Vector with user IDs
    fn users_in_group(&self, gid: &str, _search: &str, _limit: i32, _offset: i32) -> Vec<String> {
        if let Some(users) = self.groups.get(gid) {
            users.clone()
        } else {
            Vec::new()
        }
    }
}