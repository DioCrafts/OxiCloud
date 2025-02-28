// Módulos generados automáticamente

pub mod jsresourcelocator;
pub mod templatefilelocator;
pub mod resourcelocator;
pub mod cssresourcelocator;
pub mod functions;
pub mod base;

// Contenido fusionado desde src/lib/private/template.rs
// Copyright (c) 2012 Frank Karlitschek frank@owncloud.org
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
use std::path::{Path, PathBuf};
use std::sync::Arc;
use async_trait::async_trait;

mod template_functions;
use crate::template::base::Base as TemplateBase;
use crate::l10n::L10n;
use crate::defaults::Defaults;
use crate::util::Util;
use crate::config::Config;
use crate::session::Session;
use crate::template::template_file_locator::TemplateFileLocator;
use crate::template::template_layout::TemplateLayout;
use crate::app::App;

/// This struct provides the templates for ownCloud.
pub struct Template {
    render_as: String,          // Create a full page?
    path: PathBuf,              // The path to the template
    headers: Vec<Header>,       // Custom headers
    base: TemplateBase,
}

#[derive(Clone, Debug)]
pub struct Header {
    tag: String,
    attributes: HashMap<String, String>,
    text: String,
}

impl Template {
    /// Creates a new Template object.
    ///
    /// If `render_as` is set, Template will try to produce a full page in the
    /// according layout. For now, render_as can be set to "guest", "user" or
    /// "admin".
    ///
    /// # Arguments
    /// * `app` - app providing the template
    /// * `name` - name of the template file (without suffix)
    /// * `render_as` - produce a full page
    pub fn new(app: &str, name: &str, render_as: &str, session: Option<Arc<Session>>) -> Self {
        // Read the selected theme from the config file
        let theme = Util::get_theme();

        // Read the detected formfactor and use the right file name.
        let fext = Self::get_form_factor_extension(session.clone());

        let request_token = if let Some(s) = session {
            Util::call_register()
        } else {
            String::new()
        };

        // Fix translation when app is something like core/lostpassword
        let parts: Vec<&str> = app.split('/').collect();
        let l10n = L10n::get(parts[0]);
        let theme_defaults = Defaults::new();

        let (path, template) = Self::find_template(&theme, app, name, &fext);

        // Set the private data
        let render_as = render_as.to_string();
        let path = path.to_path_buf();

        let base = TemplateBase::new(&template, &request_token, l10n, theme_defaults);

        let mut template = Self {
            render_as,
            path,
            headers: Vec::new(),
            base,
        };

        // Some headers to enhance security
        template.add_security_headers();
        
        template
    }

    fn add_security_headers(&mut self) {
        // Enforce browser based XSS filters
        self.add_header("X-XSS-Protection", HashMap::from([("content".to_string(), "1; mode=block".to_string())]), "");
        
        // Disable sniffing the content type for IE
        self.add_header("X-Content-Type-Options", HashMap::from([("content".to_string(), "nosniff".to_string())]), "");

        // iFrame Restriction Policy
        let xframe_policy = Config::get_value::<bool>("xframe_restriction", true);
        if xframe_policy {
            self.add_header("X-Frame-Options", HashMap::from([("content".to_string(), "Sameorigin".to_string())]), "");
        }
        
        // Content Security Policy
        // If you change the standard policy, please also change it in config.sample.rs
        let policy = Config::get_value::<String>("custom_csp_policy", 
            "default-src 'self'; \
            script-src 'self' 'unsafe-eval'; \
            style-src 'self' 'unsafe-inline'; \
            frame-src *; \
            img-src *; \
            font-src 'self' data:; \
            media-src *".to_string());
        
        self.add_header("Content-Security-Policy", HashMap::from([("content".to_string(), policy)]), "");
    }

