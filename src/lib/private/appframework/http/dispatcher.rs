//! ownCloud - App Framework
//!
//! @author Bernhard Posselt, Thomas Tanghus, Bart Visscher
//! @copyright 2012 Bernhard Posselt nukeawhale@gmail.com
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use crate::appframework::http::Http;
use crate::appframework::middleware::MiddlewareDispatcher;
use std::collections::HashMap;
use std::fmt::Debug;

/// Trait that all controllers must implement
pub trait Controller: Send + Sync {
    // This is a marker trait that will be implemented by actual controllers
}

/// Response object that contains status, headers and output
pub struct Response {
    status: u16,
    headers: HashMap<String, String>,
    last_modified: Option<chrono::DateTime<chrono::Utc>>,
    etag: Option<String>,
    body: String,
}

impl Response {
    /// Creates a new response with default values
    pub fn new() -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            last_modified: None,
            etag: None,
            body: String::new(),
        }
    }

    /// Gets the HTTP status code
    pub fn get_status(&self) -> u16 {
        self.status
    }

    /// Gets the last modified timestamp
    pub fn get_last_modified(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        self.last_modified.as_ref()
    }

    /// Gets the ETag
    pub fn get_etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }

    /// Gets all headers
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Renders the response body
    pub fn render(&self) -> &str {
        &self.body
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

/// Class to dispatch the request to the middleware dispatcher
pub struct Dispatcher {
    middleware_dispatcher: MiddlewareDispatcher,
    protocol: Http,
}

impl Dispatcher {
    /// Creates a new dispatcher
    ///
    /// # Parameters
    /// * `protocol` - the http protocol which contains all status headers
    /// * `middleware_dispatcher` - the dispatcher which runs the middleware
    pub fn new(protocol: Http, middleware_dispatcher: MiddlewareDispatcher) -> Self {
        Self {
            protocol,
            middleware_dispatcher,
        }
    }

    /// Handles a request and calls the dispatcher on the controller
    ///
    /// # Parameters
    /// * `controller` - the controller which will be called
    /// * `method_name` - the method name which will be called on the controller
    ///
    /// # Returns
    /// A tuple containing:
    /// * String with the http main header
    /// * HashMap with headers in the form: key => value
    /// * String with the response output
    pub async fn dispatch<C, F, Fut, E>(
        &self,
        controller: &C,
        method: F,
    ) -> Result<(String, HashMap<String, String>, String), E>
    where
        C: Controller + Debug,
        F: Fn(&C) -> Fut,
        Fut: std::future::Future<Output = Result<Response, E>>,
        E: std::error::Error + 'static,
    {
        let mut response = match self.execute_controller(controller, method).await {
            Ok(response) => response,
            Err(e) => {
                // Try to handle the exception with middleware
                match self.middleware_dispatcher.after_exception(controller, &e).await {
                    Some(response) => response,
                    None => return Err(e),
                }
            }
        };

        // Process the response through middleware
        response = self
            .middleware_dispatcher
            .after_controller(controller, response)
            .await;

        // Get the output and run it through middleware
        let output = response.render().to_string();
        let processed_output = self
            .middleware_dispatcher
            .before_output(controller, &output)
            .await;

        // Create status header based on response properties
        let status_header = self.protocol.get_status_header(
            response.get_status(),
            response.get_last_modified(),
            response.get_etag(),
        );

        Ok((status_header, response.get_headers().clone(), processed_output))
    }

    /// Execute the controller method with middleware
    async fn execute_controller<C, F, Fut, E>(
        &self,
        controller: &C,
        method: F,
    ) -> Result<Response, E>
    where
        C: Controller + Debug,
        F: Fn(&C) -> Fut,
        Fut: std::future::Future<Output = Result<Response, E>>,
        E: std::error::Error + 'static,
    {
        // Run before controller middleware
        self.middleware_dispatcher.before_controller(controller).await;
        
        // Execute the controller method
        method(controller).await
    }
}