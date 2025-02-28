// OwnCloud - group interface
//
// @author Arthur Schiwon
// @copyright 2012 Arthur Schiwon blizzz@owncloud.org
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

/// Constants for group backend actions
pub mod actions {
    pub const CREATE_GROUP: u32 = 0x00000001;
    pub const DELETE_GROUP: u32 = 0x00000010;
    pub const ADD_USER: u32 = 0x00000100;
    pub const REMOVE_USER: u32 = 0x00001000;
    // Add more action constants as needed
}

/// Interface for group backends
#[async_trait::async_trait]
pub trait GroupBackend {
    /// Check if backend implements actions
    ///
    /// # Arguments
    /// * `actions` - Bitwise-or'ed actions
    ///
    /// # Returns
    /// Returns true if the backend supports the given actions
    async fn implements_actions(&self, actions: u32) -> bool;

    /// Is user in group?
    ///
    /// # Arguments
    /// * `uid` - User ID of the user
    /// * `gid` - Group ID of the group
    ///
    /// # Returns
    /// Returns true if the user is a member of the group
    async fn in_group(&self, uid: &str, gid: &str) -> Result<bool, GroupError>;

    /// Get all groups a user belongs to
    ///
    /// # Arguments
    /// * `uid` - Name of the user
    ///
    /// # Returns
    /// A vector with all group names the user belongs to.
    /// Does not check if the user exists at all.
    async fn get_user_groups(&self, uid: &str) -> Result<Vec<String>, GroupError>;

    /// Get a list of all groups
    ///
    /// # Arguments
    /// * `search` - Optional search string to filter groups
    /// * `limit` - Maximum number of results, -1 for no limit
    /// * `offset` - Offset for pagination
    ///
    /// # Returns
    /// A vector with all matching group names
    async fn get_groups(
        &self, 
        search: &str, 
        limit: i32, 
        offset: i32
    ) -> Result<Vec<String>, GroupError>;

    /// Check if a group exists
    ///
    /// # Arguments
    /// * `gid` - Group ID
    ///
    /// # Returns
    /// Returns true if the group exists
    async fn group_exists(&self, gid: &str) -> Result<bool, GroupError>;

    /// Get a list of all users in a group
    ///
    /// # Arguments
    /// * `gid` - Group ID
    /// * `search` - Optional search string to filter users
    /// * `limit` - Maximum number of results, -1 for no limit
    /// * `offset` - Offset for pagination
    ///
    /// # Returns
    /// A vector with all matching user IDs
    async fn users_in_group(
        &self, 
        gid: &str, 
        search: &str, 
        limit: i32, 
        offset: i32
    ) -> Result<Vec<String>, GroupError>;
}

/// Error type for group operations
#[derive(Debug, thiserror::Error)]
pub enum GroupError {
    #[error("Group not found: {0}")]
    GroupNotFound(String),
    
    #[error("User not found: {0}")]
    UserNotFound(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}