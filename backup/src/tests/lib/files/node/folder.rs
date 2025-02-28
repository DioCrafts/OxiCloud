extern crate async_trait;
extern crate mockall;
extern crate pretty_assertions;

use mockall::predicate::*;
use mockall::*;
use async_trait::async_trait;
use std::sync::Arc;
use std::path::Path;
use std::collections::HashMap;

// Mock types
#[derive(Debug)]
struct User {
    id: String,
    backend: Arc<dyn UserBackend>,
}

#[async_trait]
trait UserBackend {
    async fn get_display_name(&self, user_id: &str) -> Option<String>;
}

struct DummyUserBackend;

#[async_trait]
impl UserBackend for DummyUserBackend {
    async fn get_display_name(&self, _user_id: &str) -> Option<String> {
        None
    }
}

const PERMISSION_ALL: i32 = 31;
const PERMISSION_READ: i32 = 1;

#[async_trait]
trait NodeTrait: Send + Sync {
    fn get_path(&self) -> String;
    fn get_internal_path(&self) -> String;
    fn get_name(&self) -> String;
    async fn delete(&self) -> Result<(), NodeError>;
}

#[derive(Debug)]
enum NodeError {
    NotFound,
    NotPermitted,
    InvalidPath,
    AlreadyExists,
    Unknown(String),
}

#[derive(Debug)]
struct Node {
    root: Arc<Root>,
    view: Arc<dyn View>,
    path: String,
}

impl Node {
    fn new(root: Arc<Root>, view: Arc<dyn View>, path: String) -> Self {
        Self { root, view, path }
    }
}

impl NodeTrait for Node {
    fn get_path(&self) -> String {
        self.path.clone()
    }

    fn get_internal_path(&self) -> String {
        let parts: Vec<&str> = self.path.split('/').filter(|p| !p.is_empty()).collect();
        parts.last().map(|s| s.to_string()).unwrap_or_default()
    }

    fn get_name(&self) -> String {
        let parts: Vec<&str> = self.path.split('/').filter(|p| !p.is_empty()).collect();
        parts.last().map(|s| s.to_string()).unwrap_or_default()
    }

    async fn delete(&self) -> Result<(), NodeError> {
        unimplemented!()
    }
}

#[derive(Debug)]
struct Folder {
    root: Arc<Root>,
    view: Arc<dyn View>,
    path: String,
}

#[derive(Debug)]
struct File {
    root: Arc<Root>,
    view: Arc<dyn View>,
    path: String,
}

#[derive(Debug)]
struct NonExistingFolder {
    root: Arc<Root>,
    view: Arc<dyn View>,
    path: String,
}

impl Folder {
    fn new(root: Arc<Root>, view: Arc<dyn View>, path: String) -> Self {
        Self { root, view, path }
    }

    async fn delete(&self) -> Result<(), NodeError> {
        let file_info = self.view.get_file_info(&self.path).await?;
        
        if file_info.permissions & PERMISSION_ALL != PERMISSION_ALL {
            return Err(NodeError::NotPermitted);
        }
        
        self.root.emit("\\OC\\Files".to_string(), "preDelete".to_string(), self).await;
        
        if !self.view.rmdir(&self.path).await? {
            return Err(NodeError::Unknown("Failed to delete folder".to_string()));
        }
        
        let non_existing = NonExistingFolder::new(self.root.clone(), self.view.clone(), self.path.clone());
        self.root.emit("\\OC\\Files".to_string(), "postDelete".to_string(), &non_existing).await;
        
        Ok(())
    }

