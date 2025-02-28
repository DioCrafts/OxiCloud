// Copyright 2013 Thomas Tanghus (thomas@tanghus.net)
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
use std::fs::File;
use std::io::{self, Read};
use std::ops::{Index, IndexMut};
use std::path::Path;
use serde_json::Value;

/// Interface for request objects
pub trait IRequest {
    fn get_param(&self, key: &str, default: Option<&str>) -> Option<String>;
    fn get_params(&self) -> HashMap<String, String>;
    fn get_method(&self) -> String;
    fn get_uploaded_file(&self, key: &str) -> Option<HashMap<String, String>>;
    fn get_env(&self, key: &str) -> Option<String>;
    fn get_cookie(&self, key: &str) -> Option<String>;
    fn passes_csrf_check(&self) -> bool;
    fn get_header(&self, name: &str) -> Option<String>;
}

/// Class for accessing variables in the request.
/// This class provides an immutable object with request variables.
pub struct Request {
    input_stream: String,
    content: Option<Content>,
    items: HashMap<String, RequestItem>,
    allowed_keys: Vec<String>,
}

enum Content {
    Map(HashMap<String, String>),
    String(String),
    Stream(File),
    None,
}

enum RequestItem {
    Map(HashMap<String, String>),
    String(String),
    Bool(bool),
}

impl Request {
    /// Create a new Request object
    ///
    /// # Parameters
    /// * `vars` - An associative array with the following optional values:
    ///   * `url_params` - the parameters which were matched from the URL
    ///   * `get` - the GET array
    ///   * `post` - the POST array or JSON string
    ///   * `files` - the FILES array
    ///   * `server` - the SERVER array
    ///   * `env` - the ENV array
    ///   * `cookies` - the COOKIE array
    ///   * `method` - the request method (GET, POST etc)
    ///   * `requesttoken` - the requesttoken or false when not available
    pub fn new(vars: HashMap<String, HashMap<String, String>>) -> Self {
        let allowed_keys = vec![
            "get".to_string(), 
            "post".to_string(), 
            "files".to_string(),
            "server".to_string(), 
            "env".to_string(), 
            "cookies".to_string(),
            "url_params".to_string(), 
            "parameters".to_string(), 
            "method".to_string(),
            "requesttoken".to_string(),
        ];

        let mut items = HashMap::new();
        for name in &allowed_keys {
            if let Some(value) = vars.get(name) {
                items.insert(name.clone(), RequestItem::Map(value.clone()));
            } else {
                items.insert(name.clone(), RequestItem::Map(HashMap::new()));
            }
        }

        // Get method from items
        let method = if let RequestItem::Map(server) = &items["server"] {
            server.get("REQUEST_METHOD").cloned().unwrap_or_else(|| "GET".to_string())
        } else {
            "GET".to_string()
        };
        items.insert("method".to_string(), RequestItem::String(method.clone()));

        // Set input stream
        let input_stream = if cfg!(test) {
            "fakeinput://data".to_string()
        } else {
            "php://input".to_string() // This is just a placeholder in Rust
        };

        let mut request = Self {
            input_stream,
            content: None,
            items,
            allowed_keys,
        };

        // Process JSON content if needed
        if method == "POST" {
            if let Some(content_type) = request.get_header("Content-Type") {
                if content_type.contains("application/json") {
                    // In a real implementation, we would read from input_stream
                    // For now, we'll assume the post data is already parsed
                    if let Some(RequestItem::Map(post)) = request.items.get("post") {
                        request.items.insert("params".to_string(), RequestItem::Map(post.clone()));
                    }
                }
            }
        }

        // Merge parameters
        let mut parameters = HashMap::new();
        if let RequestItem::Map(get) = &request.items["get"] {
            parameters.extend(get.clone());
        }
        if let RequestItem::Map(post) = &request.items["post"] {
            parameters.extend(post.clone());
        }
        if let RequestItem::Map(url_params) = &request.items["url_params"] {
            parameters.extend(url_params.clone());
        }
        request.items.insert("parameters".to_string(), RequestItem::Map(parameters));

        request
    }

    /// Returns the request body content.
    ///
    /// If the HTTP request method is PUT and the body
    /// not application/x-www-form-urlencoded or application/json a stream
    /// resource is returned, otherwise an array.
    ///
    /// # Returns
    /// The request body content or a resource to read the body stream.
    fn get_content(&mut self) -> Result<&Content, &'static str> {
        if let Some(Content::None) = self.content {
            if self.get_method() == "PUT" {
                return Err("'put' can only be accessed once if not application/x-www-form-urlencoded or application/json.");
            }
        }

        // Check if we need to return a stream for PUT requests
        if self.get_method() == "PUT" {
            let content_type = self.get_header("Content-Type").unwrap_or_default();
            if !content_type.contains("application/x-www-form-urlencoded") && !content_type.contains("application/json") {
                // In a real implementation, we would open the stream here
                // self.content = Some(Content::Stream(File::open(&self.input_stream)?));
                self.content = Some(Content::None);
                return Ok(&self.content.as_ref().unwrap());
            }
        }

