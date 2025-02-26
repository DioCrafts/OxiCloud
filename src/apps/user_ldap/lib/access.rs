// Copyright (c) 2012, 2013 Arthur Schiwon blizzz@owncloud.com
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

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use log::{debug, error, info, warn};
use uuid::Uuid;
use rand::Rng;

pub struct Access {
    connection: Arc<Connection>,
    ldap: Arc<dyn ILDAPWrapper>,
    /// Never check this var directly, always use get_paged_search_result_state
    paged_searched_successful: Option<bool>,
    cookies: Mutex<HashMap<String, Option<String>>>,
}

#[async_trait]
impl LDAPUtility for Access {
    // LDAPUtility trait implementation methods would go here
}

impl Access {
    pub fn new(connection: Arc<Connection>, ldap: Arc<dyn ILDAPWrapper>) -> Self {
        Self {
            connection,
            ldap,
            paged_searched_successful: None,
            cookies: Mutex::new(HashMap::new()),
        }
    }

    fn check_connection(&self) -> bool {
        self.connection.is_valid()
    }

    /// Reads an attribute from an LDAP entry or checks if entry exists
    ///
    /// # Arguments
    /// * `dn` - the record in question
    /// * `attr` - the attribute that shall be retrieved, if empty, just check the record's existence
    /// * `filter` - LDAP filter to use
    ///
    /// # Returns
    /// An array of values on success or an empty array if `attr` is empty, None otherwise
    pub async fn read_attribute(&self, dn: &str, attr: &str, filter: &str) -> Option<Vec<String>> {
        if !self.check_connection() {
            warn!("user_ldap: No LDAP Connector assigned, access impossible for readAttribute.");
            return None;
        }

        let cr = self.connection.get_connection_resource().await?;
        if !self.ldap.is_resource(&cr).await {
            // LDAP not available
            debug!("user_ldap: LDAP resource not available.");
            return None;
        }

        // All or nothing! Otherwise we get in trouble with
        let filter = filter.to_string();
        let attr_vec = if attr.is_empty() { Vec::new() } else { vec![attr.to_string()] };
        self.init_paged_search(&filter, &[dn.to_string()], &attr_vec, 99999, 0).await;
        
        let dn_base = self.dn_as_base_parameter(dn);
        let rr = match self.ldap.read(&cr, &dn_base, &filter, &attr_vec).await {
            Ok(r) => r,
            Err(_) => {
                if !attr.is_empty() {
                    // Do not throw this message on userExists check, irritates
                    debug!("user_ldap: readAttribute failed for DN {}", dn);
                }
                return None;
            }
        };

        if attr.is_empty() {
            debug!("user_ldap: readAttribute: {} found", dn);
            return Some(Vec::new());
        }

        let er = self.ldap.first_entry(&cr, &rr).await?;
        
        // LDAP attributes are not case sensitive
        let result = self.ldap.get_attributes(&cr, &er).await
            .map(|attrs| {
                let mut lowercased = HashMap::new();
                for (k, v) in attrs {
                    lowercased.insert(k.to_lowercase(), v);
                }
                lowercased
            })?;
        
        let attr_lower = attr.to_lowercase();
        
        if let Some(values) = result.get(&attr_lower) {
            if !values.is_empty() {
                let mut processed_values = Vec::new();
                for value in values {
                    if self.resembles_dn(&attr_lower) {
                        processed_values.push(self.sanitize_dn(value));
                    } else if attr_lower == "objectguid" || attr_lower == "guid" {
                        processed_values.push(self.convert_object_guid_to_str(value));
                    } else {
                        processed_values.push(value.clone());
                    }
                }
                return Some(processed_values);
            }
        }
        
        debug!("user_ldap: Requested attribute {} not found for {}", attr, dn);
        None
    }

    /// Checks whether the given attribute's value is probably a DN
    fn resembles_dn(&self, attr: &str) -> bool {
        let resembling_attributes = ["dn", "uniquemember", "member"];
        resembling_attributes.contains(&attr)
    }