    async fn get_directory_listing(&self) -> Result<Vec<Arc<dyn NodeTrait>>, NodeError> {
        let (storage, internal_path) = self.view.resolve_path(&self.path).await?;
        
        let cache = storage.get_cache();
        if cache.get_status(&internal_path).await != CacheStatus::Complete {
            return Err(NodeError::NotFound);
        }
        
        let contents = cache.get_folder_contents(&internal_path).await?;
        let permissions_cache = storage.get_permissions_cache();
        let dir_permissions = permissions_cache.get_directory_permissions().await;
        
        let mut result = Vec::new();
        
        for entry in &contents {
            if let Some(perms) = dir_permissions.get(&entry.file_id) {
                let path = format!("{}/{}", self.path, entry.name);
                if entry.mimetype == "httpd/unix-directory" {
                    result.push(Arc::new(Folder::new(
                        self.root.clone(),
                        self.view.clone(),
                        path,
                    )) as Arc<dyn NodeTrait>);
                } else {
                    result.push(Arc::new(File::new(
                        self.root.clone(),
                        self.view.clone(),
                        path,
                    )) as Arc<dyn NodeTrait>);
                }
            }
        }
        
        let mounts = self.root.get_mounts_in(&self.path).await;
        
        // Process mounts if needed
        
        Ok(result)
    }

    async fn get(&self, path: &str) -> Result<Arc<dyn NodeTrait>, NodeError> {
        let full_path = format!("{}/{}", self.path, path);
        self.root.get(&full_path).await
    }

    async fn node_exists(&self, path: &str) -> bool {
        let result = self.get(path).await;
        result.is_ok()
    }

    async fn new_folder(&self, name: &str) -> Result<Folder, NodeError> {
        let file_info = self.view.get_file_info(&self.path).await?;
        
        if file_info.permissions & PERMISSION_ALL != PERMISSION_ALL {
            return Err(NodeError::NotPermitted);
        }
        
        let new_path = format!("{}/{}", self.path, name);
        if !self.view.mkdir(&new_path).await? {
            return Err(NodeError::Unknown("Failed to create folder".to_string()));
        }
        
        Ok(Folder::new(self.root.clone(), self.view.clone(), new_path))
    }

    async fn new_file(&self, name: &str) -> Result<File, NodeError> {
        let file_info = self.view.get_file_info(&self.path).await?;
        
        if file_info.permissions & PERMISSION_ALL != PERMISSION_ALL {
            return Err(NodeError::NotPermitted);
        }
        
        let new_path = format!("{}/{}", self.path, name);
        if !self.view.touch(&new_path).await? {
            return Err(NodeError::Unknown("Failed to create file".to_string()));
        }
        
        Ok(File::new(self.root.clone(), self.view.clone(), new_path))
    }

    async fn get_free_space(&self) -> Result<i64, NodeError> {
        Ok(self.view.free_space(&self.path).await?)
    }

    async fn search(&self, query: &str) -> Result<Vec<Arc<dyn NodeTrait>>, NodeError> {
        let search_query = format!("%{}%", query);
        let (storage, internal_path) = self.view.resolve_path(&self.path).await?;
        
        let cache = storage.get_cache();
        let results = cache.search(&search_query).await?;
        
        let mut nodes = Vec::new();
        
        for entry in results {
            let path = format!("{}/{}", self.path, entry.name);
            if entry.mimetype == "httpd/unix-directory" {
                nodes.push(Arc::new(Folder::new(
                    self.root.clone(),
                    self.view.clone(),
                    path
                )) as Arc<dyn NodeTrait>);
            } else {
                nodes.push(Arc::new(File::new(
                    self.root.clone(),
                    self.view.clone(),
                    path
                )) as Arc<dyn NodeTrait>);
            }
        }
        
        // Search in sub-storages
        let mounts = self.root.get_mounts_in(&self.path).await;
        for mount in mounts {
            let mount_point = mount.get_mount_point().await;
            let sub_storage = mount.get_storage().await;
            let sub_cache = sub_storage.get_cache();
            
            let sub_results = sub_cache.search(&search_query).await?;
            
            for entry in sub_results {
                let relative_path = Path::new(&mount_point).join(&entry.path);
                let path = relative_path.to_str().unwrap_or_default().to_string();
                
                if entry.mimetype == "httpd/unix-directory" {
                    nodes.push(Arc::new(Folder::new(
                        self.root.clone(),
                        self.view.clone(),
                        path
                    )) as Arc<dyn NodeTrait>);
                } else {
                    nodes.push(Arc::new(File::new(
                        self.root.clone(),
                        self.view.clone(),
                        path
                    )) as Arc<dyn NodeTrait>);
                }
            }
        }
        
        Ok(nodes)
    }

