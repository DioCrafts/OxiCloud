// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::files::cache::Cache;
use crate::files::errors::{NotPermittedError, NotFoundError};
use crate::files::mount::Manager;
use crate::files::node::Root as NodeRoot;
use crate::files::node::File;
use crate::files::storage::Storage;
use crate::files::view::View;
use crate::user::User;
use crate::user::dummy::DummyBackend;
use anyhow::Result;
use async_trait::async_trait;
use mockall::predicate::*;
use mockall::mock;
use std::path::Path;

mock! {
    StorageMock {}
    
    #[async_trait]
    impl Storage for StorageMock {
        // Implement required methods for the Storage trait
    }
}

mock! {
    ViewMock {}
    
    #[async_trait]
    impl View for ViewMock {
        async fn get_file_info(&self, path: &str) -> Result<Option<FileInfo>>;
        async fn is_dir(&self, path: &str) -> Result<bool>;
        async fn file_exists(&self, path: &str) -> Result<bool>;
    }
}

struct FileInfo {
    fileid: i64,
    path: String,
    name: String,
    mimetype: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    struct RootTest {
        user: User,
    }

    impl RootTest {
        fn new() -> Self {
            Self {
                user: User::new("", Box::new(DummyBackend::new())),
            }
        }
    }

    #[test]
    async fn test_get() {
        let test = RootTest::new();
        let manager = Manager::new();
        let mut view = ViewMock::new();
        let root = NodeRoot::new(manager, Box::new(view.clone()), test.user);
        
        let file_info = FileInfo {
            fileid: 10,
            path: "bar/foo".to_string(),
            name: "foo".to_string(),
            mimetype: "text/plain".to_string(),
        };

        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(move |_| Ok(Some(file_info.clone())));

        view.expect_is_dir()
            .with(eq("/bar/foo"))
            .returning(|_| Ok(false));

        view.expect_file_exists()
            .with(eq("/bar/foo"))
            .returning(|_| Ok(true));

        let storage = Box::new(StorageMock::new());
        root.mount(storage, "").await;
        
        let node = root.get("/bar/foo").await.unwrap();
        assert_eq!(10, node.get_id());
        assert!(node.as_any().downcast_ref::<File>().is_some());
    }

    #[test]
    async fn test_get_not_found() {
        let test = RootTest::new();
        let manager = Manager::new();
        let mut view = ViewMock::new();
        let root = NodeRoot::new(manager, Box::new(view.clone()), test.user);

        view.expect_file_exists()
            .with(eq("/bar/foo"))
            .returning(|_| Ok(false));

        let storage = Box::new(StorageMock::new());
        root.mount(storage, "").await;
        
        let result = root.get("/bar/foo").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err().downcast::<NotFoundError>(), Ok(_)));
    }

    #[test]
    async fn test_get_invalid_path() {
        let test = RootTest::new();
        let manager = Manager::new();
        let view = ViewMock::new();
        let root = NodeRoot::new(manager, Box::new(view), test.user);

        let result = root.get("/../foo").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err().downcast::<NotPermittedError>(), Ok(_)));
    }

    #[test]
    async fn test_get_no_storages() {
        let test = RootTest::new();
        let manager = Manager::new();
        let view = ViewMock::new();
        let root = NodeRoot::new(manager, Box::new(view), test.user);

        let result = root.get("/bar/foo").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err().downcast::<NotFoundError>(), Ok(_)));
    }
}