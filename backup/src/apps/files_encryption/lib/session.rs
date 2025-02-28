// Copyright (c) 2012 Sam Tuke samtuke@owncloud.com
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

use std::time::{SystemTime, UNIX_EPOCH};
use md5::{Md5, Digest};
use hex::encode;

use crate::encryption::crypt::Crypt;
use crate::encryption::helper::Helper;
use crate::app_config::AppConfig;
use crate::filesystem::FilesystemView;
use crate::session_manager::{Session as OcSession, SessionManager};
use crate::file_proxy::FileProxy;

/// Class for handling encryption related session data
pub struct Session {
    view: FilesystemView,
}

impl Session {
    pub const NOT_INITIALIZED: &'static str = "0";
    pub const INIT_EXECUTED: &'static str = "1";
    pub const INIT_SUCCESSFUL: &'static str = "2";

    /// If session is started, check if ownCloud key pair is set up, if not create it
    ///
    /// The ownCloud key pair is used to allow public link sharing even if encryption is enabled
    pub fn new(view: FilesystemView) -> Self {
        let mut session = Session { view };
        session.initialize();
        session
    }

    fn initialize(&mut self) {
        if !self.view.is_dir("owncloud_private_key") {
            self.view.mkdir("owncloud_private_key").unwrap_or_else(|e| {
                log::error!("Could not create directory owncloud_private_key: {}", e);
            });
        }

        let public_share_key_id = match AppConfig::get_value("files_encryption", "publicShareKeyId") {
            Some(id) => id,
            None => {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                let mut hasher = Md5::new();
                hasher.update(timestamp.to_string().as_bytes());
                let result = hasher.finalize();
                let hash = encode(&result[..]);
                
                let key_id = format!("pubShare_{}", &hash[0..8]);
                AppConfig::set_value("files_encryption", "publicShareKeyId", &key_id);
                key_id
            }
        };

        let public_key_path = format!("/public-keys/{}.public.key", public_share_key_id);
        let private_key_path = format!("/owncloud_private_key/{}.private.key", public_share_key_id);

        if !self.view.file_exists(&public_key_path) || !self.view.file_exists(&private_key_path) {
            let keypair = Crypt::create_keypair();

            // Disable encryption proxy to prevent recursive calls
            let proxy_status = FileProxy::enabled();
            FileProxy::set_enabled(false);

            // Save public key
            if !self.view.is_dir("/public-keys") {
                self.view.mkdir("/public-keys").unwrap_or_else(|e| {
                    log::error!("Could not create directory /public-keys: {}", e);
                });
            }

            self.view.file_put_contents(&public_key_path, &keypair.public_key).unwrap_or_else(|e| {
                log::error!("Could not save public key: {}", e);
            });

            // Encrypt private key with empty passphrase
            let encrypted_private_key = Crypt::symmetric_encrypt_file_content(&keypair.private_key, "").unwrap_or_else(|e| {
                log::error!("Could not encrypt private key: {}", e);
                String::new()
            });

            // Save private key
            self.view.file_put_contents(&private_key_path, &encrypted_private_key).unwrap_or_else(|e| {
                log::error!("Could not save private key: {}", e);
            });

            FileProxy::set_enabled(proxy_status);
        }

        if Helper::is_public_access() {
            // Disable encryption proxy to prevent recursive calls
            let proxy_status = FileProxy::enabled();
            FileProxy::set_enabled(false);

            match self.view.file_get_contents(&private_key_path) {
                Ok(encrypted_key) => {
                    match Crypt::decrypt_private_key(&encrypted_key, "") {
                        Ok(private_key) => {
                            self.set_public_share_private_key(&private_key);
                        }
                        Err(e) => {
                            log::error!("Could not decrypt private key: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Could not read private key file: {}", e);
                }
            }

            FileProxy::set_enabled(proxy_status);
        }
    }

    /// Sets user private key to session
    ///
    /// This should only be set on login
    pub fn set_private_key(&self, private_key: &str) -> bool {
        SessionManager::get().set("privateKey", private_key.to_string());
        true
    }

    /// Sets status of encryption app
    ///
    /// This doesn't indicate if the init was successful, we just remember the try!
    pub fn set_initialized(&self, init: &str) -> bool {
        SessionManager::get().set("encryptionInitialized", init.to_string());
        true
    }

    /// Gets status if we already tried to initialize the encryption app
    ///
    /// This doesn't indicate if the init was successful, we just remember the try!
    pub fn get_initialized(&self) -> String {
        SessionManager::get().get("encryptionInitialized")
            .unwrap_or_else(|| Self::NOT_INITIALIZED.to_string())
    }

    /// Gets user or public share private key from session
    pub fn get_private_key(&self) -> Option<String> {
        // return the public share private key if this is a public access
        if Helper::is_public_access() {
            self.get_public_share_private_key()
        } else {
            SessionManager::get().get("privateKey")
        }
    }

    /// Sets public user private key to session
    pub fn set_public_share_private_key(&self, private_key: &str) -> bool {
        SessionManager::get().set("publicSharePrivateKey", private_key.to_string());
        true
    }

    /// Gets public share private key from session
    pub fn get_public_share_private_key(&self) -> Option<String> {
        SessionManager::get().get("publicSharePrivateKey")
    }

    /// Sets user legacy key to session
    pub fn set_legacy_key(&self, legacy_key: &str) -> bool {
        SessionManager::get().set("legacyKey", legacy_key.to_string());
        true
    }

    /// Gets user legacy key from session
    pub fn get_legacy_key(&self) -> Option<String> {
        SessionManager::get().get("legacyKey")
    }
}