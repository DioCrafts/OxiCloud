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

// # Public interface of ownCloud for apps to use.
// Response module.

use chrono::{DateTime, Duration, Utc};
use std::path::Path;

/// This struct provides convenient functions to send the correct http response headers
pub struct Response;

impl Response {
    /// Enable response caching by sending correct HTTP headers
    /// # Arguments
    /// * `cache_time` - time to cache the response
    ///   * > 0: cache time in seconds
    ///   * 0 and < 0: enable default browser caching
    ///   * None: cache indefinitely
    pub fn enable_caching(cache_time: Option<i64>) {
        // Call internal implementation
        crate::internal::oc_response::enable_caching(cache_time);
    }

    /// Checks and set Last-Modified header, when the request matches sends a
    /// 'not modified' response
    /// # Arguments
    /// * `last_modified` - time when the response was last modified
    pub fn set_last_modified_header(last_modified: &str) {
        crate::internal::oc_response::set_last_modified_header(last_modified);
    }

    /// Disable browser caching
    /// See enable_caching with cache_time = 0
    pub fn disable_caching() {
        crate::internal::oc_response::disable_caching();
    }

    /// Checks and set ETag header, when the request matches sends a
    /// 'not modified' response
    /// # Arguments
    /// * `etag` - token to use for modification check
    pub fn set_etag_header(etag: &str) {
        crate::internal::oc_response::set_etag_header(etag);
    }

    /// Send file as response, checking and setting caching headers
    /// # Arguments
    /// * `filepath` - path of file to send
    pub fn send_file<P: AsRef<Path>>(filepath: P) -> std::io::Result<()> {
        crate::internal::oc_response::send_file(filepath)
    }

    /// Set response expire time
    /// # Arguments
    /// * `expires` - when the response expires
    ///   * can be a Duration from now
    ///   * can be a DateTime specifying when to expire response
    pub fn set_expires_header<E: Into<ExpiresTime>>(expires: E) {
        crate::internal::oc_response::set_expires_header(expires.into());
    }

    /// Send redirect response
    /// # Arguments
    /// * `location` - URL to redirect to
    pub fn redirect(location: &str) {
        crate::internal::oc_response::redirect(location);
    }
}

/// Represents possible formats for expires header
#[derive(Debug)]
pub enum ExpiresTime {
    /// Duration from now
    Duration(Duration),
    /// Specific date and time
    DateTime(DateTime<Utc>),
}

impl From<Duration> for ExpiresTime {
    fn from(duration: Duration) -> Self {
        ExpiresTime::Duration(duration)
    }
}

impl From<DateTime<Utc>> for ExpiresTime {
    fn from(date_time: DateTime<Utc>) -> Self {
        ExpiresTime::DateTime(date_time)
    }
}