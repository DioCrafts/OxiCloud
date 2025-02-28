// Google Drive storage module.
//
// Provides storage backend for Google Drive integration.

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use google_drive3::{api::File as GoogleDriveFile, DriveHub, oauth2, hyper, hyper_rustls};
use oauth2::authenticator::Authenticator;
use serde_json::Value;
use tempfile::NamedTempFile;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use url::Url;

use crate::files::storage::common::Storage;
use crate::files::stream::dir::Dir;
use crate::files::filesystem;
use crate::util::{self, AppConfig};

/// Google Drive storage adapter.
pub struct Google {
    id: String,
    client: Mutex<DriveHub>,
    drive_files: Mutex<HashMap<String, Option<GoogleDriveFile>>>,
}

// Google Doc mimetypes
const FOLDER: &str = "application/vnd.google-apps.folder";
const DOCUMENT: &str = "application/vnd.google-apps.document";
const SPREADSHEET: &str = "application/vnd.google-apps.spreadsheet";
const DRAWING: &str = "application/vnd.google-apps.drawing";
const PRESENTATION: &str = "application/vnd.google-apps.presentation";

// Static store for temporary files mapping
lazy_static::lazy_static! {
    static ref TEMP_FILES: Mutex<HashMap<PathBuf, String>> = Mutex::new(HashMap::new());
}

