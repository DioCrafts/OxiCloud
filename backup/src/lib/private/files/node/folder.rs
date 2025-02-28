use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::files::cache::cache::Cache;
use crate::files::cache::scanner::Scanner;
use crate::files::root::Root;
use crate::files::view::View;
use crate::files::node::{Node, NodeTrait};
use crate::files::node::file::File;
use crate::files::node::non_existing_file::NonExistingFile;
use crate::files::node::non_existing_folder::NonExistingFolder;
use crate::files::mount::Mount;
use crate::lib::constants::{PERMISSION_CREATE, PERMISSION_DELETE};
use crate::lib::errors::{NotFoundException, NotPermittedException};

/// Represents a folder in the file system
pub struct Folder {
    root: Arc<Root>,
    view: Arc<View>,
    path: String,
    pub(crate) exists: bool,
}

impl Folder {
    pub fn new(root: Arc<Root>, view: Arc<View>, path: String) -> Self {
        Self {
            root,
            view,
            path,
            exists: true,
        }
    }

    /// Get the full path for a relative path within this folder
    ///
    /// # Arguments
    ///
    /// * `path` - path relative to the folder
    ///
    /// # Returns
    ///
    /// Full path
    ///
    /// # Errors
    ///
    /// Returns `NotPermittedException` if the path is not valid
    pub fn get_full_path(&self, path: &str) -> Result<String, NotPermittedException> {
        if !self.is_valid_path(path) {
            return Err(NotPermittedException);
        }
        Ok(format!("{}{}", self.path, self.normalize_path(path)))
    }

    /// Get the relative path of a node to this folder
    ///
    /// # Arguments
    ///
    /// * `path` - path to convert to relative path
    ///
    /// # Returns
    ///
    /// Relative path
    ///
    /// # Errors
    ///
    /// Returns `NotFoundException` if the path is not a child of this folder
    pub fn get_relative_path(&self, path: &str) -> Result<String, NotFoundException> {
        if self.path.is_empty() || self.path == "/" {
            return Ok(self.normalize_path(path));
        }
        
        if !path.starts_with(&self.path) {
            return Err(NotFoundException);
        }
        
        let relative = &path[self.path.len()..];
        if relative.is_empty() {
            Ok("/".to_string())
        } else {
            Ok(self.normalize_path(relative))
        }
    }

    /// Check if a node is a (grand-)child of the folder
    ///
    /// # Arguments
    ///
    /// * `node` - node to check
    ///
    /// # Returns
    ///
    /// `true` if the node is a subnode, `false` otherwise
    pub fn is_sub_node(&self, node: &dyn NodeTrait) -> bool {
        node.get_path().starts_with(&format!("{}/", self.path))
    }

