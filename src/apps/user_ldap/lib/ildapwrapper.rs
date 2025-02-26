//! ownCloud – LDAP Wrapper Interface
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

#![allow(clippy::borrowed_box)]

use std::collections::HashMap;

/// LDAP resource type
pub type LdapLink = Box<dyn std::any::Any + Send + Sync>;
/// LDAP result type
pub type LdapResult = Box<dyn std::any::Any + Send + Sync>;
/// LDAP entry type
pub type LdapEntry = Box<dyn std::any::Any + Send + Sync>;
/// LDAP attributes map
pub type LdapAttributes = HashMap<String, Vec<String>>;
/// LDAP entries collection
pub type LdapEntries = Vec<LdapAttributes>;
/// LDAP cookie for pagination
pub type LdapCookie = Vec<u8>;

/// Interface for LDAP wrapper implementations
pub trait LdapWrapper {
    /// Bind to LDAP directory
    ///
    /// With `dn` and `password` as None, an anonymous bind is attempted.
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `dn` - an RDN to log in with
    /// * `password` - the password
    ///
    /// # Returns
    /// `Ok(true)` on success, `Ok(false)` or `Err` otherwise
    fn bind(&self, link: &LdapLink, dn: Option<&str>, password: Option<&str>) -> Result<bool, String>;

    /// Connect to an LDAP server
    ///
    /// # Arguments
    /// * `host` - The host to connect to
    /// * `port` - The port to connect to
    ///
    /// # Returns
    /// A link resource on success, error otherwise
    fn connect(&self, host: &str, port: u16) -> Result<LdapLink, String>;

    /// Send LDAP pagination control
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `pagesize` - number of results per page
    /// * `is_critical` - Indicates whether the pagination is critical or not
    /// * `cookie` - structure sent by LDAP server
    ///
    /// # Returns
    /// `Ok(true)` on success, `Ok(false)` or `Err` otherwise
    fn control_paged_result(
        &self,
        link: &LdapLink,
        pagesize: u32,
        is_critical: bool,
        cookie: Option<&LdapCookie>,
    ) -> Result<bool, String>;

    /// Retrieve the LDAP pagination cookie
    ///
    /// Corresponds to ldap_control_paged_result_response
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `result` - LDAP result resource
    ///
    /// # Returns
    /// The cookie on success, error otherwise
    fn control_paged_result_response(
        &self,
        link: &LdapLink,
        result: &LdapResult,
    ) -> Result<LdapCookie, String>;

    /// Count the number of entries in a search
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `result` - LDAP result resource
    ///
    /// # Returns
    /// Number of results on success, error otherwise
    fn count_entries(&self, link: &LdapLink, result: &LdapResult) -> Result<usize, String>;

    /// Return the LDAP error number of the last LDAP command
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    ///
    /// # Returns
    /// Error code as integer
    fn errno(&self, link: &LdapLink) -> i32;

    /// Return the LDAP error message of the last LDAP command
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    ///
    /// # Returns
    /// Error message as string
    fn error(&self, link: &LdapLink) -> String;

    /// Return first result entry
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `result` - LDAP result resource
    ///
    /// # Returns
    /// An LDAP entry result resource, or error
    fn first_entry(&self, link: &LdapLink, result: &LdapResult) -> Result<LdapEntry, String>;

    /// Get attributes from a search result entry
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `entry` - LDAP entry resource
    ///
    /// # Returns
    /// HashMap containing the attributes, or error
    fn get_attributes(&self, link: &LdapLink, entry: &LdapEntry) -> Result<LdapAttributes, String>;

    /// Get the DN of a result entry
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `entry` - LDAP entry resource
    ///
    /// # Returns
    /// String containing the DN, or error
    fn get_dn(&self, link: &LdapLink, entry: &LdapEntry) -> Result<String, String>;

    /// Get all result entries
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `result` - LDAP result resource
    ///
    /// # Returns
    /// Vector containing the entries, or error
    fn get_entries(&self, link: &LdapLink, result: &LdapResult) -> Result<LdapEntries, String>;

    /// Return next result entry
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `entry` - LDAP entry result resource
    ///
    /// # Returns
    /// An LDAP entry resource or None if no more entries, or error
    fn next_entry(
        &self,
        link: &LdapLink,
        entry: &LdapEntry,
    ) -> Result<Option<LdapEntry>, String>;

    /// Read an entry
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `base_dn` - The DN of the entry to read from
    /// * `filter` - An LDAP filter
    /// * `attr` - Vec of the attributes to read
    ///
    /// # Returns
    /// An LDAP search result resource, or error
    fn read(
        &self,
        link: &LdapLink,
        base_dn: &str,
        filter: &str,
        attr: &[String],
    ) -> Result<LdapResult, String>;

    /// Search LDAP tree
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `base_dn` - The DN of the entry to read from
    /// * `filter` - An LDAP filter
    /// * `attr` - Vec of the attributes to read
    ///
    /// # Returns
    /// An LDAP search result resource, or error
    fn search(
        &self,
        link: &LdapLink,
        base_dn: &str,
        filter: &str,
        attr: &[String],
    ) -> Result<LdapResult, String>;

    /// Sets the value of the specified option
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `option` - a defined LDAP Server option
    /// * `value` - the new value for the option
    ///
    /// # Returns
    /// `Ok(true)` on success, `Ok(false)` or `Err` otherwise
    fn set_option<T>(&self, link: &LdapLink, option: i32, value: T) -> Result<bool, String>;

    /// Establish Start TLS
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    ///
    /// # Returns
    /// `Ok(true)` on success, `Ok(false)` or `Err` otherwise
    fn start_tls(&self, link: &LdapLink) -> Result<bool, String>;

    /// Sort the result of a LDAP search
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    /// * `result` - LDAP result resource
    /// * `sort_filter` - attribute to use as key in sort
    ///
    /// # Returns
    /// `Ok(())` on success, `Err` on failure
    fn sort(&self, link: &LdapLink, result: &LdapResult, sort_filter: &str) -> Result<(), String>;

    /// Unbind from LDAP directory
    ///
    /// # Arguments
    /// * `link` - LDAP link resource
    ///
    /// # Returns
    /// `Ok(true)` on success, `Ok(false)` or `Err` otherwise
    fn unbind(&self, link: &LdapLink) -> Result<bool, String>;

    /// Checks whether the server supports LDAP
    ///
    /// # Returns
    /// `true` if it is the case, `false` otherwise
    fn are_ldap_functions_available(&self) -> bool;

    /// Checks whether there is support for LDAP Paged Results
    ///
    /// # Returns
    /// `true` if it is the case, `false` otherwise
    fn has_paged_result_support(&self) -> bool;

    /// Checks whether the submitted parameter is a resource
    ///
    /// # Arguments
    /// * `resource` - the resource variable to check
    ///
    /// # Returns
    /// `true` if it is a resource, `false` otherwise
    fn is_resource<T: std::any::Any>(&self, resource: &T) -> bool;
}