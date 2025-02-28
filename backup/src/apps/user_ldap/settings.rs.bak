// User LDAP settings module
//
// Authors:
// - Dominik Schmidt
// - Arthur Schiwon
// 
// Copyright:
// - 2011 Dominik Schmidt dev@dominik-schmidt.de
// - 2012-2013 Arthur Schiwon blizzz@owncloud.com
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

use crate::core::util::{self, Util};
use crate::core::template::Template;
use crate::user_ldap::lib::helper::Helper;
use crate::user_ldap::lib::configuration::Configuration;
use std::collections::HashMap;

pub async fn render_settings() -> Result<String, Box<dyn std::error::Error>> {
    // Check admin user permissions
    Util::check_admin_user()?;

    // Add required scripts and styles
    util::add_script("user_ldap", "settings");
    util::add_script("core", "jquery.multiselect");
    util::add_style("user_ldap", "settings");
    util::add_style("core", "jquery.multiselect");
    util::add_style("core", "jquery-ui-1.10.0.custom");

    // Create the main template
    let mut tmpl = Template::new("user_ldap", "settings");

    // Get server configuration data
    let prefixes = Helper::get_server_configuration_prefixes().await?;
    let hosts = Helper::get_server_configuration_hosts().await?;

    let mut wizard_html = String::new();
    let mut toc = HashMap::new();

    // Load control templates
    let w_controls = Template::new("user_ldap", "part.wizardcontrols").fetch_page().await?;
    let s_controls = Template::new("user_ldap", "part.settingcontrols").fetch_page().await?;

    // Define wizard tabs
    let wiz_tabs = vec![
        ("part.wizard-server", "Server"),
        ("part.wizard-userfilter", "User Filter"),
        ("part.wizard-loginfilter", "Login Filter"),
        ("part.wizard-groupfilter", "Group Filter"),
    ];

    // Build wizard tabs HTML
    for (i, (tpl, cap)) in wiz_tabs.iter().enumerate() {
        let mut tab = Template::new("user_ldap", tpl);
        
        if i == 0 {
            tab.assign("serverConfigurationPrefixes", &prefixes);
            tab.assign("serverConfigurationHosts", &hosts);
        }
        
        tab.assign("wizardControls", &w_controls);
        wizard_html.push_str(&tab.fetch_page().await?);
        toc.insert(format!("#ldapWizard{}", i+1), cap.to_string());
    }

    // Assign template variables
    tmpl.assign("tabs", &wizard_html);
    tmpl.assign("toc", &toc);
    tmpl.assign("settingControls", &s_controls);

    // Assign default values
    let config = Configuration::new("", false);
    let defaults = config.get_defaults();
    
    for (key, default) in defaults {
        tmpl.assign(&format!("{}_default", key), &default);
    }

    // Return the rendered page
    Ok(tmpl.fetch_page().await?)
}