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

/// Abstract trait for group management
/// This trait should only be used as a reference for method signatures and their descriptions
#[async_trait::async_trait]
pub trait GroupExample {
    /// Try to create a new group
    ///
    /// # Arguments
    /// * `gid` - The name of the group to create
    ///
    /// # Returns
    /// `Ok(())` on success, `Err` if the group name already exists
    ///
    /// Tries to create a new group. If the group name already exists,
    /// an error will be returned.
    async fn create_group(gid: &str) -> Result<(), GroupError>;

    /// Delete a group
    ///
    /// # Arguments
    /// * `gid` - ID of the group to delete
    ///
    /// # Returns
    /// `Ok(())` on success, `Err` otherwise
    ///
    /// Deletes a group and removes it from the group_user table
    async fn delete_group(gid: &str) -> Result<(), GroupError>;

    /// Check if user is in group
    ///
    /// # Arguments
    /// * `uid` - ID of the user
    /// * `gid` - ID of the group
    ///
    /// # Returns
    /// `true` if user is member of the group, `false` otherwise
    ///
    /// Checks whether the user is member of a group or not.
    async fn in_group(uid: &str, gid: &str) -> Result<bool, GroupError>;

    /// Add a user to a group
    ///
    /// # Arguments
    /// * `uid` - Name of the user to add to group
    /// * `gid` - Name of the group in which to add the user
    ///
    /// # Returns
    /// `Ok(())` on success, `Err` otherwise
    ///
    /// Adds a user to a group.
    async fn add_to_group(uid: &str, gid: &str) -> Result<(), GroupError>;

    /// Remove a user from a group
    ///
    /// # Arguments
    /// * `uid` - Name of the user to remove from group
    /// * `gid` - Name of the group from which to remove the user
    ///
    /// # Returns
    /// `Ok(())` on success, `Err` otherwise
    ///
    /// Removes the user from a group.
    async fn remove_from_group(uid: &str, gid: &str) -> Result<(), GroupError>;

    /// Get all groups a user belongs to
    ///
    /// # Arguments
    /// * `uid` - Name of the user
    ///
    /// # Returns
    /// Vector with group names
    ///
    /// This function fetches all groups a user belongs to. It does not check
    /// if the user exists at all.
    async fn get_user_groups(uid: &str) -> Result<Vec<String>, GroupError>;

    /// Get a list of all groups
    ///
    /// # Arguments
    /// * `search` - Optional search string to filter groups
    /// * `limit` - Maximum number of results to return, -1 for no limit
    /// * `offset` - Number of results to skip
    ///
    /// # Returns
    /// Vector with group names
    ///
    /// Returns a list with all groups matching the criteria
    async fn get_groups(search: &str, limit: i32, offset: usize) -> Result<Vec<String>, GroupError>;

    /// Check if a group exists
    ///
    /// # Arguments
    /// * `gid` - ID of the group
    ///
    /// # Returns
    /// `true` if the group exists, `false` otherwise
    async fn group_exists(gid: &str) -> Result<bool, GroupError>;

    /// Get a list of all users in a group
    ///
    /// # Arguments
    /// * `gid` - ID of the group
    /// * `search` - Optional search string to filter users
    /// * `limit` - Maximum number of results to return, -1 for no limit
    /// * `offset` - Number of results to skip
    ///
    /// # Returns
    /// Vector with user IDs
    async fn users_in_group(gid: &str, search: &str, limit: i32, offset: usize) -> Result<Vec<String>, GroupError>;
}

/// Error type for group operations
#[derive(Debug, thiserror::Error)]
pub enum GroupError {
    #[error("Group already exists")]
    GroupExists,
    
    #[error("Group not found")]
    GroupNotFound,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Unknown error: {0}")]
    Other(String),
}