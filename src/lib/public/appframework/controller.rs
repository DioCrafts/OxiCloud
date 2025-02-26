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

use std::collections::HashMap;

/// Interface for the app container
pub trait IAppContainer {
    /// Get the app name
    fn get_app_name(&self) -> &str;
}

/// Interface for HTTP requests
pub trait IRequest {
    /// Get a parameter from the request
    fn get_param<T>(&self, key: &str, default: Option<T>) -> Option<T>;
    
    /// Get all parameters from the request
    fn get_params(&self) -> HashMap<String, String>;
    
    /// Get the HTTP method
    fn get_method(&self) -> String;
    
    /// Get an uploaded file
    fn get_uploaded_file(&self, key: &str) -> Option<HashMap<String, String>>;
    
    /// Get an environment variable
    fn get_env(&self, key: &str) -> Option<String>;
    
    /// Get a cookie
    fn get_cookie(&self, key: &str) -> Option<String>;
}

/// Template response for rendering views
pub struct TemplateResponse {
    app_name: String,
    template_name: String,
    params: HashMap<String, String>,
    render_as: String,
    headers: HashMap<String, String>,
}

impl TemplateResponse {
    /// Create a new template response
    pub fn new(app_name: String, template_name: String) -> Self {
        Self {
            app_name,
            template_name,
            params: HashMap::new(),
            render_as: "user".to_string(),
            headers: HashMap::new(),
        }
    }
    
    /// Set the parameters for the template
    pub fn set_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }
    
    /// Set the render mode
    pub fn render_as(&mut self, render_as: &str) {
        self.render_as = render_as.to_string();
    }
    
    /// Add a header to the response
    pub fn add_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_string(), value.to_string());
    }
}

/// Base class to inherit your controllers from
pub struct Controller<A: IAppContainer, R: IRequest> {
    app: A,
    request: R,
}

impl<A: IAppContainer, R: IRequest> Controller<A, R> {
    /// Create a new Controller instance
    ///
    /// # Arguments
    /// * `app` - interface to the app
    /// * `request` - an instance of the request
    pub fn new(app: A, request: R) -> Self {
        Self { app, request }
    }
    
    /// Lets you access post and get parameters by the index
    ///
    /// # Arguments
    /// * `key` - the key which you want to access in the URL Parameter
    ///           placeholder, $_POST or $_GET array.
    ///           The priority how they're returned is the following:
    ///           1. URL parameters
    ///           2. POST parameters
    ///           3. GET parameters
    /// * `default` - If the key is not found, this value will be returned
    ///
    /// # Returns
    /// The content of the array
    pub fn params<T>(&self, key: &str, default: Option<T>) -> Option<T> {
        self.request.get_param(key, default)
    }
    
    /// Returns all params that were received, be it from the request
    /// (as GET or POST) or throuh the URL by the route
    ///
    /// # Returns
    /// The array with all parameters
    pub fn get_params(&self) -> HashMap<String, String> {
        self.request.get_params()
    }
    
    /// Returns the method of the request
    ///
    /// # Returns
    /// The method of the request (POST, GET, etc)
    pub fn method(&self) -> String {
        self.request.get_method()
    }
    
    /// Shortcut for accessing an uploaded file
    ///
    /// # Arguments
    /// * `key` - the key that will be taken from the files array
    ///
    /// # Returns
    /// The file information
    pub fn get_uploaded_file(&self, key: &str) -> Option<HashMap<String, String>> {
        self.request.get_uploaded_file(key)
    }
    
    /// Shortcut for getting env variables
    ///
    /// # Arguments
    /// * `key` - the key that will be taken from the ENV array
    ///
    /// # Returns
    /// The value in the ENV element
    pub fn env(&self, key: &str) -> Option<String> {
        self.request.get_env(key)
    }
    
    /// Shortcut for getting cookie variables
    ///
    /// # Arguments
    /// * `key` - the key that will be taken from the COOKIE array
    ///
    /// # Returns
    /// The value in the COOKIE element
    pub fn cookie(&self, key: &str) -> Option<String> {
        self.request.get_cookie(key)
    }
    
    /// Shortcut for rendering a template
    ///
    /// # Arguments
    /// * `template_name` - the name of the template
    /// * `params` - the template parameters in key => value structure
    /// * `render_as` - user renders a full page, blank only your template
    ///                 admin an entry in the admin settings
    /// * `headers` - set additional headers in name/value pairs
    ///
    /// # Returns
    /// TemplateResponse containing the page
    pub fn render(
        &self,
        template_name: &str,
        params: HashMap<String, String>,
        render_as: &str,
        headers: HashMap<String, String>,
    ) -> TemplateResponse {
        let mut response = TemplateResponse::new(
            self.app.get_app_name().to_string(),
            template_name.to_string(),
        );
        
        response.set_params(params);
        response.render_as(render_as);
        
        for (name, value) in headers {
            response.add_header(&name, &value);
        }
        
        response
    }
}