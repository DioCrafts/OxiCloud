//! ownCloud
//!
//! @author Sam Tuke, Robin Appelman
//! @copyright 2012 Sam Tuke samtuke@owncloud.com, Robin Appelman
//! icewind1991@gmail.com
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

//! Encryption proxy which handles filesystem operations before and after
//! execution and encrypts, and handles keyfiles accordingly. Used for
//! webui.

use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Once;
use std::fs::File;
use std::io::Result as IoResult;
use async_trait::async_trait;
use once_cell::sync::Lazy;

use crate::crypt::Crypt;
use crate::helper::Helper;
use crate::keymanager::Keymanager;
use crate::session::Session;
use crate::util::Util;

/// File proxy implementation for encryption
pub struct Proxy {
    black_list: Arc<Mutex<Option<Vec<String>>>>,
}

#[async_trait]
impl FileProxy for Proxy {
    async fn pre_file_put_contents(&self, path: &str, data: &mut Vec<u8>) -> Result<bool, Error> {
        if self.should_encrypt(path) {
            if !data.is_empty() {
                // Get root view
                let view = FilesystemView::new("/")?;
                
                // Get relative path
                let relative_path = Helper::strip_user_files_path(path);
                
                if relative_path.is_none() {
                    return Ok(true);
                }
                
                let temp_path = format!("{}.etmp", path);
                let crypt_path = format!("crypt://{}", temp_path);
                
                let mut handle = CryptFile::open(&crypt_path, "w")?;
                
                // Write data to stream
                handle.write_all(data)?;
                
                // Close stream
                drop(handle);
                
                // Disable encryption proxy to prevent recursive calls
                let proxy_status = FileProxy::disable()?;
                
                // Get encrypted content
                let encrypted_data = view.file_get_contents(&temp_path)?;
                
                // Remove our temp file
                view.unlink(&temp_path)?;
                
                // Re-enable proxy - our work is done
                FileProxy::set_enabled(proxy_status)?;
                
                // Replace data with encrypted content
                *data = encrypted_data;
            }
        }
        
        Ok(true)
    }
    
    async fn post_file_get_contents(&self, path: &str, data: &mut Vec<u8>) -> Result<(), Error> {
        let mut plain_data = None;
        let view = FilesystemView::new("/")?;
        
        // Init session
        let session = Session::new(&view);
        
        // If data is a catfile
        if Crypt::mode() == "server" && Crypt::is_catfile_content(data) {
            let crypt_path = format!("crypt://{}", path);
            let mut handle = CryptFile::open(&crypt_path, "r")?;
            
            let mut buffer = String::new();
            handle.read_to_string(&mut buffer)?;
            plain_data = Some(buffer.into_bytes());
        } else if Crypt::mode() == "server" 
                 && AppSession::exists("legacyenckey") 
                 && Crypt::is_encrypted_meta(path) {
            // Disable encryption proxy to prevent recursive calls
            let proxy_status = FileProxy::disable()?;
            
            let decrypted = Crypt::legacy_block_decrypt(data, &session.get_legacy_key())?;
            plain_data = Some(decrypted);
            
            FileProxy::set_enabled(proxy_status)?;
        }
        
        if let Some(plain) = plain_data {
            *data = plain;
        }
        
        Ok(())
    }
    
    async fn pre_unlink(&self, path: &str) -> Result<bool, Error> {
        // Let the trashbin handle this
        if App::is_enabled("files_trashbin")? {
            return Ok(true);
        }
        
        // Disable encryption proxy to prevent recursive calls
        let proxy_status = FileProxy::disable()?;
        
        let view = FilesystemView::new("/")?;
        let user_id = User::get_user()?;
        
        let util = Util::new(&view, &user_id);
        
        // Get relative path
        let relative_path = Helper::strip_user_files_path(path);
        
        if let Some(rel_path) = relative_path {
            let (owner, owner_path) = util.get_uid_and_filename(&rel_path)?;
            
            // Delete keyfile & shareKey so it isn't orphaned
            if !Keymanager::delete_file_key(&view, &owner, &owner_path)? {
                Util::write_log(
                    "Encryption library",
                    &format!("Keyfile or shareKey could not be deleted for file \"{}\"", owner_path),
                    LogLevel::Error
                );
            }
            
            Keymanager::del_all_share_keys(&view, &owner, &owner_path)?;
        }
        
        FileProxy::set_enabled(proxy_status)?;
        
        // If we don't return true then file delete will fail; better
        // to leave orphaned keyfiles than to disallow file deletion
        Ok(true)
    }
    
