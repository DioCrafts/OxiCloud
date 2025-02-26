//! Metadata cache for the filesystem
//!
//! Don't use this struct directly if you need to get metadata, use Filesystem::get_file_info instead

use std::collections::HashMap;
use std::sync::OnceLock;
use md5::{Md5, Digest};

// Define the state of the file in the cache
pub const NOT_FOUND: i32 = 0;
pub const PARTIAL: i32 = 1; // only partial data available, file not cached in the database
pub const SHALLOW: i32 = 2; // folder in cache, but not all child files are completely scanned
pub const COMPLETE: i32 = 3;

static MIMETYPE_IDS: OnceLock<HashMap<String, i32>> = OnceLock::new();
static MIMETYPES: OnceLock<HashMap<i32, String>> = OnceLock::new();

pub struct Cache {
    /// Partial data for the cache
    partial: HashMap<String, HashMap<String, serde_json::Value>>,
    
    /// Storage ID
    storage_id: String,
    
    /// Storage cache
    storage_cache: crate::files::cache::Storage,
}

impl Cache {
    /// Create a new cache for the given storage
    pub fn new<S>(storage: S) -> Self 
    where 
        S: Into<String> + AsRef<str> + Clone,
    {
        let storage_id = match storage.as_ref().parse::<crate::files::Storage>() {
            Ok(storage_obj) => storage_obj.get_id(),
            Err(_) => storage.clone().into(),
        };

        let storage_id = if storage_id.len() > 64 {
            format!("{:x}", md5::compute(storage_id.as_bytes()))
        } else {
            storage_id
        };

        let storage_cache = crate::files::cache::Storage::new(storage);

        Self {
            partial: HashMap::new(),
            storage_id,
            storage_cache,
        }
    }

    /// Get the numeric storage ID
    pub fn get_numeric_storage_id(&self) -> i64 {
        self.storage_cache.get_numeric_id()
    }

    /// Normalize mimetypes and get the mimetype ID
    pub fn get_mimetype_id(&self, mime: &str) -> i32 {
        let mime = if mime.is_empty() {
            "application/octet-stream"
        } else {
            mime
        };

        if MIMETYPE_IDS.get().is_none() {
            self.load_mimetypes();
        }

        let mimetype_ids = MIMETYPE_IDS.get().unwrap();
        
        if let Some(id) = mimetype_ids.get(mime) {
            return *id;
        }

        // If not found, insert it into database
        match crate::db::execute_audited(
            "INSERT INTO `*PREFIX*mimetypes`(`mimetype`) VALUES(?)",
            &[&mime],
        ) {
            Ok(_) => {
                let id = crate::db::insert_id("*PREFIX*mimetypes") as i32;
                
                // Update our in-memory cache
                let mut mimetype_ids = mimetype_ids.clone();
                mimetype_ids.insert(mime.to_string(), id);
                MIMETYPE_IDS.set(mimetype_ids).unwrap_or_default();
                
                let mut mimetypes = MIMETYPES.get().unwrap_or(&HashMap::new()).clone();
                mimetypes.insert(id, mime.to_string());
                MIMETYPES.set(mimetypes).unwrap_or_default();
                
                id
            }
            Err(e) => {
                crate::log::write("core", &format!("Exception during mimetype insertion: {}", e), crate::log::DEBUG);
                -1
            }
        }
    }

    /// Get mimetype from ID
    pub fn get_mimetype(&self, id: i32) -> Option<String> {
        if MIMETYPES.get().is_none() {
            self.load_mimetypes();
        }

        MIMETYPES.get()
            .unwrap_or(&HashMap::new())
            .get(&id)
            .cloned()
    }

    /// Load mimetypes from database
    pub fn load_mimetypes(&self) {
        let result = crate::db::execute_audited(
            "SELECT `id`, `mimetype` FROM `*PREFIX*mimetypes`",
            &[],
        );

        if let Ok(result) = result {
            let mut mimetype_ids = HashMap::new();
            let mut mimetypes = HashMap::new();

            for row in result.rows().unwrap_or_default() {
                let id: i32 = row.get("id").unwrap_or(0);
                let mimetype: String = row.get("mimetype").unwrap_or_default();
                
                mimetype_ids.insert(mimetype.clone(), id);
                mimetypes.insert(id, mimetype);
            }

            MIMETYPE_IDS.set(mimetype_ids).unwrap_or_default();
            MIMETYPES.set(mimetypes).unwrap_or_default();
        }
    }

