// ownCloud – LDAP Helper
//
// @author Arthur Schiwon
// @copyright 2013 Arthur Schiwon blizzz@owncloud.com
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

use crate::core::config;
use crate::core::db;
use crate::core::user;
use std::collections::HashMap;
use url::Url;

pub struct Helper;

impl Helper {
    /// Returns prefixes for each saved LDAP/AD server configuration.
    ///
    /// Configuration prefixes are used to set up configurations for n LDAP or
    /// AD servers. Since configuration is stored in the database, table
    /// appconfig under appid user_ldap, the common identifiers in column
    /// 'configkey' have a prefix. The prefix for the very first server
    /// configuration is empty.
    /// Configkey Examples:
    /// Server 1: ldap_login_filter
    /// Server 2: s1_ldap_login_filter
    /// Server 3: s2_ldap_login_filter
    ///
    /// The prefix needs to be passed to the constructor of Connection class,
    /// except the default (first) server shall be connected to.
    pub async fn get_server_configuration_prefixes(active_configurations: bool) -> Vec<String> {
        let reference_configkey = "ldap_configuration_active";
        
        let mut query = String::from(
            "SELECT DISTINCT `configkey` 
            FROM `*PREFIX*appconfig` 
            WHERE `appid` = 'user_ldap' 
            AND `configkey` LIKE ?"
        );
        
        if active_configurations {
            query.push_str(" AND `configvalue` = '1'");
        }
        
        let params = vec![format!("%{}", reference_configkey)];
        let server_configs = db::prepare_and_execute(&query, &params).await.unwrap_or_default();
        
        let mut prefixes = Vec::new();
        
        for server_config in server_configs {
            if let Some(config_key) = server_config.get("configkey") {
                if let Some(config_key_str) = config_key.as_str() {
                    let len = config_key_str.len() - reference_configkey.len();
                    prefixes.push(config_key_str[0..len].to_string());
                }
            }
        }
        
        prefixes
    }

    /// Determines the host for every configured connection
    /// 
    /// @return a HashMap with configprefix as keys
    pub async fn get_server_configuration_hosts() -> HashMap<String, String> {
        let reference_configkey = "ldap_host";
        
        let query = format!(
            "SELECT DISTINCT `configkey`, `configvalue` 
            FROM `*PREFIX*appconfig` 
            WHERE `appid` = 'user_ldap' 
            AND `configkey` LIKE ?"
        );
        
        let params = vec![format!("%{}", reference_configkey)];
        let config_hosts = db::prepare_and_execute(&query, &params).await.unwrap_or_default();
        
        let mut result = HashMap::new();
        
        for config_host in config_hosts {
            if let (Some(config_key), Some(config_value)) = (
                config_host.get("configkey").and_then(|v| v.as_str()),
                config_host.get("configvalue").and_then(|v| v.as_str())
            ) {
                let len = config_key.len() - reference_configkey.len();
                let prefix = &config_key[0..len];
                result.insert(prefix.to_string(), config_value.to_string());
            }
        }
        
        result
    }

    /// Deletes a given saved LDAP/AD server configuration.
    ///
    /// @param prefix the configuration prefix of the config to delete
    /// @return true on success, false otherwise
    pub async fn delete_server_configuration(prefix: &str) -> bool {
        // Just to be on the safe side
        if !user::check_admin_user().await {
            return false;
        }

        let prefixes = Self::get_server_configuration_prefixes(false).await;
        if !prefixes.contains(&prefix.to_string()) {
            return false;
        }

        let query = 
            "DELETE FROM `*PREFIX*appconfig` 
            WHERE `configkey` LIKE ? 
            AND `appid` = 'user_ldap' 
            AND `configkey` NOT IN ('enabled', 'installed_version', 'types', 'bgjUpdateGroupsLastRun')";
        
        let params = vec![format!("{}%", prefix)];
        
        match db::prepare_and_execute(query, &params).await {
            Ok(result) => {
                let affected_rows = result.affected_rows().unwrap_or(0);
                affected_rows > 0
            },
            Err(_) => false,
        }
    }

    /// Truncate's the given mapping table
    ///
    /// @param mapping either 'user' or 'group'
    /// @return true on success, false otherwise
    pub async fn clear_mapping(mapping: &str) -> bool {
        let table = match mapping {
            "user" => "`*PREFIX*ldap_user_mapping`",
            "group" => "`*PREFIX*ldap_group_mapping`",
            _ => return false,
        };

        let query = if config::get_system_value::<String>("dbtype").await
            .map(|db_type| db_type.contains("sqlite"))
            .unwrap_or(false) 
        {
            format!("DELETE FROM {}", table)
        } else {
            format!("TRUNCATE {}", table)
        };

        match db::execute_query(&query, &[]).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Extracts the domain from a given URL
    ///
    /// @param url the URL
    /// @return Option with domain as string on success, None otherwise
    pub fn get_domain_from_url(url: &str) -> Option<String> {
        let parsed_url = match Url::parse(url) {
            Ok(url) => url,
            Err(_) => return None,
        };

        if let Some(host) = parsed_url.host_str() {
            Some(host.to_string())
        } else if let Some(path) = parsed_url.path().strip_prefix('/') {
            if !path.is_empty() {
                Some(path.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
}