    async fn post_touch(&self, path: &str) -> Result<bool, Error> {
        self.handle_file(path).await?;
        
        Ok(true)
    }
    
    async fn post_fopen(&self, path: &str, result: Option<File>) -> Result<Option<File>, Error> {
        if result.is_none() {
            return Ok(None);
        }
        
        let path = Filesystem::normalize_path(path);
        
        // Split the path parts
        let path_parts: Vec<&str> = path.split('/').collect();
        
        // Get relative path
        let relative_path = Helper::strip_user_files_path(&path);
        
        // FIXME: handling for /userId/cache used by webdav for chunking. The cache chunks are NOT encrypted
        if path_parts.len() > 2 && path_parts[2] == "cache" {
            return Ok(result);
        }
        
        // Disable encryption proxy to prevent recursive calls
        let proxy_status = FileProxy::disable()?;
        
        let result_file = result.unwrap();
        let meta = result_file.metadata()?;
        let mode = if meta.permissions().readonly() { "r" } else { "w" };
        
        let view = FilesystemView::new("")?;
        let util = Util::new(&view, &User::get_user()?);
        
        let new_result = if Crypt::mode() == "server" && util.is_encrypted_path(&path)? {
            // Close the original encrypted file
            drop(result_file);
            
            // Open the file using the crypto stream wrapper
            // protocol and let it do the decryption work instead
            let crypt_path = format!("crypt://{}", path);
            Some(CryptFile::open(&crypt_path, mode)?.into_file())
        } else if self.should_encrypt(&path) && mode != "r" && mode != "rb" {
            drop(result_file);
            let crypt_path = format!("crypt://{}", path);
            Some(CryptFile::open(&crypt_path, mode)?.into_file())
        } else {
            Some(result_file)
        };
        
        // Re-enable the proxy
        FileProxy::set_enabled(proxy_status)?;
        
        Ok(new_result)
    }
    
    async fn post_get_file_info(&self, path: &str, mut data: FileInfo) -> Result<FileInfo, Error> {
        // If path is a folder do nothing
        if App::is_enabled("files_encryption")? && data.contains_key("size") {
            // Disable encryption proxy to prevent recursive calls
            let proxy_status = FileProxy::disable()?;
            
            // Get file size
            if let Some(size) = data.get("size") {
                if let Some(new_size) = self.post_file_size(path, *size).await? {
                    data.insert("size".to_string(), new_size);
                }
            }
            
            // Re-enable the proxy
            FileProxy::set_enabled(proxy_status)?;
        }
        
        Ok(data)
    }
    
