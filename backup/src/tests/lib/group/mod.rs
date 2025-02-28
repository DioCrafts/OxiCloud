// Módulos generados automáticamente

pub mod group;
pub mod dummy;
pub mod manager;
pub mod database;
pub mod backend;

// Contenido fusionado desde src/tests/lib/group.rs
// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// Copyright (c) 2012 Bernhard Posselt <nukeawhale@gmail.com>
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

use std::collections::{HashMap, HashSet};
use uuid::Uuid;

// Mock del backend de usuarios para pruebas
struct OcUserDummy {
    users: HashSet<String>,
}

impl OcUserDummy {
    fn new() -> Self {
        Self {
            users: HashSet::new(),
        }
    }

    fn create_user(&mut self, user_id: &str, _password: &str) -> bool {
        self.users.insert(user_id.to_string())
    }
}

// Mock del backend de grupos para pruebas
struct OcGroupDummy {
    groups: HashMap<String, HashSet<String>>,
}

impl OcGroupDummy {
    fn new() -> Self {
        Self {
            groups: HashMap::new(),
        }
    }

    fn get_groups(&self) -> Vec<String> {
        self.groups.keys().cloned().collect()
    }

    fn create_group(&mut self, group_id: &str) -> bool {
        if !self.groups.contains_key(group_id) {
            self.groups.insert(group_id.to_string(), HashSet::new());
            true
        } else {
            false
        }
    }

    fn in_group(&self, user_id: &str, group_id: &str) -> bool {
        self.groups
            .get(group_id)
            .map_or(false, |users| users.contains(user_id))
    }
}

// Manager para gestionar múltiples backends de grupos
struct OcGroupManager {
    backends: Vec<OcGroupDummy>,
    user_manager: OcUserManager,
}

impl OcGroupManager {
    fn new(user_manager: OcUserManager) -> Self {
        Self {
            backends: Vec::new(),
            user_manager,
        }
    }

    fn clear_backends(&mut self) {
        self.backends.clear();
    }

    fn use_backend(&mut self, backend: OcGroupDummy) {
        self.backends.push(backend);
    }

    fn create_group(&mut self, group_id: Option<&str>) -> bool {
        if let Some(group_id) = group_id {
            if group_id.is_empty() || self.group_exists(group_id) {
                return false;
            }

            if let Some(backend) = self.backends.first_mut() {
                return backend.create_group(group_id);
            }
        }
        false
    }

    fn delete_group(&mut self, group_id: &str) -> bool {
        if group_id == "admin" {
            return false;
        }

        for backend in &mut self.backends {
            if backend.groups.remove(group_id).is_some() {
                return true;
            }
        }
        false
    }

    fn group_exists(&self, group_id: &str) -> bool {
        for backend in &self.backends {
            if backend.groups.contains_key(group_id) {
                return true;
            }
        }
        false
    }

    fn get_groups(&self) -> Vec<String> {
        let mut result = Vec::new();
        for backend in &self.backends {
            result.extend(backend.get_groups());
        }
        result
    }

    fn add_to_group(&mut self, user_id: &str, group_id: &str) -> bool {
        if !self.group_exists(group_id) {
            return false;
        }

        for backend in &mut self.backends {
            if backend.groups.contains_key(group_id) {
                if let Some(users) = backend.groups.get_mut(group_id) {
                    users.insert(user_id.to_string());
                    return true;
                }
            }
        }
        false
    }

    fn in_group(&self, user_id: &str, group_id: &str) -> bool {
        for backend in &self.backends {
            if backend.in_group(user_id, group_id) {
                return true;
            }
        }
        false
    }

    fn users_in_group(&self, group_id: &str) -> Vec<String> {
        let mut result = Vec::new();
        for backend in &self.backends {
            if let Some(users) = backend.groups.get(group_id) {
                result.extend(users.iter().cloned());
            }
        }
        result
    }

    fn get_user_groups(&self, user_id: &str) -> Vec<String> {
        let mut result = Vec::new();
        for backend in &self.backends {
            for (group_id, users) in &backend.groups {
                if users.contains(user_id) {
                    result.push(group_id.clone());
                }
            }
        }
        result
    }

    fn display_names_in_group(&self, group_id: &str) -> HashMap<String, String> {
        let users = self.users_in_group(group_id);
        users.into_iter().map(|uid| (uid.clone(), uid)).collect()
    }

    fn users_in_groups(&self, groups: &[String]) -> Vec<String> {
        let mut result = HashSet::new();
        for group_id in groups {
            let users_in_group = self.users_in_group(group_id);
            for user in users_in_group {
                result.insert(user);
            }
        }
        result.into_iter().collect()
    }
}

// Manager para usuarios
struct OcUserManager {
    backends: Vec<OcUserDummy>,
}

