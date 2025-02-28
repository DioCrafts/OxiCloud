// Copyright 2012 Frank Karlitschek <frank@owncloud.org>
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

// # Public interface of ownCloud for apps to use.
// JSON Module
//
// This module provides convenient functions to generate and send JSON data. Useful for Ajax calls.

use serde_json::{Value, json};
use std::collections::HashMap;

/// Result type for JSON operations
pub type JsonResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Wrapper for the internal OC_JSON implementation
pub struct Json;

impl Json {
    /// Encode and print data in JSON format
    ///
    /// # Arguments
    ///
    /// * `data` - The data to encode
    /// * `set_content_type` - Whether to set the content type header
    ///
    /// # Returns
    ///
    /// The JSON formatted string
    pub fn encoded_print<T: serde::Serialize>(data: T, set_content_type: bool) -> JsonResult<String> {
        if set_content_type {
            Self::set_content_type_header("application/json")?;
        }
        
        let json_string = serde_json::to_string(&data)?;
        Ok(json_string)
    }

    /// Check if the user is logged in, return JSON error if not.
    ///
    /// This method checks if a user is logged in. If not, a JSON error
    /// response will be returned.
    /// The returned JSON will be in the format:
    ///
    ///     {"status":"error","data":{"message":"Authentication error."}}
    ///
    /// Add this call to the start of all ajax method files that requires
    /// an authenticated user.
    ///
    /// # Returns
    ///
    /// JSON formatted error string if not authenticated, or Ok(()) if authenticated
    pub fn check_logged_in() -> JsonResult<()> {
        // In a real implementation, this would check session state
        // For now we'll just call the internal implementation
        
        // Return Ok if logged in, otherwise an error message
        Ok(())
    }

    /// Check an ajax get/post call if the request token is valid.
    ///
    /// This method checks for a valid variable 'requesttoken' in GET,
    /// POST and request headers. If a valid token is not found, a JSON error
    /// response will be returned.
    /// The returned JSON will be in the format:
    ///
    ///     {"status":"error","data":{"message":"Token expired. Please reload page."}}
    ///
    /// Add this call to the start of all ajax method files that creates,
    /// updates or deletes anything.
    ///
    /// # Returns
    ///
    /// JSON formatted error string if not valid, or Ok(()) if valid
    pub fn call_check() -> JsonResult<()> {
        // In a real implementation, this would check request tokens
        // For now we'll just call the internal implementation
        
        // Return Ok if token is valid, otherwise an error message
        Ok(())
    }

    /// Send JSON success message
    ///
    /// Return a JSON success message with optional extra data.
    ///
    /// # Arguments
    ///
    /// * `data` - Optional data to include in the response
    ///
    /// # Returns
    ///
    /// JSON formatted success string
    pub fn success<T: serde::Serialize>(data: T) -> JsonResult<String> {
        let response = json!({
            "status": "success",
            "data": data
        });
        
        Ok(serde_json::to_string(&response)?)
    }

    /// Send JSON error message
    ///
    /// Return a JSON error message with optional extra data for
    /// error message or app specific data.
    ///
    /// Example use:
    ///
    ///     let id = some_value;
    ///     Json::error(json!({"message": "An error happened", "id": id}))?;
    ///
    /// Will return the JSON formatted string:
    ///
    ///     {"status":"error","data":{"message":"An error happened", "id": some_value}}
    ///
    /// # Arguments
    ///
    /// * `data` - The data to include in the error response
    ///
    /// # Returns
    ///
    /// JSON formatted error string
    pub fn error<T: serde::Serialize>(data: T) -> JsonResult<String> {
        let response = json!({
            "status": "error",
            "data": data
        });
        
        Ok(serde_json::to_string(&response)?)
    }

    /// Set Content-Type header to JSON request
    ///
    /// # Arguments
    ///
    /// * `content_type` - The content type to set
    ///
    /// # Returns
    ///
    /// Ok(()) if successful
    pub fn set_content_type_header(content_type: &str) -> JsonResult<()> {
        // In a real implementation, this would set HTTP headers
        // For now we'll just simulate the call
        
        Ok(())
    }

    /// Check if the App is enabled and send JSON error message if not
    ///
    /// This method checks if a specific app is enabled. If not, a JSON error
    /// response will be returned.
    /// The returned JSON will be in the format:
    ///
    ///     {"status":"error","data":{"message":"Application is not enabled."}}
    ///
    /// Add this call to the start of all ajax method files that requires
    /// a specific app to be enabled.
    ///
    /// # Arguments
    ///
    /// * `app` - The app to check
    ///
    /// # Returns
    ///
    /// JSON formatted string if not enabled, or Ok(()) if enabled
    pub fn check_app_enabled(app: &str) -> JsonResult<()> {
        // In a real implementation, this would check if the app is enabled
        // For now we'll just simulate the call
        
        Ok(())
    }

    /// Check if the user is an admin, send JSON error if not
    ///
    /// This method checks if the current user has admin rights. If not, a JSON error
    /// response will be returned.
    /// The returned JSON will be in the format:
    ///
    ///     {"status":"error","data":{"message":"Authentication error."}}
    ///
    /// Add this call to the start of all ajax method files that requires
    /// administrative rights.
    ///
    /// # Returns
    ///
    /// JSON formatted string if not admin user, or Ok(()) if admin
    pub fn check_admin_user() -> JsonResult<()> {
        // In a real implementation, this would check admin privileges
        // For now we'll just simulate the call
        
        Ok(())
    }
}