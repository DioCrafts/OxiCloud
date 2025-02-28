// Copyright (c) 2012 Arthur Schiwon blizzz@owncloud.com
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

use crate::lib::access::Access;
use crate::lib::backend_utility::BackendUtility;
use std::collections::HashMap;
use async_trait::async_trait;

pub const OC_GROUP_BACKEND_GET_DISPLAYNAME: u32 = 0x00000001;

/// LDAP group backend
pub struct GroupLdap {
    access: Access,
    enabled: bool,
}

#[async_trait]
pub trait GroupInterface {
    async fn in_group(&self, uid: &str, gid: &str) -> bool;
    async fn get_user_groups(&self, uid: &str) -> Vec<String>;
    async fn users_in_group(&self, gid: &str, search: &str, limit: i32, offset: i32) -> Vec<String>;
    async fn display_names_in_group(&self, gid: &str, search: &str, limit: i32, offset: i32) -> HashMap<String, String>;
    async fn get_groups(&self, search: &str, limit: i32, offset: i32) -> Vec<String>;
    async fn group_exists(&self, gid: &str) -> bool;
    fn implements_actions(&self, actions: u32) -> bool;
}

impl GroupLdap {
    pub fn new(access: Access) -> Self {
        let filter = &access.connection.ldap_group_filter;
        let gassoc = &access.connection.ldap_group_member_assoc_attr;
        let enabled = !filter.is_empty() && !gassoc.is_empty();
        
        Self { access, enabled }
    }
}

#[async_trait]
impl GroupInterface for GroupLdap {
    /// Checks whether the user is member of a group or not.
    async fn in_group(&self, uid: &str, gid: &str) -> bool {
        if !self.enabled {
            return false;
        }
        
        let cache_key = format!("inGroup{}:{}", uid, gid);
        if let Some(cached) = self.access.connection.get_from_cache(&cache_key) {
            return cached.as_bool().unwrap_or(false);
        }
        
        let dn_user = match self.access.username2dn(uid).await {
            Some(dn) => dn,
            None => {
                self.access.connection.write_to_cache(&cache_key, false).await;
                return false;
            }
        };
        
        let dn_group = match self.access.groupname2dn(gid).await {
            Some(dn) => dn,
            None => {
                self.access.connection.write_to_cache(&cache_key, false).await;
                return false;
            }
        };
        
        let members = match self.access.read_attribute(
            &dn_group,
            &self.access.connection.ldap_group_member_assoc_attr
        ).await {
            Some(members) => members,
            None => {
                self.access.connection.write_to_cache(&cache_key, false).await;
                return false;
            }
        };
        
        let members = if self.access.connection.ldap_group_member_assoc_attr.to_lowercase() == "memberuid" {
            let mut dns = Vec::new();
            for mid in members {
                let filter = self.access.connection.ldap_login_filter.replace("%uid", &mid);
                let ldap_users = self.access.fetch_list_of_users(&filter, "dn").await;
                if ldap_users.is_empty() {
                    continue;
                }
                dns.push(ldap_users[0].clone());
            }
            dns
        } else {
            members
        };
        
        let is_in_group = members.contains(&dn_user);
        self.access.connection.write_to_cache(&cache_key, is_in_group).await;
        
        is_in_group
    }
    
    /// Get all groups a user belongs to
    async fn get_user_groups(&self, uid: &str) -> Vec<String> {
        if !self.enabled {
            return Vec::new();
        }
        
        let cache_key = format!("getUserGroups{}", uid);
        if let Some(cached) = self.access.connection.get_from_cache(&cache_key) {
            return cached.as_array().unwrap_or_default().clone();
        }
        
        let user_dn = match self.access.username2dn(uid).await {
            Some(dn) => dn,
            None => {
                self.access.connection.write_to_cache(&cache_key, Vec::<String>::new()).await;
                return Vec::new();
            }
        };
        
        let uid = match self.access.connection.ldap_group_member_assoc_attr.to_lowercase().as_str() {
            "uniquemember" | "member" => user_dn,
            "memberuid" => {
                let result = self.access.read_attribute(&user_dn, "uid").await.unwrap_or_default();
                if result.is_empty() {
                    user_dn
                } else {
                    result[0].clone()
                }
            },
            _ => user_dn,
        };
        
        let filter = self.access.combine_filter_with_and(&[
            &self.access.connection.ldap_group_filter,
            &format!("{}={}", self.access.connection.ldap_group_member_assoc_attr, uid)
        ]);
        
        let groups = self.access.fetch_list_of_groups(
            &filter,
            &[&self.access.connection.ldap_group_display_name, "dn"]
        ).await;
        
        let mut groups = self.access.own_cloud_group_names(groups).await;
        groups.sort();
        groups.dedup();
        
        self.access.connection.write_to_cache(&cache_key, groups.clone()).await;
        
        groups
    }
    
