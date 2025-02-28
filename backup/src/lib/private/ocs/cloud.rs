// Copyright (C) 2012 Frank Karlitschek <frank@owncloud.org>
// Copyright (C) 2012 Tom Needham <tom@owncloud.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize)]
pub struct OcsResult<T> {
    status: String,
    status_code: i32,
    message: Option<String>,
    data: Option<T>,
}

impl<T> OcsResult<T> {
    pub fn new(data: Option<T>) -> Self {
        Self {
            status: "ok".to_string(),
            status_code: 100,
            message: None,
            data,
        }
    }

    pub fn error(status_code: i32, message: Option<String>) -> Self {
        Self {
            status: "failure".to_string(),
            status_code,
            message,
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    major: u32,
    minor: u32,
    micro: u32,
    string: String,
    edition: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreCapabilities {
    pollinterval: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
    core: CoreCapabilities,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCapabilities {
    version: VersionInfo,
    capabilities: Capabilities,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageQuota {
    free: u64,
    used: u64,
    total: u64,
    relative: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    quota: StorageQuota,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUserInfo {
    id: String,
    #[serde(rename = "display-name")]
    display_name: String,
    email: String,
}

#[async_trait]
pub trait UtilService {
    async fn get_version(&self) -> (u32, u32, u32);
    async fn get_version_string(&self) -> String;
    async fn get_edition_string(&self) -> String;
}

#[async_trait]
pub trait ConfigService {
    async fn get_value(&self, key: &str, default: u64) -> u64;
}

#[async_trait]
pub trait UserService {
    async fn get_user(&self) -> String;
    async fn get_display_name(&self) -> String;
    async fn user_exists(&self, username: &str) -> bool;
    async fn is_admin_user(&self, username: &str) -> bool;
}

#[async_trait]
pub trait PreferencesService {
    async fn get_value(&self, user: &str, app: &str, key: &str, default: &str) -> String;
}

#[async_trait]
pub trait HelperService {
    async fn get_storage_info(&self, path: &str) -> HashMap<String, serde_json::Value>;
}

pub struct OcsCloud<U, C, P, H> {
    util_service: U,
    config_service: C,
    user_service: P,
    preferences_service: H,
    helper_service: H,
}

impl<U, C, P, H> OcsCloud<U, C, P, H>
where
    U: UtilService + Send + Sync,
    C: ConfigService + Send + Sync,
    P: UserService + Send + Sync,
    H: PreferencesService + HelperService + Send + Sync,
{
    pub fn new(
        util_service: U,
        config_service: C,
        user_service: P,
        preferences_service: H,
        helper_service: H,
    ) -> Self {
        Self {
            util_service,
            config_service,
            user_service,
            preferences_service,
            helper_service,
        }
    }

    pub async fn get_capabilities(&self) -> OcsResult<ServerCapabilities> {
        let (major, minor, micro) = self.util_service.get_version().await;
        
        let version_info = VersionInfo {
            major,
            minor,
            micro,
            string: self.util_service.get_version_string().await,
            edition: self.util_service.get_edition_string().await,
        };
        
        let core = CoreCapabilities {
            pollinterval: self.config_service.get_value("pollinterval", 60).await,
        };
        
        let capabilities = Capabilities { core };
        
        let result = ServerCapabilities {
            version: version_info,
            capabilities,
        };
        
        OcsResult::new(Some(result))
    }
    
    /// Gets user info
    ///
    /// Exposes the quota of a user:
    /// ```xml
    /// <data>
    ///   <quota>
    ///      <free>1234</free>
    ///      <used>4321</used>
    ///      <total>5555</total>
    ///      <ralative>0.78</ralative>
    ///   </quota>
    /// </data>
    ///
    ///
    /// The parameter `userid` identifies the user from whom the information will be returned
    pub async fn get_user(&self, userid: &str) -> OcsResult<UserInfo> {
        // Check if they are viewing information on themselves
        if userid == self.user_service.get_user().await {
            // Self lookup
            let storage = self.helper_service.get_storage_info("/").await;
            
            let quota = StorageQuota {
                free: storage["free"].as_u64().unwrap_or(0),
                used: storage["used"].as_u64().unwrap_or(0),
                total: storage["total"].as_u64().unwrap_or(0),
                relative: storage["relative"].as_f64().unwrap_or(0.0),
            };
            
            OcsResult::new(Some(UserInfo { quota }))
        } else {
            // No permission to view this user data
            OcsResult::error(997, None)
        }
    }

    pub async fn get_current_user(&self) -> OcsResult<CurrentUserInfo> {
        let user = self.user_service.get_user().await;
        let email = self.preferences_service
            .get_value(&user, "settings", "email", "")
            .await;
        
        let data = CurrentUserInfo {
            id: user,
            display_name: self.user_service.get_display_name().await,
            email,
        };
        
        OcsResult::new(Some(data))
    }

    pub async fn get_user_publickey(&self, user: &str) -> OcsResult<HashMap<String, String>> {
        if self.user_service.user_exists(user).await {
            // TODO: calculate the disc space
            OcsResult::new(Some(HashMap::new()))
        } else {
            OcsResult::error(300, None)
        }
    }

    pub async fn get_user_privatekey(&self, username: &str) -> OcsResult<String> {
        let current_user = self.user_service.get_user().await;
        
        if self.user_service.is_admin_user(&current_user).await || current_user == username {
            if self.user_service.user_exists(username).await {
                // Return private key
                let private_key = format!("this is the private key of {}", username);
                OcsResult::new(Some(private_key))
            } else {
                OcsResult::error(300, Some("User does not exist".to_string()))
            }
        } else {
            OcsResult::error(300, Some("You don't have permission to access this resource".to_string()))
        }
    }
}