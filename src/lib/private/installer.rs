//! This module provides the functionality needed to install, update and remove plugins/apps

use std::collections::HashMap;
use std::fs::{self, File, rename};
use std::io::copy;
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Context, Result};

/// Installer module for managing app installations
pub struct Installer;

impl Installer {
    /// Installs an app
    ///
    /// This function installs an app. All information needed are passed in the
    /// hash map `data`.
    ///
    /// The following keys are required:
    ///   - source: string, can be "path" or "http"
    ///
    /// One of the following keys is required:
    ///   - path: path to the file containing the app
    ///   - href: link to the downloadable file containing the app
    ///
    /// The following keys are optional:
    ///   - pretend: boolean, if set true the system won't do anything
    ///   - noinstall: boolean, if true appinfo/install.php won't be loaded
    ///   - inactive: boolean, if set true the appconfig/app.sample.php won't be
    ///     renamed
    ///
    /// This function works as follows
    ///   -# fetching the file
    ///   -# unzipping it
    ///   -# check the code
    ///   -# installing the database at appinfo/database.xml
    ///   -# including appinfo/install.php
    ///   -# setting the installed version
    ///
    /// It is the task of oc_app_install to create the tables and do whatever is
    /// needed to get the app working.
    pub fn install_app(data: HashMap<String, serde_json::Value>) -> Result<String> {
        let l10n = L10n::get("lib");

        if !data.contains_key("source") {
            return Err(anyhow!(l10n.t("No source specified when installing app")));
        }

        // Download the file if necessary
        let path = if data["source"] == "http" {
            let tmp_file = Helper::tmp_file()?;
            
            if !data.contains_key("href") {
                return Err(anyhow!(l10n.t("No href specified when installing app from http")));
            }
            
            let href = data["href"].as_str().unwrap();
            let mut response = reqwest::blocking::get(href)?;
            let mut file = File::create(&tmp_file)?;
            copy(&mut response, &mut file)?;
            
            tmp_file
        } else {
            if !data.contains_key("path") {
                return Err(anyhow!(l10n.t("No path specified when installing app from local file")));
            }
            data["path"].as_str().unwrap().to_string()
        };

        // Detect the archive type
        let mime = Helper::get_mime_type(&path)?;
        let archive_path = match mime.as_str() {
            "application/zip" => {
                let new_path = format!("{}.zip", path);
                rename(&path, &new_path)?;
                new_path
            },
            "application/x-gzip" => {
                let new_path = format!("{}.tgz", path);
                rename(&path, &new_path)?;
                new_path
            },
            _ => return Err(anyhow!(l10n.t("Archives of type {} are not supported", mime)))
        };

        // Extract the archive in a temporary folder
        let extract_dir = Helper::tmp_folder()?;
        Helper::rmdir_recursive(&extract_dir)?;
        fs::create_dir_all(&extract_dir)?;
        
        let archive = Archive::open(&archive_path).ok_or_else(|| {
            if data["source"] == "http" {
                fs::remove_file(&archive_path).unwrap_or(());
            }
            anyhow!(l10n.t("Failed to open archive when installing app"))
        })?;
        
        archive.extract(&extract_dir).map_err(|e| {
            Helper::rmdir_recursive(&extract_dir).unwrap_or(());
            if data["source"] == "http" {
                fs::remove_file(&archive_path).unwrap_or(());
            }
            e
        })?;

        // Load the info.xml file of the app
        let info_path = Self::find_info_xml(&extract_dir)?;
        
        if !Path::new(&info_path).exists() {
            Helper::rmdir_recursive(&extract_dir)?;
            if data["source"] == "http" {
                fs::remove_file(&archive_path)?;
            }
            return Err(anyhow!(l10n.t("App does not provide an info.xml file")));
        }
        
        let info = App::get_app_info(&info_path, true)?;
        
        // Check the code for not allowed calls
        if !Self::check_code(&info["id"].as_str().unwrap(), &extract_dir)? {
            Helper::rmdir_recursive(&extract_dir)?;
            return Err(anyhow!(l10n.t("App can't be installed because of not allowed code in the App")));
        }

        // Check app compatibility with this version of ownCloud
        if !info.contains_key("require") 
            || !App::is_app_version_compatible(Util::get_version(), info["require"].clone())? 
        {
            Helper::rmdir_recursive(&extract_dir)?;
            return Err(anyhow!(l10n.t("App can't be installed because it is not compatible with this version of ownCloud")));
        }

        // Check if shipped tag is set which is only allowed for apps that are shipped with ownCloud
        if info.contains_key("shipped") && info["shipped"] == "true" {
            Helper::rmdir_recursive(&extract_dir)?;
            return Err(anyhow!(l10n.t("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps")));
        }

        // Check if the ocs version is the same as the version in info.xml/version
        if !info.contains_key("version") 
            || info["version"] != data.get("appdata").and_then(|v| v.get("version")).unwrap_or(&serde_json::Value::Null) 
        {
            Helper::rmdir_recursive(&extract_dir)?;
            return Err(anyhow!(l10n.t("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store")));
        }

        let app_id = info["id"].as_str().unwrap();
        let base_dir = Path::new(App::get_install_path()).join(app_id);
        
        // Check if the destination directory already exists
        if base_dir.exists() {
            Helper::rmdir_recursive(&extract_dir)?;
            if data["source"] == "http" {
                fs::remove_file(&archive_path)?;
            }
            return Err(anyhow!(l10n.t("App directory already exists")));
        }

        if data.get("pretend").and_then(|v| v.as_bool()).unwrap_or(false) {
            return Ok(app_id.to_string());
        }

        // Copy the app to the correct place
        fs::create_dir_all(&base_dir)
            .map_err(|_| anyhow!(l10n.t("Can't create app folder. Please fix permissions. {}", base_dir.display())))?;
        
        Helper::copy_recursive(&extract_dir, &base_dir)?;

        // Remove temporary files
        Helper::rmdir_recursive(&extract_dir)?;
        if data["source"] == "http" {
            fs::remove_file(&archive_path)?;
        }

        // Install the database
        let db_path = base_dir.join("appinfo/database.xml");
        if db_path.exists() {
            if Appconfig::get_value(app_id, "installed_version").is_none() {
                DB::create_db_from_structure(&db_path)?;
            } else {
                DB::update_db_from_structure(&db_path)?;
            }
        }

        // Run appinfo/install.php
        let no_install = data.get("noinstall").and_then(|v| v.as_bool()).unwrap_or(false);
        let install_path = base_dir.join("appinfo/install.php");
        if !no_install && install_path.exists() {
            // In Rust we'd call the PHP runtime or have a different mechanism
            // to load and execute the install script
            // Here we'd use FFI or a PHP execution library
            php_executor::run_file(&install_path)?;
        }

        // Set the installed version
        Appconfig::set_value(app_id, "installed_version", &App::get_app_version(app_id)?)?;
        Appconfig::set_value(app_id, "enabled", "no")?;

        // Set remote/public handlers
        if let Some(remotes) = info.get("remote").and_then(|v| v.as_object()) {
            for (name, path) in remotes {
                let path_str = path.as_str().unwrap_or_default();
                Config::set_app_value("core", &format!("remote_{}", name), &format!("{}/{}", app_id, path_str))?;
            }
        }
        
        if let Some(publics) = info.get("public").and_then(|v| v.as_object()) {
            for (name, path) in publics {
                let path_str = path.as_str().unwrap_or_default();
                Config::set_app_value("core", &format!("public_{}", name), &format!("{}/{}", app_id, path_str))?;
            }
        }

        App::set_app_types(app_id)?;

        Ok(app_id.to_string())
    }

