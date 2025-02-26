//! OCS Result type for the API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result object for OCS API operations
///
/// Contains data, status code, and message for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcsResult<T> {
    data: Option<T>,
    status_code: i32,
    message: Option<String>,
    items: Option<i32>,
    per_page: Option<i32>,
}

impl<T> OcsResult<T> {
    /// Create a new OCS result object
    ///
    /// # Arguments
    /// * `data` - The data to return
    /// * `code` - Status code (default: 100)
    /// * `message` - Optional message
    pub fn new(data: Option<T>, code: i32, message: Option<String>) -> Self {
        Self {
            data,
            status_code: code,
            message,
            items: None,
            per_page: None,
        }
    }

    /// Set the total number of items available
    ///
    /// # Arguments
    /// * `items` - Total number of items
    pub fn set_total_items(&mut self, items: i32) {
        self.items = Some(items);
    }

    /// Set the number of items per page
    ///
    /// # Arguments
    /// * `items` - Items per page
    pub fn set_items_per_page(&mut self, items: i32) {
        self.per_page = Some(items);
    }

    /// Get the status code
    pub fn get_status_code(&self) -> i32 {
        self.status_code
    }

    /// Get the metadata for the result
    pub fn get_meta(&self) -> HashMap<String, serde_json::Value> {
        let mut meta = HashMap::new();
        
        let status = if self.status_code == 100 { "ok" } else { "failure" };
        meta.insert("status".to_string(), serde_json::Value::String(status.to_string()));
        meta.insert("statuscode".to_string(), serde_json::Value::Number(self.status_code.into()));
        
        if let Some(message) = &self.message {
            meta.insert("message".to_string(), serde_json::Value::String(message.clone()));
        }
        
        if let Some(items) = self.items {
            meta.insert("totalitems".to_string(), serde_json::Value::Number(items.into()));
        }
        
        if let Some(per_page) = self.per_page {
            meta.insert("itemsperpage".to_string(), serde_json::Value::Number(per_page.into()));
        }
        
        meta
    }

    /// Get the result data
    pub fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// Check if the method succeeded
    pub fn succeeded(&self) -> bool {
        let first_digit = self.status_code / 100;
        first_digit == 1
    }
}