//! LDAP Connection module for ownCloud
//!
//! Copyright 2012, 2013 Arthur Schiwon blizzz@owncloud.com
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

use std::collections::HashMap;
use std::env;
use std::fmt;
use std::sync::{Arc, Mutex};

// Using log crate for logging
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

// Traits
pub trait LDAPWrapper: Send + Sync {
    fn connect(&self, host: &str, port: u16) -> Result<LDAPResource, LDAPError>;
    fn bind(&self, resource: &LDAPResource, agent_name: &str, agent_password: &str) -> Result<bool, LDAPError>;
    fn unbind(&self, resource: &LDAPResource) -> Result<(), LDAPError>;
    fn start_tls(&self, resource: &LDAPResource) -> Result<(), LDAPError>;
    fn set_option(&self, resource: &LDAPResource, option: LDAPOption, value: LDAPOptionValue) -> Result<bool, LDAPError>;
    fn is_resource(&self, resource: Option<&LDAPResource>) -> bool;
    fn errno(&self, resource: &LDAPResource) -> i32;
    fn error(&self, resource: &LDAPResource) -> String;
    fn has_paged_result_support(&self) -> bool;
}

pub trait Cache: Send + Sync {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, value: &str, ttl: u32) -> bool;
    fn has_key(&self, key: &str) -> bool;
    fn clear(&self, prefix: &str);
}

#[derive(Debug)]
pub struct LDAPResource {
    id: usize,
}

#[derive(Debug)]
pub enum LDAPError {
    ConnectionFailed(String),
    BindFailed(String),
    Other(String),
}

impl fmt::Display for LDAPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LDAPError::ConnectionFailed(msg) => write!(f, "LDAP connection failed: {}", msg),
            LDAPError::BindFailed(msg) => write!(f, "LDAP bind failed: {}", msg),
            LDAPError::Other(msg) => write!(f, "LDAP error: {}", msg),
        }
    }
}

impl std::error::Error for LDAPError {}

pub enum LDAPOption {
    ProtocolVersion,
    Referrals,
}

pub enum LDAPOptionValue {
    Int(i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    config_prefix: String,
    ldap_agent_name: String,
    ldap_agent_password: String,
    ldap_base: Vec<String>,
    ldap_base_users: Vec<String>,
    ldap_base_groups: Vec<String>,
    ldap_configuration_active: bool,
    ldap_host: String,
    ldap_port: u16,
    ldap_backup_host: String,
    ldap_backup_port: u16,
    ldap_override_main_server: bool,
    ldap_user_display_name: String,
    ldap_group_display_name: String,
    ldap_login_filter: String,
    ldap_group_filter: String,
    ldap_uuid_user_attribute: String,
    ldap_uuid_group_attribute: String,
    ldap_expert_uuid_user_attr: String,
    ldap_expert_uuid_group_attr: String,
    ldap_attributes_for_user_search: Vec<String>,
    ldap_attributes_for_group_search: Vec<String>,
    ldap_tls: bool,
    ldap_cache_ttl: u32,
    ldap_turn_off_cert_check: bool,
    home_folder_naming_rule: String,
}

impl Configuration {
    pub fn new(config_prefix: &str) -> Self {
        Configuration {
            config_prefix: config_prefix.to_string(),
            ldap_agent_name: String::new(),
            ldap_agent_password: String::new(),
            ldap_base: Vec::new(),
            ldap_base_users: Vec::new(),
            ldap_base_groups: Vec::new(),
            ldap_configuration_active: false,
            ldap_host: String::new(),
            ldap_port: 389,
            ldap_backup_host: String::new(),
            ldap_backup_port: 389,
            ldap_override_main_server: false,
            ldap_user_display_name: String::new(),
            ldap_group_display_name: String::new(),
            ldap_login_filter: String::new(),
            ldap_group_filter: String::new(),
            ldap_uuid_user_attribute: "auto".to_string(),
            ldap_uuid_group_attribute: "auto".to_string(),
            ldap_expert_uuid_user_attr: String::new(),
            ldap_expert_uuid_group_attr: String::new(),
            ldap_attributes_for_user_search: Vec::new(),
            ldap_attributes_for_group_search: Vec::new(),
            ldap_tls: false,
            ldap_cache_ttl: 600,
            ldap_turn_off_cert_check: false,
            home_folder_naming_rule: String::new(),
        }
    }

