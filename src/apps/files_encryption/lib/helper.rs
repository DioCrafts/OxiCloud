// Copyright (c) 2013 Florin Peter <owncloud@florin-peter.de>
//
// SPDX-License-Identifier: AGPL-3.0-or-later
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library. If not, see <http://www.gnu.org/licenses/>.

use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::time::SystemTime;
use std::fs;
use log::{debug, error};

/// Helper class to manage hooks registration and various helper methods
pub struct Helper;

impl Helper {
    /// Register share related hooks
    pub fn register_share_hooks() {
        // In Rust we'd typically use a registry or event system pattern
        // This is a simplified translation of the concept
        util::connect_hook("OCP\\Share", "pre_shared", "OCA\\Encryption\\Hooks", "pre_shared");
        util::connect_hook("OCP\\Share", "post_shared", "OCA\\Encryption\\Hooks", "post_shared");
        util::connect_hook("OCP\\Share", "post_unshare", "OCA\\Encryption\\Hooks", "post_unshare");
    }

    /// Register user related hooks
    pub fn register_user_hooks() {
        util::connect_hook("OC_User", "post_login", "OCA\\Encryption\\Hooks", "login");
        util::connect_hook("OC_User", "post_set_password", "OCA\\Encryption\\Hooks", "set_passphrase");
        util::connect_hook("OC_User", "pre_set_password", "OCA\\Encryption\\Hooks", "pre_set_passphrase");
        util::connect_hook("OC_User", "post_create_user", "OCA\\Encryption\\Hooks", "post_create_user");
        util::connect_hook("OC_User", "post_delete_user", "OCA\\Encryption\\Hooks", "post_delete_user");
    }

    /// Register filesystem related hooks
    pub fn register_filesystem_hooks() {
        util::connect_hook("OC_Filesystem", "post_rename", "OCA\\Encryption\\Hooks", "post_rename");
    }

    /// Register app management related hooks
    pub fn register_app_hooks() {
        util::connect_hook("OC_App", "pre_disable", "OCA\\Encryption\\Hooks", "pre_disable");
        util::connect_hook("OC_App", "post_disable", "OCA\\Encryption\\Hooks", "post_enable");
    }

    /// Setup user for files_encryption
    pub fn setup_user(util: &Util, password: &str) -> bool {
        // Check files_encryption infrastructure is ready for action
        if !util.ready() {
            debug!("Encryption library: User account \"{}\" is not ready for encryption; configuration started", 
                util.get_user_id());

            if !util.setup_server_side(password) {
                return false;
            }
        }

        true
    }

    /// Enable recovery
    pub fn admin_enable_recovery(recovery_key_id: Option<&str>, recovery_password: &str) -> bool {
        let view = FileView::new("/");
        
        let recovery_key_id = match recovery_key_id {
            Some(id) => id.to_string(),
            None => {
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let random_id = md5::compute(timestamp.to_string()).0[0..8].iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>();
                let new_id = format!("recovery_{}", random_id);
                app_config::set_value("files_encryption", "recoveryKeyId", &new_id);
                new_id
            }
        };

        if !view.is_dir("/owncloud_private_key") {
            if let Err(_) = view.mkdir("/owncloud_private_key") {
                return false;
            }
        }

        if !view.file_exists(&format!("/public-keys/{}.public.key", recovery_key_id)) ||
           !view.file_exists(&format!("/owncloud_private_key/{}.private.key", recovery_key_id)) {
            
            let keypair = Crypt::create_keypair().unwrap_or_default();
            
            file_proxy::set_enabled(false);

            // Save public key
            if !view.is_dir("/public-keys") {
                if let Err(_) = view.mkdir("/public-keys") {
                    file_proxy::set_enabled(true);
                    return false;
                }
            }

            if let Err(_) = view.file_put_contents(
                &format!("/public-keys/{}.public.key", recovery_key_id), 
                keypair.public_key.as_bytes()
            ) {
                file_proxy::set_enabled(true);
                return false;
            }

            // Encrypt private key with empty passphrase
            let encrypted_private_key = match Crypt::symmetric_encrypt_file_content(
                &keypair.private_key, 
                recovery_password
            ) {
                Ok(key) => key,
                Err(_) => {
                    file_proxy::set_enabled(true);
                    return false;
                }
            };

            // Save private key
            if let Err(_) = view.file_put_contents(
                &format!("/owncloud_private_key/{}.private.key", recovery_key_id),
                encrypted_private_key.as_bytes()
            ) {
                file_proxy::set_enabled(true);
                return false;
            }

            file_proxy::set_enabled(true);

            // Set recoveryAdmin as enabled
            app_config::set_value("files_encryption", "recoveryAdminEnabled", "1");

            true
        } else {
            // Get recovery key and check the password
            let util = Util::new(FileView::new("/"), &user::get_user());
            let check_result = util.check_recovery_password(recovery_password);
            
            if check_result {
                app_config::set_value("files_encryption", "recoveryAdminEnabled", "1");
            }
            
            check_result
        }
    }

