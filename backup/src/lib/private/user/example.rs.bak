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

use std::path::PathBuf;
use async_trait::async_trait;

/// Reference trait for user management
/// This trait should only be used as a reference for method signatures and their descriptions
#[async_trait]
pub trait UserBackend: Send + Sync {
    /// Create a new user
    ///
    /// Creates a new user. Basic checking of username is done in User
    /// itself, not in its implementations.
    ///
    /// # Parameters
    /// * `uid` - The username of the user to create
    /// * `password` - The password of the new user
    ///
    /// # Returns
    /// `Result<(), UserError>` - Ok on success, Err otherwise
    async fn create_user(&self, uid: &str, password: &str) -> Result<(), UserError>;

    /// Set password
    ///
    /// Change the password of a user
    ///
    /// # Parameters
    /// * `uid` - The username
    /// * `password` - The new password
    ///
    /// # Returns
    /// `Result<(), UserError>` - Ok on success, Err otherwise
    async fn set_password(&self, uid: &str, password: &str) -> Result<(), UserError>;

    /// Check if the password is correct
    ///
    /// Check if the password is correct without logging in the user
    ///
    /// # Parameters
    /// * `uid` - The username
    /// * `password` - The password
    ///
    /// # Returns
    /// `Option<String>` - Some(user_id) if correct, None otherwise
    async fn check_password(&self, uid: &str, password: &str) -> Option<String>;

    /// Get the user's home directory
    ///
    /// # Parameters
    /// * `uid` - The username
    ///
    /// # Returns
    /// `Option<PathBuf>` - Some(path) if found, None otherwise
    async fn get_home(&self, uid: &str) -> Option<PathBuf>;
}

/// Example user backend implementation
pub struct UserExample {
    // Implementation details would go here
}

#[async_trait]
impl UserBackend for UserExample {
    async fn create_user(&self, uid: &str, password: &str) -> Result<(), UserError> {
        // Implementation would go here
        unimplemented!()
    }

    async fn set_password(&self, uid: &str, password: &str) -> Result<(), UserError> {
        // Implementation would go here
        unimplemented!()
    }

    async fn check_password(&self, uid: &str, password: &str) -> Option<String> {
        // Implementation would go here
        unimplemented!()
    }

    async fn get_home(&self, uid: &str) -> Option<PathBuf> {
        // Implementation would go here
        unimplemented!()
    }
}

/// Error type for user operations
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("User already exists")]
    UserExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Unknown error: {0}")]
    Other(String),
}