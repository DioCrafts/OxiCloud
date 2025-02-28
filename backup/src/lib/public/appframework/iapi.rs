// Copyright (c) 2012 Bernhard Posselt <nukeawhale@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/// A few very basic and frequently used API functions are combined in here
pub trait Api {
    /// Gets the userid of the current user
    ///
    /// # Returns
    /// The user id of the current user
    fn get_user_id(&self) -> Option<String>;

    /// Adds a new javascript file
    ///
    /// # Arguments
    /// * `script_name` - the name of the javascript in js/ without the suffix
    /// * `app_name` - the name of the app, defaults to the current one
    fn add_script(&self, script_name: &str, app_name: Option<&str>);

    /// Adds a new css file
    ///
    /// # Arguments
    /// * `style_name` - the name of the css file in css/without the suffix
    /// * `app_name` - the name of the app, defaults to the current one
    fn add_style(&self, style_name: &str, app_name: Option<&str>);

    /// Shorthand for add_script for files in the 3rdparty directory
    ///
    /// # Arguments
    /// * `name` - the name of the file without the suffix
    fn add_3rd_party_script(&self, name: &str);

    /// Shorthand for add_style for files in the 3rdparty directory
    ///
    /// # Arguments
    /// * `name` - the name of the file without the suffix
    fn add_3rd_party_style(&self, name: &str);

    /// Checks if an app is enabled
    ///
    /// # Arguments
    /// * `app_name` - the name of an app
    ///
    /// # Returns
    /// `true` if app is enabled
    fn is_app_enabled(&self, app_name: &str) -> bool;
}