        if self.content.is_none() {
            // In a real implementation, we would read from input_stream
            // For now, we'll use an empty string
            let content_str = String::new();
            
            let content_type = self.get_header("Content-Type").unwrap_or_default();
            if content_type.contains("application/x-www-form-urlencoded") {
                // Parse x-www-form-urlencoded data
                let mut map = HashMap::new();
                for pair in content_str.split('&') {
                    let mut parts = pair.splitn(2, '=');
                    if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                        map.insert(key.to_string(), value.to_string());
                    }
                }
                self.content = Some(Content::Map(map));
            } else if content_type.contains("application/json") {
                // Parse JSON data
                if let Ok(json) = serde_json::from_str::<serde_json::Map<String, Value>>(&content_str) {
                    let mut map = HashMap::new();
                    for (key, value) in json {
                        if let Some(string_val) = value.as_str() {
                            map.insert(key, string_val.to_string());
                        } else {
                            map.insert(key, value.to_string());
                        }
                    }
                    self.content = Some(Content::Map(map));
                } else {
                    self.content = Some(Content::String(content_str));
                }
            } else {
                self.content = Some(Content::String(content_str));
            }
        }

        Ok(self.content.as_ref().unwrap())
    }
}

impl Index<&str> for Request {
    type Output = str;

    fn index(&self, key: &str) -> &Self::Output {
        if let Some(RequestItem::Map(params)) = self.items.get("parameters") {
            if let Some(value) = params.get(key) {
                return value;
            }
        }
        ""
    }
}

impl IRequest for Request {
    fn get_param(&self, key: &str, default: Option<&str>) -> Option<String> {
        if let Some(RequestItem::Map(params)) = self.items.get("parameters") {
            if let Some(value) = params.get(key) {
                return Some(value.clone());
            }
        }
        default.map(String::from)
    }

    fn get_params(&self) -> HashMap<String, String> {
        if let Some(RequestItem::Map(params)) = self.items.get("parameters") {
            params.clone()
        } else {
            HashMap::new()
        }
    }

    fn get_method(&self) -> String {
        if let Some(RequestItem::String(method)) = self.items.get("method") {
            method.clone()
        } else {
            "GET".to_string()
        }
    }

    fn get_uploaded_file(&self, key: &str) -> Option<HashMap<String, String>> {
        if let Some(RequestItem::Map(files)) = self.items.get("files") {
            if let Some(file) = files.get(key) {
                // In PHP, files are arrays of metadata
                // For simplicity, we'll just return a HashMap with the file path
                let mut file_data = HashMap::new();
                file_data.insert("path".to_string(), file.clone());
                return Some(file_data);
            }
        }
        None
    }

    fn get_env(&self, key: &str) -> Option<String> {
        if let Some(RequestItem::Map(env)) = self.items.get("env") {
            env.get(key).cloned()
        } else {
            None
        }
    }

    fn get_cookie(&self, key: &str) -> Option<String> {
        if let Some(RequestItem::Map(cookies)) = self.items.get("cookies") {
            cookies.get(key).cloned()
        } else {
            None
        }
    }

    fn get_header(&self, name: &str) -> Option<String> {
        let header_name = format!("HTTP_{}", name.to_uppercase().replace("-", "_"));
        
        if let Some(RequestItem::Map(server)) = self.items.get("server") {
            if let Some(value) = server.get(&header_name) {
                return Some(value.clone());
            }
            
            // Special cases
            match name.to_uppercase().as_str() {
                "CONTENT_TYPE" | "CONTENT_LENGTH" => {
                    if let Some(value) = server.get(&name.to_uppercase()) {
                        return Some(value.clone());
                    }
                },
                _ => {}
            }
        }
        
        None
    }

    fn passes_csrf_check(&self) -> bool {
        let request_token = match &self.items.get("requesttoken") {
            Some(RequestItem::String(token)) => token,
            Some(RequestItem::Bool(false)) => return false,
            _ => return false,
        };

        let token = if let Some(RequestItem::Map(get)) = self.items.get("get") {
            if let Some(token) = get.get("requesttoken") {
                Some(token)
            } else if let Some(RequestItem::Map(post)) = self.items.get("post") {
                post.get("requesttoken")
            } else if let Some(RequestItem::Map(server)) = self.items.get("server") {
                server.get("HTTP_REQUESTTOKEN")
            } else {
                None
            }
        } else {
            None
        };

        match token {
            Some(token) => token == request_token,
            None => false,
        }
    }
}

// Implementation for Countable trait (count method)
impl Request {
    pub fn count(&self) -> usize {
        if let Some(RequestItem::Map(params)) = self.items.get("parameters") {
            params.len()
        } else {
            0
        }
    }
}