    /// Autodetect the formfactor of the used device
    /// * default -> the normal desktop browser interface
    /// * mobile -> interface for smartphones
    /// * tablet -> interface for tablets
    /// * standalone -> the default interface but without header, footer and
    ///   sidebar, just the application. Useful to use just a specific
    ///   app on the desktop in a standalone window.
    pub fn detect_formfactor() -> String {
        // please add more useragent strings for other devices
        if let Ok(user_agent) = std::env::var("HTTP_USER_AGENT") {
            let user_agent = user_agent.to_lowercase();
            
            if user_agent.contains("ipad") {
                "tablet".to_string()
            } else if user_agent.contains("iphone") {
                "mobile".to_string()
            } else if user_agent.contains("n9") && user_agent.contains("nokia") {
                "mobile".to_string()
            } else {
                "default".to_string()
            }
        } else {
            "default".to_string()
        }
    }

    /// Returns the formfactor extension for current formfactor
    pub fn get_form_factor_extension(session: Option<Arc<Session>>) -> String {
        if session.is_none() {
            return String::new();
        }
        
        let session = session.unwrap();
        
        // If the formfactor is not yet autodetected do the
        // autodetection now. For possible formfactors check the
        // detectFormfactor documentation
        if !session.exists("formfactor") {
            session.set("formfactor", Self::detect_formfactor());
        }
        
        // Allow manual override via GET parameter
        if let Ok(formfactor) = std::env::var("formfactor") {
            session.set("formfactor", formfactor);
        }
        
        let formfactor = session.get("formfactor");
        
        match formfactor.as_str() {
            "default" => String::new(),
            "mobile" => ".mobile".to_string(),
            "tablet" => ".tablet".to_string(),
            "standalone" => ".standalone".to_string(),
            _ => String::new(),
        }
    }

    /// Find the template with the given name
    /// 
    /// Will select the template file for the selected theme and formfactor.
    /// Checking all the possible locations.
    ///
    /// # Arguments
    /// * `theme` - the theme to use
    /// * `app` - the application providing the template
    /// * `name` - name of the template file (without suffix)
    /// * `fext` - form factor extension
    fn find_template(theme: &str, app: &str, name: &str, fext: &str) -> (PathBuf, String) {
        // Check if it is an app template or not.
        let dirs = if !app.is_empty() {
            Self::get_app_template_dirs(theme, app, &PathBuf::from("OC_SERVERROOT"), &App::get_app_path(app))
        } else {
            Self::get_core_template_dirs(theme, &PathBuf::from("OC_SERVERROOT"))
        };
        
        let locator = TemplateFileLocator::new(fext, dirs);
        let template = locator.find(name);
        let path = locator.get_path();
        
        (path, template)
    }

    fn get_app_template_dirs(_theme: &str, _app: &str, _server_root: &Path, _app_path: &Path) -> Vec<PathBuf> {
        // Implementation depends on the actual directory structure
        // This is a placeholder implementation
        vec![PathBuf::from("/templates")]
    }

    fn get_core_template_dirs(_theme: &str, _server_root: &Path) -> Vec<PathBuf> {
        // Implementation depends on the actual directory structure
        // This is a placeholder implementation
        vec![PathBuf::from("/core/templates")]
    }

    /// Add a custom element to the header
    ///
    /// # Arguments
    /// * `tag` - tag name of the element
    /// * `attributes` - hashmap of attributes for the element
    /// * `text` - the text content for the element
    pub fn add_header(&mut self, tag: &str, attributes: HashMap<String, String>, text: &str) {
        self.headers.push(Header {
            tag: tag.to_string(),
            attributes,
            text: text.to_string(),
        });
    }

    /// Process the template
    ///
    /// This function processes the template. If `render_as` is set, it
    /// will produce a full page.
    pub fn fetch_page(&self) -> String {
        let data = self.base.fetch_page();

        if !self.render_as.is_empty() {
            let mut page = TemplateLayout::new(&self.render_as);

            // Add custom headers
            page.assign("headers", self.headers.clone(), false);
            
            for header in Util::get_headers() {
                page.append("headers", header);
            }

            page.assign("content", data, false);
            page.fetch_page()
        } else {
            data
        }
    }

