// Mount configuration module
//
// This module is responsible for configuring external storage mounts
// for the ownCloud system.
//
// # License
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
use std::fs::{self, File, create_dir_all, read_dir};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};

use crate::files::storage::{Storage, StorageResult};
use crate::util::{self, L10N};
use crate::config;
use crate::user;

/// Mount types for external storage
pub enum MountType {
    Global,
    Group,
    User,
}

impl MountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MountType::Global => "global",
            MountType::Group => "group",
            MountType::User => "user",
        }
    }
}

/// Backend status information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackendStatus {
    pub status: bool,
}

/// Mount point configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MountPoint {
    pub class: String,
    pub backend: String,
    pub configuration: HashMap<String, String>,
    pub applicable: Option<ApplicableTarget>,
    pub status: Option<bool>,
}

/// Target for a mount point
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ApplicableTarget {
    pub groups: Vec<String>,
    pub users: Vec<String>,
}

/// Mount configuration manager
pub struct MountConfig;

impl MountConfig {
    /// Get details on each of the external storage backends, used for the mount config UI
    /// If a custom UI is needed, add the key 'custom' and a javascript file with that name will be loaded
    /// If the configuration parameter should be secret, add a '*' to the beginning of the value
    /// If the configuration parameter is a boolean, add a '!' to the beginning of the value
    /// If the configuration parameter is optional, add a '&' to the beginning of the value
    /// If the configuration parameter is hidden, add a '#' to the beginning of the value
    pub fn get_backends() -> HashMap<String, Value> {
        let mut backends = HashMap::new();
        
        backends.insert(
            String::from(r"\OC\Files\Storage\Local"),
            json!({
                "backend": "Local",
                "configuration": {
                    "datadir": "Location"
                }
            }),
        );
        
        backends.insert(
            String::from(r"\OC\Files\Storage\AmazonS3"),
            json!({
                "backend": "Amazon S3",
                "configuration": {
                    "key": "Access Key",
                    "secret": "*Secret Key",
                    "bucket": "Bucket",
                    "hostname": "Hostname (optional)",
                    "port": "Port (optional)",
                    "region": "Region (optional)",
                    "use_ssl": "!Enable SSL",
                    "use_path_style": "!Enable Path Style"
                }
            }),
        );
        
        backends.insert(
            String::from(r"\OC\Files\Storage\Dropbox"),
            json!({
                "backend": "Dropbox",
                "configuration": {
                    "configured": "#configured",
                    "app_key": "App key",
                    "app_secret": "App secret",
                    "token": "#token",
                    "token_secret": "#token_secret"
                },
                "custom": "dropbox"
            }),
        );
        
        if Self::check_php_ftp() {
            backends.insert(
                String::from(r"\OC\Files\Storage\FTP"),
                json!({
                    "backend": "FTP",
                    "configuration": {
                        "host": "URL",
                        "user": "Username",
                        "password": "*Password",
                        "root": "&Root",
                        "secure": "!Secure ftps://"
                    }
                }),
            );
        }
        
        if Self::check_curl() {
            backends.insert(
                String::from(r"\OC\Files\Storage\Google"),
                json!({
                    "backend": "Google Drive",
                    "configuration": {
                        "configured": "#configured",
                        "client_id": "Client ID",
                        "client_secret": "Client secret",
                        "token": "#token"
                    },
                    "custom": "google"
                }),
            );
            
            backends.insert(
                String::from(r"\OC\Files\Storage\Swift"),
                json!({
                    "backend": "OpenStack Object Storage",
                    "configuration": {
                        "user": "Username (required)",
                        "bucket": "Bucket (required)",
                        "region": "&Region (optional for OpenStack Object Storage)",
                        "key": "*API Key (required for Rackspace Cloud Files)",
                        "tenant": "&Tenantname (required for OpenStack Object Storage)",
                        "password": "*Password (required for OpenStack Object Storage)",
                        "service_name": "&Service Name (required for OpenStack Object Storage)",
                        "url": "&URL of identity endpoint (required for OpenStack Object Storage)",
                        "timeout": "&Timeout of HTTP requests in seconds (optional)"
                    }
                }),
            );
        }
        
        if !util::running_on_windows() {
            if Self::check_smbclient() {
                backends.insert(
                    String::from(r"\OC\Files\Storage\SMB"),
                    json!({
                        "backend": "SMB / CIFS",
                        "configuration": {
                            "host": "URL",
                            "user": "Username",
                            "password": "*Password",
                            "share": "Share",
                            "root": "&Root"
                        }
                    }),
                );
            }
        }
        
        if Self::check_curl() {
            backends.insert(
                String::from(r"\OC\Files\Storage\DAV"),
                json!({
                    "backend": "ownCloud / WebDAV",
                    "configuration": {
                        "host": "URL",
                        "user": "Username",
                        "password": "*Password",
                        "root": "&Root",
                        "secure": "!Secure https://"
                    }
                }),
            );
        }
        
        backends.insert(
            String::from(r"\OC\Files\Storage\SFTP"),
            json!({
                "backend": "SFTP",
                "configuration": {
                    "host": "URL",
                    "user": "Username",
                    "password": "*Password",
                    "root": "&Root"
                }
            }),
        );
        
        backends.insert(
            String::from(r"\OC\Files\Storage\iRODS"),
            json!({
                "backend": "iRODS",
                "configuration": {
                    "host": "Host",
                    "port": "Port",
                    "use_logon_credentials": "!Use ownCloud login",
                    "user": "Username",
                    "password": "*Password",
                    "auth_mode": "Authentication Mode",
                    "zone": "Zone"
                }
            }),
        );
        
        backends
    }
    