    /// Get the content of this directory
    ///
    /// # Returns
    ///
    /// List of nodes in the directory
    ///
    /// # Errors
    ///
    /// Returns `NotFoundException` if the directory does not exist
    pub fn get_directory_listing(&self) -> Result<Vec<Arc<dyn NodeTrait>>, NotFoundException> {
        let mut result = Vec::new();

        let (storage_opt, internal_path) = self.view.resolve_path(&self.path);
        
        let files = if let Some(storage) = storage_opt {
            let cache = storage.get_cache(&internal_path);
            let permissions_cache = storage.get_permissions_cache(&internal_path);

            // Trigger cache update check
            self.view.get_file_info(&self.path);

            let files = cache.get_folder_contents(&internal_path);
            let permissions = permissions_cache.get_directory_permissions(
                self.get_id(),
                &self.root.get_user().get_uid(),
            );
            
            files
        } else {
            Vec::new()
        };

        // Add a folder for any mountpoint in this directory and add the sizes of other mountpoints to the folders
        let mounts = self.root.get_mounts_in(&self.path);
        let dir_length = self.path.len();
        
        let mut files = files;
        
        for mount in mounts {
            if let Some(sub_storage) = mount.get_storage() {
                let sub_cache = sub_storage.get_cache("");

                if sub_cache.get_status("") == Cache::NOT_FOUND {
                    let sub_scanner = sub_storage.get_scanner("");
                    sub_scanner.scan_file("");
                }

                if let Some(root_entry) = sub_cache.get("") {
                    let relative_path = mount.get_mount_point()[dir_length..]
                        .trim_start_matches('/')
                        .to_string();
                    
                    if let Some(pos) = relative_path.find('/') {
                        // Mountpoint inside subfolder, add size to the correct folder
                        let entry_name = &relative_path[..pos];
                        for entry in &mut files {
                            if entry.get("name") == Some(entry_name) {
                                if root_entry.get("size").unwrap_or(&-1) >= &0 {
                                    *entry.get_mut("size").unwrap() += root_entry.get("size").unwrap();
                                } else {
                                    *entry.get_mut("size").unwrap() = -1;
                                }
                            }
                        }
                    } else {
                        // Mountpoint in this folder, add an entry for it
                        let mut root_entry = root_entry.clone();
                        root_entry.insert("name".to_string(), relative_path);
                        root_entry.insert("storageObject".to_string(), sub_storage);

                        // Remove any existing entry with the same name
                        files.retain(|file| 
                            file.get("name") != root_entry.get("name")
                        );
                        
                        files.push(root_entry);
                    }
                }
            }
        }

        for file in files {
            if let Some(file_id) = file.get("fileid") {
                if let Some(file_permissions) = permissions.get(file_id) {
                    file.insert("permissions".to_string(), *file_permissions);
                }
            }
            
            let node = self.create_node(
                format!("{}/{}", self.path, file.get("name").unwrap_or(&"".to_string())), 
                Some(&file)
            );
            
            result.push(node);
        }

        Ok(result)
    }

    /// Create a node for a path with optional info
    ///
    /// # Arguments
    ///
    /// * `path` - path for the node
    /// * `info` - optional info array
    ///
    /// # Returns
    ///
    /// Node instance
    fn create_node(&self, path: String, info: Option<&std::collections::HashMap<String, serde_json::Value>>) -> Arc<dyn NodeTrait> {
        let is_dir = if let Some(info) = info {
            if let Some(mimetype) = info.get("mimetype") {
                mimetype == "httpd/unix-directory"
            } else {
                self.view.is_dir(&path)
            }
        } else {
            self.view.is_dir(&path)
        };

        if is_dir {
            Arc::new(Folder::new(self.root.clone(), self.view.clone(), path))
        } else {
            Arc::new(File::new(self.root.clone(), self.view.clone(), path))
        }
    }

    /// Get the node at a specified path
    ///
    /// # Arguments
    ///
    /// * `path` - path relative to the folder
    ///
    /// # Returns
    ///
    /// Node at the specified path
    ///
    /// # Errors
    ///
    /// Returns `NotFoundException` if the node does not exist
    pub fn get(&self, path: &str) -> Result<Arc<dyn NodeTrait>, NotFoundException> {
        let full_path = self.get_full_path(path)?;
        self.root.get(&full_path)
    }

    /// Check if a node exists at the specified path
    ///
    /// # Arguments
    ///
    /// * `path` - path to check
    ///
    /// # Returns
    ///
    /// `true` if the node exists, `false` otherwise
    pub fn node_exists(&self, path: &str) -> bool {
        self.get(path).is_ok()
    }

    /// Create a new folder at the specified path
    ///
    /// # Arguments
    ///
    /// * `path` - path for the new folder
    ///
    /// # Returns
    ///
    /// New folder node
    ///
    /// # Errors
    ///
    /// Returns `NotPermittedException` if the folder cannot be created
    pub fn new_folder(&self, path: &str) -> Result<Arc<Folder>, NotPermittedException> {
        if self.check_permissions(PERMISSION_CREATE) {
            let full_path = self.get_full_path(path)?;
            let non_existing = NonExistingFolder::new(
                self.root.clone(), 
                self.view.clone(), 
                full_path.clone()
            );
            
            self.root.emit("\\OC\\Files", "preWrite", &[&non_existing]);
            self.root.emit("\\OC\\Files", "preCreate", &[&non_existing]);
            
            self.view.mkdir(&full_path)?;
            
            let node = Folder::new(self.root.clone(), self.view.clone(), full_path);
            let node_arc = Arc::new(node);
            
            self.root.emit("\\OC\\Files", "postWrite", &[&node_arc]);
            self.root.emit("\\OC\\Files", "postCreate", &[&node_arc]);
            
            Ok(node_arc)
        } else {
            Err(NotPermittedException)
        }
    }

