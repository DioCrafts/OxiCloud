//! # Filesystem Module
//! Class for abstraction of filesystem functions
//! This module won't call any filesystem functions for itself but will pass them to the correct Storage object
//! This module also handles all the file permission related stuff
//!
//! Hooks provided:
//!  read(path)
//!  write(path, &run)
//!  post_write(path)
//!  create(path, &run) (when a file is created, both create and write will be emitted in that order)
//!  post_create(path)
//!  delete(path, &run)
//!  post_delete(path)
//!  rename(oldpath,newpath, &run)
//!  post_rename(oldpath,newpath)
//!  copy(oldpath,newpath, &run) (if the newpath doesn't exists yes, copy, create and write will be emitted in that order)
//!  post_rename(oldpath,newpath)
//!  post_initMountPoints(user, user_dir)
//!
//!  the &run parameter can be set to false to prevent the operation from occurring

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock, Once};
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;

use crate::files::mount::{Manager as MountManager, Mount};
use crate::files::storage::loader::Loader;
use crate::files::view::View;
use crate::files::storage::Storage;
use crate::util::Util;
use crate::user::{User, UserManager};
use crate::config::Config;
use crate::group;
use crate::hooks::Hooks;
use crate::array_parser::ArrayParser;

pub const SPACE_NOT_COMPUTED: i64 = -1;
pub const SPACE_UNKNOWN: i64 = -2;
pub const SPACE_UNLIMITED: i64 = -3;

/// Signal names for filesystem events
pub struct Signal;

impl Signal {
    pub const CLASSNAME: &'static str = "OC_Filesystem";
    
    pub const RENAME: &'static str = "rename";
    pub const POST_RENAME: &'static str = "post_rename";
    pub const CREATE: &'static str = "create";
    pub const POST_CREATE: &'static str = "post_create";
    pub const COPY: &'static str = "copy";
    pub const POST_COPY: &'static str = "post_copy";
    pub const WRITE: &'static str = "write";
    pub const POST_WRITE: &'static str = "post_write";
    pub const READ: &'static str = "read";
    pub const DELETE: &'static str = "delete";
    
    pub const PARAM_PATH: &'static str = "path";
    pub const PARAM_OLDPATH: &'static str = "oldpath";
    pub const PARAM_NEWPATH: &'static str = "newpath";
    pub const PARAM_RUN: &'static str = "run";
}

lazy_static! {
    static ref MOUNTS: RwLock<Option<Arc<RwLock<MountManager>>>> = RwLock::new(None);
    static ref LOADER: RwLock<Option<Arc<RwLock<Loader>>>> = RwLock::new(None);
    static ref DEFAULT_INSTANCE: RwLock<Option<Arc<RwLock<View>>>> = RwLock::new(None);
    static ref LOADED: RwLock<bool> = RwLock::new(false);
}

/// Filesystem abstraction layer
pub struct Filesystem;

impl Filesystem {
    /// Add a storage wrapper
    pub fn add_storage_wrapper<F>(wrapper: F) 
    where 
        F: Fn(Box<dyn Storage>) -> Box<dyn Storage> + 'static + Send + Sync
    {
        let loader = Self::get_loader();
        let mut loader = loader.write().unwrap();
        loader.add_storage_wrapper(Box::new(wrapper));

        let mount_manager = Self::get_mount_manager();
        let mounts = mount_manager.read().unwrap().get_all();
        
        for mount in mounts {
            mount.write().unwrap().wrap_storage(Box::new(wrapper.clone()));
        }
    }

    /// Get the storage loader
    pub fn get_loader() -> Arc<RwLock<Loader>> {
        let loader_guard = LOADER.read().unwrap();
        if let Some(loader) = &*loader_guard {
            return loader.clone();
        }
        drop(loader_guard);
        
        let mut loader_guard = LOADER.write().unwrap();
        let loader = Arc::new(RwLock::new(Loader::new()));
        *loader_guard = Some(loader.clone());
        loader
    }