    /// Get a list of all users in a group
    async fn users_in_group(&self, gid: &str, search: &str, limit: i32, offset: i32) -> Vec<String> {
        if !self.enabled {
            return Vec::new();
        }
        
        if !self.group_exists(gid).await {
            return Vec::new();
        }
        
        let cache_key = format!("usersInGroup-{}-{}-{}-{}", gid, search, limit, offset);
        if let Some(cached) = self.access.connection.get_from_cache(&cache_key) {
            return cached.as_array().unwrap_or_default().clone();
        }
        
        let cache_key_without_limit = format!("usersInGroup-{}-{}", gid, search);
        if let Some(cached) = self.access.connection.get_from_cache(&cache_key_without_limit) {
            let group_users = cached.as_array().unwrap_or_default().clone();
            let start = offset as usize;
            let end = if limit < 0 { group_users.len() } else { start + limit as usize };
            let sliced = group_users.into_iter().skip(start).take(end - start).collect::<Vec<_>>();
            self.access.connection.write_to_cache(&cache_key, sliced.clone()).await;
            return sliced;
        }
        
        let group_dn = match self.access.groupname2dn(gid).await {
            Some(dn) => dn,
            None => {
                self.access.connection.write_to_cache(&cache_key, Vec::<String>::new()).await;
                return Vec::new();
            }
        };
        
        let members = match self.access.read_attribute(
            &group_dn,
            &self.access.connection.ldap_group_member_assoc_attr
        ).await {
            Some(members) => members,
            None => {
                self.access.connection.write_to_cache(&cache_key, Vec::<String>::new()).await;
                return Vec::new();
            }
        };
        
        let is_member_uid = self.access.connection.ldap_group_member_assoc_attr.to_lowercase() == "memberuid";
        let mut group_users = Vec::new();
        
        for member in members {
            if is_member_uid {
                let filter = self.access.combine_filter_with_and(&[
                    &self.access.connection.ldap_login_filter.replace("%uid", &member),
                    &self.access.get_filter_part_for_user_search(search)
                ]);
                
                let ldap_users = self.access.fetch_list_of_users(&filter, "dn").await;
                if ldap_users.is_empty() {
                    continue;
                }
                
                if let Some(username) = self.access.dn2username(&ldap_users[0]).await {
                    group_users.push(username);
                }
            } else {
                if !search.is_empty() {
                    if !self.access.read_attribute(
                        &member,
                        &self.access.connection.ldap_user_display_name,
                        &self.access.get_filter_part_for_user_search(search)
                    ).await.is_some() {
                        continue;
                    }
                }
                
                if let Some(ocname) = self.access.dn2username(&member).await {
                    group_users.push(ocname);
                }
            }
        }
        
        group_users.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        self.access.connection.write_to_cache(&cache_key_without_limit, group_users.clone()).await;
        
        let start = offset as usize;
        let end = if limit < 0 { group_users.len() } else { start + limit as usize };
        let sliced = group_users.into_iter().skip(start).take(end - start).collect::<Vec<_>>();
        self.access.connection.write_to_cache(&cache_key, sliced.clone()).await;
        
        sliced
    }
    
    /// Get a list of all display names in a group
    async fn display_names_in_group(&self, gid: &str, search: &str, limit: i32, offset: i32) -> HashMap<String, String> {
        if !self.enabled {
            return HashMap::new();
        }
        
        if !self.group_exists(gid).await {
            return HashMap::new();
        }
        
        let users = self.users_in_group(gid, search, limit, offset).await;
        let mut display_names = HashMap::new();
        
        for user in users {
            let display_name = crate::utils::get_user_display_name(&user).await;
            display_names.insert(user, display_name);
        }
        
        display_names
    }
    
    /// Get a list of all groups
    async fn get_groups(&self, search: &str, limit: i32, offset: i32) -> Vec<String> {
        if !self.enabled {
            return Vec::new();
        }
        
        let cache_key = format!("getGroups-{}-{}-{}", search, limit, offset);
        
        // Check cache before driving unnecessary searches
        log::debug!("getGroups {}", cache_key);
        if let Some(cached) = self.access.connection.get_from_cache(&cache_key) {
            return cached.as_array().unwrap_or_default().clone();
        }
        
        let filter = self.access.combine_filter_with_and(&[
            &self.access.connection.ldap_group_filter,
            &self.access.get_filter_part_for_group_search(search)
        ]);
        
        log::debug!("getGroups Filter {}", filter);
        
        let ldap_limit = if limit <= 0 { None } else { Some(limit) };
        let ldap_groups = self.access.fetch_list_of_groups(
            &filter,
            &[&self.access.connection.ldap_group_display_name, "dn"],
            ldap_limit,
            Some(offset)
        ).await;
        
        let ldap_groups = self.access.own_cloud_group_names(ldap_groups).await;
        
        self.access.connection.write_to_cache(&cache_key, ldap_groups.clone()).await;
        
        ldap_groups
    }
    
    /// Check if a group exists
    async fn group_exists(&self, gid: &str) -> bool {
        let cache_key = format!("groupExists{}", gid);
        if let Some(cached) = self.access.connection.get_from_cache(&cache_key) {
            return cached.as_bool().unwrap_or(false);
        }
        
        // Getting dn, if false the group does not exist. If dn, it may be mapped
        // only, requires more checking.
        let dn = match self.access.groupname2dn(gid).await {
            Some(dn) => dn,
            None => {
                self.access.connection.write_to_cache(&cache_key, false).await;
                return false;
            }
        };
        
        // If group really still exists, we will be able to read its objectclass
        let objcs = match self.access.read_attribute(&dn, "objectclass").await {
            Some(objcs) if !objcs.is_empty() => objcs,
            _ => {
                self.access.connection.write_to_cache(&cache_key, false).await;
                return false;
            }
        };
        
        self.access.connection.write_to_cache(&cache_key, true).await;
        true
    }
    
    /// Check if backend implements actions
    fn implements_actions(&self, actions: u32) -> bool {
        (OC_GROUP_BACKEND_GET_DISPLAYNAME & actions) != 0
    }
}