    /// Create a new file at the specified path
    ///
    /// # Arguments
    ///
    /// * `path` - path for the new file
    ///
    /// # Returns
    ///
    /// New file node
    ///
    /// # Errors
    ///
    /// Returns `NotPermittedException` if the file cannot be created
    pub fn new_file(&self, path: &str) -> Result<Arc<File>, NotPermittedException> {
        if self.check_permissions(PERMISSION_CREATE) {
            let full_path = self.get_full_path(path)?;
            let non_existing = NonExistingFile::new(
                self.root.clone(), 
                self.view.clone(), 
                full_path.clone()
            );
            
            self.root.emit("\\OC\\Files", "preWrite", &[&non_existing]);
            self.root.emit("\\OC\\Files", "preCreate", &[&non_existing]);
            
            self.view.touch(&full_path)?;
            
            let node = File::new(self.root.clone(), self.view.clone(), full_path);
            let node_arc = Arc::new(node);
            
            self.root.emit("\\OC\\Files", "postWrite", &[&node_arc]);
            self.root.emit("\\OC\\Files", "postCreate", &[&node_arc]);
            
            Ok(node_arc)
        } else {
            Err(NotPermittedException)
        }
    }

    /// Search for files with the name matching the query
    ///
    /// # Arguments
    ///
    /// * `query` - search query
    ///
    /// # Returns
    ///
    /// List of matching nodes
    pub fn search(&self, query: &str) -> Vec<Arc<dyn NodeTrait>> {
        self.search_common(&format!("%{}%", query), "search")
    }

    /// Search for files by mimetype
    ///
    /// # Arguments
    ///
    /// * `mimetype` - mimetype to search for
    ///
    /// # Returns
    ///
    /// List of matching nodes
    pub fn search_by_mime(&self, mimetype: &str) -> Vec<Arc<dyn NodeTrait>> {
        self.search_common(mimetype, "searchByMime")
    }

    /// Common search implementation
    ///
    /// # Arguments
    ///
    /// * `query` - search query
    /// * `method` - search method to use
    ///
    /// # Returns
    ///
    /// List of matching nodes
    fn search_common(&self, query: &str, method: &str) -> Vec<Arc<dyn NodeTrait>> {
        let mut files = Vec::new();
        let root_length = self.path.len();
        
        let (storage_opt, internal_path) = self.view.resolve_path(&self.path);
        let internal_root_length = internal_path.len();

        if let Some(storage) = storage_opt {
            let cache = storage.get_cache("");

            let results = match method {
                "search" => cache.search(query),
                "searchByMime" => cache.search_by_mime(query),
                _ => Vec::new(),
            };
            
            for mut result in results {
                if internal_root_length == 0 || result.get("path").unwrap_or(&"".to_string()).starts_with(&internal_path) {
                    result.insert("internalPath".to_string(), result.get("path").unwrap().clone());
                    let path = result.get("path").unwrap()[internal_root_length..].to_string();
                    result.insert("path".to_string(), path);
                    result.insert("storage".to_string(), storage.clone());
                    files.push(result);
                }
            }
        }

        let mounts = self.root.get_mounts_in(&self.path);
        for mount in mounts {
            if let Some(storage) = mount.get_storage() {
                let cache = storage.get_cache("");

                let relative_mount_point = &mount.get_mount_point()[root_length..];
                
                let results = match method {
                    "search" => cache.search(query),
                    "searchByMime" => cache.search_by_mime(query),
                    _ => Vec::new(),
                };
                
                for mut result in results {
                    result.insert("internalPath".to_string(), result.get("path").unwrap().clone());
                    let path = format!("{}{}", relative_mount_point, result.get("path").unwrap());
                    result.insert("path".to_string(), path);
                    result.insert("storage".to_string(), storage.clone());
                    files.push(result);
                }
            }
        }

        let mut result = Vec::new();
        for file in files {
            let path = self.normalize_path(&format!("{}/{}", self.path, file.get("path").unwrap()));
            result.push(self.create_node(path, Some(&file)));
        }

        result
    }

