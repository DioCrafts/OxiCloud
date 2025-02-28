/*
 * Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use std::sync::Arc;
use std::collections::HashMap;

use mockall::{automock, mock, predicate::*};
use mockall::predicate::eq;
use rstest::rstest;

use crate::group::backend::GroupDatabase;
use crate::user::{User, Manager as UserManager};

struct TestUser {
    uid: String,
}

impl TestUser {
    fn new(uid: &str) -> Self {
        Self { uid: uid.to_string() }
    }

    fn get_uid(&self) -> &str {
        &self.uid
    }
}

#[automock]
trait UserManagerTrait {
    fn get(&self, uid: &str) -> Option<Arc<TestUser>>;
}

struct TestUserManager {
    users: HashMap<String, Arc<TestUser>>,
}

impl TestUserManager {
    fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("user1".to_string(), Arc::new(TestUser::new("user1")));
        users.insert("user2".to_string(), Arc::new(TestUser::new("user2")));
        users.insert("user3".to_string(), Arc::new(TestUser::new("user3")));
        Self { users }
    }
}

impl UserManagerTrait for TestUserManager {
    fn get(&self, uid: &str) -> Option<Arc<TestUser>> {
        self.users.get(uid).cloned()
    }
}

#[automock]
trait GroupDatabaseTrait {
    fn users_in_group(&self, gid: &str, search: Option<&str>, limit: Option<usize>, offset: Option<usize>) -> Vec<String>;
    fn in_group(&self, uid: &str, gid: &str) -> bool;
    fn implements_actions(&self) -> bool;
    fn add_to_group(&self, uid: &str, gid: &str);
    fn remove_from_group(&self, uid: &str, gid: &str);
    fn delete_group(&self, gid: &str);
}

struct Group {
    gid: String,
    backends: Vec<Box<dyn GroupDatabaseTrait>>,
    user_manager: Arc<dyn UserManagerTrait>,
}

impl Group {
    fn new(gid: &str, backends: Vec<Box<dyn GroupDatabaseTrait>>, user_manager: Arc<dyn UserManagerTrait>) -> Self {
        Self {
            gid: gid.to_string(),
            backends,
            user_manager,
        }
    }

    fn get_users(&self) -> HashMap<String, Arc<TestUser>> {
        let mut users = HashMap::new();
        
        for backend in &self.backends {
            let uids = backend.users_in_group(&self.gid, None, None, None);
            for uid in uids {
                if let Some(user) = self.user_manager.get(&uid) {
                    users.insert(uid, user);
                }
            }
        }
        
        users
    }

    fn in_group(&self, user: &TestUser) -> bool {
        let uid = user.get_uid();
        
        for backend in &self.backends {
            if backend.in_group(uid, &self.gid) {
                return true;
            }
        }
        
        false
    }

    fn add_user(&self, user: &TestUser) {
        let uid = user.get_uid();
        
        for backend in &self.backends {
            if backend.implements_actions() && !backend.in_group(uid, &self.gid) {
                backend.add_to_group(uid, &self.gid);
                break;
            }
        }
    }

    fn remove_user(&self, user: &TestUser) {
        let uid = user.get_uid();
        
        for backend in &self.backends {
            if backend.implements_actions() && backend.in_group(uid, &self.gid) {
                backend.remove_from_group(uid, &self.gid);
            }
        }
    }

    fn search_users(&self, search: &str, limit: Option<usize>, offset: Option<usize>) -> Vec<Arc<TestUser>> {
        let mut users = Vec::new();
        let mut remaining_limit = limit;
        let mut remaining_offset = offset;
        
        for backend in &self.backends {
            let backend_limit = remaining_limit;
            let backend_offset = remaining_offset;
            
            let uids = backend.users_in_group(&self.gid, Some(search), backend_limit, backend_offset);
            
            for uid in uids {
                if let Some(user) = self.user_manager.get(&uid) {
                    users.push(user);
                    
                    if let Some(ref mut limit) = remaining_limit {
                        *limit -= 1;
                        if *limit == 0 {
                            return users;
                        }
                    }
                }
            }
            
            if let Some(ref mut offset) = remaining_offset {
                if let Some(count) = backend_limit {
                    if *offset > count {
                        *offset -= count;
                    } else {
                        *offset = 0;
                    }
                }
            }
        }
        
        users
    }

    fn delete(&self) {
        for backend in &self.backends {
            if backend.implements_actions() {
                backend.delete_group(&self.gid);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    fn get_user_manager() -> Arc<TestUserManager> {
        Arc::new(TestUserManager::new())
    }

    #[test]
    fn test_get_users_single_backend() {
        let mut backend = MockGroupDatabaseTrait::new();
        backend.expect_users_in_group()
            .with(eq("group1"), eq(None), eq(None), eq(None))
            .return_const(vec!["user1".to_string(), "user2".to_string()]);
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend)], user_manager.clone());

        let users = group.get_users();

        assert_eq!(2, users.len());
        assert_eq!("user1", users.get("user1").unwrap().get_uid());
        assert_eq!("user2", users.get("user2").unwrap().get_uid());
    }

    #[test]
    fn test_get_users_multiple_backends() {
        let mut backend1 = MockGroupDatabaseTrait::new();
        backend1.expect_users_in_group()
            .with(eq("group1"), eq(None), eq(None), eq(None))
            .return_const(vec!["user1".to_string(), "user2".to_string()]);

        let mut backend2 = MockGroupDatabaseTrait::new();
        backend2.expect_users_in_group()
            .with(eq("group1"), eq(None), eq(None), eq(None))
            .return_const(vec!["user2".to_string(), "user3".to_string()]);
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend1), Box::new(backend2)], user_manager.clone());

        let users = group.get_users();

        assert_eq!(3, users.len());
        assert_eq!("user1", users.get("user1").unwrap().get_uid());
        assert_eq!("user2", users.get("user2").unwrap().get_uid());
        assert_eq!("user3", users.get("user3").unwrap().get_uid());
    }

    #[test]
    fn test_in_group_single_backend() {
        let mut backend = MockGroupDatabaseTrait::new();
        backend.expect_in_group()
            .with(eq("user1"), eq("group1"))
            .return_const(true);
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend)], user_manager.clone());
        let user = TestUser::new("user1");

        assert!(group.in_group(&user));
    }

    #[test]
    fn test_in_group_multiple_backends() {
        let mut backend1 = MockGroupDatabaseTrait::new();
        backend1.expect_in_group()
            .with(eq("user1"), eq("group1"))
            .return_const(false);

        let mut backend2 = MockGroupDatabaseTrait::new();
        backend2.expect_in_group()
            .with(eq("user1"), eq("group1"))
            .return_const(true);
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend1), Box::new(backend2)], user_manager.clone());
        let user = TestUser::new("user1");

        assert!(group.in_group(&user));
    }

    #[test]
    fn test_add_user() {
        let mut backend = MockGroupDatabaseTrait::new();
        backend.expect_in_group()
            .with(eq("user1"), eq("group1"))
            .return_const(false);
        backend.expect_implements_actions()
            .return_const(true);
        backend.expect_add_to_group()
            .with(eq("user1"), eq("group1"))
            .times(1)
            .return_const(());
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend)], user_manager.clone());
        let user = TestUser::new("user1");

        group.add_user(&user);
    }

    #[test]
    fn test_add_user_already_in_group() {
        let mut backend = MockGroupDatabaseTrait::new();
        backend.expect_in_group()
            .with(eq("user1"), eq("group1"))
            .return_const(true);
        backend.expect_implements_actions()
            .return_const(true);
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend)], user_manager.clone());
        let user = TestUser::new("user1");

        group.add_user(&user);
    }

    #[test]
    fn test_remove_user() {
        let mut backend = MockGroupDatabaseTrait::new();
        backend.expect_in_group()
            .with(eq("user1"), eq("group1"))
            .return_const(true);
        backend.expect_implements_actions()
            .return_const(true);
        backend.expect_remove_from_group()
            .with(eq("user1"), eq("group1"))
            .times(1)
            .return_const(());
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend)], user_manager.clone());
        let user = TestUser::new("user1");

        group.remove_user(&user);
    }

    #[test]
    fn test_remove_user_not_in_group() {
        let mut backend = MockGroupDatabaseTrait::new();
        backend.expect_in_group()
            .with(eq("user1"), eq("group1"))
            .return_const(false);
        backend.expect_implements_actions()
            .return_const(true);
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend)], user_manager.clone());
        let user = TestUser::new("user1");

        group.remove_user(&user);
    }

    #[test]
    fn test_remove_user_multiple_backends() {
        let mut backend1 = MockGroupDatabaseTrait::new();
        backend1.expect_in_group()
            .with(eq("user1"), eq("group1"))
            .return_const(true);
        backend1.expect_implements_actions()
            .return_const(true);
        backend1.expect_remove_from_group()
            .with(eq("user1"), eq("group1"))
            .times(1)
            .return_const(());

        let mut backend2 = MockGroupDatabaseTrait::new();
        backend2.expect_in_group()
            .with(eq("user1"), eq("group1"))
            .return_const(true);
        backend2.expect_implements_actions()
            .return_const(true);
        backend2.expect_remove_from_group()
            .with(eq("user1"), eq("group1"))
            .times(1)
            .return_const(());
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend1), Box::new(backend2)], user_manager.clone());
        let user = TestUser::new("user1");

        group.remove_user(&user);
    }

    #[test]
    fn test_search_users() {
        let mut backend = MockGroupDatabaseTrait::new();
        backend.expect_users_in_group()
            .with(eq("group1"), eq(Some("2")), eq(None), eq(None))
            .return_const(vec!["user2".to_string()]);
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend)], user_manager.clone());

        let users = group.search_users("2", None, None);

        assert_eq!(1, users.len());
        assert_eq!("user2", users[0].get_uid());
    }

    #[test]
    fn test_search_users_multiple_backends() {
        let mut backend1 = MockGroupDatabaseTrait::new();
        backend1.expect_users_in_group()
            .with(eq("group1"), eq(Some("2")), eq(None), eq(None))
            .return_const(vec!["user2".to_string()]);

        let mut backend2 = MockGroupDatabaseTrait::new();
        backend2.expect_users_in_group()
            .with(eq("group1"), eq(Some("2")), eq(None), eq(None))
            .return_const(vec!["user2".to_string()]);
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend1), Box::new(backend2)], user_manager.clone());

        let users = group.search_users("2", None, None);

        assert_eq!(2, users.len());
        assert_eq!("user2", users[0].get_uid());
        assert_eq!("user2", users[1].get_uid());
    }

    #[test]
    fn test_search_users_limit_and_offset() {
        let mut backend = MockGroupDatabaseTrait::new();
        backend.expect_users_in_group()
            .with(eq("group1"), eq(Some("user")), eq(Some(1)), eq(Some(1)))
            .return_const(vec!["user2".to_string()]);
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend)], user_manager.clone());

        let users = group.search_users("user", Some(1), Some(1));

        assert_eq!(1, users.len());
        assert_eq!("user2", users[0].get_uid());
    }

    #[test]
    fn test_search_users_multiple_backends_limit_and_offset() {
        let mut backend1 = MockGroupDatabaseTrait::new();
        backend1.expect_users_in_group()
            .with(eq("group1"), eq(Some("user")), eq(Some(2)), eq(Some(1)))
            .return_const(vec!["user2".to_string()]);

        let mut backend2 = MockGroupDatabaseTrait::new();
        backend2.expect_users_in_group()
            .with(eq("group1"), eq(Some("user")), eq(Some(1)), eq(Some(0)))
            .return_const(vec!["user1".to_string()]);
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend1), Box::new(backend2)], user_manager.clone());

        let users = group.search_users("user", Some(2), Some(1));

        assert_eq!(2, users.len());
        assert_eq!("user2", users[0].get_uid());
        assert_eq!("user1", users[1].get_uid());
    }

    #[test]
    fn test_delete() {
        let mut backend = MockGroupDatabaseTrait::new();
        backend.expect_implements_actions()
            .return_const(true);
        backend.expect_delete_group()
            .with(eq("group1"))
            .times(1)
            .return_const(());
        
        let user_manager = get_user_manager();
        let group = Group::new("group1", vec![Box::new(backend)], user_manager.clone());

        group.delete();
    }
}