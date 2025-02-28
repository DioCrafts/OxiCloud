use actix_web::{web, HttpResponse, Error};
use actix_multipart::{Field, Multipart};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use std::collections::HashMap;
use uuid::Uuid;
use anyhow::{Result, Context, anyhow};
use tempfile::NamedTempFile;
use std::fs;

const PERMISSION_ALL: u32 = 31; // OCP\PERMISSION_ALL
const PERMISSION_READ: u32 = 1; // OCP\PERMISSION_READ
const PERMISSION_CREATE: u32 = 4; // OCP\PERMISSION_CREATE

#[derive(Debug, Serialize)]
struct FileUploadResult {
    status: String,
    mime: String,
    mtime: i64,
    size: u64,
    id: String,
    name: String,
    etag: String,
    originalname: String,
    upload_max_filesize: i64,
    max_human_filesize: String,
    permissions: u32,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    data: ErrorData,
}

#[derive(Debug, Serialize)]
struct ErrorData {
    message: String,
    upload_max_filesize: i64,
    max_human_filesize: String,
}

#[derive(Debug, Deserialize)]
struct UploadFormData {
    dir: Option<String>,
    dir_token: Option<String>,
    subdir: Option<String>,
    resolution: Option<String>,
}

#[derive(Debug)]
struct StorageStats {
    upload_max_filesize: i64,
    max_human_filesize: String,
}

struct Translator {
    // In a real implementation, this would contain localization data
}

impl Translator {
    fn new() -> Self {
        Self {}
    }

    fn t(&self, msg: &str) -> String {
        // In a real implementation, this would translate the message
        msg.to_string()
    }
}

struct Share {
    file_source: String,
    permissions: u32,
    uid_owner: String,
}

struct Filesystem;

impl Filesystem {
    fn get_path(file_source: &str) -> String {
        // Mock implementation
        format!("/files/{}", file_source)
    }

    fn normalize_path(path: &str) -> String {
        // Simple normalization
        path.replace("//", "/")
    }

    fn file_exists(path: &str) -> bool {
        // Mock implementation
        Path::new(path).exists()
    }

    fn from_tmp_file(tmp_path: &str, target_path: &str) -> Result<bool> {
        // Mock implementation for moving a temp file to the target location
        // In a real implementation, this would handle the file copy/move
        Ok(true)
    }

    fn get_file_info(path: &str) -> Result<Option<FileInfo>> {
        // Mock implementation
        Ok(Some(FileInfo {
            mimetype: "application/octet-stream".to_string(),
            mtime: chrono::Utc::now().timestamp(),
            size: 1024,
            fileid: Uuid::new_v4().to_string(),
            etag: Uuid::new_v4().to_string(),
            permissions: PERMISSION_ALL,
        }))
    }
}

struct FileInfo {
    mimetype: String,
    mtime: i64,
    size: u64,
    fileid: String,
    etag: String,
    permissions: u32,
}

struct FileHelper;

impl FileHelper {
    fn build_file_storage_statistics(dir: &str) -> StorageStats {
        // Mock implementation
        StorageStats {
            upload_max_filesize: 10 * 1024 * 1024, // 10MB
            max_human_filesize: "10 MB".to_string(),
        }
    }

    fn build_not_existing_filename(dir: &str, filename: &str) -> String {
        // Mock implementation for generating a non-conflicting filename
        if Filesystem::file_exists(&format!("{}/{}", dir, filename)) {
            let path = Path::new(filename);
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let ext = path.extension().map_or("", |e| e.to_str().unwrap());
            
            if ext.is_empty() {
                format!("{}/{} (2)", dir, stem)
            } else {
                format!("{}/{} (2).{}", dir, stem, ext)
            }
        } else {
            format!("{}/{}", dir, filename)
        }
    }
}

struct ShareService;

impl ShareService {
    fn get_share_by_token(token: &str) -> Result<Option<Share>> {
        // Mock implementation
        Ok(Some(Share {
            file_source: "somefile".to_string(),
            permissions: PERMISSION_CREATE,
            uid_owner: "user1".to_string(),
        }))
    }

    fn resolve_reshare(share: &Share) -> Share {
        // Mock implementation
        Share {
            file_source: share.file_source.clone(),
            permissions: share.permissions,
            uid_owner: share.uid_owner.clone(),
        }
    }
}

struct UserSession;

impl UserSession {
    fn is_logged_in() -> bool {
        // Mock implementation
        true
    }

    fn tear_down_fs() {
        // Mock implementation
    }

    fn setup_fs(uid: &str) {
        // Mock implementation
    }
}

async fn handle_upload_file(field: Field, tmp_dir: &Path) -> Result<(String, String, u64)> {
    let content_disposition = field.content_disposition().ok_or(anyhow!("No content disposition"))?;
    let filename = content_disposition.get_filename().ok_or(anyhow!("No filename"))?.to_string();
    
    // Create a temp file
    let mut tmp_file = NamedTempFile::new_in(tmp_dir)?;
    let tmp_path = tmp_file.path().to_string_lossy().to_string();
    
    // Save file data to temp file
    let mut size: u64 = 0;
    let mut field_stream = field.into_stream();
    
    while let Some(chunk) = field_stream.next().await {
        let data = chunk.context("Error while reading multipart stream")?;
        size += data.len() as u64;
        tmp_file.write_all(&data).context("Error writing file")?;
    }
    
    Ok((filename, tmp_path, size))
}