impl Google {
    /// Creates a new Google Drive storage instance.
    pub async fn new(params: HashMap<String, String>) -> Result<Self, anyhow::Error> {
        if params.get("configured") == Some(&"true".to_string()) && 
           params.contains_key("client_id") && 
           params.contains_key("client_secret") && 
           params.contains_key("token") {
            
            let client_id = params.get("client_id").unwrap();
            let client_secret = params.get("client_secret").unwrap();
            let token_json = params.get("token").unwrap();
            
            let token: Value = serde_json::from_str(token_json)?;
            let created = token["created"].as_i64().unwrap_or(0);
            
            let id = format!("google::{}{}",
                &client_id[..30.min(client_id.len())],
                created
            );
            
            // Set up OAuth authenticator
            let auth = Self::create_auth(
                client_id.to_string(), 
                client_secret.to_string(), 
                token_json.to_string()
            ).await?;
            
            // Create Drive API client
            let client = DriveHub::new(
                hyper::Client::builder().build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()
                        .https_only()
                        .build()
                ),
                auth
            );
            
            Ok(Self {
                id,
                client: Mutex::new(client),
                drive_files: Mutex::new(HashMap::new()),
            })
        } else {
            Err(anyhow::anyhow!("Creating Google Drive storage failed: missing required parameters"))
        }
    }
    
    /// Creates an OAuth authenticator for Google Drive API.
    async fn create_auth(
        client_id: String,
        client_secret: String,
        token_json: String
    ) -> Result<Authenticator<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>, anyhow::Error> {
        let secret = oauth2::ApplicationSecret {
            client_id,
            client_secret,
            auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            redirect_uris: vec!["urn:ietf:wg:oauth:2.0:oob".to_string()],
            ..Default::default()
        };
        
        let token: oauth2::TokenResponse<oauth2::StandardTokenResponse> = 
            serde_json::from_str(&token_json)?;
        
        let authenticator = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::Interactive,
        )
        .persist_tokens_to_disk("/tmp/tokencache.json")
        .build()
        .await?;
        
        // Set the access token we already have
        authenticator.token.write().await.update_token(token);
        
        Ok(authenticator)
    }

    /// Get the Google Drive file object for the specified path.
    async fn get_drive_file(&self, path: &str) -> Result<Option<GoogleDriveFile>, anyhow::Error> {
        // Remove leading and trailing slashes
        let path = path.trim_matches('/');
        
        // Check cache first
        {
            let cache = self.drive_files.lock().await;
            if let Some(file) = cache.get(path) {
                return Ok(file.clone());
            }
        }
        
        // Root directory handling
        if path.is_empty() {
            let client = self.client.lock().await;
            let root = client.files().get("root").doit().await?;
            
            let mut cache = self.drive_files.lock().await;
            cache.insert(path.to_owned(), Some(root));
            return Ok(Some(root));
        }
        
        // For other paths, we need to traverse the hierarchy
        let mut parent_id = {
            match self.get_drive_file("").await? {
                Some(root) => root.id.unwrap_or_default(),
                None => return Ok(None)
            }
        };
        
        let folder_names: Vec<&str> = path.split('/').collect();
        let mut current_path = String::new();
        
        // Loop through each folder of the path to get to the file
        for name in folder_names {
            // Reconstruct path from beginning
            if current_path.is_empty() {
                current_path = name.to_string();
            } else {
                current_path = format!("{}/{}", current_path, name);
            }
            
            // Check if we already have this path cached
            let cached_file = {
                let cache = self.drive_files.lock().await;
                cache.get(&current_path).cloned()
            };
            
            if let Some(file) = cached_file {
                if let Some(f) = file {
                    parent_id = f.id.unwrap_or_default();
                } else {
                    return Ok(None); // File previously marked as missing
                }
            } else {
                // Query the Drive API
                let client = self.client.lock().await;
                let query = format!("title='{}' and '{}' in parents and trashed = false", 
                    name, parent_id);
                
                let result = client.files().list()
                    .q(&query)
                    .doit().await?;
                
                let items = result.items.unwrap_or_default();
                
                if !items.is_empty() {
                    // Google Drive allows files with the same name, but we don't
                    if items.len() > 1 {
                        self.on_duplicate_file_detected(&current_path).await;
                        self.set_drive_file(&current_path, None).await;
                        return Ok(None);
                    } else {
                        let file = items[0].clone();
                        parent_id = file.id.clone().unwrap_or_default();
                        self.set_drive_file(&current_path, Some(file.clone())).await;
                    }
                } else {
                    // Google Docs have no extension in their title, so try without extension
                    if let Some(pos) = current_path.rfind('.') {
                        let path_without_ext = &current_path[0..pos];
                        if let Some(Some(file)) = self.get_drive_file(path_without_ext).await? {
                            // Switch cached GoogleDriveFile to the correct index
                            self.set_drive_file(path_without_ext, None).await;
                            self.set_drive_file(&current_path, Some(file.clone())).await;
                            parent_id = file.id.unwrap_or_default();
                        } else {
                            self.set_drive_file(&current_path, None).await;
                            return Ok(None);
                        }
                    } else {
                        self.set_drive_file(&current_path, None).await;
                        return Ok(None);
                    }
                }
            }
        }
        
        // Return the final file object from the cache
        let cache = self.drive_files.lock().await;
        Ok(cache.get(path).cloned().unwrap_or(None))
    }

    /// Set the Google Drive file object in the cache.
    async fn set_drive_file(&self, path: &str, file: Option<GoogleDriveFile>) {
        let path = path.trim_matches('/').to_string();
        let mut cache = self.drive_files.lock().await;
        
        cache.insert(path.clone(), file);
        
        if file.is_none() {
            // Set all child paths as None
            let keys: Vec<String> = cache.keys()
                .filter(|k| k.starts_with(&path))
                .cloned()
                .collect();
                
            for key in keys {
                cache.insert(key, None);
            }
        }
    }

    /// Write a log message to inform about duplicate file names.
    async fn on_duplicate_file_detected(&self, path: &str) {
        let client = self.client.lock().await;
        if let Ok(about) = client.about().get().doit().await {
            let user = about.name.unwrap_or_else(|| "unknown".to_string());
            log::info!(
                "Ignoring duplicate file name: {} on Google Drive for Google user: {}",
                path, user
            );
        }
    }

    /// Generate file extension for a Google Doc, choosing Open Document formats for download.
    fn get_google_doc_extension(&self, mimetype: &str) -> &'static str {
        match mimetype {
            DOCUMENT => "odt",
            SPREADSHEET => "ods",
            DRAWING => "jpg",
            PRESENTATION => "pdf", // Download as .odp is not available
            _ => "",
        }
    }
    
    /// Write back a temporary file to Google Drive.
    async fn write_back(&self, tmp_file: &Path) -> Result<(), anyhow::Error> {
        let path = {
            let temp_files = TEMP_FILES.lock().await;
            match temp_files.get(tmp_file) {
                Some(p) => p.clone(),
                None => return Ok(()),
            }
        };
        
        let parent_folder = match self.get_drive_file(&path.rsplit_once('/').map(|(p, _)| p).unwrap_or("")).await? {
            Some(folder) => folder,
            None => return Err(anyhow::anyhow!("Parent folder not found")),
        };
        
        // Determine mimetype
        let mimetype = util::get_mime_type(tmp_file).unwrap_or_else(|| "application/octet-stream".to_string());
        
        // Read file contents
        let mut data = Vec::new();
        let mut file = fs::File::open(tmp_file)?;
        file.read_to_end(&mut data)?;
        
        let client = self.client.lock().await;
        
        let result = if let Some(Some(existing_file)) = self.get_drive_file(&path).await? {
            // Update existing file
            let mut file_metadata = GoogleDriveFile::default();
            file_metadata.title = Some(Path::new(&path).file_name().unwrap().to_string_lossy().to_string());
            
            client.files().update(file_metadata, &existing_file.id.unwrap())
                .upload(data.as_slice(), mimetype.parse().unwrap())
                .await?
        } else {
            // Create new file
            let mut file_metadata = GoogleDriveFile::default();
            file_metadata.title = Some(Path::new(&path).file_name().unwrap().to_string_lossy().to_string());
            file_metadata.mime_type = Some(mimetype.clone());
            
            // Set parent folder
            file_metadata.parents = Some(vec![
                google_drive3::api::ParentReference {
                    id: Some(parent_folder.id.unwrap()),
                    ..Default::default()
                }
            ]);
            
            client.files().insert(file_metadata)
                .upload(data.as_slice(), mimetype.parse().unwrap())
                .await?
        };
        
        // Update cache
        self.set_drive_file(&path, Some(result)).await;
        
        // Remove temp file reference
        {
            let mut temp_files = TEMP_FILES.lock().await;
            temp_files.remove(tmp_file);
        }
        
        // Delete temp file
        fs::remove_file(tmp_file)?;
        
        Ok(())
    }
}

