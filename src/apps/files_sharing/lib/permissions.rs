// Copyright (C) 2012 Michael Gapczynski (mtgap@owncloud.com)
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
use sqlx::{Pool, Row, Sqlite};

use crate::ocp::Share;
use crate::oc_share_backend_file::OcShareBackendFile;

// Constants from OCP namespace
pub const PERMISSION_READ: i32 = 1;

#[async_trait]
pub trait Permissions {
    async fn get(&self, file_id: i32, user: &str) -> Result<i32, anyhow::Error>;
    async fn set(&self, file_id: i32, user: &str, permissions: i32) -> Result<(), anyhow::Error>;
    async fn get_multiple(&self, file_ids: &[i32], user: &str) -> Result<HashMap<i32, i32>, anyhow::Error>;
    async fn get_directory_permissions(&self, parent_id: i32, user: &str) -> Result<HashMap<i32, i32>, anyhow::Error>;
    async fn remove(&self, file_id: i32, user: Option<&str>) -> Result<(), anyhow::Error>;
    async fn remove_multiple(&self, file_ids: &[i32], user: &str) -> Result<(), anyhow::Error>;
}

pub struct SharedPermissions {
    pool: Pool<Sqlite>,
    share: Share,
}

impl SharedPermissions {
    pub fn new(pool: Pool<Sqlite>, share: Share) -> Self {
        Self { pool, share }
    }
}

#[async_trait]
impl Permissions for SharedPermissions {
    /// Get the permissions for a single file
    ///
    /// @param file_id - The ID of the file
    /// @param user - The user to get permissions for
    /// @return The permission value, or -1 if no permissions set
    async fn get(&self, file_id: i32, user: &str) -> Result<i32, anyhow::Error> {
        if file_id == -1 {
            return Ok(PERMISSION_READ);
        }

        let source = self.share.get_item_shared_with_by_source(
            "file",
            file_id,
            OcShareBackendFile::FORMAT_SHARED_STORAGE,
            None,
            true,
        ).await?;

        match source {
            Some(source_data) => Ok(source_data.permissions),
            None => Ok(-1),
        }
    }

    /// Set the permissions of a file
    ///
    /// @param file_id - The ID of the file
    /// @param user - The user to set permissions for
    /// @param permissions - The permission value to set
    async fn set(&self, _file_id: i32, _user: &str, _permissions: i32) -> Result<(), anyhow::Error> {
        // Not a valid action for Shared Permissions
        Ok(())
    }

    /// Get the permissions of multiple files
    ///
    /// @param file_ids - Array of file IDs
    /// @param user - The user to get permissions for
    /// @return HashMap of file IDs to permission values
    async fn get_multiple(&self, file_ids: &[i32], user: &str) -> Result<HashMap<i32, i32>, anyhow::Error> {
        if file_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let mut file_permissions = HashMap::new();
        for &file_id in file_ids {
            let permission = self.get(file_id, user).await?;
            file_permissions.insert(file_id, permission);
        }

        Ok(file_permissions)
    }

    /// Get the permissions for all files in a folder
    ///
    /// @param parent_id - The ID of the parent folder
    /// @param user - The user to get permissions for
    /// @return HashMap of file IDs to permission values
    async fn get_directory_permissions(&self, parent_id: i32, user: &str) -> Result<HashMap<i32, i32>, anyhow::Error> {
        // Root of the Shared folder
        if parent_id == -1 {
            return Ok(self.share.get_items_shared_with(
                "file",
                OcShareBackendFile::FORMAT_PERMISSIONS,
            ).await?);
        }

        let permissions = self.get(parent_id, user).await?;
        let mut file_permissions = HashMap::new();

        let rows = sqlx::query("SELECT `fileid` FROM `*PREFIX*filecache` WHERE `parent` = ?")
            .bind(parent_id)
            .fetch_all(&self.pool)
            .await?;

        for row in rows {
            let file_id: i32 = row.get("fileid");
            file_permissions.insert(file_id, permissions);
        }

        Ok(file_permissions)
    }

    /// Remove the permissions for a file
    ///
    /// @param file_id - The ID of the file
    /// @param user - The user to remove permissions for (optional)
    async fn remove(&self, _file_id: i32, _user: Option<&str>) -> Result<(), anyhow::Error> {
        // Not a valid action for Shared Permissions
        Ok(())
    }

    /// Remove the permissions for multiple files
    ///
    /// @param file_ids - Array of file IDs
    /// @param user - The user to remove permissions for
    async fn remove_multiple(&self, _file_ids: &[i32], _user: &str) -> Result<(), anyhow::Error> {
        // Not a valid action for Shared Permissions
        Ok(())
    }
}