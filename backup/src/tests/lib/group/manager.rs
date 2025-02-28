// tests/group/manager.rs

use std::sync::Arc;

use mockall::{automock, predicate::*};
use nextcloud_core::group::{GroupTrait, Group, Manager as GroupManager};
use nextcloud_core::user::{User, Manager as UserManager};

#[automock]
pub trait GroupBackend {
    fn group_exists(&self, gid: &str) -> bool;
    fn create_group(&self, gid: &str) -> bool;
    fn implements_actions(&self) -> bool;
    fn get_groups(&self, search: &str, limit: i32, offset: i32) -> Vec<String>;
    fn get_user_groups(&self, uid: &str) -> Vec<String>;
}

struct OcGroupDatabase;
struct OcGroupDummy {
    groups: Vec<String>,
}

impl OcGroupDummy {
    fn new() -> Self {
        Self { groups: vec![] }
    }

    fn create_group(&mut self, gid: &str) {
        self.groups.push(gid.to_string());
    }
}

impl GroupBackend for OcGroupDummy {
    fn group_exists(&self, gid: &str) -> bool {
        self.groups.contains(&gid.to_string())
    }

    fn create_group(&self, _gid: &str) -> bool {
        true
    }

    fn implements_actions(&self) -> bool {
        true
    }

    fn get_groups(&self, _search: &str, _limit: i32, _offset: i32) -> Vec<String> {
        self.groups.clone()
    }

