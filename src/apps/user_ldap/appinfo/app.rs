//! # ownCloud - user_ldap
//!
//! Originally by Dominik Schmidt
//! Copyright 2011 Dominik Schmidt dev@dominik-schmidt.de
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

use ocp::app::App;
use ocp::backgroundjob::Backgroundjob;
use ocp::util::{self, LogLevel};
use oc_user::User;
use oc_group::Group;

use crate::lib::helper::Helper;
use crate::lib::ldap::LDAP;
use crate::lib::connection::Connection;
use crate::lib::access::Access;
use crate::lib::jobs::Jobs;
use crate::user_ldap::UserLdap;
use crate::group_ldap::GroupLdap;
use crate::user_proxy::UserProxy;
use crate::group_proxy::GroupProxy;

pub fn register_app() -> Result<(), Box<dyn std::error::Error>> {
    // Register admin settings
    App::register_admin("user_ldap", "settings");

    // Get LDAP server configuration prefixes
    let config_prefixes = Helper::get_server_configuration_prefixes(true)?;
    let ldap_wrapper = LDAP::new();

    let (user_backend, group_backend) = if config_prefixes.len() == 1 {
        // Single LDAP configuration
        let connector = Connection::new(ldap_wrapper.clone(), &config_prefixes[0]);
        let ldap_access = Access::new(connector, ldap_wrapper.clone());
        let user_backend = UserLdap::new(ldap_access.clone());
        let group_backend = GroupLdap::new(ldap_access);
        
        (Some(user_backend), Some(group_backend))
    } else if config_prefixes.len() > 1 {
        // Multiple LDAP configurations
        let user_backend = UserProxy::new(config_prefixes.clone(), ldap_wrapper.clone());
        let group_backend = GroupProxy::new(config_prefixes.clone(), ldap_wrapper);
        
        (Some(user_backend), Some(group_backend))
    } else {
        (None, None)
    };

    // Register user and group backends if we have configurations
    if !config_prefixes.is_empty() {
        if let Some(backend) = user_backend {
            User::use_backend(backend);
        }
        
        if let Some(backend) = group_backend {
            Group::use_backend(backend);
        }
    }

    // Add settings page to navigation
    let entry = NavigationEntry {
        id: String::from("user_ldap_settings"),
        order: 1,
        href: util::link_to("user_ldap", "settings.php")?,
        name: String::from("LDAP"),
    };

    // Register background job
    Backgroundjob::register_job::<Jobs>()?;

    // Check for incompatible app
    if App::is_enabled("user_webdavauth")? {
        util::write_log(
            "user_ldap",
            "user_ldap and user_webdavauth are incompatible. You may experience unexpected behaviour",
            LogLevel::Warn,
        );
    }

    Ok(())
}

struct NavigationEntry {
    id: String,
    order: i32,
    href: String,
    name: String,
}