impl OcUserManager {
    fn new() -> Self {
        Self {
            backends: Vec::new(),
        }
    }

    fn register_backend(&mut self, backend: OcUserDummy) {
        self.backends.push(backend);
    }

    fn clear_backends(&mut self) {
        self.backends.clear();
    }
}

// Módulo de pruebas
#[cfg(test)]
mod tests {
    use super::*;
    
    fn generate_unique_id() -> String {
        Uuid::new_v4().to_string()
    }

    struct TestGroupSetup {
        group_manager: OcGroupManager,
        user_manager: OcUserManager,
    }

    impl TestGroupSetup {
        fn new() -> Self {
            let mut user_manager = OcUserManager::new();
            let group_manager = OcGroupManager::new(user_manager.clone());
            
            Self {
                group_manager,
                user_manager,
            }
        }

        fn set_up(&mut self) {
            self.group_manager.clear_backends();
            self.user_manager.clear_backends();
        }
    }

    #[test]
    fn test_single_backend() {
        let mut setup = TestGroupSetup::new();
        setup.set_up();
        
        let mut user_backend = OcUserDummy::new();
        setup.user_manager.register_backend(user_backend);
        setup.group_manager.use_backend(OcGroupDummy::new());

        let group1 = generate_unique_id();
        let group2 = generate_unique_id();
        setup.group_manager.create_group(Some(&group1));
        setup.group_manager.create_group(Some(&group2));

        let user1 = generate_unique_id();
        let user2 = generate_unique_id();
        setup.user_manager.backends[0].create_user(&user1, "");
        setup.user_manager.backends[0].create_user(&user2, "");

        assert!(!setup.group_manager.in_group(&user1, &group1));
        assert!(!setup.group_manager.in_group(&user2, &group1));
        assert!(!setup.group_manager.in_group(&user1, &group2));
        assert!(!setup.group_manager.in_group(&user2, &group2));

        assert!(setup.group_manager.add_to_group(&user1, &group1));

        assert!(setup.group_manager.in_group(&user1, &group1));
        assert!(!setup.group_manager.in_group(&user2, &group1));
        assert!(!setup.group_manager.in_group(&user1, &group2));
        assert!(!setup.group_manager.in_group(&user2, &group2));

        assert!(setup.group_manager.add_to_group(&user1, &group1));

        assert_eq!(vec![user1.clone()], setup.group_manager.users_in_group(&group1));
        assert_eq!(Vec::<String>::new(), setup.group_manager.users_in_group(&group2));

        assert_eq!(vec![group1.clone()], setup.group_manager.get_user_groups(&user1));
        assert_eq!(Vec::<String>::new(), setup.group_manager.get_user_groups(&user2));

        setup.group_manager.delete_group(&group1);
        assert_eq!(Vec::<String>::new(), setup.group_manager.get_user_groups(&user1));
        assert_eq!(Vec::<String>::new(), setup.group_manager.users_in_group(&group1));
        assert!(!setup.group_manager.in_group(&user1, &group1));
    }

    #[test]
    fn test_no_empty_gids() {
        let mut setup = TestGroupSetup::new();
        setup.set_up();
        
        setup.group_manager.use_backend(OcGroupDummy::new());
        let empty_group = None;

        assert!(!setup.group_manager.create_group(empty_group));
    }

    #[test]
    fn test_no_groups_twice() {
        let mut setup = TestGroupSetup::new();
        setup.set_up();
        
        setup.group_manager.use_backend(OcGroupDummy::new());
        let group = generate_unique_id();
        setup.group_manager.create_group(Some(&group));

        let group_copy = group.clone();

        setup.group_manager.create_group(Some(&group_copy));
        assert_eq!(vec![group], setup.group_manager.get_groups());
    }

    #[test]
    fn test_dont_delete_admin_group() {
        let mut setup = TestGroupSetup::new();
        setup.set_up();
        
        setup.group_manager.use_backend(OcGroupDummy::new());
        let admin_group = "admin";
        setup.group_manager.create_group(Some(admin_group));

        assert!(!setup.group_manager.delete_group(admin_group));
        assert_eq!(vec![admin_group.to_string()], setup.group_manager.get_groups());
    }

    #[test]
    fn test_dont_add_user_to_nonexistent_group() {
        let mut setup = TestGroupSetup::new();
        setup.set_up();
        
        setup.group_manager.use_backend(OcGroupDummy::new());
        let group_non_existent = "notExistent";
        let user = generate_unique_id();

        assert_eq!(false, setup.group_manager.add_to_group(&user, group_non_existent));
        assert_eq!(Vec::<String>::new(), setup.group_manager.get_groups());
    }

