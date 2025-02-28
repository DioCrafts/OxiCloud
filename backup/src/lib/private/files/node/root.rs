use std::sync::{Arc, Weak};

use crate::files::cache::cache::Cache;
use crate::files::mount::manager::Manager;
use crate::files::mount::mount::Mount;
use crate::files::node::folder::Folder;
use crate::files::node::node::Node;
use crate::files::view::View;
use crate::hooks::emitter::Emitter;
use crate::hooks::public_emitter::PublicEmitter;
use crate::user::user::User;
use crate::OCP;
use crate::NotFoundException;
use crate::NotPermittedException;

/// # Root
///
/// Hooks available in scope \OC\Files
/// - preWrite(\OCP\Files\Node $node)
/// - postWrite(\OCP\Files\Node $node)
/// - preCreate(\OCP\Files\Node $node)
/// - postCreate(\OCP\Files\Node $node)
/// - preDelete(\OCP\Files\Node $node)
/// - postDelete(\OCP\Files\Node $node)
/// - preTouch(\OC\FilesP\Node $node, int $mtime)
/// - postTouch(\OCP\Files\Node $node)
/// - preCopy(\OCP\Files\Node $source, \OCP\Files\Node $target)
/// - postCopy(\OCP\Files\Node $source, \OCP\Files\Node $target)
/// - preRename(\OCP\Files\Node $source, \OCP\Files\Node $target)
/// - postRename(\OCP\Files\Node $source, \OCP\Files\Node $target)
pub struct Root {
    folder: Folder,
    mount_manager: Arc<Manager>,
    emitter: PublicEmitter,
    user: Arc<User>,
}

impl Root {
    /// Creates a new Root instance
    ///
    /// # Arguments
    ///
    /// * `manager` - The mount manager
    /// * `view` - The file view
    /// * `user` - The user
    pub fn new(manager: Arc<Manager>, view: Arc<View>, user: Arc<User>) -> Self {
        let root = Self {
            folder: Folder::new(None, view.clone(), String::from("")),
            mount_manager: manager,
            user: user,
            emitter: PublicEmitter::new(),
        };
        
        // Set the root as parent for the folder
        root.folder.set_root(Arc::new(root.clone()));
        
        root
    }

    /// Get the user for which the filesystem is setup
    pub fn get_user(&self) -> Arc<User> {
        self.user.clone()
    }

    /// Mount a storage
    ///
    /// # Arguments
    ///
    /// * `storage` - The storage to mount
    /// * `mount_point` - The mount point
    /// * `arguments` - Additional arguments
    pub fn mount(&self, storage: Arc<dyn crate::files::storage::Storage>, mount_point: &str, arguments: Vec<String>) {
        let mount = Mount::new(storage, mount_point.to_string(), arguments);
        self.mount_manager.add_mount(mount);
    }

    /// Get a mount by mount point
    ///
    /// # Arguments
    ///
    /// * `mount_point` - The mount point
    pub fn get_mount(&self, mount_point: &str) -> Option<Arc<Mount>> {
        self.mount_manager.find(mount_point)
    }

    /// Get mounts in a specific mount point
    ///
    /// # Arguments
    ///
    /// * `mount_point` - The mount point
    pub fn get_mounts_in(&self, mount_point: &str) -> Vec<Arc<Mount>> {
        self.mount_manager.find_in(mount_point)
    }

    /// Get mounts by storage ID
    ///
    /// # Arguments
    ///
    /// * `storage_id` - The storage ID
    pub fn get_mount_by_storage_id(&self, storage_id: &str) -> Vec<Arc<Mount>> {
        self.mount_manager.find_by_storage_id(storage_id)
    }

    /// Get mounts by numeric storage ID
    ///
    /// # Arguments
    ///
    /// * `numeric_id` - The numeric storage ID
    pub fn get_mount_by_numeric_storage_id(&self, numeric_id: i64) -> Vec<Arc<Mount>> {
        self.mount_manager.find_by_numeric_id(numeric_id)
    }

    /// Unmount a mount
    ///
    /// # Arguments
    ///
    /// * `mount` - The mount to unmount
    pub fn un_mount(&self, mount: Arc<Mount>) {
        self.mount_manager.remove(mount);
    }

    /// Get a node by path
    ///
    /// # Arguments
    ///
    /// * `path` - The path
    ///
    /// # Returns
    ///
    /// The node at the specified path
    ///
    /// # Errors
    ///
    /// * `NotFoundException` - If the node does not exist
    /// * `NotPermittedException` - If the node is not accessible
    pub fn get(&self, path: &str) -> Result<Arc<dyn Node>, Box<dyn std::error::Error>> {
        let path = self.folder.normalize_path(path);
        if self.folder.is_valid_path(&path) {
            let full_path = self.folder.get_full_path(&path);
            if self.folder.get_view().file_exists(&full_path) {
                Ok(self.folder.create_node(&full_path))
            } else {
                Err(Box::new(NotFoundException::new()))
            }
        } else {
            Err(Box::new(NotPermittedException::new()))
        }
    }

