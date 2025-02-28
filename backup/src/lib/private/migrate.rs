// Migration functionality for ownCloud
// 
// Provides an interface to migrate users and whole owncloud instances
// 
// Original Author: Tom Needham
// Copyright: 2012 Tom Needham tom@owncloud.com
// 
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
// 
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
// 
// You should have received a copy of the GNU Affero General Public
// License along with this library. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use zip::{ZipArchive, ZipWriter};
use zip::write::FileOptions;
use serde::{Serialize, Deserialize};
use serde_json::{self, json};
use chrono::{DateTime, Utc, Local};
use rusqlite::Connection;
use log::{info, error, fatal};
use regex::Regex;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Provider trait for migration functionality
pub trait MigrateProvider: Send + Sync {
    fn get_id(&self) -> &str;
    fn set_data(&mut self, uid: &str, content: Arc<Mutex<MigrationContent>>, info: Option<&ExportInfo>) -> Result<()>;
    fn export(&self) -> Result<bool>;
    fn import(&self, app_data: &AppData, import_info: &ImportInfo) -> Result<bool>;
}

/// Content structure for migration data
pub struct MigrationContent {
    zip: Arc<Mutex<ZipWriter<File>>>,
    db: Option<Arc<Mutex<Connection>>>,
    temp_files: Vec<PathBuf>,
}

impl MigrationContent {
    fn new(zip: Arc<Mutex<ZipWriter<File>>>, db: Option<Arc<Mutex<Connection>>>) -> Self {
        Self {
            zip,
            db,
            temp_files: Vec::new(),
        }
    }

    fn add_dir(&mut self, path: &Path, recursive: bool, zip_path: &str) -> Result<()> {
        if !path.exists() || !path.is_dir() {
            return Err(format!("Directory not found: {:?}", path).into());
        }

        let base_path = path.to_path_buf();
        let entries = fs::read_dir(path)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() && recursive {
                let relative_path = path.strip_prefix(&base_path)?;
                let new_zip_path = format!("{}/{}", zip_path, relative_path.display());
                self.add_dir(&path, recursive, &new_zip_path)?;
            } else if path.is_file() {
                let relative_path = path.strip_prefix(&base_path)?;
                let file_zip_path = format!("{}/{}", zip_path, relative_path.display());
                
                let mut file = File::open(&path)?;
                let mut contents = Vec::new();
                file.read_to_end(&mut contents)?;
                
                let mut zip = self.zip.lock().unwrap();
                zip.start_file(file_zip_path, FileOptions::default())?;
                zip.write_all(&contents)?;
            }
        }
        