    pub fn read_configuration(&mut self) {
        // This would load configuration from a database or config file
        // Simplified for example
    }

    pub fn save_configuration(&self) {
        // This would save configuration to a database or config file
        // Simplified for example
    }

    pub fn set_configuration(&mut self, config: HashMap<String, String>, set_parameters: &mut Vec<String>) -> bool {
        // Process config map to update configuration
        // Simplified for example
        for (key, value) in config {
            set_parameters.push(key.clone());
            // Update corresponding fields based on key
        }
        true
    }

    pub fn get_configuration(&self) -> HashMap<String, String> {
        // Convert configuration to a map
        // Simplified for example
        HashMap::new()
    }

    pub fn get_config_translation_array(&self) -> HashMap<String, String> {
        // Return the mapping between DB keys and config properties
        // Simplified for example
        HashMap::new()
    }
}

pub struct LDAPUtility {
    ldap: Arc<dyn LDAPWrapper>,
}

impl LDAPUtility {
    pub fn new(ldap: Arc<dyn LDAPWrapper>) -> Self {
        LDAPUtility { ldap }
    }
}

pub struct Connection {
    ldap_connection_res: Option<LDAPResource>,
    config_prefix: String,
    config_id: String,
    configured: bool,
    dont_destruct: bool,
    has_paged_result_support: bool,
    cache: Arc<dyn Cache>,
    configuration: Configuration,
    do_not_validate: bool,
    ldap: Arc<dyn LDAPWrapper>,
}

impl Connection {
    /// Constructor
    ///
    /// # Arguments
    ///
    /// * `ldap` - LDAP wrapper implementation
    /// * `config_prefix` - Prefix for the configkey column
    /// * `config_id` - Value for the appid column or null for on-the-fly connections
    pub fn new(
        ldap: Arc<dyn LDAPWrapper>,
        cache: Arc<dyn Cache>,
        config_prefix: &str,
        config_id: &str,
        helper: &dyn Fn() -> Vec<String>,
    ) -> Self {
        let configuration = Configuration::new(config_prefix);
        let has_paged_result_support = ldap.has_paged_result_support();
        let do_not_validate = !helper().contains(&config_prefix.to_string());

        Connection {
            ldap_connection_res: None,
            config_prefix: config_prefix.to_string(),
            config_id: config_id.to_string(),
            configured: false,
            dont_destruct: false,
            has_paged_result_support,
            cache,
            configuration,
            do_not_validate,
            ldap,
        }
    }

    pub fn init(&mut self, force: bool) {
        self.read_configuration(force);
        let _ = self.establish_connection();
    }

    pub fn get_connection_resource(&mut self) -> Option<&LDAPResource> {
        if self.ldap_connection_res.is_none() {
            self.init();
        } else if !self.ldap.is_resource(self.ldap_connection_res.as_ref()) {
            self.ldap_connection_res = None;
            let _ = self.establish_connection();
        }

        if self.ldap_connection_res.is_none() {
            error!("LDAP connection could not be established");
        }

        self.ldap_connection_res.as_ref()
    }

    fn get_cache_key(&self, key: Option<&str>) -> String {
        let prefix = format!("LDAP-{}-{}-", self.config_id, self.config_prefix);
        match key {
            Some(k) => format!("{}{:x}", prefix, md5::compute(k.as_bytes())),
            None => prefix,
        }
    }

    pub fn get_from_cache(&mut self, key: &str) -> Option<String> {
        if !self.configured {
            self.read_configuration(false);
        }
        if self.configuration.ldap_cache_ttl == 0 {
            return None;
        }
        if !self.is_cached(key) {
            return None;
        }

        let cache_key = self.get_cache_key(Some(key));
        self.cache.get(&cache_key).and_then(|val| {
            let decoded = base64::decode(&val).ok()?;
            bincode::deserialize(&decoded).ok()
        })
    }

