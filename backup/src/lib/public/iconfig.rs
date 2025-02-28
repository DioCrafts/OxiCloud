// Copyright (C) 2013 Bart Visscher <bartv@thisnet.nl>
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

// Public interface of ownCloud for apps to use.
// Config interface

use std::error::Error;

/// Access to all the configuration options ownCloud offers
#[async_trait::async_trait]
pub trait IConfig: Send + Sync {
    /// Looks up a system wide defined value
    ///
    /// # Arguments
    /// * `key` - the key of the value, under which it was saved
    ///
    /// # Returns
    /// The saved value or an error if not found
    async fn get_system_value(&self, key: &str) -> Result<String, Box<dyn Error + Send + Sync>>;

    /// Writes a new app wide value
    ///
    /// # Arguments
    /// * `app_name` - the app_name that we want to store the value under
    /// * `key` - the key of the value, under which will be saved
    /// * `value` - the value that should be stored
    async fn set_app_value(&self, app_name: &str, key: &str, value: &str) -> Result<(), Box<dyn Error + Send + Sync>>;

    /// Looks up an app wide defined value
    ///
    /// # Arguments
    /// * `app_name` - the app_name that we stored the value under
    /// * `key` - the key of the value, under which it was saved
    ///
    /// # Returns
    /// The saved value or an error if not found
    async fn get_app_value(&self, app_name: &str, key: &str) -> Result<String, Box<dyn Error + Send + Sync>>;

    /// Set a user defined value
    ///
    /// # Arguments
    /// * `user_id` - the user_id of the user that we want to store the value under
    /// * `app_name` - the app_name that we want to store the value under
    /// * `key` - the key under which the value is being stored
    /// * `value` - the value that you want to store
    async fn set_user_value(&self, user_id: &str, app_name: &str, key: &str, value: &str) -> Result<(), Box<dyn Error + Send + Sync>>;

    /// Shortcut for getting a user defined value
    ///
    /// # Arguments
    /// * `user_id` - the user_id of the user that we want to store the value under
    /// * `app_name` - the app_name that we stored the value under
    /// * `key` - the key under which the value is being stored
    ///
    /// # Returns
    /// The saved value or an error if not found
    async fn get_user_value(&self, user_id: &str, app_name: &str, key: &str) -> Result<String, Box<dyn Error + Send + Sync>>;
}