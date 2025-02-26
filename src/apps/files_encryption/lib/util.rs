//! ownCloud Encryption Utility
//!
//! @author Sam Tuke, Frank Karlitschek
//! @copyright 2012 Sam Tuke <samtuke@owncloud.com>, Frank Karlitschek <frank@owncloud.org>
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

use std::collections::HashMap;
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;
use log::{info, warn, error, debug};

use crate::crypt::Crypt;
use crate::helper::Helper;
use crate::keymanager::Keymanager;
use crate::session::Session;
use crate::filesystem::{FilesystemView, FileInfo, FileProxy};

/// Utility class for operations relating to encrypted file storage system
pub struct Util {
    // Constants for migration status
    pub const MIGRATION_COMPLETED: i32 = 1;
    pub const MIGRATION_IN_PROGRESS: i32 = -1;
    pub const MIGRATION_OPEN: i32 = 0;

    view: FilesystemView,
    user_id: String,
    client: bool,
    public_key_dir: String,
    encryption_dir: String,
    keyfiles_path: String,
    share_keys_path: String,
    public_key_path: String,
    private_key_path: String,
    public_share_key_id: String,
    recovery_key_id: String,
    is_public: bool,
    user_dir: String,
    file_folder_name: String,
    user_files_dir: String,
}

impl Util {
    /// Create a new Util instance
    ///
    /// # Arguments
    ///
    /// * `view` - FilesystemView object for filesystem operations
    /// * `user_id` - ID of the currently logged-in user
    /// * `client` - Client side encryption mode flag (optional)
    pub fn new(view: FilesystemView, user_id: String, client: bool) -> Self {
        let app_config = AppConfig::new();
        let public_share_key_id = app_config.get_value("files_encryption", "publicShareKeyId")
            .unwrap_or_else(|| String::from(""));
        let recovery_key_id = app_config.get_value("files_encryption", "recoveryKeyId")
            .unwrap_or_else(|| String::from(""));
        
        let mut util = Self {
            view,
            user_id,
            client,
            public_key_dir: String::new(),
            encryption_dir: String::new(), 
            keyfiles_path: String::new(),
            share_keys_path: String::new(),
            public_key_path: String::new(),
            private_key_path: String::new(),
            public_share_key_id,
            recovery_key_id,
            is_public: false,
            user_dir: String::new(),
            file_folder_name: String::new(),
            user_files_dir: String::new(),
        };
        
        // If we are anonymous/public
        if Helper::is_public_access() {
            util.user_id = util.public_share_key_id.clone();
            
            // Only handle for files_sharing app
            if let Some(app) = globals::get("app") {
                if app == "files_sharing" {
                    let file_owner = globals::get("fileOwner").unwrap_or_default();
                    util.user_dir = format!("/{}", file_owner);
                    util.file_folder_name = "files".to_string();
                    util.user_files_dir = format!("/{}/{}", file_owner, util.file_folder_name);
                    util.public_key_dir = "/public-keys".to_string();
                    util.encryption_dir = format!("/{}/files_encryption", file_owner);
                    util.keyfiles_path = format!("{}/keyfiles", util.encryption_dir);
                    util.share_keys_path = format!("{}/share-keys", util.encryption_dir);
                    util.public_key_path = format!("{}/{}.public.key", util.public_key_dir, util.user_id);
                    util.private_key_path = format!("/owncloud_private_key/{}.private.key", util.user_id);
                    util.is_public = true;
                }
            }
        } else {
            util.user_dir = format!("/{}", util.user_id);
            util.file_folder_name = "files".to_string();
            util.user_files_dir = format!("/{}/{}", util.user_id, util.file_folder_name);
            util.public_key_dir = "/public-keys".to_string();
            util.encryption_dir = format!("/{}/files_encryption", util.user_id);
            util.keyfiles_path = format!("{}/keyfiles", util.encryption_dir);
            util.share_keys_path = format!("{}/share-keys", util.encryption_dir);
            util.public_key_path = format!("{}/{}.public.key", util.public_key_dir, util.user_id);
            util.private_key_path = format!("{}/{}.private.key", util.encryption_dir, util.user_id);
        }
        
        util
    }
    
