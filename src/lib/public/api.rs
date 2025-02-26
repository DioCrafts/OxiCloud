//! ownCloud
//!
//! @author Tom Needham
//! @copyright 2012 Tom Needham tom@owncloud.com
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

//! Public interface of ownCloud for apps to use.
//! API module

use std::collections::HashMap;

use crate::oc_api::{self, AuthLevel};

/// This module provides functions to manage apps in ownCloud
pub struct Api;

/// Callback type for API actions
pub type ActionCallback = Box<dyn Fn() -> Result<(), ApiError> + Send + Sync>;

/// Error type for API operations
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("API error: {0}")]
    General(String),
}

impl Api {
    /// Registers an API call
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method
    /// * `url` - The URL to match
    /// * `action` - The function to run
    /// * `app` - The ID of the app registering the call
    /// * `auth_level` - The level of authentication required for the call
    /// * `defaults` - Default parameters
    /// * `requirements` - Parameter requirements
    pub fn register(
        method: &str,
        url: &str,
        action: ActionCallback,
        app: &str,
        auth_level: AuthLevel,
        defaults: HashMap<String, String>,
        requirements: HashMap<String, String>,
    ) -> Result<(), ApiError> {
        oc_api::register(method, url, action, app, auth_level, defaults, requirements)
            .map_err(|e| ApiError::General(e.to_string()))
    }
}