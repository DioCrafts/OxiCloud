// ownCloud
//
// Original authors:
// - Frank Karlitschek
// - Dominik Schmidt
// Copyright 2012 Frank Karlitschek frank@owncloud.org
// Copyright 2011 Dominik Schmidt dev@dominik-schmidt.de
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

/// Error code for functions not provided by the user backend
pub const USER_BACKEND_NOT_IMPLEMENTED: i32 = -501;

/// Actions that user backends can define
pub const USER_BACKEND_CREATE_USER: u32 = 0x000001;
pub const USER_BACKEND_SET_PASSWORD: u32 = 0x000010;
pub const USER_BACKEND_CHECK_PASSWORD: u32 = 0x000100;
pub const USER_BACKEND_GET_HOME: u32 = 0x001000;
pub const USER_BACKEND_GET_DISPLAYNAME: u32 = 0x010000;
pub const USER_BACKEND_SET_DISPLAYNAME: u32 = 0x100000;

/// User interface trait that all user backends must implement
pub trait UserInterface {
    /// Create a new user
    fn create_user(&self, uid: &str, password: &str) -> bool;

    /// Set password for user
    fn set_password(&self, uid: &str, password: &str) -> bool;

    /// Check if password is correct
    fn check_password(&self, uid: &str, password: &str) -> Option<String>;

    /// Get home directory for user
    fn get_home(&self, uid: &str) -> Option<String>;

    /// Get display name of user
    fn get_display_name(&self, uid: &str) -> Option<String>;

    /// Set display name of user
    fn set_display_name(&self, uid: &str, display_name: &str) -> bool;
}

/// Abstract base class for user management. Provides methods for querying backend
/// capabilities.
///
/// Subclass this for your own backends, and see UserExample for descriptions
pub trait UserBackend: UserInterface {
    /// Get all supported actions
    ///
    /// Returns the supported actions as int to be
    /// compared with USER_BACKEND_CREATE_USER etc.
    fn get_supported_actions(&self) -> u32;

    /// Check if backend implements actions
    ///
    /// Returns the supported actions as int to be
    /// compared with USER_BACKEND_CREATE_USER etc.
    fn implements_actions(&self, actions: u32) -> bool {
        (self.get_supported_actions() & actions) != 0
    }

    /// Delete a user
    ///
    /// Deletes a user
    fn delete_user(&self, uid: &str) -> bool {
        false
    }

    /// Get a list of all users
    ///
    /// Get a list of all users.
    fn get_users(&self, search: &str, limit: Option<usize>, offset: Option<usize>) -> Vec<String> {
        Vec::new()
    }

    /// Check if a user exists
    fn user_exists(&self, uid: &str) -> bool {
        false
    }

    /// Get a list of all display names
    ///
    /// Get a list of all display names and user ids.
    fn get_display_names(&self, search: &str, limit: Option<usize>, offset: Option<usize>) -> HashMap<String, String> {
        let mut display_names = HashMap::new();
        let users = self.get_users(search, limit, offset);
        for user in users {
            display_names.insert(user.clone(), user);
        }
        display_names
    }

    /// Check if a user list is available or not
    fn has_user_listings(&self) -> bool {
        false
    }
}

/// Implementation of UserBackend for implementors of UserInterface
impl<T: UserInterface> UserBackend for T {
    fn get_supported_actions(&self) -> u32 {
        let mut actions = 0;
        
        // Check if methods are implemented by using trait objects and vtable
        
        // This approach doesn't work in Rust as it does in PHP
        // Instead, each implementation should override this method
        // and specify which actions it supports
        
        actions
    }
}