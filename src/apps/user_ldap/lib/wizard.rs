//! ownCloud – LDAP Wizard
//!
//! Author: Arthur Schiwon
//! Copyright 2013 Arthur Schiwon blizzz@owncloud.com
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
use url::Url;
use std::collections::HashSet;
use async_trait::async_trait;
use ldap3::{Ldap, LdapConnAsync, LdapError, SearchEntry, Scope, ResultEntry};
use log::{debug, error};

use crate::helper::Helper;
use crate::ldap_utility::LdapUtility;
use crate::wizard_result::WizardResult;
use crate::configuration::Configuration;
use crate::i_ldap_wrapper::ILdapWrapper;

pub struct Wizard {
    configuration: Configuration,
    ldap: Box<dyn ILdapWrapper>,
    result: WizardResult,
    result_cache: HashMap<String, HashMap<String, Vec<String>>>,
    cr: Option<Ldap>, // Connection resource
}

impl Drop for Wizard {
    fn drop(&mut self) {
        if self.result.has_changes() {
            let _ = self.configuration.save_configuration();
        }
    }
}

const LRESULT_PROCESSED_OK: i32 = 2;
const LRESULT_PROCESSED_INVALID: i32 = 3;
const LRESULT_PROCESSED_SKIP: i32 = 4;

const LFILTER_LOGIN: i32 = 2;
const LFILTER_USER_LIST: i32 = 3;
const LFILTER_GROUP_LIST: i32 = 4;

const LDAP_NW_TIMEOUT: i32 = 4;

impl Wizard {
    /// Constructor
    /// 
    /// # Arguments
    /// * `configuration` - an instance of Configuration
    /// * `ldap` - an instance of ILDAPWrapper
    pub fn new(configuration: Configuration, ldap: Box<dyn ILdapWrapper>) -> Self {
        Wizard {
            configuration,
            ldap,
            result: WizardResult::new(),
            result_cache: HashMap::new(),
            cr: None,
        }
    }