    /// Include template
    ///
    /// Includes another template. use `let included = self.inc("template");` to
    /// do this.
    pub fn inc(&self, file: &str, additional_params: Option<HashMap<String, String>>) -> String {
        let file_path = self.path.join(format!("{}.php", file));
        self.base.load(file_path.to_str().unwrap(), additional_params)
    }

    /// Shortcut to print a simple page for users
    ///
    /// # Arguments
    /// * `application` - The application we render the template for
    /// * `name` - Name of the template
    /// * `parameters` - Parameters for the template
    pub fn print_user_page(application: &str, name: &str, parameters: HashMap<String, String>) -> String {
        let session = Arc::new(Session::new());
        let mut content = Self::new(application, name, "user", Some(session));
        
        for (key, value) in parameters {
            content.base.assign(&key, value, true);
        }
        
        content.fetch_page()
    }

    /// Shortcut to print a simple page for admins
    ///
    /// # Arguments
    /// * `application` - The application we render the template for
    /// * `name` - Name of the template
    /// * `parameters` - Parameters for the template
    pub fn print_admin_page(application: &str, name: &str, parameters: HashMap<String, String>) -> String {
        let session = Arc::new(Session::new());
        let mut content = Self::new(application, name, "admin", Some(session));
        
        for (key, value) in parameters {
            content.base.assign(&key, value, true);
        }
        
        content.fetch_page()
    }

    /// Shortcut to print a simple page for guests
    ///
    /// # Arguments
    /// * `application` - The application we render the template for
    /// * `name` - Name of the template
    /// * `parameters` - Parameters for the template
    pub fn print_guest_page(application: &str, name: &str, parameters: HashMap<String, String>) -> String {
        let session = Arc::new(Session::new());
        let mut content = Self::new(application, name, "guest", Some(session));
        
        for (key, value) in parameters {
            content.base.assign(&key, value, true);
        }
        
        content.fetch_page()
    }

    /// Print a fatal error page and terminates the script
    ///
    /// # Arguments
    /// * `error_msg` - The error message to show
    /// * `hint` - An optional hint message
    /// Warning: All data passed to `hint` needs to get sanitized using Util::sanitize_html
    pub fn print_error_page(error_msg: &str, hint: &str) -> ! {
        let session = Arc::new(Session::new());
        let mut content = Self::new("", "error", "error", Some(session));
        
        let errors = vec![("error".to_string(), error_msg.to_string()), ("hint".to_string(), hint.to_string())];
        content.base.assign("errors", errors, true);
        
        println!("{}", content.fetch_page());
        std::process::exit(1);
    }
    
    /// Print error page using Exception details
    ///
    /// # Arguments
    /// * `exception` - The exception to display
    pub fn print_exception_error_page<E: std::error::Error>(exception: E) -> ! {
        let mut error_msg = exception.to_string();
        
        if let Some(code) = exception.source().map(|e| e.to_string()) {
            error_msg = format!("[{}] {}", code, error_msg);
        }

        let mut hint = String::new();
        if cfg!(debug_assertions) {
            if let Some(bt) = std::backtrace::Backtrace::capture().to_string().parse().ok() {
                if !bt.is_empty() {
                    hint = format!("<pre>{}</pre>", bt);
                }
            }
            
            let l = L10n::get("lib");
            // In Rust we don't have the same exception chaining mechanism,
            // so this part is simplified
            if let Some(source) = exception.source() {
                error_msg.push_str(&format!("<br/>{} ", l.t("Caused by:")));
                error_msg.push_str(&source.to_string());
            }
        } else {
            // Check if exception implements a hint trait
            // This would need to be implemented for specific error types
            // that correspond to OC\HintException in PHP
        }
        
        Self::print_error_page(&error_msg, &hint);
    }
}