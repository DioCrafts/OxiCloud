//! ownCloud - App Framework
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

use std::error::Error;

/// Middleware is used to provide hooks before or after controller methods and
/// deal with possible exceptions raised in the controller methods.
/// They're modeled after Django's middleware system:
/// https://docs.djangoproject.com/en/dev/topics/http/middleware/
pub trait Middleware {
    /// This is being run in normal order before the controller is being
    /// called which allows several modifications and checks
    ///
    /// # Arguments
    /// * `controller` - the controller that is being called
    /// * `method_name` - the name of the method that will be called on the controller
    fn before_controller<C: Controller>(&self, controller: &C, method_name: &str) {}

    /// This is being run when either the before_controller method or the
    /// controller method itself is throwing an exception. The middleware is
    /// asked in reverse order to handle the exception and to return a response.
    /// If the response is None, it is assumed that the exception could not be
    /// handled and the error will be thrown again
    ///
    /// # Arguments
    /// * `controller` - the controller that is being called
    /// * `method_name` - the name of the method that will be called on the controller
    /// * `error` - the thrown exception
    ///
    /// # Returns
    /// * `Option<Response>` - a Response object in case that the exception was handled or None
    fn after_exception<C: Controller, E: Error>(
        &self,
        controller: &C, 
        method_name: &str, 
        error: E
    ) -> Option<Response> {
        None
    }

    /// This is being run after a successful controllermethod call and allows
    /// the manipulation of a Response object. The middleware is run in reverse order
    ///
    /// # Arguments
    /// * `controller` - the controller that is being called
    /// * `method_name` - the name of the method that will be called on the controller
    /// * `response` - the generated response from the controller
    ///
    /// # Returns
    /// * `Response` - a Response object
    fn after_controller<C: Controller>(
        &self,
        controller: &C, 
        method_name: &str, 
        response: Response
    ) -> Response {
        response
    }

    /// This is being run after the response object has been rendered and
    /// allows the manipulation of the output. The middleware is run in reverse order
    ///
    /// # Arguments
    /// * `controller` - the controller that is being called
    /// * `method_name` - the name of the method that will be called on the controller
    /// * `output` - the generated output from a response
    ///
    /// # Returns
    /// * `String` - the output that should be printed
    fn before_output<C: Controller>(
        &self,
        controller: &C, 
        method_name: &str, 
        output: String
    ) -> String {
        output
    }
}

// These are placeholder types that would be defined elsewhere in the application
pub trait Controller {}
pub struct Response {}