// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

mod tests {
    pub mod files {
        pub mod node {
            use super::super::super::*;
            
            pub struct IntegrationTests {
                root: Arc<dyn Root>,
                storages: Vec<Arc<dyn Storage>>,
                view: Arc<View>,
            }
            
            #[automock]
            #[async_trait]
            pub trait Root: Send + Sync {
                async fn new_file(&self, path: &str) -> Result<Arc<dyn File>, NodeError>;
                async fn new_folder(&self, path: &str) -> Result<Arc<dyn Folder>, NodeError>;
                async fn node_exists(&self, path: &str) -> bool;
                async fn get_directory_listing(&self) -> Vec<Arc<dyn Node>>;
                async fn mount(&self, storage: Arc<dyn Storage>, path: &str) -> Result<(), MountError>;
            }
            
            #[automock]
            #[async_trait]
            pub trait Node: Send + Sync {
                async fn get_id(&self) -> String;
                async fn get_name(&self) -> String;
                async fn get_internal_path(&self) -> String;
                async fn get_storage(&self) -> Arc<dyn Storage>;
                async fn move_to(&mut self, destination: &str) -> Result<(), NodeError>;
            }
            
            #[automock]
            #[async_trait]
            pub trait File: Node {
                async fn put_content(&mut self, content: &str) -> Result<(), FileError>;
                async fn get_content(&self) -> Result<String, FileError>;
                async fn get_mime_type(&self) -> Result<String, FileError>;
            }
            
            #[automock]
            #[async_trait]
            pub trait Folder: Node {
                async fn get_directory_listing(&self) -> Vec<Arc<dyn Node>>;
                async fn get(&self, path: &str) -> Result<Arc<dyn Node>, NodeError>;
                async fn new_file(&self, path: &str) -> Result<Arc<dyn File>, NodeError>;
            }
            
            #[automock]
            #[async_trait]
            pub trait Storage: Send + Sync {
                async fn get_cache(&self) -> Arc<dyn Cache>;
            }
            
            #[automock]
            #[async_trait]
            pub trait Cache: Send + Sync {
                async fn clear(&self) -> Result<(), CacheError>;
            }
            
            #[derive(Debug)]
            pub enum NodeError {
                NotFound(String),
                NotPermitted(String),
                Other(String),
            }
            
            #[derive(Debug)]
            pub enum FileError {
                ReadError(String),
                WriteError(String),
                Other(String),
            }
            
            #[derive(Debug)]
            pub enum MountError {
                InvalidPath(String),
                StorageError(String),
                Other(String),
            }
            
            #[derive(Debug)]
            pub enum CacheError {
                ClearFailed(String),
                Other(String),
            }
            
            struct RootImpl {
                manager: Arc<Manager>,
                view: Arc<View>,
                user: Arc<User>,
            }
            
            struct View {}
            
            struct Manager {}
            
            struct User {
                uid: String,
            }
            
            struct TemporaryStorage {
                cache: Arc<dyn Cache>,
            }
            
            impl IntegrationTests {
                pub async fn setup() -> Self {
                    Filesystem::init("", "").await;
                    Filesystem::clear_mounts().await;
                    let manager = Filesystem::get_mount_manager().await;
                    
                    Hook::clear("OC_Filesystem").await;
                    
                    Hook::connect("OC_Filesystem", "post_write", "\\OC\\Files\\Cache\\Updater", "write_hook").await;
                    Hook::connect("OC_Filesystem", "post_delete", "\\OC\\Files\\Cache\\Updater", "delete_hook").await;
                    Hook::connect("OC_Filesystem", "post_rename", "\\OC\\Files\\Cache\\Updater", "rename_hook").await;
                    Hook::connect("OC_Filesystem", "post_touch", "\\OC\\Files\\Cache\\Updater", "touch_hook").await;
                    
                    let user_id = format!("user_{}", Uuid::new_v4());
                    let user = Arc::new(User::new(&user_id, Arc::new(UserDummy::new())));
                    User::set_user_id(&user.get_uid()).await;
                    
                    let view = Arc::new(View::new());
                    let root = RootImpl::new(manager.clone(), view.clone(), user.clone());
                    let root: Arc<dyn Root> = Arc::new(root);
                    
                    let storage = Arc::new(TemporaryStorage::new(HashMap::new()));
                    let sub_storage = Arc::new(TemporaryStorage::new(HashMap::new()));
                    
                    let mut storages: Vec<Arc<dyn Storage>> = Vec::new();
                    storages.push(storage.clone());
                    storages.push(sub_storage.clone());
                    
                    root.mount(storage, "/").await.unwrap();
                    root.mount(sub_storage, "/substorage/").await.unwrap();
                    
                    IntegrationTests {
                        root,
                        storages,
                        view,
                    }
                }
                
