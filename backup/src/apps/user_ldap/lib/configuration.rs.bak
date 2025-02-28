/**
 * ownCloud – LDAP Connection
 *
 * @author Arthur Schiwon
 * @copyright 2012, 2013 Arthur Schiwon blizzz@owncloud.com
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

use std::collections::HashMap;
use std::convert::TryFrom;

pub struct Configuration {
    config_prefix: String,
    config_read: bool,
    config: HashMap<String, ConfigValue>,
}

#[derive(Clone, Debug)]
pub enum ConfigValue {
    String(String),
    StringArray(Vec<String>),
    Integer(i64),
    Boolean(bool),
    None,
}

impl ConfigValue {
    pub fn as_string(&self) -> Option<String> {
        match self {
            ConfigValue::String(s) => Some(s.clone()),
            ConfigValue::StringArray(arr) => Some(arr.join("\n")),
            ConfigValue::Integer(i) => Some(i.to_string()),
            ConfigValue::Boolean(b) => Some(b.to_string()),
            ConfigValue::None => None,
        }
    }

    pub fn as_string_array(&self) -> Option<Vec<String>> {
        match self {
            ConfigValue::String(s) => {
                if s.is_empty() {
                    Some(vec![])
                } else {
                    Some(s.split('\n').map(|s| s.to_string()).collect())
                }
            }
            ConfigValue::StringArray(arr) => Some(arr.clone()),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            ConfigValue::Integer(i) => Some(*i),
            ConfigValue::String(s) => s.parse::<i64>().ok(),
            ConfigValue::Boolean(b) => Some(if *b { 1 } else { 0 }),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            ConfigValue::Boolean(b) => Some(*b),
            ConfigValue::Integer(i) => Some(*i != 0),
            ConfigValue::String(s) => {
                if s == "true" || s == "1" {
                    Some(true)
                } else if s == "false" || s == "0" || s.is_empty() {
                    Some(false)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl From<&str> for ConfigValue {
    fn from(s: &str) -> Self {
        ConfigValue::String(s.to_string())
    }
}

impl From<String> for ConfigValue {
    fn from(s: String) -> Self {
        ConfigValue::String(s)
    }
}

impl From<Vec<String>> for ConfigValue {
    fn from(arr: Vec<String>) -> Self {
        ConfigValue::StringArray(arr)
    }
}

impl From<i64> for ConfigValue {
    fn from(i: i64) -> Self {
        ConfigValue::Integer(i)
    }
}

impl From<bool> for ConfigValue {
    fn from(b: bool) -> Self {
        ConfigValue::Boolean(b)
    }
}

pub trait ConfigStorage {
    fn get_app_value(&self, app: &str, key: &str, default: &str) -> String;
    fn set_app_value(&self, app: &str, key: &str, value: &str) -> bool;
    fn get_system_value(&self, key: &str, default: bool) -> bool;
}

struct OcpConfig;

impl ConfigStorage for OcpConfig {
    fn get_app_value(&self, app: &str, key: &str, default: &str) -> String {
        // This would call into the OCP::Config PHP functionality
        // For now, just return the default
        default.to_string()
    }

    fn set_app_value(&self, app: &str, key: &str, value: &str) -> bool {
        // This would call into the OCP::Config PHP functionality
        // For now, just pretend it succeeded
        true
    }

    fn get_system_value(&self, key: &str, default: bool) -> bool {
        // This would call into the OCP::Config PHP functionality
        // For now, just return the default
        default
    }
}

impl Configuration {
    pub fn new(config_prefix: &str, autoread: bool) -> Self {
        let mut config = Self {
            config_prefix: config_prefix.to_string(),
            config_read: false,
            config: HashMap::new(),
        };

        config.init_config();

        if autoread {
            config.read_configuration();
        }

        config
    }

    fn init_config(&mut self) {
        // Initialize with default values
        self.config.insert("ldapHost".to_string(), ConfigValue::None);
        self.config.insert("ldapPort".to_string(), ConfigValue::None);
        self.config.insert("ldapBackupHost".to_string(), ConfigValue::None);
        self.config.insert("ldapBackupPort".to_string(), ConfigValue::None);
        self.config.insert("ldapBase".to_string(), ConfigValue::None);
        self.config.insert("ldapBaseUsers".to_string(), ConfigValue::None);
        self.config.insert("ldapBaseGroups".to_string(), ConfigValue::None);
        self.config.insert("ldapAgentName".to_string(), ConfigValue::None);
        self.config.insert("ldapAgentPassword".to_string(), ConfigValue::None);
        self.config.insert("ldapTLS".to_string(), ConfigValue::None);
        self.config.insert("ldapNoCase".to_string(), ConfigValue::None);
        self.config.insert("turnOffCertCheck".to_string(), ConfigValue::None);
        self.config.insert("ldapIgnoreNamingRules".to_string(), ConfigValue::None);
        self.config.insert("ldapUserDisplayName".to_string(), ConfigValue::None);
        self.config.insert("ldapUserFilterObjectclass".to_string(), ConfigValue::None);
        self.config.insert("ldapUserFilterGroups".to_string(), ConfigValue::None);
        self.config.insert("ldapUserFilter".to_string(), ConfigValue::None);
        self.config.insert("ldapGroupFilter".to_string(), ConfigValue::None);
        self.config.insert("ldapGroupFilterObjectclass".to_string(), ConfigValue::None);
        self.config.insert("ldapGroupFilterGroups".to_string(), ConfigValue::None);
        self.config.insert("ldapGroupDisplayName".to_string(), ConfigValue::None);
        self.config.insert("ldapGroupMemberAssocAttr".to_string(), ConfigValue::None);
        self.config.insert("ldapLoginFilter".to_string(), ConfigValue::None);
        self.config.insert("ldapLoginFilterEmail".to_string(), ConfigValue::None);
        self.config.insert("ldapLoginFilterUsername".to_string(), ConfigValue::None);
        self.config.insert("ldapLoginFilterAttributes".to_string(), ConfigValue::None);
        self.config.insert("ldapQuotaAttribute".to_string(), ConfigValue::None);
        self.config.insert("ldapQuotaDefault".to_string(), ConfigValue::None);
        self.config.insert("ldapEmailAttribute".to_string(), ConfigValue::None);
        self.config.insert("ldapCacheTTL".to_string(), ConfigValue::None);
        self.config.insert("ldapUuidUserAttribute".to_string(), ConfigValue::from("auto"));
        self.config.insert("ldapUuidGroupAttribute".to_string(), ConfigValue::from("auto"));
        self.config.insert("ldapOverrideMainServer".to_string(), ConfigValue::from(false));
        self.config.insert("ldapConfigurationActive".to_string(), ConfigValue::from(false));
        self.config.insert("ldapAttributesForUserSearch".to_string(), ConfigValue::None);
        self.config.insert("ldapAttributesForGroupSearch".to_string(), ConfigValue::None);
        self.config.insert("homeFolderNamingRule".to_string(), ConfigValue::None);
        self.config.insert("hasPagedResultSupport".to_string(), ConfigValue::from(false));
        self.config.insert("hasMemberOfFilterSupport".to_string(), ConfigValue::from(false));
        self.config.insert("ldapExpertUsernameAttr".to_string(), ConfigValue::None);
        self.config.insert("ldapExpertUUIDUserAttr".to_string(), ConfigValue::None);
        self.config.insert("ldapExpertUUIDGroupAttr".to_string(), ConfigValue::None);
    }

    pub fn get(&self, name: &str) -> Option<ConfigValue> {
        self.config.get(name).cloned()
    }

    pub fn set(&mut self, name: &str, value: ConfigValue) {
        let mut config = HashMap::new();
        config.insert(name.to_string(), value);
        self.set_configuration(&config, None);
    }

    pub fn get_configuration(&self) -> &HashMap<String, ConfigValue> {
        &self.config
    }

    /// Set LDAP configuration with values delivered by a HashMap, not read
    /// from configuration. It does not save the configuration! To do so, you
    /// must call save_configuration afterwards.
    pub fn set_configuration(&mut self, config: &HashMap<String, ConfigValue>, applied: Option<&mut Vec<String>>) -> bool {
        if config.is_empty() {
            return false;
        }

        let cta = self.get_config_translation_array();
        
        for (input_key, val) in config {
            let key = if input_key.contains('_') && cta.contains_key(input_key) {
                cta.get(input_key).unwrap().clone()
            } else if self.config.contains_key(input_key) {
                input_key.clone()
            } else {
                continue;
            };

            match key.as_str() {
                "homeFolderNamingRule" => {
                    if let ConfigValue::String(mut s) = val.clone() {
                        if !s.is_empty() && !s.starts_with("attr:") {
                            s = format!("attr:{}", s);
                        }
                        self.set_multi_line(&key, ConfigValue::from(s));
                    } else {
                        self.set_multi_line(&key, val.clone());
                    }
                },
                "ldapBase" | "ldapBaseUsers" | "ldapBaseGroups" | 
                "ldapAttributesForUserSearch" | "ldapAttributesForGroupSearch" |
                "ldapUserFilterObjectclass" | "ldapUserFilterGroups" | 
                "ldapGroupFilterObjectclass" | "ldapGroupFilterGroups" | 
                "ldapLoginFilterAttributes" => {
                    self.set_multi_line(&key, val.clone());
                },
                _ => {
                    self.set_value(&key, val.clone());
                }
            }

            if let Some(applied_vec) = applied {
                applied_vec.push(input_key.clone());
            }
        }

        true
    }

    pub fn read_configuration(&mut self) {
        if !self.config_read && !self.config_prefix.is_empty() {
            let cta = self.get_config_translation_array_flipped();
            let config_storage = OcpConfig;

            for (key, _) in self.config.iter() {
                if !cta.contains_key(key) {
                    // Some are determined
                    continue;
                }

                let db_key = cta.get(key).unwrap();

                match key.as_str() {
                    "ldapBase" | "ldapBaseUsers" | "ldapBaseGroups" |
                    "ldapAttributesForUserSearch" | "ldapAttributesForGroupSearch" |
                    "ldapUserFilterObjectclass" | "ldapUserFilterGroups" |
                    "ldapGroupFilterObjectclass" | "ldapGroupFilterGroups" |
                    "ldapLoginFilterAttributes" => {
                        let value = self.get_multi_line(db_key, &config_storage);
                        self.config.insert(key.clone(), value);
                    },
                    "ldapIgnoreNamingRules" => {
                        let value = self.get_system_value(key, &config_storage);
                        self.config.insert(key.clone(), value);
                    },
                    "ldapAgentPassword" => {
                        let value = self.get_pwd(db_key, &config_storage);
                        self.config.insert(key.clone(), value);
                    },
                    "ldapUserDisplayName" | "ldapGroupDisplayName" => {
                        let value = self.get_lc_value(db_key, &config_storage);
                        self.config.insert(key.clone(), value);
                    },
                    _ => {
                        let value = self.get_value(db_key, &config_storage);
                        self.config.insert(key.clone(), value);
                    }
                }
            }
            self.config_read = true;
        }
    }

    pub fn save_configuration(&self) {
        let cta = self.get_config_translation_array_flipped();
        let config_storage = OcpConfig;

        for (key, value) in &self.config {
            match key.as_str() {
                // Skip these options, as they are not stored but detected
                "ldapIgnoreNamingRules" | "hasPagedResultSupport" |
                "ldapUuidUserAttribute" | "ldapUuidGroupAttribute" => {
                    continue;
                },
                _ => {}
            }

            if let Some(db_key) = cta.get(key) {
                let save_value = match key.as_str() {
                    "ldapAgentPassword" => {
                        match value {
                            ConfigValue::String(s) => {
                                let encoded = base64::encode(s);
                                Some(encoded)
                            },
                            _ => None
                        }
                    },
                    "ldapBase" | "ldapBaseUsers" | "ldapBaseGroups" |
                    "ldapAttributesForUserSearch" | "ldapAttributesForGroupSearch" |
                    "ldapUserFilterObjectclass" | "ldapUserFilterGroups" |
                    "ldapGroupFilterObjectclass" | "ldapGroupFilterGroups" |
                    "ldapLoginFilterAttributes" => {
                        match value {
                            ConfigValue::StringArray(arr) => Some(arr.join("\n")),
                            ConfigValue::String(s) => Some(s.clone()),
                            _ => None
                        }
                    },
                    _ => {
                        value.as_string()
                    }
                };

                if let Some(val) = save_value {
                    self.save_value(db_key, &val, &config_storage);
                } else {
                    self.save_value(db_key, "", &config_storage);
                }
            }
        }
    }

    fn get_multi_line(&self, varname: &str, config_storage: &impl ConfigStorage) -> ConfigValue {
        let value = self.get_value(varname, config_storage);
        
        match value {
            ConfigValue::String(s) => {
                if s.is_empty() {
                    ConfigValue::from(Vec::<String>::new())
                } else {
                    let lines: Vec<String> = s.split('\n').map(|s| s.to_string()).collect();
                    ConfigValue::from(lines)
                }
            },
            _ => ConfigValue::from(Vec::<String>::new())
        }
    }

    fn set_multi_line(&mut self, varname: &str, value: ConfigValue) {
        let processed_value = match value {
            ConfigValue::String(s) => {
                if s.is_empty() {
                    ConfigValue::from("")
                } else {
                    let lines: Vec<String> = s.split('\n').map(|s| s.to_string()).collect();
                    ConfigValue::from(lines)
                }
            },
            ConfigValue::StringArray(_) => value,
            _ => ConfigValue::from("")
        };

        self.set_value(varname, processed_value);
    }

    fn get_pwd(&self, varname: &str, config_storage: &impl ConfigStorage) -> ConfigValue {
        let value = self.get_value(varname, config_storage);
        
        match value {
            ConfigValue::String(s) => {
                if let Ok(decoded) = base64::decode(&s) {
                    if let Ok(s) = String::from_utf8(decoded) {
                        return ConfigValue::from(s);
                    }
                }
                ConfigValue::from("")
            },
            _ => ConfigValue::from("")
        }
    }

    fn get_lc_value(&self, varname: &str, config_storage: &impl ConfigStorage) -> ConfigValue {
        let value = self.get_value(varname, config_storage);
        
        match value {
            ConfigValue::String(s) => {
                ConfigValue::from(s.to_lowercase())
            },
            _ => ConfigValue::from("")
        }
    }

    fn get_system_value(&self, varname: &str, config_storage: &impl ConfigStorage) -> ConfigValue {
        ConfigValue::from(config_storage.get_system_value(varname, false))
    }

    fn get_value(&self, varname: &str, config_storage: &impl ConfigStorage) -> ConfigValue {
        let defaults = self.get_defaults();
        let default_value = defaults.get(varname).unwrap_or(&String::new()).clone();
        
        let value = config_storage.get_app_value(
            "user_ldap",
            &format!("{}{}", self.config_prefix, varname),
            &default_value
        );
        
        ConfigValue::from(value)
    }

    fn set_value(&mut self, varname: &str, value: ConfigValue) {
        self.config.insert(varname.to_string(), value);
    }

    fn save_value(&self, varname: &str, value: &str, config_storage: &impl ConfigStorage) -> bool {
        config_storage.set_app_value(
            "user_ldap",
            &format!("{}{}", self.config_prefix, varname),
            value
        )
    }

    /// Returns an HashMap with the default values. Keys correspond
    /// to config-value entries in the database table
    pub fn get_defaults(&self) -> HashMap<String, String> {
        let mut defaults = HashMap::new();
        
        defaults.insert("ldap_host".to_string(), "".to_string());
        defaults.insert("ldap_port".to_string(), "".to_string());
        defaults.insert("ldap_backup_host".to_string(), "".to_string());
        defaults.insert("ldap_backup_port".to_string(), "".to_string());
        defaults.insert("ldap_override_main_server".to_string(), "".to_string());
        defaults.insert("ldap_dn".to_string(), "".to_string());
        defaults.insert("ldap_agent_password".to_string(), "".to_string());
        defaults.insert("ldap_base".to_string(), "".to_string());
        defaults.insert("ldap_base_users".to_string(), "".to_string());
        defaults.insert("ldap_base_groups".to_string(), "".to_string());
        defaults.insert("ldap_userlist_filter".to_string(), "".to_string());
        defaults.insert("ldap_userfilter_objectclass".to_string(), "".to_string());
        defaults.insert("ldap_userfilter_groups".to_string(), "".to_string());
        defaults.insert("ldap_login_filter".to_string(), "uid=%uid".to_string());
        defaults.insert("ldap_loginfilter_email".to_string(), "0".to_string());
        defaults.insert("ldap_loginfilter_username".to_string(), "1".to_string());
        defaults.insert("ldap_loginfilter_attributes".to_string(), "".to_string());
        defaults.insert("ldap_group_filter".to_string(), "".to_string());
        defaults.insert("ldap_groupfilter_objectclass".to_string(), "".to_string());
        defaults.insert("ldap_groupfilter_groups".to_string(), "".to_string());
        defaults.insert("ldap_display_name".to_string(), "displayName".to_string());
        defaults.insert("ldap_group_display_name".to_string(), "cn".to_string());
        defaults.insert("ldap_tls".to_string(), "1".to_string());
        defaults.insert("ldap_nocase".to_string(), "0".to_string());
        defaults.insert("ldap_quota_def".to_string(), "".to_string());
        defaults.insert("ldap_quota_attr".to_string(), "".to_string());
        defaults.insert("ldap_email_attr".to_string(), "".to_string());
        defaults.insert("ldap_group_member_assoc_attribute".to_string(), "uniqueMember".to_string());
        defaults.insert("ldap_cache_ttl".to_string(), "600".to_string());
        defaults.insert("ldap_uuid_user_attribute".to_string(), "auto".to_string());
        defaults.insert("ldap_uuid_group_attribute".to_string(), "auto".to_string());
        defaults.insert("home_folder_naming_rule".to_string(), "".to_string());
        defaults.insert("ldap_turn_off_cert_check".to_string(), "0".to_string());
        defaults.insert("ldap_configuration_active".to_string(), "0".to_string());
        defaults.insert("ldap_attributes_for_user_search".to_string(), "".to_string());
        defaults.insert("ldap_attributes_for_group_search".to_string(), "".to_string());
        defaults.insert("ldap_expert_username_attr".to_string(), "".to_string());
        defaults.insert("ldap_expert_uuid_user_attr".to_string(), "".to_string());
        defaults.insert("ldap_expert_uuid_group_attr".to_string(), "".to_string());
        defaults.insert("has_memberof_filter_support".to_string(), "0".to_string());
        
        defaults
    }

    /// Returns a HashMap that maps internal variable names to database fields
    fn get_config_translation_array(&self) -> HashMap<String, String> {
        let mut array = HashMap::new();
        
        array.insert("ldap_host".to_string(), "ldapHost".to_string());
        array.insert("ldap_port".to_string(), "ldapPort".to_string());
        array.insert("ldap_backup_host".to_string(), "ldapBackupHost".to_string());
        array.insert("ldap_backup_port".to_string(), "ldapBackupPort".to_string());
        array.insert("ldap_override_main_server".to_string(), "ldapOverrideMainServer".to_string());
        array.insert("ldap_dn".to_string(), "ldapAgentName".to_string());
        array.insert("ldap_agent_password".to_string(), "ldapAgentPassword".to_string());
        array.insert("ldap_base".to_string(), "ldapBase".to_string());
        array.insert("ldap_base_users".to_string(), "ldapBaseUsers".to_string());
        array.insert("ldap_base_groups".to_string(), "ldapBaseGroups".to_string());
        array.insert("ldap_userfilter_objectclass".to_string(), "ldapUserFilterObjectclass".to_string());
        array.insert("ldap_userfilter_groups".to_string(), "ldapUserFilterGroups".to_string());
        array.insert("ldap_userlist_filter".to_string(), "ldapUserFilter".to_string());
        array.insert("ldap_login_filter".to_string(), "ldapLoginFilter".to_string());
        array.insert("ldap_loginfilter_email".to_string(), "ldapLoginFilterEmail".to_string());
        array.insert("ldap_loginfilter_username".to_string(), "ldapLoginFilterUsername".to_string());
        array.insert("ldap_loginfilter_attributes".to_string(), "ldapLoginFilterAttributes".to_string());
        array.insert("ldap_group_filter".to_string(), "ldapGroupFilter".to_string());
        array.insert("ldap_groupfilter_objectclass".to_string(), "ldapGroupFilterObjectclass".to_string());
        array.insert("ldap_groupfilter_groups".to_string(), "ldapGroupFilterGroups".to_string());
        array.insert("ldap_display_name".to_string(), "ldapUserDisplayName".to_string());
        array.insert("ldap_group_display_name".to_string(), "ldapGroupDisplayName".to_string());
        array.insert("ldap_tls".to_string(), "ldapTLS".to_string());
        array.insert("ldap_nocase".to_string(), "ldapNoCase".to_string());
        array.insert("ldap_quota_def".to_string(), "ldapQuotaDefault".to_string());
        array.insert("ldap_quota_attr".to_string(), "ldapQuotaAttribute".to_string());
        array.insert("ldap_email_attr".to_string(), "ldapEmailAttribute".to_string());
        array.insert("ldap_group_member_assoc_attribute".to_string(), "ldapGroupMemberAssocAttr".to_string());
        array.insert("ldap_cache_ttl".to_string(), "ldapCacheTTL".to_string());
        array.insert("home_folder_naming_rule".to_string(), "homeFolderNamingRule".to_string());
        array.insert("ldap_turn_off_cert_check".to_string(), "turnOffCertCheck".to_string());
        array.insert("ldap_configuration_active".to_string(), "ldapConfigurationActive".to_string());
        array.insert("ldap_attributes_for_user_search".to_string(), "ldapAttributesForUserSearch".to_string());
        array.insert("ldap_attributes_for_group_search".to_string(), "ldapAttributesForGroupSearch".to_string());
        array.insert("ldap_expert_username_attr".to_string(), "ldapExpertUsernameAttr".to_string());
        array.insert("ldap_expert_uuid_user_attr".to_string(), "ldapExpertUUIDUserAttr".to_string());
        array.insert("ldap_expert_uuid_group_attr".to_string(), "ldapExpertUUIDGroupAttr".to_string());
        array.insert("has_memberof_filter_support".to_string(), "hasMemberOfFilterSupport".to_string());
        
        array
    }

    /// Returns a HashMap with keys and values swapped compared to get_config_translation_array
    fn get_config_translation_array_flipped(&self) -> HashMap<String, String> {
        let mut flipped = HashMap::new();
        
        for (k, v) in self.get_config_translation_array() {
            flipped.insert(v, k);
        }
        
        flipped
    }
}