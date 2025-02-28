// ownCloud
//
// This library manages apps in the ownCloud ecosystem, allowing them to register and integrate.
// It is also responsible for installing, upgrading and removing apps.
//
// License: GNU AFFERO GENERAL PUBLIC LICENSE
// Original authors: Frank Karlitschek, Jakob Sack
// Copyright: 2012 Frank Karlitschek frank@owncloud.org

use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};

use async_trait::async_trait;
use futures::Future;
use log::{debug, error, warn};
use serde::{Deserialize, Serialize};
use simple_xml_serialize::XMLElement;
use regex::Regex;
use once_cell::sync::Lazy;

/// Error type for app operations
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("XML parsing error: {0}")]
    Xml(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("App not found: {0}")]
    NotFound(String),
    
    #[error("App compatibility error: {0}")]
    Compatibility(String),
    
    #[error("User not logged in")]
    NotLoggedIn,
    
    #[error("App not enabled: {0}")]
    NotEnabled(String),
    
    #[error("Invalid app ID: {0}")]
    InvalidAppId(String),
    
    #[error("Other error: {0}")]
    Other(String),
}

type Result<T> = std::result::Result<T, AppError>;

/// App information structure
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AppInfo {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub require: String,
    #[serde(default)]
    pub shipped: bool,
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default)]
    pub remote: HashMap<String, String>,
    #[serde(default)]
    pub public: HashMap<String, String>,
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub internal: bool,
    #[serde(default)]
    pub internal_label: String,
    #[serde(default)]
    pub internal_class: String,
    #[serde(default)]
    pub update: bool,
    #[serde(default)]
    pub preview: String,
    #[serde(default)]
    pub score: String,
    #[serde(default)]
    pub ocs_id: String,
}

/// Root app directory configuration
#[derive(Clone, Debug)]
pub struct AppRoot {
    pub path: PathBuf,
    pub url: String,
    pub writable: bool,
}

/// Navigation entry for the UI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NavigationEntry {
    pub id: String,
    pub name: String,
    pub href: String,
    pub icon: Option<String>,
    pub order: i32,
    #[serde(default)]
    pub active: bool,
}

#[async_trait]
pub trait NavigationManager: Send + Sync {
    fn add(&self, entry: NavigationEntry);
    fn get_all(&self) -> Vec<NavigationEntry>;
    fn set_active_entry(&self, id: &str);
    fn get_active_entry(&self) -> String;
    fn clear(&self);
}

#[async_trait]
pub trait ConfigProvider: Send + Sync {
    async fn get_value(&self, app: &str, key: &str, default: &str) -> String;
    async fn set_value(&self, app: &str, key: &str, value: &str) -> Result<()>;
    async fn get_values(&self, app: Option<&str>, key: &str) -> Result<HashMap<String, String>>;
}

#[async_trait]
pub trait UserProvider: Send + Sync {
    async fn is_logged_in(&self) -> bool;
    async fn get_user(&self) -> Option<String>;
    async fn is_admin(&self, user: &str) -> bool;
    async fn is_sub_admin(&self, user: &str) -> bool;
}