    /// Check if encryption is ready for this user
    pub fn ready(&self) -> bool {
        self.view.file_exists(&self.encryption_dir) &&
        self.view.file_exists(&self.keyfiles_path) &&
        self.view.file_exists(&self.share_keys_path) &&
        self.view.file_exists(&self.public_key_path) &&
        self.view.file_exists(&self.private_key_path)
    }
    
    /// Sets up user folders and keys for serverside encryption
    ///
    /// # Arguments
    ///
    /// * `passphrase` - Passphrase to encrypt server-stored private key with (optional)
    pub fn setup_server_side(&self, passphrase: Option<String>) -> bool {
        // Set directories to check / create
        let set_up_dirs = vec![
            &self.user_dir,
            &self.user_files_dir,
            &self.public_key_dir,
            &self.encryption_dir,
            &self.keyfiles_path,
            &self.share_keys_path
        ];
        
        // Check / create all necessary dirs
        for dir_path in set_up_dirs {
            if !self.view.file_exists(dir_path) {
                if let Err(e) = self.view.mkdir(dir_path) {
                    error!("Failed to create directory {}: {}", dir_path, e);
                    return false;
                }
            }
        }
        
        // Create user keypair, we should never override a keyfile
        if !self.view.file_exists(&self.public_key_path) && !self.view.file_exists(&self.private_key_path) {
            // Generate keypair
            let keypair = match Crypt::create_keypair() {
                Some(kp) => kp,
                None => {
                    error!("Failed to create keypair for user {}", self.user_id);
                    return false;
                }
            };
            
            // Disable file proxy
            let proxy_enabled = FileProxy::is_enabled();
            FileProxy::set_enabled(false);
            
            // Encrypt private key with user pwd as passphrase
            if let Some(pass) = passphrase {
                match Crypt::symmetric_encrypt_file_content(&keypair.private_key, &pass) {
                    Some(encrypted_private_key) => {
                        // Save key-pair
                        if let Err(e) = self.view.file_put_contents(&self.private_key_path, &encrypted_private_key) {
                            error!("Failed to write private key: {}", e);
                            FileProxy::set_enabled(proxy_enabled);
                            return false;
                        }
                        
                        if let Err(e) = self.view.file_put_contents(&self.public_key_path, &keypair.public_key) {
                            error!("Failed to write public key: {}", e);
                            FileProxy::set_enabled(proxy_enabled);
                            return false;
                        }
                    },
                    None => {
                        error!("Failed to encrypt private key for user {}", self.user_id);
                        FileProxy::set_enabled(proxy_enabled);
                        return false;
                    }
                }
            } else {
                error!("No passphrase provided for encryption setup");
                FileProxy::set_enabled(proxy_enabled);
                return false;
            }
            
            // Restore file proxy state
            FileProxy::set_enabled(proxy_enabled);
        } else {
            // Check if public-key exists but private-key is missing
            if self.view.file_exists(&self.public_key_path) && !self.view.file_exists(&self.private_key_path) {
                error!("Public key exists but private key is missing for \"{}\"", self.user_id);
                return false;
            } else if !self.view.file_exists(&self.public_key_path) && self.view.file_exists(&self.private_key_path) {
                error!("Private key exists but public key is missing for \"{}\"", self.user_id);
                return false;
            }
        }
        
        // If there's no record for this user's encryption preferences
        if !self.recovery_enabled_for_user().is_ok() {
            // Create database configuration
            let query = "INSERT INTO `*PREFIX*encryption` (`uid`,`mode`,`recovery_enabled`,`migration_status`) VALUES (?,?,?,?)";
            let args = vec![
                DBParam::String(self.user_id.clone()),
                DBParam::String("server-side".to_string()),
                DBParam::Int(0),
                DBParam::Int(Self::MIGRATION_OPEN),
            ];
            
            if let Err(e) = DB::execute(query, args) {
                error!("Failed to insert encryption record: {}", e);
                return false;
            }
        }
        
        true
    }
    
    /// Get the public share key ID
    pub fn get_public_share_key_id(&self) -> &str {
        &self.public_share_key_id
    }
    