    #[test]
    fn test_display_names_in_group() {
        let mut setup = TestGroupSetup::new();
        setup.set_up();
        
        setup.group_manager.use_backend(OcGroupDummy::new());
        let mut user_backend = OcUserDummy::new();
        setup.user_manager.register_backend(user_backend);

        let group1 = generate_unique_id();
        let user1 = "uid1";
        let user2 = "uid2";
        setup.group_manager.create_group(Some(&group1));
        setup.user_manager.backends[0].create_user(user1, "");
        setup.user_manager.backends[0].create_user(user2, "");
        setup.group_manager.add_to_group(user1, &group1);
        setup.group_manager.add_to_group(user2, &group1);
        
        let mut expected = HashMap::new();
        expected.insert(user1.to_string(), user1.to_string());
        expected.insert(user2.to_string(), user2.to_string());
        
        assert_eq!(expected, setup.group_manager.display_names_in_group(&group1));
    }

    #[test]
    fn test_users_in_group() {
        let mut setup = TestGroupSetup::new();
        setup.set_up();
        
        setup.group_manager.use_backend(OcGroupDummy::new());
        let mut user_backend = OcUserDummy::new();
        setup.user_manager.register_backend(user_backend);

        let group1 = generate_unique_id();
        let group2 = generate_unique_id();
        let group3 = generate_unique_id();
        let user1 = generate_unique_id();
        let user2 = generate_unique_id();
        let user3 = generate_unique_id();
        setup.group_manager.create_group(Some(&group1));
        setup.group_manager.create_group(Some(&group2));
        setup.group_manager.create_group(Some(&group3));

        setup.user_manager.backends[0].create_user(&user1, "");
        setup.user_manager.backends[0].create_user(&user2, "");
        setup.user_manager.backends[0].create_user(&user3, "");

        setup.group_manager.add_to_group(&user1, &group1);
        setup.group_manager.add_to_group(&user2, &group1);
        setup.group_manager.add_to_group(&user3, &group1);
        setup.group_manager.add_to_group(&user3, &group2);

        let mut expected = vec![user1.clone(), user2.clone(), user3.clone()];
        expected.sort();
        
        let mut actual = setup.group_manager.users_in_groups(&[group1.clone(), group2.clone(), group3.clone()]);
        actual.sort();
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multi_backend() {
        let mut setup = TestGroupSetup::new();
        setup.set_up();
        
        let mut user_backend = OcUserDummy::new();
        setup.user_manager.register_backend(user_backend);
        let backend1 = OcGroupDummy::new();
        let backend2 = OcGroupDummy::new();
        setup.group_manager.use_backend(backend1);
        setup.group_manager.use_backend(backend2);

        let group1 = generate_unique_id();
        let group2 = generate_unique_id();
        setup.group_manager.create_group(Some(&group1));

        // Grupos se deben agregar al primer backend registrado
        assert_eq!(vec![group1.clone()], setup.group_manager.backends[0].get_groups());
        assert_eq!(Vec::<String>::new(), setup.group_manager.backends[1].get_groups());

        assert_eq!(vec![group1.clone()], setup.group_manager.get_groups());
        assert!(setup.group_manager.group_exists(&group1));
        assert!(!setup.group_manager.group_exists(&group2));

        setup.group_manager.backends[0].create_group(&group2);

        let mut expected_groups = vec![group1.clone(), group2.clone()];
        expected_groups.sort();
        
        let mut actual_groups = setup.group_manager.get_groups();
        actual_groups.sort();
        
        assert_eq!(expected_groups, actual_groups);
        assert!(setup.group_manager.group_exists(&group1));
        assert!(setup.group_manager.group_exists(&group2));

        let user1 = generate_unique_id();
        let user2 = generate_unique_id();

        setup.user_manager.backends[0].create_user(&user1, "");
        setup.user_manager.backends[0].create_user(&user2, "");

        assert!(!setup.group_manager.in_group(&user1, &group1));
        assert!(!setup.group_manager.in_group(&user2, &group1));

        assert!(setup.group_manager.add_to_group(&user1, &group1));

        assert!(setup.group_manager.in_group(&user1, &group1));
        assert!(!setup.group_manager.in_group(&user2, &group1));
        assert!(!setup.group_manager.backends[1].in_group(&user1, &group1));

        setup.group_manager.add_to_group(&user1, &group1);

        assert_eq!(vec![user1.clone()], setup.group_manager.users_in_group(&group1));

        assert_eq!(vec![group1.clone()], setup.group_manager.get_user_groups(&user1));
        assert_eq!(Vec::<String>::new(), setup.group_manager.get_user_groups(&user2));

        setup.group_manager.delete_group(&group1);
        assert_eq!(Vec::<String>::new(), setup.group_manager.get_user_groups(&user1));
        assert_eq!(Vec::<String>::new(), setup.group_manager.users_in_group(&group1));
        assert!(!setup.group_manager.in_group(&user1, &group1));
    }
}