#[async_trait]
impl Storage for Google {
    fn get_id(&self) -> &str {
        &self.id
    }
    
    async fn mkdir(&self, path: &str) -> Result<(), anyhow::Error> {
        if self.is_dir(path).await? {
            return Ok(());
        }
        
        let parent_path = Path::new(path).parent().unwrap().to_string_lossy();
        let parent_folder = match self.get_drive_file(&parent_path).await? {
            Some(folder) => folder,
            None => return Err(anyhow::anyhow!("Parent folder not found")),
        };
        
        let mut folder = GoogleDriveFile::default();
        folder.title = Some(Path::new(path).file_name().unwrap().to_string_lossy().to_string());
        folder.mime_type = Some(FOLDER.to_string());
        
        // Set parent reference
        folder.parents = Some(vec![
            google_drive3::api::ParentReference {
                id: Some(parent_folder.id.unwrap()),
                ..Default::default()
            }
        ]);
        
        let client = self.client.lock().await;
        let result = client.files().insert(folder).doit().await?;
        
        self.set_drive_file(path, Some(result)).await;
        
        Ok(())
    }
    
    async fn rmdir(&self, path: &str) -> Result<(), anyhow::Error> {
        let path = path.trim_matches('/');
        
        if path.is_empty() {
            // This is the root directory, clear all files
            let entries = self.opendir(path).await?;
            
            for entry in entries {
                if entry != "." && entry != ".." {
                    self.unlink(&format!("{}/{}", path, entry)).await?;
                }
            }
            
            // Clear drive files cache
            let mut cache = self.drive_files.lock().await;
            cache.clear();
            
            Ok(())
        } else {
            // Delete like a regular file
            self.unlink(path).await
        }
    }
    
