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

use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::env;
use std::path::PathBuf;

pub struct ApiConfig {
    /// True if objects should be returned by the service classes.
    /// False if associative arrays should be returned (default behavior).
    pub use_objects: bool,
    
    /// The application_name is included in the User-Agent HTTP header.
    pub application_name: String,

    /// OAuth2 Settings, you can get these keys at https://code.google.com/apis/console
    pub oauth2_client_id: String,
    pub oauth2_client_secret: String,
    pub oauth2_redirect_uri: String,

    /// The developer key, you get this at https://code.google.com/apis/console
    pub developer_key: String,
    
    /// Site name to show in the Google's OAuth 1 authentication screen.
    pub site_name: String,

    /// Which Authentication, Storage and HTTP IO classes to use.
    pub auth_class: String,
    pub io_class: String,
    pub cache_class: String,

    /// Don't change these unless you're working against a special development or testing environment.
    pub base_path: String,

    /// IO Class dependent configuration
    pub io_file_cache_directory: PathBuf,

    /// Definition of service specific values like scopes, oauth token URLs, etc
    pub services: HashMap<String, ServiceConfig>,
}

pub struct ServiceConfig {
    pub scope: Vec<String>,
}

pub static API_CONFIG: Lazy<ApiConfig> = Lazy::new(|| {
    let temp_dir = env::temp_dir().join("Google_Client");
    
    let mut services = HashMap::new();
    
    // Analytics
    services.insert(
        "analytics".to_string(), 
        ServiceConfig {
            scope: vec!["https://www.googleapis.com/auth/analytics.readonly".to_string()],
        }
    );
    
    // Calendar
    services.insert(
        "calendar".to_string(), 
        ServiceConfig {
            scope: vec![
                "https://www.googleapis.com/auth/calendar".to_string(),
                "https://www.googleapis.com/auth/calendar.readonly".to_string(),
            ],
        }
    );
    
    // Books
    services.insert(
        "books".to_string(), 
        ServiceConfig {
            scope: vec!["https://www.googleapis.com/auth/books".to_string()],
        }
    );
    
    // Latitude
    services.insert(
        "latitude".to_string(), 
        ServiceConfig {
            scope: vec![
                "https://www.googleapis.com/auth/latitude.all.best".to_string(),
                "https://www.googleapis.com/auth/latitude.all.city".to_string(),
            ],
        }
    );
    
    // Moderator
    services.insert(
        "moderator".to_string(), 
        ServiceConfig {
            scope: vec!["https://www.googleapis.com/auth/moderator".to_string()],
        }
    );
    
    // OAuth2
    services.insert(
        "oauth2".to_string(), 
        ServiceConfig {
            scope: vec![
                "https://www.googleapis.com/auth/userinfo.profile".to_string(),
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ],
        }
    );
    
    // Plus
    services.insert(
        "plus".to_string(), 
        ServiceConfig {
            scope: vec!["https://www.googleapis.com/auth/plus.login".to_string()],
        }
    );
    
    // SiteVerification
    services.insert(
        "siteVerification".to_string(), 
        ServiceConfig {
            scope: vec!["https://www.googleapis.com/auth/siteverification".to_string()],
        }
    );
    
    // Tasks
    services.insert(
        "tasks".to_string(), 
        ServiceConfig {
            scope: vec!["https://www.googleapis.com/auth/tasks".to_string()],
        }
    );
    
    // Urlshortener
    services.insert(
        "urlshortener".to_string(), 
        ServiceConfig {
            scope: vec!["https://www.googleapis.com/auth/urlshortener".to_string()],
        }
    );
    
    ApiConfig {
        use_objects: false,
        application_name: String::new(),
        oauth2_client_id: String::new(),
        oauth2_client_secret: String::new(),
        oauth2_redirect_uri: String::new(),
        developer_key: String::new(),
        site_name: "www.example.org".to_string(),
        auth_class: "Google_OAuth2".to_string(),
        io_class: "Google_CurlIO".to_string(),
        cache_class: "Google_FileCache".to_string(),
        base_path: "https://www.googleapis.com".to_string(),
        io_file_cache_directory: temp_dir,
        services,
    }
});