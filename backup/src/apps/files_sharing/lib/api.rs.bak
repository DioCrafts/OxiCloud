// Implementation of the Sharing API
//
// This module provides functionality for sharing files and folders,
// implementing the API endpoints for listing, creating, updating,
// and deleting shares.

use crate::db::{self, QueryBuilder};
use crate::files::view::View;
use crate::ocs::{OcsResult, OcsError};
use crate::share::{self, ShareType, ShareItem, SharePermission};
use crate::user;
use crate::util;
use crate::app_config;
use crate::app;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::convert::TryFrom;

/// API implementation for file sharing functionality
pub struct Api;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareData {
    id: Option<String>,
    url: Option<String>,
    token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ShareParams {
    pub item_source: Option<i64>,
    pub item_type: Option<String>,
    pub path: Option<String>,
    pub reshares: Option<bool>,
    pub specific_share: Option<bool>,
}

impl Api {
    /// Get all shares, optionally filtered by file path
    ///
    /// Returns share information for the specified file/folder or all shares
    pub async fn get_all_shares(params: &mut HashMap<String, String>) -> OcsResult<serde_json::Value> {
        // If a file is specified, get the share for this file
        if let Some(path) = params.get("path") {
            let mut share_params = ShareParams {
                item_source: Self::get_file_id(path).await,
                path: Some(path.clone()),
                item_type: Self::get_item_type(path).await,
                reshares: None,
                specific_share: None,
            };
            
            if let Some(reshares) = params.get("reshares") {
                share_params.reshares = Some(reshares != "false");
            } else {
                share_params.reshares = Some(false);
            }
            
            if let Some(subfiles) = params.get("subfiles") {
                if subfiles != "false" {
                    return Self::get_shares_from_folder(&share_params).await;
                }
            }
            
            return Self::collect_shares(&share_params).await;
        }
        
        // Get all shares if no path is specified
        match share::get_item_shared("file", None).await {
            Ok(share) => Ok(serde_json::to_value(share).unwrap()),
            Err(_) => Err(OcsError::new(404, "could not get shares"))
        }
    }
    
    /// Get information for a specific share by ID
    pub async fn get_share(params: &HashMap<String, String>) -> OcsResult<serde_json::Value> {
        let id = match params.get("id") {
            Some(id) => id,
            None => return Err(OcsError::new(400, "share id not provided")),
        };
        
        let s = match Self::get_share_from_id(id).await {
            Some(share) => share,
            None => return Err(OcsError::new(404, "share doesn't exist")),
        };
        
        let share_params = ShareParams {
            item_source: Some(s["item_source"].as_i64().unwrap_or(0)),
            item_type: Some(s["item_type"].as_str().unwrap_or("").to_string()),
            path: None,
            reshares: None,
            specific_share: Some(true),
        };
        
        Self::collect_shares(&share_params).await
    }
    
    /// Collect share information for a specific item or share ID
    async fn collect_shares(params: &ShareParams) -> OcsResult<serde_json::Value> {
        let item_source = match params.item_source {
            Some(source) => source,
            None => return Err(OcsError::new(404, "invalid item source")),
        };
        
        let item_type = match &params.item_type {
            Some(t) => t.clone(),
            None => return Err(OcsError::new(404, "invalid item type")),
        };
        
        let get_specific_share = params.specific_share.unwrap_or(false);
        
        let shares = share::get_item_shared(&item_type, Some(item_source)).await.unwrap_or_default();
        let received_from = share::get_item_shared_with_by_source(&item_type, item_source).await.ok();
        
        let mut result = shares.clone();
        
        // If a specific share was specified, only return this one
        if get_specific_share {
            if let Some(id) = params.item_source {
                let filtered_shares = shares
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .filter(|share| share["id"].as_i64() == Some(id))
                    .cloned()
                    .collect::<Vec<_>>();
                
                if !filtered_shares.is_empty() {
                    result = serde_json::json!({ "element": filtered_shares[0] });
                }
            }
        }
        
        // Include reshares in the lists if requested
        if params.reshares.unwrap_or(false) {
            result = Self::add_reshares(result, item_source).await;
        }
        
        // Add information about who shared this with the current user
        if let Some(from) = received_from {
            let uid_owner = from["uid_owner"].as_str().unwrap_or("");
            
            let mut result_map = serde_json::Map::new();
            if let Some(obj) = result.as_object() {
                for (k, v) in obj {
                    result_map.insert(k.clone(), v.clone());
                }
            }
            
            result_map.insert("received_from".to_string(), serde_json::json!(uid_owner));
            result_map.insert("received_from_displayname".to_string(), 
                            serde_json::json!(user::get_display_name(uid_owner).await.unwrap_or_default()));
            
            result = serde_json::Value::Object(result_map);
        }
        
        if result.is_null() || (result.as_array().map_or(true, |a| a.is_empty()) && 
                               result.as_object().map_or(true, |o| o.is_empty())) {
            Err(OcsError::new(404, "share doesn't exist"))
        } else {
            Ok(result)
        }
    }
    