    async fn opendir(&self, path: &str) -> Result<Vec<String>, anyhow::Error> {
        let path = path.trim_matches('/');
        
        let folder = match self.get_drive_file(path).await? {
            Some(folder) => folder,
            None => return Err(anyhow::anyhow!("Folder not found")),
        };
        
        let mut files = Vec::new();
        let mut duplicates = HashMap::new();
        let mut page_token: Option<String> = None;
        
        let client = self.client.lock().await;
        
        loop {
            let mut request = client.files().list();
            
            if let Some(token) = page_token {
                request = request.page_token(&token);
            }
            
            let query = format!("'{}' in parents and trashed = false", folder.id.unwrap());
            request = request.q(&query);
            
            let response = request.doit().await?;
            let items = response.items.unwrap_or_default();
            
            for child in items {
                let mut name = child.title.clone().unwrap_or_default();
                
                // Check if this is a Google Doc (no extension)
                if child.file_extension.is_none() && child.mime_type != Some(FOLDER.to_string()) {
                    if let Some(mime_type) = &child.mime_type {
                        name = format!("{}.{}", name, self.get_google_doc_extension(mime_type));
                    }
                }
                
                let filepath = if path.is_empty() {
                    name.clone()
                } else {
                    format!("{}/{}", path, name)
                };
                
                // Google Drive allows files with the same name, ownCloud doesn't
                if files.contains(&name) || duplicates.contains_key(&filepath) {
                    if !duplicates.contains_key(&filepath) {
                        duplicates.insert(filepath.clone(), true);
                        
                        // Remove from files list if it was there
                        if let Some(pos) = files.iter().position(|x| x == &name) {
                            files.remove(pos);
                        }
                        
                        self.set_drive_file(&filepath, None).await;
                        self.on_duplicate_file_detected(&filepath).await;
                    }
                } else {
                    // Cache the file for future use
                    self.set_drive_file(&filepath, Some(child)).await;
                    files.push(name);
                }
            }
            
            // Check if there are more pages
            page_token = response.next_page_token;
            if page_token.is_none() {
                break;
            }
        }
        
        Ok(files)
    }
    
    async fn stat(&self, path: &str) -> Result<HashMap<String, i64>, anyhow::Error> {
        let file = match self.get_drive_file(path).await? {
            Some(file) => file,
            None => return Err(anyhow::anyhow!("File not found")),
        };
        
        let mut stat = HashMap::new();
        
        if self.is_dir(path).await? {
            stat.insert("size".to_string(), 0);
        } else {
            // Check if this is a Google Doc
            let mime_type = self.get_mime_type(path).await?;
            
            if mime_type != file.mime_type.unwrap_or_default() {
                // Return unknown file size for Google Docs
                stat.insert("size".to_string(), -1); // SPACE_UNKNOWN
            } else if let Some(size) = file.file_size {
                stat.insert("size".to_string(), size.parse().unwrap_or(0));
            }
        }
        
        // Parse times
        if let Some(viewed) = file.last_viewed_by_me_date {
            let dt = DateTime::parse_from_rfc3339(&viewed).unwrap_or_default();
            stat.insert("atime".to_string(), dt.timestamp());
        }
        
        if let Some(modified) = file.modified_date {
            let dt = DateTime::parse_from_rfc3339(&modified).unwrap_or_default();
            stat.insert("mtime".to_string(), dt.timestamp());
        }
        
        if let Some(created) = file.created_date {
            let dt = DateTime::parse_from_rfc3339(&created).unwrap_or_default();
            stat.insert("ctime".to_string(), dt.timestamp());
        }
        
        Ok(stat)
    }
    
    async fn filetype(&self, path: &str) -> Result<String, anyhow::Error> {
        if path.is_empty() {
            return Ok("dir".to_string());
        }
        
        let file = match self.get_drive_file(path).await? {
            Some(file) => file,
            None => return Err(anyhow::anyhow!("File not found")),
        };
        
        if file.mime_type == Some(FOLDER.to_string()) {
            Ok("dir".to_string())
        } else {
            Ok("file".to_string())
        }
    }
    
