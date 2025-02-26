//! # OCP Share
//! 
//! This module provides the ability for apps to share their content between users.
//! Apps must create a backend that implements the ShareBackend trait and register it with this struct.

use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::sync::RwLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Row, FromRow, Error as SqlxError, query};
use sqlx::mysql::MySqlPool;
use thiserror::Error;
use uuid::Uuid;
use async_trait::async_trait;

/// Error types for the Share module
#[derive(Error, Debug)]
pub enum ShareError {
    #[error("Sharing {0} failed: {1}")]
    SharingFailed(String, String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),
    
    #[error("Item not found: {0}")]
    ItemNotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Invalid source: {0}")]
    InvalidSource(String),
    
    #[error("Backend error: {0}")]
    BackendError(String),
}

type ShareResult<T> = Result<T, ShareError>;

/// Share types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShareType {
    User = 0,
    Group = 1,
    Link = 3,
    Email = 4,
    Contact = 5,
    Remote = 6,
    GroupUserUnique = 2, // Internal use
}

impl TryFrom<i32> for ShareType {
    type Error = ShareError;
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ShareType::User),
            1 => Ok(ShareType::Group),
            2 => Ok(ShareType::GroupUserUnique),
            3 => Ok(ShareType::Link),
            4 => Ok(ShareType::Email),
            5 => Ok(ShareType::Contact),
            6 => Ok(ShareType::Remote),
            _ => Err(ShareError::InvalidSource(format!("Invalid share type: {}", value))),
        }
    }
}

/// Format types for returned items
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    None = -1,
    Statuses = -2,
    Sources = -3,
}

impl TryFrom<i32> for Format {
    type Error = ShareError;
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Format::None),
            -2 => Ok(Format::Statuses),
            -3 => Ok(Format::Sources),
            _ => Err(ShareError::InvalidSource(format!("Invalid format: {}", value))),
        }
    }
}

/// Permissions for shared items (Create, Read, Update, Delete, Share)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permission(pub u32);

impl Permission {
    pub const CREATE: Permission = Permission(1);
    pub const READ: Permission = Permission(2);
    pub const UPDATE: Permission = Permission(4);
    pub const DELETE: Permission = Permission(8);
    pub const SHARE: Permission = Permission(16);
    
    pub fn has_permission(&self, permission: Permission) -> bool {
        (self.0 & permission.0) != 0
    }
    
    pub fn add_permission(&mut self, permission: Permission) {
        self.0 |= permission.0;
    }
    
    pub fn remove_permission(&mut self, permission: Permission) {
        self.0 &= !permission.0;
    }
}

impl From<u32> for Permission {
    fn from(value: u32) -> Self {
        Permission(value)
    }
}

/// Represents a shared item
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ShareItem {
    pub id: i64,
    pub item_type: String,
    pub item_source: String,
    pub item_target: Option<String>,
    pub parent: Option<i64>,
    pub share_type: i32,
    pub share_with: Option<String>,
    pub uid_owner: String,
    pub permissions: u32,
    pub stime: i64,
    pub file_source: Option<i64>,
    pub file_target: Option<String>,
    pub token: Option<String>,
    pub expiration: Option<String>,
    pub mail_send: Option<i32>,
}

impl ShareItem {
    pub fn get_share_type(&self) -> ShareResult<ShareType> {
        ShareType::try_from(self.share_type)
    }
    
    pub fn get_permissions(&self) -> Permission {
        Permission(self.permissions)
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(expiration) = &self.expiration {
            if let Ok(expiration_date) = DateTime::parse_from_str(expiration, "%Y-%m-%d %H:%M") {
                return Utc::now() > expiration_date;
            }
        }
        false
    }
}

/// Backend trait that apps must implement to share content
#[async_trait]
pub trait ShareBackend: Send + Sync {
    /// Get the source of the item to be stored in the database
    async fn is_valid_source(&self, item_source: &str, uid_owner: &str) -> ShareResult<bool>;
    