    fn is_sub_node(&self, node: &dyn NodeTrait) -> bool {
        let node_path = node.get_path();
        let self_path = self.path.clone();
        
        if node_path == self_path {
            return false;
        }
        
        node_path.starts_with(&format!("{}/", self_path))
    }
}

impl File {
    fn new(root: Arc<Root>, view: Arc<dyn View>, path: String) -> Self {
        Self { root, view, path }
    }
}

impl NonExistingFolder {
    fn new(root: Arc<Root>, view: Arc<dyn View>, path: String) -> Self {
        Self { root, view, path }
    }
    
    fn get_internal_path(&self) -> String {
        let parts: Vec<&str> = self.path.split('/').filter(|p| !p.is_empty()).collect();
        parts.last().map(|s| s.to_string()).unwrap_or_default()
    }
    
    fn get_path(&self) -> String {
        self.path.clone()
    }
}

#[derive(Debug)]
struct Root {
    manager: Arc<dyn MountManager>,
    view: Arc<dyn View>,
    user: Arc<User>,
    listeners: HashMap<String, Vec<Box<dyn Fn(&dyn std::any::Any) + Send + Sync>>>,
}

impl Root {
    fn new(manager: Arc<dyn MountManager>, view: Arc<dyn View>, user: Arc<User>) -> Self {
        Self {
            manager,
            view,
            user,
            listeners: HashMap::new(),
        }
    }
    
    fn get_user(&self) -> Arc<User> {
        self.user.clone()
    }
    
    async fn emit<T: std::any::Any>(&self, class: String, event: String, param: &T) {
        let event_key = format!("{}{}", class, event);
        if let Some(listeners) = self.listeners.get(&event_key) {
            for listener in listeners {
                listener(param);
            }
        }
    }
    
    fn listen<F>(&mut self, class: &str, event: &str, listener: F)
    where
        F: Fn(&dyn std::any::Any) + Send + Sync + 'static,
    {
        let event_key = format!("{}{}", class, event);
        let listeners = self.listeners.entry(event_key).or_insert_with(Vec::new);
        listeners.push(Box::new(listener));
    }
    
    async fn get_mounts_in(&self, path: &str) -> Vec<Arc<dyn Mount>> {
        // Implementation would defer to the mount manager
        Vec::new()
    }
    
    async fn get(&self, path: &str) -> Result<Arc<dyn NodeTrait>, NodeError> {
        // Implementation for retrieving a node
        Err(NodeError::NotFound)
    }
}

// Storage related mocks
#[derive(Debug, PartialEq)]
enum CacheStatus {
    Complete,
    Partial,
    Uncached,
}

struct FileEntry {
    file_id: i32,
    path: String,
    name: String,
    size: i64,
    mtime: i64,
    mimetype: String,
}

#[derive(Debug)]
struct FileInfo {
    permissions: i32,
    file_id: Option<i32>,
}

#[async_trait]
trait Cache: Send + Sync {
    async fn get_status(&self, path: &str) -> CacheStatus;
    async fn get_folder_contents(&self, path: &str) -> Result<Vec<FileEntry>, NodeError>;
    async fn search(&self, query: &str) -> Result<Vec<FileEntry>, NodeError>;
}

#[async_trait]
trait PermissionsCache: Send + Sync {
    async fn get_directory_permissions(&self) -> HashMap<i32, i32>;
}

#[async_trait]
trait Storage: Send + Sync {
    fn get_cache(&self) -> Arc<dyn Cache>;
    fn get_permissions_cache(&self) -> Arc<dyn PermissionsCache>;
}

#[async_trait]
trait View: Send + Sync {
    async fn get_file_info(&self, path: &str) -> Result<FileInfo, NodeError>;
    async fn rmdir(&self, path: &str) -> Result<bool, NodeError>;
    async fn mkdir(&self, path: &str) -> Result<bool, NodeError>;
    async fn touch(&self, path: &str) -> Result<bool, NodeError>;
    async fn free_space(&self, path: &str) -> Result<i64, NodeError>;
    async fn resolve_path(&self, path: &str) -> Result<(Arc<dyn Storage>, String), NodeError>;
}