    async fn post_file_size(&self, path: &str, size: u64) -> Result<Option<u64>, Error> {
        let view = FilesystemView::new("/")?;
        
        let user_id = User::get_user()?;
        let util = Util::new(&view, &user_id);
        
        // If encryption is no longer enabled or if the files aren't migrated yet
        // we return the default file size
        if !App::is_enabled("files_encryption")? || 
           util.get_migration_status()? != Util::MIGRATION_COMPLETED {
            return Ok(Some(size));
        }
        
        // If path is a folder do nothing
        if view.is_dir(path)? {
            return Ok(Some(size));
        }
        
        // Get relative path
        let relative_path = Helper::strip_user_files_path(path);
        
        // If path is empty we cannot resolve anything
        if relative_path.is_none() {
            return Ok(Some(size));
        }
        
        let relative_path = relative_path.unwrap();
        
        let mut file_info = None;
        // Get file info from database/cache if not .part file
        if !Helper::is_partial_file_path(path) {
            file_info = view.get_file_info(path)?;
        }
        
        let new_size = if let Some(mut info) = file_info {
            if info.get("encrypted") == Some(&true) {
                // Try to fix unencrypted file size if it doesn't look plausible
                if info.get("size").copied().unwrap_or(0) > 0 && 
                   info.get("unencrypted_size").copied().unwrap_or(0) == 0 {
                    let fix_size = util.get_file_size(path)?;
                    info.insert("unencrypted_size".to_string(), fix_size);
                    
                    // Put file info if not .part file
                    if !Helper::is_partial_file_path(&relative_path) {
                        view.put_file_info(path, &info)?;
                    }
                }
                info.get("unencrypted_size").copied().unwrap_or(size)
            } else {
                // Self healing if file was removed from file cache
                let fix_size = util.get_file_size(path)?;
                if fix_size > 0 {
                    info.insert("encrypted".to_string(), true);
                    info.insert("unencrypted_size".to_string(), fix_size);
                    
                    // Put file info if not .part file
                    if !Helper::is_partial_file_path(&relative_path) {
                        view.put_file_info(path, &info)?;
                    }
                    
                    fix_size
                } else {
                    size
                }
            }
        } else {
            // Handle case with no file info
            let fix_size = util.get_file_size(path)?;
            if fix_size > 0 {
                let mut info = FileInfo::new();
                info.insert("encrypted".to_string(), true);
                info.insert("unencrypted_size".to_string(), fix_size);
                
                // Put file info if not .part file
                if !Helper::is_partial_file_path(&relative_path) {
                    view.put_file_info(path, &info)?;
                }
                
                fix_size
            } else {
                size
            }
        };
        
        Ok(Some(new_size))
    }
}

impl Proxy {
    /// Creates a new Proxy instance
    pub fn new() -> Self {
        Proxy {
            black_list: Arc::new(Mutex::new(None)),
        }
    }

    /// Check if a file requires encryption
    ///
    /// Tests if server side encryption is enabled, and file is allowed by blacklists
    fn should_encrypt(&self, path: &str) -> bool {
        if !App::is_enabled("files_encryption").unwrap_or(false) || 
           Crypt::mode() != "server" ||
           !path.contains(&format!("/{}/files", User::get_user().unwrap_or_default())) {
            return false;
        }
        
        // Initialize blacklist if not already done
        let mut black_list_guard = self.black_list.lock().expect("Failed to lock black_list");
        if black_list_guard.is_none() {
            let black_list_str = Config::get_app_value(
                "files_encryption", 
                "type_blacklist", 
                ""
            ).unwrap_or_default();
            
            *black_list_guard = Some(
                black_list_str.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            );
        }
        
        if Crypt::is_catfile_content(path) {
            return true;
        }
        
        if let Some(extension) = Path::new(path).extension() {
            if let Some(ext_str) = extension.to_str() {
                if let Some(ref black_list) = *black_list_guard {
                    return !black_list.contains(&ext_str.to_string());
                }
            }
        }
        
        true
    }

    /// Handle file operations for encryption
    async fn handle_file(&self, path: &str) -> Result<(), Error> {
        // Disable encryption proxy to prevent recursive calls
        let proxy_status = FileProxy::disable()?;
        
        let view = FilesystemView::new("/")?;
        let session = Session::new(&view);
        let user_id = User::get_user()?;
        let util = Util::new(&view, &user_id);
        
        // Split the path parts
        let path_parts: Vec<&str> = path.split('/').collect();
        
        // Get relative path
        let relative_path = Helper::strip_user_files_path(path);
        
        // Only if file is on 'files' folder fix file size and sharing
        if path_parts.len() > 2 && path_parts[2] == "files" {
            if let Some(rel_path) = relative_path {
                if util.fix_file_size(path)? {
                    // Get sharing app state
                    let sharing_enabled = Share::is_enabled()?;
                    
                    // Get users
                    let users_sharing = util.get_sharing_users_array(sharing_enabled, &rel_path)?;
                    
                    // Update sharing-keys
                    util.set_shared_file_keyfiles(&session, &users_sharing, &rel_path)?;
                }
            }
        }
        
        FileProxy::set_enabled(proxy_status)?;
        
        Ok(())
    }
}