    /// Check if a path is a .part file
    pub fn is_partial_file_path(path: &str) -> bool {
        let path = Path::new(path);
        match path.extension() {
            Some(ext) => ext == "part" || ext == "etmp",
            None => false,
        }
    }

    /// Remove .part extension from a file path
    /// This is needed for reusing keys
    pub fn strip_partial_file_extension(path: &str) -> String {
        let path = Path::new(path);
        
        if let Some(ext) = path.extension() {
            if ext == "part" || ext == "etmp" {
                if let Some(path_without_ext) = path.parent().and_then(|p| p.join(path.file_stem().unwrap_or_default()).to_str()) {
                    // Check for transaction ID
                    let new_path = Path::new(path_without_ext);
                    if let Some(ext) = new_path.extension() {
                        if let Some(ext_str) = ext.to_str() {
                            if ext_str.starts_with("ocTransferId") {
                                if let Some(base_path) = new_path.parent().and_then(|p| p.join(new_path.file_stem().unwrap_or_default()).to_str()) {
                                    return base_path.to_string();
                                }
                            }
                        }
                    }
                    return path_without_ext.to_string();
                }
            }
        }
        
        path.to_str().unwrap_or("").to_string()
    }

    /// Disable recovery
    pub fn admin_disable_recovery(recovery_password: &str) -> bool {
        let util = Util::new(FileView::new("/"), &user::get_user());
        let check_result = util.check_recovery_password(recovery_password);

        if check_result {
            app_config::set_value("files_encryption", "recoveryAdminEnabled", "0");
        }

        check_result
    }

    /// Checks if access is public/anonymous user
    pub fn is_public_access() -> bool {
        if user::get_user().is_empty() || 
           (get_param("service") == Some("files".to_string()) && 
            get_param("t").is_some()) {
            return true;
        }
        
        false
    }

    /// Format a path to be relative to the /user/files/ directory
    pub fn strip_user_files_path(path: &str) -> Option<String> {
        let trimmed = path.trim_start_matches('/');
        let split: Vec<&str> = trimmed.split('/').collect();

        // It is not a file relative to data/user/files
        if split.len() < 3 || split[1] != "files" {
            return None;
        }

        let sliced = &split[2..];
        let rel_path = sliced.join("/");

        Some(rel_path)
    }

    /// Get path to the corresponding file in data/user/files
    pub fn get_path_to_real_file(path: &str) -> Option<String> {
        let trimmed = path.trim_start_matches('/');
        let split: Vec<&str> = trimmed.split('/').collect();

        if split.len() < 3 || split[1] != "files_versions" {
            return None;
        }

        let sliced = &split[2..];
        let real_path = sliced.join("/");
        
        // Remove the last .v
        if let Some(pos) = real_path.rfind(".v") {
            Some(real_path[0..pos].to_string())
        } else {
            Some(real_path)
        }
    }

    /// Redirect to an error page
    pub fn redirect_to_error_page(session: &Session, error_code: Option<i32>) {
        let error_code = match error_code {
            Some(code) => code,
            None => {
                match session.get_initialized() {
                    SessionInitState::Executed => Crypt::ENCRYPTION_PRIVATE_KEY_NOT_VALID_ERROR,
                    SessionInitState::NotInitialized => Crypt::ENCRYPTION_NOT_INITIALIZED_ERROR,
                    _ => Crypt::ENCRYPTION_UNKNOWN_ERROR,
                }
            }
        };

        let location = helper::link_to_absolute("apps/files_encryption/files", "error.php");
        let post = if has_post_data() { 1 } else { 0 };
        
        // In a real implementation, this would be handled by the web framework
        redirect(&format!("{}?p={}&errorCode={}", location, post, error_code));
    }

    /// Check requirements for encryption app
    pub fn check_requirements() -> bool {
        // Check for OpenSSL support
        let has_openssl = cfg!(feature = "openssl");
        
        // In Rust we don't need to check PHP version
        // Instead we'd check compatibility with the Rust version at compile time
        
        has_openssl
    }

    /// Check some common errors if the server isn't configured properly for encryption
    pub fn check_configuration() -> bool {
        Self::get_openssl_pkey().is_ok()
    }

    /// Create an openssl pkey with config-supplied settings
    /// WARNING: This initializes a new private keypair, which is computationally expensive
    pub fn get_openssl_pkey() -> Result<PKey<Private>, openssl::error::ErrorStack> {
        let config = Self::get_openssl_config();
        let bits = config.get("private_key_bits").unwrap_or(&4096);
        
        let rsa = Rsa::generate(*bits)?;
        PKey::from_rsa(rsa)
    }

