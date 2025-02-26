//! ownCloud – LDAP Background Jobs
//!
//! @author Arthur Schiwon
//! @copyright 2012 Arthur Schiwon blizzz@owncloud.com
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

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, Once};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::helper::Helper;
use crate::ldap::LDAP;
use crate::connection::Connection;
use crate::access::Access;
use crate::group_ldap::GroupLDAP;
use crate::group_proxy::GroupProxy;
use crate::util::{Config, Util, DB};

type Users = Vec<String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GroupData {
    owncloud_name: String,
    owncloud_users: String,
}

pub struct Jobs {
    interval: u64,
}

/// Static variables using lazy initialization
lazy_static! {
    static ref GROUPS_FROM_DB: Mutex<Option<HashMap<String, GroupData>>> = Mutex::new(None);
    static ref GROUP_BE: Mutex<Option<Arc<dyn GroupBackend>>> = Mutex::new(None);
}

#[async_trait]
pub trait GroupBackend: Send + Sync {
    async fn get_groups(&self) -> Result<Vec<String>, String>;
    async fn users_in_group(&self, group: &str) -> Result<Users, String>;
}

#[async_trait]
pub trait TimedJob {
    fn get_interval(&self) -> u64;
    async fn run(&self, argument: Option<String>) -> Result<(), String>;
}

#[async_trait]
impl TimedJob for Jobs {
    fn get_interval(&self) -> u64 {
        self.interval
    }

    async fn run(&self, _argument: Option<String>) -> Result<(), String> {
        Jobs::update_groups().await
    }
}

impl Jobs {
    pub fn new() -> Self {
        Jobs {
            interval: Self::get_refresh_interval(),
        }
    }

    fn get_refresh_interval() -> u64 {
        // defaults to every hour
        Config::get_app_value("user_ldap", "bgjRefreshInterval").unwrap_or(3600)
    }

    pub async fn update_groups() -> Result<(), String> {
        Util::write_log("user_ldap", "Run background job \"updateGroups\"", "DEBUG");

        let known_groups = Self::get_known_groups().await?
            .keys()
            .cloned()
            .collect::<HashSet<_>>();
        
        let group_be = Self::get_group_be().await?;
        let actual_groups = group_be.get_groups().await?
            .into_iter()
            .collect::<HashSet<_>>();

        if actual_groups.is_empty() && known_groups.is_empty() {
            Util::write_log("user_ldap", 
                "bgJ \"updateGroups\" – groups do not seem to be configured properly, aborting.",
                "INFO");
            return Ok(());
        }

        let intersection: Vec<String> = actual_groups.intersection(&known_groups)
            .cloned()
            .collect();
        Self::handle_known_groups(&intersection).await?;

        let created_groups: Vec<String> = actual_groups.difference(&known_groups)
            .cloned()
            .collect();
        Self::handle_created_groups(&created_groups).await?;

        let removed_groups: Vec<String> = known_groups.difference(&actual_groups)
            .cloned()
            .collect();
        Self::handle_removed_groups(&removed_groups).await?;

        Util::write_log("user_ldap", "bgJ \"updateGroups\" – Finished.", "DEBUG");
        Ok(())
    }

    async fn handle_known_groups(groups: &[String]) -> Result<(), String> {
        Util::write_log("user_ldap", "bgJ \"updateGroups\" – Dealing with known Groups.", "DEBUG");
        
        let groups_from_db = Self::get_known_groups().await?;
        let group_be = Self::get_group_be().await?;
        
        for group in groups {
            // We assume that groups_from_db has been retrieved already
            let known_users: Users = serde_json::from_str(&groups_from_db[group].owncloud_users)
                .map_err(|e| format!("Failed to deserialize users: {}", e))?;
            
            let actual_users = group_be.users_in_group(group).await?;
            let mut has_changed = false;
            
            // Find removed users
            for removed_user in known_users.iter().filter(|u| !actual_users.contains(u)) {
                Util::emit_hook("OC_User", "post_removeFromGroup", HashMap::from([
                    ("uid".to_string(), removed_user.clone()),
                    ("gid".to_string(), group.clone()),
                ]));
                
                Util::write_log("user_ldap",
                    &format!("bgJ \"updateGroups\" – \"{}\" removed from \"{}\".", removed_user, group),
                    "INFO");
                
                has_changed = true;
            }
            
            // Find added users
            for added_user in actual_users.iter().filter(|u| !known_users.contains(u)) {
                Util::emit_hook("OC_User", "post_addToGroup", HashMap::from([
                    ("uid".to_string(), added_user.clone()),
                    ("gid".to_string(), group.clone()),
                ]));
                
                Util::write_log("user_ldap",
                    &format!("bgJ \"updateGroups\" – \"{}\" added to \"{}\".", added_user, group),
                    "INFO");
                
                has_changed = true;
            }
            
            if has_changed {
                let serialized_users = serde_json::to_string(&actual_users)
                    .map_err(|e| format!("Failed to serialize users: {}", e))?;
                
                DB::update("ldap_group_members")
                    .set("owncloudusers", &serialized_users)
                    .where_clause("owncloudname", "=", group)
                    .execute()
                    .await
                    .map_err(|e| format!("Database error: {}", e))?;
            }
        }
        
        Util::write_log("user_ldap", 
            "bgJ \"updateGroups\" – FINISHED dealing with known Groups.",
            "DEBUG");
        
        Ok(())
    }

    async fn handle_created_groups(created_groups: &[String]) -> Result<(), String> {
        Util::write_log("user_ldap", "bgJ \"updateGroups\" – dealing with created Groups.", "DEBUG");
        
        let group_be = Self::get_group_be().await?;
        
        for created_group in created_groups {
            Util::write_log("user_ldap",
                &format!("bgJ \"updateGroups\" – new group \"{}\" found.", created_group),
                "INFO");
            
            let users = group_be.users_in_group(created_group).await?;
            let serialized_users = serde_json::to_string(&users)
                .map_err(|e| format!("Failed to serialize users: {}", e))?;
            
            DB::insert("ldap_group_members")
                .columns(&["owncloudname", "owncloudusers"])
                .values(&[created_group.clone(), serialized_users])
                .execute()
                .await
                .map_err(|e| format!("Database error: {}", e))?;
        }
        
        Util::write_log("user_ldap",
            "bgJ \"updateGroups\" – FINISHED dealing with created Groups.",
            "DEBUG");
        
        Ok(())
    }

    async fn handle_removed_groups(removed_groups: &[String]) -> Result<(), String> {
        Util::write_log("user_ldap", "bgJ \"updateGroups\" – dealing with removed groups.", "DEBUG");
        
        for removed_group in removed_groups {
            Util::write_log("user_ldap",
                &format!("bgJ \"updateGroups\" – group \"{}\" was removed.", removed_group),
                "INFO");
            
            DB::delete("ldap_group_members")
                .where_clause("owncloudname", "=", removed_group)
                .execute()
                .await
                .map_err(|e| format!("Database error: {}", e))?;
        }
        
        Util::write_log("user_ldap",
            "bgJ \"updateGroups\" – FINISHED dealing with removed groups.",
            "DEBUG");
        
        Ok(())
    }

    async fn get_group_be() -> Result<Arc<dyn GroupBackend>, String> {
        let mut group_be_lock = GROUP_BE.lock().map_err(|_| "Failed to acquire lock")?;
        
        if let Some(group_be) = group_be_lock.as_ref() {
            return Ok(group_be.clone());
        }
        
        let config_prefixes = Helper::get_server_configuration_prefixes(true);
        let ldap_wrapper = Arc::new(LDAP::new());
        
        let new_group_be: Arc<dyn GroupBackend> = if config_prefixes.len() == 1 {
            // Avoid the proxy when there is only one LDAP server configured
            let connector = Arc::new(Connection::new(ldap_wrapper.clone(), &config_prefixes[0]));
            let ldap_access = Arc::new(Access::new(connector, ldap_wrapper));
            Arc::new(GroupLDAP::new(ldap_access))
        } else {
            Arc::new(GroupProxy::new(config_prefixes, ldap_wrapper))
        };
        
        *group_be_lock = Some(new_group_be.clone());
        Ok(new_group_be)
    }

    async fn get_known_groups() -> Result<HashMap<String, GroupData>, String> {
        let mut groups_lock = GROUPS_FROM_DB.lock().map_err(|_| "Failed to acquire lock")?;
        
        if let Some(groups) = groups_lock.as_ref() {
            return Ok(groups.clone());
        }
        
        let results = DB::select("owncloudname, owncloudusers")
            .from("ldap_group_members")
            .execute::<GroupData>()
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        
        let mut groups_map = HashMap::new();
        for data in results {
            groups_map.insert(data.owncloud_name.clone(), data);
        }
        
        *groups_lock = Some(groups_map.clone());
        Ok(groups_map)
    }
}