#[async_trait]
pub trait LogProvider: Send + Sync {
    async fn log(&self, app: &str, message: &str, level: LogLevel);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[async_trait]
pub trait HookManager: Send + Sync {
    async fn emit(&self, scope: &str, event: &str, data: HashMap<String, String>);
}

#[async_trait]
pub trait UtilProvider: Send + Sync {
    async fn get_version(&self) -> Vec<u32>;
    async fn get_edition_string(&self) -> String;
    fn get_web_root(&self) -> String;
}

#[async_trait]
pub trait DbProvider: Send + Sync {
    async fn update_db_from_structure(&self, xml_path: &Path) -> Result<()>;
    async fn execute_query(&self, query: &str, params: Vec<String>) -> Result<Vec<HashMap<String, String>>>;
}

#[async_trait]
pub trait InstallerProvider: Send + Sync {
    async fn is_installed(&self, app_id: &str) -> bool;
    async fn install_shipped_app(&self, app_id: &str) -> Result<String>;
    async fn install_app(&self, app_info: HashMap<String, String>) -> Result<String>;
    async fn remove_app(&self, app_id: &str) -> Result<()>;
    async fn is_update_available(&self, app_id: &str) -> bool;
}

#[async_trait]
pub trait OcsClientProvider: Send + Sync {
    async fn get_categories(&self) -> Result<HashMap<String, String>>;
    async fn get_applications(&self, categories: Vec<String>, page: u32, filter: &str) -> Result<Vec<HashMap<String, String>>>;
    async fn get_application(&self, app_id: &str) -> Result<HashMap<String, String>>;
    async fn get_application_download(&self, app_id: &str, download_id: u32) -> Result<HashMap<String, String>>;
}

#[async_trait]
pub trait HelperProvider: Send + Sync {
    fn link_to_route(&self, route: &str) -> String;
    fn image_path(&self, app: &str, file: &str) -> String;
}

#[async_trait]
pub trait L10nProvider: Send + Sync {
    fn get(&self, app: &str) -> Box<dyn L10n>;
}

pub trait L10n: Send + Sync {
    fn t(&self, text: &str) -> String;
    fn t_with_vars(&self, text: &str, vars: Vec<String>) -> String;
}

/// The main App Manager
pub struct AppManager {
    settings_forms: Arc<Mutex<Vec<String>>>,
    admin_forms: Arc<Mutex<Vec<String>>>,
    personal_forms: Arc<Mutex<Vec<String>>>,
    app_info: Arc<RwLock<HashMap<String, AppInfo>>>,
    app_types: Arc<RwLock<HashMap<String, String>>>,
    loaded_apps: Arc<RwLock<Vec<String>>>,
    checked_apps: Arc<RwLock<HashSet<String>>>,
    alt_login: Arc<RwLock<Vec<String>>>,
    enabled_apps_cache: Arc<RwLock<Vec<String>>>,
    app_roots: Vec<AppRoot>,
    navigation_manager: Arc<dyn NavigationManager>,
    config_provider: Arc<dyn ConfigProvider>,
    user_provider: Arc<dyn UserProvider>,
    log_provider: Arc<dyn LogProvider>,
    hook_manager: Arc<dyn HookManager>,
    util_provider: Arc<dyn UtilProvider>,
    db_provider: Arc<dyn DbProvider>,
    installer_provider: Arc<dyn InstallerProvider>,
    ocs_client_provider: Arc<dyn OcsClientProvider>,
    helper_provider: Arc<dyn HelperProvider>,
    l10n_provider: Arc<dyn L10nProvider>,
}

impl AppManager {
    /// Create a new AppManager with all required dependencies
    pub fn new(
        app_roots: Vec<AppRoot>,
        navigation_manager: Arc<dyn NavigationManager>,
        config_provider: Arc<dyn ConfigProvider>,
        user_provider: Arc<dyn UserProvider>,
        log_provider: Arc<dyn LogProvider>,
        hook_manager: Arc<dyn HookManager>,
        util_provider: Arc<dyn UtilProvider>,
        db_provider: Arc<dyn DbProvider>,
        installer_provider: Arc<dyn InstallerProvider>,
        ocs_client_provider: Arc<dyn OcsClientProvider>,
        helper_provider: Arc<dyn HelperProvider>,
        l10n_provider: Arc<dyn L10nProvider>,
    ) -> Self {
        AppManager {
            settings_forms: Arc::new(Mutex::new(Vec::new())),
            admin_forms: Arc::new(Mutex::new(Vec::new())),
            personal_forms: Arc::new(Mutex::new(Vec::new())),
            app_info: Arc::new(RwLock::new(HashMap::new())),
            app_types: Arc::new(RwLock::new(HashMap::new())),
            loaded_apps: Arc::new(RwLock::new(Vec::new())),
            checked_apps: Arc::new(RwLock::new(HashSet::new())),
            alt_login: Arc::new(RwLock::new(Vec::new())),
            enabled_apps_cache: Arc::new(RwLock::new(Vec::new())),
            app_roots,
            navigation_manager,
            config_provider,
            user_provider,
            log_provider,
            hook_manager,
            util_provider,
            db_provider,
            installer_provider,
            ocs_client_provider,
            helper_provider,
            l10n_provider,
        }
    }