    fn get_user_groups(&self, _uid: &str) -> Vec<String> {
        self.groups.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn test_get() {
        let mut backend = MockGroupBackend::new();
        backend.expect_group_exists()
            .with(eq("group1"))
            .returning(|_| true);

        let user_manager = Arc::new(MockUserManager::new());
        let mut manager = GroupManager::new(user_manager);
        manager.add_backend(Arc::new(backend));

        let group = manager.get("group1");
        assert!(group.is_some());
        assert_eq!("group1", group.unwrap().get_gid());
    }

    #[test]
    fn test_get_no_backend() {
        let user_manager = Arc::new(MockUserManager::new());
        let manager = GroupManager::new(user_manager);

        let group = manager.get("group1");
        assert!(group.is_none());
    }

    #[test]
    fn test_get_not_exists() {
        let mut backend = MockGroupBackend::new();
        backend.expect_group_exists()
            .with(eq("group1"))
            .returning(|_| false);

        let user_manager = Arc::new(MockUserManager::new());
        let mut manager = GroupManager::new(user_manager);
        manager.add_backend(Arc::new(backend));

        let group = manager.get("group1");
        assert!(group.is_none());
    }

    #[test]
    fn test_get_deleted() {
        let dummy = Arc::new(Mutex::new(OcGroupDummy::new()));
        {
            let mut dummy_lock = dummy.lock().unwrap();
            dummy_lock.create_group("group1");
        }

        let user_manager = Arc::new(MockUserManager::new());
        let mut manager = GroupManager::new(user_manager);
        manager.add_backend(dummy.clone());

        let group = manager.get("group1").unwrap();
        group.delete();
        
        let group = manager.get("group1");
        assert!(group.is_none());
    }

    #[test]
    fn test_get_multiple_backends() {
        let mut backend1 = MockGroupBackend::new();
        backend1.expect_group_exists()
            .with(eq("group1"))
            .returning(|_| false);

        let mut backend2 = MockGroupBackend::new();
        backend2.expect_group_exists()
            .with(eq("group1"))
            .returning(|_| true);

        let user_manager = Arc::new(MockUserManager::new());
        let mut manager = GroupManager::new(user_manager);
        manager.add_backend(Arc::new(backend1));
        manager.add_backend(Arc::new(backend2));

        let group = manager.get("group1");
        assert!(group.is_some());
        assert_eq!("group1", group.unwrap().get_gid());
    }

    #[test]
    fn test_create() {
        let mut backend = MockGroupBackend::new();
        backend.expect_group_exists()
            .with(eq("group1"))
            .returning(|_| false);
        backend.expect_implements_actions()
            .returning(|| true);
        backend.expect_create_group()
            .with(eq("group1"))
            .returning(|_| true);

        let user_manager = Arc::new(MockUserManager::new());
        let mut manager = GroupManager::new(user_manager);
        manager.add_backend(Arc::new(backend));

        let group = manager.create_group("group1");
        assert!(group.is_some());
        assert_eq!("group1", group.unwrap().get_gid());
    }

    #[test]
    fn test_create_exists() {
        let mut backend = MockGroupBackend::new();
        backend.expect_group_exists()
            .with(eq("group1"))
            .returning(|_| true);
        backend.expect_create_group()
            .never();

        let user_manager = Arc::new(MockUserManager::new());
        let mut manager = GroupManager::new(user_manager);
        manager.add_backend(Arc::new(backend));

        let group = manager.create_group("group1");
        assert!(group.is_some());
        assert_eq!("group1", group.unwrap().get_gid());
    }

    #[test]
    fn test_search() {
        let mut backend = MockGroupBackend::new();
        backend.expect_get_groups()
            .with(eq("1"), eq(-1), eq(0))
            .returning(|_, _, _| vec!["group1".to_string()]);

        let user_manager = Arc::new(MockUserManager::new());
        let mut manager = GroupManager::new(user_manager);
        manager.add_backend(Arc::new(backend));

        let groups = manager.search("1", -1, 0);
        assert_eq!(1, groups.len());
        assert_eq!("group1", groups[0].get_gid());
    }

    #[test]
    fn test_search_multiple_backends() {
        let mut backend1 = MockGroupBackend::new();
        backend1.expect_get_groups()
            .with(eq("1"), eq(-1), eq(0))
            .returning(|_, _, _| vec!["group1".to_string()]);

        let mut backend2 = MockGroupBackend::new();
        backend2.expect_get_groups()
            .with(eq("1"), eq(-1), eq(0))
            .returning(|_, _, _| vec!["group12".to_string(), "group1".to_string()]);

        let user_manager = Arc::new(MockUserManager::new());
        let mut manager = GroupManager::new(user_manager);
        manager.add_backend(Arc::new(backend1));
        manager.add_backend(Arc::new(backend2));

        let groups = manager.search("1", -1, 0);
        assert_eq!(2, groups.len());
        assert_eq!("group1", groups[0].get_gid());
        assert_eq!("group12", groups[1].get_gid());
    }

    #[test]
    fn test_search_multiple_backends_limit_and_offset() {
        let mut backend1 = MockGroupBackend::new();
        backend1.expect_get_groups()
            .with(eq("1"), eq(2), eq(1))
            .returning(|_, _, _| vec!["group1".to_string()]);

        let mut backend2 = MockGroupBackend::new();
        backend2.expect_get_groups()
            .with(eq("1"), eq(1), eq(0))
            .returning(|_, _, _| vec!["group12".to_string()]);

        let user_manager = Arc::new(MockUserManager::new());
        let mut manager = GroupManager::new(user_manager);
        manager.add_backend(Arc::new(backend1));
        manager.add_backend(Arc::new(backend2));

        let groups = manager.search("1", 2, 1);
        assert_eq!(2, groups.len());
        assert_eq!("group1", groups[0].get_gid());
        assert_eq!("group12", groups[1].get_gid());
    }

    #[test]
    fn test_get_user_groups() {
        let mut backend = MockGroupBackend::new();
        backend.expect_get_user_groups()
            .with(eq("user1"))
            .returning(|_| vec!["group1".to_string()]);

        let user_manager = Arc::new(MockUserManager::new());
        let mut manager = GroupManager::new(user_manager);
        manager.add_backend(Arc::new(backend));

        let user = User::new("user1", None);
        let groups = manager.get_user_groups(&user);
        assert_eq!(1, groups.len());
        assert_eq!("group1", groups[0].get_gid());
    }

    #[test]
    fn test_get_user_groups_multiple_backends() {
        let mut backend1 = MockGroupBackend::new();
        backend1.expect_get_user_groups()
            .with(eq("user1"))
            .returning(|_| vec!["group1".to_string()]);

        let mut backend2 = MockGroupBackend::new();
        backend2.expect_get_user_groups()
            .with(eq("user1"))
            .returning(|_| vec!["group1".to_string(), "group2".to_string()]);

        let user_manager = Arc::new(MockUserManager::new());
        let mut manager = GroupManager::new(user_manager);
        manager.add_backend(Arc::new(backend1));
        manager.add_backend(Arc::new(backend2));

        let user = User::new("user1", None);
        let groups = manager.get_user_groups(&user);
        assert_eq!(2, groups.len());
        assert_eq!("group1", groups[0].get_gid());
        assert_eq!("group2", groups[1].get_gid());
    }
}

// Mock for UserManager
#[automock]
pub trait UserManager {}