    /// Sanitizes a DN received from the LDAP server
    fn sanitize_dn(&self, dn: &str) -> String {
        if dn.contains(&[',', '[', ']']) {
            // Treating multiple base DNs
            if let Some(multiple_dns) = dn.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                let dns: Vec<String> = multiple_dns.split(',')
                    .map(|s| self.sanitize_dn(s.trim()))
                    .collect();
                return dns.join(",");
            }
        }

        // OID sometimes gives back DNs with whitespace after the comma
        // a la "uid=foo, cn=bar, dn=..." We need to tackle this!
        let dn = regex::Regex::new(r"([^\\]),(\s+)")
            .unwrap()
            .replace_all(dn, "$1,");

        // Make comparisons and everything work
        let dn = dn.to_lowercase();

        // Escape DN values according to RFC 2253 - this is already done by ldap_explode_dn
        // To use the DN in search filters, \ needs to be escaped to \5c additionally
        // To use them in bases, we convert them back to simple backslashes in read_attribute()
        let replacements = [
            ("\\,", "\\5c2C"),
            ("\\=", "\\5c3D"),
            ("\\+", "\\5c2B"),
            ("\\<", "\\5c3C"),
            ("\\>", "\\5c3E"),
            ("\\;", "\\5c3B"),
            ("\\\"", "\\5c22"),
            ("\\#", "\\5c23"),
            ("(", "\\28"),
            (")", "\\29"),
            ("*", "\\2A"),
        ];

        let mut result = dn.to_string();
        for (from, to) in &replacements {
            result = result.replace(from, to);
        }