#[async_trait]
trait MountManager: Send + Sync {
    async fn get_mounts_in(&self, path: &str) -> Vec<Arc<dyn Mount>>;
}

#[async_trait]
trait Mount: Send + Sync {
    async fn get_storage(&self) -> Arc<dyn Storage>;
    async fn get_mount_point(&self) -> String;
}

// Mock implementations for testing
mock! {
    pub Storage {}
    
    #[async_trait]
    impl Storage for Storage {
        fn get_cache(&self) -> Arc<dyn Cache>;
        fn get_permissions_cache(&self) -> Arc<dyn PermissionsCache>;
    }
}

mock! {
    pub View {}
    
    #[async_trait]
    impl View for View {
        async fn get_file_info(&self, path: &str) -> Result<FileInfo, NodeError>;
        async fn rmdir(&self, path: &str) -> Result<bool, NodeError>;
        async fn mkdir(&self, path: &str) -> Result<bool, NodeError>;
        async fn touch(&self, path: &str) -> Result<bool, NodeError>;
        async fn free_space(&self, path: &str) -> Result<i64, NodeError>;
        async fn resolve_path(&self, path: &str) -> Result<(Arc<dyn Storage>, String), NodeError>;
    }
}

mock! {
    pub MountManager {}
    
    #[async_trait]
    impl MountManager for MountManager {
        async fn get_mounts_in(&self, path: &str) -> Vec<Arc<dyn Mount>>;
    }
}

mock! {
    pub Root {}
    
    impl Root {
        fn get_user(&self) -> Arc<User>;
        async fn emit<T: std::any::Any>(&self, class: String, event: String, param: &T);
        async fn get_mounts_in(&self, path: &str) -> Vec<Arc<dyn Mount>>;
        async fn get(&self, path: &str) -> Result<Arc<dyn NodeTrait>, NodeError>;
    }
}

mock! {
    pub Cache {}
    
    #[async_trait]
    impl Cache for Cache {
        async fn get_status(&self, path: &str) -> CacheStatus;
        async fn get_folder_contents(&self, path: &str) -> Result<Vec<FileEntry>, NodeError>;
        async fn search(&self, query: &str) -> Result<Vec<FileEntry>, NodeError>;
    }
}

mock! {
    pub PermissionsCache {}
    
    #[async_trait]
    impl PermissionsCache for PermissionsCache {
        async fn get_directory_permissions(&self) -> HashMap<i32, i32>;
    }
}

