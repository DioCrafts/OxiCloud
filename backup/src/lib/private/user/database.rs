// ownCloud
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
//
// @author Frank Karlitschek
// @copyright 2012 Frank Karlitschek frank@owncloud.org

/*
 * The following SQL statement is just a help for developers and will not be
 * executed!
 *
 * CREATE TABLE `users` (
 *   `uid` varchar(64) COLLATE utf8_unicode_ci NOT NULL,
 *   `password` varchar(255) COLLATE utf8_unicode_ci NOT NULL,
 *   PRIMARY KEY (`uid`)
 * ) ENGINE=MyISAM DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
 */

use crate::config::{self, Config};
use crate::db::{DB, Error as DBError, QueryResult};
use crate::log;
use crate::server::ServerRoot;
use crate::user::UserBackend;
use lazy_static::lazy_static;
use phpass::{PasswordHash, PasswordHasher};
use std::sync::Mutex;
use std::path::PathBuf;

lazy_static! {
    static ref HASHER: Mutex<Option<PasswordHasher>> = Mutex::new(None);
}

/// Class for user management in a SQL Database (e.g. MySQL, SQLite)
pub struct DatabaseBackend;

impl DatabaseBackend {
    pub fn new() -> Self {
        DatabaseBackend
    }

    fn get_hasher(&self) -> PasswordHasher {
        let mut hasher_lock = HASHER.lock().unwrap();
        if hasher_lock.is_none() {
            // We don't want to use DES based crypt(), since it doesn't return a hash with a recognisable prefix
            let force_portable = !PasswordHasher::has_blowfish_support();
            *hasher_lock = Some(PasswordHasher::new(8, force_portable));
        }
        hasher_lock.as_ref().unwrap().clone()
    }
}

impl UserBackend for DatabaseBackend {
    /// Create a new user
    ///
    /// Creates a new user. Basic checking of username is done in User
    /// itself, not in its subclasses.
    ///
    /// # Arguments
    ///
    /// * `uid` - The username of the user to create
    /// * `password` - The password of the new user
    ///
    /// # Returns
    ///
    /// `true` if user was created successfully, `false` otherwise
    fn create_user(&self, uid: &str, password: &str) -> Result<bool, DBError> {
        if self.user_exists(uid)? {
            Ok(false)
        } else {
            let hasher = self.get_hasher();
            let salt = Config::get_value("passwordsalt", "").unwrap_or_default();
            let hash = hasher.hash_password(&format!("{}{}", password, salt));
            
            let query = DB::prepare("INSERT INTO `*PREFIX*users` ( `uid`, `password` ) VALUES( ?, ? )")?;
            let result = query.execute(&[uid, &hash])?;
            
            Ok(result.is_ok())
        }
    }

    /// Delete a user
    ///
    /// # Arguments
    ///
    /// * `uid` - The username of the user to delete
    ///
    /// # Returns
    ///
    /// `true` if user was deleted successfully, `false` otherwise
    fn delete_user(&self, uid: &str) -> Result<bool, DBError> {
        let query = DB::prepare("DELETE FROM `*PREFIX*users` WHERE `uid` = ?")?;
        query.execute(&[uid])?;
        Ok(true)
    }