    /// Get the stored metadata of a file or folder
    pub fn get<T: Into<String>>(&self, file: T) -> Option<HashMap<String, serde_json::Value>> {
        let (where_clause, params): (String, Vec<Box<dyn crate::db::ToSql>>) = match file.into() {
            s if s.is_empty() || s.parse::<i64>().is_err() => {
                // It's a path string
                let normalized = self.normalize(&s);
                let path_hash = format!("{:x}", md5::compute(normalized.as_bytes()));
                
                (
                    "WHERE `storage` = ? AND `path_hash` = ?".to_string(),
                    vec![
                        Box::new(self.get_numeric_storage_id()),
                        Box::new(path_hash),
                    ],
                )
            },
            s => {
                // It's a file ID
                let file_id = s.parse::<i64>().unwrap_or_default();
                (
                    "WHERE `fileid` = ?".to_string(),
                    vec![Box::new(file_id)],
                )
            }
        };

        let sql = format!(
            "SELECT `fileid`, `storage`, `path`, `parent`, `name`, `mimetype`, `mimepart`, `size`, `mtime`, \
            `storage_mtime`, `encrypted`, `unencrypted_size`, `etag` FROM `*PREFIX*filecache` {}",
            where_clause
        );

        let result = crate::db::execute_audited(&sql, &params);
        
        if let Ok(result) = result {
            if let Some(row) = result.rows().unwrap_or_default().next() {
                let mut data = HashMap::new();
                
                // Convert DB row to HashMap
                data.insert("fileid".to_string(), serde_json::to_value(row.get::<_, i64>("fileid").unwrap_or_default()).unwrap());
                data.insert("storage".to_string(), serde_json::to_value(self.storage_id.clone()).unwrap());
                data.insert("path".to_string(), serde_json::to_value(row.get::<_, String>("path").unwrap_or_default()).unwrap());
                data.insert("parent".to_string(), serde_json::to_value(row.get::<_, i64>("parent").unwrap_or_default()).unwrap());
                data.insert("name".to_string(), serde_json::to_value(row.get::<_, String>("name").unwrap_or_default()).unwrap());
                
                let mimetype_id = row.get::<_, i32>("mimetype").unwrap_or_default();
                let mimepart_id = row.get::<_, i32>("mimepart").unwrap_or_default();
                data.insert("mimetype".to_string(), serde_json::to_value(self.get_mimetype(mimetype_id).unwrap_or_default()).unwrap());
                data.insert("mimepart".to_string(), serde_json::to_value(self.get_mimetype(mimepart_id).unwrap_or_default()).unwrap());
                
                data.insert("size".to_string(), serde_json::to_value(row.get::<_, i64>("size").unwrap_or_default()).unwrap());
                data.insert("mtime".to_string(), serde_json::to_value(row.get::<_, i64>("mtime").unwrap_or_default()).unwrap());
                
                let storage_mtime = row.get::<_, i64>("storage_mtime").unwrap_or_default();
                let mtime = row.get::<_, i64>("mtime").unwrap_or_default();
                data.insert("storage_mtime".to_string(), serde_json::to_value(if storage_mtime == 0 { mtime } else { storage_mtime }).unwrap());
                
                data.insert("encrypted".to_string(), serde_json::to_value(row.get::<_, bool>("encrypted").unwrap_or_default()).unwrap());
                data.insert("unencrypted_size".to_string(), serde_json::to_value(row.get::<_, i64>("unencrypted_size").unwrap_or_default()).unwrap());
                data.insert("etag".to_string(), serde_json::to_value(row.get::<_, String>("etag").unwrap_or_default()).unwrap());
                
                return Some(data);
            }
        }

        // Check if we have partial data
        if let Some(path) = match file.into() {
            s if s.is_empty() || s.parse::<i64>().is_err() => Some(s),
            _ => None,
        } {
            if let Some(partial_data) = self.partial.get(&path) {
                return Some(partial_data.clone());
            }
        }

        None
    }

