// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::config::OcConfig;
use crate::hooks::{register_hook, HookCategory, HookEvent};
use crate::registry::{register_admin_app, register_personal_app};
use crate::storage::{
    AmazonS3Storage, DavStorage, DropboxStorage, FtpStorage, GoogleStorage, IrodsStorage,
    SftpStorage, SmbStorage, StreamWrapperStorage, SwiftStorage,
};
use std::sync::Arc;

/// Register external storage modules and related components.
pub fn initialize(config: Arc<OcConfig>) -> Result<(), Box<dyn std::error::Error>> {
    // Register class paths in the module loader
    register_storage_modules()?;

    // Register admin section
    register_admin_app("files_external", "settings")?;

    // Register personal section if user mounting is allowed
    if config.get_app_value("files_external", "allow_user_mounting", "yes")? == "yes" {
        register_personal_app("files_external", "personal")?;
    }

    // Register hooks
    register_hook(
        HookCategory::User,
        HookEvent::PostLogin,
        IrodsStorage::login,
    )?;

    Ok(())
}

/// Register all storage modules in the system
fn register_storage_modules() -> Result<(), Box<dyn std::error::Error>> {
    // The class path registration in PHP is handled by our module system
    // These would be handled by the module system automatically through traits and implementations
    
    // In Rust, we might use a plugin system or dynamic registration instead
    // of the static class path mapping in PHP
    
    Ok(())
}