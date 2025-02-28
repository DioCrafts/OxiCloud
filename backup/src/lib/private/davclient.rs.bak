/*
 * ownCloud
 *
 * @author Vincent Petry
 * @copyright 2013 Vincent Petry <pvince81@owncloud.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use std::time::Duration;
use reqwest::{Client, RequestBuilder};
use anyhow::Result;

/// This struct extends the SabreDAV client with additional functionality
/// like request timeout.
pub struct OcDavClient {
    inner: Client,
    request_timeout: Option<Duration>,
}

impl OcDavClient {
    /// Creates a new DAV client
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: Client::new(),
            request_timeout: None,
        })
    }

    /// Sets the request timeout or None to disable timeout.
    /// 
    /// # Arguments
    /// * `timeout` - timeout in seconds or 0 to disable
    pub fn set_request_timeout(&mut self, timeout: u64) {
        if timeout > 0 {
            self.request_timeout = Some(Duration::from_secs(timeout));
        } else {
            self.request_timeout = None;
        }
    }
    
    /// Performs an HTTP request with the configured client settings
    /// 
    /// # Arguments
    /// * `request_builder` - The RequestBuilder to execute
    pub async fn request(&self, mut request_builder: RequestBuilder) -> Result<reqwest::Response> {
        if let Some(timeout) = self.request_timeout {
            request_builder = request_builder.timeout(timeout);
        }
        
        let response = request_builder.send().await?;
        Ok(response)
    }
}