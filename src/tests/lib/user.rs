/**
 * Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use mockall::{mock, predicate::*};
use async_trait::async_trait;

mod test {
    use super::*;
    use crate::oc::hooks::PublicEmitter;
    use crate::oc::user::{Manager, UserBackend, UserBackendActions, User as OCUser};

    mock! {
        UserDummy {}
        
        #[async_trait]
        impl UserBackend for UserDummy {
            async fn check_password(&self, uid: &str, password: &str) -> Option<String>;
            fn implements_actions(&self, actions: u32) -> bool;
            async fn create_user(&self, uid: &str, password: &str) -> Result<OCUser, String>;
            async fn delete_user(&self, uid: &str) -> Result<bool, String>;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test_case::test_case;
        use tokio::test;

        const USER_BACKEND_CHECK_PASSWORD: u32 = 1;
        const USER_BACKEND_CREATE_USER: u32 = 2;

        struct UserTest {
            backend: MockUserDummy,
        }

        impl UserTest {
            async fn setup() -> Self {
                let mut backend = MockUserDummy::new();
                let manager = Manager::get_instance().await;
                manager.register_backend(Box::new(backend.clone())).await;
                
                Self { backend }
            }
        }

        #[test]
        async fn test_check_password() {
            let mut test = UserTest::setup().await;
            
            test.backend
                .expect_check_password()
                .with(eq("foo"), eq("bar"))
                .times(1)
                .returning(|_, _| Some("foo".to_string()));

            test.backend
                .expect_implements_actions()
                .returning(|actions| actions == USER_BACKEND_CHECK_PASSWORD);

            let uid = OCUser::check_password("foo", "bar").await;
            assert_eq!(uid, Some("foo".to_string()));
        }

        #[test]
        async fn test_delete_user() {
            let test = UserTest::setup().await;
            
            // Attempt to delete non-existent user
            let fail = OCUser::delete_user("victim").await;
            assert_eq!(fail, Ok(false));
            
            // Create and then delete user
            let success = OCUser::create_user("victim", "password").await;
            assert!(success.is_ok());
            
            let success = OCUser::delete_user("victim").await;
            assert_eq!(success, Ok(true));
        }

        #[test]
        async fn test_create_user() {
            let mut test = UserTest::setup().await;
            
            test.backend
                .expect_implements_actions()
                .returning(|actions| actions == USER_BACKEND_CREATE_USER);
            
            let user = OCUser::create_user("newuser", "newpassword").await;
            assert!(user.is_ok());
            let user = user.unwrap();
            assert_eq!(user.get_uid(), "newuser");
        }
    }
}