    /// Set password
    ///
    /// Change the password of a user
    ///
    /// # Arguments
    ///
    /// * `uid` - The username
    /// * `password` - The new password
    ///
    /// # Returns
    ///
    /// `true` if password was changed successfully, `false` otherwise
    fn set_password(&self, uid: &str, password: &str) -> Result<bool, DBError> {
        if self.user_exists(uid)? {
            let hasher = self.get_hasher();
            let salt = Config::get_value("passwordsalt", "").unwrap_or_default();
            let hash = hasher.hash_password(&format!("{}{}", password, salt));
            
            let query = DB::prepare("UPDATE `*PREFIX*users` SET `password` = ? WHERE `uid` = ?")?;
            query.execute(&[&hash, uid])?;
            
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Set display name
    ///
    /// Change the display name of a user
    ///
    /// # Arguments
    ///
    /// * `uid` - The username
    /// * `display_name` - The new display name
    ///
    /// # Returns
    ///
    /// `true` if display name was changed successfully, `false` otherwise
    fn set_display_name(&self, uid: &str, display_name: &str) -> Result<bool, DBError> {
        if self.user_exists(uid)? {
            let query = DB::prepare("UPDATE `*PREFIX*users` SET `displayname` = ? WHERE `uid` = ?")?;
            query.execute(&[display_name, uid])?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get display name of the user
    ///
    /// # Arguments
    ///
    /// * `uid` - user ID of the user
    ///
    /// # Returns
    ///
    /// Display name of the user or None if user doesn't exist
    fn get_display_name(&self, uid: &str) -> Result<Option<String>, DBError> {
        if self.user_exists(uid)? {
            let query = DB::prepare("SELECT `displayname` FROM `*PREFIX*users` WHERE `uid` = ?")?;
            let result = query.execute(&[uid])?;
            let rows = result.fetch_all()?;
            
            if let Some(row) = rows.first() {
                let display_name = row.get::<String>("displayname").trim().to_string();
                if !display_name.is_empty() {
                    Ok(Some(display_name))
                } else {
                    Ok(Some(uid.to_string()))
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Get a list of all display names
    ///
    /// Get a list of all display names and user ids.
    ///
    /// # Arguments
    ///
    /// * `search` - Optional search string to filter users
    /// * `limit` - Optional limit for number of results
    /// * `offset` - Optional offset for pagination
    ///
    /// # Returns
    ///
    /// HashMap with all displayNames (value) and the corresponding uids (key)
    fn get_display_names(&self, search: &str, limit: Option<usize>, offset: Option<usize>) -> Result<std::collections::HashMap<String, String>, DBError> {
        let mut display_names = std::collections::HashMap::new();
        
        let query = DB::prepare_with_limit_offset(
            "SELECT `uid`, `displayname` FROM `*PREFIX*users` WHERE LOWER(`displayname`) LIKE LOWER(?) OR LOWER(`uid`) LIKE LOWER(?)",
            limit,
            offset
        )?;
        
        let search_pattern = format!("{}%", search);
        let result = query.execute(&[&search_pattern, &search_pattern])?;
        let rows = result.fetch_all()?;
        
        for row in rows {
            let uid = row.get::<String>("uid");
            let display_name = row.get::<String>("displayname");
            display_names.insert(uid, display_name);
        }
        
        Ok(display_names)
    }

    /// Check if the password is correct
    ///
    /// Check if the password is correct without logging in the user
    ///
    /// # Arguments
    ///
    /// * `uid` - The username
    /// * `password` - The password
    ///
    /// # Returns
    ///
    /// Some(user_id) if password is correct, None otherwise
    fn check_password(&self, uid: &str, password: &str) -> Result<Option<String>, DBError> {
        let query = DB::prepare("SELECT `uid`, `password` FROM `*PREFIX*users` WHERE LOWER(`uid`) = LOWER(?)")?;
        let result = query.execute(&[uid])?;
        let rows = result.fetch_all()?;
        
        if let Some(row) = rows.first() {
            let stored_hash = row.get::<String>("password");
            let uid = row.get::<String>("uid");
            
            if stored_hash.starts_with('$') {
                // The new phpass based hashing
                let hasher = self.get_hasher();
                let salt = Config::get_value("passwordsalt", "").unwrap_or_default();
                
                if hasher.check_password(&format!("{}{}", password, salt), &stored_hash) {
                    return Ok(Some(uid));
                }
            } else {
                // Old sha1 based hashing
                use sha1::{Sha1, Digest};
                let mut hasher = Sha1::new();
                hasher.update(password.as_bytes());
                let result = format!("{:x}", hasher.finalize());
                
                if result == stored_hash {
                    // Upgrade to new hashing
                    self.set_password(&uid, password)?;
                    return Ok(Some(uid));
                }
            }
        }
        
        Ok(None)
    }

    /// Get a list of all users
    ///
    /// # Arguments
    ///
    /// * `search` - Optional search string to filter users
    /// * `limit` - Optional limit for number of results
    /// * `offset` - Optional offset for pagination
    ///
    /// # Returns
    ///
    /// Vector with all user IDs
    fn get_users(&self, search: &str, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<String>, DBError> {
        let query = DB::prepare_with_limit_offset(
            "SELECT `uid` FROM `*PREFIX*users` WHERE LOWER(`uid`) LIKE LOWER(?)",
            limit,
            offset
        )?;
        
        let search_pattern = format!("{}%", search);
        let result = query.execute(&[&search_pattern])?;
        let rows = result.fetch_all()?;
        
        let mut users = Vec::new();
        for row in rows {
            users.push(row.get::<String>("uid"));
        }
        
        Ok(users)
    }

    /// Check if a user exists
    ///
    /// # Arguments
    ///
    /// * `uid` - The username
    ///
    /// # Returns
    ///
    /// `true` if user exists, `false` otherwise
    fn user_exists(&self, uid: &str) -> Result<bool, DBError> {
        let query = DB::prepare("SELECT COUNT(*) as count FROM `*PREFIX*users` WHERE LOWER(`uid`) = LOWER(?)")?;
        let result = query.execute(&[uid])?;
        
        match result.fetch_all() {
            Ok(rows) => {
                if let Some(row) = rows.first() {
                    let count: i64 = row.get("count");
                    Ok(count > 0)
                } else {
                    Ok(false)
                }
            },
            Err(err) => {
                log::error("core", &format!("Database error: {}", err.to_string()));
                Ok(false)
            }
        }
    }

    /// Get the user's home directory
    ///
    /// # Arguments
    ///
    /// * `uid` - The username
    ///
    /// # Returns
    ///
    /// Path to the user's home directory or None if user doesn't exist
    fn get_home(&self, uid: &str) -> Result<Option<PathBuf>, DBError> {
        if self.user_exists(uid)? {
            let data_dir = Config::get_value("datadirectory", ServerRoot::get().join("data"))
                .unwrap_or_else(|| ServerRoot::get().join("data"));
            
            Ok(Some(PathBuf::from(data_dir).join(uid)))
        } else {
            Ok(None)
        }
    }

    /// Check if this backend provides user listings
    ///
    /// # Returns
    ///
    /// `true` if this backend provides user listings
    fn has_user_listings(&self) -> bool {
        true
    }
}