    /// Check whether pwd recovery is enabled for a given user
    ///
    /// # Returns
    ///
    /// * `Ok(1)` - Yes
    /// * `Ok(0)` - No
    /// * `Err` - No record
    pub fn recovery_enabled_for_user(&self) -> Result<i32, String> {
        let query = "SELECT `recovery_enabled` FROM `*PREFIX*encryption` WHERE `uid` = ?";
        let args = vec![DBParam::String(self.user_id.clone())];
        
        match DB::query(query, args) {
            Ok(result) => {
                if result.num_rows() > 0 {
                    if let Some(row) = result.fetch_row() {
                        if let Some(DBValue::Int(recovery_enabled)) = row.get("recovery_enabled") {
                            return Ok(recovery_enabled);
                        }
                    }
                }
                Err("No record found".to_string())
            },
            Err(e) => {
                error!("DB error: {}", e);
                Err(e)
            }
        }
    }
    
    /// Enable / disable pwd recovery for a given user
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable or disable recovery
    pub fn set_recovery_for_user(&self, enabled: bool) -> bool {
        let enabled_int = if enabled { 1 } else { 0 };
        
        let (query, args) = match self.recovery_enabled_for_user() {
            // If no record exists, create one
            Err(_) => {
                (
                    "INSERT INTO `*PREFIX*encryption` (`uid`,`mode`,`recovery_enabled`) VALUES (?,?,?)",
                    vec![
                        DBParam::String(self.user_id.clone()),
                        DBParam::String("server-side".to_string()),
                        DBParam::Int(enabled_int),
                    ]
                )
            },
            // If a record exists, update it
            Ok(_) => {
                (
                    "UPDATE `*PREFIX*encryption` SET `recovery_enabled` = ? WHERE `uid` = ?",
                    vec![
                        DBParam::Int(enabled_int),
                        DBParam::String(self.user_id.clone()),
                    ]
                )
            }
        };
        
        match DB::execute(query, args) {
            Ok(rows) => rows > 0,
            Err(e) => {
                error!("DB error: {}", e);
                false
            }
        }
    }
    
    /// Find all files and their encryption status within a directory
    ///
    /// # Arguments
    ///
    /// * `directory` - The path of the parent directory to search
    ///
    /// # Returns
    ///
    /// A HashMap with keys 'plain', 'encrypted', and 'legacy', each containing
    /// a vector of files with that encryption status.
    pub fn find_enc_files(&self, directory: &str) -> Option<HashMap<String, Vec<FileInfo>>> {
        let mut found = HashMap::new();
        found.insert("plain".to_string(), Vec::new());
        found.insert("encrypted".to_string(), Vec::new());
        found.insert("legacy".to_string(), Vec::new());
        
        self.find_enc_files_recursive(directory, &mut found)
    }
    
    fn find_enc_files_recursive(&self, directory: &str, found: &mut HashMap<String, Vec<FileInfo>>) -> Option<HashMap<String, Vec<FileInfo>>> {
        // Disable proxy - we don't want files to be decrypted before we handle them
        let proxy_enabled = FileProxy::is_enabled();
        FileProxy::set_enabled(false);
        
        if !self.view.is_dir(directory) {
            FileProxy::set_enabled(proxy_enabled);
            return None;
        }
        
        let handle = match self.view.opendir(directory) {
            Ok(h) => h,
            Err(_) => {
                FileProxy::set_enabled(proxy_enabled);
                return None;
            }
        };
        
        for entry in handle {
            let file = match entry {
                Ok(f) => f,
                Err(_) => continue,
            };
            
            let file_name = file.name();
            if file_name == "." || file_name == ".." {
                continue;
            }
            
            let file_path = format!("{}/{}", directory, self.view.get_relative_path(&format!("/{}", file_name)));
            let rel_path = Helper::strip_user_files_path(&file_path);
            
            // If the path is a directory, search its contents
            if self.view.is_dir(&file_path) {
                self.find_enc_files_recursive(&file_path, found);
            } else if self.view.is_file(&file_path) {
                // Disable proxies again
                FileProxy::set_enabled(false);
                
                let is_encrypted_path = self.is_encrypted_path(&file_path);
                
                // Determine file encryption status
                if Keymanager::get_file_key(&self.view, &self.user_id, &rel_path).is_some() && is_encrypted_path {
                    // The file is encrypted
                    if let Some(encrypted_files) = found.get_mut("encrypted") {
                        encrypted_files.push(FileInfo {
                            name: file_name.to_string(),
                            path: file_path.to_string(),
                        });
                    }
                } else if Crypt::is_legacy_encrypted_content(is_encrypted_path, &rel_path) {
                    // The file uses old encryption system
                    if let Some(legacy_files) = found.get_mut("legacy") {
                        legacy_files.push(FileInfo {
                            name: file_name.to_string(),
                            path: file_path.to_string(),
                        });
                    }
                } else {
                    // The file is not encrypted
                    if let Some(plain_files) = found.get_mut("plain") {
                        plain_files.push(FileInfo {
                            name: file_name.to_string(),
                            path: rel_path.to_string(),
                        });
                    }
                }
            }
        }
        
        FileProxy::set_enabled(proxy_enabled);
        
        if found.values().all(|v| v.is_empty()) {
            None
        } else {
            Some(found.clone())
        }
    }
    
