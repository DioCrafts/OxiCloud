// ownCloud - App Framework
//
// @author Thomas Müller
// @copyright 2013 Thomas Müller thomas.mueller@tmit.eu
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

use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Error type for RouteConfig operations
#[derive(Debug)]
pub struct RouteConfigError {
    message: String,
}

impl RouteConfigError {
    fn new(message: &str) -> Self {
        RouteConfigError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for RouteConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RouteConfigError: {}", self.message)
    }
}

impl Error for RouteConfigError {}

/// The DIContainer trait defines the interface for dependency injection containers
pub trait DIContainer {
    fn get_app_name(&self) -> &str;
    fn get_service(&self, name: &str) -> Option<&dyn std::any::Any>;
}

/// The Router trait defines the interface for handling routes
pub trait Router {
    fn create(&mut self, name: &str, url: &str) -> Box<dyn Route>;
}

/// The Route trait defines the interface for route operations
pub trait Route {
    fn method(&mut self, verb: &str) -> &mut Self;
    fn action(&mut self, handler: RouteActionHandler) -> &mut Self;
}

/// RouteActionHandler handles route actions by invoking the controller
#[derive(Clone)]
pub struct RouteActionHandler {
    container_name: String,
    controller_name: String,
    action_name: String,
}

impl RouteActionHandler {
    pub fn new(container_name: String, controller_name: String, action_name: String) -> Self {
        RouteActionHandler {
            container_name,
            controller_name,
            action_name,
        }
    }
}

/// RouteConfig configures routes for an application
pub struct RouteConfig<'a> {
    container: &'a dyn DIContainer,
    router: &'a mut dyn Router,
    routes: HashMap<String, serde_json::Value>,
    app_name: String,
}

impl<'a> RouteConfig<'a> {
    /// Create a new RouteConfig
    ///
    /// # Arguments
    ///
    /// * `container` - The dependency injection container
    /// * `router` - The router
    /// * `routes` - The routes configuration
    pub fn new(
        container: &'a dyn DIContainer,
        router: &'a mut dyn Router,
        routes: HashMap<String, serde_json::Value>,
    ) -> Self {
        let app_name = container.get_app_name().to_string();
        RouteConfig {
            container,
            router,
            routes,
            app_name,
        }
    }

    /// Register routes with the router
    pub fn register(&mut self) -> Result<(), Box<dyn Error>> {
        // Process simple routes
        self.process_simple_routes()?;

        // Process resource routes
        self.process_resources()?;

        Ok(())
    }

    /// Process simple routes from the configuration
    fn process_simple_routes(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(serde_json::Value::Array(simple_routes)) = self.routes.get("routes") {
            for route_value in simple_routes {
                if let serde_json::Value::Object(route) = route_value {
                    let name = match route.get("name") {
                        Some(serde_json::Value::String(name)) => name,
                        _ => return Err(Box::new(RouteConfigError::new("Missing route name"))),
                    };

                    let url = match route.get("url") {
                        Some(serde_json::Value::String(url)) => url,
                        _ => return Err(Box::new(RouteConfigError::new("Missing route URL"))),
                    };

                    let verb = match route.get("verb") {
                        Some(serde_json::Value::String(verb)) => verb.to_uppercase(),
                        _ => "GET".to_string(),
                    };

                    let split: Vec<&str> = name.split('#').collect();
                    if split.len() != 2 {
                        return Err(Box::new(RouteConfigError::new("Invalid route name")));
                    }

                    let controller = split[0];
                    let action = split[1];

                    let controller_name = self.build_controller_name(controller);
                    let action_name = self.build_action_name(action);

                    let route_name = format!("{}.{}.{}", self.app_name, controller, action);
                    let handler = RouteActionHandler::new(
                        self.app_name.clone(),
                        controller_name,
                        action_name,
                    );

                    self.router
                        .create(&route_name, url)
                        .method(&verb)
                        .action(handler);
                }
            }
        }
        Ok(())
    }

    /// Process resource routes from the configuration
    fn process_resources(&mut self) -> Result<(), Box<dyn Error>> {
        // Define the standard RESTful actions
        let actions = vec![
            HashMap::from([
                ("name", "index"),
                ("verb", "GET"),
                ("on-collection", "true"),
            ]),
            HashMap::from([("name", "show"), ("verb", "GET")]),
            HashMap::from([
                ("name", "create"),
                ("verb", "POST"),
                ("on-collection", "true"),
            ]),
            HashMap::from([("name", "update"), ("verb", "PUT")]),
            HashMap::from([("name", "destroy"), ("verb", "DELETE")]),
        ];

        if let Some(serde_json::Value::Object(resources)) = self.routes.get("resources") {
            for (resource, config) in resources {
                let config = match config {
                    serde_json::Value::Object(config) => config,
                    _ => continue,
                };

                let url = match config.get("url") {
                    Some(serde_json::Value::String(url)) => url,
                    _ => continue,
                };

                // Generate the resource ID for URL parameters
                let resource_id = self.build_resource_id(resource);

                for action in &actions {
                    let method = action.get("name").unwrap();
                    let verb = action.get("verb").unwrap().to_uppercase();
                    let collection_action = action.get("on-collection").is_some();

                    let mut action_url = url.clone();
                    if !collection_action {
                        action_url = format!("{}/{}", action_url, resource_id);
                    }

                    if let Some(postfix) = action.get("url-postfix") {
                        action_url = format!("{}/{}", action_url, postfix);
                    }

                    let controller_name = self.build_controller_name(resource);
                    let action_name = self.build_action_name(method);

                    let route_name = format!(
                        "{}.{}.{}",
                        self.app_name,
                        resource.to_lowercase(),
                        method.to_lowercase()
                    );

                    let handler = RouteActionHandler::new(
                        self.app_name.clone(),
                        controller_name,
                        action_name,
                    );

                    self.router
                        .create(&route_name, &action_url)
                        .method(&verb)
                        .action(handler);
                }
            }
        }
        Ok(())
    }

    /// Build a controller name from a route part
    fn build_controller_name(&self, controller: &str) -> String {
        format!("{}Controller", self.under_score_to_camel_case(&controller.to_string().with_first_letter_uppercase()))
    }

    /// Build an action name from a route part
    fn build_action_name(&self, action: &str) -> String {
        self.under_score_to_camel_case(action)
    }

    /// Build a resource ID for URL parameters
    fn build_resource_id(&self, resource: &str) -> String {
        let resource = resource.trim_end_matches('s');
        format!("{{{}}}", self.under_score_to_camel_case(resource) + "Id")
    }

    /// Convert an underscored string to camel case
    fn under_score_to_camel_case(&self, input: &str) -> String {
        let re = Regex::new(r"_[a-z]?").unwrap();
        re.replace_all(input, |caps: &regex::Captures| {
            let matched = caps.get(0).unwrap().as_str();
            matched
                .trim_start_matches('_')
                .to_uppercase()
        }).to_string()
    }
}

/// Extension trait to capitalize the first letter of a string
trait StringExt {
    fn with_first_letter_uppercase(&self) -> String;
}

impl StringExt for String {
    fn with_first_letter_uppercase(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(first_char) => first_char.to_uppercase().to_string() + chars.as_str(),
        }
    }
}