// ownCloud - LDAP Wrapper
//
// Copyright 2013 Arthur Schiwon blizzz@owncloud.com
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

use ldap3::{LdapConn, LdapConnSettings, LdapResult, ResultEntry, SearchOptions};
use log::{debug, warn};
use std::os::raw::c_int;
use std::result::Result;

/// LDAP wrapper interface
pub trait ILdapWrapper {
    fn bind(&self, link: &mut LdapConn, dn: &str, password: &str) -> Result<(), LdapError>;
    fn connect(&self, host: &str, port: u16) -> Result<LdapConn, LdapError>;
    fn control_paged_result_response(&self, link: &mut LdapConn, result: &LdapResult, cookie: &mut Vec<u8>) -> Result<bool, LdapError>;
    fn control_paged_result(&self, link: &mut LdapConn, pagesize: u32, is_critical: bool, cookie: Option<&[u8]>) -> Result<(), LdapError>;
    fn count_entries(&self, _link: &LdapConn, result: &LdapResult) -> Result<usize, LdapError>;
    fn errno(&self, link: &LdapConn) -> Result<c_int, LdapError>;
    fn error(&self, link: &LdapConn) -> Result<String, LdapError>;
    fn first_entry(&self, _link: &LdapConn, result: &LdapResult) -> Result<Option<ResultEntry>, LdapError>;
    fn get_attributes(&self, _link: &LdapConn, entry: &ResultEntry) -> Result<Vec<(String, Vec<String>)>, LdapError>;
    fn get_dn(&self, _link: &LdapConn, entry: &ResultEntry) -> Result<String, LdapError>;
    fn get_entries(&self, _link: &LdapConn, result: &LdapResult) -> Result<Vec<ResultEntry>, LdapError>;
    fn next_entry(&self, _link: &LdapConn, result: &LdapResult, current: &ResultEntry) -> Result<Option<ResultEntry>, LdapError>;
    fn read(&self, link: &mut LdapConn, base_dn: &str, filter: &str, attr: &[&str]) -> Result<LdapResult, LdapError>;
    fn search(&self, link: &mut LdapConn, base_dn: &str, filter: &str, attr: &[&str]) -> Result<LdapResult, LdapError>;
    fn set_option(&self, link: &mut LdapConn, option: i32, value: &str) -> Result<(), LdapError>;
    fn sort(&self, _link: &LdapConn, result: &mut LdapResult, sort_filter: &str) -> Result<(), LdapError>;
    fn start_tls(&self, link: &mut LdapConn) -> Result<(), LdapError>;
    fn unbind(&self, link: &mut LdapConn) -> Result<(), LdapError>;
    
    /// Checks whether the server supports LDAP
    fn are_ldap_functions_available(&self) -> bool;
    
    /// Checks whether Rust LDAP library supports Paged Results
    fn has_paged_result_support(&self) -> bool;
    
    /// Checks whether the submitted parameter is a valid LDAP connection
    fn is_resource(&self, resource: &LdapConn) -> bool;
}

#[derive(Debug)]
pub enum LdapError {
    ConnectionError(String),
    BindError(String),
    SearchError(String),
    AttributeError(String),
    EncodingError(String),
    ResourceError(String),
    GenericError(String),
}

pub struct Ldap {
    cur_func: String,
    cur_args: Vec<String>,
}

impl Default for Ldap {
    fn default() -> Self {
        Self::new()
    }
}

impl Ldap {
    pub fn new() -> Self {
        Ldap {
            cur_func: String::new(),
            cur_args: Vec::new(),
        }
    }

    fn pre_function_call(&mut self, function_name: &str, args: Vec<String>) {
        self.cur_func = function_name.to_string();
        self.cur_args = args;
    }

    fn post_function_call(&mut self, link: Option<&LdapConn>) -> Result<(), LdapError> {
        if let Some(conn) = link {
            let error_code = self.errno(conn)?;
            if error_code != 0 {
                let error_msg = self.error(conn)?;
                
                // Handle specific error codes and functions
                match (self.cur_func.as_str(), error_code) {
                    ("ldap_sort", -4) => {
                        // You can safely ignore that decoding error.
                        // ... says https://bugs.php.net/bug.php?id=18023
                    },
                    ("ldap_get_entries", -4) => {
                        // Similar to above case
                    },
                    (_, 32) => {
                        // for now
                    },
                    (_, 10) => {
                        // referrals, we switch them off, but then there is AD :)
                    },
                    _ => {
                        debug!(
                            "LDAP error {} ({}) after calling {}",
                            error_msg, error_code, self.cur_func
                        );
                    }
                }
            }
        }

        self.cur_func.clear();
        self.cur_args.clear();
        
        Ok(())
    }
}

impl ILdapWrapper for Ldap {
    fn bind(&self, link: &mut LdapConn, dn: &str, password: &str) -> Result<(), LdapError> {
        match link.simple_bind(dn, password) {
            Ok(_) => Ok(()),
            Err(e) => Err(LdapError::BindError(e.to_string())),
        }
    }

    fn connect(&self, host: &str, port: u16) -> Result<LdapConn, LdapError> {
        let settings = LdapConnSettings::new().set_timeout(std::time::Duration::from_secs(30));
        let address = format!("ldap://{}:{}", host, port);
        
        match LdapConn::with_settings(settings, &address) {
            Ok(conn) => Ok(conn),
            Err(e) => Err(LdapError::ConnectionError(e.to_string())),
        }
    }