    /// Fetch the last lines of a file efficiently
    ///
    /// # Arguments
    ///
    /// * `filename` - Path to the file
    /// * `num_lines` - Number of lines to read from the end
    ///
    /// # Returns
    ///
    /// The last lines of the file as a String
    pub fn tail(&self, filename: &str, mut num_lines: usize) -> Result<String, std::io::Error> {
        let proxy_enabled = FileProxy::is_enabled();
        FileProxy::set_enabled(false);
        
        let mut text = String::new();
        let mut pos: i64 = -1;
        
        let mut handle = self.view.fopen(filename, "r")?;
        
        while num_lines > 0 {
            pos -= 1;
            
            if handle.seek(SeekFrom::End(pos)).is_err() {
                handle.seek(SeekFrom::Start(0))?;
                num_lines = 0;
            } else {
                let mut buf = [0; 1];
                handle.read_exact(&mut buf)?;
                if buf[0] == b'\n' {
                    num_lines -= 1;
                }
            }
            
            let block_size = (-pos as usize) % 8192;
            if block_size == 0 || num_lines == 0 {
                let size = if block_size == 0 { 8192 } else { block_size };
                handle.seek(SeekFrom::End(pos))?;
                let mut buffer = vec![0; size];
                let bytes_read = handle.read(&mut buffer)?;
                buffer.truncate(bytes_read);
                let chunk = String::from_utf8_lossy(&buffer);
                text = format!("{}{}", chunk, text);
            }
        }
        
        FileProxy::set_enabled(proxy_enabled);
        
        Ok(text)
    }
    
    /// Check if a given path identifies an encrypted file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file
    pub fn is_encrypted_path(&self, path: &str) -> bool {
        // Disable encryption proxy so data retrieved is in its original form
        let proxy_enabled = FileProxy::is_enabled();
        FileProxy::set_enabled(false);
        
        let mut data = String::new();
        if let Ok(mut handle) = self.view.fopen(path, "r") {
            if handle.seek(SeekFrom::End(-24)).is_ok() {
                let _ = handle.read_to_string(&mut data);
            }
        }
        
        // Re-enable proxy
        FileProxy::set_enabled(proxy_enabled);
        
        Crypt::is_catfile_content(&data)
    }
    