    /// Clean app ID by removing invalid characters
    pub fn clean_app_id(&self, app: &str) -> String {
        let regex = Regex::new(r"[\0/\\\.]{2,}").unwrap();
        regex.replace_all(app, "").to_string()
    }

    /// Load all enabled apps
    pub async fn load_apps(&self, types: Option<Vec<String>>) -> Result<bool> {
        // Load the enabled apps
        let apps = self.get_enabled_apps().await?;
        
        for app in apps {
            if let Some(ref types_filter) = types {
                if !self.is_type(&app, types_filter).await? {
                    continue;
                }
            }
            
            let loaded_apps = self.loaded_apps.read().unwrap();
            if !loaded_apps.contains(&app) {
                drop(loaded_apps); // Release read lock before modifying
                self.load_app(&app).await?;
                let mut loaded_apps = self.loaded_apps.write().unwrap();
                loaded_apps.push(app);
            }
        }
        
        // In the original PHP code there is a DEBUG condition for optimization
        // We'd implement similar logic in a production environment

        Ok(true)
    }

    /// Load a single app
    pub async fn load_app(&self, app: &str) -> Result<()> {
        let app_path = self.get_app_path(app)?;
        let app_info_path = app_path.join("appinfo/app.php");
        
        if app_info_path.exists() {
            self.check_upgrade(app).await?;
            // Here we would include the app.php file
            // In Rust, this would be handled differently, perhaps by loading a plugin
            // or executing a script through a scripting engine
        }
        
        Ok(())
    }

