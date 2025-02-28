// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use mockall::{mock, predicate::*};
use std::path::PathBuf;

use crate::hooks::public_emitter::PublicEmitter;
use crate::user::{OcUserBackend, OcUserBackendGetDisplayName, OcUserBackendGetHome, OcUserBackendSetDisplayName, OcUserBackendSetPassword};
use crate::user::User as OcUser;
use crate::config::OcConfig;
use crate::OC;

mock! {
    UserBackend {}

    impl OcUserBackend for UserBackend {
        fn get_display_name(&self, uid: &str) -> Option<String>;
        fn set_display_name(&self, uid: &str, display_name: &str) -> bool;
        fn set_password(&self, uid: &str, password: &str) -> bool;
        fn delete_user(&self, uid: &str) -> bool;
        fn get_home(&self, uid: &str) -> Option<PathBuf>;
        fn implements_actions(&self, actions: u32) -> bool;
    }
}

mock! {
    UserDummy {}

    impl OcUserBackend for UserDummy {
        fn get_display_name(&self, uid: &str) -> Option<String>;
        fn set_display_name(&self, uid: &str, display_name: &str) -> bool;
        fn set_password(&self, uid: &str, password: &str) -> bool;
        fn delete_user(&self, uid: &str) -> bool;
        fn get_home(&self, uid: &str) -> Option<PathBuf>;
        fn implements_actions(&self, actions: u32) -> bool;
    }
}