    /// Add reshares to a array of shares
    async fn add_reshares(shares: serde_json::Value, item_source: i64) -> serde_json::Value {
        // If there are no shares then there are also no reshares
        let first_share = shares.as_array().and_then(|arr| arr.first().cloned());
        
        let path = match first_share {
            Some(share) => share["path"].as_str().unwrap_or("").to_string(),
            None => return shares,
        };
        
        let current_user = user::get_user().await;
        
        let select = "`*PREFIX*share`.`id`, `item_type`, `*PREFIX*share`.`parent`, `share_type`, \
                    `share_with`, `file_source`, `path`, `permissions`, `stime`, `expiration`, \
                    `token`, `storage`, `mail_send`, `mail_send`";
        
        let query = format!(
            "SELECT {} FROM `*PREFIX*share` \
            INNER JOIN `*PREFIX*filecache` ON `file_source` = `*PREFIX*filecache`.`fileid` \
            WHERE `*PREFIX*share`.`file_source` = ? AND `*PREFIX*share`.`item_type` IN ('file', 'folder') \
            AND `uid_owner` != ?", 
            select
        );
        
        let mut qb = QueryBuilder::new(query);
        qb.add_param(item_source);
        qb.add_param(current_user);
        
        let reshares: Vec<serde_json::Value> = match db::query(&qb).await {
            Ok(rows) => {
                let mut result = Vec::with_capacity(rows.len());
                
                for row in rows {
                    let mut reshare = row.clone();
                    
                    // Add display name for the share_with field
                    if let Some(share_with) = reshare.get("share_with")
                        .and_then(|sw| sw.as_str())
                        .filter(|sw| !sw.is_empty()) {
                        
                        let display_name = user::get_display_name(share_with).await.unwrap_or_default();
                        reshare.as_object_mut().unwrap().insert(
                            "share_with_displayname".to_string(),
                            serde_json::json!(display_name)
                        );
                    }
                    
                    // Add correct path to the result
                    reshare.as_object_mut().unwrap().insert(
                        "path".to_string(),
                        serde_json::json!(path)
                    );
                    
                    result.push(reshare);
                }
                
                result
            },
            Err(_) => Vec::new(),
        };
        
        // Merge the original shares with the reshares
        if let Some(mut original_shares) = shares.as_array().cloned() {
            original_shares.extend(reshares);
            serde_json::json!(original_shares)
        } else {
            shares
        }
    }
    
    /// Get share information for all files in a given folder (non-recursive)
    async fn get_shares_from_folder(params: &ShareParams) -> OcsResult<serde_json::Value> {
        let path = match &params.path {
            Some(p) => p,
            None => return Err(OcsError::new(400, "path not provided")),
        };
        
        let current_user = user::get_user().await;
        let view = View::new(format!("/{}/files", current_user));
        
        if !view.is_dir(path).await {
            return Err(OcsError::new(404, "not a directory"));
        }
        
        let content = match view.get_directory_content(path).await {
            Ok(content) => content,
            Err(_) => return Err(OcsError::new(500, "failed to get directory content")),
        };
        
        let mut result = Vec::new();
        
        for file in content {
            // Workaround because folders are named 'dir' in this context
            let item_type = if file["type"].as_str() == Some("file") { "file" } else { "folder" };
            
            let file_id = match file["fileid"].as_i64() {
                Some(id) => id,
                None => continue,
            };
            
            let share = share::get_item_shared(item_type, Some(file_id)).await.ok();
            let received_from = share::get_item_shared_with_by_source(item_type, file_id).await.ok();
            
            if let Some(mut share_data) = share {
                // Add received_from information if available
                if let Some(from) = received_from {
                    let uid_owner = from["uid_owner"].as_str().unwrap_or("");
                    
                    let share_obj = share_data.as_object_mut().unwrap();
                    share_obj.insert("received_from".to_string(), serde_json::json!(uid_owner));
                    share_obj.insert("received_from_displayname".to_string(), 
                                   serde_json::json!(user::get_display_name(uid_owner).await.unwrap_or_default()));
                }
                
                // Add filename
                if let Some(name) = file.get("name").and_then(|n| n.as_str()) {
                    share_data.as_object_mut().unwrap().insert("filename".to_string(), serde_json::json!(name));
                }
                
                result.push(share_data);
            }
        }
        
        Ok(serde_json::json!(result))
    }
    
