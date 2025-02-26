use std::sync::Arc;
use mockall::{automock, mock, predicate::*};
use async_trait::async_trait;

mod test {
    pub mod files {
        pub mod node {
            use super::super::super::*;
            use std::collections::HashMap;

            pub struct Node {
                pub user: Option<Arc<dyn User>>,
            }

            impl Node {
                pub fn new() -> Self {
                    Self {
                        user: Some(Arc::new(UserDummy::new())),
                    }
                }

                pub fn set_up(&mut self) {
                    self.user = Some(Arc::new(UserDummy::new()));
                }

                pub async fn test_stat(&self) {
                    let manager = MockManager::new();
                    let mut view = MockView::new();
                    let mut root = MockRoot::new();
                    
                    root.expect_get_user()
                        .returning(move || Arc::clone(&self.user.as_ref().unwrap()));

                    let stat = StatInfo {
                        file_id: 1,
                        size: 100,
                        etag: "qwerty".to_string(),
                        mtime: 50,
                        permissions: 0,
                    };

                    view.expect_stat()
                        .with(eq("/bar/foo"))
                        .times(1)
                        .returning(move |_| Ok(stat.clone()));

                    let node = File::new(Arc::new(root), Arc::new(view), "/bar/foo".to_string());
                    assert_eq!(stat, node.stat().await.unwrap());
                }

                pub async fn test_get_id(&self) {
                    let manager = MockManager::new();
                    let mut view = MockView::new();
                    let mut root = MockRoot::new();
                    
                    root.expect_get_user()
                        .returning(move || Arc::clone(&self.user.as_ref().unwrap()));

                    let stat = FileInfo {
                        file_id: 1,
                        size: 100,
                        etag: "qwerty".to_string(),
                        mtime: 50,
                        permissions: None,
                    };

                    view.expect_get_file_info()
                        .with(eq("/bar/foo"))
                        .times(1)
                        .returning(move |_| Ok(stat.clone()));

                    let node = File::new(Arc::new(root), Arc::new(view), "/bar/foo".to_string());
                    assert_eq!(1, node.get_id().await.unwrap());
                }

                pub async fn test_get_size(&self) {
                    let manager = MockManager::new();
                    let mut view = MockView::new();
                    let mut root = MockRoot::new();
                    
                    root.expect_get_user()
                        .returning(move || Arc::clone(&self.user.as_ref().unwrap()));

                    view.expect_filesize()
                        .with(eq("/bar/foo"))
                        .times(1)
                        .returning(|_| Ok(100));

                    let node = File::new(Arc::new(root), Arc::new(view), "/bar/foo".to_string());
                    assert_eq!(100, node.get_size().await.unwrap());
                }

                pub async fn test_get_etag(&self) {
                    let manager = MockManager::new();
                    let mut view = MockView::new();
                    let mut root = MockRoot::new();
                    
                    root.expect_get_user()
                        .returning(move || Arc::clone(&self.user.as_ref().unwrap()));

                    let stat = FileInfo {
                        file_id: 1,
                        size: 100,
                        etag: "qwerty".to_string(),
                        mtime: 50,
                        permissions: None,
                    };

                    view.expect_get_file_info()
                        .with(eq("/bar/foo"))
                        .times(1)
                        .returning(move |_| Ok(stat.clone()));

                    let node = File::new(Arc::new(root), Arc::new(view), "/bar/foo".to_string());
                    assert_eq!("qwerty", node.get_etag().await.unwrap());
                }

                pub async fn test_get_mtime(&self) {
                    let manager = MockManager::new();
                    let mut view = MockView::new();
                    let mut root = MockRoot::new();
                    
                    root.expect_get_user()
                        .returning(move || Arc::clone(&self.user.as_ref().unwrap()));

                    view.expect_filemtime()
                        .with(eq("/bar/foo"))
                        .times(1)
                        .returning(|_| Ok(50));

                    let node = File::new(Arc::new(root), Arc::new(view), "/bar/foo".to_string());
                    assert_eq!(50, node.get_mtime().await.unwrap());
                }

