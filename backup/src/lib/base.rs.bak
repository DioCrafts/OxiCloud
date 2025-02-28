// ownCloud
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
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use std::env;
use std::sync::{Arc, Mutex, RwLock};
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;
use url::Url;
use async_trait::async_trait;
use tokio::io;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use actix_web::{web, HttpRequest, HttpResponse, HttpServer, App, middleware};
use log::{error, warn, debug, info};

mod config;
mod util;
mod user;
mod app;
mod session;
mod template;
mod log_handler;
mod router;
mod minimizer;
mod file_system;
mod server;
mod autoloader;
mod cache;
mod preview;
mod share;

use config::Config;
use session::Session;
use router::Router;
use log_handler::LogHandler;
use user::{User, Group, UserBackend, GroupBackend};
use autoloader::Autoloader;
use server::Server;

#[derive(Error, Debug)]
pub enum OcError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Config error: {0}")]
    Config(String),
    
    #[error("Path error: {0}")]
    Path(String),
    
    #[error("Server error: {0}")]
    Server(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Third party directory not found")]
    ThirdPartyDirNotFound,
    
    #[error("Apps directory not found")]
    AppsDirNotFound,
    
    #[error("Session error: {0}")]
    Session(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Feature not implemented: {0}")]
    NotImplemented(String),
}

type Result<T> = std::result::Result<T, OcError>;

/// Class that is a namespace for all global OC variables
pub struct OC {
    /// Associative array for autoloading. classname => filename
    pub static class_path: RwLock<HashMap<String, String>>,
    
    /// The installation path for owncloud on the server (e.g. /srv/http/owncloud)
    pub static server_root: RwLock<String>,
    
    /// the current request path relative to the owncloud root (e.g. files/index.php)
    static sub_uri: RwLock<String>,
    
    /// the owncloud root path for http requests (e.g. owncloud/)
    pub static web_root: RwLock<String>,
    
    /// The installation path of the 3rdparty folder on the server
    pub static third_party_root: RwLock<String>,
    
    /// the root path of the 3rdparty folder for http requests
    pub static third_party_web_root: RwLock<String>,
    
    /// The installation path array of the apps folder on the server
    pub static apps_roots: RwLock<Vec<AppRoot>>,
    
    /// requested app
    pub static requested_app: RwLock<String>,
    
    /// requested file of app
    pub static requested_file: RwLock<Option<String>>,
    
    /// check if owncloud runs in cli mode
    pub static cli: RwLock<bool>,
    
    /// Router instance
    pub static router: RwLock<Option<Router>>,
    
    /// Session instance
    pub static session: RwLock<Option<Arc<dyn Session>>>,
    
    /// Autoloader instance
    pub static loader: RwLock<Option<Autoloader>>,
    
    /// Server instance
    pub static server: RwLock<Option<Server>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppRoot {
    pub path: PathBuf,
    pub url: String,
    pub writable: bool,
}

impl OC {
    pub fn init_paths() -> Result<()> {
        // calculate the root directories
        let current_dir = env::current_dir()?;
        let server_root = current_dir.parent().ok_or(OcError::Path("Could not get parent directory".to_string()))?;
        
        let server_root_str = server_root.to_string_lossy().replace("\\", "/");
        *Self::server_root.write().unwrap() = server_root_str.clone();
        
        // ensure we can find Config
        env::set_var("RUST_PATH", format!("{}:{}", 
            server_root.join("lib").to_string_lossy(),
            env::var("RUST_PATH").unwrap_or_default()
        ));
        
        // Set sub_uri
        let script_filename = env::var("SCRIPT_FILENAME").unwrap_or_default();
        let real_path = fs::canonicalize(script_filename)?;
        let sub_uri = real_path.strip_prefix(server_root)
            .map_err(|_| OcError::Path("Failed to strip prefix".to_string()))?
            .to_string_lossy()
            .replace("\\", "/");
        *Self::sub_uri.write().unwrap() = sub_uri.clone();
        
        // Get script_name
        let script_name = util::Request::script_name();
        let mut script_name = script_name.as_str();
        
        if script_name.ends_with('/') {
            script_name = format!("{}index.php", script_name).as_str();
            
            // make sure suburi follows the same rules as scriptName
            let mut sub_uri = Self::sub_uri.write().unwrap();
            if !sub_uri.ends_with("index.php") {
                if !sub_uri.ends_with('/') {
                    *sub_uri = format!("{}/", sub_uri);
                }
                *sub_uri = format!("{}index.php", sub_uri);
            }
        }
        
        // Set web_root
        let web_root = &script_name[0..script_name.len() - sub_uri.len()];
        let mut web_root_lock = Self::web_root.write().unwrap();
        *web_root_lock = web_root.to_string();
        
        if !web_root_lock.is_empty() && !web_root_lock.starts_with('/') {
            *web_root_lock = format!("/{}", web_root_lock);
        }
        
        // search the 3rdparty folder
        let third_party_root;
        let third_party_web_root;
        
        if !Config::get_value("3rdpartyroot", "").is_empty() && !Config::get_value("3rdpartyurl", "").is_empty() {
            third_party_root = Config::get_value("3rdpartyroot", "");
            third_party_web_root = Config::get_value("3rdpartyurl", "");
        } else if Path::new(&format!("{}/3rdparty", server_root_str)).exists() {
            third_party_root = server_root_str.clone();
            third_party_web_root = web_root_lock.clone();
        } else if Path::new(&format!("{}/3rdparty", server_root.parent().unwrap_or(server_root).to_string_lossy())).exists() {
            let parent_dir = server_root.parent().unwrap_or(server_root);
            third_party_web_root = util::rtrim(&dirname(&web_root_lock), "/").to_string();
            third_party_root = util::rtrim(&parent_dir.to_string_lossy(), "/").to_string();
        } else {
            return Err(OcError::ThirdPartyDirNotFound);
        }
        
        *Self::third_party_root.write().unwrap() = third_party_root;
        *Self::third_party_web_root.write().unwrap() = third_party_web_root;
        
        // search the apps folder
        let config_paths: Vec<AppRoot> = Config::get_value("apps_paths", Vec::new());
        let mut apps_roots = Self::apps_roots.write().unwrap();
        
        if !config_paths.is_empty() {
            for mut path in config_paths {
                if !path.url.is_empty() && !path.path.to_string_lossy().is_empty() {
                    path.url = util::rtrim(&path.url, "/").to_string();
                    let path_str = path.path.to_string_lossy().to_string();
                    path.path = PathBuf::from(util::rtrim(&path_str, "/"));
                    apps_roots.push(path);
                }
            }
        } else if Path::new(&format!("{}/apps", server_root_str)).exists() {
            apps_roots.push(AppRoot {
                path: PathBuf::from(format!("{}/apps", server_root_str)),
                url: "/apps".to_string(),
                writable: true,
            });
        } else if Path::new(&format!("{}/apps", server_root.parent().unwrap_or(server_root).to_string_lossy())).exists() {
            let parent_dir = server_root.parent().unwrap_or(server_root);
            apps_roots.push(AppRoot {
                path: PathBuf::from(format!("{}/apps", util::rtrim(&parent_dir.to_string_lossy(), "/"))),
                url: "/apps".to_string(),
                writable: true,
            });
        }
        
        if apps_roots.is_empty() {
            return Err(OcError::AppsDirNotFound);
        }
        
        let mut paths = Vec::new();
        for path in apps_roots.iter() {
            paths.push(path.path.to_string_lossy().to_string());
        }
        
        // set the right include path
        env::set_var("RUST_PATH", format!("{}:{}:{}:{}:{}:{}",
            format!("{}/lib/private", server_root_str),
            format!("{}/config", server_root_str),
            format!("{}/3rdparty", Self::third_party_root.read().unwrap()),
            paths.join(":"),
            env::var("RUST_PATH").unwrap_or_default(),
            server_root_str
        ));
        
        Ok(())
    }
    
    pub fn check_config() -> Result<()> {
        let server_root = Self::server_root.read().unwrap();
        let config_path = format!("{}/config/config.php", server_root);
        
        if Path::new(&config_path).exists() && !util::is_writable(&config_path) {
            let defaults = util::Defaults::new();
            template::Template::print_error_page(
                "Can't write into config directory!",
                &format!("This can usually be fixed by <a href=\"{}\" target=\"_blank\">giving the webserver write access to the config directory</a>.",
                    util::Helper::link_to_docs("admin-dir_permissions"))
            );
        }
        
        Ok(())
    }
    
    pub fn check_installed() -> Result<()> {
        // Redirect to installer if not installed
        if !Config::get_value("installed", false) && *Self::sub_uri.read().unwrap() != "/index.php" {
            if !*Self::cli.read().unwrap() {
                let server_name = env::var("SERVER_NAME").unwrap_or_default();
                let web_root = Self::web_root.read().unwrap();
                let url = format!("http://{}{}/index.php", server_name, web_root);
                
                // Set Location header in HTTP response
                // In Rust actual redirection would happen at the web framework level
                // This is a placeholder for that functionality
                println!("Location: {}", url);
            }
            std::process::exit(0);
        }
        
        Ok(())
    }
    
    pub fn check_ssl() -> Result<()> {
        // redirect to https site if configured
        if Config::get_value("forcessl", false) {
            // Set Strict-Transport-Security header
            println!("Strict-Transport-Security: max-age=31536000");
            
            // Set secure cookie
            // This would be handled differently in Rust
            
            if util::Request::server_protocol() != "https" && !*Self::cli.read().unwrap() {
                let host = util::Request::server_host();
                let request_uri = util::Request::request_uri();
                let url = format!("https://{}{}", host, request_uri);
                
                // Set Location header
                println!("Location: {}", url);
                std::process::exit(0);
            }
        } else if util::Request::server_protocol() == "https" {
            // Invalidate HSTS headers
            println!("Strict-Transport-Security: max-age=0");
        }
        
        Ok(())
    }
    
    pub fn check_maintenance_mode() -> Result<()> {
        // Allow ajax update script to execute without being stopped
        if Config::get_value("maintenance", false) && *Self::sub_uri.read().unwrap() != "/core/ajax/update.php" {
            // send http status 503
            println!("HTTP/1.1 503 Service Temporarily Unavailable");
            println!("Status: 503 Service Temporarily Unavailable");
            println!("Retry-After: 120");
            
            // render error page
            let tmpl = template::Template::new("", "update.user", "guest");
            tmpl.print_page();
            std::process::exit(0);
        }
        
        Ok(())
    }
    
    pub fn check_upgrade(show_template: bool) -> Result<bool> {
        if Config::get_value("installed", false) {
            let installed_version = Config::get_value("version", "0.0.0".to_string());
            let current_version = util::Util::get_version().join(".");
            
            if version_compare(&current_version, &installed_version, ">") {
                if show_template && !Config::get_value("maintenance", false) {
                    Config::set_value("theme", "".to_string());
                    
                    let minimizer_css = minimizer::MinimizerCss::new();
                    minimizer_css.clear_cache();
                    
                    let minimizer_js = minimizer::MinimizerJs::new();
                    minimizer_js.clear_cache();
                    
                    util::Util::add_script("update");
                    
                    let mut tmpl = template::Template::new("", "update.admin", "guest");
                    tmpl.assign("version", &util::Util::get_version_string());
                    tmpl.print_page();
                    
                    std::process::exit(0);
                } else {
                    return Ok(true);
                }
            }
            
            Ok(false)
        } else {
            Ok(false)
        }
    }
    
    pub fn init_template_engine() -> Result<()> {
        // Add the stuff we need always
        util::Util::add_script("jquery-1.10.0.min");
        util::Util::add_script("jquery-migrate-1.2.1.min");
        util::Util::add_script("jquery-ui-1.10.0.custom");
        util::Util::add_script("jquery-showpassword");
        util::Util::add_script("jquery.infieldlabel");
        util::Util::add_script("jquery.placeholder");
        util::Util::add_script("jquery-tipsy");
        util::Util::add_script("compatibility");
        util::Util::add_script("jquery.ocdialog");
        util::Util::add_script("oc-dialogs");
        util::Util::add_script("js");
        util::Util::add_script("octemplate");
        util::Util::add_script("eventsource");
        util::Util::add_script("config");
        util::Util::add_script("search", "result");
        util::Util::add_script("router");
        util::Util::add_script("oc-requesttoken");
        
        // avatars
        if Config::get_value("enable_avatars", true) {
            util::Util::add_script("placeholder");
            util::Util::add_script("3rdparty", "md5/md5.min");
            util::Util::add_script("jquery.avatar");
            util::Util::add_script("avatar");
        }
        
        util::Util::add_style("styles");
        util::Util::add_style("apps");
        util::Util::add_style("fixes");
        util::Util::add_style("multiselect");
        util::Util::add_style("jquery-ui-1.10.0.custom");
        util::Util::add_style("jquery-tipsy");
        util::Util::add_style("jquery.ocdialog");
        
        Ok(())
    }
    
    pub fn init_session() -> Result<()> {
        // prevents javascript from accessing php session cookies
        // This would be handled by the web framework in Rust
        
        // set the cookie path to the ownCloud directory
        let cookie_path = if Self::web_root.read().unwrap().is_empty() {
            "/".to_string()
        } else {
            Self::web_root.read().unwrap().clone()
        };
        
        // set the session object to a dummy session so code relying on the session existing still works
        let mut session_lock = Self::session.write().unwrap();
        *session_lock = Some(Arc::new(session::MemorySession::new("")));
        
        // try to establish a real session
        match session::InternalSession::new(&util::Util::get_instance_id()) {
            Ok(session) => {
                *session_lock = Some(Arc::new(session));
            },
            Err(e) => {
                // if session cant be started break with http 500 error
                util::Response::set_status(util::Response::STATUS_INTERNAL_SERVER_ERROR);
                template::Template::print_exception_error_page(&e);
                return Err(OcError::Session(e.to_string()));
            }
        }
        
        // Drop the write lock before proceeding
        drop(session_lock);
        
        // Now use a read lock to access the session
        let session = Self::session.read().unwrap();
        let session = session.as_ref().unwrap();
        
        let session_lifetime = Self::get_session_lifetime();
        
        // regenerate session id periodically to avoid session fixation
        if !session.exists("SID_CREATED") {
            session.set("SID_CREATED", &SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string());
        } else if let Ok(created) = session.get::<String>("SID_CREATED") {
            if let Ok(created) = created.parse::<u64>() {
                let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                if now - created > session_lifetime / 2 {
                    // In Rust, session regeneration would be handled differently
                    // This is a placeholder for that functionality
                    session.set("SID_CREATED", &now.to_string());
                }
            }
        }
        
        // session timeout
        if session.exists("LAST_ACTIVITY") {
            if let Ok(last_activity) = session.get::<String>("LAST_ACTIVITY") {
                if let Ok(last_activity) = last_activity.parse::<u64>() {
                    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                    if now - last_activity > session_lifetime {
                        // In a Rust web framework, cookie and session handling would be different
                        // This is a placeholder for that functionality
                    }
                }
            }
        }
        
        session.set("LAST_ACTIVITY", &SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string());
        
        Ok(())
    }
    
    fn get_session_lifetime() -> u64 {
        Config::get_value("session_lifetime", 60 * 60 * 24)
    }
    
    pub fn get_router() -> Result<Router> {
        let mut router = Self::router.write().unwrap();
        
        if router.is_none() {
            let mut new_router = Router::new();
            new_router.load_routes()?;
            *router = Some(new_router);
        }
        
        Ok(router.as_ref().unwrap().clone())
    }
    
    pub fn load_app_class_paths() -> Result<()> {
        for app in app::App::get_enabled_apps()? {
            let file = format!("{}/appinfo/classpath.php", app::App::get_app_path(&app)?);
            if Path::new(&file).exists() {
                // In Rust, we would handle this differently
                // This is a placeholder for the functionality
            }
        }
        
        Ok(())
    }
    
    pub fn init() -> Result<()> {
        // register autoloader
        let mut loader = Autoloader::new();
        loader.register_prefix("Doctrine\\Common", "doctrine/common/lib");
        loader.register_prefix("Doctrine\\DBAL", "doctrine/dbal/lib");
        loader.register_prefix("Symfony\\Component\\Routing", "symfony/routing");
        loader.register_prefix("Symfony\\Component\\Console", "symfony/console");
        loader.register_prefix("Sabre\\VObject", "3rdparty");
        loader.register_prefix("Sabre_", "3rdparty");
        loader.register_prefix("Patchwork", "3rdparty");
        
        // Store the loader in the static variable
        *Self::loader.write().unwrap() = Some(loader);
        
        // set some stuff
        // error_reporting would be handled differently in Rust
        
        *Self::cli.write().unwrap() = env::var("RUST_SAPI_NAME").unwrap_or_default() == "cli";
        
        // Setup timezone
        // In Rust this would be handled differently
        
        // Try to configure PHP settings for big file uploads
        // In Rust these would be handled by the web server and framework configuration
        
        // Copy HTTP auth headers (in Rust, these would be handled by the web framework)
        
        Self::init_paths()?;
        
        if Config::get_value("instanceid", false) {
            // Try to set up memory cache
            if let Some(loader) = Self::loader.write().unwrap().as_mut() {
                match cache::Factory::create_low_latency("Autoloader") {
                    Ok(cache) => loader.set_memory_cache(cache),
                    Err(_) => {} // Ignore errors
                }
            }
        }
        
        util::Util::is_set_locale_working();
        
        // set debug mode if an xdebug session is active
        if !env::var("DEBUG").unwrap_or_default().eq_ignore_ascii_case("true") {
            if let Ok(cookie) = env::var("HTTP_COOKIE") {
                if cookie.contains("XDEBUG_SESSION") {
                    env::set_var("DEBUG", "true");
                }
            }
        }
        
        // Setup error handlers
        if env::var("PHPUNIT_RUN").is_err() {
            if env::var("DEBUG").unwrap_or_default().eq_ignore_ascii_case("true") {
                // In a Rust app, error handling would be different
                // This is a placeholder for that functionality
            } else {
                log_handler::ErrorHandler::register();
                log_handler::ErrorHandler::set_logger(&log_handler::Log::instance());
            }
        }
        
        // register the stream wrappers
        // In Rust, file handling would be different
        // This is a placeholder for that functionality
        
        // setup the basic server
        *Self::server.write().unwrap() = Some(Server::new());
        
        Self::init_template_engine()?;
        
        if !*Self::cli.read().unwrap() {
            Self::init_session()?;
        } else {
            *Self::session.write().unwrap() = Some(Arc::new(session::MemorySession::new("")));
        }
        
        Self::check_config()?;
        Self::check_installed()?;
        Self::check_ssl()?;
        
        let errors = util::Util::check_server();
        if !errors.is_empty() {
            let mut template = template::Template::new("", "error", "guest");
            template.assign("errors", &errors);
            template.print_page();
            std::process::exit(1);
        }
        
        // try to set the session lifetime
        // In Rust, session configuration would be different
        
        // User and Groups
        if !Config::get_value("installed", false) {
            if let Some(session) = Self::session.read().unwrap().as_ref() {
                session.set("user_id", "");
            }
        }
        
        // Setup User and Group backends
        User::use_backend(Box::new(user::UserDatabase::new()));
        Group::use_backend(Box::new(user::GroupDatabase::new()));
        
        // Check for session/auth mismatch
        if let Ok(auth_user) = env::var("PHP_AUTH_USER") {
            if let Some(session) = Self::session.read().unwrap().as_ref() {
                if session.exists("user_id") {
                    if let Ok(session_user) = session.get::<String>("user_id") {
                        if !session_user.is_empty() && auth_user != session_user {
                            log_handler::Log::write("core", 
                                &format!("Session user-id ({}) doesn't match SERVER[PHP_AUTH_USER] ({}).", 
                                    session_user, auth_user), 
                                log_handler::Log::WARN);
                            User::logout();
                        }
                    }
                }
            }
        }
        
        // Load Apps
        // This includes plugins for users and filesystems as well
        let runtime_noapps = env::var("RUNTIME_NOAPPS").unwrap_or_default().eq_ignore_ascii_case("true");
        let runtime_apptypes = env::var("RUNTIME_APPTYPES").ok();
        
        if !runtime_noapps && !Self::check_upgrade(false)? {
            if let Some(apptypes) = runtime_apptypes {
                app::App::load_apps(&apptypes.split(',').collect::<Vec<_>>())?;
            } else {
                app::App::load_apps(&[])?;
            }
        }
        
        // setup extra user backends
        User::setup_backends();
        
        Self::register_cache_hooks();
        Self::register_filesystem_hooks();
        Self::register_preview_hooks();
        Self::register_share_hooks();
        Self::register_log_rotate();
        
        // Make sure temporary files are cleaned up
        // In Rust, we'd use Drop trait or similar for cleanup
        
        // parse the given parameters
        let requested_app = if let Ok(app) = env::var("QUERY_APP") {
            if !app.trim().is_empty() {
                app::App::clean_app_id(&app.trim())
            } else {
                Config::get_value("defaultapp", "files".to_string())
            }
        } else {
            Config::get_value("defaultapp", "files".to_string())
        };
        
        let mut requested_app = requested_app;
        if requested_app.contains('?') {
            let parts: Vec<&str> = requested_app.splitn(2, '?').collect();
            requested_app = parts[0].to_string();
            
            // In Rust, query parameter handling would be done by the web framework
            // This is a placeholder for that functionality
        }
        
        *Self::requested_app.write().unwrap() = requested_app;
        
        let requested_file = env::var("QUERY_GETFILE").ok();
        let mut requested_file = requested_file;
        
        if let Some(ref file) = requested_file {
            if file.contains('?') {
                let parts: Vec<&str> = file.splitn(2, '?').collect();
                requested_file = Some(parts[0].to_string());
                
                // In Rust, query parameter handling would be done by the web framework
                // This is a placeholder for that functionality
            }
        }
        
        *Self::requested_file.write().unwrap() = requested_file.clone();
        
        if let Some(ref file) = requested_file {
            let app = Self::requested_app.read().unwrap();
            let subdir = format!("{}/{}", app::App::get_app_path(&app)?, file);
            let parent = app::App::get_app_path(&app)?;
            
            if !util::Helper::is_subdirectory(&subdir, &parent) {
                *Self::requested_file.write().unwrap() = None;
                // In Rust, HTTP responses would be handled by the web framework
                // This is a placeholder for that functionality
                return Err(OcError::FileNotFound("File not found".to_string()));
            }
        }
        
        // write error into log if locale can't be set
        if !util::Util::is_set_locale_working() {
            log_handler::Log::write("core", 
                "setting locale to en_US.UTF-8/en_US.UTF8 failed. Support is probably not installed on your system", 
                log_handler::Log::ERROR);
        }
        
        if Config::get_value("installed", false) && !Self::check_upgrade(false)? {
            if app::Appconfig::get_value("core", "backgroundjobs_mode", "ajax".to_string()) == "ajax" {
                util::Util

}}}} // Añadido por reparador automático