    /// Get a unique name of the item for the specified user
    async fn generate_target(
        &self,
        item_source: &str, 
        share_with: Option<&str>, 
        exclude: Option<&[String]>
    ) -> ShareResult<String>;
    
    /// Converts the shared item sources back into the item in the specified format
    async fn format_items(
        &self, 
        items: Vec<ShareItem>, 
        format: Format, 
        parameters: Option<HashMap<String, String>>
    ) -> ShareResult<Vec<HashMap<String, serde_json::Value>>>;
}

/// Backend for share backends that share content that is dependent on files
#[async_trait]
pub trait ShareBackendFileDependent: ShareBackend {
    /// Get the file path of the item
    async fn get_file_path(&self, item_source: &str, uid_owner: &str) -> ShareResult<String>;
}

/// Backend for collections of items implemented by another share backend
#[async_trait]
pub trait ShareBackendCollection: ShareBackend {
    /// Get the sources of the children of the item
    async fn get_children(&self, item_source: &str) -> ShareResult<Vec<HashMap<String, String>>>;
}

/// Main Share struct that provides the public API
pub struct Share {
    pool: Pool<MySqlPool>,
    backends: RwLock<HashMap<String, Box<dyn ShareBackend>>>,
    backend_types: RwLock<HashMap<String, BackendInfo>>,
}

/// Information about registered backends
struct BackendInfo {
    class_name: String,
    collection_of: Option<String>,
    supported_file_extensions: Option<Vec<String>>,
}

// Constants
const TOKEN_LENGTH: usize = 32;

lazy_static! {
    static ref SHARE_TYPE_USER_AND_GROUPS: i32 = -1;
    static ref IS_RESHARING_ALLOWED: RwLock<Option<bool>> = RwLock::new(None);
}

impl Share {
    /// Create a new Share instance
    pub fn new(pool: Pool<MySqlPool>) -> Self {
        Share {
            pool,
            backends: RwLock::new(HashMap::new()),
            backend_types: RwLock::new(HashMap::new()),
        }
    }
    
    /// Register a sharing backend for an item type
    pub async fn register_backend(
        &self,
        item_type: &str,
        class_name: &str,
        collection_of: Option<&str>,
        supported_file_extensions: Option<Vec<String>>
    ) -> ShareResult<bool> {
        if !self.is_enabled().await? {
            return Ok(false);
        }
        
        let mut backend_types = self.backend_types.write().unwrap();
        if !backend_types.contains_key(item_type) {
            backend_types.insert(
                item_type.to_string(), 
                BackendInfo {
                    class_name: class_name.to_string(),
                    collection_of: collection_of.map(|s| s.to_string()),
                    supported_file_extensions,
                }
            );
            
            // TODO: Add scripts and styles
            // if backend_types.len() == 1 {
            //     OC_Util::add_script('core', 'share');
            //     OC_Util::add_style('core', 'share');
            // }
            
            Ok(true)
        } else {
            // Log warning about already registered backend
            log::warn!(
                "Sharing backend {} not registered, {} is already registered for {}",
                class_name,
                backend_types[item_type].class_name,
                item_type
            );
            Ok(false)
        }
    }
    
    /// Check if the Share API is enabled
    pub async fn is_enabled(&self) -> ShareResult<bool> {
        // TODO: Implement app config
        // App config should be checked here, for now return true
        Ok(true)
    }
    
    /// Prepare a path to be passed to DB as file_target
    pub fn prep_file_target(&self, path: &str) -> String {
        if !path.starts_with('/') {
            format!("/{}", path)
        } else {
            path.to_string()
        }
    }
    