    /// Create a new share
    pub async fn create_share(params: &mut HashMap<String, String>) -> OcsResult<serde_json::Value> {
        let path = match params.get("path") {
            Some(p) => p.clone(),
            None => return Err(OcsError::new(400, "please specify a file or folder path")),
        };
        
        let item_source = match Self::get_file_id(&path).await {
            Some(id) => id,
            None => return Err(OcsError::new(404, "wrong path, file/folder doesn't exist.")),
        };
        
        let item_type = match Self::get_item_type(&path).await {
            Some(t) => t,
            None => return Err(OcsError::new(404, "wrong path, file/folder doesn't exist.")),
        };
        
        let share_with = params.get("shareWith").cloned();
        let share_type = params.get("shareType")
            .and_then(|s| s.parse::<i32>().ok())
            .map(|n| ShareType::try_from(n).unwrap_or(ShareType::User));
        
        if share_type.is_none() {
            return Err(OcsError::new(400, "share type not specified"));
        }
        
        let share_type = share_type.unwrap();
        
        let permissions = match share_type {
            ShareType::User | ShareType::Group => {
                params.get("permissions")
                    .and_then(|p| p.parse::<i32>().ok())
                    .unwrap_or(31) // Default: all permissions
            },
            ShareType::Link => {
                // Allow password protection
                let share_with = params.get("password").cloned();
                
                // Check public link share
                let public_upload_enabled = app_config::get_value("core", "shareapi_allow_public_upload", "yes").await;
                let encryption_enabled = app::is_enabled("files_encryption").await;
                
                if params.get("publicUpload").is_some() && (encryption_enabled || public_upload_enabled != "yes") {
                    return Err(OcsError::new(404, "public upload disabled by the administrator"));
                }
                
                let public_upload = params.get("publicUpload").map_or("false", |v| v.as_str()) == "true";
                
                // Read, create, update (7) if public upload is enabled or
                // read (1) if public upload is disabled
                if public_upload { 7 } else { 1 }
            },
            _ => return Err(OcsError::new(404, "unknown share type")),
        };
        
        // Try to create the share
        let token = match share::share_item(
            &item_type,
            item_source,
            share_type,
            share_with.as_deref(),
            permissions,
        ).await {
            Ok(token) => token,
            Err(e) => return Err(OcsError::new(404, &e.to_string())),
        };
        
        if token.is_some() {
            let mut data = ShareData {
                id: Some("unknown".to_string()),
                url: None,
                token: None,
            };
            
            let shares = share::get_item_shared(&item_type, Some(item_source)).await.unwrap_or_default();
            let shares_array = shares.as_array().unwrap_or(&Vec::new());
            
            if let Some(token_str) = token.as_ref().and_then(|t| t.as_str().map(|s| s.to_string())) {
                // Public link share
                for share in shares_array {
                    if share["token"].as_str() == Some(&token_str) {
                        if let Some(id) = share["id"].as_str() {
                            data.id = Some(id.to_string());
                            break;
                        }
                    }
                }
                
                let url = util::link_to_public(&format!("files&t={}", token_str)).await;
                data.url = Some(url);
                data.token = Some(token_str);
            } else {
                // Regular share
                for share in shares_array {
                    if share["share_type"].as_i64() == Some(share_type as i64) &&
                       share["share_with"].as_str() == share_with.as_deref() {
                        if let Some(id) = share["id"].as_str() {
                            data.id = Some(id.to_string());
                            break;
                        }
                    }
                }
            }
            
            Ok(serde_json::to_value(data).unwrap())
        } else {
            Err(OcsError::new(404, "couldn't share file"))
        }
    }
    
