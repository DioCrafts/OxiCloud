// ownCloud
//
// @author Bart Visscher
// @copyright 2013 Bart Visscher bartv@thisnet.nl
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
/// Helper interface
pub mod ocp {
    /// Functions that don't have any specific interface to place
    pub trait Helper {
        /// Gets the content of an URL by using HTTP client or a fallback if it is not
        /// available
        ///
        /// # Arguments
        ///
        /// * `url` - The URL that should be fetched
        ///
        /// # Returns
        ///
        /// The content of the webpage as a String, or an error if the request failed
        fn get_url_content(&self, url: &str) -> Result<String, Box<dyn std::error::Error>>;

        /// Async version of get_url_content
        ///
        /// # Arguments
        ///
        /// * `url` - The URL that should be fetched
        ///
        /// # Returns
        ///
        /// The content of the webpage as a String, or an error if the request failed
        async fn get_url_content_async(&self, url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    }
}