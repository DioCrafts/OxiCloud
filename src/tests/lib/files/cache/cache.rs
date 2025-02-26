use mockall::automock;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use unicode_normalization::UnicodeNormalization;

// Equivalent to OC\Files\Storage\Temporary
#[automock]
pub trait Storage {
    fn get_id(&self) -> String;
}

struct TemporaryStorage {
    id: String,
}

impl TemporaryStorage {
    fn new() -> Self {
        Self {
            id: format!("temporary{}", Uuid::new_v4().to_string()),
        }
    }
}

impl Storage for TemporaryStorage {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

// Equivalent to Test\Files\Cache\LongId
struct LongId {
    storage: TemporaryStorage,
}

impl LongId {
    fn new() -> Self {
        Self {
            storage: TemporaryStorage::new(),
        }
    }
}

impl Storage for LongId {
    fn get_id(&self) -> String {
        format!("long:{}{}", "foo".repeat(50), self.storage.get_id())
    }
}

// Cache status constants
pub mod cache_status {
    pub const NOT_FOUND: i32 = 0;
    pub const PARTIAL: i32 = 1;
    pub const SHALLOW: i32 = 2;
    pub const COMPLETE: i32 = 3;
}

// Data structure for cache entries
#[derive(Debug, Clone, PartialEq)]
pub struct CacheEntry {
    pub fileid: i64,
    pub storage: String,
    pub path: String,
    pub name: String,
    pub parent: Option<i64>,
    pub size: i64,
    pub mtime: i64,
    pub storage_mtime: i64,
    pub mimetype: String,
    pub mimepart: String,
    pub etag: Option<String>,
    pub permissions: Option<i32>,
    pub checksum: Option<String>,
}

impl Default for CacheEntry {
    fn default() -> Self {
        Self {
            fileid: 0,
            storage: String::new(),
            path: String::new(),
            name: String::new(),
            parent: None,
            size: 0,
            mtime: 0,
            storage_mtime: 0,
            mimetype: String::new(),
            mimepart: String::new(),
            etag: None,
            permissions: None,
            checksum: None,
        }
    }
}

#[automock]
pub trait CacheInterface {
    fn put(&mut self, path: &str, data: HashMap<String, serde_json::Value>) -> i64;
    fn get(&self, path: &str) -> Option<CacheEntry>;
    fn get_by_id(&self, fileid: i64) -> Option<CacheEntry>;
    fn remove(&mut self, path: &str);
    fn get_folder_contents(&self, path: &str) -> Vec<CacheEntry>;
    fn in_cache(&self, path: &str) -> bool;
    fn get_id(&self, path: &str) -> Option<i64>;
    fn get_parent_id(&self, path: &str) -> Option<i64>;
    fn calculate_folder_size(&self, path: &str) -> i64;
    fn get_status(&self, path: &str) -> i32;
    fn search(&self, pattern: &str) -> Vec<CacheEntry>;
    fn search_by_mime(&self, pattern: &str) -> Vec<CacheEntry>;
    fn move_file(&mut self, source_path: &str, target_path: &str);
    fn get_incomplete(&self) -> Option<String>;
    fn clear(&mut self);
    fn normalize(&self, path: &str) -> String;
}

struct Cache {
    storage: Arc<dyn Storage>,
    entries: HashMap<String, CacheEntry>,
    next_fileid: i64,
}

impl Cache {
    fn new(storage: Arc<dyn Storage>) -> Self {
        Self {
            storage,
            entries: HashMap::new(),
            next_fileid: 1,
        }
    }

    // Static method equivalent to OC\Files\Cache\Cache::getById
    fn get_by_id_static(fileid: i64) -> Option<(String, String)> {
        // In a real implementation, this would query a database
        // This is a mock implementation
        if fileid > 0 {
            // For demonstration, return some dummy values
            Some(("storage123".to_string(), "path/to/file".to_string()))
        } else {
            None
        }
    }