    /// Update shares, e.g. password, permissions, etc.
    pub async fn update_share(params: &HashMap<String, String>, put_params: &HashMap<String, String>) -> OcsResult<serde_json::Value> {
        let id = match params.get("id") {
            Some(id) => id,
            None => return Err(OcsError::new(400, "share id not provided")),
        };
        
        let share = match Self::get_share_from_id(id).await {
            Some(share) => share,
            None => return Err(OcsError::new(404, "wrong share Id, share doesn't exist.")),
        };
        
        let item_source = share["item_source"].as_i64();
        
        if item_source.is_none() {
            return Err(OcsError::new(404, "wrong share Id, share doesn't exist."));
        }
        
        let result = if put_params.contains_key("permissions") {
            Self::update_permissions(&share, put_params).await
        } else if put_params.contains_key("password") {
            Self::update_password(&share, put_params).await
        } else if put_params.contains_key("publicUpload") {
            Self::update_public_upload(&share, put_params).await
        } else {
            return Err(OcsError::new(400, "Wrong or no update parameter given"));
        };
        
        result
    }
    
    /// Update permissions for a share
    async fn update_permissions(share: &serde_json::Value, params: &HashMap<String, String>) -> OcsResult<serde_json::Value> {
        let item_source = match share["item_source"].as_i64() {
            Some(source) => source,
            None => return Err(OcsError::new(400, "invalid item source")),
        };
        
        let item_type = match share["item_type"].as_str() {
            Some(t) => t.to_string(),
            None => return Err(OcsError::new(400, "invalid item type")),
        };
        
        let share_with = share["share_with"].as_str().map(|s| s.to_string());
        
        let share_type = match share["share_type"].as_i64() {
            Some(t) => ShareType::try_from(t as i32).unwrap_or(ShareType::User),
            None => return Err(OcsError::new(400, "invalid share type")),
        };
        
        let permissions = match params.get("permissions").and_then(|p| p.parse::<i32>().ok()) {
            Some(p) => p,
            None => return Err(OcsError::new(400, "invalid permissions parameter")),
        };
        
        let public_upload_status = app_config::get_value("core", "shareapi_allow_public_upload", "yes").await;
        let encryption_enabled = app::is_enabled("files_encryption").await;
        
        let public_upload_enabled = !encryption_enabled && public_upload_status == "yes";
        
        // Only change permissions for public shares if public upload is enabled
        // and we want to set permissions to 1 (read only) or 7 (allow upload)
        if share_type == ShareType::Link && 
           (!public_upload_enabled || (permissions != 7 && permissions != 1)) {
            return Err(OcsError::new(400, "can't change permission for public link share"));
        }
        
        match share::set_permissions(
            &item_type,
            item_source,
            share_type,
            share_with.as_deref(),
            permissions
        ).await {
            Ok(true) => Ok(serde_json::json!({})),
            Ok(false) => Err(OcsError::new(404, "couldn't set permissions")),
            Err(e) => Err(OcsError::new(404, &e.to_string())),
        }
    }
    
    /// Enable/disable public upload
    async fn update_public_upload(share: &serde_json::Value, params: &HashMap<String, String>) -> OcsResult<serde_json::Value> {
        let public_upload_enabled = app_config::get_value("core", "shareapi_allow_public_upload", "yes").await;
        let encryption_enabled = app::is_enabled("files_encryption").await;
        
        if encryption_enabled || public_upload_enabled != "yes" {
            return Err(OcsError::new(404, "public upload disabled by the administrator"));
        }
        
        let item_type = share["item_type"].as_str().unwrap_or("");
        let share_type = share["share_type"].as_i64().unwrap_or(0);
        
        if item_type != "folder" || share_type != ShareType::Link as i64 {
            return Err(OcsError::new(404, "public upload is only possible for public shared folders"));
        }
        
        // Create a new params map with the calculated permissions
        let mut updated_params = params.clone();
        let public_upload = params.get("publicUpload").map_or("false", |v| v.as_str()) == "true";
        
        // Read, create, update (7) if public upload is enabled or
        // read (1) if public upload is disabled
        updated_params.insert("permissions".to_string(), if public_upload { "7".to_string() } else { "1".to_string() });
        
        Self::update_permissions(share, &updated_params).await
    }
    