    async fn is_readable(&self, path: &str) -> Result<bool, anyhow::Error> {
        self.file_exists(path).await
    }
    
    async fn is_updatable(&self, path: &str) -> Result<bool, anyhow::Error> {
        let file = match self.get_drive_file(path).await? {
            Some(file) => file,
            None => return Ok(false),
        };
        
        Ok(file.editable.unwrap_or(false))
    }
    
    async fn file_exists(&self, path: &str) -> Result<bool, anyhow::Error> {
        Ok(self.get_drive_file(path).await?.is_some())
    }
    
    async fn unlink(&self, path: &str) -> Result<(), anyhow::Error> {
        let file = match self.get_drive_file(path).await? {
            Some(file) => file,
            None => return Err(anyhow::anyhow!("File not found")),
        };
        
        let client = self.client.lock().await;
        client.files().trash(&file.id.unwrap()).doit().await?;
        
        self.set_drive_file(path, None).await;
        
        Ok(())
    }
    
    async fn rename(&self, path1: &str, path2: &str) -> Result<(), anyhow::Error> {
        let file = match self.get_drive_file(path1).await? {
            Some(file) => file,
            None => return Err(anyhow::anyhow!("Source file not found")),
        };
        
        let path1_dir = Path::new(path1).parent().unwrap().to_string_lossy();
        let path2_dir = Path::new(path2).parent().unwrap().to_string_lossy();
        
        // Create a mutable copy of the file for updates
        let mut updated_file = file.clone();
        
        if path1_dir == path2_dir {
            // Just rename the file
            updated_file.title = Some(Path::new(path2).file_name().unwrap().to_string_lossy().to_string());
        } else {
            // Change file parent
            let parent_folder2 = match self.get_drive_file(&path2_dir).await? {
                Some(folder) => folder,
                None => return Err(anyhow::anyhow!("Destination folder not found")),
            };
            
            updated_file.parents = Some(vec![
                google_drive3::api::ParentReference {
                    id: Some(parent_folder2.id.unwrap()),
                    ..Default::default()
                }
            ]);
        }
        
        let client = self.client.lock().await;
        let result = client.files().update(updated_file, &file.id.unwrap()).doit().await?;
        
        // Update caches
        self.set_drive_file(path1, None).await;
        self.set_drive_file(path2, Some(result)).await;
        
        Ok(())
    }
    
    async fn fopen(&self, path: &str, mode: &str) -> Result<Box<dyn tokio::io::AsyncRead + Unpin + Send>, anyhow::Error> {
        let ext = Path::new(path).extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();
        
        match mode {
            "r" | "rb" => {
                let file = match self.get_drive_file(path).await? {
                    Some(file) => file,
                    None => return Err(anyhow::anyhow!("File not found")),
                };
                
                let mime_type = self.get_mime_type(path).await?;
                let download_url = if file.export_links.is_some() && 
                                    file.export_links.as_ref().unwrap().contains_key(&mime_type) {
                    file.export_links.unwrap().get(&mime_type).cloned()
                } else {
                    file.download_url.clone()
                };
                
                if let Some(url) = download_url {
                    let client = self.client.lock().await;
                    // Use the auth context to download the file
                    let response = client.hub().get_client()
                        .get(url.parse()?)
                        .send()
                        .await?;
                    
                    if response.status().is_success() {
                        let data = response.bytes().await?;
                        let mut tmp_file = tempfile::NamedTempFile::new()?;
                        tmp_file.write_all(&data)?;
                        
                        // Return an async reader for the temp file
                        let file = tokio::fs::File::open(tmp_file.path()).await?;
                        Ok(Box::new(file))
                    } else {
                        Err(anyhow::anyhow!("Failed to download file: {}", response.status()))
                    }
                } else {
                    Err(anyhow::anyhow!("No download URL available"))
                }
            },
            "w" | "wb" | "a" | "ab" | "r+" | "w+" | "wb+" | "a+" | "x" | "x+" | "c" | "c+" => {
                // Create a temporary file for writing
                let mut tmp_file = tempfile::NamedTempFile::new()?;
                let tmp_path = tmp_file.path().to_path_buf();
                
                // If file exists and we're not in write mode, copy content first
                if self.file_exists(path).await? && 
                   (mode == "a" || mode == "ab" || mode == "r+" || 
                    mode == "a+" || mode == "c" || mode == "c+") {
                    let source = self.fopen(path, "r").await?;
                    let mut buffer = Vec::new();
                    tokio::io::copy(&mut source.as_ref(), &mut buffer).await?;
                    tmp_file.write_all(&buffer)?;
                }
                
                // Register this temp file for write-back on close
                {
                    let mut temp_files = TEMP_FILES.lock().await;
                    temp_files.insert(tmp_path.clone(), path.to_string());
                }
                
                // Return an async writer for the temp file
                let file = tokio::fs::File::options()
                    .read(true)
                    .write(true)
                    .open(&tmp_path)
                    .await?;
                
                Ok(Box::new(file))
            },
            _ => Err(anyhow::anyhow!("Unsupported file mode: {}", mode)),
        }
    }
    
