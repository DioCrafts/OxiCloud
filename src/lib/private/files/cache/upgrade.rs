use async_trait::async_trait;
use chrono::NaiveDateTime;
use log::{error, info};
use md5::{Digest, Md5};
use std::collections::HashMap;
use std::sync::Arc;

use crate::db::{self, Connection, DatabaseExecutor, QueryBuilder};
use crate::files::cache::{Cache, CacheEntry, Legacy, Scanner};
use crate::files::storage::Storage;
use crate::hooks::{self, Hook};
use crate::users::config::Config;
use crate::utils::permissions::{self, Permission};

/// Handles upgrading the file cache from legacy format to the current version
pub struct Upgrade {
    legacy: Arc<dyn Legacy>,
    numeric_ids: HashMap<String, i64>,
    mimetype_ids: HashMap<String, i64>,
}

impl Upgrade {
    /// Create a new cache upgrade handler
    pub fn new(legacy: Arc<dyn Legacy>) -> Self {
        Self {
            legacy,
            numeric_ids: HashMap::new(),
            mimetype_ids: HashMap::new(),
        }
    }

    /// Perform an upgrade for a path and its children
    ///
    /// # Arguments
    /// * `path` - The path to upgrade
    /// * `mode` - Scan mode (recursive or not)
    pub async fn upgrade_path(&mut self, path: &str, mode: Scanner) -> Result<(), db::Error> {
        if !self.legacy.has_items().await? {
            return Ok(());
        }

        hooks::emit("\\OC\\Files\\Cache\\Upgrade", "migrate_path", path).await?;

        if let Some(row) = self.legacy.get(path).await? {
            if let Some(data) = self.get_new_data(&row).await? {
                self.insert(&data).await?;
                self.upgrade_childs(data.id, mode).await?;
            }
        }

        Ok(())
    }

    /// Upgrade all child elements of an item
    ///
    /// # Arguments
    /// * `id` - The parent ID
    /// * `mode` - Scan mode (recursive or not)
    async fn upgrade_childs(&mut self, id: i64, mode: Scanner) -> Result<(), db::Error> {
        let children = self.legacy.get_children(id).await?;
        
        for child in children {
            if let Some(child_data) = self.get_new_data(&child).await? {
                hooks::emit("\\OC\\Files\\Cache\\Upgrade", "migrate_path", &child.path).await?;
                
                self.insert(&child_data).await?;
                
                if mode == Scanner::Recursive {
                    self.upgrade_childs(child.id, mode).await?;
                }
            }
        }

        Ok(())
    }

    /// Insert data into the new cache
    ///
    /// # Arguments
    /// * `data` - The data for the new cache
    async fn insert(&self, data: &CacheEntry) -> Result<(), db::Error> {
        if !self.in_cache(data.storage, &data.path_hash, data.id).await? {
            let query = QueryBuilder::new()
                .insert_into("filecache")
                .columns(&[
                    "fileid", "storage", "path", "path_hash", "parent", "name",
                    "mimetype", "mimepart", "size", "mtime", "encrypted", "etag"
                ])
                .values(&[
                    &data.id, &data.storage, &data.path, &data.path_hash, 
                    &data.parent, &data.name, &data.mimetype, &data.mimepart, 
                    &data.size, &data.mtime, &data.encrypted, &data.etag
                ])
                .build();

            db::execute(&query).await?;
        }

        Ok(())
    }

    /// Check if an item is already in the new cache
    ///
    /// # Arguments
    /// * `storage` - Storage ID
    /// * `path_hash` - Hash of the path
    /// * `id` - File ID
    async fn in_cache(&self, storage: i64, path_hash: &str, id: i64) -> Result<bool, db::Error> {
        let query = QueryBuilder::new()
            .select("fileid")
            .from("filecache")
            .where_condition("(storage = ? AND path_hash = ?) OR fileid = ?")
            .build();

        let result = db::execute_with_params(&query, &[&storage, &path_hash, &id]).await?;
        
        Ok(result.rows_affected() > 0)
    }

