// Copyright (C) Georg Ehrke
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
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use sqlx::{Pool, Executor, Row};
use crate::hook::{Hook, HookManager};
use crate::db::Db;
use crate::user::User;
use crate::group::Group;

/// Initialize hooks for SubAdmin
pub fn register_hooks(hook_manager: &mut HookManager) {
    hook_manager.register_hook("OC_User", "post_deleteUser", Box::new(SubAdmin::post_delete_user));
    hook_manager.register_hook("OC_User", "post_deleteGroup", Box::new(SubAdmin::post_delete_group));
}

/// This struct provides all methods needed for managing subadmins.
///
/// Hooks provided:
///   post_createSubAdmin($gid)
///   post_deleteSubAdmin($gid)
pub struct SubAdmin;

#[async_trait]
impl SubAdmin {
    /// Create a new SubAdmin
    ///
    /// # Arguments
    /// * `uid` - uid of the SubAdmin
    /// * `gid` - gid of the group
    ///
    /// # Returns
    /// Result indicating success
    pub async fn create_sub_admin(db: &Pool<Db>, hook_manager: &HookManager, uid: &str, gid: &str) -> Result<()> {
        sqlx::query("INSERT INTO `*PREFIX*group_admin` (`gid`,`uid`) VALUES(?,?)")
            .bind(gid)
            .bind(uid)
            .execute(db)
            .await?;
        
        let mut params = HashMap::new();
        params.insert("gid".to_string(), gid.to_string());
        hook_manager.emit("OC_SubAdmin", "post_createSubAdmin", params).await;
        
        Ok(())
    }

    /// Delete a SubAdmin
    ///
    /// # Arguments
    /// * `uid` - uid of the SubAdmin
    /// * `gid` - gid of the group
    ///
    /// # Returns
    /// Result indicating success
    pub async fn delete_sub_admin(db: &Pool<Db>, hook_manager: &HookManager, uid: &str, gid: &str) -> Result<()> {
        sqlx::query("DELETE FROM `*PREFIX*group_admin` WHERE `gid` = ? AND `uid` = ?")
            .bind(gid)
            .bind(uid)
            .execute(db)
            .await?;
        
        let mut params = HashMap::new();
        params.insert("gid".to_string(), gid.to_string());
        hook_manager.emit("OC_SubAdmin", "post_deleteSubAdmin", params).await;
        
        Ok(())
    }

    /// Get groups of a SubAdmin
    ///
    /// # Arguments
    /// * `uid` - uid of the SubAdmin
    ///
    /// # Returns
    /// Vector of group ids
    pub async fn get_sub_admins_groups(db: &Pool<Db>, uid: &str) -> Result<Vec<String>> {
        let rows = sqlx::query("SELECT `gid` FROM `*PREFIX*group_admin` WHERE `uid` = ?")
            .bind(uid)
            .fetch_all(db)
            .await?;
        
        let gids = rows.iter()
            .map(|row| row.get("gid"))
            .collect();
        
        Ok(gids)
    }

    /// Get SubAdmins of a group
    ///
    /// # Arguments
    /// * `gid` - gid of the group
    ///
    /// # Returns
    /// Vector of user ids
    pub async fn get_groups_sub_admins(db: &Pool<Db>, gid: &str) -> Result<Vec<String>> {
        let rows = sqlx::query("SELECT `uid` FROM `*PREFIX*group_admin` WHERE `gid` = ?")
            .bind(gid)
            .fetch_all(db)
            .await?;
        
        let uids = rows.iter()
            .map(|row| row.get("uid"))
            .collect();
        
        Ok(uids)
    }

    /// Get all SubAdmins
    ///
    /// # Returns
    /// Vector of subadmin entries with uid and gid
    pub async fn get_all_sub_admins(db: &Pool<Db>) -> Result<Vec<HashMap<String, String>>> {
        let rows = sqlx::query("SELECT * FROM `*PREFIX*group_admin`")
            .fetch_all(db)
            .await?;
        
        let mut subadmins = Vec::new();
        for row in rows {
            let mut entry = HashMap::new();
            entry.insert("uid".to_string(), row.get("uid"));
            entry.insert("gid".to_string(), row.get("gid"));
            subadmins.push(entry);
        }
        
        Ok(subadmins)
    }

    /// Check if a user is a SubAdmin of a group
    ///
    /// # Arguments
    /// * `uid` - uid of the subadmin
    /// * `gid` - gid of the group
    ///
    /// # Returns
    /// true if user is subadmin of the group, false otherwise
    pub async fn is_sub_admin_of_group(db: &Pool<Db>, uid: &str, gid: &str) -> Result<bool> {
        let row = sqlx::query("SELECT COUNT(*) AS `count` FROM `*PREFIX*group_admin` WHERE `uid` = ? AND `gid` = ?")
            .bind(uid)
            .bind(gid)
            .fetch_one(db)
            .await?;
        
        let count: i64 = row.get("count");
        Ok(count >= 1)
    }

    /// Check if a user is a SubAdmin
    ///
    /// # Arguments
    /// * `uid` - uid of the subadmin
    ///
    /// # Returns
    /// true if user is a subadmin, false otherwise
    pub async fn is_sub_admin(db: &Pool<Db>, uid: &str) -> Result<bool> {
        // Check if the user is already an admin
        if Group::in_group(db, uid, "admin").await? {
            return Ok(true);
        }

        let row = sqlx::query("SELECT COUNT(*) AS `count` FROM `*PREFIX*group_admin` WHERE `uid` = ?")
            .bind(uid)
            .fetch_one(db)
            .await?;
        
        let count: i64 = row.get("count");
        Ok(count > 0)
    }

    /// Check if a user is accessible by a subadmin
    ///
    /// # Arguments
    /// * `subadmin` - uid of the subadmin
    /// * `user` - uid of the user to check
    ///
    /// # Returns
    /// true if user is accessible, false otherwise
    pub async fn is_user_accessible(db: &Pool<Db>, subadmin: &str, user: &str) -> Result<bool> {
        if !Self::is_sub_admin(db, subadmin).await? {
            return Ok(false);
        }
        
        if User::is_admin_user(db, user).await? {
            return Ok(false);
        }
        
        let accessible_groups = Self::get_sub_admins_groups(db, subadmin).await?;
        for group in accessible_groups {
            if Group::in_group(db, user, &group).await? {
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Alias for is_sub_admin_of_group
    pub async fn is_group_accessible(db: &Pool<Db>, subadmin: &str, group: &str) -> Result<bool> {
        Self::is_sub_admin_of_group(db, subadmin, group).await
    }

    /// Hook handler for user deletion
    async fn post_delete_user(db: &Pool<Db>, params: HashMap<String, String>) -> Result<()> {
        let uid = params.get("uid").ok_or_else(|| anyhow!("Missing uid parameter"))?;
        
        sqlx::query("DELETE FROM `*PREFIX*group_admin` WHERE `uid` = ?")
            .bind(uid)
            .execute(db)
            .await?;
        
        Ok(())
    }

    /// Hook handler for group deletion
    async fn post_delete_group(db: &Pool<Db>, params: HashMap<String, String>) -> Result<()> {
        let gid = params.get("gid").ok_or_else(|| anyhow!("Missing gid parameter"))?;
        
        sqlx::query("DELETE FROM `*PREFIX*group_admin` WHERE `gid` = ?")
            .bind(gid)
            .execute(db)
            .await?;
        
        Ok(())
    }
}