    /// Get the mount manager
    pub fn get_mount_manager() -> Arc<RwLock<MountManager>> {
        let mounts_guard = MOUNTS.read().unwrap();
        if let Some(mounts) = &*mounts_guard {
            return mounts.clone();
        }
        drop(mounts_guard);
        
        Util::setup_fs();
        
        let mut mounts_guard = MOUNTS.write().unwrap();
        let mounts = Arc::new(RwLock::new(MountManager::new()));
        *mounts_guard = Some(mounts.clone());
        mounts
    }

    /// Get the mountpoint of the storage object for a path
    pub fn get_mount_point(path: &str) -> String {
        let mounts = Self::get_mount_manager();
        let mounts = mounts.read().unwrap();
        
        if let Some(mount) = mounts.find(path) {
            return mount.read().unwrap().get_mount_point();
        }
        
        String::new()
    }

    /// Get a list of all mount points in a directory
    pub fn get_mount_points(path: &str) -> Vec<String> {
        let mounts = Self::get_mount_manager();
        let mounts = mounts.read().unwrap();
        
        let mut result = Vec::new();
        let mounts_in_path = mounts.find_in(path);
        
        for mount in mounts_in_path {
            result.push(mount.read().unwrap().get_mount_point());
        }
        
        result
    }

    /// Get the storage mounted at a specific mount point
    pub fn get_storage(mount_point: &str) -> Option<Arc<RwLock<dyn Storage>>> {
        let mounts = Self::get_mount_manager();
        let mounts = mounts.read().unwrap();
        
        if let Some(mount) = mounts.find(mount_point) {
            return Some(mount.read().unwrap().get_storage());
        }
        
        None
    }

    /// Get mounts by storage ID
    pub fn get_mount_by_storage_id(id: &str) -> Vec<Arc<RwLock<Mount>>> {
        let mounts = Self::get_mount_manager();
        let mounts = mounts.read().unwrap();
        
        mounts.find_by_storage_id(id)
    }

    /// Get mounts by numeric ID
    pub fn get_mount_by_numeric_id(id: i64) -> Vec<Arc<RwLock<Mount>>> {
        let mounts = Self::get_mount_manager();
        let mounts = mounts.read().unwrap();
        
        mounts.find_by_numeric_id(id)
    }

    /// Resolve a path to a storage and internal path
    pub fn resolve_path(path: &str) -> (Option<Arc<RwLock<dyn Storage>>>, Option<String>) {
        let mounts = Self::get_mount_manager();
        let mounts = mounts.read().unwrap();
        
        if let Some(mount) = mounts.find(path) {
            let mount = mount.read().unwrap();
            return (
                Some(mount.get_storage()),
                Some(mount.get_internal_path(path))
            );
        }
        
        (None, None)
    }

    /// Initialize the filesystem
    pub fn init(user: &str, root: &str) -> bool {
        {
            let default_instance = DEFAULT_INSTANCE.read().unwrap();
            if default_instance.is_some() {
                return false;
            }
        }
        
        Self::get_loader();
        
        let mut default_instance = DEFAULT_INSTANCE.write().unwrap();
        *default_instance = Some(Arc::new(RwLock::new(View::new(root))));
        
        {
            let mounts = MOUNTS.read().unwrap();
            if mounts.is_none() {
                drop(mounts);
                let mut mounts = MOUNTS.write().unwrap();
                *mounts = Some(Arc::new(RwLock::new(MountManager::new())));
            }
        }
        
        // Load custom mount config
        Self::init_mount_points(user);
        
        {
            let mut loaded = LOADED.write().unwrap();
            *loaded = true;
        }
        
        true
    }

    /// Initialize mounts
    pub fn init_mounts() {
        let mounts = MOUNTS.read().unwrap();
        if mounts.is_none() {
            drop(mounts);
            let mut mounts = MOUNTS.write().unwrap();
            *mounts = Some(Arc::new(RwLock::new(MountManager::new())));
        }
    }

