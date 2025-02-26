/*
 * Copyright 2010 Google Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use chrono::{self, TimeZone, Utc};
use lazy_static::lazy_static;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

// Import required modules
mod auth;
mod cache;
mod config;
mod io;
mod service;

// Export necessary types
pub use auth::{
    AssertionCredentials, Auth, AuthException, LoginTicket, P12Signer, Signer,
};
pub use cache::Cache;
pub use io::IO;
pub use service::{
    BatchRequest, MediaFileUpload, Model, Service, ServiceResource,
};

// Define exceptions as custom error types
#[derive(Debug)]
pub struct GoogleException {
    message: String,
}

impl GoogleException {
    pub fn new(message: &str) -> Self {
        GoogleException {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for GoogleException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GoogleException: {}", self.message)
    }
}

impl Error for GoogleException {}

#[derive(Debug)]
pub struct CacheException {
    message: String,
}

impl CacheException {
    pub fn new(message: &str) -> Self {
        CacheException {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for CacheException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CacheException: {}", self.message)
    }
}

impl Error for CacheException {}

#[derive(Debug)]
pub struct IOException {
    message: String,
}

impl IOException {
    pub fn new(message: &str) -> Self {
        IOException {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for IOException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IOException: {}", self.message)
    }
}

impl Error for IOException {}

#[derive(Debug)]
pub struct ServiceException {
    message: String,
    code: i32,
    errors: Vec<HashMap<String, String>>,
}

impl ServiceException {
    pub fn new(message: &str, code: i32, errors: Vec<HashMap<String, String>>) -> Self {
        ServiceException {
            message: message.to_string(),
            code,
            errors,
        }
    }

    pub fn get_errors(&self) -> &Vec<HashMap<String, String>> {
        &self.errors
    }
}

impl fmt::Display for ServiceException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ServiceException: {} (code: {})", self.message, self.code)
    }
}

impl Error for ServiceException {}

// Main client struct for Google API
pub struct GoogleClient {
    auth: Arc<Mutex<Box<dyn Auth>>>,
    cache: Arc<Mutex<Box<dyn Cache>>>,
    io: Arc<Mutex<Box<dyn IO>>>,
    scopes: Vec<String>,
    use_objects: bool,
    services: HashMap<String, HashMap<String, String>>,
    authenticated: bool,
}

// Static configuration
lazy_static! {
    static ref USE_BATCH: Mutex<bool> = Mutex::new(false);
    static ref API_CONFIG: Mutex<HashMap<String, serde_json::Value>> = {
        let mut config = HashMap::new();
        // Load default configuration from config
        let default_config = config::get_default_config();
        for (k, v) in default_config {
            config.insert(k, v);
        }
        
        // Try to load local configuration if exists
        let local_config_path = Path::new("local_config.json");
        if local_config_path.exists() {
            if let Ok(local_config_str) = fs::read_to_string(local_config_path) {
                if let Ok(local_config) = serde_json::from_str::<HashMap<String, serde_json::Value>>(&local_config_str) {
                    for (k, v) in local_config {
                        config.insert(k, v);
                    }
                }
            }
        }
        
        Mutex::new(config)
    };
}

impl GoogleClient {
    pub fn new(config: Option<HashMap<String, serde_json::Value>>) -> Result<Self, Box<dyn Error>> {
        // Check for required dependencies
        if !Self::check_requirements() {
            return Err(Box::new(GoogleException::new(
                "Missing required dependencies for Google API Client",
            )));
        }

        // Set default timezone if not set
        if chrono::Local::now().timezone() == chrono::FixedOffset::east(0) {
            // Default to UTC if timezone is not set
            // (This is just a placeholder as Rust handles timezones differently)
        }

        // Update config with provided values
        if let Some(cfg) = config {
            let mut api_config = API_CONFIG.lock().unwrap();
            for (k, v) in cfg {
                api_config.insert(k, v);
            }
        }

        let api_config = API_CONFIG.lock().unwrap();
        
        // Create the cache, auth, and io instances
        let cache_class = api_config.get("cache_class").and_then(|v| v.as_str()).unwrap_or("cache::FileCache");
        let auth_class = api_config.get("auth_class").and_then(|v| v.as_str()).unwrap_or("auth::OAuth2");
        let io_class = api_config.get("io_class").and_then(|v| v.as_str()).unwrap_or("io::HttpIO");

        let cache: Box<dyn Cache> = match cache_class {
            "cache::FileCache" => Box::new(cache::FileCache::new()),
            _ => Box::new(cache::FileCache::new()), // Default
        };

        let auth: Box<dyn Auth> = match auth_class {
            "auth::OAuth2" => Box::new(auth::OAuth2::new()),
            _ => Box::new(auth::OAuth2::new()), // Default
        };

        let io: Box<dyn IO> = match io_class {
            "io::HttpIO" => Box::new(io::HttpIO::new()),
            _ => Box::new(io::HttpIO::new()), // Default
        };

        Ok(GoogleClient {
            auth: Arc::new(Mutex::new(auth)),
            cache: Arc::new(Mutex::new(cache)),
            io: Arc::new(Mutex::new(io)),
            scopes: Vec::new(),
            use_objects: false,
            services: HashMap::new(),
            authenticated: false,
        })
    }

    // Check required dependencies
    fn check_requirements() -> bool {
        // In Rust, we'll rely on cargo dependencies instead of runtime checks
        true
    }

    pub fn add_service(&mut self, service: &str, version: Option<&str>) -> Result<(), Box<dyn Error>> {
        if self.authenticated {
            return Err(Box::new(GoogleException::new(
                "Can't add services after having authenticated",
            )));
        }

        self.services.insert(service.to_string(), HashMap::new());
        
        // Merge with service config if available
        let api_config = API_CONFIG.lock().unwrap();
        if let Some(serde_json::Value::Object(service_config)) = api_config.get("services").and_then(|s| s.get(service)) {
            if let Some(service_map) = self.services.get_mut(service) {
                for (k, v) in service_config {
                    if let Some(v_str) = v.as_str() {
                        service_map.insert(k.clone(), v_str.to_string());
                    }
                }
            }
        }

        Ok(())
    }

    pub fn authenticate(&mut self, code: Option<&str>) -> Result<bool, Box<dyn Error>> {
        let service = self.prepare_service();
        self.authenticated = true;
        let mut auth = self.auth.lock().unwrap();
        auth.authenticate(&service, code.map(|s| s.to_string()))
    }

    pub fn prepare_service(&self) -> HashMap<String, String> {
        let mut service = HashMap::new();
        let mut scopes = Vec::new();

        if !self.scopes.is_empty() {
            scopes = self.scopes.clone();
        } else {
            for (key, val) in &self.services {
                if let Some(scope) = val.get("scope") {
                    if scope.contains(' ') {
                        // If it's a space-delimited list, split and add each
                        for s in scope.split(' ') {
                            scopes.push(s.to_string());
                        }
                    } else {
                        scopes.push(scope.clone());
                    }
                } else {
                    scopes.push(format!("https://www.googleapis.com/auth/{}", key));
                }

                // Copy values from service to the service object, excluding some keys
                for (k, v) in val {
                    if k != "discoveryURI" && k != "scope" {
                        service.insert(k.clone(), v.clone());
                    }
                }
            }
        }

        service.insert("scope".to_string(), scopes.join(" "));
        service
    }

    pub fn set_access_token(&self, access_token: Option<&str>) -> Result<(), Box<dyn Error>> {
        let token = match access_token {
            Some(token) if token == "null" => None,
            Some(token) => Some(token),
            None => None,
        };

        let mut auth = self.auth.lock().unwrap();
        auth.set_access_token(token.map(|s| s.to_string()))
    }

    pub fn set_auth_class(&mut self, auth_class_name: &str) -> Result<(), Box<dyn Error>> {
        // In Rust, we'd need to instantiate the auth class differently
        // This is a simplification
        let new_auth: Box<dyn Auth> = match auth_class_name {
            "auth::OAuth2" => Box::new(auth::OAuth2::new()),
            _ => return Err(Box::new(GoogleException::new("Unknown auth class"))),
        };

        self.auth = Arc::new(Mutex::new(new_auth));
        Ok(())
    }

    pub fn create_auth_url(&self) -> Result<String, Box<dyn Error>> {
        let service = self.prepare_service();
        let scope = service.get("scope").unwrap_or(&"".to_string()).clone();
        let auth = self.auth.lock().unwrap();
        auth.create_auth_url(&scope)
    }

    pub fn get_access_token(&self) -> Result<Option<String>, Box<dyn Error>> {
        let auth = self.auth.lock().unwrap();
        let token = auth.get_access_token()?;
        Ok(match token {
            Some(t) if t == "null" => None,
            Some(t) => Some(t),
            None => None,
        })
    }

    pub fn is_access_token_expired(&self) -> Result<bool, Box<dyn Error>> {
        let auth = self.auth.lock().unwrap();
        auth.is_access_token_expired()
    }

    pub fn set_developer_key(&self, developer_key: &str) -> Result<(), Box<dyn Error>> {
        let mut auth = self.auth.lock().unwrap();
        auth.set_developer_key(developer_key.to_string())
    }

    pub fn set_state(&self, state: &str) -> Result<(), Box<dyn Error>> {
        let mut auth = self.auth.lock().unwrap();
        auth.set_state(state.to_string())
    }

    pub fn set_access_type(&self, access_type: &str) -> Result<(), Box<dyn Error>> {
        let mut auth = self.auth.lock().unwrap();
        auth.set_access_type(access_type.to_string())
    }

    pub fn set_approval_prompt(&self, approval_prompt: &str) -> Result<(), Box<dyn Error>> {
        let mut auth = self.auth.lock().unwrap();
        auth.set_approval_prompt(approval_prompt.to_string())
    }

    pub fn set_application_name(&self, application_name: &str) -> Result<(), Box<dyn Error>> {
        let mut api_config = API_CONFIG.lock().unwrap();
        api_config.insert(
            "application_name".to_string(),
            serde_json::Value::String(application_name.to_string()),
        );
        Ok(())
    }

    pub fn set_client_id(&self, client_id: &str) -> Result<(), Box<dyn Error>> {
        {
            let mut api_config = API_CONFIG.lock().unwrap();
            api_config.insert(
                "oauth2_client_id".to_string(),
                serde_json::Value::String(client_id.to_string()),
            );
        }
        let mut auth = self.auth.lock().unwrap();
        auth.set_client_id(client_id.to_string())
    }

    pub fn get_client_id(&self) -> Result<Option<String>, Box<dyn Error>> {
        let auth = self.auth.lock().unwrap();
        auth.get_client_id()
    }

    pub fn set_client_secret(&self, client_secret: &str) -> Result<(), Box<dyn Error>> {
        {
            let mut api_config = API_CONFIG.lock().unwrap();
            api_config.insert(
                "oauth2_client_secret".to_string(),
                serde_json::Value::String(client_secret.to_string()),
            );
        }
        let mut auth = self.auth.lock().unwrap();
        auth.set_client_secret(client_secret.to_string())
    }

    pub fn get_client_secret(&self) -> Result<Option<String>, Box<dyn Error>> {
        let auth = self.auth.lock().unwrap();
        auth.get_client_secret()
    }

    pub fn set_redirect_uri(&self, redirect_uri: &str) -> Result<(), Box<dyn Error>> {
        {
            let mut api_config = API_CONFIG.lock().unwrap();
            api_config.insert(
                "oauth2_redirect_uri".to_string(),
                serde_json::Value::String(redirect_uri.to_string()),
            );
        }
        let mut auth = self.auth.lock().unwrap();
        auth.set_redirect_uri(redirect_uri.to_string())
    }

    pub fn get_redirect_uri(&self) -> Result<Option<String>, Box<dyn Error>> {
        let auth = self.auth.lock().unwrap();
        auth.get_redirect_uri()
    }

    pub fn refresh_token(&self, refresh_token: &str) -> Result<(), Box<dyn Error>> {
        let mut auth = self.auth.lock().unwrap();
        auth.refresh_token(refresh_token.to_string())
    }

    pub fn revoke_token(&self, token: Option<&str>) -> Result<bool, Box<dyn Error>> {
        let mut auth = self.auth.lock().unwrap();
        auth.revoke_token(token.map(|s| s.to_string()))
    }

    pub fn verify_id_token(&self, token: Option<&str>) -> Result<LoginTicket, Box<dyn Error>> {
        let mut auth = self.auth.lock().unwrap();
        auth.verify_id_token(token.map(|s| s.to_string()))
    }

    pub fn set_assertion_credentials(&self, creds: AssertionCredentials) -> Result<(), Box<dyn Error>> {
        let mut auth = self.auth.lock().unwrap();
        auth.set_assertion_credentials(creds)
    }

    pub fn set_scopes(&mut self, scopes: &str) -> Result<(), Box<dyn Error>> {
        if scopes.contains(' ') {
            self.scopes = scopes.split(' ').map(|s| s.to_string()).collect();
        } else {
            self.scopes = vec![scopes.to_string()];
        }
        Ok(())
    }

    pub fn get_scopes(&self) -> Vec<String> {
        self.scopes.clone()
    }

    pub fn set_use_objects(&mut self, use_objects: bool) -> Result<(), Box<dyn Error>> {
        self.use_objects = use_objects;
        let mut api_config = API_CONFIG.lock().unwrap();
        api_config.insert(
            "use_objects".to_string(),
            serde_json::Value::Bool(use_objects),
        );
        Ok(())
    }

    pub fn set_use_batch(&self, use_batch: bool) -> Result<(), Box<dyn Error>> {
        let mut batch = USE_BATCH.lock().unwrap();
        *batch = use_batch;
        Ok(())
    }

    pub fn get_auth(&self) -> Arc<Mutex<Box<dyn Auth>>> {
        self.auth.clone()
    }

    pub fn get_io(&self) -> Arc<Mutex<Box<dyn IO>>> {
        self.io.clone()
    }

    pub fn get_cache(&self) -> Arc<Mutex<Box<dyn Cache>>> {
        self.cache.clone()
    }
}