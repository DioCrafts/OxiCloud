use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{self, File, DirBuilder};
use std::io;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use url::Url;
use once_cell::sync::Lazy;

// Simulación de la sesión PHP
static SESSION: Lazy<Arc<Mutex<HashMap<String, HashMap<String, String>>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// Trait equivalente a la clase base StreamWrapper
#[async_trait]
pub trait StreamWrapper: Send + Sync {
    async fn file_exists(&self, path: &str) -> bool;
    async fn is_dir(&self, path: &str) -> bool;
    async fn opendir(&self, path: &str) -> io::Result<Vec<String>>;
    async fn filemtime(&self, path: &str) -> Option<u64>;
    async fn file_get_contents(&self, path: &str) -> io::Result<Vec<u8>>;
    async fn file_put_contents(&self, path: &str, data: &[u8]) -> io::Result<usize>;
}

pub struct IRods {
    password: String,
    user: String,
    host: String,
    port: u16,
    zone: String,
    root: String,
    use_logon_credentials: bool,
    auth_mode: String,
}

impl IRods {
    pub fn new(params: HashMap<String, String>) -> Result<Self, &'static str> {
        if !params.contains_key("host") || !params.contains_key("zone") {
            return Err("Missing required parameters");
        }

        let host = params.get("host").unwrap().clone();
        let port = params.get("port")
            .map(|p| p.parse::<u16>().unwrap_or(1247))
            .unwrap_or(1247);
        let user = params.get("user").cloned().unwrap_or_default();
        let password = params.get("password").cloned().unwrap_or_default();
        let use_logon_credentials = params.get("use_logon_credentials")
            .map(|p| p == "true")
            .unwrap_or(false);
        let zone = params.get("zone").unwrap().clone();
        let auth_mode = params.get("auth_mode").cloned().unwrap_or_default();

        let mut root = params.get("root").cloned().unwrap_or_else(|| "/".to_string());
        if root.is_empty() || !root.starts_with('/') {
            root = format!("/{}", root);
        }

        let mut irods = IRods {
            password,
            user,
            host,
            port,
            zone,
            root,
            use_logon_credentials,
            auth_mode,
        };

        // Take user and password from the session if needed
        if use_logon_credentials {
            if let Some(credentials) = SESSION.lock().unwrap().get("irods-credentials") {
                if let (Some(uid), Some(password)) = (credentials.get("uid"), credentials.get("password")) {
                    irods.user = uid.clone();
                    irods.password = password.clone();
                }
            }
        }

        // Create the root folder if necessary
        if !irods.is_dir("").await {
            if let Err(_) = irods.mkdir("").await {
                return Err("Could not create root directory");
            }
        }

        Ok(irods)
    }

    pub fn login(params: HashMap<String, String>) {
        SESSION.lock().unwrap().insert("irods-credentials".to_string(), params);
    }

    pub fn get_id(&self) -> String {
        format!("irods::{}@{}/{}", self.user, self.host, self.root)
    }

    /// Construct the rods URL
    fn construct_url(&self, path: &str) -> String {
        let path = path.trim_end_matches('/');
        let path = if path.is_empty() || !path.starts_with('/') {
            format!("/{}", path)
        } else {
            path.to_string()
        };

        // Adding auth method
        let mut user_with_zone = format!("{}.{}", self.user, self.zone);
        if !self.auth_mode.is_empty() {
            user_with_zone = format!("{}.{}", user_with_zone, self.auth_mode);
        }

        // URL wrapper schema is named rods
        format!("rods://{}:{}@{}:{}{}{}",
                user_with_zone, self.password, self.host, self.port, self.root, path)
    }

    /// Get the file type
    pub async fn filetype(&self, path: &str) -> Option<String> {
        // This would need to call the iRODS API
        // For now, simulating based on directory status
        if self.is_dir(path).await {
            Some("dir".to_string())
        } else if self.file_exists(path).await {
            Some("file".to_string())
        } else {
            None
        }
    }

    /// Create a directory
    pub async fn mkdir(&self, path: &str) -> io::Result<()> {
        // This would need to call the iRODS API
        // Simulating the operation for now
        Ok(())
    }

    /// Touch a file (create or update timestamp)
    pub async fn touch(&self, path: &str, mtime: Option<u64>) -> io::Result<bool> {
        // We cannot set a time
        if mtime.is_some() {
            return Ok(false);
        }

        let url = self.construct_url(path);

        // If the file doesn't exist we create it
        if !self.file_exists(path).await {
            self.file_put_contents(path, &[]).await?;
            return Ok(true);
        }

        // mtime updates are not supported
        Ok(false)
    }

    /// Check if a file or folder has been updated since a given time
    pub async fn has_updated(&self, path: &str, time: u64) -> bool {
        // This is a work around for folder mtimes -> we loop its content
        if self.is_dir(path).await {
            if let Some(actual_time) = self.collection_mtime(path).await {
                return actual_time > time;
            }
            return false;
        }

        if let Some(actual_time) = self.filemtime(path).await {
            return actual_time > time;
        }
        
        false
    }

    /// Get the best guess for the modification time of an iRODS collection
    async fn collection_mtime(&self, path: &str) -> Option<u64> {
        let files = match self.opendir(path).await {
            Ok(files) => files,
            Err(_) => return self.filemtime(path).await,
        };

        let mut last_ctime = self.filemtime(path).await.unwrap_or(0);

        for file in files {
            if file != "." && file != ".." {
                if let Some(time) = self.filemtime(&file).await {
                    if time > last_ctime {
                        last_ctime = time;
                    }
                }
            }
        }

        Some(last_ctime)
    }
}

#[async_trait]
impl StreamWrapper for IRods {
    async fn file_exists(&self, path: &str) -> bool {
        // Implementation would call iRODS API
        // Simulating for now
        true
    }

    async fn is_dir(&self, path: &str) -> bool {
        // Implementation would call iRODS API
        // Simulating for now
        path.is_empty() || path.ends_with('/')
    }

    async fn opendir(&self, path: &str) -> io::Result<Vec<String>> {
        // Implementation would call iRODS API
        // Simulating for now
        Ok(vec![])
    }

    async fn filemtime(&self, path: &str) -> Option<u64> {
        // Implementation would call iRODS API
        // Simulating for now
        Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        )
    }

    async fn file_get_contents(&self, path: &str) -> io::Result<Vec<u8>> {
        // Implementation would call iRODS API
        // Simulating for now
        Ok(vec![])
    }

    async fn file_put_contents(&self, path: &str, data: &[u8]) -> io::Result<usize> {
        // Implementation would call iRODS API
        // Simulating for now
        Ok(data.len())
    }
}