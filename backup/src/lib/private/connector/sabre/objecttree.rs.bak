// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::files::filesystem;
use crate::files::view::View;
use async_trait::async_trait;
use sabre_dav::{
    dav_exception::{Forbidden, NotFound},
    dav_inode::INode,
    dav_icollection::ICollection,
    dav_objecttree::ObjectTreeTrait,
    dav_urlutil::URLUtil,
};
use std::{collections::HashMap, sync::{Arc, Mutex}};

pub struct ObjectTree {
    /// keep this public to allow mock injection during unit test
    pub file_view: Option<Arc<View>>,
    root_node: Arc<dyn INode>,
    cache: Mutex<HashMap<String, Arc<dyn INode>>>,
}

impl ObjectTree {
    pub fn new(root_node: Arc<dyn INode>) -> Self {
        Self {
            file_view: None,
            root_node,
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Returns the INode object for the requested path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to get the node for
    ///
    /// # Returns
    ///
    /// The node for the given path
    ///
    /// # Errors
    ///
    /// If the file could not be located
    pub async fn get_node_for_path(&self, path: &str) -> Result<Arc<dyn INode>, NotFound> {
        let path = path.trim_matches('/');
        
        // Check cache first
        let cache = self.cache.lock().unwrap();
        if let Some(node) = cache.get(path) {
            return Ok(node.clone());
        }
        drop(cache); // Release the lock

        // Is it the root node?
        if path.is_empty() {
            return Ok(self.root_node.clone());
        }

        let file_view = self.get_file_view();
        let info = file_view.get_file_info(path).await;

        if let Some(info) = info {
            let node: Arc<dyn INode> = if info.get("mimetype") == Some("httpd/unix-directory") {
                Arc::new(crate::connector::sabre::directory::Directory::new(path.to_string()))
            } else {
                Arc::new(crate::connector::sabre::file::File::new(path.to_string()))
            };

            // Cast to set fileinfo cache
            if let Some(node_with_cache) = node.clone().downcast_arc::<dyn FileInfoCache>() {
                node_with_cache.set_fileinfo_cache(info);
            }

            // Update cache
            let mut cache = self.cache.lock().unwrap();
            cache.insert(path.to_string(), node.clone());
            
            Ok(node)
        } else {
            Err(NotFound::new(format!("File with name {} could not be located", path)))
        }
    }

    /// Moves a file from one location to another
    ///
    /// # Arguments
    ///
    /// * `source_path` - The path to the file which should be moved
    /// * `destination_path` - The full destination path, so not just the destination parent node
    ///
    /// # Errors
    ///
    /// If the move could not be performed
    pub async fn move_node(&self, source_path: &str, destination_path: &str) -> Result<(), Forbidden> {
        let source_node = self.get_node_for_path(source_path).await
            .map_err(|_| Forbidden::new(""))?;
        
        if source_node.is::<dyn ICollection>() && self.node_exists(destination_path).await {
            return Err(Forbidden::new(
                format!("Could not copy directory {}, target exists", source_path)
            ));
        }
        
        let (source_dir, _) = URLUtil::split_path(source_path);
        let (destination_dir, _) = URLUtil::split_path(destination_path);

        // check update privileges
        let fs = self.get_file_view();
        if !fs.is_updatable(source_path).await {
            return Err(Forbidden::new(""));
        }
        
        if source_dir != destination_dir {
            // for a full move we need update privileges on sourcePath and sourceDir as well as destinationDir
            if !fs.is_updatable(&source_dir).await {
                return Err(Forbidden::new(""));
            }
            if !fs.is_updatable(&destination_dir).await {
                return Err(Forbidden::new(""));
            }
            if !fs.is_deletable(source_path).await {
                return Err(Forbidden::new(""));
            }
        }

        let rename_okay = fs.rename(source_path, destination_path).await;
        if !rename_okay {
            return Err(Forbidden::new(""));
        }

        // update properties
        update_properties(destination_path, source_path).await;

        self.mark_dirty(&source_dir).await;
        self.mark_dirty(&destination_dir).await;
        
        Ok(())
    }

    /// Copies a file or directory.
    ///
    /// This method must work recursively and delete the destination
    /// if it exists
    ///
    /// # Arguments
    ///
    /// * `source` - Source path
    /// * `destination` - Destination path
    pub async fn copy(&self, source: &str, destination: &str) -> Result<(), Forbidden> {
        if filesystem::is_file(source).await {
            filesystem::copy(source, destination).await?;
        } else {
            filesystem::mkdir(destination).await?;
            
            if let Ok(mut dh) = filesystem::opendir(source).await {
                while let Some(subnode) = dh.read_dir().await {
                    if subnode == "." || subnode == ".." {
                        continue;
                    }
                    
                    let source_path = format!("{}/{}", source, subnode);
                    let destination_path = format!("{}/{}", destination, subnode);
                    self.copy(&source_path, &destination_path).await?;
                }
            }
        }

        let (destination_dir, _) = URLUtil::split_path(destination);
        self.mark_dirty(&destination_dir).await;
        
        Ok(())
    }

    pub fn get_file_view(&self) -> Arc<View> {
        if let Some(view) = &self.file_view {
            view.clone()
        } else {
            filesystem::get_view()
        }
    }
    
    async fn node_exists(&self, path: &str) -> bool {
        self.get_node_for_path(path).await.is_ok()
    }
    
    async fn mark_dirty(&self, path: &str) {
        // Implementation for marking directories as dirty
    }
}

#[async_trait]
impl ObjectTreeTrait for ObjectTree {
    async fn get_node_for_path(&self, path: &str) -> Result<Arc<dyn INode>, NotFound> {
        self.get_node_for_path(path).await
    }
    
    async fn move_node(&self, source: &str, destination: &str) -> Result<(), Forbidden> {
        self.move_node(source, destination).await
    }
    
    async fn copy(&self, source: &str, destination: &str) -> Result<(), Forbidden> {
        self.copy(source, destination).await
    }
}

#[async_trait]
pub trait FileInfoCache {
    async fn set_fileinfo_cache(&self, info: HashMap<String, String>);
}

async fn update_properties(destination_path: &str, source_path: &str) {
    let query = "UPDATE `*PREFIX*properties` SET `propertypath` = ? WHERE `userid` = ? AND `propertypath` = ?";
    let user = crate::user::get_user();
    
    crate::db::prepare_and_execute(query, &[
        destination_path,
        &user,
        source_path,
    ]).await;
}