    /// Get the metadata of all files stored in folder
    pub fn get_folder_contents(&self, folder: &str) -> Vec<HashMap<String, serde_json::Value>> {
        let file_id = self.get_id(folder);
        
        if file_id <= -1 {
            return Vec::new();
        }
        
        let sql = "SELECT `fileid`, `storage`, `path`, `parent`, `name`, `mimetype`, `mimepart`, `size`, `mtime`, \
                  `storage_mtime`, `encrypted`, `unencrypted_size`, `etag` \
                  FROM `*PREFIX*filecache` WHERE `parent` = ? ORDER BY `name` ASC";
        
        let result = crate::db::execute_audited(sql, &[&file_id]);
        
        if let Ok(result) = result {
            let mut files = Vec::new();
            
            for row in result.rows().unwrap_or_default() {
                let mut file = HashMap::new();
                
                file.insert("fileid".to_string(), serde_json::to_value(row.get::<_, i64>("fileid").unwrap_or_default()).unwrap());
                file.insert("storage".to_string(), serde_json::to_value(self.storage_id.clone()).unwrap());
                file.insert("path".to_string(), serde_json::to_value(row.get::<_, String>("path").unwrap_or_default()).unwrap());
                file.insert("parent".to_string(), serde_json::to_value(row.get::<_, i64>("parent").unwrap_or_default()).unwrap());
                file.insert("name".to_string(), serde_json::to_value(row.get::<_, String>("name").unwrap_or_default()).unwrap());
                
                let mimetype_id = row.get::<_, i32>("mimetype").unwrap_or_default();
                let mimepart_id = row.get::<_, i32>("mimepart").unwrap_or_default();
                file.insert("mimetype".to_string(), serde_json::to_value(self.get_mimetype(mimetype_id).unwrap_or_default()).unwrap());
                file.insert("mimepart".to_string(), serde_json::to_value(self.get_mimetype(mimepart_id).unwrap_or_default()).unwrap());
                
                file.insert("size".to_string(), serde_json::to_value(row.get::<_, i64>("size").unwrap_or_default()).unwrap());
                file.insert("mtime".to_string(), serde_json::to_value(row.get::<_, i64>("mtime").unwrap_or_default()).unwrap());
                
                let storage_mtime = row.get::<_, i64>("storage_mtime").unwrap_or_default();
                let mtime = row.get::<_, i64>("mtime").unwrap_or_default();
                file.insert("storage_mtime".to_string(), serde_json::to_value(if storage_mtime == 0 { mtime } else { storage_mtime }).unwrap());
                
                file.insert("encrypted".to_string(), serde_json::to_value(row.get::<_, bool>("encrypted").unwrap_or_default()).unwrap());
                file.insert("unencrypted_size".to_string(), serde_json::to_value(row.get::<_, i64>("unencrypted_size").unwrap_or_default()).unwrap());
                file.insert("etag".to_string(), serde_json::to_value(row.get::<_, String>("etag").unwrap_or_default()).unwrap());
                
                files.push(file);
            }
            
            return files;
        }
        
        Vec::new()
    }

    /// Store meta data for a file or folder
    pub fn put(&mut self, file: &str, mut data: HashMap<String, serde_json::Value>) -> i64 {
        let id = self.get_id(file);
        
        if id > -1 {
            self.update(id, &data);
            return id;
        }
        
        // Normalize file
        let file = self.normalize(file);
        
        // Add any saved partial data
        if let Some(partial_data) = self.partial.get(&file) {
            for (key, value) in partial_data {
                if !data.contains_key(key) {
                    data.insert(key.clone(), value.clone());
                }
            }
            self.partial.remove(&file);
        }
        
        // Check required fields
        let required_fields = ["size", "mtime", "mimetype"];
        for field in &required_fields {
            if !data.contains_key(*field) {
                // Data not complete, save as partial and return
                self.partial.insert(file, data);
                return -1;
            }
        }
        
        data.insert("path".to_string(), serde_json::to_value(&file).unwrap());
        data.insert("parent".to_string(), serde_json::to_value(self.get_parent_id(&file)).unwrap());
        data.insert("name".to_string(), serde_json::to_value(crate::util::basename(&file)).unwrap());
        
        let (query_parts, params) = self.build_parts(&data);
        let mut query_parts = query_parts;
        let mut params = params;
        
        query_parts.push("`storage`".to_string());
        params.push(Box::new(self.get_numeric_storage_id()));
        
        let values_placeholder = vec!["?"; query_parts.len()].join(", ");
        
        let sql = format!(
            "INSERT INTO `*PREFIX*filecache` ({}) VALUES ({})",
            query_parts.join(", "),
            values_placeholder
        );
        
        if let Ok(_) = crate::db::execute_audited(&sql, &params) {
            return crate::db::insert_id("*PREFIX*filecache");
        }
        
        -1
    }