    pub async fn count_groups(&mut self) -> Result<&WizardResult, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_port",
            "ldap_agent_name",
            "ldap_agent_password",
            "ldap_base",
        ]) {
            return Err("Requirements not met".to_string());
        }

        let base = &self.configuration.ldap_base[0];
        let filter = &self.configuration.ldap_group_filter;
        debug!("Wiz: g filter {:?}", filter);

        let l10n = self.get_l10n("user_ldap");
        
        if filter.is_empty() {
            let output = l10n.n_with_vars("%s group found", "%s groups found", 0, &[0.to_string()]);
            self.result.add_change("ldap_group_count", output);
            return Ok(&self.result);
        }

        let cr = self.get_connection().await?;
        
        let search_result = self.ldap.search(cr, base, filter, &["dn"]).await;
        if search_result.is_err() {
            return Err("Search error".to_string());
        }
        
        let entries = self.ldap.count_entries(cr, &search_result.unwrap()).await;
        let entries = entries.unwrap_or(0);
        
        let output = l10n.n_with_vars("%s group found", "%s groups found", entries, &[entries.to_string()]);
        self.result.add_change("ldap_group_count", output);

        Ok(&self.result)
    }

    pub async fn count_users(&mut self) -> Result<&WizardResult, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_port",
            "ldap_agent_name",
            "ldap_agent_password",
            "ldap_base",
            "ldap_user_filter",
        ]) {
            return Err("Requirements not met".to_string());
        }

        let cr = self.get_connection().await?;

        let base = &self.configuration.ldap_base[0];
        let filter = &self.configuration.ldap_user_filter;
        
        let search_result = self.ldap.search(cr, base, filter, &["dn"]).await;
        if search_result.is_err() {
            return Err("Search error".to_string());
        }
        
        let entries = self.ldap.count_entries(cr, &search_result.unwrap()).await;
        let entries = entries.unwrap_or(0);
        
        let l10n = self.get_l10n("user_ldap");
        let output = l10n.n_with_vars("%s user found", "%s users found", entries, &[entries.to_string()]);
        self.result.add_change("ldap_user_count", output);

        Ok(&self.result)
    }

    pub async fn determine_attributes(&mut self) -> Result<&WizardResult, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_port",
            "ldap_agent_name",
            "ldap_agent_password",
            "ldap_base",
            "ldap_user_filter",
        ]) {
            return Err("Requirements not met".to_string());
        }

        let attributes = self.get_user_attributes().await?;

        let mut sorted_attributes = attributes.clone();
        sorted_attributes.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        self.result.add_options("ldap_loginfilter_attributes", sorted_attributes);

        let selected = &self.configuration.ldap_login_filter_attributes;
        if !selected.is_empty() {
            self.result.add_change("ldap_loginfilter_attributes", selected.clone());
        }

        Ok(&self.result)
    }

    /// Detects the available LDAP attributes
    async fn get_user_attributes(&mut self) -> Result<Vec<String>, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_port",
            "ldap_agent_name",
            "ldap_agent_password",
            "ldap_base",
            "ldap_user_filter",
        ]) {
            return Err("Requirements not met".to_string());
        }
        
        let cr = self.get_connection().await?;

        let base = &self.configuration.ldap_base[0];
        let filter = &self.configuration.ldap_user_filter;
        
        let search_result = self.ldap.search(cr, base, filter, &[]).await;
        if search_result.is_err() {
            return Err("Search error".to_string());
        }
        
        let entries = self.ldap.get_entries(cr, &search_result.unwrap()).await?;
        if entries.is_empty() {
            return Err("No entries found".to_string());
        }
        
        let first_entry = &entries[0];
        let mut pure_attributes = Vec::new();
        
        for (attr_name, _) in &first_entry.attrs {
            pure_attributes.push(attr_name.clone());
        }

        Ok(pure_attributes)
    }

    /// Detects the available LDAP groups for groups
    pub async fn determine_groups_for_groups(&mut self) -> Result<&WizardResult, String> {
        self.determine_groups("ldap_groupfilter_groups", "ldap_group_filter_groups", false).await
    }

    /// Detects the available LDAP groups for users
    pub async fn determine_groups_for_users(&mut self) -> Result<&WizardResult, String> {
        self.determine_groups("ldap_userfilter_groups", "ldap_user_filter_groups", true).await
    }

    /// Detects the available LDAP groups
    async fn determine_groups(&mut self, db_key: &str, conf_key: &str, test_member_of: bool) -> Result<&WizardResult, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_port",
            "ldap_agent_name",
            "ldap_agent_password",
            "ldap_base",
        ]) {
            return Err("Requirements not met".to_string());
        }
        
        let cr = self.get_connection().await?;

        let obclasses = vec!["posixGroup".to_string(), "group".to_string(), "*".to_string()];
        self.determine_feature(&obclasses, "cn", db_key, conf_key, false).await?;

        if test_member_of {
            let has_member_of = self.test_member_of().await?;
            self.configuration.has_member_of_filter_support = has_member_of;
            self.result.mark_change();
            
            if !has_member_of {
                return Err("memberOf is not supported by the server".to_string());
            }
        }

        Ok(&self.result)
    }

    pub async fn determine_group_member_assoc(&mut self) -> Result<&WizardResult, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_port",
            "ldap_agent_name",
            "ldap_agent_password",
            "ldap_group_filter",
        ]) {
            return Err("Requirements not met".to_string());
        }
        
        let attribute = self.detect_group_member_assoc().await?;
        self.configuration.set_configuration(&[("ldap_group_member_assoc_attr", attribute)]);
        self.result.mark_change();

        Ok(&self.result)
    }

    /// Detects the available object classes for groups
    pub async fn determine_group_object_classes(&mut self) -> Result<&WizardResult, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_port",
            "ldap_agent_name",
            "ldap_agent_password",
            "ldap_base",
        ]) {
            return Err("Requirements not met".to_string());
        }
        
        let cr = self.get_connection().await?;

        let obclasses = vec!["group".to_string(), "posixGroup".to_string(), "*".to_string()];
        self.determine_feature(&obclasses, "objectclass", "ldap_groupfilter_objectclass", 
                              "ldap_group_filter_objectclass", false).await?;

        Ok(&self.result)
    }

    /// Detects the available object classes for users
    pub async fn determine_user_object_classes(&mut self) -> Result<&WizardResult, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_port",
            "ldap_agent_name",
            "ldap_agent_password",
            "ldap_base",
        ]) {
            return Err("Requirements not met".to_string());
        }
        
        let cr = self.get_connection().await?;

        let obclasses = vec![
            "inetOrgPerson".to_string(), 
            "person".to_string(), 
            "organizationalPerson".to_string(),
            "user".to_string(), 
            "posixAccount".to_string(), 
            "*".to_string()
        ];
        
        let filter = &self.configuration.ldap_user_filter;
        let is_empty_filter = filter.is_empty();
        
        self.determine_feature(&obclasses, "objectclass", "ldap_userfilter_objectclass", 
                              "ldap_user_filter_objectclass", is_empty_filter).await?;

        Ok(&self.result)
    }

    pub async fn get_group_filter(&mut self) -> Result<&WizardResult, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_port",
            "ldap_agent_name",
            "ldap_agent_password",
            "ldap_base",
        ]) {
            return Err("Requirements not met".to_string());
        }
        
        let filter = self.compose_ldap_filter(LFILTER_GROUP_LIST).await?;
        self.apply_find("ldap_group_filter", filter);
        
        Ok(&self.result)
    }

    pub async fn get_user_list_filter(&mut self) -> Result<&WizardResult, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_port",
            "ldap_agent_name",
            "ldap_agent_password",
            "ldap_base",
        ]) {
            return Err("Requirements not met".to_string());
        }
        
        let filter = self.compose_ldap_filter(LFILTER_USER_LIST).await?;
        self.apply_find("ldap_userlist_filter", filter);
        
        Ok(&self.result)
    }

    pub async fn get_user_login_filter(&mut self) -> Result<&WizardResult, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_port",
            "ldap_agent_name",
            "ldap_agent_password",
            "ldap_base",
            "ldap_user_filter",
        ]) {
            return Err("Requirements not met".to_string());
        }
        
        let filter = self.compose_ldap_filter(LFILTER_LOGIN).await?;
        self.apply_find("ldap_login_filter", filter);
        
        Ok(&self.result)
    }

    /// Tries to determine the port, requires given Host, User DN and Password
    pub async fn guess_port_and_tls(&mut self) -> Result<&WizardResult, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_agent_name",
            "ldap_agent_password",
        ]) {
            return Err("Requirements not met".to_string());
        }
        
        self.check_host();
        let port_settings = self.get_port_settings_to_try();

        // Proceed from the best configuration and return on first success
        for setting in port_settings {
            let port = setting.port;
            let tls = setting.tls;
            debug!("Wiz: trying port {}, TLS {}", port, tls);
            
            if let Ok(_) = self.connect_and_bind(port, tls, false).await {
                let config = [
                    ("ldap_port", port.to_string()),
                    ("ldap_tls", (tls as i32).to_string()),
                ];
                self.configuration.set_configuration(&config);
                debug!("Wiz: detected Port {}", port);
                self.result.add_change("ldap_port", port.to_string());
                self.result.add_change("ldap_tls", (tls as i32).to_string());
                return Ok(&self.result);
            }
        }

        // Custom port, undetected (we do not brute force)
        Err("Could not determine port and TLS settings".to_string())
    }

    /// Tries to determine a base dn from User DN or LDAP Host
    pub async fn guess_base_dn(&mut self) -> Result<&WizardResult, String> {
        if !self.check_requirements(&[
            "ldap_host",
            "ldap_agent_name",
            "ldap_agent_password",
            "ldap_port",
        ]) {
            return Err("Requirements not met".to_string());
        }

        // Check whether a DN is given in the agent name (99.9% of all cases)
        let mut base = None;
        let agent_name = &self.configuration.ldap_agent_name;
        if let Some(i) = agent_name.to_lowercase().find("dc=") {
            let potential_base = &agent_name[i..];
            if self.test_base_dn(potential_base).await? {
                self.apply_find("ldap_base", potential_base.to_string());
                return Ok(&self.result);
            }
        }

        // This did not help :(
        // Let's see whether we can parse the Host URL and convert the domain to a base DN
        let domain = Helper::get_domain_from_url(&self.configuration.ldap_host)
            .ok_or_else(|| "Could not determine domain from URL".to_string())?;

        let dparts: Vec<_> = domain.split('.').collect();
        let base2 = dparts.iter()
            .map(|&part| format!("dc={}", part))
            .collect::<Vec<_>>()
            .join(",");
        
        if let Some(ref b) = base {
            if b != &base2 && self.test_base_dn(&base2).await? {
                self.apply_find("ldap_base", base2);
                return Ok(&self.result);
            }
        } else if self.test_base_dn(&base2).await? {
            self.apply_find("ldap_base", base2);
            return Ok(&self.result);
        }

        Err("Could not determine base DN".to_string())
    }

    /// Sets the found value for the configuration key in the WizardResult
    /// as well as in the Configuration instance
    fn apply_find(&mut self, key: &str, value: String) {
        self.result.add_change(key, value.clone());
        self.configuration.set_configuration(&[(key, value)]);
    }

    /// Checks, whether a port was entered in the Host configuration field.
    /// In this case the port will be stripped off, but also stored as setting.
    fn check_host(&mut self) {
        let host = &self.configuration.ldap_host;
        if let Ok(url) = Url::parse(&format!("ldap://{}", host)) {
            if let Some(port) = url.port() {
                let host_without_port = format!("{}:{}", url.host_str().unwrap_or(""), "")
                    .trim_end_matches(':')
                    .to_string();
                
                self.apply_find("ldap_host", host_without_port);
                self.apply_find("ldap_port", port.to_string());
            }
        }
    }

    /// Tries to detect the group member association attribute which is
    /// one of 'uniqueMember', 'memberUid', 'member'
    async fn detect_group_member_assoc(&mut self) -> Result<String, String> {
        let possible_attrs = vec!["uniqueMember", "memberUid", "member", "unfugasdfasdfdfa"];
        let filter = &self.configuration.ldap_group_filter;
        
        if filter.is_empty() {
            return Err("Empty group filter".to_string());
        }
        
        let cr = self.get_connection().await?;
        let base = &self.configuration.ldap_base[0];
        
        let search_result = self.ldap.search(cr, base, filter, &possible_attrs).await;
        if search_result.is_err() {
            return Err("Search error".to_string());
        }
        
        let entries = self.ldap.get_entries(cr, &search_result.unwrap()).await?;
        
        for entry in entries {
            let mut result = HashMap::new();
            for attr in &possible_attrs {
                if let Some(values) = entry.attrs.get(*attr) {
                    result.insert(attr.to_string(), values.len());
                }
            }
            
            if !result.is_empty() {
                let mut result_vec: Vec<_> = result.into_iter().collect();
                result_vec.sort_by(|a, b| a.1.cmp(&b.1));
                return Ok(result_vec[0].0.clone());
            }
        }

        Err("Could not detect group member association attribute".to_string())
    }

    /// Checks whether for a given BaseDN results will be returned
    async fn test_base_dn(&mut self, base: &str) -> Result<bool, String> {
        let cr = self.get_connection().await?;

        // Base is there, let's validate it. If we search for anything, we should
        // get a result set > 0 on a proper base
        let search_result = self.ldap.search(cr, base, "objectClass=*", &["dn"]).await;
        if search_result.is_err() {
            return Ok(false);
        }
        
        let entries = self.ldap.count_entries(cr, &search_result.unwrap()).await;
        Ok(entries.unwrap_or(0) > 0)
    }

    /// Checks whether the server supports memberOf in LDAP Filter.
    /// Requires that groups are determined, thus internally called from within
    /// determine_groups()
    async fn test_member_of(&mut self) -> Result<bool, String> {
        let cr = self.get_connection().await?;
        
        if self.configuration.ldap_base.is_empty() {
            return Ok(false);
        }
        
        let base = &self.configuration.ldap_base[0];
        let filter_prefix = "(&(objectclass=*)(memberOf=";
        let filter_suffix = "))";

        for (dn, properties) in &self.result_cache {
            if !properties.contains_key("cn") {
                // Assuming only groups have their cn cached :)
                continue;
            }
            
            let filter = format!("{}{}{}", filter_prefix, dn, filter_suffix).to_lowercase();
            let search_result = self.ldap.search(cr, base, &filter, &["dn"]).await;
            
            if search_result.is_err() {
                continue;
            }
            
            let entries = self.ldap.count_entries(cr, &search_result.unwrap()).await;
            
            // We do not know which groups are empty, so test any and return
            // success on the first match that returns at least one user
            if entries.unwrap_or(0) > 0 {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Creates an LDAP Filter from given configuration
    async fn compose_ldap_filter(&mut self, filter_type: i32) -> Result<String, String> {
        let mut filter = String::new();
        let mut parts = 0;
        
        match filter_type {
            LFILTER_USER_LIST => {
                let objcs = &self.configuration.ldap_user_filter_objectclass;
                // Glue objectclasses
                if !objcs.is_empty() {
                    filter.push_str("(|");
                    for objc in objcs {
                        filter.push_str(&format!("(objectclass={})", objc));
                    }
                    filter.push_str(")");
                    parts += 1;
                }
                
                // Glue group memberships
                if self.configuration.has_member_of_filter_support {
                    let cns = &self.configuration.ldap_user_filter_groups;
                    if !cns.is_empty() {
                        filter.push_str("(|");
                        let cr = self.get_connection().await?;
                        let base = &self.configuration.ldap_base[0];
                        
                        for cn in cns {
                            let search_filter = format!("cn={}", cn);
                            let search_result = self.ldap.search(cr, base, &search_filter, &["dn"]).await;
                            
                            if search_result.is_err() {
                                continue;
                            }
                            
                            let entries = self.ldap.get_entries(cr, &search_result.unwrap()).await?;
                            
                            if !entries.is_empty() {
                                let dn = &entries[0].dn;
                                filter.push_str(&format!("(memberof={})", dn));
                            }
                        }
                        filter.push_str(")");
                    }
                    parts += 1;
                }
                
                // Wrap parts in AND condition
                if parts > 1 {
                    filter = format!("(&{})", filter);
                }
                
                if filter.is_empty() {
                    filter = "(objectclass=*)".to_string();
                }
            },
            
            LFILTER_GROUP_LIST => {
                let objcs = &self.configuration.ldap_group_filter_objectclass;
                // Glue objectclasses
                if !objcs.is_empty() {
                    filter.push_str("(|");
                    for objc in objcs {
                        filter.push_str(&format!("(objectclass={})", objc));
                    }
                    filter.push_str(")");
                    parts += 1;
                }
                
                // Glue group memberships
                let cns = &self.configuration.ldap_group_filter_groups;
                if !cns.is_empty() {
                    filter.push_str("(|");
                    for cn in cns {
                        filter.push_str(&format!("(cn={})", cn));
                    }
                    filter.push_str(")");
                }
                parts += 1;
                
                // Wrap parts in AND condition
                if parts > 1 {
                    filter = format!("(&{})", filter);
                }
            },
            
            LFILTER_LOGIN => {
                let ulf = &self.configuration.ldap_user_filter;
                let login_part = "=%uid";
                let mut filter_username = String::new();
                let user_attributes = self.get_user_attributes().await?;
                let user_attributes_map: HashMap<_, _> = user_attributes.iter()
                    .map(|s| (s.to_lowercase(), true))
                    .collect();
                
                let mut parts = 0;

                if self.configuration.ldap_login_filter_username == "1" {
                    let attr = if user_attributes_map.contains_key("uid") {
                        "uid"
                    } else if user_attributes_map.contains_key("samaccountname") {
                        "samaccountname"
                    } else if user_attributes_map.contains_key("cn") {
                        // Fallback
                        "cn"
                    } else {
                        ""
                    };
                    
                    if !attr.is_empty() {
                        filter_username = format!("({}{})", attr, login_part);
                        parts += 1;
                    }
                }

                let mut filter_email = String::new();
                if self.configuration.ldap_login_filter_email == "1" {
                    filter_email = format!("(|(mailPrimaryAddress{})(mail{}))", login_part, login_part);
                    parts += 1;
                }

                let mut filter_attributes = String::new();
                let attrs_to_filter = &self.configuration.ldap_login_filter_attributes;
                if !attrs_to_filter.is_empty() {
                    filter_attributes.push_str("(|");
                    for attribute in attrs_to_filter {
                        filter_attributes.push_str(&format!("({}{})", attribute, login_part));
                    }
                    filter_attributes.push_str(")");
                    parts += 1;
                }

                let mut filter_login = String::new();
                if parts > 1 {
                    filter_login.push_str("(|");
                }
                filter_login.push_str(&filter_username);
                filter_login.push_str(&filter_email);
                filter_login.push_str(&filter_attributes);
                if parts > 1 {
                    filter_login.push_str(")");
                }

                filter = format!("(&{}{})", ulf, filter_login);
            },
            
            _ => return Err(format!("Invalid filter type: {}", filter_type)),
        }

        debug!("Wiz: Final filter {}", filter);
        Ok(filter)
    }

    /// Connects and Binds to an LDAP Server
    async fn connect_and_bind(&mut self, port: u16, tls: bool, ncc: bool) -> Result<Ldap, String> {
        if ncc {
            // No certificate check
            env::set_var("LDAPTLS_REQCERT", "never");
        }

        // Connect, does not really trigger any server communication
        debug!("Wiz: Checking Host Info");
        let host = &self.configuration.ldap_host;
        
        let host = if let Ok(url) = Url::parse(host) {
            if url.port().is_some() {
                host.to_string()
            } else {
                format!("{}:{}", host, port)
            }
        } else {
            format!("{}:{}", host, port)
        };
        
        debug!("Wiz: Attempting to connect");
        let conn_result = LdapConnAsync::new(&host).await;
        let (conn, mut ldap) = match conn_result {
            Ok(val) => val,
            Err(_) => return Err("Invalid Host".to_string()),
        };
        
        // Set LDAP options
        debug!("Wiz: Setting LDAP Options");
        ldap.set_timeout(std::time::Duration::from_secs(LDAP_NW_TIMEOUT as u64));
        
        if tls {
            if let Err(_) = ldap.start_tls().await {
                return Err("TLS connection failed".to_string());
            }
        }

        debug!("Wiz: Attempting to Bind");
        // Do the bind!
        let bind_result = ldap.simple_bind(
            &self.configuration.ldap_agent_name,
            &self.configuration.ldap_agent_passwor