    /// Find which users can access a shared item
    pub async fn get_users_sharing_file(
        &self,
        path: &str,
        user: &str,
        include_owner: bool
    ) -> ShareResult<(Vec<String>, bool)> {
        let mut shares = Vec::new();
        let mut public_share = false;
        let mut source = -1;
        let mut cache = None;
        
        // TODO: Implement file view and cache logic
        
        // For now, simplified implementation
        let users = Vec::new();
        
        // Include owner in list of users if requested
        let mut result = if include_owner {
            let mut users = users;
            users.push(user.to_string());
            users
        } else {
            users
        };
        
        // Remove duplicates
        result.sort();
        result.dedup();
        
        Ok((result, public_share))
    }
    
    /// Get the items of item type shared with the current user
    pub async fn get_items_shared_with(
        &self,
        item_type: &str,
        format: Format,
        parameters: Option<HashMap<String, String>>,
        limit: i32,
        include_collections: bool
    ) -> ShareResult<Vec<HashMap<String, serde_json::Value>>> {
        // TODO: Get current user
        let user = "current_user".to_string();
        
        self.get_items(
            item_type,
            None,
            Some(*SHARE_TYPE_USER_AND_GROUPS),
            Some(&user),
            None,
            format,
            parameters,
            limit,
            include_collections,
            false
        ).await
    }
    
    /// Get the item of item type shared with the current user
    pub async fn get_item_shared_with(
        &self,
        item_type: &str,
        item_target: &str,
        format: Format,
        parameters: Option<HashMap<String, String>>,
        include_collections: bool
    ) -> ShareResult<Option<HashMap<String, serde_json::Value>>> {
        // TODO: Get current user
        let user = "current_user".to_string();
        
        let result = self.get_items(
            item_type,
            Some(item_target),
            Some(*SHARE_TYPE_USER_AND_GROUPS),
            Some(&user),
            None,
            format,
            parameters,
            1,
            include_collections,
            false
        ).await?;
        
        Ok(result.into_iter().next())
    }
    
    /// Get the item of item type shared with a given user by source
    pub async fn get_item_shared_with_user(
        &self,
        item_type: &str,
        item_source: &str,
        user: &str
    ) -> ShareResult<Vec<HashMap<String, String>>> {
        let mut shares = Vec::new();
        
        // First check if there is a db entry for the specific user
        let query_str = "
            SELECT file_target, permissions, expiration
            FROM *PREFIX*share
            WHERE item_source = ? AND item_type = ? AND share_with = ?
        ";
        
        let rows = sqlx::query(query_str)
            .bind(item_source)
            .bind(item_type)
            .bind(user)
            .fetch_all(&self.pool)
            .await?;
        
        for row in rows {
            let share = HashMap::from([
                ("file_target".to_string(), row.get("file_target")),
                ("permissions".to_string(), row.get("permissions")),
                ("expiration".to_string(), row.get("expiration")),
            ]);
            shares.push(share);
        }
        
        // If didn't find a result then let's look for a group share
        if shares.is_empty() {
            // TODO: Get user groups
            let groups = vec!["group1".to_string(), "group2".to_string()];
            
            let placeholders = groups.iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(", ");
            
            let query_str = format!(
                "
                SELECT file_target, permissions, expiration
                FROM *PREFIX*share
                WHERE item_source = ? AND item_type = ? AND share_with IN ({})
                ",
                placeholders
            );
            
            let mut query = sqlx::query(&query_str);
            query = query.bind(item_source).bind(item_type);
            
            for group in groups {
                query = query.bind(group);
            }
            
            let rows = query.fetch_all(&self.pool).await?;
            
            for row in rows {
                let share = HashMap::from([
                    ("file_target".to_string(), row.get("file_target")),
                    ("permissions".to_string(), row.get("permissions")),
                    ("expiration".to_string(), row.get("expiration")),
                ]);
                shares.push(share);
            }
        }
        
        Ok(shares)
    }
    
