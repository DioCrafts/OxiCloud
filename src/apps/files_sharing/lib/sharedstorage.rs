// Copyright 2011 Michael Gapczynski mtgap@owncloud.com
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::path::Path;
use std::io::{Read, Write};
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::files::filesystem;
use crate::files::storage::common::Common;
use crate::files::stream::dir;
use crate::files::cache::{self, SharedCache, SharedPermissions, SharedWatcher};
use crate::helper;
use crate::share::{self, ShareBackendFile, ShareBackendFolder};
use crate::util;

const SPACE_UNKNOWN: i64 = -1;
const PERMISSION_CREATE: u32 = 4;
const PERMISSION_UPDATE: u32 = 2;
const PERMISSION_DELETE: u32 = 8;
const PERMISSION_SHARE: u32 = 16;

/// Convert target path to source path and pass the function call to the correct storage provider
pub struct Shared {
    shared_folder: String,
    files: HashMap<String, HashMap<String, String>>,
}

impl Shared {
    pub fn new(arguments: HashMap<String, String>) -> Self {
        Self {
            shared_folder: arguments.get("sharedFolder").unwrap_or(&String::new()).clone(),
            files: HashMap::new(),
        }
    }

    /// Get the source file path, permissions, and owner for a shared file
    ///
    /// # Arguments
    ///
    /// * `target` - Shared target file path
    ///
    /// # Returns
    ///
    /// Returns Option with HashMap containing keys path, permissions, and owner or None if not found
    pub fn get_file(&mut self, target: &str) -> Option<&HashMap<String, String>> {
        if !self.files.contains_key(target) {
            // Check for partial files
            let path_info = Path::new(target);
            if path_info.extension().and_then(|e| e.to_str()) == Some("part") {
                if let Some(mut source) = ShareBackendFile::get_source(
                    &target[0..target.len() - 5]
                ) {
                    let path = source.get_mut("path")?;
                    *path = format!("{}.part", path);
                    
                    // All partial files have delete permission
                    if let Some(permissions) = source.get_mut("permissions") {
                        if let Ok(perm_val) = permissions.parse::<u32>() {
                            *permissions = (perm_val | PERMISSION_DELETE).to_string();
                        }
                    }
                    
                    self.files.insert(target.to_string(), source);
                }
            } else {
                if let Some(source) = ShareBackendFile::get_source(target) {
                    self.files.insert(target.to_string(), source);
                }
            }
        }
        
        self.files.get(target)
    }

    /// Get the source file path for a shared file
    ///
    /// # Arguments
    ///
    /// * `target` - Shared target file path
    ///
    /// # Returns
    ///
    /// String source file path or None if not found
    pub fn get_source_path(&mut self, target: &str) -> Option<String> {
        let source = self.get_file(target)?;
        
        if !source.contains_key("fullPath") {
            let file_owner = source.get("fileOwner")?;
            filesystem::init_mount_points(file_owner);
            
            let storage_id = source.get("storage")?;
            let mount = filesystem::get_mount_by_numeric_id(storage_id);
            
            if let Some(mount) = mount {
                let mount_key = mount.keys().next()?;
                let mount_point = mount.get(mount_key)?.get_mount_point();
                let path = source.get("path")?;
                
                let full_path = format!("{}{}", mount_point, path);
                let mut source = self.files.get_mut(target)?;
                source.insert("fullPath".to_string(), full_path.clone());
                
                return Some(full_path);
            } else {
                let mut source = self.files.get_mut(target)?;
                source.insert("fullPath".to_string(), "".to_string());
                return None;
            }
        }
        
        let full_path = source.get("fullPath")?;
        if full_path.is_empty() {
            None
        } else {
            Some(full_path.clone())
        }
    }

    /// Get the permissions granted for a shared file
    ///
    /// # Arguments
    ///
    /// * `target` - Shared target file path
    ///
    /// # Returns
    ///
    /// CRUDS permissions granted or None if not found
    pub fn get_permissions(&mut self, target: &str) -> Option<u32> {
        let source = self.get_file(target)?;
        source.get("permissions").and_then(|p| p.parse::<u32>().ok())
    }
}