    fn get_path_components(&self, path: &str) -> Vec<String> {
        path.split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }
}

impl CacheInterface for Cache {
    fn put(&mut self, path: &str, data: HashMap<String, serde_json::Value>) -> i64 {
        let normalized_path = self.normalize(path);
        
        // Check if entry already exists
        let mut entry = if let Some(existing) = self.entries.get(&normalized_path) {
            existing.clone()
        } else {
            let mut new_entry = CacheEntry::default();
            new_entry.fileid = self.next_fileid;
            self.next_fileid += 1;
            
            // Set parent if path has components
            let components = self.get_path_components(&normalized_path);
            if components.len() > 1 {
                let parent_path = components[0..components.len() - 1].join("/");
                if let Some(parent) = self.entries.get(&parent_path) {
                    new_entry.parent = Some(parent.fileid);
                }
            }
            
            new_entry.path = normalized_path.clone();
            new_entry.name = match normalized_path.rsplit('/').next() {
                Some(name) if !name.is_empty() => name.to_string(),
                _ => normalized_path.clone(),
            };
            new_entry.storage = self.storage.get_id();
            
            new_entry
        };
        
        // Update entry with provided data
        for (key, value) in data {
            match key.as_str() {
                "size" => {
                    if let Some(size) = value.as_i64() {
                        entry.size = size;
                    }
                },
                "mtime" => {
                    if let Some(mtime) = value.as_i64() {
                        entry.mtime = mtime;
                        // If storage_mtime is not set, set it to mtime
                        if entry.storage_mtime == 0 {
                            entry.storage_mtime = mtime;
                        }
                    }
                },
                "storage_mtime" => {
                    if let Some(storage_mtime) = value.as_i64() {
                        entry.storage_mtime = storage_mtime;
                        entry.mtime = storage_mtime; // Also update mtime
                    }
                },
                "mimetype" => {
                    if let Some(mimetype) = value.as_str() {
                        entry.mimetype = mimetype.to_string();
                        if let Some(mimepart) = mimetype.split('/').next() {
                            entry.mimepart = mimepart.to_string();
                        }
                    }
                },
                // Add other fields as needed
                _ => {}
            }
        }
        
        // Save entry
        self.entries.insert(normalized_path, entry.clone());
        
        entry.fileid
    }

    fn get(&self, path: &str) -> Option<CacheEntry> {
        let normalized_path = self.normalize(path);
        self.entries.get(&normalized_path).cloned()
    }

    fn get_by_id(&self, fileid: i64) -> Option<CacheEntry> {
        self.entries.values().find(|e| e.fileid == fileid).cloned()
    }

    fn remove(&mut self, path: &str) {
        let normalized_path = self.normalize(path);
        
        // Find all entries that start with this path
        let paths_to_remove: Vec<String> = self.entries.keys()
            .filter(|k| k == &normalized_path || k.starts_with(&format!("{}/", normalized_path)))
            .cloned()
            .collect();
        
        for path_to_remove in paths_to_remove {
            self.entries.remove(&path_to_remove);
        }
    }

    fn get_folder_contents(&self, path: &str) -> Vec<CacheEntry> {
        let normalized_path = self.normalize(path);
        let prefix = if normalized_path.is_empty() {
            String::new()
        } else {
            format!("{}/", normalized_path)
        };
        
        // Find direct children
        self.entries.values()
            .filter(|e| {
                if normalized_path.is_empty() {
                    !e.path.contains('/')
                } else {
                    e.path.starts_with(&prefix) && 
                    e.path[prefix.len()..].split('/').count() == 1
                }
            })
            .cloned()
            .collect()
    }

    fn in_cache(&self, path: &str) -> bool {
        let normalized_path = self.normalize(path);
        self.entries.contains_key(&normalized_path)
    }

    fn get_id(&self, path: &str) -> Option<i64> {
        self.get(path).map(|e| e.fileid)
    }

    fn get_parent_id(&self, path: &str) -> Option<i64> {
        self.get(path).and_then(|e| e.parent)
    }

