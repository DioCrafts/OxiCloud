// Copyright (c) 2012 Arthur Schiwon blizzz@owncloud.org
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

/// Bitwise flags for user backend actions
pub struct UserBackendActions;

impl UserBackendActions {
    pub const CREATE_USER: u32 = 0x00000001;
    // Additional action constants can be defined here as needed
}

/// User interface for user backends
pub trait UserInterface {
    /// Check if backend implements actions
    ///
    /// Returns the supported actions as int to be
    /// compared with UserBackendActions constants
    fn implements_actions(&self, actions: u32) -> bool;

    /// Delete a user
    ///
    /// # Arguments
    /// * `uid` - The username of the user to delete
    ///
    /// # Returns
    /// `true` if successful, `false` otherwise
    fn delete_user(&self, uid: &str) -> bool;

    /// Get a list of all users
    ///
    /// # Arguments
    /// * `search` - Optional search term to filter users
    /// * `limit` - Optional maximum number of results
    /// * `offset` - Optional offset for pagination
    ///
    /// # Returns
    /// Vector with all matching user IDs
    fn get_users(&self, search: &str, limit: Option<usize>, offset: Option<usize>) -> Vec<String>;

    /// Check if a user exists
    ///
    /// # Arguments
    /// * `uid` - The username to check
    ///
    /// # Returns
    /// `true` if the user exists, `false` otherwise
    fn user_exists(&self, uid: &str) -> bool;

    /// Get display name of the user
    ///
    /// # Arguments
    /// * `uid` - User ID of the user
    ///
    /// # Returns
    /// Display name if found, None otherwise
    fn get_display_name(&self, uid: &str) -> Option<String>;

    /// Get a list of all display names
    ///
    /// # Arguments
    /// * `search` - Optional search term to filter users
    /// * `limit` - Optional maximum number of results
    /// * `offset` - Optional offset for pagination
    ///
    /// # Returns
    /// HashMap with user IDs as keys and display names as values
    fn get_display_names(
        &self, 
        search: &str, 
        limit: Option<usize>, 
        offset: Option<usize>
    ) -> HashMap<String, String>;

    /// Check if a user list is available or not
    ///
    /// # Returns
    /// `true` if users can be listed, `false` otherwise
    fn has_user_listings(&self) -> bool;
}