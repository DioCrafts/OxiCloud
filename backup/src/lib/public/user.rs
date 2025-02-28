// ownCloud
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

/// Public interface of ownCloud for apps to use.
/// User module provides access to user management functionality.
pub mod user {
    use crate::internal::oc_user;
    use crate::internal::oc_util;
    
    /// This struct provides access to the user management.
    /// You can get information about the currently logged in user and the permissions.
    pub struct User;
    
    impl User {
        /// Get the user id of the user currently logged in.
        ///
        /// # Returns
        /// `Option<String>` - The user id or None if no user is logged in
        pub fn get_user() -> Option<String> {
            oc_user::get_user()
        }
        
        /// Get a list of all users
        ///
        /// # Parameters
        /// * `search` - Optional search pattern
        /// * `limit` - Optional limit for results
        /// * `offset` - Optional offset for results
        ///
        /// # Returns
        /// `Vec<String>` - A list of all user ids matching the criteria
        pub fn get_users(search: Option<&str>, limit: Option<usize>, offset: Option<usize>) -> Vec<String> {
            let search_str = search.unwrap_or("");
            oc_user::get_users(search_str, limit, offset)
        }
        
        /// Get the user display name of the user currently logged in or specified user.
        ///
        /// # Parameters
        /// * `user` - Optional user id, if None the current user is used
        ///
        /// # Returns
        /// `Option<String>` - The display name or None if user doesn't exist
        pub fn get_display_name(user: Option<&str>) -> Option<String> {
            oc_user::get_display_name(user)
        }
        
        /// Get a list of all display names and user ids.
        ///
        /// # Parameters
        /// * `search` - Optional search pattern
        /// * `limit` - Optional limit for results
        /// * `offset` - Optional offset for results
        ///
        /// # Returns
        /// `std::collections::HashMap<String, String>` - Map of user ids to display names
        pub fn get_display_names(
            search: Option<&str>,
            limit: Option<usize>,
            offset: Option<usize>,
        ) -> std::collections::HashMap<String, String> {
            let search_str = search.unwrap_or("");
            oc_user::get_display_names(search_str, limit, offset)
        }
        
        /// Check if the user is logged in
        ///
        /// # Returns
        /// `bool` - true if user is logged in, false otherwise
        pub fn is_logged_in() -> bool {
            oc_user::is_logged_in()
        }
        
        /// Check if a user exists
        ///
        /// # Parameters
        /// * `uid` - The username to check
        /// * `excluding_backend` - Optional backend to exclude from check
        ///
        /// # Returns
        /// `bool` - true if user exists, false otherwise
        pub fn user_exists(uid: &str, excluding_backend: Option<&str>) -> bool {
            oc_user::user_exists(uid, excluding_backend)
        }
        
        /// Logs the user out including all the session data
        /// Logout, destroys session
        pub fn logout() {
            oc_user::logout();
        }
        
        /// Check if the password is correct without logging in the user
        ///
        /// # Parameters
        /// * `uid` - The username
        /// * `password` - The password
        ///
        /// # Returns
        /// `Option<String>` - Some(username) on success, None otherwise
        pub fn check_password(uid: &str, password: &str) -> Option<String> {
            oc_user::check_password(uid, password)
        }
        
        /// Check if the user is an admin, redirects to home if not
        pub fn check_admin_user() {
            oc_util::check_admin_user();
        }
        
        /// Check if the user is logged in, redirects to home if not.
        /// With redirect URL parameter to the request URI.
        pub fn check_logged_in() {
            oc_util::check_logged_in();
        }
    }
}