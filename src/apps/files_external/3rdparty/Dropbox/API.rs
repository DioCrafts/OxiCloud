use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use reqwest::multipart::{Form, Part};
use reqwest::{Client, ClientBuilder, Response, StatusCode};
use url::Url;
use anyhow::{Result, anyhow, bail, Context};
use std::io::BufReader;
use async_trait::async_trait;
use uuid::Uuid;

/// Dropbox OAuth trait for authentication
#[async_trait]
pub trait DropboxOauth {
    async fn fetch(&self, url: &str, params: HashMap<String, String>, method: &str) -> Result<DropboxResponse>;
    async fn fetch_with_body(&self, url: &str, body: &str, method: &str, headers: HashMap<String, String>) -> Result<DropboxResponse>;
}

/// Response from Dropbox API
#[derive(Debug, Clone)]
pub struct DropboxResponse {
    pub body: String,
    pub http_status: u16,
}

/// Dropbox API client
pub struct DropboxApi<T: DropboxOauth> {
    /// API URL
    api_url: String,
    
    /// Content API URL
    api_content_url: String,
    
    /// OAuth client
    oauth: T,
    
    /// Default root path
    root: String,
    
    /// Use SSL
    use_ssl: bool,
}

/// Root path constants
pub const ROOT_SANDBOX: &str = "sandbox";
pub const ROOT_DROPBOX: &str = "dropbox";

impl<T: DropboxOauth> DropboxApi<T> {
    /// Constructor
    /// 
    /// # Arguments
    /// * `oauth` - Dropbox OAuth implementation
    /// * `root` - Default root path (sandbox or dropbox)
    /// * `use_ssl` - Whether to use SSL
    pub fn new(oauth: T, root: Option<String>, use_ssl: bool) -> Result<Self> {
        if !use_ssl {
            bail!("Dropbox REST API now requires that all requests use SSL");
        }

        Ok(Self {
            api_url: "https://api.dropbox.com/1/".to_string(),
            api_content_url: "https://api-content.dropbox.com/1/".to_string(),
            oauth,
            root: root.unwrap_or_else(|| ROOT_DROPBOX.to_string()),
            use_ssl,
        })
    }

    /// Returns information about the current dropbox account
    pub async fn get_account_info(&self) -> Result<Value> {
        let response = self.oauth.fetch(&format!("{}account/info", self.api_url), HashMap::new(), "GET").await?;
        let data: Value = serde_json::from_str(&response.body)?;
        Ok(data)
    }

    /// Returns a file's contents
    /// 
    /// # Arguments
    /// * `path` - Path to the file
    /// * `root` - Use this to override the default root path
    pub async fn get_file(&self, path: &str, root: Option<&str>) -> Result<String> {
        let root = root.unwrap_or(&self.root);
        let path = self.encode_path(path);
        let url = format!("{}files/{}/{}", self.api_content_url, root, path.trim_start_matches('/'));
        
        let response = self.oauth.fetch(&url, HashMap::new(), "GET").await?;
        Ok(response.body)
    }

    /// Uploads a new file
    /// 
    /// # Arguments
    /// * `path` - Target path (including filename)
    /// * `file_path` - Path to the file to upload
    /// * `root` - Use this to override the default root path
    pub async fn put_file<P: AsRef<Path>>(&self, path: &str, file_path: P, root: Option<&str>) -> Result<bool> {
        let directory = Path::new(path).parent()
            .map_or("", |p| p.to_str().unwrap_or(""));
        let filename = Path::new(path).file_name()
            .map_or("", |p| p.to_str().unwrap_or(""));

        let directory = if directory.is_empty() { 
            "".to_string() 
        } else { 
            self.encode_path(directory)
        };
        
        let root = root.unwrap_or(&self.root);
        
        let file = File::open(file_path)?;
        
        let url = format!("{}files/{}/{}", 
            self.api_content_url, 
            root, 
            directory.trim_matches('/'));
        
        let result = self.multipart_fetch(&url, file, filename).await?;
        
        if result.http_status != 200 {
            bail!("Uploading file to Dropbox failed");
        }
        
        Ok(true)
    }

    /// Copies a file or directory from one location to another
    /// 
    /// # Arguments
    /// * `from` - Source path
    /// * `to` - Destination path
    /// * `root` - Use this to override the default root path
    pub async fn copy(&self, from: &str, to: &str, root: Option<&str>) -> Result<Value> {
        let root = root.unwrap_or(&self.root);
        
        let mut params = HashMap::new();
        params.insert("from_path".to_string(), from.to_string());
        params.insert("to_path".to_string(), to.to_string());
        params.insert("root".to_string(), root.to_string());
        
        let response = self.oauth.fetch(&format!("{}fileops/copy", self.api_url), params, "POST").await?;
        let data: Value = serde_json::from_str(&response.body)?;
        Ok(data)
    }