    /// Get nodes by id
    ///
    /// # Arguments
    ///
    /// * `id` - id to search for
    ///
    /// # Returns
    ///
    /// List of matching nodes
    pub fn get_by_id(&self, id: i64) -> Vec<Arc<dyn NodeTrait>> {
        let nodes = self.root.get_by_id(id);
        let mut result = Vec::new();
        
        for node in nodes {
            let path_part = &node.get_path()[..self.get_path().len() + 1];
            if self.path == "/" || path_part == format!("{}/", self.get_path()) {
                result.push(node.clone());
            }
        }
        
        result
    }

    /// Get the free space in the folder
    ///
    /// # Returns
    ///
    /// Free space in bytes
    pub fn get_free_space(&self) -> i64 {
        self.view.free_space(&self.path)
    }

    /// Check if the folder is creatable
    ///
    /// # Returns
    ///
    /// `true` if the folder is creatable, `false` otherwise
    pub fn is_creatable(&self) -> bool {
        self.check_permissions(PERMISSION_CREATE)
    }

    /// Delete the folder
    ///
    /// # Errors
    ///
    /// Returns `NotPermittedException` if the folder cannot be deleted
    pub fn delete(&mut self) -> Result<(), NotPermittedException> {
        if self.check_permissions(PERMISSION_DELETE) {
            self.send_hooks(&["preDelete"]);
            self.view.rmdir(&self.path)?;
            
            let non_existing = NonExistingFolder::new(
                self.root.clone(), 
                self.view.clone(), 
                self.path.clone()
            );
            
            self.root.emit("\\OC\\Files", "postDelete", &[&non_existing]);
            self.exists = false;
            Ok(())
        } else {
            Err(NotPermittedException)
        }
    }

    /// Copy the folder to a target path
    ///
    /// # Arguments
    ///
    /// * `target_path` - target path
    ///
    /// # Returns
    ///
    /// New node at the target path
    ///
    /// # Errors
    ///
    /// Returns `NotPermittedException` if the folder cannot be copied
    pub fn copy(&self, target_path: &str) -> Result<Arc<dyn NodeTrait>, NotPermittedException> {
        let target_path = self.normalize_path(target_path);
        let parent_path = Path::new(&target_path)
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_string_lossy()
            .to_string();
            
        let parent = self.root.get(&parent_path)?;
        
        if let Some(parent_folder) = parent.as_folder() {
            if self.is_valid_path(&target_path) && parent_folder.is_creatable() {
                let non_existing = NonExistingFolder::new(
                    self.root.clone(), 
                    self.view.clone(), 
                    target_path.clone()
                );
                
                self.root.emit("\\OC\\Files", "preCopy", &[self, &non_existing]);
                self.root.emit("\\OC\\Files", "preWrite", &[&non_existing]);
                
                self.view.copy(&self.path, &target_path)?;
                
                let target_node = self.root.get(&target_path)?;
                
                self.root.emit("\\OC\\Files", "postCopy", &[self, &target_node]);
                self.root.emit("\\OC\\Files", "postWrite", &[&target_node]);
                
                Ok(target_node)
            } else {
                Err(NotPermittedException)
            }
        } else {
            Err(NotPermittedException)
        }
    }