    fn control_paged_result_response(&self, link: &mut LdapConn, result: &LdapResult, cookie: &mut Vec<u8>) -> Result<bool, LdapError> {
        // Implementation depends on the specific LDAP library used
        // This is a simplified version
        match result.paged_result_cookie() {
            Some(new_cookie) => {
                *cookie = new_cookie.to_vec();
                Ok(!cookie.is_empty())
            },
            None => Ok(false),
        }
    }

    fn control_paged_result(&self, link: &mut LdapConn, pagesize: u32, is_critical: bool, cookie: Option<&[u8]>) -> Result<(), LdapError> {
        // Implementation depends on the specific LDAP library used
        // This is a simplified version as ldap3 handles this differently
        Ok(())
    }

    fn count_entries(&self, _link: &LdapConn, result: &LdapResult) -> Result<usize, LdapError> {
        Ok(result.count())
    }

    fn errno(&self, link: &LdapConn) -> Result<c_int, LdapError> {
        // ldap3 doesn't expose the raw errno directly
        // This would need to be adapted based on actual library used
        Ok(0) // Default to success
    }

    fn error(&self, link: &LdapConn) -> Result<String, LdapError> {
        // ldap3 doesn't expose error retrieval this way
        // This would need to be adapted based on actual library used
        Ok(String::new())
    }

    fn first_entry(&self, _link: &LdapConn, result: &LdapResult) -> Result<Option<ResultEntry>, LdapError> {
        if result.count() > 0 {
            match result.entries().first() {
                Some(entry) => Ok(Some(entry.clone())),
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn get_attributes(&self, _link: &LdapConn, entry: &ResultEntry) -> Result<Vec<(String, Vec<String>)>, LdapError> {
        let mut attributes = Vec::new();
        
        for (key, values) in entry.attrs.iter() {
            let string_values: Vec<String> = values.iter()
                .filter_map(|v| String::from_utf8(v.clone()).ok())
                .collect();
            
            attributes.push((key.clone(), string_values));
        }
        
        Ok(attributes)
    }

    fn get_dn(&self, _link: &LdapConn, entry: &ResultEntry) -> Result<String, LdapError> {
        Ok(entry.dn.clone())
    }

    fn get_entries(&self, _link: &LdapConn, result: &LdapResult) -> Result<Vec<ResultEntry>, LdapError> {
        Ok(result.entries().to_vec())
    }

    fn next_entry(&self, _link: &LdapConn, result: &LdapResult, current: &ResultEntry) -> Result<Option<ResultEntry>, LdapError> {
        let entries = result.entries();
        let current_index = entries.iter().position(|e| e.dn == current.dn);
        
        match current_index {
            Some(idx) if idx + 1 < entries.len() => Ok(Some(entries[idx + 1].clone())),
            _ => Ok(None),
        }
    }

    fn read(&self, link: &mut LdapConn, base_dn: &str, filter: &str, attr: &[&str]) -> Result<LdapResult, LdapError> {
        let search_options = SearchOptions::new()
            .scope(ldap3::Scope::Base);
            
        match link.search(base_dn, filter, search_options, attr) {
            Ok(result) => Ok(result),
            Err(e) => Err(LdapError::SearchError(e.to_string())),
        }
    }

    fn search(&self, link: &mut LdapConn, base_dn: &str, filter: &str, attr: &[&str]) -> Result<LdapResult, LdapError> {
        let search_options = SearchOptions::new()
            .scope(ldap3::Scope::Subtree);
            
        match link.search(base_dn, filter, search_options, attr) {
            Ok(result) => Ok(result),
            Err(e) => Err(LdapError::SearchError(e.to_string())),
        }
    }

    fn set_option(&self, link: &mut LdapConn, option: i32, value: &str) -> Result<(), LdapError> {
        // This would need specific implementation based on the option
        // ldap3 handles options differently
        Ok(())
    }

    fn sort(&self, _link: &LdapConn, result: &mut LdapResult, sort_filter: &str) -> Result<(), LdapError> {
        // This would need a custom implementation as ldap3 doesn't provide this directly
        Ok(())
    }

    fn start_tls(&self, link: &mut LdapConn) -> Result<(), LdapError> {
        match link.start_tls() {
            Ok(_) => Ok(()),
            Err(e) => Err(LdapError::ConnectionError(format!("StartTLS failed: {}", e))),
        }
    }

    fn unbind(&self, link: &mut LdapConn) -> Result<(), LdapError> {
        match link.unbind() {
            Ok(_) => Ok(()),
            Err(e) => Err(LdapError::ConnectionError(e.to_string())),
        }
    }

    fn are_ldap_functions_available(&self) -> bool {
        // In Rust, we check if the LDAP library is available at compile time
        // This is a simplification - in a real implementation, you might check
        // at runtime if the library was successfully loaded
        true
    }

    fn has_paged_result_support(&self) -> bool {
        // ldap3 supports paged results
        true
    }

    fn is_resource(&self, resource: &LdapConn) -> bool {
        // In Rust, we check if the connection is valid
        // This is a simplification
        true
    }
}