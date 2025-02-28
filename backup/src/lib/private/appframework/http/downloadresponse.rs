// Copyright (C) 2012 Bernhard Posselt <nukeawhale@gmail.com>
// This file is licensed under the GNU AFFERO GENERAL PUBLIC LICENSE Version 3
// See the LICENSE file for details.

use std::collections::HashMap;

/// Represents an HTTP response in the application framework
pub trait Response {
    fn get_status(&self) -> u16;
    fn set_status(&mut self, status: u16);
    fn get_headers(&self) -> &HashMap<String, String>;
    fn add_header(&mut self, key: &str, value: &str);
    fn get_content(&self) -> &[u8];
    fn set_content(&mut self, content: Vec<u8>);
}

/// Base implementation of the Response trait
pub struct BaseResponse {
    status: u16,
    headers: HashMap<String, String>,
    content: Vec<u8>,
}

impl BaseResponse {
    pub fn new() -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            content: Vec::new(),
        }
    }
}

impl Response for BaseResponse {
    fn get_status(&self) -> u16 {
        self.status
    }

    fn set_status(&mut self, status: u16) {
        self.status = status;
    }

    fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    fn get_content(&self) -> &[u8] {
        &self.content
    }

    fn set_content(&mut self, content: Vec<u8>) {
        self.content = content;
    }
}

/// Prompts the user to download the a file
pub struct DownloadResponse {
    response: BaseResponse,
    filename: String,
    content_type: String,
}

impl DownloadResponse {
    /// Creates a response that prompts the user to download the file
    /// 
    /// # Arguments
    /// * `filename` - the name that the downloaded file should have
    /// * `content_type` - the mimetype that the downloaded file should have
    pub fn new(filename: &str, content_type: &str) -> Self {
        let mut response = BaseResponse::new();
        
        response.add_header(
            "Content-Disposition", 
            &format!("attachment; filename=\"{}\"", filename)
        );
        response.add_header("Content-Type", content_type);
        
        Self {
            response,
            filename: filename.to_string(),
            content_type: content_type.to_string(),
        }
    }
}

impl Response for DownloadResponse {
    fn get_status(&self) -> u16 {
        self.response.get_status()
    }

    fn set_status(&mut self, status: u16) {
        self.response.set_status(status);
    }

    fn get_headers(&self) -> &HashMap<String, String> {
        self.response.get_headers()
    }

    fn add_header(&mut self, key: &str, value: &str) {
        self.response.add_header(key, value);
    }

    fn get_content(&self) -> &[u8] {
        self.response.get_content()
    }

    fn set_content(&mut self, content: Vec<u8>) {
        self.response.set_content(content);
    }
}