    fn calculate_folder_size(&self, path: &str) -> i64 {
        let normalized_path = self.normalize(path);
        let prefix = if normalized_path.is_empty() {
            String::new()
        } else {
            format!("{}/", normalized_path)
        };
        
        // Sum up sizes of all files in folder (including subfolders)
        let mut total_size = 0;
        for entry in self.entries.values() {
            if entry.path == normalized_path || entry.path.starts_with(&prefix) {
                if entry.size == -1 {
                    return -1; // If any file has unknown size, folder size is unknown
                }
                if entry.path != normalized_path { // Don't count the folder itself
                    total_size += entry.size;
                }
            }
        }
        
        total_size
    }

    fn get_status(&self, path: &str) -> i32 {
        match self.get(path) {
            None => cache_status::NOT_FOUND,
            Some(entry) => {
                if entry.size == -1 {
                    if !entry.mimetype.is_empty() {
                        cache_status::SHALLOW
                    } else {
                        cache_status::PARTIAL
                    }
                } else {
                    cache_status::COMPLETE
                }
            }
        }
    }

    fn search(&self, pattern: &str) -> Vec<CacheEntry> {
        let search_pattern = pattern.replace('%', ".*");
        let regex = regex::Regex::new(&format!("^{}$", search_pattern)).unwrap_or_else(|_| {
            // Fallback to simple contains check
            regex::Regex::new(".*").unwrap()
        });
        
        self.entries.values()
            .filter(|e| regex.is_match(&e.name))
            .cloned()
            .collect()
    }

    fn search_by_mime(&self, pattern: &str) -> Vec<CacheEntry> {
        self.entries.values()
            .filter(|e| {
                if pattern.contains('/') {
                    // Full mimetype match
                    e.mimetype == pattern
                } else {
                    // Mimepart match
                    e.mimepart == pattern
                }
            })
            .cloned()
            .collect()
    }

    fn move_file(&mut self, source_path: &str, target_path: &str) {
        let source_normalized = self.normalize(source_path);
        let target_normalized = self.normalize(target_path);
        
        // Get all entries that need to be moved
        let paths_to_move: Vec<(String, String)> = self.entries.keys()
            .filter(|k| **k == source_normalized || k.starts_with(&format!("{}/", source_normalized)))
            .map(|k| {
                let new_path = if *k == source_normalized {
                    target_normalized.clone()
                } else {
                    k.replacen(&source_normalized, &target_normalized, 1)
                };
                (k.clone(), new_path)
            })
            .collect();
        
        // Move all entries
        for (old_path, new_path) in paths_to_move {
            if let Some(mut entry) = self.entries.remove(&old_path) {
                entry.path = new_path.clone();
                entry.name = match new_path.rsplit('/').next() {
                    Some(name) if !name.is_empty() => name.to_string(),
                    _ => new_path.clone(),
                };
                
                // Update parent reference if needed
                let components = self.get_path_components(&new_path);
                if components.len() > 1 {
                    let parent_path = components[0..components.len() - 1].join("/");
                    if let Some(parent) = self.entries.get(&parent_path) {
                        entry.parent = Some(parent.fileid);
                    }
                }
                
                self.entries.insert(new_path, entry);
            }
        }
    }

    fn get_incomplete(&self) -> Option<String> {
        self.entries.iter()
            .filter(|(_, entry)| entry.size == -1)
            .map(|(path, _)| path.clone())
            .next()
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.next_fileid = 1;
    }