// The following are mock types/traits assuming the rest of the codebase

type Error = Box<dyn std::error::Error + Send + Sync>;
type FileInfo = std::collections::HashMap<String, Value>;

enum Value {
    String(String),
    Bool(bool),
    U64(u64),
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::U64(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

enum LogLevel {
    Error,
}

#[async_trait]
trait FileProxy {
    async fn pre_file_put_contents(&self, path: &str, data: &mut Vec<u8>) -> Result<bool, Error>;
    async fn post_file_get_contents(&self, path: &str, data: &mut Vec<u8>) -> Result<(), Error>;
    async fn pre_unlink(&self, path: &str) -> Result<bool, Error>;
    async fn post_touch(&self, path: &str) -> Result<bool, Error>;
    async fn post_fopen(&self, path: &str, result: Option<File>) -> Result<Option<File>, Error>;
    async fn post_get_file_info(&self, path: &str, data: FileInfo) -> Result<FileInfo, Error>;
    async fn post_file_size(&self, path: &str, size: u64) -> Result<Option<u64>, Error>;
    
    fn disable() -> Result<bool, Error> {
        // Mock implementation
        Ok(true)
    }
    
    fn set_enabled(_enabled: bool) -> Result<(), Error> {
        // Mock implementation
        Ok(())
    }
}

struct FilesystemView {
    root: String,
}

impl FilesystemView {
    fn new(root: &str) -> Result<Self, Error> {
        Ok(Self {
            root: root.to_string(),
        })
    }
    
    fn file_get_contents(&self, path: &str) -> Result<Vec<u8>, Error> {
        // Mock implementation
        Ok(Vec::new())
    }
    
    fn unlink(&self, path: &str) -> Result<bool, Error> {
        // Mock implementation
        Ok(true)
    }
    
    fn is_dir(&self, path: &str) -> Result<bool, Error> {
        // Mock implementation
        Ok(false)
    }
    
    fn get_file_info(&self, path: &str) -> Result<Option<FileInfo>, Error> {
        // Mock implementation
        Ok(None)
    }
    
    fn put_file_info(&self, path: &str, info: &FileInfo) -> Result<bool, Error> {
        // Mock implementation
        Ok(true)
    }
}

struct CryptFile;

impl CryptFile {
    fn open(path: &str, mode: &str) -> Result<Self, Error> {
        // Mock implementation
        Ok(Self)
    }
    
    fn write_all(&mut self, data: &[u8]) -> Result<(), Error> {
        // Mock implementation
        Ok(())
    }
    
    fn read_to_string(&mut self, buf: &mut String) -> Result<usize, Error> {
        // Mock implementation
        Ok(0)
    }
    
    fn into_file(self) -> File {
        // Mock implementation
        File::open("/dev/null").unwrap()
    }
}

struct App;

impl App {
    fn is_enabled(app: &str) -> Result<bool, Error> {
        // Mock implementation
        Ok(false)
    }
}

struct User;

impl User {
    fn get_user() -> Result<String, Error> {
        // Mock implementation
        Ok("".to_string())
    }
}

struct Config;

impl Config {
    fn get_app_value(app: &str, key: &str, default: &str) -> Result<String, Error> {
        // Mock implementation
        Ok(default.to_string())
    }
}

struct Share;

impl Share {
    fn is_enabled() -> Result<bool, Error> {
        // Mock implementation
        Ok(false)
    }
}

struct Filesystem;

impl Filesystem {
    fn normalize_path(path: &str) -> String {
        // Mock implementation
        path.to_string()
    }
}

struct AppSession;

impl AppSession {
    fn exists(key: &str) -> bool {
        // Mock implementation
        false
    }
}