    /// Get the system mount points
    /// The returned array is not in the same format as get_user_mount_points()
    pub fn get_system_mount_points() -> HashMap<String, MountPoint> {
        let mount_points = Self::read_data(false).unwrap_or_default();
        let backends = Self::get_backends();
        let mut system = HashMap::new();
        
        if let Some(group_mounts) = mount_points.get(MountType::Group.as_str()) {
            if let Some(group_mounts) = group_mounts.as_object() {
                for (group, mounts) in group_mounts {
                    if let Some(mounts) = mounts.as_object() {
                        for (mount_point, mount) in mounts {
                            if let (Some(class), Some(options)) = (
                                mount.get("class").and_then(|c| c.as_str()),
                                mount.get("options").and_then(|o| o.as_object()),
                            ) {
                                // Update old classes to new namespace
                                let class_name = if class.starts_with("OC_Filestorage_") {
                                    format!(r"\OC\Files\Storage\{}", &class[15..])
                                } else {
                                    class.to_string()
                                };
                                
                                // Remove '/$user/files/' from mount point
                                let mount_point_name = mount_point[13..].to_string();
                                
                                // Convert options to HashMap
                                let config: HashMap<String, String> = options
                                    .iter()
                                    .filter_map(|(k, v)| v.as_str().map(|v| (k.clone(), v.to_string())))
                                    .collect();
                                
                                // Merge the mount point into the current mount points
                                if let Some(existing) = system.get_mut(&mount_point_name) {
                                    if existing.configuration == config {
                                        if let Some(ref mut applicable) = existing.applicable {
                                            applicable.groups.push(group.clone());
                                        }
                                    } else {
                                        let backend_name = backends
                                            .get(&class_name)
                                            .and_then(|b| b.get("backend"))
                                            .and_then(|b| b.as_str())
                                            .unwrap_or("Unknown")
                                            .to_string();
                                        
                                        let status = Self::get_backend_status(&class_name, &config);
                                        
                                        let mut applicable = ApplicableTarget::default();
                                        applicable.groups.push(group.clone());
                                        
                                        system.insert(
                                            mount_point_name.clone(),
                                            MountPoint {
                                                class: class_name.clone(),
                                                backend: backend_name,
                                                configuration: config,
                                                applicable: Some(applicable),
                                                status: Some(status),
                                            },
                                        );
                                    }
                                } else {
                                    let backend_name = backends
                                        .get(&class_name)
                                        .and_then(|b| b.get("backend"))
                                        .and_then(|b| b.as_str())
                                        .unwrap_or("Unknown")
                                        .to_string();
                                    
                                    let status = Self::get_backend_status(&class_name, &config);
                                    
                                    let mut applicable = ApplicableTarget::default();
                                    applicable.groups.push(group.clone());
                                    
                                    system.insert(
                                        mount_point_name,
                                        MountPoint {
                                            class: class_name,
                                            backend: backend_name,
                                            configuration: config,
                                            applicable: Some(applicable),
                                            status: Some(status),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if let Some(user_mounts) = mount_points.get(MountType::User.as_str()) {
            if let Some(user_mounts) = user_mounts.as_object() {
                for (user, mounts) in user_mounts {
                    if let Some(mounts) = mounts.as_object() {
                        for (mount_point, mount) in mounts {
                            if let (Some(class), Some(options)) = (
                                mount.get("class").and_then(|c| c.as_str()),
                                mount.get("options").and_then(|o| o.as_object()),
                            ) {
                                // Update old classes to new namespace
                                let class_name = if class.starts_with("OC_Filestorage_") {
                                    format!(r"\OC\Files\Storage\{}", &class[15..])
                                } else {
                                    class.to_string()
                                };
                                
                                // Remove '/$user/files/' from mount point
                                let mount_point_name = mount_point[13..].to_string();
                                
                                // Convert options to HashMap
                                let config: HashMap<String, String> = options
                                    .iter()
                                    .filter_map(|(k, v)| v.as_str().map(|v| (k.clone(), v.to_string())))
                                    .collect();
                                
                                // Merge the mount point into the current mount points
                                if let Some(existing) = system.get_mut(&mount_point_name) {
                                    if existing.configuration == config {
                                        if let Some(ref mut applicable) = existing.applicable {
                                            applicable.users.push(user.clone());
                                        }
                                    } else {
                                        let backend_name = backends
                                            .get(&class_name)
                                            .and_then(|b| b.get("backend"))
                                            .and_then(|b| b.as_str())
                                            .unwrap_or("Unknown")
                                            .to_string();
                                        
                                        let status = Self::get_backend_status(&class_name, &config);
                                        
                                        let mut applicable = ApplicableTarget::default();
                                        applicable.users.push(user.clone());
                                        
                                        system.insert(
                                            mount_point_name.clone(),
                                            MountPoint {
                                                class: class_name.clone(),
                                                backend: backend_name,
                                                configuration: config,
                                                applicable: Some(applicable),
                                                status: Some(status),
                                            },
                                        );
                                    }
                                } else {
                                    let backend_name = backends
                                        .get(&class_name)
                                        .and_then(|b| b.get("backend"))
                                        .and_then(|b| b.as_str())
                                        .unwrap_or("Unknown")
                                        .to_string();
                                    
                                    let status = Self::get_backend_status(&class_name, &config);
                                    
                                    let mut applicable = ApplicableTarget::default();
                                    applicable.users.push(user.clone());
                                    
                                    system.insert(
                                        mount_point_name,
                                        MountPoint {
                                            class: class_name,
                                            backend: backend_name,
                                            configuration: config,
                                            applicable: Some(applicable),
                                            status: Some(status),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        
        system
    }
    
    /// Get the personal mount points of the current user
    /// The returned array is not in the same format as get_user_mount_points()
    pub fn get_personal_mount_points() -> HashMap<String, MountPoint> {
        let mount_points = Self::read_data(true).unwrap_or_default();
        let backends = Self::get_backends();
        let uid = user::get_user().unwrap_or_default();
        let mut personal = HashMap::new();
        
        if let Some(user_mounts) = mount_points.get(MountType::User.as_str()) {
            if let Some(user_mounts) = user_mounts.as_object() {
                if let Some(mounts) = user_mounts.get(&uid).and_then(|m| m.as_object()) {
                    for (mount_point, mount) in mounts {
                        if let (Some(class), Some(options)) = (
                            mount.get("class").and_then(|c| c.as_str()),
                            mount.get("options").and_then(|o| o.as_object()),
                        ) {
                            // Update old classes to new namespace
                            let class_name = if class.starts_with("OC_Filestorage_") {
                                format!(r"\OC\Files\Storage\{}", &class[15..])
                            } else {
                                class.to_string()
                            };
                            
                            // Remove '/uid/files/' from mount point
                            let prefix_length = uid.len() + 8;
                            if mount_point.len() > prefix_length {
                                let mount_point_name = mount_point[prefix_length..].to_string();
                                
                                // Convert options to HashMap
                                let config: HashMap<String, String> = options
                                    .iter()
                                    .filter_map(|(k, v)| v.as_str().map(|v| (k.clone(), v.to_string())))
                                    .collect();
                                
                                let backend_name = backends
                                    .get(&class_name)
                                    .and_then(|b| b.get("backend"))
                                    .and_then(|b| b.as_str())
                                    .unwrap_or("Unknown")
                                    .to_string();
                                
                                let status = Self::get_backend_status(&class_name, &config);
                                
                                personal.insert(
                                    mount_point_name,
                                    MountPoint {
                                        class: class_name,
                                        backend: backend_name,
                                        configuration: config,
                                        applicable: None,
                                        status: Some(status),
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
        
        personal
    }
    
    fn get_backend_status(class: &str, options: &HashMap<String, String>) -> bool {
        // Replace $user with the current user
        let uid = user::get_user().unwrap_or_default();
        let mut processed_options = HashMap::new();
        
        for (key, value) in options {
            let processed_value = value.replace("$user", &uid);
            processed_options.insert(key.clone(), processed_value);
        }
        
        // Here would be implementation-specific code to test the storage backend
        // For this example, we'll just return true if the class exists
        // In a real implementation, you would instantiate the storage class and test it
        true // Placeholder
    }
    
    /// Add a mount point to the filesystem
    pub fn add_mount_point(
        mount_point: &str,
        class: &str,
        class_options: HashMap<String, String>,
        mount_type: MountType,
        applicable: &str,
        is_personal: bool,
    ) -> bool {
        if is_personal {
            // Verify that the mount point applies for the current user
            // Prevent non-admin users from mounting local storage
            let current_user = user::get_user().unwrap_or_default();
            if applicable != current_user || class == r"\OC\Files\Storage\Local" {
                return false;
            }
            
            let mount_point = format!("/{}/files/{}", applicable, mount_point.trim_start_matches('/'));
            let mut mount_points = Self::read_data(is_personal).unwrap_or_default();
            
            // Create mount structure
            let mut applicable_mounts = HashMap::new();
            applicable_mounts.insert(
                mount_point,
                json!({
                    "class": class,
                    "options": class_options
                }),
            );
            
            // Ensure the mount type exists
            if !mount_points.contains_key(mount_type.as_str()) {
                mount_points.insert(mount_type.as_str().to_string(), json!({}));
            }
            
            // Get the mount type object
            if let Some(mount_type_obj) = mount_points.get_mut(mount_type.as_str()) {
                if let Some(mount_type_obj) = mount_type_obj.as_object_mut() {
                    // Create or update applicable entry
                    if !mount_type_obj.contains_key(applicable) {
                        mount_type_obj.insert(applicable.to_string(), json!({}));
                    }
                    
                    if let Some(app_mounts) = mount_type_obj.get_mut(applicable) {
                        if let Some(app_mounts) = app_mounts.as_object_mut() {
                            // Add the new mount points
                            for (mount, config) in applicable_mounts {
                                app_mounts.insert(mount, config);
                            }
                        }
                    }
                }
            }
            
            Self::write_data(is_personal, mount_points);
            Self::get_backend_status(class, &class_options)
        } else {
            let mount_point = format!("/$user/files/{}", mount_point.trim_start_matches('/'));
            let mut mount_points = Self::read_data(is_personal).unwrap_or_default();
            
            // Create mount structure
            let mut applicable_mounts = HashMap::new();
            applicable_mounts.insert(
                mount_point,
                json!({
                    "class": class,
                    "options": class_options
                }),
            );
            
            // Ensure the mount type exists
            if !mount_points.contains_key(mount_type.as_str()) {
                mount_points.insert(mount_type.as_str().to_string(), json!({}));
            }
            
            // Get the mount type object
            if let Some(mount_type_obj) = mount_points.get_mut(mount_type.as_str()) {
                if let Some(mount_type_obj) = mount_type_obj.as_object_mut() {
                    // Create or update applicable entry
                    if !mount_type_obj.contains_key(applicable) {
                        mount_type_obj.insert(applicable.to_string(), json!({}));
                    }
                    
                    if let Some(app_mounts) = mount_type_obj.get_mut(applicable) {
                        if let Some(app_mounts) = app_mounts.as_object_mut() {
                            // Add the new mount points
                            for (mount, config) in applicable_mounts {
                                app_mounts.insert(mount, config);
                            }
                        }
                    }
                }
            }
            
            Self::write_data(is_personal, mount_points);
            Self::get_backend_status(class, &class_options)
        }
    }
    
    /// Remove a mount point
    pub fn remove_mount_point(
        mount_point: &str,
        mount_type: MountType,
        applicable: &str,
        is_personal: bool,
    ) -> bool {
        // Verify that the mount point applies for the current user
        if is_personal {
            let current_user = user::get_user().unwrap_or_default();
            if applicable != current_user {
                return false;
            }
            
            let mount_point = format!("/{}/files/{}", applicable, mount_point.trim_start_matches('/'));
            let mut mount_points = Self::read_data(is_personal).unwrap_or_default();
            
            // Remove mount point
            if let Some(mount_type_obj) = mount_points.get_mut(mount_type.as_str()) {
                if let Some(mount_type_obj) = mount_type_obj.as_object_mut() {
                    if let Some(app_mounts) = mount_type_obj.get_mut(applicable) {
                        if let Some(app_mounts) = app_mounts.as_object_mut() {
                            app_mounts.remove(&mount_point);
                            
                            // Clean up empty objects
                            if app_mounts.is_empty() {
                                mount_type_obj.remove(applicable);
                                
                                if mount_type_obj.is_empty() {
                                    mount_points.remove(mount_type.as_str());
                                }
                            }
                        }
                    }
                }
            }
            
            Self::write_data(is_personal, mount_points);
            true
        } else {
            let mount_point = format!("/$user/files/{}", mount_point.trim_start_matches('/'));
            let mut mount_points = Self::read_data(is_personal).unwrap_or_default();
            
            // Remove mount point
            if let Some(mount_type_obj) = mount_points.get_mut(mount_type.as_str()) {
                if let Some(mount_type_obj) = mount_type_obj.as_object_mut() {
                    if let Some(app_mounts) = mount_type_obj.get_mut(applicable) {
                        if let Some(app_mounts) = app_mounts.as_object_mut() {
                            app_mounts.remove(&mount_point);
                            
                            // Clean up empty objects
                            if app_mounts.is_empty() {
                                mount_type_obj.remove(applicable);
                                
                                if mount_type_obj.is_empty() {
                                    mount_points.remove(mount_type.as_str());
                                }
                            }
                        }
                    }
                }
            }
            
            Self::write_data(is_personal, mount_points);
            true
        }
    }
    
    /// Read the mount points in the config file into an array
    fn read_data(is_personal: bool) -> Result<serde_json::Map<String, Value>, String> {
        if is_personal {
            let uid = user::get_user().unwrap_or_default();
            let home_dir = user::get_home(&uid).unwrap_or_default();
            let json_file = format!("{}/mount.json", home_dir);
            let php_file = format!("{}/mount.php", home_dir);
            
            if Path::new(&json_file).exists() {
                return match fs::read_to_string(&json_file) {
                    Ok(content) => match serde_json::from_str(&content) {
                        Ok(value) => {
                            if let Value::Object(map) = value {
                                Ok(map)
                            } else {
                                Ok(serde_json::Map::new())
                            }
                        }
                        Err(_) => Ok(serde_json::Map::new()),
                    },
                    Err(_) => Ok(serde_json::Map::new()),
                };
            } else if Path::new(&php_file).exists() {
                // PHP parsing would go here, but for this conversion 
                // we'll just return an empty map
                return Ok(serde_json::Map::new());
            }
        } else {
            let datadir = config::get_value("datadirectory", &format!("{}/data", util::get_server_root()));
            let json_file = format!("{}/mount.json", datadir);
            let php_file = format!("{}/config/mount.php", util::get_server_root());
            
            if Path::new(&json_file).exists() {
                return match fs::read_to_string(&json_file) {
                    Ok(content) => match serde_json::from_str(&content) {
                        Ok(value) => {
                            if let Value::Object(map) = value {
                                Ok(map)
                            } else {
                                Ok(serde_json::Map::new())
                            }
                        }
                        Err(_) => Ok(serde_json::Map::new()),
                    },
                    Err(_) => Ok(serde_json::Map::new()),
                };
            } else if Path::new(&php_file).exists() {
                // PHP parsing would go here, but for this conversion 
                // we'll just return an empty map
                return Ok(serde_json::Map::new());
            }
        }
        
        Ok(serde_json::Map::new())
    }
    
    /// Write the mount points to the config file
    fn write_data(is_personal: bool, data: serde_json::Map<String, Value>) -> bool {
        let file_path = if is_personal {
            let uid = user::get_user().unwrap_or_default();
            let home_dir = user::get_home(&uid).unwrap_or_default();
            format!("{}/mount.json", home_dir)
        } else {
            let datadir = config::get_value("datadirectory", &format!("{}/data", util::get_server_root()));
            format!("{}/mount.json", datadir)
        };
        
        let content = serde_json::to_string(&data).unwrap_or_default();
        
        match File::create(&file_path) {
            Ok(mut file) => {
                if let Err(_) = file.write_all(content.as_bytes()) {
                    return false;
                }
                
                // Set file permissions
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = fs::metadata(&file_path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o640);
                        let _ = fs::set_permissions(&file_path, perms);
                    }
                }
                
                true
            }
            Err(_) => false,
        }
    }
    
    /// Returns all user uploaded ssl root certificates
    pub fn get_certificates() -> Vec<String> {
        let storage = util::get_storage("files_external");
        let path = format!("{}{}/uploads/", 
            config::get_system_value("datadirectory", ""),
            storage.get_absolute_path(""));
            
        let path = Path::new(&path);
        
        // Create directory if it doesn't exist
        if !path.exists() {
            if let Err(_) = create_dir_all(path) {
                return Vec::new();
            }
        }
        
        let mut result = Vec::new();
        if let Ok(entries) = read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                let file_name = entry.file_name();
                if let Some(name) = file

}}}} // Añadido por reparador automático