impl Common for Shared {
    fn get_id(&self) -> String {
        format!("shared::{}", self.shared_folder)
    }

    fn mkdir(&mut self, path: &str) -> bool {
        if path.is_empty() || path == "/" || !self.is_creatable(&Path::new(path).parent().unwrap().to_string_lossy()) {
            false
        } else if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.mkdir(&internal_path)
        } else {
            false
        }
    }

    fn rmdir(&mut self, path: &str) -> bool {
        if let Some(source) = self.get_source_path(path) {
            if self.is_deletable(path) {
                let (storage, internal_path) = filesystem::resolve_path(&source);
                storage.rmdir(&internal_path)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn opendir(&mut self, path: &str) -> Option<File> {
        if path.is_empty() || path == "/" {
            let files = share::get_items_shared_with("file", ShareBackendFolder::FORMAT_OPENDIR);
            dir::register("shared", files);
            File::open("fakedir://shared").ok()
        } else if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.opendir(&internal_path)
        } else {
            None
        }
    }

    fn is_dir(&mut self, path: &str) -> bool {
        if path.is_empty() || path == "/" {
            true
        } else if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.is_dir(&internal_path)
        } else {
            false
        }
    }

    fn is_file(&mut self, path: &str) -> bool {
        if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.is_file(&internal_path)
        } else {
            false
        }
    }

    fn stat(&mut self, path: &str) -> Option<HashMap<String, i64>> {
        if path.is_empty() || path == "/" {
            let mut stat = HashMap::new();
            stat.insert("size".to_string(), self.filesize(path) as i64);
            stat.insert("mtime".to_string(), self.filemtime(path) as i64);
            Some(stat)
        } else if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.stat(&internal_path)
        } else {
            None
        }
    }

    fn filetype(&mut self, path: &str) -> Option<String> {
        if path.is_empty() || path == "/" {
            Some("dir".to_string())
        } else if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.filetype(&internal_path)
        } else {
            None
        }
    }

    fn filesize(&mut self, path: &str) -> u64 {
        if path.is_empty() || path == "/" || self.is_dir(path) {
            0
        } else if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.filesize(&internal_path)
        } else {
            0
        }
    }

    fn is_creatable(&mut self, path: &str) -> bool {
        if path.is_empty() {
            false
        } else {
            self.get_permissions(path).map_or(false, |p| p & PERMISSION_CREATE != 0)
        }
    }

    fn is_readable(&mut self, path: &str) -> bool {
        self.file_exists(path)
    }

    fn is_updatable(&mut self, path: &str) -> bool {
        if path.is_empty() {
            false
        } else {
            self.get_permissions(path).map_or(false, |p| p & PERMISSION_UPDATE != 0)
        }
    }

    fn is_deletable(&mut self, path: &str) -> bool {
        if path.is_empty() {
            true
        } else {
            self.get_permissions(path).map_or(false, |p| p & PERMISSION_DELETE != 0)
        }
    }

    fn is_sharable(&mut self, path: &str) -> bool {
        if path.is_empty() {
            false
        } else {
            self.get_permissions(path).map_or(false, |p| p & PERMISSION_SHARE != 0)
        }
    }

    fn file_exists(&mut self, path: &str) -> bool {
        if path.is_empty() || path == "/" {
            true
        } else if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.file_exists(&internal_path)
        } else {
            false
        }
    }

    fn filemtime(&mut self, path: &str) -> u64 {
        if path.is_empty() || path == "/" {
            let mut mtime = 0;
            if let Some(dh) = self.opendir(path) {
                // Reading directory and getting filemtime for each entry
                // This is a simplified approach since we can't directly translate readdir from PHP
                // In a real implementation, you'd iterate through the directory handle
                // For now we'll just return current time as a placeholder
                mtime = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
            }
            mtime
        } else if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.filemtime(&internal_path)
        } else {
            0
        }
    }

    fn file_get_contents(&mut self, path: &str) -> Option<Vec<u8>> {
        if let Some(source) = self.get_source_path(path) {
            let info = HashMap::from([
                ("target".to_string(), format!("{}{}", self.shared_folder, path)),
                ("source".to_string(), source.clone()),
            ]);
            
            util::emit_hook(
                r"\OC\Files\Storage\Shared",
                "file_get_contents",
                &info
            );
            
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.file_get_contents(&internal_path)
        } else {
            None
        }
    }

    fn file_put_contents(&mut self, path: &str, data: &[u8]) -> bool {
        if let Some(source) = self.get_source_path(path) {
            // Check if permission is granted
            if (self.file_exists(path) && !self.is_updatable(path))
                || (self.is_dir(path) && !self.is_creatable(path)) {
                return false;
            }
            
            let info = HashMap::from([
                ("target".to_string(), format!("{}{}", self.shared_folder, path)),
                ("source".to_string(), source.clone()),
            ]);
            
            util::emit_hook(
                r"\OC\Files\Storage\Shared",
                "file_put_contents",
                &info
            );
            
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.file_put_contents(&internal_path, data)
        } else {
            false
        }
    }

    fn unlink(&mut self, path: &str) -> bool {
        // Delete the file if DELETE permission is granted
        if let Some(source) = self.get_source_path(path) {
            if self.is_deletable(path) {
                let (storage, internal_path) = filesystem::resolve_path(&source);
                storage.unlink(&internal_path)
            } else {
                let dir_name = Path::new(path).parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
                if dir_name == "/" || dir_name == "." {
                    // Unshare the file from the user if in the root of the Shared folder
                    let item_type = if self.is_dir(path) { "folder" } else { "file" };
                    share::unshare_from_self(item_type, path)
                } else {
                    false
                }
            }
        } else {
            false
        }
    }

    fn rename(&mut self, path1: &str, path2: &str) -> bool {
        // Check for partial files
        let path_info = Path::new(path1);
        if path_info.extension().and_then(|e| e.to_str()) == Some("part") {
            if let Some(old_source) = self.get_source_path(path1) {
                let (storage, old_internal_path) = filesystem::resolve_path(&old_source);
                let new_internal_path = &old_internal_path[0..old_internal_path.len() - 5];
                return storage.rename(&old_internal_path, new_internal_path);
            }
        } else {
            // Renaming/moving is only allowed within shared folders
            let pos1 = path1.find('/', 1);
            let pos2 = path2.find('/', 1);
            
            if let (Some(pos1), Some(pos2)) = (pos1, pos2) {
                if let Some(old_source) = self.get_source_path(path1) {
                    let path2_dir = Path::new(path2).parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
                    let new_source = format!("{}/{}", self.get_source_path(&path2_dir).unwrap_or_default(), Path::new(path2).file_name().unwrap_or_default().to_string_lossy());
                    
                    let path1_dir = Path::new(path1).parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
                    if path1_dir == path2_dir {
                        // Rename the file if UPDATE permission is granted
                        if self.is_updatable(path1) {
                            let (storage, old_internal_path) = filesystem::resolve_path(&old_source);
                            let (_, new_internal_path) = filesystem::resolve_path(&new_source);
                            return storage.rename(&old_internal_path, &new_internal_path);
                        }
                    } else {
                        // Move the file if DELETE and CREATE permissions are granted
                        if self.is_deletable(path1) && self.is_creatable(&path2_dir) {
                            // Get the root shared folder
                            let folder1 = &path1[0..pos1];
                            let folder2 = &path2[0..pos2];
                            
                            // Copy and unlink the file if it exists in a different shared folder
                            if folder1 != folder2 {
                                if self.copy(path1, path2) {
                                    return self.unlink(path1);
                                }
                            } else {
                                let (storage, old_internal_path) = filesystem::resolve_path(&old_source);
                                let (_, new_internal_path) = filesystem::resolve_path(&new_source);
                                return storage.rename(&old_internal_path, &new_internal_path);
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn copy(&mut self, path1: &str, path2: &str) -> bool {
        // Copy the file if CREATE permission is granted
        let path2_dir = Path::new(path2).parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
        if self.is_creatable(&path2_dir) {
            if let (Some(mut source), Some(mut target)) = (self.fopen(path1, "r"), self.fopen(path2, "w")) {
                let (count, result) = helper::stream_copy(&mut source, &mut target);
                return result;
            }
        }
        false
    }

    fn fopen(&mut self, path: &str, mode: &str) -> Option<File> {
        if let Some(source) = self.get_source_path(path) {
            let write_modes = ["r+", "rb+", "w+", "wb+", "x+", "xb+", "a+", "ab+", "w", "wb", "x", "xb", "a", "ab"];
            
            if write_modes.contains(&mode) {
                let exists = self.file_exists(path);
                if exists && !self.is_updatable(path) {
                    return None;
                }
                
                let path_dir = Path::new(path).parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
                if !exists && !self.is_creatable(&path_dir) {
                    return None;
                }
            }
            
            let info = HashMap::from([
                ("target".to_string(), format!("{}{}", self.shared_folder, path)),
                ("source".to_string(), source.clone()),
                ("mode".to_string(), mode.to_string()),
            ]);
            
            util::emit_hook(
                r"\OC\Files\Storage\Shared",
                "fopen",
                &info
            );
            
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.fopen(&internal_path, mode)
        } else {
            None
        }
    }

    fn get_mime_type(&mut self, path: &str) -> Option<String> {
        if path.is_empty() || path == "/" {
            Some("httpd/unix-directory".to_string())
        } else if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.get_mime_type(&internal_path)
        } else {
            None
        }
    }

    fn free_space(&mut self, path: &str) -> i64 {
        if path.is_empty() {
            return SPACE_UNKNOWN;
        }
        
        if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.free_space(&internal_path)
        } else {
            SPACE_UNKNOWN
        }
    }

    fn get_local_file(&mut self, path: &str) -> Option<String> {
        if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.get_local_file(&internal_path)
        } else {
            None
        }
    }

    fn touch(&mut self, path: &str, mtime: Option<u64>) -> bool {
        if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.touch(&internal_path, mtime)
        } else {
            false
        }
    }

    fn has_updated(&mut self, path: &str, time: u64) -> bool {
        if path.is_empty() {
            false
        } else {
            self.filemtime(path) > time
        }
    }

    fn get_cache(&self, path: &str) -> Box<dyn cache::Cache> {
        Box::new(SharedCache::new(self))
    }

    fn get_scanner(&self, path: &str) -> Box<dyn cache::Scanner> {
        Box::new(cache::Scanner::new(self))
    }

    fn get_permissions_cache(&self, path: &str) -> Box<dyn cache::Permissions> {
        Box::new(SharedPermissions::new(self))
    }

    fn get_watcher(&self, path: &str) -> Box<dyn cache::Watcher> {
        Box::new(SharedWatcher::new(self))
    }

    fn get_owner(&mut self, path: &str) -> Option<String> {
        if path.is_empty() {
            return None;
        }
        
        let source = self.get_file(path)?;
        source.get("fileOwner").cloned()
    }

    fn get_etag(&mut self, path: &str) -> Option<String> {
        if path.is_empty() {
            return Common::get_etag(self, path);
        }
        
        if let Some(source) = self.get_source_path(path) {
            let (storage, internal_path) = filesystem::resolve_path(&source);
            storage.get_etag(&internal_path)
        } else {
            None
        }
    }
}

impl Shared {
    pub fn setup(options: &HashMap<String, String>) -> bool {
        let user = options.get("user")?;
        
        if !crate::user::is_logged_in() || 
           crate::user::get_user() != user || 
           !share::get_items_shared_with("file").is_empty() {
            
            let user_dir = options.get("user_dir")?;
            
            filesystem::mount(
                "\OC\Files\Storage\Shared",
                HashMap::from([("sharedFolder".to_string(), "/Shared".to_string())]),
                format!("{}/Shared/", user_dir)
            );
            
            return true;
        }
        
        false
    }
}