    pub fn is_cached(&mut self, key: &str) -> bool {
        if !self.configured {
            self.read_configuration(false);
        }
        if self.configuration.ldap_cache_ttl == 0 {
            return false;
        }

        let cache_key = self.get_cache_key(Some(key));
        self.cache.has_key(&cache_key)
    }

    pub fn write_to_cache(&mut self, key: &str, value: bool) {
        if !self.configured {
            self.read_configuration(false);
        }
        if self.configuration.ldap_cache_ttl == 0 || !self.configuration.ldap_configuration_active {
            return;
        }

        let cache_key = self.get_cache_key(Some(key));
        if let Ok(serialized) = bincode::serialize(&value) {
            let encoded = base64::encode(&serialized);
            self.cache.set(&cache_key, &encoded, self.configuration.ldap_cache_ttl);
        }
    }

    pub fn clear_cache(&self) {
        self.cache.clear(&self.get_cache_key(None));
    }

    fn read_configuration(&mut self, force: bool) {
        if ((!self.configured || force) && !self.config_id.is_empty()) {
            self.configuration.read_configuration();
            self.configured = self.validate_configuration();
        }
    }

    pub fn set_configuration(&mut self, config: HashMap<String, String>, set_parameters: &mut Vec<String>) -> bool {
        self.do_not_validate = false;
        self.configuration.set_configuration(config, set_parameters);
        if !set_parameters.is_empty() {
            self.configured = self.validate_configuration();
        }
        self.configured
    }

    pub fn save_configuration(&self) {
        self.configuration.save_configuration();
        self.clear_cache();
    }

    pub fn get_configuration(&mut self) -> HashMap<String, String> {
        self.read_configuration(false);
        let config = self.configuration.get_configuration();
        let cta = self.configuration.get_config_translation_array();
        
        let mut result = HashMap::new();
        for (db_key, config_key) in cta {
            // This is simplified; actual implementation would handle different cases
            if let Some(value) = config.get(&config_key) {
                result.insert(db_key, value.clone());
            }
        }
        result
    }

    fn do_soft_validation(&mut self) {
        // Handle empty base settings
        if self.configuration.ldap_base_users.is_empty() {
            info!("Base tree for Users is empty, using Base DN");
            self.configuration.ldap_base_users = self.configuration.ldap_base.clone();
        }
        
        if self.configuration.ldap_base_groups.is_empty() {
            info!("Base tree for Groups is empty, using Base DN");
            self.configuration.ldap_base_groups = self.configuration.ldap_base.clone();
        }

        // Group filter check
        if self.configuration.ldap_group_filter.is_empty() {
            info!("No group filter is specified, LDAP group feature will not be used.");
        }

        // UUID attribute checks
        if !self.configuration.ldap_expert_uuid_user_attr.is_empty() {
            self.configuration.ldap_uuid_user_attribute = self.configuration.ldap_expert_uuid_user_attr.clone();
        } else {
            let uuid_attributes = vec!["auto", "entryuuid", "nsuniqueid", "objectguid", "guid"];
            if !uuid_attributes.contains(&self.configuration.ldap_uuid_user_attribute.as_str()) && !self.config_id.is_empty() {
                self.configuration.ldap_uuid_user_attribute = "auto".to_string();
                self.configuration.save_configuration();
                info!("Illegal value for the ldapUuidUserAttribute, reset to autodetect.");
            }
        }

        if !self.configuration.ldap_expert_uuid_group_attr.is_empty() {
            self.configuration.ldap_uuid_group_attribute = self.configuration.ldap_expert_uuid_group_attr.clone();
        } else {
            let uuid_attributes = vec!["auto", "entryuuid", "nsuniqueid", "objectguid", "guid"];
            if !uuid_attributes.contains(&self.configuration.ldap_uuid_group_attribute.as_str()) && !self.config_id.is_empty() {
                self.configuration.ldap_uuid_group_attribute = "auto".to_string();
                self.configuration.save_configuration();
                info!("Illegal value for the ldapUuidGroupAttribute, reset to autodetect.");
            }
        }

        // Backup port check
        if self.configuration.ldap_backup_port == 0 {
            self.configuration.ldap_backup_port = self.configuration.ldap_port;
        }

        // Empty search attributes check
        if self.configuration.ldap_attributes_for_user_search.len() == 1 
           && self.configuration.ldap_attributes_for_user_search[0].is_empty() {
            self.configuration.ldap_attributes_for_user_search = Vec::new();
        }
        
        if self.configuration.ldap_attributes_for_group_search.len() == 1 
           && self.configuration.ldap_attributes_for_group_search[0].is_empty() {
            self.configuration.ldap_attributes_for_group_search = Vec::new();
        }

        // LDAPS and TLS check
        if self.configuration.ldap_host.to_lowercase().starts_with("ldaps://") && self.configuration.ldap_tls {
            self.configuration.ldap_tls = false;
            info!("LDAPS (already using secure connection) and TLS do not work together. Switched off TLS.");
        }
    }

