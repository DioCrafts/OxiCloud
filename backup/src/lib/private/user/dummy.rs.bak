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

/// User backend trait defining operations that can be performed on users
pub trait UserBackend {
    fn create_user(&mut self, uid: &str, password: &str) -> bool;
    fn delete_user(&mut self, uid: &str) -> bool;
    fn set_password(&mut self, uid: &str, password: &str) -> bool;
    fn check_password(&self, uid: &str, password: &str) -> Option<String>;
    fn get_users(&self, search: &str, limit: Option<usize>, offset: Option<usize>) -> Vec<String>;
    fn user_exists(&self, uid: &str) -> bool;
    fn has_user_listings(&self) -> bool;
}

/// Dummy user backend, does not keep state, only for testing use
pub struct DummyUserBackend {
    users: HashMap<String, String>,
}

impl DummyUserBackend {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

impl Default for DummyUserBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl UserBackend for DummyUserBackend {
    /// Create a new user
    ///
    /// Creates a new user. Basic checking of username is done in the User module
    /// itself, not in its implementations.
    ///
    /// # Arguments
    /// * `uid` - The username of the user to create
    /// * `password` - The password of the new user
    ///
    /// # Returns
    /// * `bool` - Whether the user was successfully created
    fn create_user(&mut self, uid: &str, password: &str) -> bool {
        if self.users.contains_key(uid) {
            false
        } else {
            self.users.insert(uid.to_string(), password.to_string());
            true
        }
    }

    /// Delete a user
    ///
    /// # Arguments
    /// * `uid` - The username of the user to delete
    ///
    /// # Returns
    /// * `bool` - Whether the user was successfully deleted
    fn delete_user(&mut self, uid: &str) -> bool {
        self.users.remove(uid).is_some()
    }

    /// Set password
    ///
    /// Change the password of a user
    ///
    /// # Arguments
    /// * `uid` - The username
    /// * `password` - The new password
    ///
    /// # Returns
    /// * `bool` - Whether the password was successfully updated
    fn set_password(&mut self, uid: &str, password: &str) -> bool {
        if let Some(pwd) = self.users.get_mut(uid) {
            *pwd = password.to_string();
            true
        } else {
            false
        }
    }

    /// Check if the password is correct
    ///
    /// Check if the password is correct without logging in the user
    /// returns the user id as Some(String) or None if invalid
    ///
    /// # Arguments
    /// * `uid` - The username
    /// * `password` - The password
    ///
    /// # Returns
    /// * `Option<String>` - Some(uid) if the password is correct, None otherwise
    fn check_password(&self, uid: &str, password: &str) -> Option<String> {
        match self.users.get(uid) {
            Some(pwd) if pwd == password => Some(uid.to_string()),
            _ => None,
        }
    }

    /// Get a list of all users
    ///
    /// # Arguments
    /// * `search` - Optional search string to filter users
    /// * `limit` - Optional maximum number of users to return
    /// * `offset` - Optional offset for pagination
    ///
    /// # Returns
    /// * `Vec<String>` - List of user IDs
    fn get_users(&self, _search: &str, _limit: Option<usize>, _offset: Option<usize>) -> Vec<String> {
        self.users.keys().cloned().collect()
    }

    /// Check if a user exists
    ///
    /// # Arguments
    /// * `uid` - The username
    ///
    /// # Returns
    /// * `bool` - Whether the user exists
    fn user_exists(&self, uid: &str) -> bool {
        self.users.contains_key(uid)
    }

    /// Check if this backend supports listing users
    ///
    /// # Returns
    /// * `bool` - Always returns true for this implementation
    fn has_user_listings(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let mut backend = DummyUserBackend::new();
        assert!(backend.create_user("test", "password"));
        assert!(!backend.create_user("test", "password")); // Already exists
    }

    #[test]
    fn test_delete_user() {
        let mut backend = DummyUserBackend::new();
        backend.create_user("test", "password");
        assert!(backend.delete_user("test"));
        assert!(!backend.delete_user("test")); // Already deleted
    }

    #[test]
    fn test_set_password() {
        let mut backend = DummyUserBackend::new();
        backend.create_user("test", "password");
        assert!(backend.set_password("test", "newpassword"));
        assert!(!backend.set_password("nonexistent", "password")); // User doesn't exist
    }

    #[test]
    fn test_check_password() {
        let mut backend = DummyUserBackend::new();
        backend.create_user("test", "password");
        assert_eq!(backend.check_password("test", "password"), Some("test".to_string()));
        assert_eq!(backend.check_password("test", "wrongpassword"), None);
        assert_eq!(backend.check_password("nonexistent", "password"), None);
    }

    #[test]
    fn test_user_exists() {
        let mut backend = DummyUserBackend::new();
        backend.create_user("test", "password");
        assert!(backend.user_exists("test"));
        assert!(!backend.user_exists("nonexistent"));
    }

    #[test]
    fn test_get_users() {
        let mut backend = DummyUserBackend::new();
        backend.create_user("test1", "password");
        backend.create_user("test2", "password");
        let users = backend.get_users("", None, None);
        assert_eq!(users.len(), 2);
        assert!(users.contains(&"test1".to_string()));
        assert!(users.contains(&"test2".to_string()));
    }
}