    /// Update the metadata in the cache
    pub fn update(&self, id: i64, data: &HashMap<String, serde_json::Value>) {
        let mut updated_data = data.clone();
        
        // Normalize path if it exists
        if let Some(path) = data.get("path") {
            if let Some(path_str) = path.as_str() {
                updated_data.insert("path".to_string(), serde_json::to_value(self.normalize(path_str)).unwrap());
            }
        }
        
        // Normalize name if it exists
        if let Some(name) = data.get("name") {
            if let Some(name_str) = name.as_str() {
                updated_data.insert("name".to_string(), serde_json::to_value(self.normalize(name_str)).unwrap());
            }
        }
        
        let (query_parts, mut params) = self.build_parts(&updated_data);
        params.push(Box::new(id));
        
        let sql = format!(
            "UPDATE `*PREFIX*filecache` SET {} WHERE `fileid` = ?",
            query_parts.iter().map(|part| format!("{} = ?", part)).collect::<Vec<_>>().join(", ")
        );
        
        let _ = crate::db::execute_audited(&sql, &params);
    }

    /// Extract query parts and params array from data map
    fn build_parts(&self, data: &HashMap<String, serde_json::Value>) -> (Vec<String>, Vec<Box<dyn crate::db::ToSql>>) {
        let fields = ["path", "parent", "name", "mimetype", "size", "mtime", "storage_mtime", "encrypted", "unencrypted_size", "etag"];
        let mut params: Vec<Box<dyn crate::db::ToSql>> = Vec::new();
        let mut query_parts = Vec::new();
        
        for (name, value) in data {
            if fields.contains(&name.as_str()) {
                if name == "path" {
                    if let Some(path) = value.as_str() {
                        let path_hash = format!("{:x}", md5::compute(path.as_bytes()));
                        params.push(Box::new(path_hash));
                        query_parts.push("`path_hash`".to_string());
                    }
                } else if name == "mimetype" {
                    if let Some(mimetype) = value.as_str() {
                        if let Some(pos) = mimetype.find('/') {
                            let mimepart = &mimetype[0..pos];
                            params.push(Box::new(self.get_mimetype_id(mimepart)));
                            query_parts.push("`mimepart`".to_string());
                        }
                        params.push(Box::new(self.get_mimetype_id(mimetype)));
                        query_parts.push("`mimetype`".to_string());
                    }
                } else if name == "storage_mtime" {
                    if !data.contains_key("mtime") {
                        if let Some(mtime) = value.as_i64() {
                            params.push(Box::new(mtime));
                            query_parts.push("`mtime`".to_string());
                        }
                    }
                    if let Some(storage_mtime) = value.as_i64() {
                        params.push(Box::new(storage_mtime));
                        query_parts.push("`storage_mtime`".to_string());
                    }
                } else if name == "encrypted" {
                    if let Some(encrypted) = value.as_bool() {
                        params.push(Box::new(if encrypted { 1 } else { 0 }));
                        query_parts.push("`encrypted`".to_string());
                    }
                } else {
                    if let Some(val) = value.as_i64() {
                        params.push(Box::new(val));
                        query_parts.push(format!("`{}`", name));
                    } else if let Some(val) = value.as_str() {
                        params.push(Box::new(val.to_string()));
                        query_parts.push(format!("`{}`", name));
                    } else if let Some(val) = value.as_bool() {
                        params.push(Box::new(val));
                        query_parts.push(format!("`{}`", name));
                    }
                }
            }
        }
        
        (query_parts, params)
    }

