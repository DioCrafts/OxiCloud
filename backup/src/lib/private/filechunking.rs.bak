use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;
use std::sync::Arc;
use regex::Regex;
use md5::{Md5, Digest};
use lazy_static::lazy_static;
use rand::Rng;
use thiserror::Error;

// Placeholder for required dependencies that would be defined elsewhere
use crate::cache::file::FileCache;
use crate::files::filesystem::{Filesystem, Hook, FileProxy};

#[derive(Error, Debug)]
pub enum FileChunkingError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    
    #[error("Filesystem error: {0}")]
    FilesystemError(String),
}

type Result<T> = std::result::Result<T, FileChunkingError>;

pub struct FileChunking {
    info: HashMap<String, serde_json::Value>,
    cache: Option<Arc<FileCache>>,
}

impl FileChunking {
    pub fn decode_name(name: &str) -> Option<HashMap<String, String>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<name>.*)-chunking-(?P<transferid>\d+)-(?P<chunkcount>\d+)-(?P<index>\d+)"
            ).unwrap();
        }

        RE.captures(name).map(|caps| {
            let mut matches = HashMap::new();
            matches.insert("name".to_string(), caps.name("name").map_or("", |m| m.as_str()).to_string());
            matches.insert("transferid".to_string(), caps.name("transferid").map_or("", |m| m.as_str()).to_string());
            matches.insert("chunkcount".to_string(), caps.name("chunkcount").map_or("", |m| m.as_str()).to_string());
            matches.insert("index".to_string(), caps.name("index").map_or("", |m| m.as_str()).to_string());
            matches
        })
    }

    pub fn new(info: HashMap<String, serde_json::Value>) -> Self {
        Self {
            info,
            cache: None,
        }
    }

    pub fn get_prefix(&self) -> String {
        let name = self.info.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let transferid = self.info.get("transferid")
            .and_then(|v| v.as_str().or_else(|| v.as_u64().map(|n| n.to_string().as_str())))
            .unwrap_or("");

        format!("{}-chunking-{}-", name, transferid)
    }

    fn get_cache(&mut self) -> Arc<FileCache> {
        if self.cache.is_none() {
            self.cache = Some(Arc::new(FileCache::new()));
        }
        Arc::clone(self.cache.as_ref().unwrap())
    }

    /// Stores the given data under the given index - the number of stored bytes is returned
    ///
    /// # Arguments
    ///
    /// * `index` - The chunk index
    /// * `data` - The data to store
    ///
    /// # Returns
    ///
    /// The size of the stored data
    pub fn store(&mut self, index: usize, data: &[u8]) -> Result<usize> {
        let cache = self.get_cache();
        let name = format!("{}{}", self.get_prefix(), index);
        
        cache.set(&name, data)
            .map_err(|e| FileChunkingError::CacheError(e.to_string()))?;

        cache.size(&name)
            .map_err(|e| FileChunkingError::CacheError(e.to_string()))
    }

    pub fn is_complete(&mut self) -> Result<bool> {
        let prefix = self.get_prefix();
        let mut parts = 0;
        let cache = self.get_cache();
        
        let chunk_count = self.info.get("chunkcount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;

        for i in 0..chunk_count {
            if cache.has_key(&format!("{}{}", prefix, i))
                .map_err(|e| FileChunkingError::CacheError(e.to_string()))? 
            {
                parts += 1;
            }
        }

        Ok(parts == chunk_count)
    }

    pub fn assemble<W: Write>(&mut self, f: &mut W) -> Result<usize> {
        let cache = self.get_cache();
        let prefix = self.get_prefix();
        let mut count = 0;

        let chunk_count = self.info.get("chunkcount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;

        for i in 0..chunk_count {
            let chunk = cache.get(&format!("{}{}", prefix, i))
                .map_err(|e| FileChunkingError::CacheError(e.to_string()))?;
            
            count += f.write(&chunk)
                .map_err(FileChunkingError::IoError)?;
        }

        self.cleanup()?;
        Ok(count)
    }

    /// Removes all chunks which belong to this transmission
    pub fn cleanup(&mut self) -> Result<()> {
        let cache = self.get_cache();
        let prefix = self.get_prefix();

        let chunk_count = self.info.get("chunkcount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;

        for i in 0..chunk_count {
            cache.remove(&format!("{}{}", prefix, i))
                .map_err(|e| FileChunkingError::CacheError(e.to_string()))?;
        }

        Ok(())
    }

    /// Removes one specific chunk
    pub fn remove(&mut self, index: usize) -> Result<()> {
        let cache = self.get_cache();
        let prefix = self.get_prefix();
        
        cache.remove(&format!("{}{}", prefix, index))
            .map_err(|e| FileChunkingError::CacheError(e.to_string()))?;
            
        Ok(())
    }

    pub fn signature_split<R: Read, S: Read>(
        &mut self, 
        org_file: &mut R, 
        input: &mut S
    ) -> Result<HashMap<String, serde_json::Value>> {
        let mut block_size_buf = [0u8; 2];
        input.read_exact(&mut block_size_buf)
            .map_err(FileChunkingError::IoError)?;
        
        let block_size = ((block_size_buf[0] as u16) << 8) | (block_size_buf[1] as u16);
        
        // Generate random transfer ID
        let transfer_id = rand::thread_rng().gen::<u64>();
        self.info.insert("transferid".to_string(), serde_json::json!(transfer_id));
        
        let mut count = 0;
        let mut needed = Vec::new();
        let cache = self.get_cache();
        let prefix = self.get_prefix();
        
        let mut new_md5 = [0u8; 16];
        let mut data = vec![0u8; block_size as usize];
        
        loop {
            match input.read_exact(&mut new_md5) {
                Ok(_) => {},
                Err(_) if count > 0 => break,
                Err(e) => return Err(FileChunkingError::IoError(e)),
            }
            
            let bytes_read = org_file.read(&mut data)
                .map_err(FileChunkingError::IoError)?;
                
            if bytes_read == 0 {
                break;
            }
            
            let actual_data = &data[..bytes_read];
            let mut hasher = Md5::new();
            hasher.update(actual_data);
            let org_md5 = hasher.finalize();
            
            if org_md5.as_slice() == new_md5 {
                cache.set(&format!("{}{}", prefix, count), actual_data)
                    .map_err(|e| FileChunkingError::CacheError(e.to_string()))?;
            } else {
                needed.push(count);
            }
            
            count += 1;
        }
        
        Ok(HashMap::from([
            ("transferid".to_string(), serde_json::json!(transfer_id)),
            ("needed".to_string(), serde_json::json!(needed)),
            ("count".to_string(), serde_json::json!(count)),
        ]))
    }

    pub fn file_assemble(&mut self, path: &str) -> Result<bool> {
        let absolute_path = Filesystem::normalize_path(&Filesystem::get_view().get_absolute_path(path));
        let data = "";

        // Check if file_put_contents operation is allowed by proxies
        if !FileProxy::run_pre_proxies("file_put_contents", &absolute_path, data) {
            return Ok(false);
        }

        if !Filesystem::is_valid_path(path) {
            return Err(FileChunkingError::InvalidPath(path.to_string()));
        }

        let rel_path = Filesystem::get_view().get_relative_path(&absolute_path);
        let exists = Filesystem::file_exists(&rel_path);
        let mut run = true;

        if !exists {
            Hook::emit(
                Filesystem::CLASSNAME,
                Filesystem::signal_create(),
                &[
                    (Filesystem::signal_param_path(), &rel_path),
                    (Filesystem::signal_param_run(), &mut run),
                ],
            );
        }

        Hook::emit(
            Filesystem::CLASSNAME,
            Filesystem::signal_write(),
            &[
                (Filesystem::signal_param_path(), &rel_path),
                (Filesystem::signal_param_run(), &mut run),
            ],
        );

        if !run {
            return Ok(false);
        }

        let mut target = Filesystem::fopen(&rel_path, "w")
            .map_err(|e| FileChunkingError::FilesystemError(e.to_string()))?;
        
        if let Some(target_file) = target {
            let count = self.assemble(&mut target_file)?;
            
            if !exists {
                Hook::emit(
                    Filesystem::CLASSNAME,
                    Filesystem::signal_post_create(),
                    &[(Filesystem::signal_param_path(), &rel_path)],
                );
            }
            
            Hook::emit(
                Filesystem::CLASSNAME,
                Filesystem::signal_post_write(),
                &[(Filesystem::signal_param_path(), &rel_path)],
            );
            
            FileProxy::run_post_proxies("file_put_contents", &absolute_path, count);
            
            Ok(count > 0)
        } else {
            Ok(false)
        }
    }
}