    /// Search file by id
    ///
    /// An array is returned because in the case where a single storage is mounted in different places the same file
    /// can exist in different places
    ///
    /// # Arguments
    ///
    /// * `id` - The file ID
    ///
    /// # Returns
    ///
    /// A vector of nodes
    ///
    /// # Errors
    ///
    /// * `NotFoundException` - If the node does not exist
    pub fn get_by_id(&self, id: i64) -> Result<Vec<Arc<dyn Node>>, Box<dyn std::error::Error>> {
        let result = Cache::get_by_id(id);
        match result {
            None => Err(Box::new(NotFoundException::new())),
            Some((storage_id, internal_path)) => {
                let mut nodes = Vec::new();
                let mounts = self.mount_manager.find_by_storage_id(&storage_id);
                for mount in mounts {
                    let path = format!("{}{}", mount.get_mount_point(), internal_path);
                    nodes.push(self.get(&path)?);
                }
                Ok(nodes)
            }
        }
    }

    // Most operations can't be done on the root

    /// Rename is not allowed on root
    pub fn rename(&self, _target_path: &str) -> Result<Arc<dyn Node>, Box<dyn std::error::Error>> {
        Err(Box::new(NotPermittedException::new()))
    }

    /// Delete is not allowed on root
    pub fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(NotPermittedException::new()))
    }

    /// Copy is not allowed on root
    pub fn copy(&self, _target_path: &str) -> Result<Arc<dyn Node>, Box<dyn std::error::Error>> {
        Err(Box::new(NotPermittedException::new()))
    }

    /// Touch is not allowed on root
    pub fn touch(&self, _mtime: Option<i64>) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(NotPermittedException::new()))
    }

    /// Get storage is not available on root
    pub fn get_storage(&self) -> Result<Arc<dyn crate::files::storage::Storage>, Box<dyn std::error::Error>> {
        Err(Box::new(NotFoundException::new()))
    }

    /// Get path of root
    pub fn get_path(&self) -> String {
        String::from("/")
    }

    /// Get internal path of root
    pub fn get_internal_path(&self) -> String {
        String::from("")
    }

    /// Get ID of root
    pub fn get_id(&self) -> Option<i64> {
        None
    }

    /// Get stats of root
    pub fn stat(&self) -> Option<crate::files::stat::Stat> {
        None
    }

    /// Get modification time of root
    pub fn get_mtime(&self) -> Option<i64> {
        None
    }

    /// Get size of root
    pub fn get_size(&self) -> Option<i64> {
        None
    }

    /// Get etag of root
    pub fn get_etag(&self) -> Option<String> {
        None
    }

    /// Get permissions of root
    pub fn get_permissions(&self) -> i32 {
        OCP::PERMISSION_CREATE
    }

    /// Check if root is readable
    pub fn is_readable(&self) -> bool {
        false
    }

    /// Check if root is updateable
    pub fn is_updateable(&self) -> bool {
        false
    }

    /// Check if root is deletable
    pub fn is_deletable(&self) -> bool {
        false
    }

    /// Check if root is shareable
    pub fn is_shareable(&self) -> bool {
        false
    }

    /// Get parent of root
    pub fn get_parent(&self) -> Result<Arc<dyn Node>, Box<dyn std::error::Error>> {
        Err(Box::new(NotFoundException::new()))
    }

    /// Get name of root
    pub fn get_name(&self) -> String {
        String::from("")
    }
}

impl Emitter for Root {
    /// Register a listener
    fn listen<F>(&self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&[Box<dyn std::any::Any>]) + Send + Sync + 'static,
    {
        self.emitter.listen(scope, method, callback);
    }

    /// Remove a listener
    fn remove_listener<F>(&self, scope: Option<&str>, method: Option<&str>, callback: Option<F>)
    where
        F: Fn(&[Box<dyn std::any::Any>]) + Send + Sync + 'static,
    {
        self.emitter.remove_listener(scope, method, callback);
    }

    /// Emit an event
    fn emit(&self, scope: &str, method: &str, arguments: &[Box<dyn std::any::Any>]) {
        self.emitter.emit(scope, method, arguments);
    }
}

impl Clone for Root {
    fn clone(&self) -> Self {
        Self {
            folder: self.folder.clone(),
            mount_manager: self.mount_manager.clone(),
            emitter: self.emitter.clone(),
            user: self.user.clone(),
        }
    }
}