mock! {
    pub Mount {}
    
    #[async_trait]
    impl Mount for Mount {
        async fn get_storage(&self) -> Arc<dyn Storage>;
        async fn get_mount_point(&self) -> String;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::{self, *};
    use std::sync::Arc;
    
    #[tokio::test]
    async fn test_delete() {
        let mut manager = MockMountManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        
        let user = Arc::new(User {
            id: String::new(),
            backend: Arc::new(DummyUserBackend),
        });
        
        root.expect_get_user()
            .returning(move || user.clone());
            
        root.expect_emit::<Folder>()
            .times(2)
            .returning(|_, _, _| ());
        
        view.expect_get_file_info()
            .returning(|_| {
                Ok(FileInfo {
                    permissions: PERMISSION_ALL,
                    file_id: None,
                })
            });
            
        view.expect_rmdir()
            .with(eq("/bar/foo"))
            .returning(|_| Ok(true));
        
        let root = Arc::new(root);
        let view = Arc::new(view);
        
        let folder = Folder::new(root, view, "/bar/foo".to_string());
        folder.delete().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_delete_not_permitted() {
        let mut manager = MockMountManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        
        let user = Arc::new(User {
            id: String::new(),
            backend: Arc::new(DummyUserBackend),
        });
        
        root.expect_get_user()
            .returning(move || user.clone());
        
        view.expect_get_file_info()
            .returning(|_| {
                Ok(FileInfo {
                    permissions: PERMISSION_READ,
                    file_id: None,
                })
            });
        
        let root = Arc::new(root);
        let view = Arc::new(view);
        
        let folder = Folder::new(root, view, "/bar/foo".to_string());
        let result = folder.delete().await;
        assert!(matches!(result, Err(NodeError::NotPermitted)));
    }
    
    #[tokio::test]
    async fn test_node_exists() {
        let mut manager = MockMountManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        
        let user = Arc::new(User {
            id: String::new(),
            backend: Arc::new(DummyUserBackend),
        });
        
        root.expect_get_user()
            .returning(move || user.clone());
            
        root.expect_get()
            .with(eq("/bar/foo/asd"))
            .returning(|_| {
                Ok(Arc::new(Folder::new(
                    Arc::new(MockRoot::new()),
                    Arc::new(MockView::new()),
                    "/bar/foo/asd".to_string()
                )) as Arc<dyn NodeTrait>)
            });
        
        let root = Arc::new(root);
        let view = Arc::new(view);
        
        let folder = Folder::new(root, view, "/bar/foo".to_string());
        assert!(folder.node_exists("asd").await);
    }
    
    #[tokio::test]
    async fn test_node_exists_not_exists() {
        let mut manager = MockMountManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        
        let user = Arc::new(User {
            id: String::new(),
            backend: Arc::new(DummyUserBackend),
        });
        
        root.expect_get_user()
            .returning(move || user.clone());
            
        root.expect_get()
            .with(eq("/bar/foo/asd"))
            .returning(|_| Err(NodeError::NotFound));
        
        let root = Arc::new(root);
        let view = Arc::new(view);
        
        let folder = Folder::new(root, view, "/bar/foo".to_string());
        assert!(!folder.node_exists("asd").await);
    }
    
    #[tokio::test]
    async fn test_new_folder() {
        let mut manager = MockMountManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        
        let user = Arc::new(User {
            id: String::new(),
            backend: Arc::new(DummyUserBackend),
        });
        
        root.expect_get_user()
            .returning(move || user.clone());
        
        view.expect_get_file_info()
            .returning(|_| {
                Ok(FileInfo {
                    permissions: PERMISSION_ALL,
                    file_id: None,
                })
            });
            
        view.expect_mkdir()
            .with(eq("/bar/foo/asd"))
            .returning(|_| Ok(true));
        
        let root = Arc::new(root);
        let view = Arc::new(view);
        
        let folder = Folder::new(root.clone(), view.clone(), "/bar/foo".to_string());
        let result = folder.new_folder("asd").await.unwrap();
        
        // In a real test we would compare the result with an expected folder
    }
    
    #[tokio::test]
    async fn test_new_folder_not_permitted() {
        let mut manager = MockMountManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        
        let user = Arc::new(User {
            id: String::new(),
            backend: Arc::new(DummyUserBackend),
        });
        
        root.expect_get_user()
            .returning(move || user.clone());
        
        view.expect_get_file_info()
            .returning(|_| {
                Ok(FileInfo {
                    permissions: PERMISSION_READ,
                    file_id: None,
                })
            });
        
        let root = Arc::new(root);
        let view = Arc::new(view);
        
        let folder = Folder::new(root, view, "/bar/foo".to_string());
        let result = folder.new_folder("asd").await;
        assert!(matches!(result, Err(NodeError::NotPermitted)));
    }
    
    #[tokio::test]
    async fn test_is_sub_node() {
        let node = Node::new(
            Arc::new(MockRoot::new()),
            Arc::new(MockView::new()),
            "/foo/bar".to_string()
        );
        
        let folder = Folder::new(
            Arc::new(MockRoot::new()),
            Arc::new(MockView::new()),
            "/foo".to_string()
        );
        
        assert!(folder.is_sub_node(&node));
        
        let not_sub_node = Node::new(
            Arc::new(MockRoot::new()),
            Arc::new(MockView::new()),
            "/foobar".to_string()
        );
        
        assert!(!folder.is_sub_node(&not_sub_node));
    }
}