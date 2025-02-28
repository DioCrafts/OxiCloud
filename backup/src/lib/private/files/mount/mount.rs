use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use crate::files::filesystem;
use crate::files::storage::loader::Loader;
use crate::files::storage::storage::Storage;
use crate::log;

/// Mount point in the filesystem
pub struct Mount {
    storage: Option<Arc<Mutex<dyn Storage>>>,
    class: String,
    storage_id: Option<String>,
    arguments: Vec<String>,
    mount_point: String,
    loader: Arc<Loader>,
}

impl Mount {
    /// Create a new mount
    ///
    /// # Arguments
    ///
    /// * `storage` - Storage implementation or class name
    /// * `mount_point` - Path where the storage is mounted
    /// * `arguments` - Optional arguments for the storage
    /// * `loader` - Optional storage loader
    pub fn new<S>(
        storage: S,
        mount_point: &str,
        arguments: Option<Vec<String>>,
        loader: Option<Arc<Loader>>,
    ) -> Self 
    where
        S: Into<StorageSource>,
    {
        let arguments = arguments.unwrap_or_default();
        let loader = loader.unwrap_or_else(|| Arc::new(Loader::new()));
        
        let mount_point = Self::format_path(mount_point);
        
        let (class, initial_storage) = match storage.into() {
            StorageSource::Instance(storage_instance) => {
                let class_name = std::any::type_name_of_val(&*storage_instance).to_string();
                let wrapped = loader.wrap(&mount_point, storage_instance);
                (class_name, Some(wrapped))
            },
            StorageSource::ClassName(class_name) => {
                // Update old classes to new namespace
                let class_name = if class_name.starts_with("OC_Filestorage_") {
                    format!("OC\\Files\\Storage\\{}", &class_name[15..])
                } else {
                    class_name
                };
                (class_name, None)
            }
        };
        
        Mount {
            storage: initial_storage,
            class,
            storage_id: None,
            arguments,
            mount_point,
            loader,
        }
    }

    /// Get the mount point
    pub fn get_mount_point(&self) -> &str {
        &self.mount_point
    }

    /// Create the storage that is mounted
    fn create_storage(&self) -> Option<Arc<Mutex<dyn Storage>>> {
        if self.class_exists(&self.class) {
            match self.loader.load(&self.mount_point, &self.class, &self.arguments) {
                Ok(storage) => Some(storage),
                Err(e) => {
                    log::error("core", &format!("{}", e));
                    None
                }
            }
        } else {
            log::error("core", &format!("storage backend {} not found", self.class));
            None
        }
    }

    // Helper method to check if a class exists
    fn class_exists(&self, class: &str) -> bool {
        // This is a simplification - in Rust you'd use a registry or other mechanism
        // to check if a storage implementation exists
        true
    }

    /// Get the storage
    pub fn get_storage(&mut self) -> Option<Arc<Mutex<dyn Storage>>> {
        if self.storage.is_none() {
            self.storage = self.create_storage();
        }
        self.storage.clone()
    }

    /// Get the storage ID
    pub fn get_storage_id(&mut self) -> Option<String> {
        if self.storage_id.is_none() {
            if self.storage.is_none() {
                let storage = self.create_storage();
                if storage.is_none() {
                    return None;
                }
                self.storage = storage;
            }
            
            if let Some(storage) = &self.storage {
                let storage = storage.lock().expect("Failed to lock storage");
                let id = storage.get_id();
                
                // Hash the ID if it's too long
                if id.len() > 64 {
                    use md5::{Md5, Digest};
                    let mut hasher = Md5::new();
                    hasher.update(id.as_bytes());
                    let hash = hasher.finalize();
                    self.storage_id = Some(format!("{:x}", hash));
                } else {
                    self.storage_id = Some(id);
                }
            }
        }
        
        self.storage_id.clone()
    }

    /// Get the internal path for a given path
    pub fn get_internal_path(&self, path: &str) -> String {
        if self.mount_point == path || format!("{}/", self.mount_point) == path {
            "".to_string()
        } else {
            path[self.mount_point.len()..].to_string()
        }
    }

    /// Format a path according to the filesystem rules
    fn format_path(path: &str) -> String {
        let path = filesystem::normalize_path(path);
        if path.len() > 1 {
            format!("{}/", path)
        } else {
            path
        }
    }

    /// Wrap the storage with a custom wrapper
    pub fn wrap_storage<F>(&mut self, wrapper: F)
    where
        F: FnOnce(&str, Arc<Mutex<dyn Storage>>) -> Arc<Mutex<dyn Storage>>,
    {
        if let Some(storage) = self.storage.take() {
            self.storage = Some(wrapper(&self.mount_point, storage));
        }
    }
}

/// Represents either a storage instance or a storage class name
pub enum StorageSource {
    Instance(Arc<Mutex<dyn Storage>>),
    ClassName(String),
}

impl From<Arc<Mutex<dyn Storage>>> for StorageSource {
    fn from(storage: Arc<Mutex<dyn Storage>>) -> Self {
        StorageSource::Instance(storage)
    }
}

impl From<String> for StorageSource {
    fn from(class_name: String) -> Self {
        StorageSource::ClassName(class_name)
    }
}

impl From<&str> for StorageSource {
    fn from(class_name: &str) -> Self {
        StorageSource::ClassName(class_name.to_string())
    }
}