    /// Get the file size of the unencrypted file
    ///
    /// # Arguments
    ///
    /// * `path` - Absolute path to the file
    pub fn get_file_size(&self, path: &str) -> u64 {
        let mut result = 0;
        
        // Disable encryption proxy to prevent recursive calls
        let proxy_enabled = FileProxy::is_enabled();
        FileProxy::set_enabled(false);
        
        // Split the path parts
        let path_parts: Vec<&str> = path.split('/').collect();
        
        if path_parts.len() > 2 && path_parts[2] == "files" && 
           self.view.file_exists(path) && self.is_encrypted_path(path) {
            
            // Get the size from filesystem
            let size = match self.view.filesize(path) {
                Ok(s) => s,
                Err(_) => {
                    FileProxy::set_enabled(proxy_enabled);
                    return 0;
                }
            };
            
            // Fast path, else the calculation for last_chunk_nr is bogus
            if size == 0 {
                FileProxy::set_enabled(proxy_enabled);
                return 0;
            }
            
            // Calculate last chunk nr
            // Next highest is end of chunks, one subtracted is last one
            // We have to read the last chunk, we can't just calculate it (because of padding etc)
            let last_chunk_nr = (size as f64 / 8192.0).ceil() as u64 - 1;
            let last_chunk_size = size - (last_chunk_nr * 8192);
            
            // Open stream
            if let Ok(mut stream) = self.view.fopen(&format!("crypt://{}", path), "r") {
                // Calculate last chunk position
                let last_chunk_pos = last_chunk_nr * 8192;
                
                // Seek to end
                if stream.seek(SeekFrom::Start(last_chunk_pos)).is_ok() {
                    // Get the content of the last chunk
                    let mut last_chunk_content = Vec::new();
                    if let Ok(bytes_read) = stream.take(last_chunk_size).read_to_end(&mut last_chunk_content) {
                        // Calc the real file size with the size of the last chunk
                        result = ((last_chunk_nr * 6126) + bytes_read as u64) as u64;
                    }
                }
            }
        }
        
        FileProxy::set_enabled(proxy_enabled);
        
        result
    }
    
    /// Fix the file size of the encrypted file
    ///
    /// # Arguments
    ///
    /// * `path` - Absolute path to the file
    pub fn fix_file_size(&self, path: &str) -> bool {
        let mut result = false;
        
        // Disable encryption proxy to prevent recursive calls
        let proxy_enabled = FileProxy::is_enabled();
        FileProxy::set_enabled(false);
        
        let real_size = self.get_file_size(path);
        
        if real_size > 0 {
            match self.view.get_file_info(path) {
                Ok(mut cached) => {
                    cached.encrypted = true;
                    cached.unencrypted_size = real_size;
                    
                    // Put file info
                    if self.view.put_file_info(path, &cached).is_ok() {
                        result = true;
                    }
                },
                Err(e) => {
                    error!("Failed to get file info: {}", e);
                }
            }
        }
        
        FileProxy::set_enabled(proxy_enabled);
        
        result
    }
    
    /// Check if a path is a shared path
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check
    pub fn is_shared_path(&self, path: &str) -> bool {
        let trimmed = path.trim_start_matches('/');
        let split: Vec<&str> = trimmed.split('/').collect();
        
        split.len() > 2 && split[2] == "Shared"
    }
    
    /// Encrypt versions from given file
    ///
    /// # Arguments
    ///
    /// * `filelist` - List of encrypted files, relative to data/user/files
    fn encrypt_versions(&self, filelist: &[String]) -> bool {
        let mut successful = true;
        
        if app_is_enabled("files_versions") {
            for filename in filelist {
                let versions = FilesVersions::get_versions(&self.user_id, filename);
                
                for version in versions {
                    let path = format!("/{}/files_versions/{}.v{}", 
                                      self.user_id, version.path, version.version);
                    
                    match self.view.fopen(&format!("crypt://{}.part", path), "wb") {
                        Ok(mut enc_handle) => {
                            match self.view.fopen(&path, "rb") {
                                Ok(mut plain_handle) => {
                                    let mut buffer = Vec::new();
                                    if let Ok(_) = plain_handle.read_to_end(&mut buffer) {
                                        if let Err(_) = enc_handle.write_all(&buffer) {
                                            error!("Failed to write to encrypted version file");
                                            successful = false;
                                        }
                                    }
                                },
                                Err(_) => {
                                    error!("Couldn't open \"{}.part\", encryption failed!", path);
                                    successful = false;
                                }
                            }
                        },
                        Err(_) => {
                            error!("Couldn't open \"{}\", encryption failed!", path);
                            successful = false;
                        }
                    }
                    
                    if let Err(_) = self.view.rename(&format!("{}.part", path), &path) {
                        error!("Failed to rename version file after encryption");
                        successful = false;
                    }
                }
            }
        }
        
        successful
    }
    
