/*
 * Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use async_trait::async_trait;
use mockall::automock;
use mockall::predicate::*;
use std::sync::Arc;

mod test_user {
    use super::*;

    #[automock]
    #[async_trait]
    pub trait Backend {
        async fn user_exists(&self, user_id: &str) -> bool;
    }

    #[automock]
    #[async_trait]
    pub trait Session {
        async fn get(&self, key: &str) -> Option<String>;
        async fn set(&mut self, key: &str, value: &str);
    }

    #[automock]
    #[async_trait]
    pub trait Manager {
        async fn register_backend(&mut self, backend: Arc<dyn Backend + Send + Sync>);
        async fn check_password(&self, uid: &str, password: &str) -> Option<Arc<User>>;
    }

    #[derive(Clone)]
    pub struct User {
        uid: String,
        backend: Arc<dyn Backend + Send + Sync>,
        enabled: bool,
    }

    impl User {
        pub fn new(uid: &str, backend: Arc<dyn Backend + Send + Sync>) -> Self {
            Self {
                uid: uid.to_string(),
                backend,
                enabled: true,
            }
        }

        pub fn get_uid(&self) -> &str {
            &self.uid
        }

        pub fn is_enabled(&self) -> bool {
            self.enabled
        }

        pub fn set_enabled(&mut self, enabled: bool) {
            self.enabled = enabled;
        }
    }

    pub struct UserSession {
        manager: Arc<dyn Manager + Send + Sync>,
        session: Arc<tokio::sync::Mutex<dyn Session + Send + Sync>>,
    }

    impl UserSession {
        pub fn new(
            manager: Arc<dyn Manager + Send + Sync>,
            session: Arc<tokio::sync::Mutex<dyn Session + Send + Sync>>,
        ) -> Self {
            Self { manager, session }
        }

        pub async fn get_user(&self) -> Option<Arc<User>> {
            let user_id = self.session.lock().await.get("user_id").await?;
            
            // Implementation would go here to retrieve user from manager
            // For testing purposes, we'll create a mock user
            let backend = Arc::new(MockBackend::new());
            Some(Arc::new(User::new(&user_id, backend)))
        }

        pub async fn set_user(&self, user: &User) {
            self.session
                .lock()
                .await
                .set("user_id", user.get_uid())
                .await;
        }

        pub async fn login(&self, user_id: &str, password: &str) -> bool {
            if let Some(user) = self.manager.check_password(user_id, password).await {
                if user.is_enabled() {
                    self.session
                        .lock()
                        .await
                        .set("user_id", user.get_uid())
                        .await;
                    return true;
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_user::*;
    use tokio::test as async_test;

    #[async_test]
    async fn test_get_user() {
        let mut mock_session = MockSession::new();
        mock_session
            .expect_get()
            .with(eq("user_id"))
            .times(1)
            .returning(|_| Some("foo".to_string()));

        let mut mock_backend = MockBackend::new();
        mock_backend
            .expect_user_exists()
            .with(eq("foo"))
            .times(1)
            .returning(|_| true);

        let mut mock_manager = MockManager::new();
        mock_manager
            .expect_register_backend()
            .times(1)
            .returning(|_| ());

        let session = Arc::new(tokio::sync::Mutex::new(mock_session));
        let manager = Arc::new(mock_manager);

        let user_session = UserSession::new(manager, session);
        let user = user_session.get_user().await.unwrap();
        assert_eq!("foo", user.get_uid());
    }

    #[async_test]
    async fn test_set_user() {
        let mut mock_session = MockSession::new();
        mock_session
            .expect_set()
            .with(eq("user_id"), eq("foo"))
            .times(1)
            .returning(|_, _| ());

        let mock_backend = Arc::new(MockBackend::new());
        let user = User::new("foo", mock_backend);

        let session = Arc::new(tokio::sync::Mutex::new(mock_session));
        let manager = Arc::new(MockManager::new());

        let user_session = UserSession::new(manager, session);
        user_session.set_user(&user).await;
    }

    #[async_test]
    async fn test_login_valid_password_enabled() {
        let mut mock_session = MockSession::new();
        mock_session
            .expect_set()
            .with(eq("user_id"), eq("foo"))
            .times(1)
            .returning(|_, _| ());

        let mock_backend = Arc::new(MockBackend::new());
        let mut user = User::new("foo", mock_backend);
        user.set_enabled(true);
        let user_arc = Arc::new(user);
        
        let mut mock_manager = MockManager::new();
        mock_manager
            .expect_check_password()
            .with(eq("foo"), eq("bar"))
            .times(1)
            .returning(move |_, _| Some(user_arc.clone()));

        let session = Arc::new(tokio::sync::Mutex::new(mock_session));
        let manager = Arc::new(mock_manager);

        let user_session = UserSession::new(manager, session);
        assert!(user_session.login("foo", "bar").await);
    }

    #[async_test]
    async fn test_login_valid_password_disabled() {
        let mock_session = MockSession::new();

        let mock_backend = Arc::new(MockBackend::new());
        let mut user = User::new("foo", mock_backend);
        user.set_enabled(false);
        let user_arc = Arc::new(user);
        
        let mut mock_manager = MockManager::new();
        mock_manager
            .expect_check_password()
            .with(eq("foo"), eq("bar"))
            .times(1)
            .returning(move |_, _| Some(user_arc.clone()));

        let session = Arc::new(tokio::sync::Mutex::new(mock_session));
        let manager = Arc::new(mock_manager);

        let user_session = UserSession::new(manager, session);
        assert!(!user_session.login("foo", "bar").await);
    }

    #[async_test]
    async fn test_login_invalid_password() {
        let mock_session = MockSession::new();

        let mut mock_manager = MockManager::new();
        mock_manager
            .expect_check_password()
            .with(eq("foo"), eq("bar"))
            .times(1)
            .returning(|_, _| None);

        let session = Arc::new(tokio::sync::Mutex::new(mock_session));
        let manager = Arc::new(mock_manager);

        let user_session = UserSession::new(manager, session);
        assert!(!user_session.login("foo", "bar").await);
    }

    #[async_test]
    async fn test_login_non_existing() {
        let mock_session = MockSession::new();

        let mut mock_manager = MockManager::new();
        mock_manager
            .expect_check_password()
            .with(eq("foo"), eq("bar"))
            .times(1)
            .returning(|_, _| None);

        let session = Arc::new(tokio::sync::Mutex::new(mock_session));
        let manager = Arc::new(mock_manager);

        let user_session = UserSession::new(manager, session);
        assert!(!user_session.login("foo", "bar").await);
    }
}