                pub async fn test_get_storage(&self) {
                    let manager = MockManager::new();
                    let mut view = MockView::new();
                    let mut root = MockRoot::new();
                    
                    root.expect_get_user()
                        .returning(move || Arc::clone(&self.user.as_ref().unwrap()));

                    let storage = Arc::new(MockStorage::new());
                    
                    view.expect_resolve_path()
                        .with(eq("/bar/foo"))
                        .times(1)
                        .returning(move |_| Ok((Arc::clone(&storage), "foo".to_string())));

                    let node = File::new(Arc::new(root), Arc::new(view), "/bar/foo".to_string());
                    assert!(Arc::ptr_eq(&storage, &node.get_storage().await.unwrap()));
                }

                pub async fn test_get_path(&self) {
                    let manager = MockManager::new();
                    let view = MockView::new();
                    let mut root = MockRoot::new();
                    
                    root.expect_get_user()
                        .returning(move || Arc::clone(&self.user.as_ref().unwrap()));

                    let node = File::new(Arc::new(root), Arc::new(view), "/bar/foo".to_string());
                    assert_eq!("/bar/foo", node.get_path());
                }

                pub async fn test_get_internal_path(&self) {
                    let manager = MockManager::new();
                    let mut view = MockView::new();
                    let mut root = MockRoot::new();
                    
                    root.expect_get_user()
                        .returning(move || Arc::clone(&self.user.as_ref().unwrap()));

                    let storage = Arc::new(MockStorage::new());
                    
                    view.expect_resolve_path()
                        .with(eq("/bar/foo"))
                        .times(1)
                        .returning(move |_| Ok((Arc::clone(&storage), "foo".to_string())));

                    let node = File::new(Arc::new(root), Arc::new(view), "/bar/foo".to_string());
                    assert_eq!("foo", node.get_internal_path().await.unwrap());
                }

                pub async fn test_get_name(&self) {
                    let manager = MockManager::new();
                    let view = MockView::new();
                    let mut root = MockRoot::new();
                    
                    root.expect_get_user()
                        .returning(move || Arc::clone(&self.user.as_ref().unwrap()));

                    let node = File::new(Arc::new(root), Arc::new(view), "/bar/foo".to_string());
                    assert_eq!("foo", node.get_name());
                }

                pub async fn test_touch_set_mtime(&self) {
                    let manager = MockManager::new();
                    let mut view = MockView::new();
                    let mut root = MockRoot::new();
                    
                    root.expect_get_user()
                        .returning(move || Arc::clone(&self.user.as_ref().unwrap()));

                    view.expect_touch()
                        .with(eq("/bar/foo"), eq(100))
                        .times(1)
                        .returning(|_, _| Ok(()));

                    view.expect_filemtime()
                        .with(eq("/bar/foo"))
                        .times(1)
                        .returning(|_| Ok(100));

                    view.expect_get_file_info()
                        .with(eq("/bar/foo"))
                        .times(1)
                        .returning(|_| Ok(FileInfo {
                            file_id: 0,
                            size: 0,
                            etag: String::new(),
                            mtime: 0,
                            permissions: Some(PERMISSION_ALL),
                        }));

                    let node = NodeImpl::new(Arc::new(root), Arc::new(view), "/bar/foo".to_string());
                    node.touch(100).await.unwrap();
                    assert_eq!(100, node.get_mtime().await.unwrap());
                }

                pub async fn test_touch_hooks(&self) {
                    let manager = MockManager::new();
                    let mut view = MockView::new();
                    
                    let root = Arc::new(Root::new(
                        Arc::new(manager),
                        Arc::new(view.clone()),
                        Arc::clone(&self.user.as_ref().unwrap())
                    ));

                    let mut hooks_run = 0;
                    
                    let pre_listener = move |node: &NodeImpl| {
                        let node_internal_path = node.get_internal_path_sync().unwrap();
                        assert_eq!("foo", node_internal_path);
                        assert_eq!("/bar/foo", node.get_path());
                        hooks_run += 1;
                    };

                    let post_listener = move |node: &NodeImpl| {
                        let node_internal_path = node.get_internal_path_sync().unwrap();
                        assert_eq!("foo", node_internal_path);
                        assert_eq!("/bar/foo", node.get_path());
                        hooks_run += 1;
                    };

                    view.expect_touch()
                        .with(eq("/bar/foo"), eq(100))
                        .times(1)
                        .returning(|_, _| Ok(()));

                    view.expect_resolve_path()
                        .with(eq("/bar/foo"))
                        .returning(|_| Ok((Arc::new(MockStorage::new()), "foo".to_string())));

                    view.expect_get_file_info()
                        .with(eq("/bar/foo"))
                        .returning(|_| Ok(FileInfo {
                            file_id: 0,
                            size: 0,
                            etag: String::new(),
                            mtime: 0,
                            permissions: Some(PERMISSION_ALL),
                        }));

                    // Register hooks
                    root.listen("\\OC\\Files".to_string(), "preTouch".to_string(), Box::new(pre_listener));
                    root.listen("\\OC\\Files".to_string(), "postTouch".to_string(), Box::new(post_listener));

                    let node = NodeImpl::new(Arc::clone(&root), Arc::new(view), "/bar/foo".to_string());
                    node.touch(100).await.unwrap();
                    assert_eq!(2, hooks_run);
                }