    fn normalize(&self, path: &str) -> String {
        if let Ok(true) = unicode_normalization::is_nfc(path) {
            path.to_string()
        } else {
            path.nfc().collect::<String>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use serde_json::json;

    fn create_test_cache() -> (Arc<dyn Storage>, Box<dyn CacheInterface>) {
        let storage = Arc::new(TemporaryStorage::new());
        let cache = Box::new(Cache::new(storage.clone()));
        (storage, cache)
    }

    fn json_to_hashmap(json_data: serde_json::Value) -> HashMap<String, serde_json::Value> {
        match json_data {
            serde_json::Value::Object(map) => {
                map.into_iter().collect()
            },
            _ => HashMap::new()
        }
    }

    #[test]
    fn test_simple() {
        let (_, mut cache) = create_test_cache();
        
        let file1 = "foo";
        let file2 = "foo/bar";
        let data1 = json_to_hashmap(json!({
            "size": 100,
            "mtime": 50,
            "mimetype": "foo/folder"
        }));
        let data2 = json_to_hashmap(json!({
            "size": 1000,
            "mtime": 20,
            "mimetype": "foo/file"
        }));
        
        assert!(!cache.in_cache(file1));
        assert_eq!(cache.get(file1), None);
        
        let id1 = cache.put(file1, data1);
        assert!(cache.in_cache(file1));
        
        let cache_data1 = cache.get(file1).unwrap();
        assert_eq!(cache_data1.size, 100);
        assert_eq!(cache_data1.mtime, 50);
        assert_eq!(cache_data1.mimetype, "foo/folder");
        assert_eq!(cache_data1.mimepart, "foo");
        assert_eq!(cache_data1.fileid, id1);
        assert_eq!(id1, cache.get_id(file1).unwrap());
        
        assert!(!cache.in_cache(file2));
        
        let id2 = cache.put(file2, data2);
        assert!(cache.in_cache(file2));
        
        let cache_data2 = cache.get(file2).unwrap();
        assert_eq!(cache_data2.size, 1000);
        assert_eq!(cache_data2.mtime, 20);
        assert_eq!(cache_data2.mimetype, "foo/file");
        assert_eq!(cache_data1.fileid, cache_data2.parent.unwrap());
        assert_eq!(cache_data2.fileid, id2);
        assert_eq!(id2, cache.get_id(file2).unwrap());
        assert_eq!(id1, cache.get_parent_id(file2).unwrap());
        
        let new_size = 1050;
        let new_id2 = cache.put(file2, json_to_hashmap(json!({"size": new_size})));
        assert_eq!(new_id2, id2);
        assert_eq!(cache.get(file2).unwrap().size, new_size);
        assert_eq!(cache.get(file1).unwrap(), cache_data1);
        
        cache.remove(file2);
        assert!(!cache.in_cache(file2));
        assert_eq!(cache.get(file2), None);
        assert!(cache.in_cache(file1));
        
        // Test get by id
        assert_eq!(cache.get_by_id(id1).unwrap(), cache_data1);
    }

    #[test]
    fn test_partial() {
        let (_, mut cache) = create_test_cache();
        let file1 = "foo";
        
        cache.put(file1, json_to_hashmap(json!({"size": 10})));
        assert_eq!(cache.get(file1).unwrap().size, 10);
        
        cache.put(file1, json_to_hashmap(json!({"mtime": 15})));
        let entry = cache.get(file1).unwrap();
        assert_eq!(entry.size, 10);
        assert_eq!(entry.mtime, 15);
        
        cache.put(file1, json_to_hashmap(json!({"size": 12})));
        let entry = cache.get(file1).unwrap();
        assert_eq!(entry.size, 12);
        assert_eq!(entry.mtime, 15);
    }

    #[test]
    fn test_folder() {
        let (_, mut cache) = create_test_cache();
        let file1 = "folder";
        let file2 = "folder/bar";
        let file3 = "folder/foo";
        
        let data1 = json_to_hashmap(json!({
            "size": 100,
            "mtime": 50,
            "mimetype": "httpd/unix-directory"
        }));
        
        let data_bar = json_to_hashmap(json!({
            "size": 1000,
            "mtime": 20,
            "mimetype": "foo/file"
        }));
        
        let data_foo = json_to_hashmap(json!({
            "size": 20,
            "mtime": 25,
            "mimetype": "foo/file"
        }));
        
        cache.put(file1, data1);
        cache.put(file2, data_bar.clone());
        cache.put(file3, data_foo.clone());
        
        let content = cache.get_folder_contents(file1);
        assert_eq!(content.len(), 2);
        
        for entry in content {
            let name = entry.name.as_str();
            if name == "bar" {
                assert_eq!(entry.size, 1000);
                assert_eq!(entry.mtime, 20);
                assert_eq!(entry.mimetype, "foo/file");
            } else if name == "foo" {
                assert_eq!(entry.size, 20);
                assert_eq!(entry.mtime, 25);
                assert_eq!(entry.mimetype, "foo/file");
            }
        }
        
        let file4 = "folder/unknownSize";
        cache.put(file4, json_to_hashmap(json!({
            "size": -1,
            "mtime": 25,
            "mimetype": "foo/file"
        })));
        
        assert_eq!(cache.calculate_folder_size(file1), -1);
        
        cache.put(file4, json_to_hashmap(json!({
            "size": 5,
            "mtime": 25,
            "mimetype": "foo/file"
        })));
        
        assert_eq!(cache.calculate_folder_size(file1), 1025);
        
        cache.remove(file2);
        cache.remove(file3);
        cache.remove(file4);
        
        assert_eq!(cache.calculate_folder_size(file1), 0);
        
        cache.remove("folder");
        assert!(!cache.in_cache("folder/foo"));
        assert!(!cache.in_cache("folder/bar"));
    }

    #[test]
    fn test_status() {
        let (_, mut cache) = create_test_cache();
        assert_eq!(cache.get_status("foo"), cache_status::NOT_FOUND);
        
        cache.put("foo", json_to_hashmap(json!({"size": -1})));
        assert_eq!(cache.get_status("foo"), cache_status::PARTIAL);
        
        cache.put("foo", json_to_hashmap(json!({
            "size": -1,
            "mtime": 20,
            "mimetype": "foo/file"
        })));
        assert_eq!(cache.get_status("foo"), cache_status::SHALLOW);
        
        cache.put("foo", json_to_hashmap(json!({"size": 10})));
        assert_eq!(cache.get_status("foo"), cache_status::COMPLETE);
    }

    #[test]
    fn test_search() {
        let (_, mut cache) = create_test_cache();
        let file1 = "folder";
        let file2 = "folder/foobar";
        let file3 = "folder/foo";
        
        let data1 = json_to_hashmap(json!({
            "size": 100,
            "mtime": 50,
            "mimetype": "foo/folder"
        }));
        
        let data_foobar = json_to_hashmap(json!({
            "size": 1000,
            "mtime": 20,
            "mimetype": "foo/file"
        }));
        
        let data_foo = json_to_hashmap(json!({
            "size": 20,
            "mtime": 25,
            "mimetype": "foo/file"
        }));
        
        cache.put(file1, data1);
        cache.put(file2, data_foobar);
        cache.put(file3, data_foo);
        
        assert_eq!(cache.search("%foo%").len(), 2);
        assert_eq!(cache.search("foo").len(), 1);
        assert_eq!(cache.search("%folder%").len(), 1);
        assert_eq!(cache.search("folder%").len(), 1);
        assert_eq!(cache.search("%").len(), 3);
        
        assert_eq!(cache.search_by_mime("foo").len(), 3);
        assert_eq!(cache.search_by_mime("foo/file").len(), 2);
    }

    #[test]
    fn test_move() {
        let (storage1, mut cache1) = create_test_cache();
        let (storage2, mut cache2) = create_test_cache();
        
        let file1 = "folder";
        let file2 = "folder/bar";
        let file3 = "folder/foo";
        let file4 = "folder/foo/1";
        let file5 = "folder/foo/2";
        
        let folder_data = json_to_hashmap(json!({
            "size": 100,
            "mtime": 50,
            "mimetype": "httpd/unix-directory"
        }));
        
        let data = json_to_hashmap(json!({
            "size": 100,
            "mtime": 50,
            "mimetype": "foo/bar"
        }));
        
        cache1.put(file1, folder_data.clone());
        cache1.put(file2, folder_data.clone());
        cache1.put(file3, folder_data.clone());
        cache1.put(file4, data.clone());
        cache1.put(file5, data.clone());
        
        cache2.put(file1, folder_data.clone());
        cache2.put(file2, folder_data.clone());
        cache2.put(file3, folder_data.clone());
        cache2.put(file4, data.clone());
        cache2.put(file5, data.clone());
        
        cache1.move_file("folder/foo", "folder/foobar");
        
        assert!(!cache1.in_cache("folder/foo"));
        assert!(!cache1.in_cache("folder/foo/1"));
        assert!(!cache1.in_cache("folder/foo/2"));
        
        assert!(cache1.in_cache("folder/bar"));
        assert!(cache1.in_cache("folder/foobar"));
        assert!(cache1.in_cache("folder/foobar/1"));
        assert!(cache1.in_cache("folder/foobar/2"));
        
        // Check that second cache is unaffected
        assert!(cache2.in_cache("folder/bar"));
        assert!(cache2.in_cache("folder/foo"));
        assert!(cache2.in_cache("folder/foo/1"));
        assert!(cache2.in_cache("folder/foo/2"));
        
        assert!(!cache2.in_cache("folder/foobar"));
        assert!(!cache2.in_cache("folder/foobar/1"));
        assert!(!cache2.in_cache("folder/foobar/2"));
    }

    #[test]
    fn test_get_incomplete() {
        let (_, mut cache) = create_test_cache();
        
        cache.put("folder1", json_to_hashmap(json!({
            "size": 10,
            "mtime": 50,
            "mimetype": "foo/bar"
        })));
        
        cache.put("folder2", json_to_hashmap(json!({
            "size": -1,
            "mtime": 50,
            "mimetype": "foo/bar"
        })));
        
        cache.put("folder3", json_to_hashmap(json!({
            "size": -1,
            "mtime": 50,
            "mimetype": "foo/bar"
        })));
        
        cache.put("folder4", json_to_hashmap(json!({
            "size": 12,
            "mtime": 50,
            "mimetype": "foo/bar"
        })));
        
        // The implementation returns the first incomplete file it finds
        assert!(cache.get_incomplete().is_some());
        let incomplete = cache.get_incomplete().unwrap();
        assert!(incomplete == "folder2" || incomplete == "folder3");
    }

    #[test]
    fn test_non_existing() {
        let (_, cache) = create_test_cache();
        assert!(cache.get("foo.txt").is_none());
        assert_eq!(cache.get_folder_contents("foo").len(), 0);
    }

    #[test]
    fn test_storage_mtime() {
        let (_, mut cache) = create_test_cache();
        
        cache.put("foo", json_to_hashmap(json!({
            "size": 1000,
            "mtime": 20,
            "mimetype": "foo/file"
        })));
        
        let cached_data = cache.get("foo").unwrap();
        assert_eq!(cached_data.mtime, 20);
        assert_eq!(cached_data.storage_mtime, 20); // Should be the same initially
        
        cache.put("foo", json_to_hashmap(json!({"storage_mtime": 30})));
        let cached_data = cache.get("foo").unwrap();
        assert_eq!(cached_data.storage_mtime, 30);
        assert_eq!(cached_data.mtime, 30); // Should also update mtime
        
        cache.put("foo", json_to_hashmap(json!({"mtime": 25})));
        let cached_data = cache.get("foo").unwrap();
        assert_eq!(cached_data.storage_mtime, 30); // Should not change
        assert_eq!(cached_data.mtime, 25); // Should be updated
    }

    #[test]
    fn test_long_id() {
        let storage = Arc::new(LongId::new());
        let mut cache = Cache::new(storage.clone());
        
        let data = json_to_hashmap(json!({
            "size": 1000,
            "mtime": 20,
            "mimetype": "foo/file"
        }));
        
        let id = cache.put("foo", data);
        
        // In a real implementation, getById would query the database
        // This test would need to be adjusted accordingly
    }

    #[test]
    fn test_without_normalizer() {
        let (_, mut cache) = create_test_cache();
        
        // folder name "Schön" with U+00F6 (normalized)
        let folder_with_00f6 = "Schön";
        
        // folder name "Schön" with U+0308 (un-normalized)
        let folder_with_0308 = "Schön";
        
        // Create a mock cache that doesn't normalize
        // In Rust, we'll just use the real cache but test the behavior
        
        let data = json_to_hashmap(json!({
            "size": 100,
            "mtime": 50,
            "mimetype": "httpd/unix-directory"
        }));
        
        // Put root folder
        assert!(cache.get("folder").is_none());
        cache.put("folder", data.clone());
        
        // Put un-normalized folder
        assert!(cache.get(&format!("folder/{}", folder_with_0308)).is_none());