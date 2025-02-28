// Copyright (C) 2012 Robin Appelman <icewind@owncloud.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library. If not, see <http://www.gnu.org/licenses/>.

use rand::Uuid;
use std::sync::Arc;

// Trait representing a group backend interface
pub trait GroupBackend {
    fn create_group(&self, name: &str) -> bool;
    fn delete_group(&self, name: &str) -> bool;
    fn get_groups(&self) -> Vec<String>;
    fn add_to_group(&self, user_id: &str, group_id: &str) -> bool;
    fn remove_from_group(&self, user_id: &str, group_id: &str) -> bool;
    fn in_group(&self, user_id: &str, group_id: &str) -> bool;
    fn users_in_group(&self, group_id: &str) -> Vec<String>;
    fn get_user_groups(&self, user_id: &str) -> Vec<String>;
}

// Abstract test class for testing group backends
pub struct TestGroupBackend<T>
where
    T: GroupBackend,
{
    backend: Arc<T>,
}

impl<T: GroupBackend> TestGroupBackend<T> {
    pub fn new(backend: Arc<T>) -> Self {
        Self { backend }
    }

    /// Get a new unique group name
    /// Test implementations can override this in order to clean up created groups
    pub fn get_group_name(&self) -> String {
        format!("test_{}", Uuid::new_v4())
    }

    /// Get a new unique user name
    /// Test implementations can override this in order to clean up created users
    pub fn get_user_name(&self) -> String {
        format!("test_{}", Uuid::new_v4())
    }

    pub fn test_add_remove(&self) {
        // Get the number of groups we start with, in case there are existing groups
        let start_count = self.backend.get_groups().len();

        let name1 = self.get_group_name();
        let name2 = self.get_group_name();
        
        self.backend.create_group(&name1);
        let count = self.backend.get_groups().len() - start_count;
        assert_eq!(1, count);
        assert!(self.backend.get_groups().contains(&name1));
        assert!(!self.backend.get_groups().contains(&name2));
        
        self.backend.create_group(&name2);
        let count = self.backend.get_groups().len() - start_count;
        assert_eq!(2, count);
        assert!(self.backend.get_groups().contains(&name1));
        assert!(self.backend.get_groups().contains(&name2));

        self.backend.delete_group(&name2);
        let count = self.backend.get_groups().len() - start_count;
        assert_eq!(1, count);
        assert!(self.backend.get_groups().contains(&name1));
        assert!(!self.backend.get_groups().contains(&name2));
    }

    pub fn test_user(&self) {
        let group1 = self.get_group_name();
        let group2 = self.get_group_name();
        self.backend.create_group(&group1);
        self.backend.create_group(&group2);

        let user1 = self.get_user_name();
        let user2 = self.get_user_name();

        assert!(!self.backend.in_group(&user1, &group1));
        assert!(!self.backend.in_group(&user2, &group1));
        assert!(!self.backend.in_group(&user1, &group2));
        assert!(!self.backend.in_group(&user2, &group2));

        assert!(self.backend.add_to_group(&user1, &group1));

        assert!(self.backend.in_group(&user1, &group1));
        assert!(!self.backend.in_group(&user2, &group1));
        assert!(!self.backend.in_group(&user1, &group2));
        assert!(!self.backend.in_group(&user2, &group2));
        
        assert!(!self.backend.add_to_group(&user1, &group1));

        assert_eq!(vec![user1.clone()], self.backend.users_in_group(&group1));
        assert_eq!(Vec::<String>::new(), self.backend.users_in_group(&group2));

        assert_eq!(vec![group1.clone()], self.backend.get_user_groups(&user1));
        assert_eq!(Vec::<String>::new(), self.backend.get_user_groups(&user2));

        self.backend.delete_group(&group1);
        assert_eq!(Vec::<String>::new(), self.backend.get_user_groups(&user1));
        assert_eq!(Vec::<String>::new(), self.backend.users_in_group(&group1));
        assert!(!self.backend.in_group(&user1, &group1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock implementation of GroupBackend for testing
    struct MockGroupBackend {
        // Implementation details would go here
    }
    
    impl GroupBackend for MockGroupBackend {
        // Implementation of the trait methods would go here
        // This is just a placeholder to show how the tests would be structured
        fn create_group(&self, _name: &str) -> bool { unimplemented!() }
        fn delete_group(&self, _name: &str) -> bool { unimplemented!() }
        fn get_groups(&self) -> Vec<String> { unimplemented!() }
        fn add_to_group(&self, _user_id: &str, _group_id: &str) -> bool { unimplemented!() }
        fn remove_from_group(&self, _user_id: &str, _group_id: &str) -> bool { unimplemented!() }
        fn in_group(&self, _user_id: &str, _group_id: &str) -> bool { unimplemented!() }
        fn users_in_group(&self, _group_id: &str) -> Vec<String> { unimplemented!() }
        fn get_user_groups(&self, _user_id: &str) -> Vec<String> { unimplemented!() }
    }
    
    // Example of how tests would be defined
    #[test]
    fn example_test() {
        // This would be implemented by specific backend test implementations
    }
}