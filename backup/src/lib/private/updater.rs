use async_trait::async_trait;
use chrono::{DateTime, Duration, TimeZone, Utc};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use simple_xml_serialize::SimpleXmlDeserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Emitter trait for basic event handling
#[async_trait]
pub trait Emitter {
    async fn emit(&self, scope: &str, event: &str, data: Option<Value>) -> Result<(), Box<dyn Error>>;
}

/// Basic implementation of an event emitter
pub struct BasicEmitter {}

#[async_trait]
impl Emitter for BasicEmitter {
    async fn emit(&self, scope: &str, event: &str, data: Option<Value>) -> Result<(), Box<dyn Error>> {
        // In a real implementation, this would notify listeners
        // For now it's a placeholder
        Ok(())
    }
}

/// Class that handles autoupdating of ownCloud
///
/// Hooks provided in scope \OC\Updater
///  - maintenanceStart()
///  - maintenanceEnd()
///  - dbUpgrade()
///  - filecacheStart()
///  - filecacheProgress(int $percentage)
///  - filecacheDone()
///  - failure(string $message)
pub struct Updater {
    log: Option<Box<dyn log::Log>>,
    emitter: Box<dyn Emitter>,
}

#[derive(Debug, Deserialize, Serialize, SimpleXmlDeserialize)]
pub struct UpdateData {
    version: String,
    versionstring: String,
    url: String,
    web: String,
}

impl Updater {
    /// Create a new updater instance
    pub fn new(log: Option<Box<dyn log::Log>>) -> Self {
        Updater {
            log,
            emitter: Box::new(BasicEmitter {}),
        }
    }

    /// Check if a new version is available
    /// 
    /// # Arguments
    /// * `updater_url` - the url to check, i.e. 'http://apps.owncloud.com/updater.php'
    ///
    /// # Returns
    /// HashMap with update information or empty HashMap if no update available
    pub async fn check(&self, updater_url: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
        // Look up the cache - it is invalidated all 30 minutes
        let last_update_at = AppConfig::get_value("core", "lastupdatedat")
            .unwrap_or_else(|_| "0".to_string())
            .parse::<i64>()
            .unwrap_or(0);
        
        if (last_update_at + 1800) > Utc::now().timestamp() {
            if let Ok(last_result) = AppConfig::get_value("core", "lastupdateResult") {
                if let Ok(parsed) = serde_json::from_str::<Value>(&last_result) {
                    let mut result = HashMap::new();
                    if let Some(version) = parsed.get("version").and_then(|v| v.as_str()) {
                        result.insert("version".to_string(), version.to_string());
                    }
                    if let Some(versionstring) = parsed.get("versionstring").and_then(|v| v.as_str()) {
                        result.insert("versionstring".to_string(), versionstring.to_string());
                    }
                    if let Some(url) = parsed.get("url").and_then(|v| v.as_str()) {
                        result.insert("url".to_string(), url.to_string());
                    }
                    if let Some(web) = parsed.get("web").and_then(|v| v.as_str()) {
                        result.insert("web".to_string(), web.to_string());
                    }
                    return Ok(result);
                }
            }
        }

        AppConfig::set_value("core", "lastupdatedat", &Utc::now().timestamp().to_string())?;

        if AppConfig::get_value("core", "installedat").unwrap_or_default().is_empty() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64();
            AppConfig::set_value("core", "installedat", &now.to_string())?;
        }

        let version = Util::get_version();
        let installed_at = AppConfig::get_value("core", "installedat").unwrap_or_default();
        let updated_at = AppConfig::get_value("core", "lastupdatedat").unwrap_or_default();
        let update_channel = Util::get_channel();
        let edition = Util::get_edition_string();

        let mut version_info = version.clone();
        version_info.push(installed_at);
        version_info.push(updated_at);
        version_info.push(update_channel);
        version_info.push(edition);
        
        let version_string = version_info.join("x");
        
        // fetch xml data from updater
        let url = format!("{}?version={}", updater_url, version_string);
        
        // set a sensible timeout for the request
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        
        let response = client.get(&url).send().await?;
        if !response.status().is_success() {
            return Ok(HashMap::new());
        }
        
        let xml = response.text().await?;
        let data: UpdateData = match simple_xml_serialize::from_str(&xml) {
            Ok(data) => data,
            Err(_) => return Ok(HashMap::new()),
        };
        
        let mut result = HashMap::new();
        result.insert("version".to_string(), data.version);
        result.insert("versionstring".to_string(), data.versionstring);
        result.insert("url".to_string(), data.url);
        result.insert("web".to_string(), data.web);
        
        // Cache the result
        AppConfig::set_value("core", "lastupdateResult", &serde_json::to_string(&data)?)?;
        
