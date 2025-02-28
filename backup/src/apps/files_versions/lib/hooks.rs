// This module contains all hooks.
// 
// Copyright (c) 2012 Sam Tuke <samtuke@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;

/// Represents the Files_Versions hooks functionality
pub struct Hooks;

impl Hooks {
    /// Listen to write event.
    pub async fn write_hook(params: &HashMap<String, String>) -> Result<(), crate::Error> {
        if app::is_enabled("files_versions").await? {
            if let Some(path) = params.get("path") {
                if !path.is_empty() {
                    crate::storage::Storage::store(path).await?;
                }
            }
        }
        Ok(())
    }

    /// Erase versions of deleted file
    ///
    /// This function is connected to the delete signal of OC_Filesystem
    /// cleanup the versions directory if the actual file gets deleted
    pub async fn remove_hook(params: &HashMap<String, String>) -> Result<(), crate::Error> {
        if app::is_enabled("files_versions").await? {
            if let Some(path) = params.get("path") {
                if !path.is_empty() {
                    crate::storage::Storage::delete(path).await?;
                }
            }
        }
        Ok(())
    }

    /// Rename/move versions of renamed/moved files
    ///
    /// This function is connected to the rename signal of OC_Filesystem and adjust the name and location
    /// of the stored versions along the actual file
    pub async fn rename_hook(params: &HashMap<String, String>) -> Result<(), crate::Error> {
        if app::is_enabled("files_versions").await? {
            if let (Some(oldpath), Some(newpath)) = (params.get("oldpath"), params.get("newpath")) {
                if !oldpath.is_empty() && !newpath.is_empty() {
                    crate::storage::Storage::rename(oldpath, newpath).await?;
                }
            }
        }
        Ok(())
    }

    /// Clean up user specific settings if user gets deleted
    ///
    /// This function is connected to the pre_deleteUser signal of OC_Users
    /// to remove the used space for versions stored in the database
    pub async fn delete_user_hook(params: &HashMap<String, String>) -> Result<(), crate::Error> {
        if app::is_enabled("files_versions").await? {
            if let Some(uid) = params.get("uid") {
                crate::storage::Storage::delete_user(uid).await?;
            }
        }
        Ok(())
    }
}

mod app {
    use crate::Error;

    /// Check if an app is enabled
    pub async fn is_enabled(app_name: &str) -> Result<bool, Error> {
        // Implementation would depend on how the OCP\App::isEnabled is implemented
        // This is a placeholder
        Ok(true)
    }
}