// # External Files Settings
//
// Originally by Michael Gapczynski
// Copyright 2012 Michael Gapczynski mtgap@owncloud.com
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

use crate::utils::OcUtil;
use crate::config::{
    Config,
    MountConfig,
    User,
    Group,
};
use crate::templates::Template;
use anyhow::{Result, Context};
use async_trait::async_trait;

pub struct FilesExternalSettings;

impl FilesExternalSettings {
    /// Renders the files external settings page
    pub async fn render() -> Result<String> {
        // Check admin permissions
        OcUtil::check_admin_user().context("User is not an admin")?;

        // Add required scripts and styles
        OcUtil::add_script("files_external", "settings").await?;
        OcUtil::add_script("3rdparty", "chosen/chosen.jquery.min").await?;
        OcUtil::add_style("files_external", "settings").await?;
        OcUtil::add_style("3rdparty", "chosen/chosen").await?;

        // Create template
        let mut tmpl = Template::new("files_external", "settings")?;
        
        // Assign template variables
        tmpl.assign("isAdminPage", true);
        tmpl.assign("mounts", MountConfig::get_system_mount_points().await?);
        tmpl.assign("backends", MountConfig::get_backends().await?);
        tmpl.assign("groups", Group::get_groups().await?);
        tmpl.assign("users", User::get_users().await?);
        tmpl.assign("userDisplayNames", User::get_display_names().await?);
        tmpl.assign("dependencies", MountConfig::check_dependencies().await?);
        
        let allow_user_mounting = Config::get_app_value(
            "files_external", 
            "allow_user_mounting", 
            "yes"
        ).await?;
        
        tmpl.assign("allowUserMounting", allow_user_mounting);
        
        // Render the template
        tmpl.fetch_page().await
    }
}