    /// Update password for public link share
    async fn update_password(share: &serde_json::Value, params: &HashMap<String, String>) -> OcsResult<serde_json::Value> {
        let item_source = match share["item_source"].as_i64() {
            Some(source) => source,
            None => return Err(OcsError::new(400, "invalid item source")),
        };
        
        let item_type = match share["item_type"].as_str() {
            Some(t) => t.to_string(),
            None => return Err(OcsError::new(400, "invalid item type")),
        };
        
        let share_type = share["share_type"].as_i64().unwrap_or(0);
        
        if share_type != ShareType::Link as i64 {
            return Err(OcsError::new(400, "password protection is only supported for public shares"));
        }
        
        let share_with = params.get("password").map(|p| {
            if p.is_empty() { None } else { Some(p.clone()) }
        }).flatten();
        
        let items = share::get_item_shared(&item_type, Some(item_source)).await.unwrap_or_default();
        let items_array = items.as_array().unwrap_or(&Vec::new());
        
        let mut permissions = 0;
        let mut found = false;
        
        for item in items_array {
            if item["share_type"].as_i64() == Some(ShareType::Link as i64) {
                found = true;
                permissions = item["permissions"].as_i64().unwrap_or(0) as i32;
                break;
            }
        }
        
        if !found {
            return Err(OcsError::new(404, "share doesn't exists, can't change password"));
        }
        
        match share::share_item(
            &item_type,
            item_source,
            ShareType::Link,
            share_with.as_deref(),
            permissions
        ).await {
            Ok(_) => Ok(serde_json::json!({})),
            Err(_) => Err(OcsError::new(404, "couldn't set password")),
        }
    }
    
    /// Unshare a file/folder
    pub async fn delete_share(params: &HashMap<String, String>) -> OcsResult<serde_json::Value> {
        let id = match params.get("id") {
            Some(id) => id,
            None => return Err(OcsError::new(400, "share id not provided")),
        };
        
        let share = match Self::get_share_from_id(id).await {
            Some(share) => share,
            None => return Err(OcsError::new(404, "wrong share ID, share doesn't exist.")),
        };
        
        let item_source = match share["item_source"].as_i64() {
            Some(source) => source,
            None => return Err(OcsError::new(404, "invalid item source")),
        };
        
        let item_type = match share["item_type"].as_str() {
            Some(t) => t.to_string(),
            None => return Err(OcsError::new(404, "invalid item type")),
        };
        
        let share_with = if let Some(share_type) = share["share_type"].as_i64() {
            if share_type == ShareType::Link as i64 {
                None
            } else {
                share["share_with"].as_str().map(|s| s.to_string())
            }
        } else {
            None
        };
        
        let share_type = match share["share_type"].as_i64() {
            Some(t) => ShareType::try_from(t as i32).unwrap_or(ShareType::User),
            None => return Err(OcsError::new(400, "invalid share type")),
        };
        
        match share::unshare(
            &item_type,
            item_source,
            share_type,
            share_with.as_deref()
        ).await {
            Ok(true) => Ok(serde_json::json!({})),
            Ok(false) => Err(OcsError::new(404, "Unshare Failed")),
            Err(e) => Err(OcsError::new(404, &e.to_string())),
        }
    }
    
    /// Get file ID from a given path
    async fn get_file_id(path: &str) -> Option<i64> {
        let current_user = user::get_user().await;
        let view = View::new(format!("/{}/files", current_user));
        
        view.get_file_info(path).await.ok()?.get("fileid").and_then(|id| id.as_i64())
    }
    
    /// Get item type (file or folder) from a given path
    async fn get_item_type(path: &str) -> Option<String> {
        let current_user = user::get_user().await;
        let view = View::new(format!("/{}/files", current_user));
        
        if view.is_dir(path).await {
            Some("folder".to_string())
        } else if view.is_file(path).await {
            Some("file".to_string())
        } else {
            None
        }
    }
    
    /// Get share information from a share ID
    async fn get_share_from_id(share_id: &str) -> Option<serde_json::Value> {
        let sql = "SELECT `item_source`, `share_type`, `share_with`, `item_type`, `permissions` \
                  FROM `*PREFIX*share` WHERE `id` = ?";
        
        let mut qb = QueryBuilder::new(sql);
        qb.add_param(share_id);
        
        match db::query(&qb).await {
            Ok(rows) if !rows.is_empty() => Some(rows[0].clone()),
            _ => None,
        }
    }
}