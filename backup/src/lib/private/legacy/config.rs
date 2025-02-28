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

use crate::config::Config;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, Once};

/// An example of config.php
///
///
/// $CONFIG = array(
///     "database" => "mysql",
///     "firstrun" => false,
///     "pi" => 3.14
/// );
///

/// This struct is responsible for reading and writing config.php, the very basic
/// configuration file of ownCloud.
pub struct OcConfig {
    config: Arc<Mutex<Config>>,
}

impl OcConfig {
    fn new(server_root: &str) -> Self {
        let config_path = PathBuf::from(server_root).join("config");
        Self {
            config: Arc::new(Mutex::new(Config::new(config_path))),
        }
    }

    /// Returns the singleton instance of OcConfig
    pub fn get_object() -> &'static OcConfig {
        static mut INSTANCE: Option<OcConfig> = None;
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                // In a real implementation, you would get server_root from somewhere
                let server_root = std::env::var("SERVER_ROOT").unwrap_or_else(|_| String::from("."));
                INSTANCE = Some(OcConfig::new(&server_root));
            });
            INSTANCE.as_ref().unwrap()
        }
    }

    /// Lists all available config keys
    ///
    /// This function returns all keys saved in config.php. Please note that it
    /// does not return the values.
    pub fn get_keys(&self) -> Vec<String> {
        let config = self.config.lock().expect("Failed to lock config");
        config.get_keys()
    }

    /// Gets a value from config.php
    ///
    /// This function gets the value from config.php. If it does not exist,
    /// `default` will be returned.
    pub fn get_value<T: Clone>(&self, key: &str, default: Option<T>) -> Option<T> {
        let config = self.config.lock().expect("Failed to lock config");
        config.get_value(key, default)
    }

    /// Sets a value
    ///
    /// This function sets the value and writes the config.php.
    pub fn set_value<T>(&self, key: &str, value: T) -> Result<(), String> 
    where
        T: serde::Serialize,
    {
        let mut config = self.config.lock().expect("Failed to lock config");
        config.set_value(key, value)
    }

    /// Removes a key from the config
    ///
    /// This function removes a key from the config.php.
    pub fn delete_key(&self, key: &str) -> Result<(), String> {
        let mut config = self.config.lock().expect("Failed to lock config");
        config.delete_key(key)
    }
}