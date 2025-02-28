use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

/// ETag property name in DAV
pub const GETETAG_PROPERTYNAME: &str = "{DAV:}getetag";
/// Last modified property name in DAV
pub const LASTMODIFIED_PROPERTYNAME: &str = "{DAV:}lastmodified";

/// Type definition for ETag generation function
pub type ETagFunction = Option<(String, String)>;

/// Error types for Node operations
#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("Forbidden operation")]
    Forbidden,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Other error: {0}")]
    Other(String),
}

type Result<T> = std::result::Result<T, NodeError>;

/// File information cache
#[derive(Debug, Clone, Default)]
pub struct FileInfoCache {
    pub mtime: i64,
    pub etag: Option<String>,
    pub fileid: Option<u64>,
}

/// Filesystem interface trait
pub trait Filesystem: Send + Sync {
    fn is_updatable(&self, path: &Path) -> bool;
    fn rename(&self, old_path: &Path, new_path: &Path) -> Result<()>;
    fn get_file_info(&self, path: &Path) -> Result<FileInfoCache>;
    fn stat(&self, path: &Path) -> Result<FileInfoCache>;
    fn touch(&self, path: &Path, mtime: Option<i64>) -> Result<()>;
    fn put_file_info(&self, path: &Path, info: HashMap<String, String>) -> Result<()>;
}

/// Database interface trait
pub trait Database: Send + Sync {
    fn update_property_path(&self, new_path: &Path, user: &str, old_path: &Path) -> Result<()>;
    fn delete_property(&self, user: &str, path: &Path, property_name: &str) -> Result<()>;
    fn insert_property(&self, user: &str, path: &Path, property_name: &str, property_value: &str) -> Result<()>;
    fn update_property(&self, property_value: &str, user: &str, path: &Path, property_name: &str) -> Result<()>;
    fn get_properties(&self, user: &str, path: &Path) -> Result<HashMap<String, String>>;
    fn delete_properties(&self, user: &str, path: &Path) -> Result<()>;
}

/// DAV Node implementation
pub struct Node {
    /// Path to the current node
    path: PathBuf,
    /// Fileinfo cache
    fileinfo_cache: Option<FileInfoCache>,
    /// Property cache
    property_cache: Option<HashMap<String, String>>,
    /// Filesystem implementation
    fs: Arc<dyn Filesystem>,
    /// Database implementation
    db: Arc<dyn Database>,
    /// Current user
    user: String,
    /// Allow configuring the method used to generate ETags
    pub etag_function: RwLock<ETagFunction>,
}

/// DAV INode trait
pub trait INode {
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: &str) -> Result<()>;
    fn get_last_modified(&self) -> Result<i64>;
}

/// DAV IProperties trait
pub trait IProperties {
    fn update_properties(&mut self, properties: HashMap<String, Option<String>>) -> Result<bool>;
    fn get_properties(&mut self, properties: Vec<String>) -> Result<HashMap<String, String>>;
    fn remove_properties(&mut self) -> Result<()>;
}

impl Node {
    /// Sets up the node, expects a full path name
    pub fn new(
        path: PathBuf, 
        fs: Arc<dyn Filesystem>, 
        db: Arc<dyn Database>, 
        user: String
    ) -> Self {
        Self {
            path,
            fileinfo_cache: None,
            property_cache: None,
            fs,
            db,
            user,
            etag_function: RwLock::new(None),
        }
    }

    /// Set fileinfo cache directly
    pub fn set_fileinfo_cache(&mut self, fileinfo_cache: FileInfoCache) {
        self.fileinfo_cache = Some(fileinfo_cache);
    }

    /// Set property cache directly
    pub fn set_property_cache(&mut self, property_cache: Option<HashMap<String, String>>) {
        self.property_cache = property_cache;
    }

    /// Ensure that the fileinfo cache is filled
    fn ensure_fileinfo_cache(&mut self) -> Result<()> {
        if self.fileinfo_cache.is_none() {
            match self.fs.get_file_info(&self.path) {
                Ok(info) => self.fileinfo_cache = Some(info),
                Err(_) => {
                    let info = self.fs.stat(&self.path)?;
                    self.fileinfo_cache = Some(info);
                }
            }
        }
        Ok(())
    }

    /// Touch the file to update its modification time
    pub fn touch(&self, mtime: Option<i64>) -> Result<()> {
        self.fs.touch(&self.path, mtime)
    }

    /// Returns the file ID
    pub fn get_file_id(&mut self) -> Result<Option<String>> {
        self.ensure_fileinfo_cache()?;
        
        if let Some(ref cache) = self.fileinfo_cache {
            if let Some(fileid) = cache.fileid {
                // Get instance ID from system
                let instance_id = get_instance_id();
                let id = format!("{:08}", fileid);
                return Ok(Some(format!("{}{}", id, instance_id)));
            }
        }
        
        Ok(None)
    }

