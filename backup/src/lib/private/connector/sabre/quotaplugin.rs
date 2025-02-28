use std::cmp;

/**
 * This plugin check user quota and deny creating files when they exceeds the quota.
 *
 * @author Sergio Cambra
 * @copyright Copyright (C) 2012 entreCables S.L. All rights reserved.
 * @license http://code.google.com/p/sabredav/wiki/license Modified BSD License
 */

/// A constant representing unknown space
pub const SPACE_UNKNOWN: i64 = -1;

/// Error type for quota-related issues
#[derive(Debug, thiserror::Error)]
pub enum QuotaError {
    #[error("Insufficient storage")]
    InsufficientStorage,
    #[error("File system error: {0}")]
    FileSystemError(String),
}

/// Filesystem view abstraction
pub trait FileView {
    fn free_space(&self, path: &str) -> Result<i64, QuotaError>;
}

/// HTTP request abstraction
pub trait HttpRequest {
    fn get_header(&self, name: &str) -> Option<String>;
}

/// Server abstraction
pub trait DavServer {
    fn http_request(&self) -> &dyn HttpRequest;
    fn subscribe_event<F>(&mut self, event: &str, callback: F, priority: u8)
    where
        F: Fn(&str, Option<&[u8]>) -> Result<bool, QuotaError> + 'static;
}

/// Plugin implementation that checks user quota and denies creating files when exceeding quota
pub struct QuotaPlugin {
    server: Option<Box<dyn DavServer>>,
    file_view: Option<Box<dyn FileView>>,
}

impl QuotaPlugin {
    /// Create a new quota plugin
    pub fn new() -> Self {
        Self {
            server: None,
            file_view: None,
        }
    }

    /// Set the file view implementation
    pub fn set_file_view(&mut self, file_view: Box<dyn FileView>) {
        self.file_view = Some(file_view);
    }

    /// This initializes the plugin.
    ///
    /// This function is called by the DAV server, after
    /// add_plugin is called.
    ///
    /// This method should set up the required event subscriptions.
    pub fn initialize(&mut self, server: Box<dyn DavServer>) {
        self.server = Some(server);
        
        if let Some(server) = &mut self.server {
            let plugin = self.clone();
            server.subscribe_event("beforeWriteContent", move |uri, data| {
                plugin.check_quota(uri, data)
            }, 10);
            
            let plugin = self.clone();
            server.subscribe_event("beforeCreateFile", move |uri, data| {
                plugin.check_quota(uri, data)
            }, 10);
        }
    }

    /// This method is called before any HTTP method and validates there is enough free space to store the file
    pub fn check_quota(&self, uri: &str, data: Option<&[u8]>) -> Result<bool, QuotaError> {
        if let Some(length) = self.get_length() {
            let uri = if !uri.starts_with('/') {
                format!("/{}", uri)
            } else {
                uri.to_string()
            };
            
            let (parent_uri, _new_name) = self.split_path(&uri);
            
            if let Some(free_space) = self.get_free_space(&parent_uri)? {
                if length > free_space {
                    return Err(QuotaError::InsufficientStorage);
                }
            }
        }
        Ok(true)
    }

    /// Get the content length from request headers
    fn get_length(&self) -> Option<i64> {
        if let Some(server) = &self.server {
            let req = server.http_request();
            
            let length = req.get_header("X-Expected-Entity-Length")
                .or_else(|| req.get_header("Content-Length"))
                .and_then(|s| s.parse::<i64>().ok());
                
            let oc_length = req.get_header("OC-Total-Length")
                .and_then(|s| s.parse::<i64>().ok());
                
            if let (Some(len), Some(oc_len)) = (length, oc_length) {
                return Some(cmp::max(len, oc_len));
            }
            
            return length;
        }
        None
    }

    /// Get available free space for the given path
    fn get_free_space(&self, parent_uri: &str) -> Result<Option<i64>, QuotaError> {
        if let Some(file_view) = &self.file_view {
            let free_space = file_view.free_space(parent_uri)?;
            if free_space != SPACE_UNKNOWN {
                return Ok(Some(free_space));
            }
        }
        Ok(None)
    }
    
    /// Split a path into parent and child components
    fn split_path(&self, path: &str) -> (String, String) {
        let path = path.trim_end_matches('/');
        if let Some(pos) = path.rfind('/') {
            let parent = if pos == 0 { "/".to_string() } else { path[..pos].to_string() };
            let name = path[(pos + 1)..].to_string();
            (parent, name)
        } else {
            ("/".to_string(), path.to_string())
        }
    }
}

impl Clone for QuotaPlugin {
    fn clone(&self) -> Self {
        Self {
            server: None, // Can't clone the server reference
            file_view: None, // Can't clone the file view reference
        }
    }
}