/**
 * ownCloud
 *
 * @author Michael Gapczynski
 * @author Christian Berendt
 * @copyright 2012 Michael Gapczynski mtgap@owncloud.com
 * @copyright 2013 Christian Berendt berendt@b1-systems.de
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;

use aws_sdk_s3::{Client as S3Client, Config};
use aws_sdk_s3::model::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::error::SdkError;
use aws_smithy_http::result::SdkError as SmithySdkError;
use aws_config::meta::region::RegionProviderChain;
use aws_types::region::Region;
use aws_credential_types::provider::SharedCredentialsProvider;
use tokio::runtime::Runtime;

use oc_files::storage::common::Storage as CommonStorage;
use oc_files::stream::dir::Dir;
use oc_files::stream::close::Close;
use oc_files::helper::Helper;
use oc_files::util::Util;

pub struct AmazonS3 {
    connection: S3Client,
    bucket: String,
    test: bool,
    timeout: u64,
    id: String,
    tmp_files: HashMap<String, String>,
    runtime: Runtime,
}

impl AmazonS3 {
    pub fn new(params: HashMap<String, String>) -> Result<Self, Box<dyn std::error::Error>> {
        if !params.contains_key("key") || !params.contains_key("secret") || !params.contains_key("bucket") {
            return Err("Access Key, Secret and Bucket have to be configured.".into());
        }

        let key = params.get("key").unwrap();
        let secret = params.get("secret").unwrap();
        let bucket = params.get("bucket").unwrap();
        let id = format!("amazon::{}{}",key, md5::compute(secret).to_string());
        
        let use_ssl = params.get("use_ssl").map_or("true", |v| v) != "false";
        let scheme = if use_ssl { "https" } else { "http" };
        let test = params.contains_key("test");
        let timeout = params.get("timeout").map_or(15, |v| v.parse().unwrap_or(15));
        let region = params.get("region").map_or("eu-west-1", |v| v);
        let hostname = params.get("hostname").map_or("s3.amazonaws.com", |v| v);
        let port = params.get("port").map_or(if use_ssl { 443 } else { 80 }, |v| v.parse().unwrap_or(if use_ssl { 443 } else { 80 }));
        
        let base_url = format!("{}://{}:{}/", scheme, hostname, port);

        let runtime = Runtime::new()?;
        
        let credentials_provider = SharedCredentialsProvider::new(
            aws_credential_types::Credentials::new(
                key,
                secret,
                None,
                None,
                "static",
            )
        );

        let region_provider = RegionProviderChain::first_try(Region::new(region.to_string()));
        
        let config = runtime.block_on(async {
            Config::builder()
                .region(region_provider.region().await)
                .endpoint_url(base_url)
                .credentials_provider(credentials_provider)
                .build()
        });

        let client = S3Client::from_conf(config);
        
        let s3 = AmazonS3 {
            connection: client,
            bucket: bucket.to_string(),
            test,
            timeout,
            id,
            tmp_files: HashMap::new(),
            runtime,
        };

        // Check if bucket name is valid
        if !s3.is_valid_bucket_name() {
            return Err("The configured bucket name is invalid.".into());
        }

        // Check if bucket exists, if not create it
        if !s3.does_bucket_exist() {
            let result = s3.create_bucket()?;
            s3.wait_until_bucket_exists()?;
            s3.test_timeout();
        }

        // Create root directory if not exists
        if !s3.file_exists(".") {
            s3.runtime.block_on(async {
                s3.connection.put_object()
                    .bucket(&s3.bucket)
                    .key(".")
                    .body(ByteStream::from_static(&[]))
                    .content_type("httpd/unix-directory")
                    .content_length(0)
                    .send()
                    .await
            })?;
            s3.test_timeout();
        }

        Ok(s3)
    }

    fn normalize_path(&self, path: &str) -> String {
        let mut path = path.trim_matches('/').to_string();
        
        if path.is_empty() {
            path = ".".to_string();
        }
        
        path
    }

    fn test_timeout(&self) {
        if self.test {
            thread::sleep(Duration::from_secs(self.timeout));
        }
    }

    fn is_valid_bucket_name(&self) -> bool {
        // Simple validation - AWS SDK for Rust doesn't provide direct validation
        // Actual S3 bucket name rules: 3-63 characters, lowercase, numbers, dots, and hyphens
        // Cannot be IP address formatted and must start and end with letter or number
        let name = &self.bucket;
        let len = name.len();
        
        if len < 3 || len > 63 {
            return false;
        }
        
        if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-') {
            return false;
        }
        
        true
    }

    fn does_bucket_exist(&self) -> bool {
        let result = self.runtime.block_on(async {
            match self.connection.head_bucket()
                .bucket(&self.bucket)
                .send()
                .await {
                    Ok(_) => true,
                    Err(_) => false,
                }
        });
        
        result
    }

    fn create_bucket(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.runtime.block_on(async {
            let constraint = BucketLocationConstraint::from(self.bucket.as_str());
            
            let config = CreateBucketConfiguration::builder()
                .location_constraint(constraint)
                .build();
                
            self.connection.create_bucket()
                .bucket(&self.bucket)
                .create_bucket_configuration(config)
                .send()
                .await
        })?;
        
        Ok(())
    }

    fn wait_until_bucket_exists(&self) -> Result<(), Box<dyn std::error::Error>> {
        let max_attempts = 15;
        let interval = Duration::from_secs(1);
        
        for _ in 0..max_attempts {
            if self.does_bucket_exist() {
                return Ok(());
            }
            thread::sleep(interval);
        }
        
        Err("Timeout waiting for bucket to exist".into())
    }

    fn write_back(&mut self, tmp_file: &str) -> bool {
        if !self.tmp_files.contains_key(tmp_file) {
            return false;
        }

        let target_path = self.tmp_files.get(tmp_file).unwrap().clone();
        
        let file = match fs::read(tmp_file) {
            Ok(data) => data,
            Err(e) => {
                Util::write_log("files_external", &format!("Error reading tmp file: {}", e), Util::ERROR);
                return false;
            }
        };
        
        let content_type = Helper::get_mime_type(tmp_file);
        let content_length = file.len() as i64;

        let result = self.runtime.block_on(async {
            self.connection.put_object()
                .bucket(&self.bucket)
                .key(&target_path)
                .body(ByteStream::from(file))
                .content_type(content_type)
                .content_length(content_length)
                .send()
                .await
        });
        
        if let Err(e) = result {
            Util::write_log("files_external", &format!("Error writing back file: {}", e), Util::ERROR);
            return false;
        }
        
        self.test_timeout();
        
        if let Err(e) = fs::remove_file(tmp_file) {
            Util::write_log("files_external", &format!("Error removing tmp file: {}", e), Util::ERROR);
        }
        
        true
    }
}

impl CommonStorage for AmazonS3 {
    fn mkdir(&self, path: &str) -> bool {
        let path = self.normalize_path(path);
        
        if self.is_dir(&path) {
            return false;
        }
        
        let result = self.runtime.block_on(async {
            self.connection.put_object()
                .bucket(&self.bucket)
                .key(&format!("{}/", path))
                .body(ByteStream::from_static(&[]))
                .content_type("httpd/unix-directory")
                .content_length(0)
                .send()
                .await
        });
        
        if let Err(e) = result {
            Util::write_log("files_external", &format!("Error creating directory: {}", e), Util::ERROR);
            return false;
        }
        
        self.test_timeout();
        true
    }

    fn file_exists(&self, path: &str) -> bool {
        let mut path = self.normalize_path(path);
        
        if path.is_empty() {
            path = ".".to_string();
        } else if path != "." && self.is_dir(&path) {
            path = format!("{}/", path);
        }
        
        let result = self.runtime.block_on(async {
            self.connection.head_object()
                .bucket(&self.bucket)
                .key(&path)
                .send()
                .await
        });
        
        match result {
            Ok(_) => true,
            Err(e) => {
                // Only log if it's not a "not found" error
                if !matches!(e, SdkError::ServiceError(ref err) if err.err().is_no_such_key()) {
                    Util::write_log("files_external", &format!("Error checking if file exists: {}", e), Util::ERROR);
                }
                false
            }
        }
    }

    fn rmdir(&self, path: &str) -> bool {
        let path = self.normalize_path(path);
        
        if !self.file_exists(&path) {
            return false;
        }
        
        // Remove all contents recursively
        match self.opendir(&path) {
            Some(dir_handle) => {
                while let Some(entry) = dir_handle.read_dir().next() {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        if file_name == "." || file_name == ".." {
                            continue;
                        }
                        
                        let full_path = format!("{}/{}", path, file_name);
                        if self.is_dir(&full_path) {
                            self.rmdir(&full_path);
                        } else {
                            self.unlink(&full_path);
                        }
                    }
                }
            },
            None => return false
        }
        
        let result = self.runtime.block_on(async {
            self.connection.delete_object()
                .bucket(&self.bucket)
                .key(&format!("{}/", path))
                .send()
                .await
        });
        
        if let Err(e) = result {
            Util::write_log("files_external", &format!("Error removing directory: {}", e), Util::ERROR);
            return false;
        }
        
        self.test_timeout();
        true
    }

    fn opendir(&self, path: &str) -> Option<Dir> {
        let mut path = self.normalize_path(path);
        
        if path == "." {
            path = "".to_string();
        } else if !path.is_empty() {
            path = format!("{}/", path);
        }
        
        let result = self.runtime.block_on(async {
            self.connection.list_objects_v2()
                .bucket(&self.bucket)
                .delimiter("/")
                .prefix(&path)
                .send()
                .await
        });
        
        match result {
            Ok(resp) => {
                let mut files = Vec::new();
                
                // Add regular objects
                if let Some(contents) = resp.contents() {
                    for obj in contents {
                        if let Some(key) = obj.key() {
                            if let Some(file_name) = Path::new(key).file_name() {
                                let file_name = file_name.to_string_lossy().to_string();
                                if !file_name.is_empty() && file_name != Path::new(&path).file_name().unwrap_or_default().to_string_lossy().to_string() {
                                    files.push(file_name);
                                }
                            }
                        }
                    }
                }
                
                // Add prefixes (directories)
                if let Some(prefixes) = resp.common_prefixes() {
                    for prefix in prefixes {
                        if let Some(prefix_path) = prefix.prefix() {
                            if let Some(file_name) = Path::new(prefix_path.trim_end_matches('/')).file_name() {
                                let file_name = file_name.to_string_lossy().to_string();
                                if !file_name.is_empty() {
                                    files.push(file_name);
                                }
                            }
                        }
                    }
                }
                
                Dir::register(&format!("amazons3{}", path), files)
            },
            Err(e) => {
                Util::write_log("files_external", &format!("Error opening directory: {}", e), Util::ERROR);
                None
            }
        }
    }

    fn stat(&self, path: &str) -> Option<HashMap<String, i64>> {
        let mut path = self.normalize_path(path);
        
        if self.is_dir(&path) && path != "." {
            path = format!("{}/", path);
        }
        
        let result = self.runtime.block_on(async {
            self.connection.head_object()
                .bucket(&self.bucket)
                .key(&path)
                .send()
                .await
        });
        
        match result {
            Ok(resp) => {
                let mut stat = HashMap::new();
                
                stat.insert("size".to_string(), resp.content_length().unwrap_or(0));
                
                // Try to get last modified from metadata, fall back to object last modified
                let mtime = if let Some(metadata) = resp.metadata() {
                    if let Some(last_modified) = metadata.get("lastmodified") {
                        if let Ok(time) = chrono::DateTime::parse_from_rfc3339(last_modified) {
                            time.timestamp()
                        } else {
                            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
                        }
                    } else if let Some(last_modified) = resp.last_modified() {
                        last_modified.secs() as i64
                    } else {
                        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
                    }
                } else if let Some(last_modified) = resp.last_modified() {
                    last_modified.secs() as i64
                } else {
                    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
                };
                
                stat.insert("mtime".to_string(), mtime);
                stat.insert("atime".to_string(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
                
                Some(stat)
            },
            Err(e) => {
                Util::write_log("files_external", &format!("Error getting file stats: {}", e), Util::ERROR);
                None
            }
        }
    }

    fn filetype(&self, path: &str) -> Option<String> {
        let path = self.normalize_path(path);
        
        let result = self.runtime.block_on(async {
            // Check if it's a file
            if path != "." {
                let resp = self.connection.head_object()
                    .bucket(&self.bucket)
                    .key(&path)
                    .send()
                    .await;
                
                if resp.is_ok() {
                    return "file".to_string();
                }
            }
            
            // Check if it's a directory
            let dir_path = if path != "." { format!("{}/", path) } else { ".".to_string() };
            
            let resp = self.connection.head_object()
                .bucket(&self.bucket)
                .key(&dir_path)
                .send()
                .await;
                
            if resp.is_ok() {
                return "dir".to_string();
            }
            
            "".to_string()
        });
        
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    fn is_readable(&self, _path: &str) -> bool {
        true
    }

    fn is_updatable(&self, _path: &str) -> bool {
        true
    }

    fn unlink(&self, path: &str) -> bool {
        let path = self.normalize_path(path);
        
        let result = self.runtime.block_on(async {
            self.connection.delete_object()
                .bucket(&self.bucket)
                .key(&path)
                .send()
                .await
        });
        
        if let Err(e) = result {
            Util::write_log("files_external", &format!("Error unlinking file: {}", e), Util::ERROR);
            return false;
        }
        
        self.test_timeout();
        true
    }

    fn fopen(&mut self, path: &str, mode: &str) -> Option<File> {
        let path = self.normalize_path(path);
        
        match mode {
            "r" | "rb" => {
                let tmp_file = Helper::tmp_file("");
                self.tmp_files.insert(tmp_file.clone(), path.clone());
                
                let result = self.runtime.block_on(async {
                    self.connection.get_object()
                        .bucket(&self.bucket)
                        .key(&path)
                        .send()
                        .await
                });
                
                match result {
                    Ok(resp) => {
                        let body = resp.body.collect().await;
                        if let Ok(body) = body {
                            let data = body.into_bytes();
                            if let Ok(mut file) = File::create(&tmp_file) {
                                if file.write_all(&data).is_ok() {
                                    if let Ok(file) = File::open(&tmp_file) {
                                        return Some(file);
                                    }
                                }
                            }
                        }
                        None
                    },
                    Err(e) => {
                        Util::write_log("files_external", &format!("Error opening file for reading: {}", e), Util::ERROR);
                        None
                    }
                }
            },
            "w" | "wb" | "a" | "ab" | "r+" | "w+" | "wb+" | "a+" | "x" | "x+" | "c" | "c+" => {
                // Determine file extension for tmp file
                let ext = if let Some(dot_pos) = path.rfind('.') {
                    path[dot_pos..].to_string()
                } else {
                    "".to_string()
                };
                
                let tmp_file = Helper::tmp_file(&ext);
                
                // Register callback to write back on close
                Close::register_callback(&tmp_file, Box::new(move |tmp| {
                    self.write_back(tmp)
                }));
                
                // If file exists and we're reading, copy content to tmp file
                if self.file_exists(&path) && (mode.contains('r') || mode.contains('a')) {
                    if let Some(mut source) = self.fopen(&path, "r") {
                        if let Ok(mut dest) = File::create(&tmp_file) {
                            let mut buffer = Vec::new();
                            if source.read_to_end(&mut buffer).is_ok() {
                                if dest.write_all(&buffer).is_err() {
                                    return None;
                                }
                            }
                        }
                    }
                }
                
                self.tmp_files.insert(tmp_file.clone(), path);
                
                // Open with the close wrapper
                match File::open(&format!("close://{}", tmp_file)) {
                    Ok(file) => Some(file),
                    Err(_) => None
                }
            },
            _ => None
        }
    }

    fn get_mime_type(&self, path: &str) -> Option<String> {
        let path = self.normalize_path(path);
        
        if self.is_dir(&path) {
            return Some("httpd/unix-directory".to_string());
        } else if self.file_exists(&path) {
            let result = self.runtime.block_on(async {
                self.connection.head_object()
                    .bucket(&self.bucket)
                    .key(&path)
                    .send()
                    .await
            });
            
            match result {
                Ok(resp) => resp.content_type().map(|s| s.to_string()),
                Err(e) => {
                    Util::write_log("files_external", &format!("Error getting MIME type: {}", e), Util::ERROR);
                    None
                }
            }
        } else {
            None
        }
    }

    fn touch(&self, path: &str, mtime: Option<i64>) -> bool {
        let path = self.normalize_path(path);
        
        let metadata = if let Some(time) = mtime {
            let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(time, 0)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default();
            HashMap::from([("lastmodified".to_string(), datetime)])
        } else {
            HashMap::new()
        };
        
        let result = self.runtime.block_on(async {
            if self.file_exists(&path) {
                let full_path = if self.is_dir(&path) && path != "." {
                    format!("{}/", path)
                } else {
                    path.clone()
                };
                
                // Copy object to itself with the new metadata
                self.connection.copy_object()
                    .bucket(&self.bucket)
                    .key(&full_path)
                    .copy_source(format!("{}/{}", self.bucket, full_path))
                    .metadata(metadata)
                    .metadata_directive("REPLACE")
                    .send()
                    .await
            } else {
                // Create a new empty object with the specified metadata
                self.connection.put_object()
                    .bucket(&self.bucket)
                    .key(&path)
                    .body(ByteStream::from_static(&[]))
                    .metadata(metadata)
                    .send()
                    .await
            }
        });
        
        if let Err(e) = result {
            Util::write_log("files_external", &format!("Error touching file: {}", e), Util::ERROR);
            return false;
        }
        
        self.test_timeout();
        true
    }

    fn copy(&self, path1: &str, path2: &str) -> bool {
        let path1 = self.normalize_path(path1);
        let path2 = self.normalize_path(path2);
        
        if self.is_file(&path1) {
            let result = self.runtime.block_on(async {
                self.connection.copy_object()
                    .bucket(&self.bucket)
                    .key(&path2)
                    .copy_source(format!("{}/{}", self.bucket, path1))
                    .send()
                    .await
            });
            
            if let Err(e) = result {
                Util::write_log("files_external", &format!("Error copying file: {}", e), Util::ERROR);
                return false;
            }
            
            self.test_timeout();
        } else {
            // Directory copy
            if self.file_exists(&path2) {
                return false;
            }
            
            // Create target directory
            let result = self.runtime.block_on(async {
                self.connection.copy_object()
                    .bucket(&self.bucket)
                    .key(&format!("{}/", path2))
                    .copy_source(format!("{}/{}/", self.bucket, path1))
                    .send()
                    .await
            });
            
            if let Err(e) = result {
                Util::write_log("files_external", &format!("Error creating target directory: {}", e), Util::ERROR);
                return false;
            }
            
            self.test_timeout();
            
            // Copy all contents recursively
            if let Some(dir_handle) = self.opendir(&path1) {
                while let Some(entry) = dir_handle.read_dir().next() {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        if file_name == "." || file_name == ".." {
                            continue;
                        }
                        
                        let source = format!("{}/{}", path1, file_name);
                        let target = format!("{}/{}", path2, file_name);
                        
                        if !self.copy(&source, &target) {
                            return false;
                        }
                    }
                }
            }
        }
        
        true
    }

    fn rename(&self, path1: &str, path2: &str) -> bool {
        let path1 = self.normalize_path(path1);
        let path2 = self.normalize_path(path2);
        
        if self.is_file(&path1) {
            if !self.copy(&path1, &path2) {
                return false;
            }
            
            if !self.unlink(&path1) {
                self.unlink(&path2);
                return false;
            }
        } else {
            if self.file_exists(&path2) {
                return false;
            }
            
            if !self.copy(&path1, &path2) {
                return false;
            }
            
            if !self.rmdir(&path1) {
                self.rmdir(&path2);
                return false;
            }
        }
        
        true
    }

    fn test(&self) -> bool {
        let result = self.runtime.block_on(async {
            self.connection.list_buckets().send().await
        });
        
        result.is_ok()
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}