    async fn get_mime_type(&self, path: &str) -> Result<String, anyhow::Error> {
        let file = match self.get_drive_file(path).await? {
            Some(file) => file,
            None => return Err(anyhow::anyhow!("File not found")),
        };
        
        let mime_type = file.mime_type.unwrap_or_default();
        
        // Convert Google Doc mimetypes, choosing Open Document formats for download
        match mime_type.as_str() {
            FOLDER => Ok("httpd/unix-directory".to_string()),
            DOCUMENT => Ok("application/vnd.oasis.opendocument.text".to_string()),
            SPREADSHEET => Ok("application/x-vnd.oasis.opendocument.spreadsheet".to_string()),
            DRAWING => Ok("image/jpeg".to_string()),
            PRESENTATION => Ok("application/pdf".to_string()), // Download as .odp is not available
            _ => Ok(mime_type),
        }
    }
    
    async fn free_space(&self, _path: &str) -> Result<i64, anyhow::Error> {
        let client = self.client.lock().await;
        let about = client.about().get().doit().await?;
        
        let quota_total = about.quota_bytes_total.unwrap_or(0);
        let quota_used = about.quota_bytes_used.unwrap_or(0);
        
        Ok(quota_total - quota_used)
    }
    
    async fn touch(&self, path: &str, mtime: Option<i64>) -> Result<(), anyhow::Error> {
        let file = self.get_drive_file(path).await?;
        
        if let Some(file) = file {
            let client = self.client.lock().await;
            
            if let Some(mtime) = mtime {
                // Convert unix timestamp to RFC 3339 format
                let dt = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp_opt(mtime, 0).unwrap_or_default(),
                    Utc
                );
                let date_str = dt.to_rfc3339();
                
                // Create updated file with modified date
                let mut updated_file = file.clone();
                updated_file.modified_date = Some(date_str);
                
                // Update file with modified date
                let result = client.files().update(
                    updated_file, 
                    &file.id.unwrap()
                )
                .set_modified_date(true)
                .doit()
                .await?;
                
                self.set_drive_file(path, Some(result)).await;
            } else {
                // Just touch the file (update modified time to now)
                let result = client.files().touch(&file.id.unwrap()).doit().await?;
                self.set_drive_file(path, Some(result)).await;
            }
        } else {
            // File doesn't exist, create it
            let parent_path = Path::new(path).parent().unwrap().to_string_lossy();
            let parent_folder = match self.get_drive_file(&parent_path).await? {
                Some(folder) => folder,
                None => return Err(anyhow::anyhow!("Parent folder not found")),
            };
            
            let mut new_file = GoogleD

}}} // Añadido por reparador automático