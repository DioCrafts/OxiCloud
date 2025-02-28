use reqwest::{Client, StatusCode};
use url::Url;
use std::path::PathBuf;
use async_trait::async_trait;

/// User backend using HTTP auth requests
pub struct UserHttp {
    data_directory: PathBuf,
    server_root: PathBuf,
}

#[async_trait]
pub trait UserBackend {
    async fn check_password(&self, uid: &str, password: &str) -> Option<String>;
    fn user_exists(&self, uid: &str) -> bool;
    fn get_home(&self, uid: &str) -> Option<PathBuf>;
}

impl UserHttp {
    pub fn new(data_directory: Option<PathBuf>, server_root: PathBuf) -> Self {
        let data_directory = data_directory.unwrap_or_else(|| server_root.join("data"));
        Self {
            data_directory,
            server_root,
        }
    }

    /// Split http://user@host/path into a user and url part
    /// 
    /// # Arguments
    /// 
    /// * `url` - URL to parse
    /// 
    /// # Returns
    /// 
    /// * `Option<(String, String)>` - Tuple containing username and URL without username
    fn parse_url(&self, url_str: &str) -> Option<(String, String)> {
        let url = Url::parse(url_str).ok()?;
        
        let username = url.username();
        if username.is_empty() {
            return None;
        }
        
        let mut url_without_user = url.clone();
        // Remove the username and password
        let _ = url_without_user.set_username("");
        let _ = url_without_user.set_password(None);
        
        Some((username.to_string(), url_without_user.to_string()))
    }

    /// Check if a URL is a valid login (has a username)
    /// 
    /// # Arguments
    /// 
    /// * `url` - URL to check
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if URL has a username
    fn match_url(&self, url: &str) -> bool {
        if let Ok(parsed_url) = Url::parse(url) {
            !parsed_url.username().is_empty()
        } else {
            false
        }
    }
}

#[async_trait]
impl UserBackend for UserHttp {
    /// Check if the password is correct without logging in the user
    /// 
    /// # Arguments
    /// 
    /// * `uid` - The username (URL)
    /// * `password` - The password
    /// 
    /// # Returns
    /// 
    /// * `Option<String>` - User ID if successful, None otherwise
    async fn check_password(&self, uid: &str, password: &str) -> Option<String> {
        if !self.match_url(uid) {
            return None;
        }
        
        let (user, url) = self.parse_url(uid)?;
        
        let client = Client::new();
        let response = client
            .get(&url)
            .basic_auth(user, Some(password))
            .send()
            .await
            .ok()?;
        
        if response.status() == StatusCode::OK {
            Some(uid.to_string())
        } else {
            None
        }
    }

    /// Check if a user exists
    /// 
    /// # Arguments
    /// 
    /// * `uid` - The username (URL)
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if user exists
    fn user_exists(&self, uid: &str) -> bool {
        self.match_url(uid)
    }

    /// Get the user's home directory
    /// 
    /// # Arguments
    /// 
    /// * `uid` - The username
    /// 
    /// # Returns
    /// 
    /// * `Option<PathBuf>` - Path to user's home directory if user exists
    fn get_home(&self, uid: &str) -> Option<PathBuf> {
        if self.user_exists(uid) {
            Some(self.data_directory.join(uid))
        } else {
            None
        }
    }
}