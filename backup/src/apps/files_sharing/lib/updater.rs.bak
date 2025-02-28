// Copyright (c) 2013 Michael Gapczynski mtgap@owncloud.com
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashSet;
use std::path::Path;
use log::{warn};
use async_trait::async_trait;

use crate::files::{filesystem, FileInfo};
use crate::sharing::Share;
use crate::user::{User, Config};
use crate::util::Util;
use crate::db::{DB, QueryBuilder};

pub struct SharedUpdater;

impl SharedUpdater {
    /// Correct the parent folders' ETags for all users shared the file at `target`
    pub async fn correct_folders(target: &str) -> Result<(), Box<dyn std::error::Error>> {
        let uid = User::get_current()?;
        let uid_owner = filesystem::get_owner(target).await?;
        let info = filesystem::get_file_info(target).await?;
        
        let mut checked_user = HashSet::new();
        checked_user.insert(uid_owner.clone());
        
        // Correct Shared folders of other users shared with
        let mut users = Share::get_users_item_shared("file", info.file_id, &uid_owner, true).await?;
        
        if !users.is_empty() {
            while !users.is_empty() {
                let mut reshare_users = Vec::new();
                
                for user in &users {
                    if !checked_user.contains(user) {
                        let etag = filesystem::get_etag("").await?;
                        Config::set_user_value(user, "files_sharing", "etag", &etag).await?;
                        
                        // Look for reshares
                        let new_reshares = Share::get_users_item_shared("file", info.file_id, user, true).await?;
                        reshare_users.extend(new_reshares);
                        
                        checked_user.insert(user.clone());
                    }
                }
                
                users = reshare_users;
            }
        }
        
        Ok(())
    }

    /// Remove all shares for a given file if the file was deleted
    async fn remove_share(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_info = filesystem::get_file_info(path).await?;
        let file_source = file_info.file_id;

        let query = QueryBuilder::new()
            .delete_from("share")
            .where_eq("file_source", file_source)
            .build();
            
        match DB::execute_audited(&query, &[&file_source]).await {
            Ok(_) => Ok(()),
            Err(e) => {
                warn!("can't remove share: {}", e);
                Ok(()) // We log the error but don't fail the operation
            }
        }
    }

    pub async fn write_hook(params: &[(&str, &str)]) -> Result<(), Box<dyn std::error::Error>> {
        let path = params.iter()
            .find(|(key, _)| *key == "path")
            .map(|(_, value)| *value)
            .ok_or("Missing path parameter")?;
            
        Self::correct_folders(path).await
    }

    pub async fn rename_hook(params: &[(&str, &str)]) -> Result<(), Box<dyn std::error::Error>> {
        let new_path = params.iter()
            .find(|(key, _)| *key == "newpath")
            .map(|(_, value)| *value)
            .ok_or("Missing newpath parameter")?;
            
        let old_path = params.iter()
            .find(|(key, _)| *key == "oldpath")
            .map(|(_, value)| *value)
            .ok_or("Missing oldpath parameter")?;
            
        Self::correct_folders(new_path).await?;
        
        let dirname = Path::new(old_path)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or(".");
            
        Self::correct_folders(dirname).await
    }

    pub async fn delete_hook(params: &[(&str, &str)]) -> Result<(), Box<dyn std::error::Error>> {
        let path = params.iter()
            .find(|(key, _)| *key == "path")
            .map(|(_, value)| *value)
            .ok_or("Missing path parameter")?;
            
        Self::correct_folders(path).await?;
        Self::remove_share(path).await
    }

    pub async fn share_hook(params: &[(&str, &str)]) -> Result<(), Box<dyn std::error::Error>> {
        let item_type = params.iter()
            .find(|(key, _)| *key == "itemType")
            .map(|(_, value)| *value)
            .ok_or("Missing itemType parameter")?;
            
        if item_type == "file" || item_type == "folder" {
            let file_source = params.iter()
                .find(|(key, _)| *key == "fileSource")
                .map(|(_, value)| *value)
                .ok_or("Missing fileSource parameter")?
                .parse::<i64>()?;
                
            let uid_owner = User::get_current()?;
            let mut users = Share::get_users_item_shared(item_type, file_source, &uid_owner, true).await?;
            
            if !users.is_empty() {
                while !users.is_empty() {
                    let mut reshare_users = Vec::new();
                    
                    for user in &users {
                        if user != &uid_owner {
                            let etag = filesystem::get_etag("").await?;
                            Config::set_user_value(user, "files_sharing", "etag", &etag).await?;
                            
                            // Look for reshares
                            let new_reshares = Share::get_users_item_shared("file", file_source, user, true).await?;
                            reshare_users.extend(new_reshares);
                        }
                    }
                    
                    users = reshare_users;
                }
            }
        }
        
        Ok(())
    }
}