    /// Return a HashMap of OpenSSL config options, default + config
    pub fn get_openssl_config() -> HashMap<String, i32> {
        let mut config = HashMap::new();
        config.insert("private_key_bits".to_string(), 4096);
        
        // Merge with system config
        let system_config = config::get_system_value("openssl", HashMap::new());
        
        for (key, value) in system_config {
            if let Some(int_val) = value.as_i64() {
                config.insert(key, int_val as i32);
            }
        }
        
        config
    }

    /// Glob uses different pattern than regular expressions, escape glob pattern only
    pub fn escape_glob_pattern(path: &str) -> String {
        path.replace("*", "[*]")
            .replace("?", "[?]")
            .replace("[", "[[]")
    }
}

// Note: The following structures are placeholders to make the code compile
// In a real implementation, these would be defined in their own modules

struct util;
impl util {
    fn connect_hook(_provider: &str, _event: &str, _class: &str, _method: &str) {
        // Implementation would connect to the event system
    }
}

struct FileView {
    root: String,
}

impl FileView {
    fn new(root: &str) -> Self {
        Self { 
            root: root.to_string() 
        }
    }
    
    fn is_dir(&self, path: &str) -> bool {
        Path::new(&format!("{}{}", self.root, path)).is_dir()
    }
    
    fn mkdir(&self, path: &str) -> Result<(), std::io::Error> {
        fs::create_dir_all(format!("{}{}", self.root, path))
    }
    
    fn file_exists(&self, path: &str) -> bool {
        Path::new(&format!("{}{}", self.root, path)).exists()
    }
    
    fn file_put_contents(&self, path: &str, content: &[u8]) -> Result<(), std::io::Error> {
        fs::write(format!("{}{}", self.root, path), content)
    }
}

struct Util {
    view: FileView,
    user_id: String,
}

impl Util {
    fn new(view: FileView, user_id: &str) -> Self {
        Self {
            view,
            user_id: user_id.to_string(),
        }
    }
    
    fn ready(&self) -> bool {
        // Implementation would check if encryption is ready
        true
    }
    
    fn get_user_id(&self) -> &str {
        &self.user_id
    }
    
    fn setup_server_side(&self, _password: &str) -> bool {
        // Implementation would set up server-side encryption
        true
    }
    
    fn check_recovery_password(&self, _password: &str) -> bool {
        // Implementation would check recovery password
        true
    }
}

struct Crypt;

impl Crypt {
    const ENCRYPTION_PRIVATE_KEY_NOT_VALID_ERROR: i32 = 1;
    const ENCRYPTION_NOT_INITIALIZED_ERROR: i32 = 2;
    const ENCRYPTION_UNKNOWN_ERROR: i32 = 3;
    
    fn create_keypair() -> Result<KeyPair, String> {
        // Implementation would create a keypair
        Ok(KeyPair {
            public_key: "dummy_public_key".to_string(),
            private_key: "dummy_private_key".to_string(),
        })
    }
    
    fn symmetric_encrypt_file_content(content: &str, password: &str) -> Result<String, String> {
        // Implementation would encrypt content
        Ok(format!("encrypted_{}", content))
    }
}

struct KeyPair {
    public_key: String,
    private_key: String,
}

struct file_proxy;

impl file_proxy {
    fn set_enabled(_enabled: bool) {
        // Implementation would enable/disable file proxy
    }
}

struct app_config;

impl app_config {
    fn set_value(_app: &str, _key: &str, _value: &str) {
        // Implementation would set app config value
    }
}

struct user;

impl user {
    fn get_user() -> String {
        // Implementation would get current user
        "admin".to_string()
    }
}

enum SessionInitState {
    Executed,
    NotInitialized,
    Other,
}

struct Session;

impl Session {
    fn get_initialized(&self) -> SessionInitState {
        // Implementation would get session init state
        SessionInitState::Executed
    }
}

struct helper;

impl helper {
    fn link_to_absolute(_app: &str, _file: &str) -> String {
        // Implementation would generate absolute link
        "https://example.com/index.php".to_string()
    }
}

struct config;

impl config {
    fn get_system_value(_key: &str, default: HashMap<String, i32>) -> HashMap<String, serde_json::Value> {
        // Implementation would get system config
        let mut result = HashMap::new();
        for (key, value) in default {
            result.insert(key, serde_json::Value::Number(serde_json::Number::from(value)));
        }
        result
    }
}

fn get_param(name: &str) -> Option<String> {
    // Implementation would get request parameter
    if name == "service" {
        None
    } else {
        None
    }
}

fn has_post_data() -> bool {
    // Implementation would check for POST data
    false
}

fn redirect(_location: &str) {
    // Implementation would perform HTTP redirect
}