    /// Creates a new folder
    /// 
    /// # Arguments
    /// * `path` - Path to new folder
    /// * `root` - Use this to override the default root path
    pub async fn create_folder(&self, path: &str, root: Option<&str>) -> Result<Value> {
        let root = root.unwrap_or(&self.root);
        
        let mut params = HashMap::new();
        params.insert("path".to_string(), path.to_string());
        params.insert("root".to_string(), root.to_string());
        
        let response = self.oauth.fetch(&format!("{}fileops/create_folder", self.api_url), params, "POST").await?;
        let data: Value = serde_json::from_str(&response.body)?;
        Ok(data)
    }

    /// Deletes a file or folder
    /// 
    /// # Arguments
    /// * `path` - Path to delete
    /// * `root` - Use this to override the default root path
    pub async fn delete(&self, path: &str, root: Option<&str>) -> Result<Value> {
        let root = root.unwrap_or(&self.root);
        
        let mut params = HashMap::new();
        params.insert("path".to_string(), path.to_string());
        params.insert("root".to_string(), root.to_string());
        
        let response = self.oauth.fetch(&format!("{}fileops/delete", self.api_url), params, "POST").await?;
        let data: Value = serde_json::from_str(&response.body)?;
        Ok(data)
    }

    /// Moves a file or directory to a new location
    /// 
    /// # Arguments
    /// * `from` - Source path
    /// * `to` - Destination path
    /// * `root` - Use this to override the default root path
    pub async fn move_item(&self, from: &str, to: &str, root: Option<&str>) -> Result<Value> {
        let root = root.unwrap_or(&self.root);
        
        let mut params = HashMap::new();
        params.insert("from_path".to_string(), urlencoding::decode(from)?.to_string());
        params.insert("to_path".to_string(), urlencoding::decode(to)?.to_string());
        params.insert("root".to_string(), root.to_string());
        
        let response = self.oauth.fetch(&format!("{}fileops/move", self.api_url), params, "POST").await?;
        let data: Value = serde_json::from_str(&response.body)?;
        Ok(data)
    }

    /// Returns file and directory information
    /// 
    /// # Arguments
    /// * `path` - Path to receive information from
    /// * `list` - When set to true, this method returns information from all files in a directory
    /// * `hash` - If a hash is supplied, this method simply returns true if nothing has changed
    /// * `file_limit` - Maximum number of file-information to receive
    /// * `root` - Use this to override the default root path
    pub async fn get_metadata(
        &self,
        path: &str,
        list: bool,
        hash: Option<&str>,
        file_limit: Option<usize>,
        root: Option<&str>,
    ) -> Result<Value> {
        let root = root.unwrap_or(&self.root);
        
        let mut params = HashMap::new();
        params.insert("list".to_string(), list.to_string());
        
        if let Some(hash_val) = hash {
            params.insert("hash".to_string(), hash_val.to_string());
        }
        
        if let Some(limit) = file_limit {
            params.insert("file_limit".to_string(), limit.to_string());
        }
        
        let path = self.encode_path(path);
        let url = format!("{}metadata/{}/{}", self.api_url, root, path.trim_start_matches('/'));
        
        let response = self.oauth.fetch(&url, params, "GET").await?;
        
        if response.http_status == 304 {
            // Return true if not modified
            Ok(json!(true))
        } else {
            let data: Value = serde_json::from_str(&response.body)?;
            Ok(data)
        }
    }

    /// A way of letting you keep up with changes to files and folders in a user's Dropbox
    /// 
    /// # Arguments
    /// * `cursor` - A string that is used to keep track of your current state
    pub async fn delta(&self, cursor: &str) -> Result<Value> {
        let mut params = HashMap::new();
        params.insert("cursor".to_string(), cursor.to_string());
        
        let response = self.oauth.fetch(&format!("{}delta", self.api_url), params, "POST").await?;
        let data: Value = serde_json::from_str(&response.body)?;
        Ok(data)
    }

    /// Returns a thumbnail for a file path
    /// 
    /// # Arguments
    /// * `path` - Path to file
    /// * `size` - small, medium or large
    /// * `root` - Use this to override the default root path
    pub async fn get_thumbnail(&self, path: &str, size: &str, root: Option<&str>) -> Result<String> {
        let root = root.unwrap_or(&self.root);
        let path = self.encode_path(path);
        
        let mut params = HashMap::new();
        params.insert("size".to_string(), size.to_string());
        
        let url = format!("{}thumbnails/{}/{}", self.api_content_url, root, path.trim_start_matches('/'));
        let response = self.oauth.fetch(&url, params, "GET").await?;
        
        Ok(response.body)
    }