                pub async fn tear_down(&self) {
                    for storage in &self.storages {
                        storage.get_cache().await.clear().await.unwrap();
                    }
                    Filesystem::clear_mounts().await;
                }
                
                pub async fn test_basic_file(&self) {
                    let mut file = self.root.new_file("/foo.txt").await.unwrap();
                    let dir_listing = self.root.get_directory_listing().await;
                    assert_eq!(2, dir_listing.len());
                    assert!(self.root.node_exists("/foo.txt").await);
                    
                    let id = file.get_id().await;
                    
                    // Check file is the right type
                    let _: Arc<dyn File> = file.clone();
                    
                    file.put_content("qwerty").await.unwrap();
                    assert_eq!("text/plain", file.get_mime_type().await.unwrap());
                    assert_eq!("qwerty", file.get_content().await.unwrap());
                    
                    assert!(!self.root.node_exists("/bar.txt").await);
                    file.move_to("/bar.txt").await.unwrap();
                    assert!(!self.root.node_exists("/foo.txt").await);
                    assert!(self.root.node_exists("/bar.txt").await);
                    assert_eq!("bar.txt", file.get_name().await);
                    assert_eq!("bar.txt", file.get_internal_path().await);
                    
                    file.move_to("/substorage/bar.txt").await.unwrap();
                    assert_ne!(id, file.get_id().await);
                    assert_eq!("qwerty", file.get_content().await.unwrap());
                }
                
                pub async fn test_basic_folder(&self) {
                    let folder = self.root.new_folder("/foo").await.unwrap();
                    assert!(self.root.node_exists("/foo").await);
                    
                    let mut file = folder.new_file("/bar").await.unwrap();
                    assert!(self.root.node_exists("/foo/bar").await);
                    file.put_content("qwerty").await.unwrap();
                    
                    let listing = folder.get_directory_listing().await;
                    assert_eq!(1, listing.len());
                    assert_eq!(file.get_id().await, listing[0].get_id().await);
                    
                    // Get storage instances to compare them
                    let file_storage = file.get_storage().await;
                    let listing_storage = listing[0].get_storage().await;
                    
                    // Check that the storage instances are the same
                    // Note: In Rust we compare Arc pointers rather than the objects themselves
                    assert!(Arc::ptr_eq(&file_storage, &listing_storage));
                    
                    let root_listing = self.root.get_directory_listing().await;
                    assert_eq!(2, root_listing.len());
                    
                    folder.move_to("/asd").await.unwrap();
                    
                    let file = folder.get("/bar").await.unwrap();
                    // Check file is the right type
                    let file = file.downcast_arc::<dyn File>().unwrap();
                    
                    assert!(!self.root.node_exists("/foo/bar").await);
                    assert!(self.root.node_exists("/asd/bar").await);
                    assert_eq!("qwerty", file.get_content().await.unwrap());
                    
                    folder.move_to("/substorage/foo").await.unwrap();
                    
                    let file = folder.get("/bar").await.unwrap();
                    // Check file is the right type
                    let file = file.downcast_arc::<dyn File>().unwrap();
                    
                    assert!(self.root.node_exists("/substorage/foo/bar").await);
                    assert_eq!("qwerty", file.get_content().await.unwrap());
                }
            }
            
            // Additional implementations would be provided here in a real system
        }
    }
}