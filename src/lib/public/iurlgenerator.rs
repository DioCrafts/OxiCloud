// Copyright (C) 2013 Bart Visscher <bartv@thisnet.nl>
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
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;

/// Class to generate URLs
#[async_trait::async_trait]
pub trait UrlGenerator {
    /// Returns the URL for a route
    /// 
    /// # Arguments
    /// * `route_name` - the name of the route
    /// * `arguments` - an array with arguments which will be filled into the url
    /// 
    /// # Returns
    /// * the url
    fn link_to_route<S: AsRef<str>>(&self, route_name: S, arguments: HashMap<String, String>) -> String;

    /// Returns an URL for an image or file
    /// 
    /// # Arguments
    /// * `app_name` - the name of the app
    /// * `file` - the name of the file
    /// 
    /// # Returns
    /// * the url
    fn link_to<S: AsRef<str>>(&self, app_name: S, file: S) -> String;

    /// Returns the link to an image, like link_to but only with prepending img/
    /// 
    /// # Arguments
    /// * `app_name` - the name of the app
    /// * `file` - the name of the file
    /// 
    /// # Returns
    /// * the url
    fn image_path<S: AsRef<str>>(&self, app_name: S, file: S) -> String;

    /// Makes an URL absolute
    /// 
    /// # Arguments
    /// * `url` - the url in the owncloud host
    /// 
    /// # Returns
    /// * the absolute version of the url
    fn get_absolute_url<S: AsRef<str>>(&self, url: S) -> String;
}