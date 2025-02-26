//! ownCloud
//!
//! @author Frank Karlitschek
//! @copyright 2012 Frank Karlitschek frank@owncloud.org
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

use async_trait::async_trait;
use log::{error, Level};
use reqwest::{Client, StatusCode};
use std::sync::Arc;
use url::Url;

pub struct WebDavAuthConfig {
    webdavauth_url: String,
}

pub struct WebDavAuth {
    config: Arc<WebDavAuthConfig>,
    client: Client,
}

#[async_trait]
pub trait UserBackend {
    async fn delete_user(&self, uid: &str) -> Result<bool, String>;
    async fn set_password(&self, uid: &str, password: &str) -> Result<bool, String>;
    async fn check_password(&self, uid: &str, password: &str) -> Result<Option<String>, String>;
    async fn user_exists(&self, uid: &str) -> Result<bool, String>;
    async fn has_user_listings(&self) -> bool;
    async fn get_users(&self, search: &str, limit: usize, offset: usize) -> Result<Vec<String>, String>;
}

impl WebDavAuth {
    pub fn new(webdavauth_url: String) -> Result<Self, String> {
        Ok(Self {
            config: Arc::new(WebDavAuthConfig { webdavauth_url }),
            client: Client::new(),
        })
    }
}

#[async_trait]
impl UserBackend for WebDavAuth {
    async fn delete_user(&self, _uid: &str) -> Result<bool, String> {
        error!(
            target: "WebDavAuth", 
            "Not possible to delete users from web frontend using WebDAV user backend"
        );
        Ok(false)
    }

    async fn set_password(&self, _uid: &str, _password: &str) -> Result<bool, String> {
        error!(
            target: "WebDavAuth", 
            "Not possible to change password for users from web frontend using WebDAV user backend"
        );
        Ok(false)
    }

    async fn check_password(&self, uid: &str, password: &str) -> Result<Option<String>, String> {
        let webdavauth_url = &self.config.webdavauth_url;
        
        let parts: Vec<&str> = webdavauth_url.splitn(2, "://").collect();
        if parts.len() != 2 {
            error!(
                target: "WebDavAuth", 
                "Invalid Url: \"{}\"", webdavauth_url
            );
            return Ok(None);
        }
        
        let (webdavauth_protocol, webdavauth_url_path) = (parts[0], parts[1]);
        
        // Build the URL with authentication credentials
        let url_str = format!(
            "{}://{}:{}@{}", 
            webdavauth_protocol, 
            urlencoding::encode(uid), 
            urlencoding::encode(password), 
            webdavauth_url_path
        );
        
        let url = Url::parse(&url_str).map_err(|e| e.to_string())?;
        
        match self.client.head(url).send().await {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    Ok(Some(uid.to_string()))
                } else {
                    Ok(None)
                }
            },
            Err(e) => {
                error!(
                    target: "WebDavAuth", 
                    "Not possible to connect to WebDAV Url: \"{}://{}\" Error: {}", 
                    webdavauth_protocol, 
                    webdavauth_url_path, 
                    e
                );
                Ok(None)
            }
        }
    }

    // We don't know if a user exists without the password, so we have to return true all the time
    async fn user_exists(&self, _uid: &str) -> Result<bool, String> {
        Ok(true)
    }

    async fn has_user_listings(&self) -> bool {
        false
    }

    // We don't know the users so all we can do is return an empty array here
    async fn get_users(&self, _search: &str, _limit: usize, _offset: usize) -> Result<Vec<String>, String> {
        Ok(vec![])
    }
}