// Copyright (c) 2013 Thomas Müller deepdiver@owncloud.com
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
// Container interface

use std::any::Any;
use std::sync::Arc;

/// IContainer is the basic interface to be used for any internal dependency injection mechanism
pub trait IContainer {
    /// Look up a service for a given name in the container.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the service to look up
    ///
    /// # Returns
    ///
    /// The service instance or an error if the service could not be found
    fn query(&self, name: &str) -> Result<Arc<dyn Any + Send + Sync>, Box<dyn std::error::Error>>;

    /// A value is stored in the container with it's corresponding name
    ///
    /// # Arguments
    ///
    /// * `name` - The name to register the value under
    /// * `value` - The value to register
    fn register_parameter<T: 'static + Send + Sync>(&mut self, name: &str, value: T);

    /// A service is registered in the container where a closure is passed in which will actually
    /// create the service on demand.
    /// In case the parameter `shared` is set to true (the default usage) the once created service will remain in
    /// memory and be reused on subsequent calls.
    /// In case the parameter is false the service will be recreated on every call.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to register the service under
    /// * `factory` - The factory function that creates the service
    /// * `shared` - Whether the service should be shared (singleton) or created on each query
    fn register_service<F, T>(&mut self, name: &str, factory: F, shared: bool)
    where
        F: Fn() -> T + Send + Sync + 'static,
        T: 'static + Send + Sync;
}