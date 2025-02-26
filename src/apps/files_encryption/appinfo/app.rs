// Rust implementation for apps/files_encryption/appinfo/app.php

use std::collections::HashSet;
use std::sync::Arc;

use once_cell::sync::Lazy;

use owncloud_sdk::{
    app::{self, AppManager},
    config::Config,
    filesystem::{FilesystemView, Filesystem},
    hooks::HookManager,
    stream::StreamManager,
    user::User,
};

use crate::encryption::{
    capabilities::Capabilities,
    crypt::Crypt,
    helper::Helper,
    hooks::Hooks,
    keymanager::KeyManager,
    proxy::Proxy,
    session::Session,
    stream::Stream,
    util::Util,
};

pub fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    // Register paths for our classes
    register_class_paths();

    // Only initialize if we're not in maintenance mode
    if !Config::get_value::<bool>("maintenance", false)? {
        // Register the encryption proxy
        let proxy = Proxy::new();
        FileProxy::register(Arc::new(proxy));

        // User related hooks
        Helper::register_user_hooks()?;

        // Sharing related hooks
        Helper::register_share_hooks()?;

        // Filesystem related hooks
        Helper::register_filesystem_hooks()?;

        // App manager related hooks
        Helper::register_app_hooks()?;

        // Register stream wrapper if not already registered
        let wrappers = StreamManager::get_wrappers();
        if !wrappers.contains("crypt") {
            StreamManager::register_wrapper("crypt", Stream::new)?;
        }

        // Check if we are logged in
        if User::is_logged_in()? {
            // Ensure filesystem is loaded
            if !Filesystem::is_loaded() {
                Util::setup_fs()?;
            }

            let view = FilesystemView::new("/")?;

            // Check requirements and create session if ready
            let session_ready = Helper::check_requirements()?;
            if session_ready {
                let _session = Session::new(view)?;
            }
        }
    } else {
        // Logout user if we are in maintenance to force re-login
        User::logout()?;
    }

    // Register settings scripts
    app::register_admin("files_encryption", "settings-admin")?;
    app::register_personal("files_encryption", "settings-personal")?;

    Ok(())
}

fn register_class_paths() {
    // In Rust we use modules instead of class paths, but this function
    // exists to document where the various components should be imported from
    // as a reference point for the original PHP code.
}