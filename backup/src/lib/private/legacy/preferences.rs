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

use std::sync::{Arc, Mutex, Once};

/// Trait defining the interface for preferences storage
pub trait PreferencesBackend: Send + Sync {
    /// Get all users using the preferences
    fn get_users(&self) -> Vec<String>;

    /// Get all apps of a user
    fn get_apps(&self, user: &str) -> Vec<String>;

    /// Get the available keys for an app
    fn get_keys(&self, user: &str, app: &str) -> Vec<String>;

    /// Gets the preference value
    fn get_value(&self, user: &str, app: &str, key: &str, default: Option<&str>) -> String;

    /// Sets a value in the preferences
    fn set_value(&self, user: &str, app: &str, key: &str, value: &str);

    /// Deletes a key
    fn delete_key(&self, user: &str, app: &str, key: &str);

    /// Remove app of user from preferences
    fn delete_app(&self, user: &str, app: &str);

    /// Remove user from preferences
    fn delete_user(&self, user: &str);

    /// Remove app from all users
    fn delete_app_from_all_users(&self, app: &str);
}

/// Implementation of preference backend using a database connection
pub struct DbPreferences {
    conn: Arc<SqlConnection>,
}

impl DbPreferences {
    /// Create a new DB preferences backend
    pub fn new(conn: Arc<SqlConnection>) -> Self {
        Self { conn }
    }
}

impl PreferencesBackend for DbPreferences {
    fn get_users(&self) -> Vec<String> {
        // Implementation would use the DB connection to fetch users
        // For example:
        // let query = "SELECT DISTINCT userid FROM preferences";
        // self.conn.query_map(query, |row| row.get(0))
        Vec::new() // Placeholder
    }

    fn get_apps(&self, user: &str) -> Vec<String> {
        // Implementation would use the DB connection to fetch apps
        // For example:
        // let query = "SELECT DISTINCT appid FROM preferences WHERE userid = ?";
        // self.conn.query_map(query, params![user], |row| row.get(0))
        Vec::new() // Placeholder
    }

    fn get_keys(&self, user: &str, app: &str) -> Vec<String> {
        // Implementation would use the DB connection to fetch keys
        // For example:
        // let query = "SELECT configkey FROM preferences WHERE userid = ? AND appid = ?";
        // self.conn.query_map(query, params![user, app], |row| row.get(0))
        Vec::new() // Placeholder
    }

    fn get_value(&self, user: &str, app: &str, key: &str, default: Option<&str>) -> String {
        // Implementation would use the DB connection to fetch value
        // For example:
        // let query = "SELECT configvalue FROM preferences WHERE userid = ? AND appid = ? AND configkey = ?";
        // self.conn.query_row(query, params![user, app, key], |row| row.get(0))
        // .unwrap_or_else(|_| default.unwrap_or("").to_string())
        default.unwrap_or("").to_string() // Placeholder
    }

    fn set_value(&self, user: &str, app: &str, key: &str, value: &str) {
        // Implementation would use the DB connection to set value
        // For example:
        // let query = "REPLACE INTO preferences (userid, appid, configkey, configvalue) VALUES (?, ?, ?, ?)";
        // self.conn.execute(query, params![user, app, key, value]);
    }

    fn delete_key(&self, user: &str, app: &str, key: &str) {
        // Implementation would use the DB connection to delete key
        // For example:
        // let query = "DELETE FROM preferences WHERE userid = ? AND appid = ? AND configkey = ?";
        // self.conn.execute(query, params![user, app, key]);
    }

    fn delete_app(&self, user: &str, app: &str) {
        // Implementation would use the DB connection to delete app
        // For example:
        // let query = "DELETE FROM preferences WHERE userid = ? AND appid = ?";
        // self.conn.execute(query, params![user, app]);
    }

    fn delete_user(&self, user: &str) {
        // Implementation would use the DB connection to delete user
        // For example:
        // let query = "DELETE FROM preferences WHERE userid = ?";
        // self.conn.execute(query, params![user]);
    }

    fn delete_app_from_all_users(&self, app: &str) {
        // Implementation would use the DB connection to delete app from all users
        // For example:
        // let query = "DELETE FROM preferences WHERE appid = ?";
        // self.conn.execute(query, params![app]);
    }
}

// SqlConnection is a placeholder for the actual DB connection type
pub struct SqlConnection {}

/// This struct provides an easy way for storing user preferences.
pub struct OCPreferences {
    backend: Arc<Mutex<Box<dyn PreferencesBackend>>>,
}

impl OCPreferences {
    /// Get the singleton instance of OCPreferences
    pub fn instance() -> &'static Self {
        static mut INSTANCE: Option<OCPreferences> = None;
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                // This would typically be initialized with the actual DB connection
                let conn = Arc::new(SqlConnection {});
                let backend = Arc::new(Mutex::new(Box::new(DbPreferences::new(conn)) as Box<dyn PreferencesBackend>));
                INSTANCE = Some(OCPreferences { backend });
            });
            INSTANCE.as_ref().unwrap()
        }
    }

    /// Get all users using the preferences
    ///
    /// This function returns a list of all users that have at least one entry
    /// in the preferences table.
    pub fn get_users(&self) -> Vec<String> {
        self.backend.lock().unwrap().get_users()
    }

    /// Get all apps of a user
    ///
    /// This function returns a list of all apps of the user that have at least
    /// one entry in the preferences table.
    pub fn get_apps(&self, user: &str) -> Vec<String> {
        self.backend.lock().unwrap().get_apps(user)
    }

    /// Get the available keys for an app
    ///
    /// This function gets all keys of an app of an user. Please note that the
    /// values are not returned.
    pub fn get_keys(&self, user: &str, app: &str) -> Vec<String> {
        self.backend.lock().unwrap().get_keys(user, app)
    }

    /// Gets the preference
    ///
    /// This function gets a value from the preferences table. If the key does
    /// not exist the default value will be returned
    pub fn get_value(&self, user: &str, app: &str, key: &str, default: Option<&str>) -> String {
        self.backend.lock().unwrap().get_value(user, app, key, default)
    }

    /// Sets a value in the preferences
    ///
    /// Adds a value to the preferences. If the key did not exist before, it
    /// will be added automatically.
    pub fn set_value(&self, user: &str, app: &str, key: &str, value: &str) -> bool {
        self.backend.lock().unwrap().set_value(user, app, key, value);
        true
    }

    /// Deletes a key
    pub fn delete_key(&self, user: &str, app: &str, key: &str) -> bool {
        self.backend.lock().unwrap().delete_key(user, app, key);
        true
    }

    /// Remove app of user from preferences
    ///
    /// Removes all keys in preferences belonging to the app and the user.
    pub fn delete_app(&self, user: &str, app: &str) -> bool {
        self.backend.lock().unwrap().delete_app(user, app);
        true
    }

    /// Remove user from preferences
    ///
    /// Removes all keys in preferences belonging to the user.
    pub fn delete_user(&self, user: &str) -> bool {
        self.backend.lock().unwrap().delete_user(user);
        true
    }

    /// Remove app from all users
    ///
    /// Removes all keys in preferences belonging to the app.
    pub fn delete_app_from_all_users(&self, app: &str) -> bool {
        self.backend.lock().unwrap().delete_app_from_all_users(app);
        true
    }
}