    /// Search for files and folders
    /// 
    /// # Arguments
    /// * `query` - Search query
    /// * `root` - Use this to override the default root path
    /// * `path` - Path to search in
    pub async fn search(&self, query: &str, root: Option<&str>, path: &str) -> Result<Value> {
        let root = root.unwrap_or(&self.root);
        let path = if !path.is_empty() {
            self.encode_path(path)
        } else {
            "".to_string()
        };
        
        let mut params = HashMap::new();
        params.insert("query".to_string(), query.to_string());
        
        let url = format!("{}search/{}/{}", self.api_url, root, path.trim_start_matches('/'));
        let response = self.oauth.fetch(&url, params, "GET").await?;
        
        let data: Value = serde_json::from_str(&response.body)?;
        Ok(data)
    }

    /// Creates and returns a shareable link to files or folders
    /// 
    /// # Arguments
    /// * `path` - Path to share
    /// * `root` - Use this to override the default root path
    pub async fn share(&self, path: &str, root: Option<&str>) -> Result<Value> {
        let root = root.unwrap_or(&self.root);
        let path = self.encode_path(path);
        
        let url = format!("{}shares/{}/{}", self.api_url, root, path.trim_start_matches('/'));
        let response = self.oauth.fetch(&url, HashMap::new(), "POST").await?;
        
        let data: Value = serde_json::from_str(&response.body)?;
        Ok(data)
    }

    /// Returns a link directly to a file
    /// 
    /// # Arguments
    /// * `path` - Path to file
    /// * `root` - Use this to override the default root path
    pub async fn media(&self, path: &str, root: Option<&str>) -> Result<Value> {
        let root = root.unwrap_or(&self.root);
        let path = self.encode_path(path);
        
        let url = format!("{}media/{}/{}", self.api_url, root, path.trim_start_matches('/'));
        let response = self.oauth.fetch(&url, HashMap::new(), "POST").await?;
        
        let data: Value = serde_json::from_str(&response.body)?;
        Ok(data)
    }

    /// Creates and returns a copy_ref to a file
    /// 
    /// # Arguments
    /// * `path` - Path to file
    /// * `root` - Use this to override the default root path
    pub async fn copy_ref(&self, path: &str, root: Option<&str>) -> Result<Value> {
        let root = root.unwrap_or(&self.root);
        let path = self.encode_path(path);
        
        let url = format!("{}copy_ref/{}/{}", self.api_url, root, path.trim_start_matches('/'));
        let response = self.oauth.fetch(&url, HashMap::new(), "GET").await?;
        
        let data: Value = serde_json::from_str(&response.body)?;
        Ok(data)
    }

    // Helper methods
    
    /// Encode path for URL
    fn encode_path(&self, path: &str) -> String {
        let encoded = urlencoding::encode(path);
        encoded.replace("%2F", "/").replace("~", "%7E").to_string()
    }

    /// This method is used to generate multipart POST requests for file upload
    async fn multipart_fetch(&self, uri: &str, mut file: File, filename: &str) -> Result<DropboxResponse> {
        let boundary = format!("R50hrfBj5JYyfR3vF3wR96GPCC9Fd2q2pVMERvEaOE3D8LZTgLLbRpNwXek3{}", Uuid::new_v4());
        
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), format!("multipart/form-data; boundary={}", boundary));
        
        let mut body = String::new();
        body.push_str(&format!("--{}\r\n", boundary));
        body.push_str(&format!("Content-Disposition: form-data; name=file; filename={}\r\n", urlencoding::decode(filename)?));
        body.push_str("Content-type: application/octet-stream\r\n");
        body.push_str("\r\n");
        
        let mut file_content = Vec::new();
        file.read_to_end(&mut file_content)?;
        
        // We need to convert file_content to a String for concatenation
        // This is not ideal for binary files, but we're following the PHP pattern
        let file_content_str = unsafe { String::from_utf8_unchecked(file_content) };
        
        body.push_str(&file_content_str);
        body.push_str("\r\n");
        body.push_str(&format!("--{}--", boundary));
        
        // Append filename to the URI as a query parameter
        let uri_with_param = format!("{}?file={}", uri, filename);
        
        self.oauth.fetch_with_body(&uri_with_param, &body, "POST", headers).await
    }
}