//! ownCloud
//!
//! @author Robin Appelman
//! @copyright 2012 Robin Appelman icewind@owncloud.com
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

use std::collections::HashSet;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use async_trait::async_trait;

/// Trait representing backend user operations
#[async_trait]
pub trait UserBackend {
    /// Create a new user
    async fn create_user(&self, name: &str, password: &str) -> bool;
    
    /// Delete a user
    async fn delete_user(&self, name: &str) -> bool;
    
    /// Check if a user exists
    async fn user_exists(&self, name: &str) -> bool;
    
    /// Get a list of all user IDs
    async fn get_users(&self) -> Vec<String>;
    
    /// Check if the password is correct
    async fn check_password(&self, name: &str, password: &str) -> Option<String>;
    
    /// Update a user's password
    async fn set_password(&self, name: &str, password: &str) -> bool;
}

/// Abstract test case for user backend implementations
///
/// All implementations MUST initialize a backend field in set_up() 
/// which implements the UserBackend trait. Test methods in this class 
/// will then be run on each separate implementation and backend therein.
///
/// For an example see /tests/lib/user/dummy.rs
pub struct TestUserBackend<T: UserBackend> {
    backend: T,
}

impl<T: UserBackend> TestUserBackend<T> {
    /// Create a new test instance with the given backend
    pub fn new(backend: T) -> Self {
        Self { backend }
    }
    
    /// Get a new unique user name
    /// Test implementations can override this in order to clean up created users
    pub fn get_user(&self) -> String {
        let random_suffix: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        
        format!("test_{}", random_suffix)
    }
    
    /// Test adding and removing users
    pub async fn test_add_remove(&self) {
        // Get the number of groups we start with, in case there are existing groups
        let start_count = self.backend.get_users().await.len();
        
        let name1 = self.get_user();
        let name2 = self.get_user();
        
        self.backend.create_user(&name1, "").await;
        let users = self.backend.get_users().await;
        let count = users.len() - start_count;
        assert_eq!(1, count);
        assert!(users.contains(&name1));
        assert!(!users.contains(&name2));
        
        self.backend.create_user(&name2, "").await;
        let users = self.backend.get_users().await;
        let count = users.len() - start_count;
        assert_eq!(2, count);
        assert!(users.contains(&name1));
        assert!(users.contains(&name2));
        
        self.backend.delete_user(&name2).await;
        let users = self.backend.get_users().await;
        let count = users.len() - start_count;
        assert_eq!(1, count);
        assert!(users.contains(&name1));
        assert!(!users.contains(&name2));
    }
    
    /// Test login functionality
    pub async fn test_login(&self) {
        let name1 = self.get_user();
        let name2 = self.get_user();
        
        assert!(!self.backend.user_exists(&name1).await);
        assert!(!self.backend.user_exists(&name2).await);
        
        self.backend.create_user(&name1, "pass1").await;
        self.backend.create_user(&name2, "pass2").await;
        
        assert!(self.backend.user_exists(&name1).await);
        assert!(self.backend.user_exists(&name2).await);
        
        assert_eq!(Some(name1.clone()), self.backend.check_password(&name1, "pass1").await);
        assert_eq!(Some(name2.clone()), self.backend.check_password(&name2, "pass2").await);
        
        assert_eq!(None, self.backend.check_password(&name1, "pass2").await);
        assert_eq!(None, self.backend.check_password(&name2, "pass1").await);
        
        assert_eq!(None, self.backend.check_password(&name1, "dummy").await);
        assert_eq!(None, self.backend.check_password(&name2, "foobar").await);
        
        self.backend.set_password(&name1, "newpass1").await;
        assert_eq!(None, self.backend.check_password(&name1, "pass1").await);
        assert_eq!(Some(name1.clone()), self.backend.check_password(&name1, "newpass1").await);
        assert_eq!(None, self.backend.check_password(&name2, "newpass1").await);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Implementation would be provided here or in separate test modules
}