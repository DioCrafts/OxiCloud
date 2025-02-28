/*
 * Copyright 2011 Google Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Custom error type for authentication errors
#[derive(Debug)]
pub struct AuthError {
    message: String,
}

impl AuthError {
    pub fn new(message: &str) -> Self {
        AuthError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Authentication error: {}", self.message)
    }
}

impl Error for AuthError {}

/// Class to hold information about an authenticated login.
///
/// @author Brian Eaton <beaton@google.com>
pub struct LoginTicket {
    // Information from id token envelope.
    envelope: HashMap<String, serde_json::Value>,

    // Information from id token payload.
    payload: HashMap<String, serde_json::Value>,
}

impl LoginTicket {
    const USER_ATTR: &'static str = "id";

    /// Creates a user based on the supplied token.
    ///
    /// # Arguments
    ///
    /// * `envelope` - Header from a verified authentication token.
    /// * `payload` - Information from a verified authentication token.
    pub fn new(
        envelope: HashMap<String, serde_json::Value>,
        payload: HashMap<String, serde_json::Value>,
    ) -> Self {
        LoginTicket { envelope, payload }
    }

    /// Returns the numeric identifier for the user.
    ///
    /// # Returns
    ///
    /// The user ID as a string
    ///
    /// # Errors
    ///
    /// Returns an `AuthError` if the user ID is not present in the token
    pub fn get_user_id(&self) -> Result<String, AuthError> {
        if let Some(user_id) = self.payload.get(Self::USER_ATTR) {
            if let Some(id) = user_id.as_str() {
                return Ok(id.to_string());
            }
            
            // Handle the case where id might be a number
            if let Some(id) = user_id.as_u64() {
                return Ok(id.to_string());
            }
            
            if let Some(id) = user_id.as_i64() {
                return Ok(id.to_string());
            }
            
            return Err(AuthError::new("User ID in token has invalid format"));
        }
        
        Err(AuthError::new("No user_id in token"))
    }

    /// Returns attributes from the login ticket. This can contain
    /// various information about the user session.
    ///
    /// # Returns
    ///
    /// A HashMap containing "envelope" and "payload" entries
    pub fn get_attributes(&self) -> HashMap<String, HashMap<String, serde_json::Value>> {
        let mut attributes = HashMap::new();
        attributes.insert("envelope".to_string(), self.envelope.clone());
        attributes.insert("payload".to_string(), self.payload.clone());
        attributes
    }
}