    /// Initialize system and personal mount points for a user
    pub fn init_mount_points(user: &str) {
        let username = if user.is_empty() {
            User::get_username()
        } else {
            user.to_string()
        };
        
        let parser = ArrayParser::new();
        let root = User::get_home(&username);
        
        let user_obj = UserManager::get(&username);
        
        if Storage::exists(&format!("local::{}/", root)) || user_obj.is_none() {
            Self::mount("\\OC\\Files\\Storage\\Local", &[("datadir", &root)], &username);
        } else {
            Self::mount("\\OC\\Files\\Storage\\Home", &[("user", &username)], &username);
        }
        
        let datadir = Config::get_value("datadirectory", &format!("{}/data", Config::get_server_root()));
        
        // Move config file to its new position
        let old_config_path = format!("{}/config/mount.json", Config::get_server_root());
        let new_config_path = format!("{}/mount.json", datadir);
        
        if Path::new(&old_config_path).exists() {
            std::fs::rename(&old_config_path, &new_config_path).unwrap_or_default();
        }
        
        // Load system mount points
        let old_php_config = format!("{}/config/mount.php", Config::get_server_root());
        let mount_config = if Path::new(&new_config_path).exists() {
            let json_data = std::fs::read_to_string(&new_config_path).unwrap_or_default();
            serde_json::from_str(&json_data).unwrap_or_else(|_| HashMap::new())
        } else if Path::new(&old_php_config).exists() {
            let php_data = std::fs::read_to_string(&old_php_config).unwrap_or_default();
            parser.parse_php(&php_data)
        } else {
            HashMap::new()
        };
        
        // Process global mounts
        if let Some(global_mounts) = mount_config.get("global") {
            if let Some(global_map) = global_mounts.as_object() {
                for (mount_point, options) in global_map {
                    if let Some(options_map) = options.as_object() {
                        if let (Some(class), Some(options)) = (
                            options_map.get("class").and_then(|c| c.as_str()),
                            options_map.get("options").and_then(|o| o.as_object())
                        ) {
                            let options_vec: Vec<(&str, &str)> = options
                                .iter()
                                .filter_map(|(k, v)| v.as_str().map(|vs| (k.as_str(), vs)))
                                .collect();
                            
                            Self::mount(class, &options_vec, mount_point);
                        }
                    }
                }
            }
        }
        
        // Process group mounts
        if let Some(group_mounts) = mount_config.get("group") {
            if let Some(group_map) = group_mounts.as_object() {
                for (group_name, mounts) in group_map {
                    if group::in_group(&username, group_name) {
                        if let Some(mounts_map) = mounts.as_object() {
                            for (mount_point, options) in mounts_map {
                                let mount_point = Self::set_user_vars(&username, mount_point);
                                
                                if let Some(options_map) = options.as_object() {
                                    if let (Some(class), Some(options)) = (
                                        options_map.get("class").and_then(|c| c.as_str()),
                                        options_map.get("options").and_then(|o| o.as_object())
                                    ) {
                                        let options_vec: Vec<(&str, &str)> = options
                                            .iter()
                                            .filter_map(|(k, v)| v.as_str().map(|vs| (k.as_str(), vs)))
                                            .map(|(k, v)| (k, Self::set_user_vars(&username, v).as_str()))
                                            .collect();
                                        
                                        Self::mount(class, &options_vec, &mount_point);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Process user mounts
        if let Some(user_mounts) = mount_config.get("user") {
            if let Some(user_map) = user_mounts.as_object() {
                for (mount_user, mounts) in user_map {
                    if mount_user == "all" || mount_user.to_lowercase() == username.to_lowercase() {
                        if let Some(mounts_map) = mounts.as_object() {
                            for (mount_point, options) in mounts_map {
                                let mount_point = Self::set_user_vars(&username, mount_point);
                                
                                if let Some(options_map) = options.as_object() {
                                    if let (Some(class), Some(options)) = (
                                        options_map.get("class").and_then(|c| c.as_str()),
                                        options_map.get("options").and_then(|o| o.as_object())
                                    ) {
                                        let options_vec: Vec<(&str, &str)> = options
                                            .iter()
                                            .filter_map(|(k, v)| v.as_str().map(|vs| (k.as_str(), vs)))
                                            .map(|(k, v)| (k, Self::set_user_vars(&username, v).as_str()))
                                            .collect();
                                        
                                        Self::mount(class, &options_vec, &mount_point);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Load personal mount points
        let personal_php_config = format!("{}/mount.php", root);
        let personal_json_config = format!("{}/mount.json", root);
        
        let personal_mount_config = if Path::new(&personal_json_config).exists() {
            let json_data = std::fs::read_to_string(&personal_json_config).unwrap_or_default();
            serde_json::from_str(&json_data).unwrap_or_else(|_| HashMap::new())
        } else if Path::new(&personal_php_config).exists() {
            let php_data = std::fs::read_to_string(&personal_php_config).unwrap_or_default();
            parser.parse_php(&php_data)
        } else {
            HashMap::new()
        };
        
        if let Some(user_mounts) = personal_mount_config.get("user") {
            if let Some(user_map) = user_mounts.as_object() {
                if let Some(mounts) = user_map.get(&username) {
                    if let Some(mounts_map) = mounts.as_object() {
                        for (mount_point, options) in mounts_map {
                            if let Some(options_map) = options.as_object() {
                                if let (Some(class), Some(options)) = (
                                    options_map.get("class").and_then(|c| c.as_str()),
                                    options_map.get("options").and_then(|o| o.as_object())
                                ) {
                                    let options_vec: Vec<(&str, &str)> = options
                                        .iter()
                                        .filter_map(|(k, v)| v.as_str().map(|vs| (k.as_str(), vs)))
                                        .collect();
                                    
                                    Self::mount(class, &options_vec, mount_point);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Chance to mount for other storages
        Hooks::emit("OC_Filesystem", "post_initMountPoints", &[
            ("user", &username),
            ("user_dir", &root)
        ]);
    }

    /// Fill in the correct values for user variables
    fn set_user_vars(user: &str, input: &str) -> String {
        input.replace("$user", user)
    }

    /// Get the default filesystem view
    pub fn get_view() -> Arc<RwLock<View>> {
        DEFAULT_INSTANCE.read().unwrap().as_ref().unwrap().clone()
    }

    /// Tear down the filesystem, removing all storage providers
    pub fn tear_down() {
        Self::clear_mounts();
        let mut default_instance = DEFAULT_INSTANCE.write().unwrap();
        *default_instance = None;
    }

    /// Get the relative path of the root data directory for the current user
    pub fn get_root() -> String {
        Self::get_view().read().unwrap().get_root()
    }

    /// Clear all mounts and storage backends
    pub fn clear_mounts() {
        if let Some(mounts) = &*MOUNTS.read().unwrap() {
            mounts.write().unwrap().clear();
        }
    }

    /// Mount a Storage in our virtual filesystem
    pub fn mount(class: &str, arguments: &[(&str, &str)], mount_point: &str) {
        Util::setup_fs();
        
        let args_map: HashMap<String, String> = arguments
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        
        let mount = Mount::new(
            class,
            mount_point,
            args_map,
            Self::get_loader()
        );
        
        Self::get_mount_manager().write().unwrap().add_mount(Arc::new(RwLock::new(mount)));
    }

    /// Return the path to a local version of the file
    pub fn get_local_file(path: &str) -> String {
        Self::get_view().read().unwrap().get_local_file(path)
    }

    /// Return the path to a local version of the folder
    pub fn get_local_folder(path: &str) -> String {
        Self::get_view().read().unwrap().get_local_folder(path)
    }

    /// Return path to file which reflects one visible in browser
    pub fn get_local_path(path: &str) -> String {
        let datadir = format!("{}/files", User::get_home(&User::get_username()));
        let mut new_path = path.to_string();
        
        if new_path.starts_with(&datadir) {
            new_path = new_path[datadir.len()..].to_string();
        }
        
        new_path
    }

    /// Check if the requested path is valid
    pub fn is_valid_path(path: &str) -> bool {
        let path = Self::normalize_path(path, true);
        
        if path.is_empty() || !path.starts_with('/') {
            return false;
        }
        
        if path.contains("/../") || path.ends_with("/..") {
            return false;
        }
        
        true
    }

    /// Check if a file is blacklisted for storage in the filesystem
    pub fn is_blacklisted(data: &mut HashMap<String, String>) {
        let path = if data.contains_key("path") {
            data.get("path").cloned()
        } else if data.contains_key("newpath") {
            data.get("newpath").cloned()
        } else {
            None
        };
        
        if let Some(p) = path {
            if Self::is_file_blacklisted(&p) {
                data.insert("run".to_string(), "false".to_string());
            }
        }
    }

    /// Check if a file is blacklisted
    pub fn is_file_blacklisted(filename: &str) -> bool {
        let blacklist: Vec<String> = Config::get_value("blacklisted_files", vec![".htaccess".to_string()]);
        let filename = Path::new(filename).file_name()
            .map(|n| n.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        
        blacklist.iter().any(|b| b.to_lowercase() == filename)
    }

    /// Check if the directory should be ignored when scanning
    pub fn is_ignored_dir(dir: &str) -> bool {
        dir == "." || dir == ".."
    }

    /// Fix common problems with a file path
    pub fn normalize_path(path: &str, strip_trailing_slash: bool) -> String {
        if path.is_empty() {
            return "/".to_string();
        }
        
        // No Windows style slashes
        let mut normalized = path.replace('\\', "/");
        
        // Add leading slash
        if !normalized.starts_with('/') {
            normalized = format!("/{}", normalized);
        }
        
        // Remove duplicate slashes
        while normalized.contains("//") {
            normalized = normalized.replace("//", "/");
        }
        
        // Remove trailing slash
        if strip_trailing_slash && normalized.len() > 1 && normalized.ends_with('/') {
            normalized.pop();
        }
        
        // Normalize unicode if possible
        Util::normalize_unicode(&normalized)
    }

    // Basic filesystem operations - delegated to the view

    pub fn mkdir(path: &str) -> bool {
        Self::get_view().read().unwrap().mkdir(path)
    }

    pub fn rmdir(path: &str) -> bool {
        Self::get_view().read().unwrap().rmdir(path)
    }

    pub fn opendir(path: &str) -> Option<Box<dyn Iterator<Item = String>>> {
        Self::get_view().read().unwrap().opendir(path)
    }

    pub fn readdir(resource: &mut Box<dyn Iterator<Item = String>>) -> Option<String> {
        resource.next()
    }

    pub fn is_dir(path: &str) -> bool {
        Self::get_view().read().unwrap().is_dir(path)
    }

    pub fn is_file(path: &str) -> bool {
        Self::get_view().read().unwrap().is_file(path)
    }

    pub fn stat(path: &str) -> Option<HashMap<String, i64>> {
        Self::get_view().read().unwrap().stat(path)
    }

    pub fn filetype(path: &str) -> Option<String> {
        Self::get_view().read().unwrap().filetype(path)
    }

    pub fn filesize(path: &str) -> i64 {
        Self::get_view().read().unwrap().filesize(path)
    }

    pub fn readfile(path: &str) -> Option<Vec<u8>> {
        Self::get_view().read().unwrap().readfile(path)
    }

    pub fn is_creatable(path: &str) -> bool {
        Self::get_view().read().unwrap().is_creatable(path)
    }

    pub fn is_readable(path: &str) -> bool {
        Self::get_view().read().unwrap().is_readable(path)
    }

    pub fn is_updatable(path: &str) -> bool {
        Self::get_view().read().unwrap().is_updatable(path)
    }

    pub fn is_deletable(path: &str) -> bool {
        Self::get_view().read().unwrap().is_deletable(path)
    }

    pub fn is_sharable(path: &str) -> bool {
        Self::get_view().read().unwrap().is_sharable(path)
    }

    pub fn file_exists(path: &str) -> bool {
        Self::get_view().read().unwrap().file_exists(path)
    }

    pub fn filemtime(path: &str) -> i64 {
        Self::get_view().read().unwrap().filemtime(path)
    }

    pub fn touch(path: &str, mtime: Option<i64>) -> bool {
        Self::get_view().read().unwrap().touch(path, mtime)
    }

    pub fn file_get_contents(path: &str) -> Option<String> {
        Self::get_view().read().unwrap().file_get_contents(path)
    }

    pub fn file_put_contents(path: &str, data: &[u8]) -> Option<usize> {
        Self::get_view().read().unwrap().file_put_contents(path, data)
    }

    pub fn unlink(path: &str) -> bool {
        Self::get_view().read().unwrap().unlink(path)
    }

    pub fn rename(path1: &str, path2: &str) -> bool {
        Self::get_view().read().unwrap().rename(path1, path2)
    }

    pub fn copy(path1: &str, path2: &str) -> bool {
        Self::get_view().read().unwrap().copy(path1, path2)
    }

    pub fn fopen(path: &str, mode: &str) -> Option<Box<dyn std::io::Read + std::io::Write + std::io::Seek>> {
        Self::get_view().read().unwrap().fopen(path, mode)
    }

    pub fn to_tmp_file(path: &str) -> Option<String> {
        Self::get_view().read().unwrap().to_tmp_file(path)
    }

    pub fn from_tmp_file(tmp_file: &str, path: &str) -> bool {
        Self::get_view().read().unwrap().from_tmp_file(tmp_file, path)
    }

    pub fn get_mime_type(path: &str) -> String {
        Self::get_view().read().unwrap().get_mime_type(path)
    }

    pub fn hash(type_: &str, path: &str, raw: bool) -> Option<String> {
        Self::get_view().read().unwrap().hash(type_, path, raw)
    }

    pub fn free_space(path: &str) -> i64 {
        Self::get_view().read().unwrap().free_space(path)
    }

    pub fn search(query: &str) -> Vec<String> {
        Self::get_view().read().unwrap().search(query)
    }

    pub fn search_by_mime(query: &str) -> Vec<String> {
        Self::get_view().read().unwrap().search_by_mime(query)
    }

    pub fn has_updated(path: &str, time: i64) -> bool {
        Self::get_view().read().unwrap().has_updated(path, time)
    }

    pub fn get_file_info(path: &str) -> Option<HashMap<String, serde_json::Value>> {
        Self::get_view().read().unwrap().get_file_info(path)
    }

    pub fn put_file_info(path: &str, data: &HashMap<String, serde_json::Value>) -> Option<i64> {
        Self::get_view().read().unwrap().put_file_info(path, data)
    }

    pub fn get_directory_content(directory: &str, mimetype_filter: &str) -> Vec<HashMap<String, serde_json::Value>> {
        Self::get_view().read().unwrap().get_directory_content(directory, mimetype_filter)
    }

    pub fn get_path(id: i64) -> Option<String> {
        Self::get_view().read().unwrap().get_path(id)
    }

    pub fn get_owner(path: &str) -> String {
        Self::get_view().read().unwrap().get_owner(path)
    }

    pub fn get_etag(path: &str) -> String {
        Self::get_view().read().unwrap().get_etag(path)
    }
}

// Initialize filesystem when module is loaded
lazy_static! {
    static ref INIT: Once = Once::new();
}

pub fn setup_fs() {
    INIT.call_once(|| {
        Util::setup_fs();
    });
}