// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use md5::{Digest, Md5};
use std::collections::HashMap;
use std::sync::Arc;

pub mod test_files_mount {
    pub struct Temporary {
        id: String,
    }

    impl Temporary {
        pub fn new(_params: HashMap<String, String>) -> Self {
            Self {
                id: format!("temporary:{}", uuid::Uuid::new_v4().to_string()),
            }
        }

        pub fn get_id(&self) -> String {
            self.id.clone()
        }
    }
}

pub struct LongId {
    temporary: test_files_mount::Temporary,
}

impl LongId {
    pub fn new(params: HashMap<String, String>) -> Self {
        Self {
            temporary: test_files_mount::Temporary::new(params),
        }
    }

    pub fn get_id(&self) -> String {
        format!("long:{}{}", "foo".repeat(50), self.temporary.get_id())
    }
}

pub struct Mount {
    storage: Box<dyn Storage>,
    mount_point: String,
}

impl Mount {
    pub fn new<S: Storage + 'static>(storage: S, mount_point: String) -> Self {
        Self {
            storage: Box::new(storage),
            mount_point,
        }
    }

    pub fn get_storage_id(&self) -> String {
        let id = self.storage.get_id();
        if id.len() > 64 {
            let mut hasher = Md5::new();
            hasher.update(id.as_bytes());
            format!("{:x}", hasher.finalize())
        } else {
            id
        }
    }
}

pub trait Storage {
    fn get_id(&self) -> String;
}

impl Storage for test_files_mount::Temporary {
    fn get_id(&self) -> String {
        self.get_id()
    }
}

impl Storage for LongId {
    fn get_id(&self) -> String {
        self.get_id()
    }
}

pub struct Manager {
    mounts: Vec<Arc<Mount>>,
}

impl Manager {
    pub fn new() -> Self {
        Self { mounts: Vec::new() }
    }

    pub fn add_mount(&mut self, mount: Mount) {
        self.mounts.push(Arc::new(mount));
    }

    pub fn find(&self, path: &str) -> Option<Arc<Mount>> {
        // Implement path matching logic
        // Return deepest matching mount
        let path = path.trim_end_matches('/');
        let mut longest_match: Option<(usize, Arc<Mount>)> = None;

        for mount in &self.mounts {
            let mount_point = mount.mount_point.trim_end_matches('/');
            
            if path == mount_point || path.starts_with(&format!("{}/", mount_point)) || 
               (mount_point == "" && path.starts_with('/')) {
                let current_len = mount_point.len();
                if let Some((prev_len, _)) = longest_match {
                    if current_len > prev_len {
                        longest_match = Some((current_len, Arc::clone(mount)));
                    }
                } else {
                    longest_match = Some((current_len, Arc::clone(mount)));
                }
            }
        }

        longest_match.map(|(_, mount)| mount)
    }

    pub fn find_in(&self, path: &str) -> Vec<Arc<Mount>> {
        // Find all mounts that are direct children of the given path
        let path = if path.ends_with('/') { path.to_string() } else { format!("{}/", path) };
        
        self.mounts
            .iter()
            .filter(|mount| {
                let mount_point = &mount.mount_point;
                if mount_point == "/" || mount_point == &path {
                    return false;
                }
                
                mount_point.starts_with(&path) && 
                mount_point[path.len()..].matches('/').count() == 0
            })
            .map(Arc::clone)
            .collect()
    }

    pub fn find_by_storage_id(&self, id: &str) -> Vec<Arc<Mount>> {
        self.mounts
            .iter()
            .filter(|mount| {
                let storage_id = mount.get_storage_id();
                storage_id == id || 
                mount.storage.get_id() == id || 
                {
                    let mut hasher = Md5::new();
                    hasher.update(mount.storage.get_id().as_bytes());
                    format!("{:x}", hasher.finalize()) == id
                }
            })
            .map(Arc::clone)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_find() {
        let mut manager = Manager::new();
        assert!(manager.find("/").is_none());

        let root_mount = Mount::new(
            test_files_mount::Temporary::new(HashMap::new()),
            "/".to_string(),
        );
        manager.add_mount(root_mount);
        
        assert!(manager.find("/").is_some());
        assert!(manager.find("/foo/bar").is_some());

        let storage = test_files_mount::Temporary::new(HashMap::new());
        let mount1 = Mount::new(storage, "/foo".to_string());
        manager.add_mount(mount1);
        
        assert!(manager.find("/").is_some());
        assert_eq!(manager.find("/foo/bar").unwrap().mount_point, "/foo");

        assert_eq!(manager.find_in("/").len(), 1);
        
        let mount2 = Mount::new(
            test_files_mount::Temporary::new(HashMap::new()),
            "/bar".to_string(),
        );
        manager.add_mount(mount2);
        
        assert_eq!(manager.find_in("/").len(), 2);

        let id = manager.find("/foo").unwrap().get_storage_id();
        assert_eq!(manager.find_by_storage_id(&id).len(), 1);

        let storage2 = test_files_mount::Temporary::new(HashMap::new());
        let mount3 = Mount::new(storage2, "/foo/bar".to_string());
        manager.add_mount(mount3);
        
        assert_eq!(manager.find_by_storage_id(&id).len(), 2);
    }

    #[test]
    fn test_long() {
        let mut manager = Manager::new();
        
        let storage = LongId::new(HashMap::new());
        let mount = Mount::new(storage, "/foo".to_string());
        
        let id = mount.get_storage_id();
        let storage_id = mount.storage.get_id();
        
        manager.add_mount(mount);

        assert_eq!(manager.find_by_storage_id(&id).len(), 1);
        assert_eq!(manager.find_by_storage_id(&storage_id).len(), 1);
        
        let mut hasher = Md5::new();
        hasher.update(storage_id.as_bytes());
        let md5_id = format!("{:x}", hasher.finalize());
        
        assert_eq!(manager.find_by_storage_id(&md5_id).len(), 1);
    }
}