    fn do_critical_validation(&self) -> bool {
        let mut configuration_ok = true;
        let error_str = format!("Configuration Error (prefix {}): ", self.config_prefix);

        // Options that shall not be empty
        let options = vec![
            ("ldap_host", "LDAP Host"),
            ("ldap_port", "LDAP Port"),
            ("ldap_user_display_name", "LDAP User Display Name"),
            ("ldap_group_display_name", "LDAP Group Display Name"),
            ("ldap_login_filter", "LDAP Login Filter"),
        ];

        for (key, subj) in options {
            let is_empty = match key {
                "ldap_host" => self.configuration.ldap_host.is_empty(),
                "ldap_port" => self.configuration.ldap_port == 0,
                "ldap_user_display_name" => self.configuration.ldap_user_display_name.is_empty(),
                "ldap_group_display_name" => self.configuration.ldap_group_display_name.is_empty(),
                "ldap_login_filter" => self.configuration.ldap_login_filter.is_empty(),
                _ => false,
            };

            if is_empty {
                configuration_ok = false;
                warn!("{}No {} given!", error_str, subj);
            }
        }

        // Agent name/password combination check
        let agent = &self.configuration.ldap_agent_name;
        let pwd = &self.configuration.ldap_agent_password;
        if (agent.is_empty() && !pwd.is_empty()) || (!agent.is_empty() && pwd.is_empty()) {
            warn!("{}either no password is given for the user agent or a password is given, but not an LDAP agent.", error_str);
            configuration_ok = false;
        }

        // Base DN check
        let base = &self.configuration.ldap_base;
        let base_users = &self.configuration.ldap_base_users;
        let base_groups = &self.configuration.ldap_base_groups;
        if base.is_empty() && base_users.is_empty() && base_groups.is_empty() {
            warn!("{}Not a single Base DN given.", error_str);
            configuration_ok = false;
        }

        // Login filter check
        if !self.configuration.ldap_login_filter.contains("%uid") {
            warn!("{}login filter does not contain %uid place holder.", error_str);
            configuration_ok = false;
        }

        configuration_ok
    }

    fn validate_configuration(&mut self) -> bool {
        if self.do_not_validate {
            // Don't validate if it's a new configuration with default values
            return false;
        }

        // First step: "soft" checks
        self.do_soft_validation();

        // Second step: critical checks
        self.do_critical_validation()
    }

    fn establish_connection(&mut self) -> Result<(), LDAPError> {
        if !self.configuration.ldap_configuration_active {
            return Ok(());
        }

        if !self.configured {
            warn!("Configuration is invalid, cannot connect");
            return Err(LDAPError::Other("Configuration is invalid".to_string()));
        }

        if !self.configuration.ldap_override_main_server && self.get_from_cache("overrideMainServer").is_none() {
            match self.do_connect(&self.configuration.ldap_host, self.configuration.ldap_port) {
                Ok(_) => {
                    let bind_status = self.bind().is_ok();
                    let error = match &self.ldap_connection_res {
                        Some(res) if self.ldap.is_resource(Some(res)) => self.ldap.errno(res),
                        _ => -1,
                    };
                    
                    if !bind_status && error != 0 {
                        // Try backup server
                        self.try_backup_server(error)?;
                    }
                },
                Err(_) => {
                    // Try backup server
                    self.try_backup_server(-1)?;
                }
            }
        } else {
            // Override main server, use backup directly
            self.try_backup_server(-1)?;
        }

        Ok(())
    }