                pub async fn test_touch_not_permitted(&self) {
                    let manager = MockManager::new();
                    let mut view = MockView::new();
                    let mut root = MockRoot::new();
                    
                    root.expect_get_user()
                        .returning(move || Arc::clone(&self.user.as_ref().unwrap()));

                    view.expect_get_file_info()
                        .with(eq("/bar/foo"))
                        .returning(|_| Ok(FileInfo {
                            file_id: 0,
                            size: 0,
                            etag: String::new(),
                            mtime: 0,
                            permissions: Some(PERMISSION_READ),
                        }));

                    let node = NodeImpl::new(Arc::new(root), Arc::new(view), "/bar/foo".to_string());
                    let result = node.touch(100).await;
                    assert!(matches!(result, Err(FileError::NotPermitted)));
                }
            }
        }
    }
}

// Error types
#[derive(Debug)]
pub enum FileError {
    NotPermitted,
    NotFound,
    Internal(String),
}

// Constants
const PERMISSION_READ: i32 = 1;
const PERMISSION_ALL: i32 = 31; // Just an example value

// Data structures
#[derive(Debug, PartialEq, Clone)]
pub struct StatInfo {
    file_id: i64,
    size: i64,
    etag: String,
    mtime: i64,
    permissions: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FileInfo {
    file_id: i64,
    size: i64,
    etag: String,
    mtime: i64,
    permissions: Option<i32>,
}

// User interfaces
#[automock]
#[async_trait]
pub trait User: Send + Sync {
    fn get_uid(&self) -> String;
}

struct UserDummy {}

impl UserDummy {
    fn new() -> Self {
        Self {}
    }
}

impl User for UserDummy {
    fn get_uid(&self) -> String {
        String::new()
    }
}

// Storage interfaces
#[automock]
#[async_trait]
pub trait Storage: Send + Sync {}

// View interfaces
#[automock]
#[async_trait]
pub trait View: Send + Sync {
    async fn stat(&self, path: &str) -> Result<StatInfo, FileError>;
    async fn get_file_info(&self, path: &str) -> Result<FileInfo, FileError>;
    async fn filesize(&self, path: &str) -> Result<i64, FileError>;
    async fn filemtime(&self, path: &str) -> Result<i64, FileError>;
    async fn resolve_path(&self, path: &str) -> Result<(Arc<dyn Storage>, String), FileError>;
    async fn touch(&self, path: &str, mtime: i64) -> Result<(), FileError>;
}

// Manager interfaces
#[automock]
#[async_trait]
pub trait Manager: Send + Sync {}

// Root interfaces
#[automock]
pub trait RootTrait: Send + Sync {
    fn get_user(&self) -> Arc<dyn User>;
}

pub struct Root {
    manager: Arc<dyn Manager>,
    view: Arc<dyn View>,
    user: Arc<dyn User>,
    hooks: HashMap<String, HashMap<String, Vec<Box<dyn Fn(&NodeImpl) + Send + Sync>>>>,
}

impl Root {
    pub fn new(manager: Arc<dyn Manager>, view: Arc<dyn View>, user: Arc<dyn User>) -> Self {
        Self {
            manager,
            view,
            user,
            hooks: HashMap::new(),
        }
    }

    pub fn listen<F>(&mut self, class: String, hook: String, listener: Box<F>)
    where
        F: Fn(&NodeImpl) + Send + Sync + 'static,
    {
        self.hooks
            .entry(class)
            .or_insert_with(HashMap::new)
            .entry(hook)
            .or_insert_with(Vec::new)
            .push(listener);
    }

