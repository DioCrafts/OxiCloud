// user_ldap.rs

/**
 * ownCloud
 *
 * @author Dominik Schmidt
 * @author Artuhr Schiwon
 * @copyright 2011 Dominik Schmidt dev@dominik-schmidt.de
 * @copyright 2012 Arthur Schiwon blizzz@owncloud.com
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use std::path::Path;
use crate::lib::{ILDAPWrapper, BackendUtility};
use crate::ocp::{Config, Util, UserInterface, UserBackendActions};

pub struct UserLdap {
    access: BackendUtility,
}

impl UserLdap {
    pub fn new(access: BackendUtility) -> Self {
        Self { access }
    }

    fn update_quota(&self, dn: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut quota = None;
        let quota_default = &self.access.connection.ldap_quota_default;
        let quota_attribute = &self.access.connection.ldap_quota_attribute;
        
        if !quota_default.is_empty() {
            quota = Some(quota_default.clone());
        }
        
        if !quota_attribute.is_empty() {
            let a_quota = self.access.read_attribute(dn, quota_attribute)?;
            
            if let Some(values) = a_quota {
                if !values.is_empty() {
                    quota = Some(values[0].clone());
                }
            }
        }
        
        if let Some(quota_val) = quota {
            let username = self.access.dn2username(dn)?;
            Config::set_user_value(
                &username,
                "files",
                "quota",
                &Util::computer_file_size(&quota_val)
            )?;
        }
        
        Ok(())
    }

    fn update_email(&self, dn: &str) -> Result<(), Box<dyn std::error::Error>> {
        let email_attribute = &self.access.connection.ldap_email_attribute;
        
        if !email_attribute.is_empty() {
            let a_email = self.access.read_attribute(dn, email_attribute)?;
            
            if let Some(values) = a_email {
                if !values.is_empty() {
                    let username = self.access.dn2username(dn)?;
                    Config::set_user_value(
                        &username,
                        "settings",
                        "email",
                        &values[0]
                    )?;
                }
            }
        }
        
        Ok(())
    }
}

impl UserInterface for UserLdap {
    /**
     * Check if the password is correct
     * @param uid The username
     * @param password The password
     * @returns true/false
     *
     * Check if the password is correct without logging in the user
     */
    fn check_password(&self, uid: &str, password: &str) -> Option<String> {
        // Find out dn of the user name
        let filter = Util::mb_str_replace(
            "%uid", 
            uid, 
            &self.access.connection.ldap_login_filter, 
            "UTF-8"
        );
        
        let ldap_users = match self.access.fetch_list_of_users(&filter, "dn") {
            Ok(users) => users,
            Err(_) => return None,
        };
        
        if ldap_users.len() < 1 {
            return None;
        }
        
        let dn = &ldap_users[0];
        
        // Do we have a username for him/her?
        let ocname = match self.access.dn2username(dn) {
            Ok(name) => name,
            Err(_) => return None,
        };
        
        if !ocname.is_empty() {
            // Update some settings, if necessary
            let _ = self.update_quota(dn);
            let _ = self.update_email(dn);
            
            // Are the credentials OK?
            match self.access.are_credentials_valid(dn, password) {
                Ok(valid) if valid => Some(ocname),
                _ => None,
            }
        } else {
            None
        }
    }

    /**
     * Get a list of all users
     * @returns array with all uids
     *
     * Get a list of all users.
     */
    fn get_users(&self, search: &str, limit: i32, offset: i32) -> Vec<String> {
        let cache_key = format!("getUsers-{}-{}-{}", search, limit, offset);
        
        // Check if users are cached, if so return
        if let Some(cached_users) = self.access.connection.get_from_cache(&cache_key) {
            return cached_users;
        }
        
        // If we'd pass -1 to LDAP search, we'd end up in a Protocol
        // error. With a limit of 0, we get 0 results. So we pass null.
        let effective_limit = if limit <= 0 { None } else { Some(limit) };
        
        let filter = match self.access.combine_filter_with_and(&[
            &self.access.connection.ldap_user_filter,
            &self.access.get_filter_part_for_user_search(search),
        ]) {
            Ok(f) => f,
            Err(_) => return Vec::new(),
        };
        
        Util::write_log(
            "user_ldap",
            &format!("getUsers: Options: search {} limit {:?} offset {} Filter: {}", 
                    search, effective_limit, offset, filter),
            Util::DEBUG
        );
        
        // Do the search and translate results to owncloud names
        let ldap_users = match self.access.fetch_list_of_users(
            &filter,
            &[&self.access.connection.ldap_user_display_name, "dn"],
            effective_limit,
            Some(offset)
        ) {
            Ok(users) => users,
            Err(_) => return Vec::new(),
        };
        
        let oc_users = match self.access.own_cloud_user_names(&ldap_users) {
            Ok(users) => users,
            Err(_) => return Vec::new(),
        };
        
        Util::write_log(
            "user_ldap", 
            &format!("getUsers: {} Users found", oc_users.len()), 
            Util::DEBUG
        );
        
        let _ = self.access.connection.write_to_cache(&cache_key, &oc_users);
        oc_users
    }

    /**
     * Check if a user exists
     * @param uid the username
     * @return boolean
     */
    fn user_exists(&self, uid: &str) -> bool {
        let cache_key = format!("userExists{}", uid);
        
        if let Some(cached) = self.access.connection.get_from_cache(&cache_key) {
            return cached;
        }
        
        // Getting dn, if false the user does not exist. If dn, he may be mapped only, requires more checking.
        let dn = match self.access.username2dn(uid) {
            Ok(Some(dn)) => dn,
            _ => {
                Util::write_log(
                    "user_ldap", 
                    &format!("No DN found for {} on {}", uid, self.access.connection.ldap_host),
                    Util::DEBUG
                );
                let _ = self.access.connection.write_to_cache(&cache_key, &false);
                return false;
            }
        };
        
        // Check if user really still exists by reading its entry
        match self.access.read_attribute(&dn, "") {
            Ok(Some(_)) => {
                let _ = self.access.connection.write_to_cache(&cache_key, &true);
                let _ = self.update_quota(&dn);
                true
            },
            _ => {
                Util::write_log(
                    "user_ldap", 
                    &format!("LDAP says no user {}", dn),
                    Util::DEBUG
                );
                let _ = self.access.connection.write_to_cache(&cache_key, &false);
                false
            }
        }
    }

    /**
     * Delete a user
     * @param uid The username of the user to delete
     * @returns true/false
     *
     * Deletes a user
     */
    fn delete_user(&self, _uid: &str) -> bool {
        false
    }

    /**
     * Get the user's home directory
     * @param uid the username
     * @return Option<String>
     */
    fn get_home(&self, uid: &str) -> Option<String> {
        // User Exists check required as it is not done in user proxy!
        if !self.user_exists(uid) {
            return None;
        }
        
        let cache_key = format!("getHome{}", uid);
        
        if let Some(cached) = self.access.connection.get_from_cache(&cache_key) {
            return Some(cached);
        }
        
        if self.access.connection.home_folder_naming_rule.starts_with("attr:") {
            let attr = &self.access.connection.home_folder_naming_rule["attr:".len()..];
            
            if let Ok(Some(dn)) = self.access.username2dn(uid) {
                if let Ok(Some(homedir)) = self.access.read_attribute(&dn, attr) {
                    if !homedir.is_empty() {
                        let path = &homedir[0];
                        let homedir = if path.starts_with("/") || 
                                      (path.len() > 3 && 
                                       path.chars().next().unwrap().is_alphabetic() && 
                                       &path[1..2] == ":" && 
                                       (path[2..3] == "\\" || path[2..3] == "/")) {
                            path.clone()
                        } else {
                            let data_dir = Config::get_system_value(
                                "datadirectory",
                                &format!("{}/data", std::env::var("SERVERROOT").unwrap_or_default())
                            );
                            format!("{}/{}", data_dir, path)
                        };
                        
                        let _ = self.access.connection.write_to_cache(&cache_key, &homedir);
                        return Some(homedir);
                    }
                }
            }
        }
        
        // False will apply default behaviour as defined and done by OC_User
        let _ = self.access.connection.write_to_cache(&cache_key, &"".to_string());
        None
    }

    /**
     * Get display name of the user
     * @param uid user ID of the user
     * @return display name
     */
    fn get_display_name(&self, uid: &str) -> Option<String> {
        if !self.user_exists(uid) {
            return None;
        }
        
        let cache_key = format!("getDisplayName{}", uid);
        
        if let Some(display_name) = self.access.connection.get_from_cache(&cache_key) {
            return Some(display_name);
        }
        
        let dn = match self.access.username2dn(uid) {
            Ok(Some(dn)) => dn,
            _ => return None,
        };
        
        let display_name = match self.access.read_attribute(
            &dn,
            &self.access.connection.ldap_user_display_name
        ) {
            Ok(Some(values)) if !values.is_empty() => {
                let _ = self.access.connection.write_to_cache(&cache_key, &values[0]);
                Some(values[0].clone())
            },
            _ => None,
        };
        
        display_name
    }

    /**
     * Get a list of all display names
     * @returns Map with all displayNames (value) and the correspondig uids (key)
     *
     * Get a list of all display names and user ids.
     */
    fn get_display_names(&self, search: &str, limit: Option<i32>, offset: Option<i32>) -> std::collections::HashMap<String, String> {
        let cache_key = format!("getDisplayNames-{}-{:?}-{:?}", search, limit, offset);
        
        if let Some(display_names) = self.access.connection.get_from_cache(&cache_key) {
            return display_names;
        }
        
        let mut display_names = std::collections::HashMap::new();
        let limit_val = limit.unwrap_or(10);
        let offset_val = offset.unwrap_or(0);
        
        let users = self.get_users(search, limit_val, offset_val);
        
        for user in users {
            if let Some(display_name) = self.get_display_name(&user) {
                display_names.insert(user, display_name);
            }
        }
        
        let _ = self.access.connection.write_to_cache(&cache_key, &display_names);
        display_names
    }

    /**
     * Check if backend implements actions
     * @param actions bitwise-or'ed actions
     * @returns boolean
     *
     * Returns the supported actions as int to be
     * compared with OC_USER_BACKEND_CREATE_USER etc.
     */
    fn implements_actions(&self, actions: UserBackendActions) -> bool {
        let supported = UserBackendActions::CHECK_PASSWORD 
            | UserBackendActions::GET_HOME
            | UserBackendActions::GET_DISPLAY_NAME;
        
        supported.contains(actions)
    }

    /**
     * @return bool
     */
    fn has_user_listings(&self) -> bool {
        true
    }
}