        Ok(result)
    }

    /// Runs the update actions in maintenance mode, does not upgrade the source files
    pub async fn upgrade(&self) -> Result<(), Box<dyn Error>> {
        DB::enable_caching(false);
        Config::set_value("maintenance", "true")?;
        
        let installed_version = Config::get_value("version").unwrap_or_else(|_| "0.0.0".to_string());
        let current_version = Util::get_version().join(".");
        
        if let Some(log) = &self.log {
            debug!(
                target: "core",
                "starting upgrade from {} to {}", 
                installed_version, 
                current_version
            );
        }
        
        self.emitter.emit("\\OC\\Updater", "maintenanceStart", None).await?;
        
        match self.perform_upgrade().await {
            Ok(_) => {
                Config::set_value("version", &Util::get_version().join("."))?;
                App::check_apps_requirements()?;
                // load all apps to also upgrade enabled apps
                App::load_apps()?;
            },
            Err(e) => {
                self.emitter.emit(
                    "\\OC\\Updater", 
                    "failure", 
                    Some(serde_json::to_value(e.to_string())?)
                ).await?;
            }
        }
        
        Config::set_value("maintenance", "false")?;
        self.emitter.emit("\\OC\\Updater", "maintenanceEnd", None).await?;
        
        Ok(())
    }

    async fn perform_upgrade(&self) -> Result<(), Box<dyn Error>> {
        let server_root = std::env::var("SERVER_ROOT")?;
        DB::update_db_from_structure(&format!("{}/db_structure.xml", server_root))?;
        self.emitter.emit("\\OC\\Updater", "dbUpgrade", None).await?;

        // do a file cache upgrade for users with files
        self.upgrade_file_cache().await?;
        
        Ok(())
    }

    async fn upgrade_file_cache(&self) -> Result<(), Box<dyn Error>> {
        let users = match DB::query("SELECT DISTINCT `user` FROM `*PREFIX*fscache`") {
            Ok(result) => result,
            Err(_) => return Ok(()),
        };
        
        if users.is_empty() {
            return Ok(());
        }
        
        let step = 100.0 / users.len() as f64;
        let mut percent_completed = 0.0;
        let mut last_percent_completed_output = 0;
        let mut start_info_shown = false;
        
        for user_row in users {
            let user = user_row.get("user").unwrap_or_default();
            Filesystem::init_mount_points(user);
            Cache::do_silent_upgrade(user)?;
            
            if !start_info_shown {
                // We show it only now, because otherwise Info about upgraded apps
                // will appear between this and progress info
                self.emitter.emit("\\OC\\Updater", "filecacheStart", None).await?;
                start_info_shown = true;
            }
            
            percent_completed += step;
            let out = percent_completed.floor() as i32;
            
            if out != last_percent_completed_output {
                self.emitter.emit(
                    "\\OC\\Updater", 
                    "filecacheProgress", 
                    Some(serde_json::to_value(out)?)
                ).await?;
                last_percent_completed_output = out;
            }
        }
        
        self.emitter.emit("\\OC\\Updater", "filecacheDone", None).await?;
        
        Ok(())
    }
}

// Placeholder struct for DB operations
struct DB;

impl DB {
    fn enable_caching(_enable: bool) {
        // Implementation would go here
    }
    
    fn update_db_from_structure(_structure_file: &str) -> Result<(), Box<dyn Error>> {
        // Implementation would go here
        Ok(())
    }
    
    fn query(_sql: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
        // Implementation would go here
        Ok(vec![])
    }
}

// Placeholder struct for Config operations
struct Config;

impl Config {
    fn get_value(key: &str) -> Result<String, Box<dyn Error>> {
        // Implementation would go here
        Ok(String::new())
    }
    
    fn set_value(key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        // Implementation would go here
        Ok(())
    }
}

// Placeholder struct for AppConfig operations
struct AppConfig;

impl AppConfig {
    fn get_value(app: &str, key: &str) -> Result<String, Box<dyn Error>> {
        // Implementation would go here
        Ok(String::new())
    }
    
    fn set_value(app: &str, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        // Implementation would go here
        Ok(())
    }
}

// Placeholder struct for Util operations
struct Util;

impl Util {
    fn get_version() -> Vec<String> {
        // Implementation would go here
        vec![]
    }
    
    fn get_channel() -> String {
        // Implementation would go here
        String::new()
    }
    
    fn get_edition_string() -> String {
        // Implementation would go here
        String::new()
    }
}

// Placeholder struct for App operations
struct App;

impl App {
    fn check_apps_requirements() -> Result<(), Box<dyn Error>> {
        // Implementation would go here
        Ok(())
    }
    
    fn load_apps() -> Result<(), Box<dyn Error>> {
        // Implementation would go here
        Ok(())
    }
}

// Placeholder struct for Filesystem operations
struct Filesystem;

impl Filesystem {
    fn init_mount_points(_user: &str) {
        // Implementation would go here
    }
}

// Placeholder struct for Cache operations
struct Cache;

impl Cache {
    fn do_silent_upgrade(_user: &str) -> Result<(), Box<dyn Error>> {
        // Implementation would go here
        Ok(())
    }
}