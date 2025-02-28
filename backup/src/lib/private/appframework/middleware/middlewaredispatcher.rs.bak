// Copyright (C) 2012 Bernhard Posselt <nukeawhale@gmail.com>
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

// This module contains the middleware dispatcher implementation

use std::sync::Arc;

use crate::appframework::{
    controller::Controller,
    http::Response,
    middleware::Middleware,
};

/// This struct is used to store and run all the middleware in correct order
pub struct MiddlewareDispatcher {
    /// Vector containing all the middlewares
    middlewares: Vec<Arc<dyn Middleware>>,
    
    /// Counter which tells us what middleware was executed once an
    /// exception occurs
    middleware_counter: usize,
}

impl MiddlewareDispatcher {
    /// Constructor
    pub fn new() -> Self {
        MiddlewareDispatcher {
            middlewares: Vec::new(),
            middleware_counter: 0,
        }
    }

    /// Adds a new middleware
    pub fn register_middleware(&mut self, middleware: Arc<dyn Middleware>) {
        self.middlewares.push(middleware);
    }

    /// Returns a reference to all middleware elements
    pub fn get_middlewares(&self) -> &[Arc<dyn Middleware>] {
        &self.middlewares
    }

    /// This is being run in normal order before the controller is being
    /// called which allows several modifications and checks
    ///
    /// # Arguments
    /// * `controller` - The controller that is being called
    /// * `method_name` - The name of the method that will be called on the controller
    pub fn before_controller(&mut self, controller: &dyn Controller, method_name: &str) {
        // We need to count so that we know which middlewares we have to ask in
        // case there's an exception
        for middleware in &self.middlewares {
            self.middleware_counter += 1;
            middleware.before_controller(controller, method_name);
        }
    }

    /// This is being run when either the before_controller method or the
    /// controller method itself is throwing an exception. The middleware is asked
    /// in reverse order to handle the exception and to return a response.
    /// If the response is None, it is assumed that the exception could not be
    /// handled and the error will be thrown again
    ///
    /// # Arguments
    /// * `controller` - The controller that is being called
    /// * `method_name` - The name of the method that will be called on the controller
    /// * `error` - The thrown exception
    ///
    /// # Returns
    /// A Response object if the middleware can handle the exception
    ///
    /// # Errors
    /// The passed in exception if it can't handle it
    pub fn after_exception(
        &self,
        controller: &dyn Controller,
        method_name: &str,
        error: Box<dyn std::error::Error>,
    ) -> Result<Arc<dyn Response>, Box<dyn std::error::Error>> {
        for i in (0..self.middleware_counter).rev() {
            let middleware = &self.middlewares[i];
            match middleware.after_exception(controller, method_name, error.clone()) {
                Ok(response) => return Ok(response),
                Err(_) => continue,
            }
        }
        Err(error)
    }

    /// This is being run after a successful controller method call and allows
    /// the manipulation of a Response object. The middleware is run in reverse order
    ///
    /// # Arguments
    /// * `controller` - The controller that is being called
    /// * `method_name` - The name of the method that will be called on the controller
    /// * `response` - The generated response from the controller
    ///
    /// # Returns
    /// A Response object
    pub fn after_controller(
        &self,
        controller: &dyn Controller,
        method_name: &str,
        response: Arc<dyn Response>,
    ) -> Arc<dyn Response> {
        let mut current_response = response;
        for middleware in self.middlewares.iter().rev() {
            current_response = middleware.after_controller(controller, method_name, current_response);
        }
        current_response
    }

    /// This is being run after the response object has been rendered and
    /// allows the manipulation of the output. The middleware is run in reverse order
    ///
    /// # Arguments
    /// * `controller` - The controller that is being called
    /// * `method_name` - The name of the method that will be called on the controller
    /// * `output` - The generated output from a response
    ///
    /// # Returns
    /// The output that should be printed
    pub fn before_output(
        &self,
        controller: &dyn Controller,
        method_name: &str,
        output: String,
    ) -> String {
        let mut current_output = output;
        for middleware in self.middlewares.iter().rev() {
            current_output = middleware.before_output(controller, method_name, current_output);
        }
        current_output
    }
}

impl Default for MiddlewareDispatcher {
    fn default() -> Self {
        Self::new()
    }
}