pub async fn upload(
    multipart: Multipart,
    form: web::Query<UploadFormData>,
) -> Result<HttpResponse, Error> {
    // Firefox and Konqueror tries to download application/json for me. --Arthur
    let translator = Translator::new();
    let mut allowed_permissions = PERMISSION_ALL;
    
    // Set up the upload directory
    let dir = if let Some(ref dir_token) = form.dir_token {
        // return only read permissions for public upload
        allowed_permissions = PERMISSION_READ;
        
        let link_item = match ShareService::get_share_by_token(dir_token) {
            Ok(Some(item)) => item,
            _ => {
                return Ok(json_error(&translator.t("Invalid Token"), None));
            }
        };
        
        if link_item.permissions & PERMISSION_CREATE == 0 {
            if !UserSession::is_logged_in() {
                return Ok(json_error(&translator.t("Authentication required"), None));
            }
        } else {
            // resolve reshares
            let root_link_item = ShareService::resolve_reshare(&link_item);
            
            // Setup FS with owner
            UserSession::tear_down_fs();
            UserSession::setup_fs(&root_link_item.uid_owner);
            
            // The token defines the target directory (security reasons)
            let path = Filesystem::get_path(&link_item.file_source);
            let subdir = form.subdir.clone().unwrap_or_default();
            format!("/{}/{}", path, subdir)
        }
    } else {
        // The standard case, files are uploaded through logged in users :)
        if !UserSession::is_logged_in() {
            return Ok(json_error(&translator.t("Authentication required"), None));
        }
        
        match &form.dir {
            Some(dir) if !dir.is_empty() => dir.clone(),
            _ => {
                return Ok(json_error(&translator.t("Unable to set upload directory."), None));
            }
        }
    };
    
    if dir.contains("..") {
        return Ok(json_error(&translator.t("Invalid directory."), None));
    }
    
    // get array with current storage stats (e.g. max file size)
    let storage_stats = FileHelper::build_file_storage_statistics(&dir);
    
    // Setup temp directory for file uploads
    let tmp_dir = tempfile::tempdir()?;
    
    // Process multipart form data
    let mut multipart_data = multipart;
    let mut files: Vec<(String, String, u64)> = Vec::new();
    let mut total_size: u64 = 0;
    
    while let Ok(Some(field)) = multipart_data.try_next().await {
        if field.name() == "files" {
            match handle_upload_file(field, tmp_dir.path()).await {
                Ok((filename, tmp_path, size)) => {
                    total_size += size;
                    files.push((filename, tmp_path, size));
                },
                Err(e) => {
                    return Ok(json_error(&format!("Upload error: {}", e), Some(&storage_stats)));
                }
            }
        }
    }
    
    if files.is_empty() {
        return Ok(json_error(&translator.t("No file was uploaded. Unknown error"), Some(&storage_stats)));
    }
    
    // Check total size
    if storage_stats.upload_max_filesize >= 0 && total_size > storage_stats.upload_max_filesize as u64 {
        return Ok(json_error(
            &translator.t("Not enough storage available"), 
            Some(&storage_stats)
        ));
    }
    
    // Process uploads
    let mut results = Vec::new();
    
    for (filename, tmp_path, _) in files {
        let target = if form.resolution.as_deref() == Some("autorename") {
            // append a number in brackets like 'filename (2).ext'
            FileHelper::build_not_existing_filename(&dir, &filename)
        } else {
            Filesystem::normalize_path(&format!("{}/{}", dir, filename))
        };
        
        if !Filesystem::file_exists(&target) || form.resolution.as_deref() == Some("replace") {
            // upload and overwrite file
            match Filesystem::from_tmp_file(&tmp_path, &target) {
                Ok(true) => {
                    // updated max file size after upload
                    let updated_stats = FileHelper::build_file_storage_statistics(&dir);
                    
                    match Filesystem::get_file_info(&target) {
                        Ok(Some(meta)) => {
                            results.push(FileUploadResult {
                                status: "success".to_string(),
                                mime: meta.mimetype,
                                mtime: meta.mtime,
                                size: meta.size,
                                id: meta.fileid,
                                name: Path::new(&target).file_name().unwrap().to_string_lossy().to_string(),
                                etag: meta.etag,
                                originalname: tmp_path,
                                upload_max_filesize: updated_stats.upload_max_filesize,
                                max_human_filesize: updated_stats.max_human_filesize,
                                permissions: meta.permissions & allowed_permissions,
                            });
                        },
                        _ => {
                            return Ok(json_error(&translator.t("Upload failed. Could not get file info."), Some(&storage_stats)));
                        }
                    }
                },
                _ => {
                    return Ok(json_error(&translator.t("Upload failed. Could not find uploaded file"), Some(&storage_stats)));
                }
            }
        } else {
            // file already exists
            match Filesystem::get_file_info(&target) {
                Ok(Some(meta)) => {
                    results.push(FileUploadResult {
                        status: "existserror".to_string(),
                        mime: meta.mimetype,
                        mtime: meta.mtime,
                        size: meta.size,
                        id: meta.fileid,
                        name: Path::new(&target).file_name().unwrap().to_string_lossy().to_string(),
                        etag: meta.etag,
                        originalname: tmp_path,
                        upload_max_filesize: storage_stats.upload_max_filesize,
                        max_human_filesize: storage_stats.max_human_filesize,
                        permissions: meta.permissions & allowed_permissions,
                    });
                },
                _ => {
                    return Ok(json_error(&translator.t("Upload failed. Could not get file info."), Some(&storage_stats)));
                }
            }
        }
    }
    
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .json(results))
}

fn json_error(message: &str, stats: Option<&StorageStats>) -> HttpResponse {
    let data = ErrorData {
        message: message.to_string(),
        upload_max_filesize: stats.map_or(0, |s| s.upload_max_filesize),
        max_human_filesize: stats.map_or_else(|| "0 B".to_string(), |s| s.max_human_filesize.clone()),
    };
    
    HttpResponse::BadRequest()
        .content_type("text/plain")
        .json(ErrorResponse { data })
}