    /// Get the item of item type shared with the current user by source
    pub async fn get_item_shared_with_by_source(
        &self,
        item_type: &str,
        item_source: &str,
        format: Format,
        parameters: Option<HashMap<String, String>>,
        include_collections: bool
    ) -> ShareResult<Option<HashMap<String, serde_json::Value>>> {
        // TODO: Get current user
        let user = "current_user".to_string();
        
        let result = self.get_items(
            item_type,
            Some(item_source),
            Some(*SHARE_TYPE_USER_AND_GROUPS),
            Some(&user),
            None,
            format,
            parameters,
            1,
            include_collections,
            true
        ).await?;
        
        Ok(result.into_iter().next())
    }
    
    /// Get the item of item type shared by a link
    pub async fn get_item_shared_with_by_link(
        &self,
        item_type: &str,
        item_source: &str,
        uid_owner: &str
    ) -> ShareResult<Option<ShareItem>> {
        let result = self.get_items(
            item_type,
            Some(item_source),
            Some(ShareType::Link as i32),
            None,
            Some(uid_owner),
            Format::None,
            None,
            1,
            false,
            false
        ).await?;
        
        // Since Format::None should return ShareItem, we need to convert
        if let Some(first_item) = result.into_iter().next() {
            // Convert to ShareItem
            // In practice, this conversion would use serde to parse the HashMap into ShareItem
            // This is placeholder code
            let id = first_item.get("id")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| ShareError::BackendError("Could not parse id".to_string()))?;
                
            // More field conversions...
            
            // Placeholder, in reality we'd deserialize the whole item
            let share_item = ShareItem {
                id,
                item_type: item_type.to_string(),
                item_source: item_source.to_string(),
                item_target: None,
                parent: None,
                share_type: ShareType::Link as i32,
                share_with: None,
                uid_owner: uid_owner.to_string(),
                permissions: 0,
                stime: 0,
                file_source: None,
                file_target: None,
                token: None,
                expiration: None,
                mail_send: None,
            };
            
            Ok(Some(share_item))
        } else {
            Ok(None)
        }
    }
    
    /// Get the item shared by a token
    pub async fn get_share_by_token(&self, token: &str) -> ShareResult<Option<ShareItem>> {
        let query_str = "SELECT * FROM *PREFIX*share WHERE token = ?";
        
        let row = sqlx::query(query_str)
            .bind(token)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = row {
            let share_item = ShareItem {
                id: row.get("id"),
                item_type: row.get("item_type"),
                item_source: row.get("item_source"),
                item_target: row.get("item_target"),
                parent: row.get("parent"),
                share_type: row.get("share_type"),
                share_with: row.get("share_with"),
                uid_owner: row.get("uid_owner"),
                permissions: row.get("permissions"),
                stime: row.get("stime"),
                file_source: row.get("file_source"),
                file_target: row.get("file_target"),
                token: row.get("token"),
                expiration: row.get("expiration"),
                mail_send: row.get("mail_send"),
            };
            
            if share_item.is_expired() {
                self.unshare_item(&share_item).await?;
                Ok(None)
            } else {
                Ok(Some(share_item))
            }
        } else {
            Ok(None)
        }
    }
    
    /// Resolves reshares down to the last real share
    pub async fn resolve_reshare(&self, link_item: &ShareItem) -> ShareResult<ShareItem> {
        let mut current_item = link_item.clone();
        
        while let Some(parent_id) = current_item.parent {
            let query_str = "SELECT * FROM *PREFIX*share WHERE id = ?";
            
            let row = sqlx::query(query_str)
                .bind(parent_id)
                .fetch_optional(&self.pool)
                .await?;
                
            if let Some(row) = row {
                let parent_item = ShareItem {
                    id: row.get("id"),
                    item_type: row.get("item_type"),
                    item_source: row.get("item_source"),
                    item_target: row.get("item_target"),
                    parent: row.get("parent"),
                    share_type: row.get("share_type"),
                    share_with: row.get("share_with"),
                    uid_owner: row.get("uid_owner"),
                    permissions: row.get("permissions"),
                    stime: row.get("stime"),
                    file_source: row.get("file_source"),
                    file_target: row.get("file_target"),
                    token: row.get("token"),
                    expiration: row.get("expiration"),
                    mail_send: row.get("mail_send"),
                };
                
                if parent_item.parent.is_none() {
                    return Ok(parent_item);
                }
                
                current_item = parent_item;
            } else {
                // No parent found, return current item
                return Ok(current_item);
            }
        }
        
        Ok(current_item)
    }
    
    /// Get the shared items of item type owned by the current user
    pub async fn get_items_shared(
        &self,
        item_type: &str,
        format: Format,
        parameters: Option<HashMap<String, String>>,
        limit: i32,
        include_collections: bool
    ) -> ShareResult<Vec<HashMap<String, serde_json::Value>>> {
        // TODO: Get current user
        let user = "current_user".to_string();
        
        self.get_items(
            item_type,
            None,
            None,
            None,
            Some(&user),
            format,
            parameters,
            limit,
            include_collections,
            false
        ).await
    }
    
    /// Get the shared item of item type owned by the current user
    pub async fn get_item_shared(
        &self,
        item_type: &str,
        item_source: &str,
        format: Format,
        parameters: Option<HashMap<String, String>>,
        include_collections: bool
    ) -> ShareResult<Vec<HashMap<String, serde_json::Value>>> {
        // TODO: Get current user
        let user = "current_user".to_string();
        
        self.get_items(
            item_type,
            Some(item_source),
            None,
            None,
            Some(&user),
            format,
            parameters,
            -1,
            include_collections,
            false
        ).await
    }
    
    /// Get all users an item is shared with
    pub async fn get_users_item_shared(
        &self,
        item_type: &str,
        item_source: &str,
        uid_owner: &str,
        include_collections: bool
    ) -> ShareResult<Vec<String>> {
        let mut users = Vec::new();
        
        let items = self.get_items(
            item_type,
            Some(item_source),
            None,
            None,
            Some(uid_owner),
            Format::None,
            None,
            -1,
            include_collections,
            false
        ).await?;
        
        for item in items {
            if let Some(share_type) = item.get("share_type").and_then(|v| v.as_i64()) {
                if share_type == ShareType::User as i64 {
                    if let Some(share_with) = item.get("share_with").and_then(|v| v.as_str()) {
                        users.push(share_with.to_string());
                    }
                } else if share_type == ShareType::Group as i64 {
                    if let Some(share_with) = item.get("share_with").and_then(|v| v.as_str()) {
                        // TODO: Get group users
                        let group_users = vec!["user1".to_string(), "user2".to_string()];
                        users.extend(group_users);
                    }
                }
            }
        }
        
        // Remove duplicates
        users.sort();
        users.dedup();
        
        Ok(users)
    }
    
    /// Share an item with a user, group, or via private link
    pub async fn share_item(
        &self,
        item_type: &str,
        item_source: &str,
        share_type: ShareType,
        share_with: Option<&str>,
        permissions: Permission,
        item_source_name: Option<&str>
    ) -> ShareResult<Option<String>> {
        // TODO: Get current user
        let uid_owner = "current_user".to_string();
        
        // TODO: Get sharing policy from app config
        let sharing_policy = "global".to_string();
        
        let item_source_name = item_source_name.unwrap_or(item_source);
        
        // Verify share type and sharing conditions are met
        match share_type {
            ShareType::User => {
                let share_with = share_with.ok_or_else(|| 
                    ShareError::SharingFailed(
                        item_source_name.to_string(),
                        "No user specified for user share".to_string()
                    )
                )?;
                
                if share_with == uid_owner {
                    return Err(ShareError::SharingFailed(
                        item_source_name.to_string(),
                        format!("Because the user {} is the item owner", share_with)
                    ));
                }
                
                // TODO: Check if user exists
                
                if sharing_policy == "groups_only" {
                    // TODO: Check if users share a group
                }
                
                // Check if item is already shared with the user
                if let Some(_) = self.get_items(
                    item_type,
                    Some(item_source),
                    Some(*SHARE_TYPE_USER_AND_GROUPS),
                    Some(share_with),
                    None,
                    Format::None,
                    None,
                    1,
                    true,
                    true
                ).await?.into_iter().next() {
                    return Err(ShareError::SharingFailed(
                        item_source_name.to_string(),
                        format!("Because this item is already shared with {}", share_with)
                    ));
                }
            },
            
            ShareType::Group => {
                let share_with = share_with.ok_or_else(|| 
                    ShareError::SharingFailed(
                        item_source_name.to_string(),
                        "No group specified for group share".to_string()
                    )
                )?;
                
                // TODO: Check if group exists
                
                if sharing_policy == "groups_only" {
                    // TODO: Check if user is in group
                }
                
                // Check if item is already shared with the group
                if let Some(_) = self.get_items(
                    item_type,
                    Some(item_source),
                    Some(ShareType::Group as i32),
                    Some(share_with),
                    None,
                    Format::None,
                    None,
                    1,
                    true,
                    true
                ).await?.into_iter().next() {
                    return Err(ShareError::SharingFailed(
                        item_source_name.to_string(),
                        format!("Because this item is already shared with group {}", share_with)
                    ));
                }
                
                // Convert share_with into an array with group and users
                // TODO: Get group users, exclude owner
                let group = share_with.to_string();
                let users = vec!["user1".to_string(), "user2".to_string()];
                
                // TODO: Implement group sharing logic
                // This would be a substantial implementation that shares with a group
                // and handles the special case of unique user targets
                
                return Ok(None);
            },
            
            ShareType::Link => {
                // TODO: Check if links are allowed
                let allow_links = true;
                
                if !allow_links {
                    return Err(ShareError::SharingFailed(
                        item_source_name.to_string(),
                        "Because sharing with links is not allowed".to_string()
                    ));
                }
                
                // When updating a link share
                let old_token = if let Some(check_exists) = self.get_items(
                    item_type,
                    Some(item_source),
                    Some(ShareType::Link as i32),
                    None,
                    Some(&uid_owner),
                    Format::None,
                    None,
                    1,
                    false,
                    false
                ).await?.into_iter().next() {
                    let old_token = check_exists.get("token")
                        .and_then(|t| t.as_str())
                        .map(|s| s.to_string());
                    
                    let id = check_exists.get("id")
                        .and_then(|i| i.as_i64())
                        .ok_or_else(|| ShareError::BackendError("Invalid id".to_string()))?;
                    
                    // Delete the old share
                    self.delete(id, false, None).await?;
                    
                    old_token
                } else {
                    None
                };
                
                // Generate hash of password if provided
                let hashed_password = if let Some(password) = share_with {
                    // TODO: Implement password hashing
                    Some(format!("hashed_{}", password))
                } else {
                    None
                };
                
                // Generate token or reuse old one
                let token = old_token.unwrap_or_else(|| Uuid::new_v4().to_string());
                
                // Put the share in the database
                self.put(
                    item_type,
                    item_source,
                    ShareType::Link,
                    hashed_password.as_deref(),
                    &uid_owner,
                    permissions,
                    None,
                    Some(&token),
                    Some(item_source_name)
                ).await?;
                
                return Ok(Some(token));
            },
            
            _ => {
                return Err(ShareError::SharingFailed(
                    item_source_name.to_string(),
                    format!("Share type {:?} is not valid", share_type)
                ));
            }
        }
        
        // Put the item into the database for user share
        self.put(
            item_type,
            item_source,
            share_type,
            share_with,
            &uid_owner,
            permissions,
            None,
            None,
            Some(item_source_name)
        ).await?;
        
        Ok(None)
    }
    
    /// Unshare an item from a user, group, or delete a private link
    pub async fn unshare(
        &self,
        item_type: &str,
        item_source: &str,
        share_type: ShareType,
        share_with: Option<&str>
    ) -> ShareResult<bool> {
        // TODO: Get current user