    /// Returns the ETag surrounded by double-quotes for this path
    fn get_etag_property_for_path(&self, path: &Path) -> Result<Option<String>> {
        let data = self.fs.get_file_info(path)?;
        
        if let Some(etag) = data.etag {
            Ok(Some(format!("\"{}\"", etag)))
        } else {
            Ok(None)
        }
    }
}

impl INode for Node {
    fn get_name(&self) -> String {
        self.path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string()
    }
    
    fn set_name(&mut self, name: &str) -> Result<()> {
        // Rename is only allowed if the update privilege is granted
        if !self.fs.is_updatable(&self.path) {
            return Err(NodeError::Forbidden);
        }
        
        let parent_path = self.path.parent()
            .unwrap_or_else(|| Path::new(""));
        
        let new_path = parent_path.join(name);
        let old_path = self.path.clone();
        
        self.fs.rename(&self.path, &new_path)?;
        
        // Update the database
        self.db.update_property_path(&new_path, &self.user, &old_path)?;
        
        // Update the current path
        self.path = new_path;
        
        Ok(())
    }
    
    fn get_last_modified(&self) -> Result<i64> {
        let mut self_mut = self.to_owned();
        self_mut.ensure_fileinfo_cache()?;
        
        if let Some(ref cache) = self_mut.fileinfo_cache {
            Ok(cache.mtime)
        } else {
            Err(NodeError::NotFound("File info not found".to_string()))
        }
    }
}

impl IProperties for Node {
    fn update_properties(&mut self, properties: HashMap<String, Option<String>>) -> Result<bool> {
        let existing = self.get_properties(vec![])?;
        
        for (property_name, property_value) in properties {
            // If it was None, we need to delete the property
            match property_value {
                None => {
                    if existing.contains_key(&property_name) {
                        self.db.delete_property(&self.user, &self.path, &property_name)?;
                    }
                },
                Some(value) => {
                    if property_name == GETETAG_PROPERTYNAME {
                        let mut info = HashMap::new();
                        info.insert("etag".to_string(), value);
                        self.fs.put_file_info(&self.path, info)?;
                    } else if property_name == LASTMODIFIED_PROPERTYNAME {
                        // Convert string to timestamp and touch the file
                        if let Ok(ts) = value.parse::<i64>() {
                            self.touch(Some(ts))?;
                        }
                    } else {
                        if !existing.contains_key(&property_name) {
                            self.db.insert_property(&self.user, &self.path, &property_name, &value)?;
                        } else {
                            self.db.update_property(&value, &self.user, &self.path, &property_name)?;
                        }
                    }
                }
            }
        }
        
        self.set_property_cache(None);
        Ok(true)
    }
    
    fn get_properties(&mut self, properties: Vec<String>) -> Result<HashMap<String, String>> {
        if self.property_cache.is_none() {
            // Load properties from database
            let mut props = self.db.get_properties(&self.user, &self.path)?;
            
            // Add ETag property
            self.ensure_fileinfo_cache()?;
            if let Some(ref cache) = self.fileinfo_cache {
                if let Some(ref etag) = cache.etag {
                    props.insert(GETETAG_PROPERTYNAME.to_string(), format!("\"{}\"", etag));
                }
            }
            
            self.property_cache = Some(props);
        }
        
        let cache = self.property_cache.clone().unwrap_or_default();
        
        // If the array was empty, we need to return everything
        if properties.is_empty() {
            return Ok(cache);
        }
        
        // Otherwise filter only requested properties
        let mut result = HashMap::new();
        for prop in properties {
            if let Some(value) = cache.get(&prop) {
                result.insert(prop, value.clone());
            }
        }
        
        Ok(result)
    }
    
    fn remove_properties(&mut self) -> Result<()> {
        self.db.delete_properties(&self.user, &self.path)?;
        self.set_property_cache(None);
        Ok(())
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            fileinfo_cache: self.fileinfo_cache.clone(),
            property_cache: self.property_cache.clone(),
            fs: Arc::clone(&self.fs),
            db: Arc::clone(&self.db),
            user: self.user.clone(),
            etag_function: RwLock::new(*self.etag_function.read().unwrap()),
        }
    }
}

/// Utility functions for URL handling
pub mod url_util {
    use std::path::Path;

    /// Split a path into parent and name component
    pub fn split_path(path: &Path) -> (String, String) {
        let parent = path.parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string();
            
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
            
        (parent, name)
    }
}

/// Helper function to get instance ID
fn get_instance_id() -> String {
    // This would be implemented based on your system's requirements
    "instance123".to_string()
}