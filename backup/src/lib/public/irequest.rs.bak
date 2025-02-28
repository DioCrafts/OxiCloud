// ownCloud
//
// @author Thomas Müller
// @copyright 2013 Thomas Müller deepdiver@owncloud.com
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
// Request interface

use std::collections::HashMap;

/// This interface provides an immutable object with with accessors to
/// request variables and headers.
///
/// Access request variables by method and name.
///
/// Examples:
///
/// request.post.get("myvar"); // Only look for POST variables
/// request.get_param("myvar", None);
/// Looks in the combined GET, POST and urlParams array.
///
/// If you access e.g. post but the current HTTP request method
/// is GET a LogicError will be returned.
///
/// NOTE:
/// - When accessing put a stream resource is returned and the accessor
///   will return None on subsequent access to put or patch.
/// - When accessing patch and the Content-Type is either application/json
///   or application/x-www-form-urlencoded (most cases) it will act like get
///   and post and return a HashMap. Otherwise the raw data will be returned.
pub trait IRequest {
    /// Get a specific header by name
    fn get_header(&self, name: &str) -> Option<String>;

    /// Lets you access post and get parameters by the index
    /// In case of json requests the encoded json body is accessed
    ///
    /// # Arguments
    ///
    /// * `key` - The key which you want to access in the URL Parameter
    ///           placeholder, POST or GET array.
    ///           The priority how they're returned is the following:
    ///           1. URL parameters
    ///           2. POST parameters
    ///           3. GET parameters
    /// * `default` - If the key is not found, this value will be returned
    ///
    /// # Returns
    ///
    /// * The content associated with the key or the default value
    fn get_param<T>(&self, key: &str, default: Option<T>) -> Option<T> where T: Clone;

    /// Returns all params that were received, be it from the request
    /// (as GET or POST) or through the URL by the route
    ///
    /// # Returns
    ///
    /// * A HashMap with all parameters
    fn get_params(&self) -> HashMap<String, String>;

    /// Returns the method of the request
    ///
    /// # Returns
    ///
    /// * The method of the request (POST, GET, etc)
    fn get_method(&self) -> String;

    /// Shortcut for accessing an uploaded file through the file uploads
    ///
    /// # Arguments
    ///
    /// * `key` - The key that will be used to find the uploaded file
    ///
    /// # Returns
    ///
    /// * Information about the uploaded file
    fn get_uploaded_file(&self, key: &str) -> Option<HashMap<String, String>>;

    /// Shortcut for getting env variables
    ///
    /// # Arguments
    ///
    /// * `key` - The key that will be taken from the environment variables
    ///
    /// # Returns
    ///
    /// * The value of the environment variable
    fn get_env(&self, key: &str) -> Option<String>;

    /// Shortcut for getting cookie variables
    ///
    /// # Arguments
    ///
    /// * `key` - The key that will be taken from the cookies
    ///
    /// # Returns
    ///
    /// * The value of the cookie
    fn get_cookie(&self, key: &str) -> Option<String>;

    /// Checks if the CSRF check was correct
    ///
    /// # Returns
    ///
    /// * true if CSRF check passed
    fn passes_csrf_check(&self) -> bool;
}