// Copyright 2012 Frank Karlitschek frank@owncloud.org
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

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod user_webdavauth;

/// Entry point for the user_webdavauth application
pub async fn initialize(app_service: &AppService, user_service: &UserService) -> Result<()> {
    // Get the app path and include the user_webdavauth module
    let app_path = app_service.get_app_path("user_webdavauth")?;
    
    // Register admin settings page
    app_service.register_admin("user_webdavauth", "settings")?;
    
    // Register and use the WEBDAVAUTH backend
    user_service.register_backend("WEBDAVAUTH")?;
    user_service.use_backend("WEBDAVAUTH")?;
    
    // Add settings page to navigation
    let entry = NavigationEntry {
        id: "user_webdavauth_settings".to_string(),
        order: 1,
        href: app_service.link_to("user_webdavauth", "settings.php")?,
        name: "WEBDAVAUTH".to_string(),
    };
    
    app_service.add_navigation_entry(entry)?;
    
    Ok(())
}

/// Represents a navigation entry in the application
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NavigationEntry {
    id: String,
    order: i32,
    href: String,
    name: String,
}

/// Service for managing applications
pub trait AppService {
    /// Get the filesystem path to an app
    fn get_app_path(&self, app_name: &str) -> Result<PathBuf>;
    
    /// Register an admin page
    fn register_admin(&self, app_name: &str, page: &str) -> Result<()>;
    
    /// Create a link to an app page
    fn link_to(&self, app_name: &str, page: &str) -> Result<String>;
    
    /// Add a navigation entry
    fn add_navigation_entry(&self, entry: NavigationEntry) -> Result<()>;
}

/// Service for managing users
pub trait UserService {
    /// Register a new authentication backend
    fn register_backend(&self, backend_name: &str) -> Result<()>;
    
    /// Use a specific authentication backend
    fn use_backend(&self, backend_name: &str) -> Result<()>;
}