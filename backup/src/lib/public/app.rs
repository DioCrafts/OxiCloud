// ownCloud
//
// @author Frank Karlitschek
// @copyright 2012 Frank Karlitschek frank@owncloud.org
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

// Public interface of ownCloud for apps to use.
// App Class

// use OCP namespace for all classes that are considered public.
// This means that they should be used by apps instead of the internal ownCloud classes
// namespace OCP;

use crate::internal::OC_App;
use crate::internal::OC_Util;
use std::path::Path;

/// This class provides functions to manage apps in ownCloud
pub struct App;

impl App {
    /// Makes ownCloud aware of this app
    ///
    /// # Arguments
    /// * `data` - HashMap with all information
    ///
    /// # Returns
    /// * `bool` - Always returns true
    ///
    /// # Deprecated
    /// This method is deprecated. Do not call it anymore.
    /// It'll remain in our public API for compatibility reasons.
    #[deprecated(note = "This method is deprecated. Do not call it anymore.")]
    pub fn register<T>(data: T) -> bool {
        true // don't do anything
    }

    /// Adds an entry to the navigation
    ///
    /// # Arguments
    /// * `data` - HashMap containing the data
    ///
    /// # Returns
    /// * `bool` - Result of the operation
    ///
    /// This function adds a new entry to the navigation visible to users. `data`
    /// is a HashMap.
    ///
    /// The following keys are required:
    ///   - id: unique id for this entry ('addressbook_index')
    ///   - href: link to the page
    ///   - name: Human readable name ('Addressbook')
    ///
    /// The following keys are optional:
    ///   - icon: path to the icon of the app
    ///   - order: integer, that influences the position of your application in
    ///     the navigation. Lower values come first.
    pub fn add_navigation_entry(data: &HashMap<String, String>) -> bool {
        OC_App::add_navigation_entry(data)
    }

    /// Marks a navigation entry as active
    ///
    /// # Arguments
    /// * `id` - ID of the entry
    ///
    /// # Returns
    /// * `bool` - Result of the operation
    ///
    /// This function sets a navigation entry as active and removes the 'active'
    /// property from all other entries. The templates can use this for
    /// highlighting the current position of the user.
    pub fn set_active_navigation_entry(id: &str) -> bool {
        OC_App::set_active_navigation_entry(id)
    }

    /// Register a Configuration Screen that should appear in the personal settings section.
    ///
    /// # Arguments
    /// * `app` - App ID
    /// * `page` - Page to be included
    pub fn register_personal(app: &str, page: &str) {
        OC_App::register_personal(app, page);
    }

    /// Register a Configuration Screen that should appear in the Admin section.
    ///
    /// # Arguments
    /// * `app` - App ID
    /// * `page` - Page to be included
    pub fn register_admin(app: &str, page: &str) {
        OC_App::register_admin(app, page);
    }

    /// Read app metadata from the info.xml file
    ///
    /// # Arguments
    /// * `app` - ID of the app or the path of the info.xml file
    /// * `is_path` - Whether the first argument is a path
    ///
    /// # Returns
    /// * `Result<HashMap<String, String>, Error>` - App info or error
    pub fn get_app_info(app: &str, is_path: bool) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        OC_App::get_app_info(app, is_path)
    }

    /// Checks whether or not an app is enabled
    ///
    /// # Arguments
    /// * `app` - App ID
    ///
    /// # Returns
    /// * `bool` - Whether the app is enabled
    pub fn is_enabled(app: &str) -> bool {
        OC_App::is_enabled(app)
    }

    /// Check if the app is enabled, redirects to home if not
    ///
    /// # Arguments
    /// * `app` - App ID
    ///
    /// # Panics
    /// If the app is not enabled
    pub fn check_app_enabled(app: &str) {
        OC_Util::check_app_enabled(app);
    }

    /// Get the last version of the app, either from appinfo/version or from appinfo/info.xml
    ///
    /// # Arguments
    /// * `app` - App ID
    ///
    /// # Returns
    /// * `Option<String>` - App version, if found
    pub fn get_app_version(app: &str) -> Option<String> {
        OC_App::get_app_version(app)
    }
}