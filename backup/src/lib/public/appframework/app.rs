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

use std::collections::HashMap;
use crate::core::appframework::dependency_injection::di_container::DIContainer;
use crate::core::appframework::app as core_app;

/// App container interface that provides methods for dependency injection
pub trait IAppContainer {
    fn register_service<F, T>(&mut self, name: &str, factory: F) 
    where 
        F: Fn(&Self) -> T + 'static,
        T: 'static;
    
    fn query<T: 'static>(&self, name: &str) -> &T;
}

/// Any application must inherit this call - all controller instances to be used are
/// to be registered using IContainer::register_service
pub struct App {
    container: DIContainer,
}

impl App {
    /// Creates a new App instance
    ///
    /// # Arguments
    ///
    /// * `app_name` - The name of the application
    /// * `url_params` - An array with variables extracted from the routes
    pub fn new(app_name: String, url_params: Option<HashMap<String, String>>) -> Self {
        let url_params = url_params.unwrap_or_else(HashMap::new);
        Self {
            container: DIContainer::new(app_name, url_params),
        }
    }

    /// Returns the container instance
    pub fn get_container(&self) -> &DIContainer {
        &self.container
    }

    /// This function is called by the routing component to fire up the frameworks dispatch mechanism.
    ///
    /// Example code in routes.rs of the task app:
    ///
    /// router.create("tasks_index", "/").get().action(|params| {
    ///     let app = TaskApp::new(params);
    ///     app.dispatch("PageController", "index");
    /// });
    ///
    ///
    /// Example for TaskApp implementation:
    ///
    /// pub struct TaskApp {
    ///     app: App
    /// }
    ///
    /// impl TaskApp {
    ///     pub fn new(params: HashMap<String, String>) -> Self {
    ///         let mut app = App::new("tasks".to_string(), Some(params));
    ///         
    ///         app.get_container().register_service("PageController", |c| {
    ///             let a = c.query::<Api>("API");
    ///             let r = c.query::<Request>("Request");
    ///             PageController::new(a, r)
    ///         });
    ///         
    ///         Self { app }
    ///     }
    /// }
    ///
    ///
    /// # Arguments
    ///
    /// * `controller_name` - The name of the controller under which it is stored in the DI container
    /// * `method_name` - The method that you want to call
    pub fn dispatch(&self, controller_name: &str, method_name: &str) {
        core_app::main(controller_name, method_name, &self.container);
    }
}