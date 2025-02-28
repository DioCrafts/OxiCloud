// Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use std::fmt;
use std::path::Path;

/// A route implementation inspired by Symfony's Route
pub struct Route {
    path: String,
    defaults: HashMap<String, RouteValue>,
    requirements: HashMap<String, String>,
}

pub enum RouteValue {
    String(String),
    Action(Box<dyn Fn(HashMap<String, String>) -> Result<(), RouteError> + Send + Sync>),
}

impl fmt::Debug for RouteValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteValue::String(s) => write!(f, "String({})", s),
            RouteValue::Action(_) => write!(f, "Action(...)"),
        }
    }
}

#[derive(Debug)]
pub enum RouteError {
    NotFound,
    InvalidMethod,
    ActionFailed(String),
}

impl Route {
    /// Create a new route with the given path
    pub fn new(path: &str) -> Self {
        Route {
            path: path.to_string(),
            defaults: HashMap::new(),
            requirements: HashMap::new(),
        }
    }

    /// Specify the method when this route is to be used
    ///
    /// # Arguments
    /// * `method` - HTTP method (uppercase)
    pub fn method(&mut self, method: &str) -> &mut Self {
        self.requirements.insert("_method".to_string(), method.to_uppercase());
        self
    }

    /// Specify POST as the method to use with this route
    pub fn post(&mut self) -> &mut Self {
        self.method("POST")
    }

    /// Specify GET as the method to use with this route
    pub fn get(&mut self) -> &mut Self {
        self.method("GET")
    }

    /// Specify PUT as the method to use with this route
    pub fn put(&mut self) -> &mut Self {
        self.method("PUT")
    }

    /// Specify DELETE as the method to use with this route
    pub fn delete(&mut self) -> &mut Self {
        self.method("DELETE")
    }

    /// Specify PATCH as the method to use with this route
    pub fn patch(&mut self) -> &mut Self {
        self.method("PATCH")
    }

    /// Defaults to use for this route
    ///
    /// # Arguments
    /// * `defaults` - The defaults as a HashMap
    pub fn defaults(&mut self, defaults: HashMap<String, String>) -> &mut Self {
        // Save current action if exists
        let action = self.get_default_action();
        
        // Set new defaults
        for (key, value) in defaults {
            self.defaults.insert(key, RouteValue::String(value));
        }
        
        // Restore action if it was set before
        if let Some(action_value) = action {
            self.defaults.insert("action".to_string(), action_value);
        }
        
        self
    }

    /// Requirements for this route
    ///
    /// # Arguments
    /// * `requirements` - The requirements as a HashMap
    pub fn requirements(&mut self, requirements: HashMap<String, String>) -> &mut Self {
        // Save current method if exists
        let method = self.requirements.get("_method").cloned();
        
        // Set new requirements
        for (key, value) in requirements {
            self.requirements.insert(key, value);
        }
        
        // Restore method requirement if it was set before
        if let Some(method_value) = method {
            self.requirements.insert("_method".to_string(), method_value);
        }
        
        self
    }

    /// The action to execute when this route matches
    ///
    /// # Arguments
    /// * `action` - A closure that will be called when the route matches
    pub fn action<F>(&mut self, action: F) -> &mut Self 
    where
        F: Fn(HashMap<String, String>) -> Result<(), RouteError> + Send + Sync + 'static,
    {
        self.defaults.insert("action".to_string(), RouteValue::Action(Box::new(action)));
        self
    }

    /// The action to execute when this route matches, includes a file
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to include
    pub fn action_include(&mut self, file_path: &str) -> &mut Self {
        let file_path = file_path.to_string();
        self.action(move |mut params| {
            // Remove route parameter
            params.remove("_route");
            
            // In Rust we can't just include a file at runtime like in PHP
            // Here we'd need to use some form of plugin system or dynamic loading
            // This is a simplified version that just checks if the file exists
            if Path::new(&file_path).exists() {
                // In a real implementation, we'd use a plugin system or evaluate the file somehow
                Ok(())
            } else {
                Err(RouteError::NotFound)
            }
        })
    }

    // Helper methods for internal use

    fn get_default(&self, key: &str) -> Option<String> {
        match self.defaults.get(key) {
            Some(RouteValue::String(s)) => Some(s.clone()),
            _ => None,
        }
    }

    fn get_default_action(&self) -> Option<RouteValue> {
        self.defaults.get("action").cloned()
    }

    fn get_requirement(&self, key: &str) -> Option<String> {
        self.requirements.get(key).cloned()
    }
}