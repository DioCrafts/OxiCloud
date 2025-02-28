// tests/user/manager.rs

use crate::oc::user::{Backend, Manager, User, BackendActions};
use mockall::{mock, predicate::*};
use std::rc::Rc;

mock! {
    UserBackend {}
    impl Backend for UserBackend {
        fn user_exists(&self, uid: &str) -> bool;
        fn check_password(&self, uid: &str, password: &str) -> bool;
        fn get_users(&self, search: &str, limit: Option<usize>, offset: Option<usize>) -> Vec<String>;
        fn create_user(&self, uid: &str, password: &str) -> bool;
        fn implements_actions(&self, actions: u32) -> bool;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oc::user::OC_USER_BACKEND_CHECK_PASSWORD;

    #[test]
    fn test_user_exists_single_backend_exists() {
        let mut backend = MockUserBackend::new();
        backend.expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| true);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend));

        assert!(manager.user_exists("foo"));
    }

    #[test]
    fn test_user_exists_single_backend_not_exists() {
        let mut backend = MockUserBackend::new();
        backend.expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| false);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend));

        assert!(!manager.user_exists("foo"));
    }

    #[test]
    fn test_user_exists_no_backends() {
        let manager = Manager::new();
        assert!(!manager.user_exists("foo"));
    }

    #[test]
    fn test_user_exists_two_backends_second_exists() {
        let mut backend1 = MockUserBackend::new();
        backend1.expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| false);

        let mut backend2 = MockUserBackend::new();
        backend2.expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| true);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend1));
        manager.register_backend(Rc::new(backend2));

        assert!(manager.user_exists("foo"));
    }

    #[test]
    fn test_user_exists_two_backends_first_exists() {
        let mut backend1 = MockUserBackend::new();
        backend1.expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| true);

        let mut backend2 = MockUserBackend::new();
        // Should never be called
        
        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend1));
        manager.register_backend(Rc::new(backend2));

        assert!(manager.user_exists("foo"));
    }

    #[test]
    fn test_check_password() {
        let mut backend = MockUserBackend::new();
        backend.expect_check_password()
            .with(eq("foo"), eq("bar"))
            .times(1)
            .returning(|_, _| true);

        backend.expect_implements_actions()
            .returning(move |actions| actions == OC_USER_BACKEND_CHECK_PASSWORD);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend));

        let user = manager.check_password("foo", "bar");
        assert!(user.is_some());
    }

    #[test]
    fn test_check_password_not_supported() {
        let mut backend = MockUserBackend::new();
        // check_password should never be called
        
        backend.expect_implements_actions()
            .returning(|_| false);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend));

        assert!(manager.check_password("foo", "bar").is_none());
    }

    #[test]
    fn test_get_one_backend_exists() {
        let mut backend = MockUserBackend::new();
        backend.expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| true);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend));

        let user = manager.get("foo");
        assert!(user.is_some());
        assert_eq!("foo", user.unwrap().get_uid());
    }

    #[test]
    fn test_get_one_backend_not_exists() {
        let mut backend = MockUserBackend::new();
        backend.expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| false);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend));

        let user = manager.get("foo");
        assert!(user.is_none());
    }

    #[test]
    fn test_search_one_backend() {
        let mut backend = MockUserBackend::new();
        backend.expect_get_users()
            .with(eq("fo"), eq(None), eq(None))
            .times(1)
            .returning(|_, _, _| vec!["foo".to_string(), "afoo".to_string()]);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend));

        let result = manager.search("fo", None, None);
        assert_eq!(2, result.len());
        assert_eq!("afoo", result[0].get_uid());
        assert_eq!("foo", result[1].get_uid());
    }

    #[test]
    fn test_search_two_backend_limit_offset() {
        let mut backend1 = MockUserBackend::new();
        backend1.expect_get_users()
            .with(eq("fo"), eq(Some(3)), eq(Some(1)))
            .times(1)
            .returning(|_, _, _| vec!["foo1".to_string(), "foo2".to_string()]);

        let mut backend2 = MockUserBackend::new();
        backend2.expect_get_users()
            .with(eq("fo"), eq(Some(1)), eq(Some(0)))
            .times(1)
            .returning(|_, _, _| vec!["foo3".to_string()]);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend1));
        manager.register_backend(Rc::new(backend2));

        let result = manager.search("fo", Some(3), Some(1));
        assert_eq!(3, result.len());
        assert_eq!("foo1", result[0].get_uid());
        assert_eq!("foo2", result[1].get_uid());
        assert_eq!("foo3", result[2].get_uid());
    }

    #[test]
    fn test_create_user_single_backend_not_exists() {
        let mut backend = MockUserBackend::new();
        backend.expect_implements_actions()
            .returning(|_| true);

        backend.expect_create_user()
            .with(eq("foo"), eq("bar"))
            .times(1)
            .returning(|_, _| true);

        backend.expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| false);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend));

        let user = manager.create_user("foo", "bar");
        assert!(user.is_some());
        assert_eq!("foo", user.unwrap().get_uid());
    }

    #[test]
    #[should_panic]
    fn test_create_user_single_backend_exists() {
        let mut backend = MockUserBackend::new();
        backend.expect_implements_actions()
            .returning(|_| true);

        // create_user should never be called
        
        backend.expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| true);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend));

        let _ = manager.create_user("foo", "bar");
    }

    #[test]
    fn test_create_user_single_backend_not_supported() {
        let mut backend = MockUserBackend::new();
        backend.expect_implements_actions()
            .returning(|_| false);

        // create_user should never be called
        
        backend.expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| false);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend));

        let user = manager.create_user("foo", "bar");
        assert!(user.is_none());
    }

    #[test]
    fn test_create_user_no_backends() {
        let manager = Manager::new();
        let user = manager.create_user("foo", "bar");
        assert!(user.is_none());
    }

    #[test]
    #[should_panic]
    fn test_create_user_two_backend_exists() {
        let mut backend1 = MockUserBackend::new();
        backend1.expect_implements_actions()
            .returning(|_| true);

        // create_user should never be called
        
        backend1.expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| false);

        let mut backend2 = MockUserBackend::new();
        backend2.expect_implements_actions()
            .returning(|_| true);

        // create_user should never be called
        
        backend2.expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| true);

        let mut manager = Manager::new();
        manager.register_backend(Rc::new(backend1));
        manager.register_backend(Rc::new(backend2));

        let _ = manager.create_user("foo", "bar");
    }
}