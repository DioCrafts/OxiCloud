//! # ownCloud - trash bin
//!
//! @author Bjoern Schiessle
//! @copyright 2013 Bjoern Schiessle schiessle@owncloud.com
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
use async_trait::async_trait;
use anyhow::Result;

/// This module contains all hooks.
pub struct Hooks;

#[async_trait]
pub trait AppManager {
    async fn is_enabled(&self, app: &str) -> bool;
}

#[async_trait]
pub trait UserManager {
    async fn get_user(&self) -> Option<String>;
}

#[async_trait]
pub trait TrashbinManager {
    async fn move_to_trash(&self, path: &str) -> Result<()>;
    async fn delete_user(&self, uid: &str) -> Result<()>;
    async fn resize_trash(&self, user: &str) -> Result<()>;
}

impl Hooks {
    /// Copy files to trash bin
    ///
    /// This function is connected to the delete signal of OC_Filesystem
    /// to copy the file to the trash bin
    pub async fn remove_hook<A, T>(
        params: &HashMap<String, String>, 
        app_manager: &A,
        trashbin: &T
    ) -> Result<()> 
    where 
        A: AppManager + Sync,
        T: TrashbinManager + Sync
    {
        if app_manager.is_enabled("files_trashbin").await {
            if let Some(path) = params.get("path") {
                trashbin.move_to_trash(path).await?;
            }
        }

        Ok(())
    }

    /// Clean up user specific settings if user gets deleted
    ///
    /// This function is connected to the pre_deleteUser signal of OC_Users
    /// to remove the used space for the trash bin stored in the database
    pub async fn delete_user_hook<A, T>(
        params: &HashMap<String, String>,
        app_manager: &A,
        trashbin: &T
    ) -> Result<()> 
    where 
        A: AppManager + Sync,
        T: TrashbinManager + Sync
    {
        if app_manager.is_enabled("files_trashbin").await {
            if let Some(uid) = params.get("uid") {
                trashbin.delete_user(uid).await?;
            }
        }

        Ok(())
    }
    
    pub async fn post_write_hook<U, T>(
        _params: &HashMap<String, String>,
        user_manager: &U,
        trashbin: &T
    ) -> Result<()> 
    where 
        U: UserManager + Sync,
        T: TrashbinManager + Sync
    {
        if let Some(user) = user_manager.get_user().await {
            trashbin.resize_trash(&user).await?;
        }

        Ok(())
    }
}