    /// Finds the info.xml file in the extracted directory
    fn find_info_xml(extract_dir: &str) -> Result<String> {
        let info_path = format!("{}/appinfo/info.xml", extract_dir);
        if Path::new(&info_path).exists() {
            return Ok(info_path);
        }

        // Try to find it in a subdirectory
        for entry in fs::read_dir(extract_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let file_name = path.file_name().unwrap().to_string_lossy();
                if !file_name.starts_with('.') {
                    let subdir_info_path = path.join("appinfo/info.xml");
                    if subdir_info_path.exists() {
                        return Ok(subdir_info_path.to_string_lossy().into_owned());
                    }
                }
            }
        }
        
        Ok(info_path)
    }

    /// Checks whether or not an app is installed
    ///
    /// Checks whether or not an app is installed, i.e. registered in apps table.
    pub fn is_installed(app: &str) -> bool {
        Appconfig::get_value(app, "installed_version").is_some()
    }

    /// Update an application
    ///
    /// This function updates an app. All information needed are passed in the
    /// associative array $data.
    pub fn update_app(app: &str) -> Result<bool> {
        let ocsid = Appconfig::get_value(app, "ocsid").unwrap_or_default();
        App::disable(app)?;
        App::enable(&ocsid)?;
        Ok(true)
    }

    /// Check if an update for the app is available
    ///
    /// The function will check if an update for a version is available
    pub fn is_update_available(app: &str) -> Option<String> {
        let ocsid = Appconfig::get_value(app, "ocsid").unwrap_or_default();
        
        if !ocsid.is_empty() {
            if let Ok(ocs_data) = OCSClient::get_application(&ocsid) {
                let ocs_version = ocs_data["version"].as_str().unwrap_or_default().to_string();
                if let Ok(current_version) = App::get_app_version(app) {
                    if ocs_version != current_version {
                        return Some(ocs_version);
                    }
                }
            }
        }
        
        None
    }

    /// Check if app is already downloaded
    ///
    /// The function will check if the app is already downloaded in the apps repository
    pub fn is_downloaded(name: &str) -> bool {
        for app_root in APP_ROOTS.iter() {
            let app_path = Path::new(&app_root.path).join(name);
            if app_path.is_dir() {
                return true;
            }
        }
        false
    }

    /// Removes an app
    ///
    /// This function removes an app.
    pub fn remove_app(name: &str, options: HashMap<String, bool>) -> Result<()> {
        // Handle preferences if needed
        if options.get("keeppreferences").map_or(true, |v| !v) {
            // TODO: Remove preferences
        }

        // Handle app config if needed
        if options.get("keepappconfig").map_or(true, |v| !v) {
            // TODO: Remove app config
        }

        // Handle tables if needed
        if options.get("keeptables").map_or(true, |v| !v) {
            // TODO: Remove app database tables
        }

        // Handle files if needed
        if options.get("keepfiles").map_or(true, |v| !v) {
            // TODO: Remove user files
        }

        if Self::is_downloaded(name) {
            let app_dir = Path::new(App::get_install_path()).join(name);
            Helper::rmdir_recursive(app_dir.to_str().unwrap())?;
            Ok(())
        } else {
            Log::write("core", &format!("can't remove app {}. It is not installed.", name), Log::ERROR);
            Err(anyhow!("App is not installed"))
        }
    }

    /// Installs shipped apps
    ///
    /// This function installs all apps found in the 'apps' directory that should be enabled by default
    pub fn install_shipped_apps() -> Result<()> {
        for app_dir in APP_ROOTS.iter() {
            let path = Path::new(&app_dir.path);
            if path.exists() && path.is_dir() {
                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    
                    if !file_name.starts_with('.') && entry.path().is_dir() {
                        let app_path = entry.path().join("appinfo/app.php");
                        
                        if app_path.exists() && !Self::is_installed(&file_name) {
                            if let Ok(info) = App::get_app_info(&file_name) {
                                if let Some(true) = info.get("default_enable").and_then(|v| v.as_bool()) {
                                    Self::install_shipped_app(&file_name)?;
                                    Appconfig::set_value(&file_name, "enabled", "yes")?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Install an app already placed in the app folder
    ///
    /// @param string $app id of the app to install
    /// @returns string app id
    pub fn install_shipped_app(app: &str) -> Result<String> {
        // Install the database
        let db_path = Path::new(App::get_app_path(app)).join("appinfo/database.xml");
        if db_path.exists() {
            DB::create_db_from_structure(&db_path)?;
        }

        // Run appinfo/install.php
        let install_path = Path::new(App::get_app_path(app)).join("appinfo/install.php");
        if install_path.exists() {
            // In Rust we'd call the PHP runtime or have a different mechanism
            php_executor::run_file(&install_path)?;
        }
        
        let info = App::get_app_info(app)?;
        Appconfig::set_value(app, "installed_version", &App::get_app_version(app)?)?;

        // Set remote/public handlers
        if let Some(remotes) = info.get("remote").and_then(|v| v.as_object()) {
            for (name, path) in remotes {
                let path_str = path.as_str().unwrap_or_default();
                Config::set_app_value("core", &format!("remote_{}", name), &format!("{}/{}", app, path_str))?;
            }
        }
        
        if let Some(publics) = info.get("public").and_then(|v| v.as_object()) {
            for (name, path) in publics {
                let path_str = path.as_str().unwrap_or_default();
                Config::set_app_value("core", &format!("public_{}", name), &format!("{}/{}", app, path_str))?;
            }
        }

        let app_id = info["id"].as_str().unwrap_or(app);
        App::set_app_types(app_id)?;

        Ok(app_id.to_string())
    }

    /// Check the code of an app with some static code checks
    ///
    /// @param string $app_name name of the app to check
    /// @param string $folder the folder of the app to check
    /// @returns bool true for app is o.k. and false for app is not o.k.
    pub fn check_code(app_name: &str, folder: &str) -> Result<bool> {
        let blacklist = [
            "exec(",
            "eval(",
            // More evil pattern will go here later

            // Classes replaced by the public api
            "OC_API::",
            "OC_App::",
            "OC_AppConfig::",
            "OC_Avatar",
            "OC_BackgroundJob::",
            "OC_Config::",
            "OC_DB::",
            "OC_Files::",
            "OC_Helper::",
            "OC_Hook::",
            "OC_Image::",
            "OC_JSON::",
            "OC_L10N::",
            "OC_Log::",
            "OC_Mail::",
            "OC_Preferences::",
            "OC_Request::",
            "OC_Response::",
            "OC_Template::",
            "OC_User::",
            "OC_Util::",
        ];

        // Is the code checker enabled?
        if Config::get_value("appcodechecker", false)? {
            // Check if grep is installed
            let grep = std::process::Command::new("which")
                .arg("grep")
                .output()?;
                
            if !grep.status.success() {
                Log::write(
                    "core",
                    &format!("grep not installed. So checking the code of the app \"{}\" was not possible", app_name),
                    Log::ERROR
                );
                return Ok(true);
            }

            // Iterate the bad patterns
            for bl in blacklist.iter() {
                let output = std::process::Command::new("grep")
                    .arg("-ri")
                    .arg(bl)
                    .arg(folder)
                    .output()?;
                
                // Bad pattern found
                if !String::from_utf8_lossy(&output.stdout).is_empty() {
                    Log::write(
                        "core",
                        &format!("App \"{}\" is using a not allowed call \"{}\". Installation refused.", app_name, bl),
                        Log::ERROR
                    );
                    return Ok(false);
                }
            }
            Ok(true)
        } else {
            Ok(true)
        }
    }
}

// Stubs for the referenced types - in a real implementation these would be imported from other modules
struct L10n;
struct Helper;
struct Archive;
struct App;
struct Util;
struct Appconfig;
struct DB;
struct Config;
struct OCSClient;
struct Log;
struct php_executor;

impl L10n {
    fn get(module: &str) -> Self {
        Self
    }
    
    fn t(&self, text: &str) -> String {
        text.to_string()
    }
    
    fn t(&self, text: &str, args: impl AsRef<[String]>) -> String {
        format!("{} {:?}", text, args.as_ref())
    }
}

// This would be defined elsewhere
static APP_ROOTS: &[AppRoot] = &[];

struct AppRoot {
    path: String,
}