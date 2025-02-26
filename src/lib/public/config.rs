// Copyright (c) 2012 Frank Karlitschek frank@owncloud.org
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

//! Public interface of ownCloud for apps to use.
//! Config Class

/// This module provides functions to read and write configuration data.
/// Configuration can be on a system, application or user level.
pub struct Config;

impl Config {
    /// Gets a value from config.php
    ///
    /// This function gets the value from config.php. If it does not exist,
    /// `default` will be returned.
    ///
    /// # Parameters
    /// * `key` - The key to lookup
    /// * `default` - Default value to return if key doesn't exist
    ///
    /// # Returns
    /// The value or `default`
    pub fn get_system_value<T>(key: &str, default: Option<T>) -> T 
    where 
        T: Clone,
    {
        crate::oc_config::get_value(key, default)
    }

    /// Sets a value in the system configuration
    ///
    /// This function sets the value and writes the config file. If the file can
    /// not be written, an error will be returned.
    ///
    /// # Parameters
    /// * `key` - The key to set
    /// * `value` - The value to set
    ///
    /// # Returns
    /// `Ok(())` on success, `Err` on failure
    pub fn set_system_value<T>(key: &str, value: T) -> Result<(), Box<dyn std::error::Error>> {
        crate::oc_config::set_value(key, value)?;
        Ok(())
    }

    /// Gets the config value for an app
    ///
    /// This function gets a value from the appconfig table. If the key does
    /// not exist the default value will be returned
    ///
    /// # Parameters
    /// * `app` - The app name
    /// * `key` - The key to lookup
    /// * `default` - Default value to return if key doesn't exist
    ///
    /// # Returns
    /// The value or `default`
    pub fn get_app_value<T>(app: &str, key: &str, default: Option<T>) -> T 
    where 
        T: Clone,
    {
        crate::oc_appconfig::get_value(app, key, default)
    }

    /// Sets a value in the appconfig
    ///
    /// Sets a value. If the key did not exist before it will be created.
    ///
    /// # Parameters
    /// * `app` - The app name
    /// * `key` - The key to set
    /// * `value` - The value to set
    ///
    /// # Returns
    /// `Ok(())` on success, `Err` on failure
    pub fn set_app_value<T>(app: &str, key: &str, value: T) -> Result<(), Box<dyn std::error::Error>> {
        crate::oc_appconfig::set_value(app, key, value)?;
        Ok(())
    }

    /// Gets the preference for a user
    ///
    /// This function gets a value from the preferences table. If the key does
    /// not exist the default value will be returned
    ///
    /// # Parameters
    /// * `user` - The user ID
    /// * `app` - The app name
    /// * `key` - The key to lookup
    /// * `default` - Default value to return if key doesn't exist
    ///
    /// # Returns
    /// The value or `default`
    pub fn get_user_value<T>(user: &str, app: &str, key: &str, default: Option<T>) -> T 
    where 
        T: Clone,
    {
        crate::oc_preferences::get_value(user, app, key, default)
    }

    /// Sets a value in the preferences for a user
    ///
    /// Adds a value to the preferences. If the key did not exist before, it
    /// will be added automatically.
    ///
    /// # Parameters
    /// * `user` - The user ID
    /// * `app` - The app name
    /// * `key` - The key to set
    /// * `value` - The value to set
    ///
    /// # Returns
    /// `Ok(())` on success, `Err` on failure
    pub fn set_user_value<T>(user: &str, app: &str, key: &str, value: T) -> Result<(), Box<dyn std::error::Error>> {
        crate::oc_preferences::set_value(user, app, key, value)?;
        Ok(())
    }
}