    /// Get the file id for a file
    pub fn get_id(&self, file: &str) -> i64 {
        // Normalize file
        let file = self.normalize(file);
        
        let path_hash = format!("{:x}", md5::compute(file.as_bytes()));
        
        let sql = "SELECT `fileid` FROM `*PREFIX*filecache` WHERE `storage` = ? AND `path_hash` = ?";
        let result = crate::db::execute_audited(sql, &[&self.get_numeric_storage_id(), &path_hash]);
        
        if let Ok(result) = result {
            if let Some(row) = result.rows().unwrap_or_default().next() {
                return row.get::<_, i64>("fileid").unwrap_or(-1);
            }
        }
        
        -1
    }

    /// Get the id of the parent folder of a file
    pub fn get_parent_id(&self, file: &str) -> i64 {
        if file.is_empty() {
            return -1;
        }
        
        let parent = std::path::Path::new(file)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "".to_string());
            
        if parent == "." {
            return self.get_id("");
        }
        
        self.get_id(&parent)
    }

    /// Check if a file is available in the cache
    pub fn in_cache(&self, file: &str) -> bool {
        self.get_id(file) != -1
    }

    /// Remove a file or folder from the cache
    pub fn remove(&self, file: &str) {
        if let Some(entry) = self.get(file) {
            if let Some(serde_json::Value::String(mimetype)) = entry.get("mimetype") {
                if mimetype == "httpd/unix-directory" {
                    let children = self.get_folder_contents(file);
                    for child in children {
                        if let Some(serde_json::Value::String(path)) = child.get("path") {
                            self.remove(path);
                        }
                    }
                }
            }
            
            if let Some(serde_json::Value::Number(fileid)) = entry.get("fileid") {
                if let Some(fileid) = fileid.as_i64() {
                    let sql = "DELETE FROM `*PREFIX*filecache` WHERE `fileid` = ?";
                    let _ = crate::db::execute_audited(sql, &[&fileid]);
                    
                    let permissions_cache = crate::files::cache::Permissions::new(&self.storage_id);
                    permissions_cache.remove(fileid);
                }
            }
        }
    }

    /// Move a file or folder in the cache
    pub fn move_(&self, source: &str, target: &str) {
        // Normalize source and target
        let source = self.normalize(source);
        let target = self.normalize(target);
        
        let source_data = self.get(&source).unwrap_or_default();
        let source_id = source_data.get("fileid")
            .and_then(|v| v.as_i64())
            .unwrap_or(-1);
            
        let new_parent_id = self.get_parent_id(&target);
        
        if let Some(serde_json::Value::String(mimetype)) = source_data.get("mimetype") {
            if mimetype == "httpd/unix-directory" {
                // Find all child entries
                let sql = "SELECT `path`, `fileid` FROM `*PREFIX*filecache` WHERE `storage` = ? AND `path` LIKE ?";
                let like_pattern = format!("{}/%", source);
                let result = crate::db::execute_audited(sql, &[&self.get_numeric_storage_id(), &like_pattern]);
                
                if let Ok(result) = result {
                    let source_len = source.len();
                    let update_sql = "UPDATE `*PREFIX*filecache` SET `path` = ?, `path_hash` = ? WHERE `fileid` = ?";
                    
                    for row in result.rows().unwrap_or_default() {
                        let path: String = row.get("path").unwrap_or_default();
                        let fileid: i64 = row.get("fileid").unwrap_or_default();
                        
                        let target_path = format!("{}{}", target, &path[source_len..]);
                        let target_path_hash = format!("{:x}", md5::compute(target_path.as_bytes()));
                        
                        let _ = crate::db::execute_audited(update_sql, &[&target_path, &target_path_hash, &fileid]);
                    }
                }
            }
        }
        
        let sql = "UPDATE `*PREFIX*filecache` SET `path` = ?, `path_hash` = ?, `name` = ?, `parent` = ? WHERE `fileid` = ?";
        let target_hash = format!("{:x}", md5::compute(target.as_bytes()));
        let basename = crate::util::basename(&target);
        
        let _ = crate::db::execute_audited(sql, &[&target, &target_hash, &basename, &new_parent_id, &source_id]);
    }

    /// Remove all entries for files that are stored on the storage from the cache
    pub fn clear(&self) {
        let sql = "DELETE FROM `*PREFIX*filecache` WHERE `storage` = ?";
        let _ = crate::db::execute_audited(sql, &[&self.get_numeric_storage_id()]);
        
        let sql = "DELETE FROM `*PREFIX*storages` WHERE `id` = ?";
        let _ = crate::db::execute_audited(sql, &[&self.storage_id]);
    }

    /// Get cache status
    pub fn get_status(&self, file: &str) -> i32 {
        // Normalize file
        let file = self.normalize(file);
        
        let path_hash = format!("{:x}", md5::compute(file.as_bytes()));
        let sql = "SELECT `size` FROM `*PREFIX*filecache` WHERE `storage` = ? AND `path_hash` = ?";
        
        let result = crate::db::execute_audited(sql, &[&self.get_numeric_storage_id(), &path_hash]);
        
        if let Ok(result) = result {
            if let Some(row) = result.rows().unwrap_or_default().next() {
                let size: i64 = row.get("size").unwrap_or(0);
                if size == -1 {
                    return SHALLOW;
                } else {
                    return COMPLETE;
                }
            }
        }
        
        if self.partial.contains_key(&file) {
            PARTIAL
        } else {
            NOT_FOUND
        }
    }

    /// Search for files matching pattern
    pub fn search(&self, pattern: &str) -> Vec<HashMap<String, serde_json::Value>> {
        // Normalize pattern
        let pattern = self.normalize(pattern);
        
        let sql = "SELECT `fileid`, `storage`, `path`, `parent`, `name`, `mimetype`, `mimepart`, `size`, `mtime`, \
                  `encrypted`, `unencrypted_size`, `etag` FROM `*PREFIX*filecache` WHERE `name` LIKE ? AND `storage` = ?";
                  
        let result = crate::db::execute_audited(sql, &[&pattern, &self.get_numeric_storage_id()]);
        
        let mut files = Vec::new();
        
        if let Ok(result) = result {
            for row in result.rows().unwrap_or_default() {
                let mut file = HashMap::new();
                
                file.insert("fileid".to_string(), serde_json::to_value(row.get::<_, i64>("fileid").unwrap_or_default()).unwrap());
                file.insert("storage".to_string(), serde_json::to_value(self.storage_id.clone()).unwrap());
                file.insert("path".to_string(), serde_json::to_value(row.get::<_, String>("path").unwrap_or_default()).unwrap());
                file.insert("parent".to_string(), serde_json::to_value(row.get::<_, i64>("parent").unwrap_or_default()).unwrap());
                file.insert("name".to_string(), serde_json::to_value(row.get::<_, String>("name").unwrap_or_default()).unwrap());
                
                let mimetype_id = row.get::<_, i32>("mimetype").unwrap_or_default();
                let mimepart_id = row.get::<_, i32>("mimepart").unwrap_or_default();
                
                file.insert("mimetype".to_string(), serde_json::to_value(self.get_mimetype(mimetype_id).unwrap_or_default()).unwrap());
                file.insert("mimepart".to_string(), serde_json::to_value(self.get_mimetype(mimepart_id).unwrap_or_default()).unwrap());
                
                file.insert("size".to_string(), serde_json::to_value(row.get::<_, i64>("size").unwrap_or_default()).unwrap());
                file.insert("mtime".to_string(), serde_json::to_value(row.get::<_, i64>("mtime").unwrap_or_default()).unwrap());
                file.insert("encrypted".to_string(), serde_json::to_value(row.get::<_, bool>("encrypted").unwrap_or_default()).unwrap());
                file.insert("unencrypted_size".to_string(), serde_json::to_value(row.get::<_, i64>("unencrypted_size").unwrap_or_default()).unwrap());
                file.insert("etag".to_string(), serde_json::to_value(row.get::<_, String>("etag").unwrap_or_default()).unwrap());
                
                files.push(file);
            }
        }
        
        files
    }

    /// Search for files by mimetype
    pub fn search_by_mime(&self, mimetype: &str) -> Vec<HashMap<String, serde_json::Value>> {
        let where_clause = if mimetype.contains('/') {
            "`mimetype` = ?"
        } else {
            "`mimepart` = ?"
        