        Ok(())
    }

    fn add_from_string(&mut self, content: &str, path: &str) -> Result<()> {
        let mut zip = self.zip.lock().unwrap();
        zip.start_file(path, FileOptions::default())?;
        zip.write_all(content.as_bytes())?;
        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        // Clean up temp files
        for file in &self.temp_files {
            if file.exists() {
                fs::remove_file(file)?;
            }
        }
        self.temp_files.clear();
        
        // Finish the zip
        let mut zip = self.zip.lock().unwrap();
        zip.finish()?;
        
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TableInfo {
    tables: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AppData {
    tables: Option<Vec<String>>,
    success: bool,
    message: Option<String>,
    version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExportInfo {
    ocversion: Vec<u32>,
    exporttime: u64,
    exportedby: String,
    exporttype: String,
    exporteduser: String,
    apps: HashMap<String, AppData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ImportInfo {
    olduid: String,
    newuid: String,
}

#[derive(Debug)]
struct ExportResult {
    success: bool,
    data: Option<String>,
}

#[derive(Debug)]
struct ImportResult {
    success: bool,
    data: Option<HashMap<String, bool>>,
}

/// Main migration structure
pub struct Migrate {
    providers: Vec<Box<dyn MigrateProvider>>,
    uid: Option<String>,
    zip_path: Option<PathBuf>,
    zip: Option<Arc<Mutex<ZipWriter<File>>>>,
    export_type: Option<String>,
    db_path: Option<PathBuf>,
    db: Option<Arc<Mutex<Connection>>>,
    content: Option<Arc<Mutex<MigrationContent>>>,
}

impl Default for Migrate {
    fn default() -> Self {
        Self::new()
    }
}

impl Migrate {
    /// Create a new migration instance
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            uid: None,
            zip_path: None,
            zip: None,
            export_type: None,
            db_path: None,
            db: None,
            content: None,
        }
    }

    /// Register a new migration provider
    pub fn register_provider(&mut self, provider: Box<dyn MigrateProvider>) {
        self.providers.push(provider);
    }

    /// Find and load providers from installed apps
    fn find_providers(&mut self) -> Result<()> {
        let apps = self.get_all_apps()?;
        
        for app in apps {
            let path = format!("{}/appinfo/migrate.php", self.get_app_path(&app)?);
            if Path::new(&path).exists() {
                // In Rust we'd import modules or register providers directly
                // This is a placeholder for the PHP include functionality
                // Actual implementation would depend on how providers are registered
            }
        }
        
        Ok(())
    }

    /// Export a user or ownCloud instance
    pub fn export(&mut self, uid: Option<&str>, export_type: &str, path: Option<&Path>) -> Result<String> {
        let data_dir = self.get_data_directory()?;
        
        // Validate export type
        let valid_types = vec!["user", "instance", "system", "userfiles"];
        if !valid_types.contains(&export_type) {
            error!("Invalid export type");
            return Ok(serde_json::to_string(&json!({"success": false}))?);
        }
        
        self.export_type = Some(export_type.to_string());
        
        // Handle user ID if export_type is "user"
        if export_type == "user" {
            // Set user ID, defaulting to current user if none provided
            let current_user = self.get_current_user()?;
            self.uid = Some(uid.unwrap_or(&current_user).to_string());
            
            // Check if user exists
            if !self.user_exists(&self.uid.as_ref().unwrap())? {
                return Ok(serde_json::to_string(&json!({"success": false}))?);
            }
        }
        
        // Generate zip filename
        let now = Local::now();
        let date_str = now.format("%y-%m-%d_%H-%i-%s").to_string();
        
        let zip_name = if export_type == "user" {
            format!("oc_export_{}_{}.zip", self.uid.as_ref().unwrap(), date_str)
        } else {
            format!("oc_export_{}_{}.zip", export_type, date_str)
        };
        
        // Determine zip path
        if export_type == "user" {
            let user_dir = Path::new(&data_dir).join(self.uid.as_ref().unwrap());
            self.zip_path = Some(user_dir.join(&zip_name));
        } else if let Some(custom_path) = path {
            // Validate custom path
            if !custom_path.exists() || !self.is_writable(custom_path)? {
                error!("Path supplied is invalid.");
                return Ok(serde_json::to_string(&json!({"success": false}))?);
            }
            self.zip_path = Some(custom_path.join(&zip_name));
        } else {
            // Default path - temp directory
            self.zip_path = Some(PathBuf::from(std::env::temp_dir()).join(&zip_name));
        }
        
        // Create the zip
        if !self.create_zip()? {
            return Ok(serde_json::to_string(&json!({"success": false}))?);
        }
        
        // Find providers
        self.find_providers()?;
        
        let mut export_data = HashMap::new();
        
        match export_type {
            "user" => {
                // Connect to the database
                self.db_path = Some(Path::new(&data_dir).join(self.uid.as_ref().unwrap()).join("migration.db"));
                if !self.connect_db()? {
                    return Ok(serde_json::to_string(&json!({"success": false}))?);
                }
                
                // Create content object
                let content = MigrationContent::new(
                    self.zip.as_ref().unwrap().clone(),
                    self.db.clone(),
                );
                self.content = Some(Arc::new(Mutex::new(content)));
                
                // Export app data
                export_data = self.export_app_data()?;
                
                // Add user directory to zip
                let user_home = self.get_user_home(self.uid.as_ref().unwrap())?;
                self.content.as_ref().unwrap().lock().unwrap().add_dir(
                    &PathBuf::from(user_home),
                    true,
                    "/"
                )?;
            },
            "instance" => {
                // Create content object without DB
                let content = MigrationContent::new(
                    self.zip.as_ref().unwrap().clone(),
                    None,
                );
                self.content = Some(Arc::new(Mutex::new(content)));
                
                // Create a zip compatible with import function
                let mut db_file = std::env::temp_dir();
                db_file.push("owncloud_export_data_XXXXXX");
                
                // Get DB structure (placeholder for actual implementation)
                self.get_db_structure(&db_file)?;
                
                // Read DB export file
                let mut db_export = fs::read_to_string(&db_file)?;
                
                // Replace DB name and prefix with placeholders
                let db_name = self.get_config("dbname", "owncloud")?;
                let db_table_prefix = self.get_config("dbtableprefix", "oc_")?;
                
                let db_name_string = format!("<database>\n\n <name>{}", db_name);
                let db_table_prefix_string = format!("<table>\n\n  <name>{}", db_table_prefix);
                
                db_export = db_export.replace(&db_name_string, "<database>\n\n <name>*dbname*");
                db_export = db_export.replace(&db_table_prefix_string, "<table>\n\n  <name>*dbprefix*");
                
                // Add the export to the zip
                self.content.as_ref().unwrap().lock().unwrap().add_from_string(&db_export, "dbexport.xml")?;
                
                // Add user data
                for user in self.get_users()? {
                    let user_home = self.get_user_home(&user)?;
                    self.content.as_ref().unwrap().lock().unwrap().add_dir(
                        &PathBuf::from(user_home),
                        true,
                        "/userdata/"
                    )?;
                }
            },
            "userfiles" => {
                // Create content object without DB
                let content = MigrationContent::new(
                    self.zip.as_ref().unwrap().clone(),
                    None,
                );
                self.content = Some(Arc::new(Mutex::new(content)));
                
                // Create a zip with all user files
                for user in self.get_users()? {
                    let user_home = self.get_user_home(&user)?;
                    self.content.as_ref().unwrap().lock().unwrap().add_dir(
                        &PathBuf::from(user_home),
                        true,
                        "/"
                    )?;
                }
            },
            "system" => {
                // Create content object without DB
                let content = MigrationContent::new(
                    self.zip.as_ref().unwrap().clone(),
                    None,
                );
                self.content = Some(Arc::new(Mutex::new(content)));
                
                // Create a zip with ownCloud system files
                let server_root = self.get_server_root()?;
                
                self.content.as_ref().unwrap().lock().unwrap().add_dir(
                    &PathBuf::from(&server_root),
                    false,
                    "/"
                )?;
                
                for dir in &[".git", "3rdparty", "apps", "core", "files", "l10n", 
                             "lib", "ocs", "search", "settings", "tests"] {
                    let dir_path = PathBuf::from(&server_root).join(dir);
                    self.content.as_ref().unwrap().lock().unwrap().add_dir(
                        &dir_path,
                        true,
                        "/"
                    )?;
                }
            },
            _ => unreachable!(), // We already validated the export type
        }
        
        // Generate export info
        let export_info = self.get_export_info(&export_data)?;
        if export_info.is_none() {
            return Ok(serde_json::to_string(&json!({"success": false}))?);
        }
        
        // Add export info to zip
        self.content.as_ref().unwrap().lock().unwrap().add_from_string(
            &export_info.unwrap(),
            "export_info.json"
        )?;
        
        // Finalize the zip
        if !self.content.as_ref().unwrap().lock().unwrap().finish().is_ok() {
            return Ok(serde_json::to_string(&json!({"success": false}))?);
        }
        
        Ok(serde_json::to_string(&json!({
            "success": true,
            "data": self.zip_path.as_ref().unwrap().to_string_lossy()
        }))?)
    }

    /// Import a user or ownCloud instance
    pub fn import(&mut self, path: &Path, import_type: &str, uid: Option<&str>) -> Result<String> {
        let data_dir = self.get_data_directory()?;
        
        // Extract the zip
        let extract_path = self.extract_zip(path)?;
        if extract_path.is_none() {
            return Ok(serde_json::to_string(&json!({"success": false}))?);
        }
        let extract_path = extract_path.unwrap();
        
        // Check for export_info.json
        let export_info_path = extract_path.join("export_info.json");
        if !export_info_path.exists() {
            error!("Invalid import file, export_info.json not found");
            return Ok(serde_json::to_string(&json!({"success": false}))?);
        }
        
        // Read and parse export info
        let export_info_str = fs::read_to_string(export_info_path)?;
        let export_info: ExportInfo = serde_json::from_str(&export_info_str)?;
        
        if export_info.exporttype != import_type {
            error!("Invalid import file");
            return Ok(serde_json::to_string(&json!({"success": false}))?);
        }
        
        self.export_type = Some(import_type.to_string());
        
        let current_user = self.get_current_user()?;
        
        // Set up user ID for user imports
        if import_type == "user" {
            self.uid = Some(uid.unwrap_or(&current_user).to_string());
        }
        
        // Verify permissions - admin required for certain operations
        if (import_type == "user" && self.uid.as_ref().unwrap() != &current_user) || import_type != "user" {
            if !self.is_admin_user(&current_user)? {
                error!("Import not permitted.");
                return Ok(serde_json::to_string(&json!({"success": false}))?);
            }
        }
        
        // Handle different import types
        match import_type {
            "user" => {
                // Check if user exists
                if !self.user_exists(self.uid.as_ref().unwrap())? {
                    error!("User doesn't exist");
                    return Ok(serde_json::to_string(&json!({"success": false}))?);
                }
                
                // Validate username
                let re = Regex::new(r"[^a-zA-Z0-9 _\.@\-]")?;
                if re.is_match(&export_info.exporteduser) {
                    error!("Username is not valid");
                    return Ok(serde_json::to_string(&json!({"success": false}))?);
                }
                
                // Copy data
                let user_folder = extract_path.join(&export_info.exporteduser);
                let new_user_folder = PathBuf::from(&data_dir).join(self.uid.as_ref().unwrap());
                
                for entry in fs::read_dir(&user_folder)? {
                    let entry = entry?;
                    let path = entry.path();
                    let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                    
                    if path.is_dir() && file_name != "." && file_name != ".." {
                        let source = user_folder.join(&file_name);
                        let dest = new_user_folder.join(&file_name);
                        self.copy_recursively(&source, &dest)?;
                    }
                }
                
                // Import user app data
                let migration_db = user_folder.join("migration.db");
                let mut apps_imported = HashMap::new();
                
                if migration_db.exists() {
                    apps_imported = match self.import_app_data(
                        &migration_db,
                        &export_info,
                        self.uid.as_ref().unwrap()
                    )? {
                        Some(apps) => apps,
                        None => {
                            return Ok(serde_json::to_string(&json!({"success": false}))?);
                        }
                    };
                }
                
                // Clean up
                self.remove_directory_recursively(&extract_path)?;
                
                return Ok(serde_json::to_string(&json!({
                    "success": true,
                    "data": apps_imported
                }))?);
            },
            "instance" => {
                // EXPERIMENTAL - currently commented out in original code
                return Ok(serde_json::to_string(&json!({"success": false}))?);
            },
            _ => {
                error!("Unsupported import type");
                return Ok(serde_json::to_string(&json!({"success": false}))?);
            }
        }
    }

    /// Recursively delete a directory
    fn remove_directory_recursively(&self, dir: &Path) -> Result<bool> {
        if !dir.exists() || !dir.is_dir() {
            return Ok(false);
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.remove_directory_recursively(&path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
        
        fs::remove_dir(dir)?;
        Ok(true)
    }

    /// Extract zip archive
    fn extract_zip(&self, path: &Path) -> Result<Option<PathBuf>> {
        // Validate path
        if !path.exists() {
            error!("Zip not found");
            return Ok(None);
        }
        
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        
        let now = Local::now();
        let date_str = now.format("%y-%m-%d_%H-%i-%s").to_string();
        let extract_to = std::env::temp_dir()
            .join(format!("oc_import_{}_{}",
                self.export_type.as_ref().unwrap_or(&"unknown".to_string()),
                date_str));
        
        fs::create_dir_all(&extract_to)?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = extract_to.join(file.name());
            
            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
        
        Ok(Some(extract_to))
    }

    /// Connect to the database
    fn connect_db(&mut self) -> Result<bool> {
        if self.db_path.is_none() {
            error!("connect_db() was called without db_path being set");
            return Ok(false);
        }
        
        if self.db.is_none() {
            let conn = Connection::open(self.db_path.as_ref().unwrap())?;
            self.db = Some(Arc::new(Mutex::new(conn)));
        }
        
        Ok(true)
    }

    /// Create app tables in migration database
    fn create_app_tables(&self, app_id: &str) -> Result<Option<Vec<String>>> {
        // This is a simplification - the actual implementation would need to parse XML and create tables
        // using SQL statements via the rusqlite connection
        
        let app_path = self.get_app_path(app_id)?;
        let db_xml_path = PathBuf::from(&app_path).join("appinfo/database.xml");
        
        if !db_xml_path.exists() {
            return Ok(None);
        }
        
        // Read and parse the database.xml file
        let content = fs::read_to_string(db_xml_path)?;
        
        // Replace placeholders
        let uid = self.uid.as_ref().unwrap();
        let content = content.replace("*dbname*", &format!("{}/migration", uid));
        let content = content.replace("*dbprefix*", "");
        
        // Parse XML to extract table names (simplified)
        let mut tables = Vec::new();
        
        // In a real implementation, we would parse the XML properly
        // For now, just extract table names with a simple regex
        let re = Regex::new(r"<table>\s*<name>([^<]+)</name>")?;
        for cap in re.captures_iter(&content) {
            tables.push(cap[1].to_string());
        }
        
        // Execute SQL to create tables
        if let Some(db) = &self.db {
            let mut conn = db.lock().unwrap();
            
            // In a real implementation, we would generate and execute proper SQL here
            // This is just a placeholder
            for table in &tables {
                // Example placeholder - actual SQL would be generated from XML
                conn.execute(&format!("CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY)", table), [])?;
            }
        }
        
        Ok(Some(tables))
    }

    /// Create zip file
    fn create_zip(&mut self) -> Result<bool> {
        if self.zip_path.is_none() {
            error!("create_zip() called but zip_path has not been set");
            return Ok(false);
        }
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = self.zip_path.as_ref().unwrap().parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Create the zip file
        let file = File::create(self.zip_path.as_ref().unwrap())?;
        let zip_writer = ZipWriter::new(file);
        
        self.zip = Some(Arc::new(Mutex::new(zip_writer)));
        
        Ok(true)
    }

    /// Export app data
    fn export_app_data(&mut self) -> Result<HashMap<String, AppData>> {
        let mut return_data: HashMap<String, AppData> = HashMap::new();
        
        for provider in &mut self.providers {
            let provider_id = provider.get_id().to_string();
            
            // Check if app is enabled
            if self.is_app_enabled(&provider_id)? {
                let mut success = true;
                let mut tables = None;
                
                // Check if app uses database
                let app_path = self.get_app_path(&provider_id)?;
                let database_xml = PathBuf::from(&app_path).join("appinfo/database.xml");
                
                if database_xml.exists() {
                    // Create app tables
                    match self.create_app_tables(&provider_id)? {
                        Some(t) => {
                            tables = Some(t);
                        },
                        None => {
                            success = false;
                        }
                    }
                }
                
                if success {
                    // Set provider data
                    provider.set_data(
                        self.uid.as_ref().unwrap(),
                        self.content.as_ref().unwrap().clone(), 
                        None
                    )?;
                    
                    let export_result = provider.export()?;
                    
                    return_data.insert(provider_id.clone(), AppData {
                        tables,
                        success: export_result,
                        message: None,
                        version: Some(self.get_app_version(&provider_id)?),
                    });
                } else {
                    return_data.insert(provider_id.clone(), AppData {
                        tables,
                        success: false,
                        message: Some("Failed to create app tables".to_string()),
                        version: Some(self.get_app_version(&provider_id)?),
                    });
                }
            }
        }
        
        Ok(return_data)
    }

    /// Generate export info JSON
    fn get_export_info(&self, data: &HashMap<String, AppData>) -> Result<Option<String>> {
        let mut info = HashMap::new();
        
        info.insert("ocversion".to_string(), json!(self.get_version()?));
        info.insert("exporttime".to_string(), json!(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()));
        info.insert("exportedby".to_string(), json!(self.get_current_user()?));
        info.insert("exporttype".to_string(), json!(self.export_type.as_ref().unwrap()));
        info.insert("exporteduser".to_string(), json!(self.uid.as_ref().unwrap_or(&"".to_string())));
        info.insert("apps".to_string(), json!(data));
        
        Ok(Some(serde_json::to_string(&info)?))
    }

    /// Import app data
    fn import_app_data(&mut self, db_path: &Path, info: &ExportInfo, uid: &str) -> Result<Option<HashMap<String, bool>>> {
        if db_path.exists() {
            if !self.connect_db()? {
                error!("Failed to connect to migration.db");
                return Ok(None);
            }
        } else {
            error!("Migration.db not found at: {}", db_path.display());
            return Ok(None);
        }
        
        // Find providers
        self.find_providers()?;
        
        // Generate import info
        let import_info = ImportInfo {
            olduid: info.exporteduser.clone(),
            newuid: uid.to_string(),
        };
        
        let mut apps_status = HashMap::new();
        
        for provider in &mut self.providers {
            let id = provider.get_id();
            
            // Check if the app is in the export
            if let Some(app_data) = info.apps.get(id) {
                // Check if app is installed
                if !self.is_app_enabled(id)? {
                    info!("App: {} is not installed, can't import data.", id);
                    apps_status.insert(id.to_string(), "notsupported".to_string());
                    continue;
                }
                
                // Check if export was successful
                if app_data.success {
                    // Connect to DB and create content
                    if !self.connect_db()? {
                        return Ok(None);
                    }
                    
                    let content = MigrationContent::new(
                        self.zip.as_ref().unwrap().clone(),
                        self.db.clone()
                    );
                    
                    let content_arc = Arc::new(Mutex::new(content));
                    
                    // Set provider data
                    provider.

}}}}} // Añadido por reparador automático