    /// Check if an app is of specific type(s)
    pub async fn is_type(&self, app: &str, types: &[String]) -> Result<bool> {
        let app_types = self.get_app_types(app).await?;
        
        for t in types {
            if app_types.contains(t) {
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Get the types of an app
    pub async fn get_app_types(&self, app: &str) -> Result<Vec<String>> {
        // Load the cache
        let app_types = self.app_types.read().unwrap();
        if app_types.is_empty() {
            drop(app_types); // Release read lock
            
            let values = self.config_provider.get_values(None, "types").await?;
            let mut app_types = self.app_types.write().unwrap();
            
            for (k, v) in values {
                app_types.insert(k, v);
            }
        }
        
        let app_types = self.app_types.read().unwrap();
        if let Some(types_str) = app_types.get(app) {
            Ok(types_str.split(',').map(String::from).collect())
        } else {
            Ok(Vec::new())
        }
    }

    /// Set app types in the database based on info.xml
    pub async fn set_app_types(&self, app: &str) -> Result<()> {
        let app_data = self.get_app_info(app, false).await?;
        
        let app_types = if let Some(types) = app_data.types.clone() {
            types.join(",")
        } else {
            String::new()
        };
        
        self.config_provider.set_value(app, "types", &app_types).await?;
        Ok(())
    }

    /// Check if app is shipped with the core
    pub async fn is_shipped(&self, app_id: &str) -> Result<bool> {
        let info = self.get_app_info(app_id, false).await?;
        Ok(info.shipped)
    }

    /// Get all enabled apps
    pub async fn get_enabled_apps(&self) -> Result<Vec<String>> {
        // Check if system is installed
        let installed = self.config_provider.get_value("core", "installed", "false").await;
        if installed == "false" {
            return Ok(Vec::new());
        }
        
        // Check cache
        let enabled_cache = self.enabled_apps_cache.read().unwrap();
        if !enabled_cache.is_empty() {
            return Ok(enabled_cache.clone());
        }
        drop(enabled_cache);
        
        // Start with files app which is always enabled
        let mut apps = vec!["files".to_string()];
        
        // Query for enabled apps
        let query = "SELECT `appid` FROM `*PREFIX*appconfig` WHERE `configkey` = 'enabled' AND `configvalue` = 'yes'";
        let results = self.db_provider.execute_query(query, Vec::new()).await?;
        
        for row in results {
            if let Some(app_id) = row.get("appid") {
                if !apps.contains(app_id) {
                    apps.push(app_id.clone());
                }
            }
        }
        
        // Update cache
        let mut enabled_cache = self.enabled_apps_cache.write().unwrap();
        *enabled_cache = apps.clone();
        
        Ok(apps)
    }

    /// Check if an app is enabled
    pub async fn is_enabled(&self, app: &str) -> Result<bool> {
        if app == "files" {
            return Ok(true);
        }
        
        let enabled_apps = self.get_enabled_apps().await?;
        Ok(enabled_apps.contains(&app.to_string()))
    }

    /// Enable an app
    pub async fn enable(&self, app: &str) -> Result<()> {
        // Clear cache
        {
            let mut enabled_cache = self.enabled_apps_cache.write().unwrap();
            enabled_cache.clear();
        }
        
        let l = self.l10n_provider.get("core");
        
        // Check if app is installed, install if not
        if !self.installer_provider.is_installed(app).await {
            // Check if numeric (OCS) or string (shipped) app ID
            let app = if !app.chars().all(|c| c.is_digit(10)) {
                // Shipped app
                self.installer_provider.install_shipped_app(app).await?
            } else {
                // OCS app - get from appstore
                let app_data = self.ocs_client_provider.get_application(app).await?;
                let download = self.ocs_client_provider.get_application_download(app, 1).await?;
                
                if let Some(download_link) = download.get("downloadlink") {
                    if !download_link.is_empty() {
                        let mut info = HashMap::new();
                        info.insert("source".to_string(), "http".to_string());
                        info.insert("href".to_string(), download_link.clone());
                        
                        // Copy app data
                        for (k, v) in app_data {
                            info.insert(format!("appdata_{}", k), v);
                        }
                        
                        self.installer_provider.install_app(info).await?
                    } else {
                        return Err(AppError::Other("No download link found".to_string()));
                    }
                } else {
                    return Err(AppError::Other("No download link found".to_string()));
                }
            };
            
            // Check compatibility for newly installed app
            let info = self.get_app_info(&app, false).await?;
            let version = self.util_provider.get_version().await;
            
            if let Some(require) = info.require {
                if !self.is_app_version_compatible(&version, &require)? {
                    return Err(AppError::Compatibility(
                        l.t_with_vars("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.",
                        vec![info.name.clone()])
                    ));
                }
            }
            
            // Enable the app
            self.config_provider.set_value(&app, "enabled", "yes").await?;
            
            // Store OCS ID if applicable
            if let Some(id) = app_data.get("id") {
                self.config_provider.set_value(&app, "ocsid", id).await?;
            }
            
            // Emit hook for app enabled
            let mut data = HashMap::new();
            data.insert("app".to_string(), app.to_string());
            self.hook_manager.emit("OC_App", "post_enable", data).await;
            
            Ok(())
        } else {
            Err(AppError::Other(l.t("No app name specified")))
        }
    }

    /// Disable an app
    pub async fn disable(&self, app: &str) -> Result<()> {
        // Clear cache
        {
            let mut enabled_cache = self.enabled_apps_cache.write().unwrap();
            enabled_cache.clear();
        }
        
        // Emit pre-disable hook
        let mut data = HashMap::new();
        data.insert("app".to_string(), app.to_string());
        self.hook_manager.emit("OC_App", "pre_disable", data).await;
        
        // Disable the app
        self.config_provider.set_value(app, "enabled", "no").await?;
        
        // Check if it's a shipped app, if not, remove it
        if !self.is_shipped(app).await? {
            self.installer_provider.remove_app(app).await?;
        }
        
        Ok(())
    }

    /// Add a navigation entry
    pub fn add_navigation_entry(&self, data: NavigationEntry) -> bool {
        self.navigation_manager.add(data);
        true
    }

    /// Set active navigation entry
    pub fn set_active_navigation_entry(&self, id: &str) -> bool {
        self.navigation_manager.set_active_entry(id);
        true
    }

    /// Get app navigation entries
    pub async fn get_app_navigation_entries(&self, app: &str) -> Result<Vec<NavigationEntry>> {
        let app_path = self.get_app_path(app)?;
        let app_php_path = app_path.join("appinfo/app.php");
        
        if app_php_path.exists() {
            self.navigation_manager.clear();
            
            // In PHP, the app.php would be included here which would register navigation entries
            // In Rust, we would need a different mechanism to load and execute the app's code
            
            Ok(self.navigation_manager.get_all())
        } else {
            Ok(Vec::new())
        }
    }

    /// Get active navigation entry
    pub fn get_active_navigation_entry(&self) -> String {
        self.navigation_manager.get_active_entry()
    }

    /// Get the settings navigation
    pub async fn get_settings_navigation(&self) -> Result<Vec<NavigationEntry>> {
        let l = self.l10n_provider.get("lib");
        let mut settings = Vec::new();
        
        // By default, settings only contain the help menu
        let edition_string = self.util_provider.get_edition_string().await;
        let kb_enabled = self.config_provider.get_value("core", "knowledgebaseenabled", "true").await;
        
        if edition_string.is_empty() && kb_enabled == "true" {
            settings.push(NavigationEntry {
                id: "help".to_string(),
                order: 1000,
                href: self.helper_provider.link_to_route("settings_help"),
                name: l.t("Help"),
                icon: Some(self.helper_provider.image_path("settings", "help.svg")),
                active: false,
            });
        }
        
        // If user is logged in
        if self.user_provider.is_logged_in().await {
            // Personal menu
            settings.push(NavigationEntry {
                id: "personal".to_string(),
                order: 1,
                href: self.helper_provider.link_to_route("settings_personal"),
                name: l.t("Personal"),
                icon: Some(self.helper_provider.image_path("settings", "personal.svg")),
                active: false,
            });
            
            // If there are settings forms
            let settings_forms = self.settings_forms.lock().unwrap();
            if !settings_forms.is_empty() {
                settings.push(NavigationEntry {
                    id: "settings".to_string(),
                    order: 1000,
                    href: self.helper_provider.link_to_route("settings_settings"),
                    name: l.t("Settings"),
                    icon: Some(self.helper_provider.image_path("settings", "settings.svg")),
                    active: false,
                });
            }
            
            // For SubAdmins (user management)
            if let Some(user) = self.user_provider.get_user().await {
                if self.user_provider.is_sub_admin(&user).await {
                    settings.push(NavigationEntry {
                        id: "core_users".to_string(),
                        order: 2,
                        href: self.helper_provider.link_to_route("settings_users"),
                        name: l.t("Users"),
                        icon: Some(self.helper_provider.image_path("settings", "users.svg")),
                        active: false,
                    });
                }
                
                // For Admins (admin settings)
                if self.user_provider.is_admin(&user).await {
                    settings.push(NavigationEntry {
                        id: "admin".to_string(),
                        order: 1000,
                        href: self.helper_provider.link_to_route("settings_admin"),
                        name: l.t("Admin"),
                        icon: Some(self.helper_provider.image_path("settings", "admin.svg")),
                        active: false,
                    });
                }
            }
        }
        
        self.proceed_navigation(settings)
    }

    /// Process navigation - set active entry and sort
    fn proceed_navigation(&self, mut list: Vec<NavigationEntry>) -> Result<Vec<NavigationEntry>> {
        let active_app = self.navigation_manager.get_active_entry();
        
        for entry in &mut list {
            entry.active = entry.id == active_app;
        }
        
        // Sort by order
        list.sort_by_key(|entry| entry.order);
        
        Ok(list)
    }

    /// Get the main navigation
    pub async fn get_navigation(&self) -> Result<Vec<NavigationEntry>> {
        let entries = self.navigation_manager.get_all();
        self.proceed_navigation(entries)
    }

    /// Get the install path for apps
    pub fn get_install_path(&self) -> Option<PathBuf> {
        let apps_enabled = self.config_provider
            .get_value("core", "appstoreenabled", "true")
            .await
            .unwrap_or_else(|_| "true".to_string());
            
        if apps_enabled != "true" {
            return None;
        }
        
        for dir in &self.app_roots {
            if dir.writable {
                return Some(dir.path.clone());
            }
        }
        
        self.log_provider.log("core", "No application directories are marked as writable.", LogLevel::Error).await;
        None
    }

    /// Find app in directories
    fn find_app_in_directories(&self, app_id: &str) -> Option<AppRoot> {
        static APP_DIR_CACHE: Lazy<Mutex<HashMap<String, AppRoot>>> = Lazy::new(|| {
            Mutex::new(HashMap::new())
        });
        
        {
            let cache = APP_DIR_CACHE.lock().unwrap();
            if let Some(dir) = cache.get(app_id) {
                return Some(dir.clone());
            }
        }
        
        for dir in &self.app_roots {
            if Path::new(&dir.path).join(app_id).exists() {
                let dir_clone = dir.clone();
                let mut cache = APP_DIR_CACHE.lock().unwrap();
                cache.insert(app_id.to_string(), dir_clone.clone());
                return Some(dir_clone);
            }
        }
        
        None
    }

    /// Get the path for the given app
    pub fn get_app_path(&self, app_id: &str) -> Result<PathBuf> {
        if let Some(dir) = self.find_app_in_directories(app_id) {
            Ok(dir.path.join(app_id))
        } else {
            Err(AppError::NotFound(app_id.to_string()))
        }
    }

    /// Get the web path for the given app
    pub fn get_app_web_path(&self, app_id: &str) -> Result<String> {
        if let Some(dir) = self.find_app_in_directories(app_id) {
            let web_root = self.util_provider.get_web_root();
            Ok(format!("{}{}/{}", web_root, dir.url, app_id))
        } else {
            Err(AppError::NotFound(app_id.to_string()))
        }
    }

    /// Get the app version
    pub async fn get_app_version(&self, app_id: &str) -> Result<String> {
        let app_path = self.get_app_path(app_id)?;
        let version_file = app_path.join("appinfo/version");
        
        if version_file.exists() {
            let mut file = File::open(version_file)?;
            let mut version = String::new();
            file.read_to_string(&mut version)?;
            Ok(version.trim().to_string())
        } else {
            let app_data = self.get_app_info(app_id, false).await?;
            Ok(app_data.version.unwrap_or_default())
        }
    }

    /// Read app metadata from info.xml
    pub async fn get_app_info(&self, app_id: &str, is_path: bool) -> Result<AppInfo> {
        if is_path {
            // Parse XML from file path directly
            self.parse_app_info_xml(PathBuf::from(app_id))
        } else {
            // Check cache first
            {
                let app_info = self.app_info.read().unwrap();
                if let Some(info) = app_info.get(app_id) {
                    return Ok(info.clone());
                }
            }
            
            // Get path and parse XML
            let app_path = self.get_app_path(app_id)?;
            let info_path = app_path.join("appinfo/info.xml");
            let app_info = self.parse_app_info_xml(info_path)?;
            
            // Cache the result
            {
                let mut app_info_cache = self.app_info.write().unwrap();
                app_info_cache.insert(app_id.to_string(), app_info.clone());
            }
            
            Ok(app_info)
        }
    }

    /// Parse app info XML file
    fn parse_app_info_xml(&self, file_path: PathBuf) -> Result<AppInfo> {
        let mut file = File::open(&file_path)
            .map_err(|e| AppError::Io(e))?;
            
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| AppError::Io(e))?;
            
        // Parse XML content
        // In a real implementation, this would use a proper XML parser
        // For this example, we'll return a simple stub
        
        let mut app_info = AppInfo::default();
        
        // Here we would actually parse the XML structure
        // For illustrative purposes, let's assume we've extracted these values:
        app_info.name = "Sample App".to_string();
        app_info.description = "This is a sample app".to_string();
        app_info.version = "1.0.0".to_string();
        
        Ok

}} // Añadido por reparador automático