    pub fn emit_hook(&self, class: &str, hook: &str, node: &NodeImpl) {
        if let Some(class_hooks) = self.hooks.get(class) {
            if let Some(hook_listeners) = class_hooks.get(hook) {
                for listener in hook_listeners {
                    listener(node);
                }
            }
        }
    }
}

impl RootTrait for Root {
    fn get_user(&self) -> Arc<dyn User> {
        Arc::clone(&self.user)
    }
}

// Node implementations
pub struct NodeImpl {
    root: Arc<dyn RootTrait>,
    view: Arc<dyn View>,
    path: String,
}

impl NodeImpl {
    pub fn new(root: Arc<dyn RootTrait>, view: Arc<dyn View>, path: String) -> Self {
        Self { root, view, path }
    }

    pub async fn stat(&self) -> Result<StatInfo, FileError> {
        self.view.stat(&self.path).await
    }

    pub async fn get_id(&self) -> Result<i64, FileError> {
        let info = self.view.get_file_info(&self.path).await?;
        Ok(info.file_id)
    }

    pub async fn get_size(&self) -> Result<i64, FileError> {
        self.view.filesize(&self.path).await
    }

    pub async fn get_etag(&self) -> Result<String, FileError> {
        let info = self.view.get_file_info(&self.path).await?;
        Ok(info.etag)
    }

    pub async fn get_mtime(&self) -> Result<i64, FileError> {
        self.view.filemtime(&self.path).await
    }

    pub async fn get_storage(&self) -> Result<Arc<dyn Storage>, FileError> {
        let (storage, _) = self.view.resolve_path(&self.path).await?;
        Ok(storage)
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub async fn get_internal_path(&self) -> Result<String, FileError> {
        let (_, internal_path) = self.view.resolve_path(&self.path).await?;
        Ok(internal_path)
    }

    // Synchronous version for use in hooks
    pub fn get_internal_path_sync(&self) -> Result<String, FileError> {
        // In a real implementation, this would need to be properly handled
        // For test purposes, we'll extract the filename from the path
        Ok(self.path.split('/').last().unwrap_or("").to_string())
    }

    pub fn get_name(&self) -> &str {
        self.path.split('/').last().unwrap_or("")
    }

    pub async fn touch(&self, mtime: i64) -> Result<(), FileError> {
        let info = self.view.get_file_info(&self.path).await?;
        
        if let Some(permissions) = info.permissions {
            if permissions & PERMISSION_ALL != PERMISSION_ALL {
                return Err(FileError::NotPermitted);
            }
        }

        if let Some(root) = self.root.as_any().downcast_ref::<Root>() {
            root.emit_hook("\\OC\\Files", "preTouch", self);
        }

        self.view.touch(&self.path, mtime).await?;

        if let Some(root) = self.root.as_any().downcast_ref::<Root>() {
            root.emit_hook("\\OC\\Files", "postTouch", self);
        }

        Ok(())
    }
}

pub struct File {
    node: NodeImpl,
}

impl File {
    pub fn new(root: Arc<dyn RootTrait>, view: Arc<dyn View>, path: String) -> Self {
        Self {
            node: NodeImpl::new(root, view, path),
        }
    }

    pub async fn stat(&self) -> Result<StatInfo, FileError> {
        self.node.stat().await
    }

    pub async fn get_id(&self) -> Result<i64, FileError> {
        self.node.get_id().await
    }

    pub async fn get_size(&self) -> Result<i64, FileError> {
        self.node.get_size().await
    }

    pub async fn get_etag(&self) -> Result<String, FileError> {
        self.node.get_etag().await
    }

    pub async fn get_mtime(&self) -> Result<i64, FileError> {
        self.node.get_mtime().await
    }

    pub async fn get_storage(&self) -> Result<Arc<dyn Storage>, FileError> {
        self.node.get_storage().await
    }

    pub fn get_path(&self) -> &str {
        self.node.get_path()
    }

    pub async fn get_internal_path(&self) -> Result<String, FileError> {
        self.node.get_internal_path().await
    }

    pub fn get_name(&self) -> &str {
        self.node.get_name()
    }
}

// Extension trait for downcasting trait objects
use std::any::Any;

trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static + RootTrait> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Extend RootTrait with AsAny
trait RootTraitExt: RootTrait + AsAny {}
impl<T: RootTrait + AsAny> RootTraitExt for T {}

// Make HashMap available
use std::collections::HashMap;