        result
    }

    /// Returns the database table for the query
    fn get_map_table(&self, is_user: bool) -> String {
        if is_user {
            "*PREFIX*ldap_user_mapping".to_string()
        } else {
            "*PREFIX*ldap_group_mapping".to_string()
        }
    }

    /// Returns the LDAP DN for the given internal ownCloud name of the group
    pub async fn groupname2dn(&self, name: &str) -> Option<String> {
        self.ocname2dn(name, false).await
    }

    /// Returns the LDAP DN for the given internal ownCloud name of the user
    pub async fn username2dn(&self, name: &str) -> Option<String> {
        self.ocname2dn(name, true).await
    }

    /// Returns the LDAP DN for the given internal ownCloud name
    async fn ocname2dn(&self, name: &str, is_user: bool) -> Option<String> {
        let table = self.get_map_table(is_user);

        let query = format!(
            "SELECT `ldap_dn`
            FROM `{}`
            WHERE `owncloud_name` = ?",
            table
        );

        // This would be implemented with a DB query system
        // For now we'll return a placeholder
        Some("placeholder_dn".to_string())
    }

    /// Returns the internal ownCloud name for the given LDAP DN of the group
    pub async fn dn2groupname(&self, dn: &str, ldapname: Option<&str>) -> Option<String> {
        // To avoid bypassing the base DN settings under certain circumstances
        // with the group support, check whether the provided DN matches one of
        // the given Bases
        if !self.is_dn_part_of_base(dn, &self.connection.ldap_base_groups).await {
            return None;
        }

        self.dn2ocname(dn, ldapname, false).await
    }

    /// Returns the internal ownCloud name for the given LDAP DN of the user
    pub async fn dn2username(&self, dn: &str, ldapname: Option<&str>) -> Option<String> {
        // To avoid bypassing the base DN settings under certain circumstances
        // with the group support, check whether the provided DN matches one of
        // the given Bases
        if !self.is_dn_part_of_base(dn, &self.connection.ldap_base_users).await {
            return None;
        }

        self.dn2ocname(dn, ldapname, true).await
    }

    /// Returns an internal ownCloud name for the given LDAP DN
    async fn dn2ocname(&self, dn: &str, ldapname: Option<&str>, is_user: bool) -> Option<String> {
        let table = self.get_map_table(is_user);
        let find_mapped_name = if is_user {
            self.find_mapped_user(dn).await
        } else {
            self.find_mapped_group(dn).await
        };
        
        let name_attribute = if is_user {
            &self.connection.ldap_user_display_name
        } else {
            &self.connection.ldap_group_display_name
        };

        // Let's try to retrieve the ownCloud name from the mappings table
        if let Some(ocname) = find_mapped_name {
            return Some(ocname);
        }

        // Second try: get the UUID and check if it is known. Then, update the DN and return the name.
        if let Some(uuid) = self.get_uuid(dn, is_user).await {
            let query = format!(
                "SELECT `owncloud_name`
                FROM `{}`
                WHERE `directory_uuid` = ?",
                table
            );

            // This would use a DB query, for now stub implementation
            let component = Some("placeholder_component".to_string());

            if let Some(component) = component {
                let update_query = format!(
                    "UPDATE `{}`
                    SET `ldap_dn` = ?
                    WHERE `directory_uuid` = ?",
                    table
                );
                
                // This would execute the update query
                
                return Some(component);
            }
        } else {
            // If the UUID can't be detected something is foul.
            info!("user_ldap: Cannot determine UUID for {}. Skipping.", dn);
            return None;
        }

        let ldapname = match ldapname {
            Some(name) => name.to_string(),
            None => {
                match self.read_attribute(dn, name_attribute, "objectClass=*").await {
                    Some(values) if !values.is_empty() => values[0].clone(),
                    _ => {
                        info!("user_ldap: No or empty name for {}.", dn);
                        return None;
                    }
                }
            }
        };

        let intname = if is_user {
            let username_attribute = &self.connection.ldap_expert_username_attr;
            if !username_attribute.is_empty() {
                match self.read_attribute(dn, username_attribute, "objectClass=*").await {
                    Some(values) if !values.is_empty() => self.sanitize_username(&values[0]),
                    _ => self.sanitize_username(&uuid.unwrap_or_default()),
                }
            } else {
                self.sanitize_username(&uuid.unwrap_or_default())
            }
        } else {
            ldapname
        };

        // A new user/group! Add it only if it doesn't conflict with other backend's users or existing groups
        // Disabling Cache is required to avoid that the new user is cached as not-existing in fooExists check
        let original_ttl = self.connection.ldap_cache_ttl;
        self.connection.set_configuration(&[("ldapCacheTTL", "0")]).await;
        
        let exists = if is_user {
            self.user_exists(&intname).await
        } else {
            self.group_exists(&intname).await
        };

        if !exists {
            if self.map_component(dn, &intname, is_user).await {
                self.connection.set_configuration(&[("ldapCacheTTL", &original_ttl.to_string())]).await;
                return Some(intname);
            }
        }
        
        self.connection.set_configuration(&[("ldapCacheTTL", &original_ttl.to_string())]).await;

        let altname = self.create_alt_internal_own_cloud_name(&intname, is_user).await;
        if let Some(altname) = altname {
            if self.map_component(dn, &altname, is_user).await {
                return Some(altname);
            }
        }

        // If everything else did not help...
        info!("user_ldap: Could not create unique name for {}.", dn);
        None
    }

    /// Gives back the user names as they are used ownCloud internally
    pub async fn own_cloud_user_names(&self, ldap_users: &[HashMap<String, String>]) -> Vec<String> {
        self.ldap2own_cloud_names(ldap_users, true).await
    }

    /// Gives back the group names as they are used ownCloud internally
    pub async fn own_cloud_group_names(&self, ldap_groups: &[HashMap<String, String>]) -> Vec<String> {
        self.ldap2own_cloud_names(ldap_groups, false).await
    }

    async fn find_mapped_user(&self, dn: &str) -> Option<String> {
        let query = format!(
            "SELECT `owncloud_name`
            FROM `{}`
            WHERE `ldap_dn` = ?",
            self.get_map_table(true)
        );
        
        // This would be implemented with proper DB query execution
        // For now, return None as a placeholder
        None
    }

    async fn find_mapped_group(&self, dn: &str) -> Option<String> {
        let query = format!(
            "SELECT `owncloud_name`
            FROM `{}`
            WHERE `ldap_dn` = ?",
            self.get_map_table(false)
        );
        
        // This would be implemented with proper DB query execution
        // For now, return None as a placeholder
        None
    }

    async fn ldap2own_cloud_names(&self, ldap_objects: &[HashMap<String, String>], is_users: bool) -> Vec<String> {
        let name_attribute = if is_users {
            &self.connection.ldap_user_display_name
        } else {
            &self.connection.ldap_group_display_name
        };
        
        let mut own_cloud_names = Vec::new();

        for ldap_object in ldap_objects {
            if let Some(dn) = ldap_object.get("dn") {
                let name_by_ldap = ldap_object.get(name_attribute).cloned();
                if let Some(ocname) = self.dn2ocname(dn, name_by_ldap.as_deref(), is_users).await {
                    own_cloud_names.push(ocname);
                }
            }
        }
        
        own_cloud_names
    }

    /// Creates a unique name for internal ownCloud use for users
    async fn _create_alt_internal_own_cloud_name_for_users(&self, name: &str) -> Option<String> {
        let mut attempts = 0;
        // While loop is just a precaution. If a name is not generated within
        // 20 attempts, something else is very wrong. Avoids infinite loop.
        while attempts < 20 {
            let alt_name = format!("{}_{}", name, rand::thread_rng().gen_range(1000..10000));
            if !self.user_exists(&alt_name).await {
                return Some(alt_name);
            }
            attempts += 1;
        }
        None
    }

    /// Creates a unique name for internal ownCloud use for groups
    async fn _create_alt_internal_own_cloud_name_for_groups(&self, name: &str) -> Option<String> {
        let query = format!(
            "SELECT `owncloud_name`
            FROM `{}`
            WHERE `owncloud_name` LIKE ?",
            self.get_map_table(false)
        );
        
        // This would execute a proper DB query
        // For now, use placeholder implementation
        let used_names = Vec::<String>::new();
        
        let last_no = if used_names.is_empty() {
            1 // Will become name_2
        } else {
            // This would sort names and extract the last number
            // Placeholder implementation
            let last_name = "name_5"; // Example result
            let last_pos = last_name.rfind('_').unwrap_or(0) + 1;
            last_name[last_pos..].parse::<i32>().unwrap_or(0)
        };
        
        let mut alt_name = format!("{}_{}", name, last_no + 1);
        
        let mut attempts = 1;
        while attempts < 21 {
            // Pro forma check to be really sure it is unique
            // While loop is just a precaution.
            if !self.group_exists(&alt_name).await {
                return Some(alt_name);
            }
            alt_name = format!("{}_{}", name, last_no + attempts);
            attempts += 1;
        }
        
        None
    }

    /// Creates a unique name for internal ownCloud use
    async fn create_alt_internal_own_cloud_name(&self, name: &str, is_user: bool) -> Option<String> {
        let original_ttl = self.connection.ldap_cache_ttl;
        self.connection.set_configuration(&[("ldapCacheTTL", "0")]).await;
        
        let alt_name = if is_user {
            self._create_alt_internal_own_cloud_name_for_users(name).await
        } else {
            self._create_alt_internal_own_cloud_name_for_groups(name).await
        };
        
        self.connection.set_configuration(&[("ldapCacheTTL", &original_ttl.to_string())]).await;
        
        alt_name
    }

    /// Retrieves all known groups from the mappings table
    async fn mapped_groups(&self) -> Vec<(String, String)> {
        self.mapped_components(false).await
    }

    /// Retrieves all known users from the mappings table
    async fn mapped_users(&self) -> Vec<(String, String)> {
        self.mapped_components(true).await
    }

    async fn mapped_components(&self, is_users: bool) -> Vec<(String, String)> {
        let table = self.get_map_table(is_users);
        
        let query = format!(
            "SELECT `ldap_dn`, `owncloud_name`
            FROM `{}`",
            table
        );
        
        // This would execute a DB query and return results
        // For now, return an empty vector as placeholder
        Vec::new()
    }

    /// Inserts a new user or group into the mappings table
    async fn map_component(&self, dn: &str, ocname: &str, is_user: bool) -> bool {
        let table = self.get_map_table(is_user);
        
        let sql_adjustment = match std::env::var("DB_TYPE").unwrap_or_default().as_str() {
            "mysql" => "FROM DUAL",
            _ => "",
        };
        
        let insert = format!(
            "INSERT INTO `{}` (`ldap_dn`, `owncloud_name`, `directory_uuid`)
                SELECT ?,?,?
                {}
                WHERE NOT EXISTS (
                    SELECT 1
                    FROM `{}`
                    WHERE `ldap_dn` = ?
                        OR `owncloud_name` = ?)",
            table, sql_adjustment, table
        );
        
        // This would execute the insert statement with parameters
        // For now, assume success for placeholder implementation
        let uuid = self.get_uuid(dn, is_user).await.unwrap_or_default();
        
        // Return true if insert was successful
        true
    }

    pub async fn fetch_list_of_users(&self, filter: &str, attr: &[String], limit: Option<usize>, offset: Option<usize>) -> Vec<HashMap<String, String>> {
        self.fetch_list(self.search_users(filter, attr, limit, offset).await, attr.len() > 1)
    }

    pub async fn fetch_list_of_groups(&self, filter: &str, attr: &[String], limit: Option<usize>, offset: Option<usize>) -> Vec<HashMap<String, String>> {
        self.fetch_list(self.search_groups(filter, attr, limit, offset).await, attr.len() > 1)
    }

    fn fetch_list(&self, list: Vec<HashMap<String, String>>, many_attributes: bool) -> Vec<HashMap<String, String>> {
        if many_attributes {
            list
        } else {
            // Filter for unique values
            let mut unique_map = HashMap::new();
            for item in list {
                for (k, v) in item {
                    unique_map.entry(v).or_insert(HashMap::new()).insert(k, v.clone());
                }
            }
            unique_map.into_values().collect()
        }
    }

    /// Executes an LDAP search, optimized for Users
    pub async fn search_users(&self, filter: &str, attr: &[String], limit: Option<usize>, offset: Option<usize>) -> Vec<HashMap<String, String>> {
        self.search(filter, &self.connection.ldap_base_users, attr, limit, offset, false).await
    }

    /// Executes an LDAP search, optimized for Groups
    pub async fn search_groups(&self, filter: &str, attr: &[String], limit: Option<usize>, offset: Option<usize>) -> Vec<HashMap<String, String>> {
        self.search(filter, &self.connection.ldap_base_groups, attr, limit, offset, false).await
    }

    /// Executes an LDAP search
    async fn search(
        &self, 
        filter: &str, 
        base: &[String], 
        attr: &[String], 
        limit: Option<usize>, 
        offset: Option<usize>, 
        skip_handling: bool
    ) -> Vec<HashMap<String, String>> {
        // See if we have a resource, in case not cancel with message
        let link_resource = match self.connection.get_connection_resource().await {
            Some(resource) => resource,
            None => {
                // Seems like we didn't find any resource.
                // Return an empty array just like before.
                debug!("user_ldap: Could not search, because resource is missing.");
                return Vec::new();
            }
        };

        // Check whether paged search should be attempted
        let paged_search_ok = self.init_paged_search(filter, base, attr, limit.unwrap_or(0), offset.unwrap_or(0)).await;

        let link_resources = vec![link_resource.clone(); base.len()];
        
        let sr = match self.ldap.search(&link_resources, base, filter, attr).await {
            Ok(results) => results,
            Err(e) => {
                error!(
                    "user_ldap: Error when searching: {} code {}. Paging? {:?}",
                    e.to_string(),
                    e.code(),
                    paged_search_ok
                );
                return Vec::new();
            }
        };

        // Do the server-side sorting
        for sort_attr in attr.iter().rev() {
            for search_resource in &sr {
                self.ldap.sort(&link_resource, search_resource, sort_attr).await;
            }
        }

        let mut findings = Vec::new();
        for res in &sr {
            if let Some(entries) = self.ldap.get_entries(&link_resource, res).await {
                findings.extend(entries);
            }
        }

        if paged_search_ok {
            info!("user_ldap: Paged search successful");
            for (i, res) in sr.iter().enumerate() {
                let mut cookie = None;
                if self.ldap.control_paged_result_response(&link_resource, res, &mut cookie).await {
                    info!("user_ldap: Set paged search cookie");
                    self.set_paged_result_cookie(&base[i], filter, limit.unwrap_or(0), offset.unwrap_or(0), cookie);
                }
            }

            // Browsing through prior pages to get the cookie for the new one
            if skip_handling {
                return Vec::new();
            }
            
            // If count is bigger, then the server does not support
            // paged search. Instead, he did a normal search. We set a
            // flag here, so the callee knows how to deal with it.
            if let Some(limit) = limit {
                if findings.len() <= limit {
                    self.paged_searched_successful = Some(true);
                }
            }
        } else if limit.is_some() {
            info!("user_ldap: Paged search failed :(");
        }

        // Process findings here - filter attributes, handle DNs, etc.
        // Implementation depends on the format of the LDAP response data
        // Placeholder implementation:
        let mut selected_findings = Vec::new();
        
        // Apply offset and limit if needed
        if (!self.get_paged_search_result_state().unwrap_or(false) && paged_search_ok) 
           || (!paged_search_ok && limit.is_some()) {
            let start = offset.unwrap_or(0);
            let end = start + limit.unwrap_or(findings.len());
            if start < findings.len() {
                let end = if end > findings.len() { findings.len() } else { end };
                selected_findings = findings[start..end].to_vec();
            }
        } else {
            selected_findings = findings;
        }
        
        selected_findings
    }

    pub fn sanitize_username(&self, name: &str) -> String {
        if self.connection.ldap_ignore_naming_rules {
            return name.to_string();
        }

        // Transliteration
        // Latin characters to ASCII
        // This is a simplification - in a real implementation we'd use proper transliteration
        let name = name.to_string();
        
        // Replacements
        let name = name.replace(' ', "_");
        
        // Every remaining unallowed characters will be removed
        let re = regex::Regex::new(r"[^a-zA-Z0-9_.@-]").unwrap();
        re.replace_all(&name, "").to_string()
    }

    /// Combines the input filters with AND
    pub fn combine_filter_with_and(&self, filters: &[&str]) -> String {
        self.combine_filter(filters, '&')
    }

    /// Combines the input filters with OR
    pub fn combine_filter_with_or(&self, filters: &[&str]) -> String {
        self.combine_filter(filters, '|')
    }

    /// Combines the input filters with given operator
    fn combine_filter(&self, filters: &[&str], operator: char) -> String {
        let mut combined_filter = format!("({})", operator);
        for filter in filters {
            if !filter.is_empty() {
                if filter.starts_with('(') {
                    combined_filter.push_str(filter);
                } else {
                    combined_filter.push_str(&format!("({})", filter));
                }
            }
        }
        combined_filter.push(')');
        combined_filter
    }

    /// Creates a filter part for to perform search for users
    pub fn get_filter_part_for_user_search(&self, search: &str) -> String {
        self.get_filter_part_for_search(
            search,
            &self.connection.ldap_attributes_for_user_search,
            &self.connection.ldap_user_display_name,
        )
    }

    /// Creates a filter part for to perform search for groups
    pub fn get_filter_part_for_group_search(&self, search: &str) -> String {
        self.get_filter_part_for_search(
            search,
            &self.connection.ldap_attributes_for_group_search,
            &self.connection.ldap_group_display_name,
        )
    }

    /// Creates a filter part for searches
    fn get_filter_part_for_search(
        &self,
        search: &str,
        search_attributes: &[String],
        fallback_attribute: &str,
    ) -> String {
        let mut filter = Vec::new();
        let search = if search.is_empty() { "*" } else { &format!("*{}*", search) };
        
        if search_attributes.is_empty() {
            if fallback_attribute.is_empty() {
                return String::new();
            }
            filter.push(format!("{}={}", fallback_attribute, search));
        } else {
            for attribute in search_attributes {
                filter.push(format!("{}={}", attribute, search));
            }
        }
        
        if filter.len() == 1 {
            format!("({})", filter[0])
        } else {
            self.combine_filter_with_or(&filter.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
        }
    }

    pub async fn are_credentials_valid(&self, name: