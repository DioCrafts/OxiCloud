// Copyright (c) 2012 Bernhard Posselt nukeawhale@gmail.com
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
use chrono::{DateTime, Utc};
use crate::appframework::http::Http;

/// Base class for responses. Also used to just send headers.
///
/// It handles headers, HTTP status code, last modified and ETag.
pub struct Response {
    /// Headers - defaults to ['Cache-Control' => 'no-cache, must-revalidate']
    headers: HashMap<String, String>,

    /// HTTP status code - defaults to STATUS OK
    status: u16,

    /// Last modified date
    last_modified: Option<DateTime<Utc>>,

    /// ETag
    etag: Option<String>,
}

impl Response {
    /// Create a new Response with default values
    pub fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Cache-Control".to_string(), "no-cache, must-revalidate".to_string());
        
        Self {
            headers,
            status: Http::STATUS_OK,
            last_modified: None,
            etag: None,
        }
    }

    /// Caches the response
    /// 
    /// # Arguments
    /// * `cache_seconds` - the amount of seconds that should be cached
    ///   if 0 then caching will be disabled
    pub fn cache_for(&mut self, cache_seconds: u32) {
        if cache_seconds > 0 {
            self.add_header(
                "Cache-Control".to_string(), 
                format!("max-age={}, must-revalidate", cache_seconds)
            );
        } else {
            self.add_header(
                "Cache-Control".to_string(), 
                "no-cache, must-revalidate".to_string()
            );
        }
    }

    /// Adds a new header to the response that will be called before the render
    /// function
    /// 
    /// # Arguments
    /// * `name` - The name of the HTTP header
    /// * `value` - The value, None will delete it
    pub fn add_header(&mut self, name: String, value: impl Into<Option<String>>) {
        match value.into() {
            None => {
                self.headers.remove(&name);
            },
            Some(val) => {
                self.headers.insert(name, val);
            }
        }
    }

    /// Returns the set headers
    /// 
    /// # Returns
    /// * The headers
    pub fn get_headers(&self) -> HashMap<String, String> {
        let mut result = self.headers.clone();
        
        if let Some(last_modified) = &self.last_modified {
            result.insert(
                "Last-Modified".to_string(),
                last_modified.to_rfc2822()
            );
        }
        
        if let Some(etag) = &self.etag {
            result.insert(
                "ETag".to_string(),
                format!("\"{}\"", etag)
            );
        }
        
        result
    }

    /// By default renders no output
    pub fn render(&self) -> Option<String> {
        None
    }

    /// Set response status
    /// 
    /// # Arguments
    /// * `status` - a HTTP status code, see also the STATUS constants
    pub fn set_status(&mut self, status: u16) {
        self.status = status;
    }

    /// Get response status
    pub fn get_status(&self) -> u16 {
        self.status
    }

    /// Get the ETag
    /// 
    /// # Returns
    /// * the etag
    pub fn get_etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }

    /// Get "last modified" date
    /// 
    /// # Returns
    /// * RFC2822 formatted last modified date
    pub fn get_last_modified(&self) -> Option<&DateTime<Utc>> {
        self.last_modified.as_ref()
    }

    /// Set the ETag
    /// 
    /// # Arguments
    /// * `etag` - The ETag value
    pub fn set_etag(&mut self, etag: String) {
        self.etag = Some(etag);
    }

    /// Set "last modified" date
    /// 
    /// # Arguments
    /// * `last_modified` - The last modified datetime
    pub fn set_last_modified(&mut self, last_modified: DateTime<Utc>) {
        self.last_modified = Some(last_modified);
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}