mock! {
    UserDatabase {}

    impl OcUserBackend for UserDatabase {
        fn get_display_name(&self, uid: &str) -> Option<String>;
        fn set_display_name(&self, uid: &str, display_name: &str) -> bool;
        fn set_password(&self, uid: &str, password: &str) -> bool;
        fn delete_user(&self, uid: &str) -> bool;
        fn get_home(&self, uid: &str) -> Option<PathBuf>;
        fn implements_actions(&self, actions: u32) -> bool;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    #[test]
    fn test_display_name() {
        let mut backend = MockUserBackend::new();
        
        backend
            .expect_get_display_name()
            .with(eq("foo"))
            .times(1)
            .returning(|_| Some("Foo".to_string()));
            
        backend
            .expect_implements_actions()
            .with(eq(OcUserBackendGetDisplayName))
            .returning(|_| true);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        assert_eq!("Foo", user.get_display_name());
    }
    
    #[test]
    fn test_display_name_not_supported() {
        let mut backend = MockUserBackend::new();
        
        backend
            .expect_implements_actions()
            .with(eq(OcUserBackendGetDisplayName))
            .returning(|_| false);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        assert_eq!("foo", user.get_display_name());
    }
    
    #[test]
    fn test_set_password() {
        let mut backend = MockUserDummy::new();
        
        backend
            .expect_set_password()
            .with(eq("foo"), eq("bar"))
            .times(1)
            .returning(|_, _| true);
            
        backend
            .expect_implements_actions()
            .returning(move |actions| actions == OcUserBackendSetPassword);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        assert!(user.set_password("bar", ""));
    }
    
    #[test]
    fn test_set_password_not_supported() {
        let mut backend = MockUserDummy::new();
        
        backend
            .expect_implements_actions()
            .returning(|_| false);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        assert!(!user.set_password("bar", ""));
    }
    
    #[test]
    fn test_delete() {
        let mut backend = MockUserDummy::new();
        
        backend
            .expect_delete_user()
            .with(eq("foo"))
            .times(1)
            .returning(|_| true);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        assert!(user.delete());
    }
    
    #[test]
    fn test_get_home() {
        let mut backend = MockUserDummy::new();
        
        backend
            .expect_get_home()
            .with(eq("foo"))
            .times(1)
            .returning(|_| Some(PathBuf::from("/home/foo")));
            
        backend
            .expect_implements_actions()
            .returning(move |actions| actions == OcUserBackendGetHome);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        assert_eq!(PathBuf::from("/home/foo"), user.get_home());
    }
    
    #[test]
    fn test_get_home_not_supported() {
        let mut backend = MockUserDummy::new();
        
        backend
            .expect_implements_actions()
            .returning(|_| false);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        let expected_path = PathBuf::from(format!("{}/foo", 
            OcConfig::get_value("datadirectory", format!("{}/data", OC::server_root()))));
        assert_eq!(expected_path, user.get_home());
    }
    
    #[test]
    fn test_can_change_password() {
        let mut backend = MockUserDummy::new();
        
        backend
            .expect_implements_actions()
            .returning(move |actions| actions == OcUserBackendSetPassword);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        assert!(user.can_change_password());
    }
    
    #[test]
    fn test_can_change_password_not_supported() {
        let mut backend = MockUserDummy::new();
        
        backend
            .expect_implements_actions()
            .returning(|_| false);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        assert!(!user.can_change_password());
    }
    
    #[test]
    fn test_can_change_display_name() {
        let mut backend = MockUserDummy::new();
        
        backend
            .expect_implements_actions()
            .returning(move |actions| actions == OcUserBackendSetDisplayName);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        assert!(user.can_change_display_name());
    }
    
    #[test]
    fn test_can_change_display_name_not_supported() {
        let mut backend = MockUserDummy::new();
        
        backend
            .expect_implements_actions()
            .returning(|_| false);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        assert!(!user.can_change_display_name());
    }
    
    #[test]
    fn test_set_display_name_supported() {
        let mut backend = MockUserDatabase::new();
        
        backend
            .expect_implements_actions()
            .returning(move |actions| actions == OcUserBackendSetDisplayName);
            
        backend
            .expect_set_display_name()
            .with(eq("foo"), eq("Foo"))
            .times(1)
            .returning(|_, _| true);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        assert!(user.set_display_name("Foo"));
        assert_eq!("Foo", user.get_display_name());
    }
    
    #[test]
    fn test_set_display_name_not_supported() {
        let mut backend = MockUserDatabase::new();
        
        backend
            .expect_implements_actions()
            .returning(|_| false);
            
        let user = OcUser::new("foo".to_string(), Arc::new(backend));
        assert!(!user.set_display_name("Foo"));
        assert_eq!("foo", user.get_display_name());
    }
    
    #[test]
    fn test_set_password_hooks() {
        let mut hooks_called = 0;
        
        let mut backend = MockUserDummy::new();
        backend
            .expect_set_password()
            .times(1)
            .returning(|_, _| true);
            
        backend
            .expect_implements_actions()
            .returning(move |actions| actions == OcUserBackendSetPassword);
            
        let mut emitter = PublicEmitter::new();
            
        let hook = Box::new(move |user: &OcUser, password: &str| {
            hooks_called += 1;
            assert_eq!("foo", user.get_uid());
            assert_eq!("bar", password);
        });
            
        emitter.listen("\\OC\\User", "preSetPassword", hook.clone());
        emitter.listen("\\OC\\User", "postSetPassword", hook);
            
        let user = OcUser::new_with_emitter("foo".to_string(), Arc::new(backend), Arc::new(emitter));
            
        user.set_password("bar", "");
        assert_eq!(2, hooks_called);
    }
    
    #[test]
    fn test_delete_hooks() {
        let mut hooks_called = 0;
        
        let mut backend = MockUserDummy::new();
        backend
            .expect_delete_user()
            .times(1)
            .returning(|_| true);
            
        let mut emitter = PublicEmitter::new();
            
        let hook = Box::new(move |user: &OcUser| {
            hooks_called += 1;
            assert_eq!("foo", user.get_uid());
        });
            
        emitter.listen("\\OC\\User", "preDelete", hook.clone());
        emitter.listen("\\OC\\User", "postDelete", hook);
            
        let user = OcUser::new_with_emitter("foo".to_string(), Arc::new(backend), Arc::new(emitter));
        assert!(user.delete());
        assert_eq!(2, hooks_called);
    }
}