    fn try_backup_server(&mut self, error: i32) -> Result<(), LDAPError> {
        self.do_connect(&self.configuration.ldap_backup_host, self.configuration.ldap_backup_port)?;
        let bind_status = self.bind().is_ok();
        
        if bind_status && error == -1 {
            // When bind to backup server succeeded and failed to main server,
            // skip contacting the main server until next cache refresh
            self.write_to_cache("overrideMainServer", true);
        }
        
        if !bind_status {
            return Err(LDAPError::BindFailed("Failed to bind to backup server".to_string()));
        }
        
        Ok(())
    }

    fn do_connect(&mut self, host: &str, port: u16) -> Result<(), LDAPError> {
        if host.is_empty() {
            return Err(LDAPError::ConnectionFailed("Empty host".to_string()));
        }

        let host_with_port = if host.contains("://") {
            format!("{}:{}", host, port)
        } else {
            host.to_string()
        };

        match self.ldap.connect(&host_with_port, port) {
            Ok(resource) => {
                self.ldap_connection_res = Some(resource);
                
                // Set LDAP protocol version to 3
                if self.ldap.set_option(
                    self.ldap_connection_res.as_ref().unwrap(),
                    LDAPOption::ProtocolVersion,
                    LDAPOptionValue::Int(3)
                ).is_ok() {
                    // Disable referrals
                    if self.ldap.set_option(
                        self.ldap_connection_res.as_ref().unwrap(),
                        LDAPOption::Referrals,
                        LDAPOptionValue::Int(0)
                    ).is_ok() {
                        // Start TLS if configured
                        if self.configuration.ldap_tls {
                            let _ = self.ldap.start_tls(self.ldap_connection_res.as_ref().unwrap());
                        }
                    }
                }
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn bind(&mut self) -> Result<bool, LDAPError> {
        static mut GET_CONNECTION_RESOURCE_ATTEMPT: bool = false;
        
        if !self.configuration.ldap_configuration_active {
            return Ok(false);
        }
        
        // Safety: this is mimicking the PHP static variable behavior
        unsafe {
            if GET_CONNECTION_RESOURCE_ATTEMPT {
                GET_CONNECTION_RESOURCE_ATTEMPT = false;
                return Ok(false);
            }
            GET_CONNECTION_RESOURCE_ATTEMPT = true;
        }
        
        let cr = self.get_connection_resource();
        
        // Reset the flag
        unsafe {
            GET_CONNECTION_RESOURCE_ATTEMPT = false;
        }
        
        match cr {
            Some(resource) if self.ldap.is_resource(Some(resource)) => {
                match self.ldap.bind(
                    resource,
                    &self.configuration.ldap_agent_name,
                    &self.configuration.ldap_agent_password
                ) {
                    Ok(true) => Ok(true),
                    Ok(false) | Err(_) => {
                        error!(
                            "Bind failed: {}: {}", 
                            self.ldap.errno(resource),
                            self.ldap.error(resource)
                        );
                        self.ldap_connection_res = None;
                        Ok(false)
                    }
                }
            },
            _ => Ok(false)
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        if !self.dont_destruct {
            if let Some(ref res) = self.ldap_connection_res {
                if self.ldap.is_resource(Some(res)) {
                    let _ = self.ldap.unbind(res);
                }
            }
        }
    }
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        let mut cloned = Connection {
            ldap_connection_res: self.ldap_connection_res.clone(),
            config_prefix: self.config_prefix.clone(),
            config_id: self.config_id.clone(),
            configured: self.configured,
            dont_destruct: true, // A cloned instance should not unbind
            has_paged_result_support: self.has_paged_result_support,
            cache: self.cache.clone(),
            configuration: self.configuration.clone(),
            do_not_validate: self.do_not_validate,
            ldap: self.ldap.clone(),
        };
        
        cloned
    }
}