    /// Move the folder to a target path
    ///
    /// # Arguments
    ///
    /// * `target_path` - target path
    ///
    /// # Returns
    ///
    /// Node at the new path
    ///
    /// # Errors
    ///
    /// Returns `NotPermittedException` if the folder cannot be moved
    pub fn move_to(&mut self, target_path: &str) -> Result<Arc<dyn NodeTrait>, NotPermittedException> {
        let target_path = self.normalize_path(target_path);
        let parent_path = Path::new(&target_path)
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_string_lossy()
            .to_string();
            
        let parent = self.root.get(&parent_path)?;
        
        if let Some(parent_folder) = parent.as_folder() {
            if self.is_valid_path(&target_path) && parent_folder.is_creatable() {
                let non_existing = NonExistingFolder::new(
                    self.root.clone(), 
                    self.view.clone(), 
                    target_path.clone()
                );
                
                self.root.emit("\\OC\\Files", "preRename", &[self, &non_existing]);
                self.root.emit("\\OC\\Files", "preWrite", &[&non_existing]);
                
                self.view.rename(&self.path, &target_path)?;
                
                let target_node = self.root.get(&target_path)?;
                
                self.root.emit("\\OC\\Files", "postRename", &[self, &target_node]);
                self.root.emit("\\OC\\Files", "postWrite", &[&target_node]);
                
                self.path = target_path;
                
                Ok(target_node)
            } else {
                Err(NotPermittedException)
            }
        } else {
            Err(NotPermittedException)
        }
    }
}

impl NodeTrait for Folder {
    fn get_id(&self) -> i64 {
        let info = self.view.get_file_info(&self.path);
        info.get("fileid").unwrap_or(&-1).as_i64().unwrap_or(-1)
    }

    fn get_path(&self) -> &str {
        &self.path
    }

    fn get_name(&self) -> String {
        Path::new(&self.path)
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new(""))
            .to_string_lossy()
            .to_string()
    }

    fn get_mime_type(&self) -> String {
        "httpd/unix-directory".to_string()
    }

    fn get_size(&self) -> i64 {
        let info = self.view.get_file_info(&self.path);
        info.get("size").unwrap_or(&0).as_i64().unwrap_or(0)
    }

    fn get_etag(&self) -> String {
        let info = self.view.get_file_info(&self.path);
        info.get("etag").unwrap_or(&"").as_str().unwrap_or("").to_string()
    }

    fn get_permissions(&self) -> i64 {
        let info = self.view.get_file_info(&self.path);
        info.get("permissions").unwrap_or(&0).as_i64().unwrap_or(0)
    }

    fn check_permissions(&self, permissions: i64) -> bool {
        (self.get_permissions() & permissions) == permissions
    }

    fn is_readable(&self) -> bool {
        unimplemented!("Method not implemented")
    }

    fn is_updateable(&self) -> bool {
        unimplemented!("Method not implemented")
    }

    fn is_deletable(&self) -> bool {
        unimplemented!("Method not implemented")
    }

    fn is_shareable(&self) -> bool {
        unimplemented!("Method not implemented")
    }

    fn touch(&mut self, _mtime: i64) -> Result<(), NotPermittedException> {
        unimplemented!("Method not implemented")
    }

    fn as_folder(&self) -> Option<&Folder> {
        Some(self)
    }

    fn as_file(&self) -> Option<&File> {
        None
    }

    fn send_hooks(&self, hooks: &[&str]) {
        for hook in hooks {
            self.root.emit("\\OC\\Files", hook, &[self]);
        }
    }

    fn is_valid_path(&self, path: &str) -> bool {
        !path.contains("/../") && !path.ends_with("/..")
    }

    fn normalize_path(&self, path: &str) -> String {
        let path = path.trim();
        if path.is_empty() {
            return "/".to_string();
        }
        
        // Making sure there is a / at the beginning and no trailing /
        let mut result = path.to_string();
        if !result.starts_with('/') {
            result = format!("/{}", result);
        }
        
        if result.len() > 1 && result.ends_with('/') {
            result.pop();
        }
        
        result
    }
}