    /// Get the new data array from the old one
    ///
    /// # Arguments
    /// * `data` - The data from the old cache
    async fn get_new_data(&mut self, data: &CacheEntry) -> Result<Option<CacheEntry>, db::Error> {
        // Make sure there is a path, otherwise we can do nothing
        if data.path.is_empty() {
            return Ok(None);
        }

        let mut new_data = data.clone();
        
        let (storage, internal_path) = match crate::files::filesystem::resolve_path(&data.path).await {
            Ok(result) => result,
            Err(e) => {
                error!("Unable to migrate data from old cache for {} because the storage was not found: {}", 
                       data.path, e);
                return Ok(None);
            }
        };

        new_data.path_hash = format!("{:x}", Md5::digest(internal_path.as_bytes()));
        new_data.path = internal_path.clone();
        new_data.storage = self.get_numeric_id(&storage).await?;
        new_data.parent = if internal_path.is_empty() { -1 } else { data.parent };
        new_data.permissions = if data.writable { Permission::ALL } else { Permission::READ };
        
        new_data.mimetype = self.get_mimetype_id(&new_data.mimetype_str, &storage).await?;
        new_data.mimepart = self.get_mimetype_id(&new_data.mimepart_str, &storage).await?;

        Ok(Some(new_data))
    }

    /// Get the numeric storage id
    ///
    /// # Arguments
    /// * `storage` - The storage
    async fn get_numeric_id(&mut self, storage: &Arc<dyn Storage>) -> Result<i64, db::Error> {
        let storage_id = storage.id();
        
        if !self.numeric_ids.contains_key(&storage_id) {
            let cache = storage.get_cache().await?;
            let numeric_id = cache.get_numeric_storage_id().await?;
            self.numeric_ids.insert(storage_id.clone(), numeric_id);
        }
        
        Ok(*self.numeric_ids.get(&storage_id).unwrap())
    }

    /// Get the numeric id for a mimetype
    ///
    /// # Arguments
    /// * `mimetype` - The mimetype
    /// * `storage` - The storage
    async fn get_mimetype_id(&mut self, mimetype: &str, storage: &Arc<dyn Storage>) -> Result<i64, db::Error> {
        if !self.mimetype_ids.contains_key(mimetype) {
            let cache = Cache::new(storage.clone());
            let mimetype_id = cache.get_mimetype_id(mimetype).await?;
            self.mimetype_ids.insert(mimetype.to_string(), mimetype_id);
        }
        
        Ok(*self.mimetype_ids.get(mimetype).unwrap())
    }

    /// Check if a cache upgrade is required for a user
    ///
    /// # Arguments
    /// * `user` - The user ID
    pub async fn need_upgrade(user: &str) -> Result<bool, db::Error> {
        let cache_version = Config::get_user_value(user, "files", "cache_version", 4).await?;
        
        if cache_version < 5 {
            let legacy = Legacy::new(user.to_string());
            
            if legacy.has_items().await? {
                return Ok(true);
            }
            
            Self::upgrade_done(user).await?;
        }

        Ok(false)
    }

    /// Mark the filecache as upgraded
    ///
    /// # Arguments
    /// * `user` - The user ID
    pub async fn upgrade_done(user: &str) -> Result<(), db::Error> {
        Config::set_user_value(user, "files", "cache_version", 5).await?;
        Ok(())
    }

    /// Performs a "silent" upgrade, i.e. without an Event-Source as triggered
    /// on User-Login via Ajax. This method is called within the regular
    /// ownCloud upgrade.
    ///
    /// # Arguments
    /// * `user` - A User ID
    pub async fn do_silent_upgrade(user: &str) -> Result<(), db::Error> {
        if !Self::need_upgrade(user).await? {
            return Ok(());
        }
        
        let legacy = Arc::new(Legacy::new(user.to_string()));
        
        if legacy.has_items().await? {
            let tx = db::begin_transaction().await?;
            
            let mut upgrade = Self::new(legacy);
            upgrade.upgrade_path(&format!("/{}/files", user), Scanner::Recursive).await?;
            
            tx.commit().await?;
        }
        
        Self::upgrade_done(user).await?;
        Ok(())
    }
}