    /// Decrypt versions from given file
    ///
    /// # Arguments
    ///
    /// * `filelist` - List of decrypted files, relative to data/user/files
    fn decrypt_versions(&self, filelist: &[String]) -> bool {
        let mut successful = true;
        
        if app_is_enabled("files_versions") {
            for filename in filelist {
                let versions = FilesVersions::get_versions(&self.user_id, filename);
                
                for version in versions {
                    let path = format!("/{}/files_versions/{}.v{}", 
                                      self.user_id, version.path, version.version);
                    
                    match self.view.fopen(&format!("crypt://{}", path), "rb") {
                        Ok(mut enc_handle) => {
                            match self.view.fopen(&format!("{}.part", path), "wb") {
                                Ok(mut plain_handle) => {
                                    let mut buffer = Vec::new();
                                    if let Ok(_) = enc_handle.read_to_end(&mut buffer) {
                                        if let Err(_) = plain_handle.write_all(&buffer) {
                                            error!("Failed to write to decrypted version file");
                                            successful = false;
                                        }
                                    }
                                },
                                Err(_) => {
                                    error!("Couldn't open \"{}.part\", decryption failed!", path);
                                    successful = false;
                                }
                            }
                        },
                        Err(_) => {
                            error!("Couldn't open \"{}\", decryption failed!", path);
                            successful = false;
                        }
                    }
                    
                    if let Err(_) = self.view.rename(&format!("{}.part", path), &path) {
                        error!("Failed to rename version file after decryption");
                        successful = false;
                    }
                }
            }
        }
        
        successful
    }
    
    /// Decrypt all files
    pub fn decrypt_all(&self) -> bool {
        let found = match self.find_enc_files(&format!("{}/files", self.user_id)) {
            Some(f) => f,
            None => return true,
        };
        
        let mut successful = true;
        
        let version_status = app_is_enabled("files_versions");
        if version_status {
            app_disable("files_versions");
        }
        
        let mut decrypted_files = Vec::new();
        
        // Process encrypted files
        if let Some(encrypted_files) = found.get("encrypted") {
            for encrypted_file in encrypted_files {
                // Get file info
                let file_info = match filesystem::get_file_info(&encrypted_file.path) {
                    Ok(info) => info,
                    Err(_) => {
                        successful = false;
                        continue;
                    }
                };
                
                // Path relative to data/<user>/file
                let rel_path = Helper::strip_user_files_path(&encrypted_file.path);
                
                // Path relative to /data
                let raw_path = &encrypted_file.path;
                
                // Get timestamp
                let timestamp = match self.view.filemtime(raw_path) {
                    Ok(t) => t,
                    Err(_) => {
                        successful = false;
                        continue;
                    }
                };
                
                // Enable proxy to use FilesystemView to access the original file
                FileProxy::set_enabled(true);
                
                // Open enc file handle for binary reading
                let mut enc_handle = match self.view.fopen(raw_path, "rb") {
                    Ok(h) => h,
                    Err(_) => {
                        error!("Couldn't open \"{}\", decryption failed!", raw_path);
                        successful = false;
                        continue;
                    }
                };
                
                // Disable proxy to prevent file being encrypted again
                FileProxy::set_enabled(false);
                
                // Open plain file handle for binary writing
                let mut plain_handle = match self.view.fopen(&format!("{}.part", raw_path), "wb") {
                    Ok(h) => h,
                    Err(_) => {
                        error!("Couldn't open \"{}.part\", decryption failed!", raw_path);
                        successful = false;
                        continue;
                    }
                };
                
                // Copy file content
                let mut buffer = Vec::new();
                let size = match enc_handle.read_to_end(&mut buffer) {
                    Ok(s) => s,
                    Err(_) => {
                        error!("Failed to read encrypted file");
                        successful = false;
                        continue;
                    }
                };
                
                if size == 0 {
                    error!("Zero bytes copied of \"{}\", decryption failed!", raw_path);
                    successful = false;