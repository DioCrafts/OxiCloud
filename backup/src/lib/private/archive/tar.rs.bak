use std::collections::HashMap;
use std::fs::{self, File, rename};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use tar::{Archive, Builder, EntryType, Header};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use tempfile::{tempdir, NamedTempFile, TempDir};

pub enum TarType {
    Plain = 0,
    Gzip = 1, 
    Bzip = 2,
}

#[async_trait]
pub trait OcArchive: Send + Sync {
    async fn add_folder(&mut self, path: &str) -> io::Result<bool>;
    async fn add_file(&mut self, path: &str, source: &str) -> io::Result<bool>;
    async fn rename(&mut self, source: &str, dest: &str) -> io::Result<bool>;
    async fn file_size(&self, path: &str) -> io::Result<u64>;
    async fn mtime(&self, path: &str) -> io::Result<u64>;
    async fn get_folder(&self, path: &str) -> io::Result<Vec<String>>;
    async fn get_files(&self) -> io::Result<Vec<String>>;
    async fn get_file(&self, path: &str) -> io::Result<Vec<u8>>;
    async fn extract_file(&self, path: &str, dest: &str) -> io::Result<bool>;
    async fn extract(&self, dest: &str) -> io::Result<bool>;
    async fn file_exists(&self, path: &str) -> io::Result<bool>;
    async fn remove(&mut self, path: &str) -> io::Result<bool>;
    async fn get_stream(&mut self, path: &str, mode: &str) -> io::Result<Option<TempFileHandle>>;
}

#[derive(Debug)]
pub struct TarHeader {
    pub filename: String,
    pub size: u64,
    pub mtime: u64,
}

pub struct TempFileHandle {
    file: NamedTempFile,
    path: String,
    archive: Arc<Mutex<OcArchiveTar>>,
}

impl Drop for TempFileHandle {
    fn drop(&mut self) {
        // Write back if needed
        let file_path = self.file.path().to_string_lossy().to_string();
        let archive_path = self.path.clone();
        
        if let Ok(mut archive) = self.archive.lock() {
            if let Ok(mut file) = File::open(self.file.path()) {
                let mut contents = Vec::new();
                if file.read_to_end(&mut contents).is_ok() {
                    let _ = archive.add_file(&archive_path, &String::from_utf8_lossy(&contents));
                }
            }
        }
    }
}

pub struct OcArchiveTar {
    file_list: Option<Vec<String>>,
    cached_headers: Option<Vec<TarHeader>>,
    path: String,
    temp_files: HashMap<String, String>,
}

impl OcArchiveTar {
    pub fn new(source: &str) -> io::Result<Self> {
        let path = source.to_string();
        
        // Validate that the source file exists or can be created
        if !Path::new(&path).exists() {
            // Create the parent directory if it doesn't exist
            if let Some(parent) = Path::new(&path).parent() {
                fs::create_dir_all(parent)?;
            }
            File::create(&path)?;
        }
        
        Ok(Self {
            file_list: None,
            cached_headers: None,
            path,
            temp_files: HashMap::new(),
        })
    }
    
    /// Try to detect the type of tar compression
    pub fn get_tar_type(file: &str) -> TarType {
        if let Some(dot_pos) = file.rfind('.') {
            let extension = &file[dot_pos + 1..];
            match extension {
                "gz" | "tgz" => TarType::Gzip,
                "bz" | "bz2" => TarType::Bzip,
                _ => TarType::Plain,
            }
        } else {
            TarType::Plain
        }
    }
    
    fn get_header(&mut self, file: &str) -> io::Result<Option<TarHeader>> {
        if self.cached_headers.is_none() {
            self.cached_headers = Some(self.list_content()?);
        }
        
        if let Some(headers) = &self.cached_headers {
            for header in headers {
                if file == header.filename 
                    || format!("{}/", file) == header.filename
                    || format!("/{}/", file) == header.filename
                    || format!("/{}", file) == header.filename {
                    return Ok(Some(header.clone()));
                }
            }
        }
        
        Ok(None)
    }
    
    fn list_content(&self) -> io::Result<Vec<TarHeader>> {
        let file = File::open(&self.path)?;
        let mut result = Vec::new();
        
        match Self::get_tar_type(&self.path) {
            TarType::Plain => {
                let mut archive = Archive::new(file);
                for entry in archive.entries()? {
                    let entry = entry?;
                    let header = entry.header();
                    result.push(TarHeader {
                        filename: entry.path()?.to_string_lossy().to_string(),
                        size: header.size()?,
                        mtime: header.mtime()?,
                    });
                }
            },
            TarType::Gzip => {
                let decoder = GzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                for entry in archive.entries()? {
                    let entry = entry?;
                    let header = entry.header();
                    result.push(TarHeader {
                        filename: entry.path()?.to_string_lossy().to_string(),
                        size: header.size()?,
                        mtime: header.mtime()?,
                    });
                }
            },
            TarType::Bzip => {
                let decoder = BzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                for entry in archive.entries()? {
                    let entry = entry?;
                    let header = entry.header();
                    result.push(TarHeader {
                        filename: entry.path()?.to_string_lossy().to_string(),
                        size: header.size()?,
                        mtime: header.mtime()?,
                    });
                }
            }
        }
        
        Ok(result)
    }
    
    fn reopen(&mut self) -> io::Result<()> {
        // Nothing to do here in Rust implementation
        // The tar crate doesn't need explicit reopening
        Ok(())
    }
}

#[async_trait]
impl OcArchive for OcArchiveTar {
    async fn add_folder(&mut self, path: &str) -> io::Result<bool> {
        let mut path_with_slash = path.to_string();
        if !path_with_slash.ends_with('/') {
            path_with_slash.push('/');
        }
        
        if let Ok(true) = self.file_exists(&path_with_slash).await {
            return Ok(false);
        }
        
        let tmp_dir = tempdir()?;
        let tmp_path = tmp_dir.path().join(&path_with_slash[1..]);
        
        fs::create_dir_all(&tmp_path)?;
        
        // Create a new tar archive with the added folder
        let mut file = File::create(&self.path)?;
        let mut builder = Builder::new(file);
        
        // Add the directory entry
        let mut header = Header::new_gnu();
        header.set_entry_type(EntryType::Directory);
        header.set_path(&path_with_slash)?;
        header.set_mode(0o755);
        header.set_size(0);
        header.set_mtime(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs());
        
        builder.append(&header, &mut io::empty())?;
        builder.finish()?;
        
        self.file_list = None;
        self.cached_headers = None;
        
        Ok(true)
    }
    
    async fn add_file(&mut self, path: &str, source: &str) -> io::Result<bool> {
        if let Ok(true) = self.file_exists(path).await {
            self.remove(path).await?;
        }
        
        let tmp_dir = tempdir()?;
        
        // Create a new tar archive with the added file
        let mut file = File::open(&self.path)?;
        let mut archive = Archive::new(&file);
        let extract_result = archive.unpack(tmp_dir.path());
        
        // If the source is a file path
        if !source.is_empty() && source.starts_with('/') && Path::new(source).exists() {
            let content = fs::read(source)?;
            let dest_path = tmp_dir.path().join(path.trim_start_matches('/'));
            
            // Create parent directories if needed
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            fs::write(&dest_path, content)?;
        } else {
            // Source is content
            let dest_path = tmp_dir.path().join(path.trim_start_matches('/'));
            
            // Create parent directories if needed
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            fs::write(&dest_path, source)?;
        }
        
        // Recreate the archive with the new file
        let file = File::create(&self.path)?;
        
        match Self::get_tar_type(&self.path) {
            TarType::Plain => {
                let mut builder = Builder::new(file);
                
                for entry in walkdir::WalkDir::new(tmp_dir.path()) {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path == tmp_dir.path() {
                        continue;
                    }
                    
                    let relative_path = path.strip_prefix(tmp_dir.path())
                        .unwrap_or_else(|_| path);
                    
                    if path.is_dir() {
                        builder.append_dir(relative_path, path)?;
                    } else {
                        let mut file = File::open(path)?;
                        builder.append_file(relative_path, &mut file)?;
                    }
                }
                
                builder.finish()?;
            },
            TarType::Gzip => {
                let encoder = GzEncoder::new(file, Compression::default());
                let mut builder = Builder::new(encoder);
                
                for entry in walkdir::WalkDir::new(tmp_dir.path()) {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path == tmp_dir.path() {
                        continue;
                    }
                    
                    let relative_path = path.strip_prefix(tmp_dir.path())
                        .unwrap_or_else(|_| path);
                    
                    if path.is_dir() {
                        builder.append_dir(relative_path, path)?;
                    } else {
                        let mut file = File::open(path)?;
                        builder.append_file(relative_path, &mut file)?;
                    }
                }
                
                builder.finish()?;
            },
            TarType::Bzip => {
                let encoder = BzEncoder::new(file, bzip2::Compression::default());
                let mut builder = Builder::new(encoder);
                
                for entry in walkdir::WalkDir::new(tmp_dir.path()) {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path == tmp_dir.path() {
                        continue;
                    }
                    
                    let relative_path = path.strip_prefix(tmp_dir.path())
                        .unwrap_or_else(|_| path);
                    
                    if path.is_dir() {
                        builder.append_dir(relative_path, path)?;
                    } else {
                        let mut file = File::open(path)?;
                        builder.append_file(relative_path, &mut file)?;
                    }
                }
                
                builder.finish()?;
            }
        }
        
        self.file_list = None;
        self.cached_headers = None;
        
        Ok(true)
    }
    
    async fn rename(&mut self, source: &str, dest: &str) -> io::Result<bool> {
        let tmp_dir = tempdir()?;
        
        // Extract archive
        let file = File::open(&self.path)?;
        let mut archive = Archive::new(file);
        archive.unpack(tmp_dir.path())?;
        
        // Rename file or directory
        let source_path = tmp_dir.path().join(source.trim_start_matches('/'));
        let dest_path = tmp_dir.path().join(dest.trim_start_matches('/'));
        
        // Create parent directories if needed
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        rename(source_path, dest_path)?;
        
        // Recreate archive
        fs::remove_file(&self.path)?;
        
        let file = File::create(&self.path)?;
        
        match Self::get_tar_type(&self.path) {
            TarType::Plain => {
                let mut builder = Builder::new(file);
                
                for entry in walkdir::WalkDir::new(tmp_dir.path()) {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path == tmp_dir.path() {
                        continue;
                    }
                    
                    let relative_path = path.strip_prefix(tmp_dir.path())
                        .unwrap_or_else(|_| path);
                    
                    if path.is_dir() {
                        builder.append_dir(relative_path, path)?;
                    } else {
                        let mut file = File::open(path)?;
                        builder.append_file(relative_path, &mut file)?;
                    }
                }
                
                builder.finish()?;
            },
            TarType::Gzip => {
                let encoder = GzEncoder::new(file, Compression::default());
                let mut builder = Builder::new(encoder);
                
                for entry in walkdir::WalkDir::new(tmp_dir.path()) {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path == tmp_dir.path() {
                        continue;
                    }
                    
                    let relative_path = path.strip_prefix(tmp_dir.path())
                        .unwrap_or_else(|_| path);
                    
                    if path.is_dir() {
                        builder.append_dir(relative_path, path)?;
                    } else {
                        let mut file = File::open(path)?;
                        builder.append_file(relative_path, &mut file)?;
                    }
                }
                
                builder.finish()?;
            },
            TarType::Bzip => {
                let encoder = BzEncoder::new(file, bzip2::Compression::default());
                let mut builder = Builder::new(encoder);
                
                for entry in walkdir::WalkDir::new(tmp_dir.path()) {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path == tmp_dir.path() {
                        continue;
                    }
                    
                    let relative_path = path.strip_prefix(tmp_dir.path())
                        .unwrap_or_else(|_| path);
                    
                    if path.is_dir() {
                        builder.append_dir(relative_path, path)?;
                    } else {
                        let mut file = File::open(path)?;
                        builder.append_file(relative_path, &mut file)?;
                    }
                }
                
                builder.finish()?;
            }
        }
        
        self.file_list = None;
        self.cached_headers = None;
        
        Ok(true)
    }
    
    async fn file_size(&self, path: &str) -> io::Result<u64> {
        let mut archive = self.clone();
        let header = archive.get_header(path)?;
        
        match header {
            Some(h) => Ok(h.size),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
        }
    }
    
    async fn mtime(&self, path: &str) -> io::Result<u64> {
        let mut archive = self.clone();
        let header = archive.get_header(path)?;
        
        match header {
            Some(h) => Ok(h.mtime),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
        }
    }
    
    async fn get_folder(&self, path: &str) -> io::Result<Vec<String>> {
        let files = self.get_files().await?;
        let mut folder_content = Vec::new();
        let path_length = path.len();
        
        for file in files {
            let file_clean = file.strip_prefix('/').unwrap_or(&file);
            
            if file_clean.starts_with(path) && file_clean != path {
                let result = &file_clean[path_length..];
                
                if let Some(pos) = result.find('/') {
                    let item = &result[..pos+1];
                    if !folder_content.contains(&item.to_string()) {
                        folder_content.push(item.to_string());
                    }
                } else if !folder_content.contains(&result.to_string()) {
                    folder_content.push(result.to_string());
                }
            }
        }
        
        Ok(folder_content)
    }
    
    async fn get_files(&self) -> io::Result<Vec<String>> {
        if let Some(files) = &self.file_list {
            return Ok(files.clone());
        }
        
        let mut archived_files = Vec::new();
        
        let file = File::open(&self.path)?;
        
        match Self::get_tar_type(&self.path) {
            TarType::Plain => {
                let mut archive = Archive::new(file);
                for entry in archive.entries()? {
                    let entry = entry?;
                    archived_files.push(entry.path()?.to_string_lossy().to_string());
                }
            },
            TarType::Gzip => {
                let decoder = GzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                for entry in archive.entries()? {
                    let entry = entry?;
                    archived_files.push(entry.path()?.to_string_lossy().to_string());
                }
            },
            TarType::Bzip => {
                let decoder = BzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                for entry in archive.entries()? {
                    let entry = entry?;
                    archived_files.push(entry.path()?.to_string_lossy().to_string());
                }
            }
        }
        
        Ok(archived_files)
    }
    
    async fn get_file(&self, path: &str) -> io::Result<Vec<u8>> {
        let file = File::open(&self.path)?;
        
        match Self::get_tar_type(&self.path) {
            TarType::Plain => {
                let mut archive = Archive::new(file);
                for mut entry in archive.entries()? {
                    if entry.path()?.to_string_lossy() == path || 
                       entry.path()?.to_string_lossy() == format!("/{}", path) {
                        let mut content = Vec::new();
                        entry.read_to_end(&mut content)?;
                        return Ok(content);
                    }
                }
            },
            TarType::Gzip => {
                let decoder = GzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                for mut entry in archive.entries()? {
                    if entry.path()?.to_string_lossy() == path || 
                       entry.path()?.to_string_lossy() == format!("/{}", path) {
                        let mut content = Vec::new();
                        entry.read_to_end(&mut content)?;
                        return Ok(content);
                    }
                }
            },
            TarType::Bzip => {
                let decoder = BzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                for mut entry in archive.entries()? {
                    if entry.path()?.to_string_lossy() == path || 
                       entry.path()?.to_string_lossy() == format!("/{}", path) {
                        let mut content = Vec::new();
                        entry.read_to_end(&mut content)?;
                        return Ok(content);
                    }
                }
            }
        }
        
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found in archive"))
    }
    
    async fn extract_file(&self, path: &str, dest: &str) -> io::Result<bool> {
        if let Ok(false) = self.file_exists(path).await {
            return Ok(false);
        }
        
        let tmp_dir = tempdir()?;
        let file = File::open(&self.path)?;
        
        match Self::get_tar_type(&self.path) {
            TarType::Plain => {
                let mut archive = Archive::new(file);
                archive.unpack(&tmp_dir)?;
            },
            TarType::Gzip => {
                let decoder = GzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                archive.unpack(&tmp_dir)?;
            },
            TarType::Bzip => {
                let decoder = BzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                archive.unpack(&tmp_dir)?;
            }
        }
        
        let extracted_path = tmp_dir.path().join(path.trim_start_matches('/'));
        
        if extracted_path.exists() {
            // Create parent directories for destination if needed
            if let Some(parent) = Path::new(dest).parent() {
                fs::create_dir_all(parent)?;
            }
            
            fs::copy(extracted_path, dest)?;
            return Ok(true);
        }
        
        let alternative_path = tmp_dir.path().join(format!("/{}", path.trim_start_matches('/')));
        
        if alternative_path.exists() {
            // Create parent directories for destination if needed
            if let Some(parent) = Path::new(dest).parent() {
                fs::create_dir_all(parent)?;
            }
            
            fs::copy(alternative_path, dest)?;
            return Ok(true);
        }
        
        Ok(false)
    }
    
    async fn extract(&self, dest: &str) -> io::Result<bool> {
        let dest_path = Path::new(dest);
        
        if !dest_path.exists() {
            fs::create_dir_all(dest_path)?;
        }
        
        let file = File::open(&self.path)?;
        
        match Self::get_tar_type(&self.path) {
            TarType::Plain => {
                let mut archive = Archive::new(file);
                archive.unpack(dest_path)?;
            },
            TarType::Gzip => {
                let decoder = GzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                archive.unpack(dest_path)?;
            },
            TarType::Bzip => {
                let decoder = BzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                archive.unpack(dest_path)?;
            }
        }
        
        Ok(true)
    }
    
    async fn file_exists(&self, path: &str) -> io::Result<bool> {
        let files = self.get_files().await?;
        
        if files.contains(&path.to_string()) || files.contains(&format!("{}/", path)) {
            return Ok(true);
        } else {
            let mut folder_path = path.to_string();
            if !folder_path.ends_with('/') {
                folder_path.push('/');
            }
            
            let path_length = folder_path.len();
            for file in files {
                if file.len() > path_length && file.starts_with(&folder_path) {
                    return Ok(true);
                }
            }
        }
        
        // Try with a leading slash if not present
        if !path.starts_with('/') {
            return self.file_exists(&format!("/{}", path)).await;
        }
        
        Ok(false)
    }
    
    async fn remove(&mut self, path: &str) -> io::Result<bool> {
        if let Ok(false) = self.file_exists(path).await {
            return Ok(false);
        }
        
        let tmp_dir = tempdir()?;
        
        // Extract archive
        let file = File::open(&self.path)?;
        
        match Self::get_tar_type(&self.path) {
            TarType::Plain => {
                let mut archive = Archive::new(file);
                archive.unpack(&tmp_dir)?;
            },
            TarType::Gzip => {
                let decoder = GzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                archive.unpack(&tmp_dir)?;
            },
            TarType::Bzip => {
                let decoder = BzDecoder::new(file);
                let mut archive = Archive::new(decoder);
                archive.unpack(&tmp_dir)?;
            }
        }
        
        // Remove file or directory
        let remove_path = tmp_dir.path().join(path.trim_start_matches('/'));
        if remove_path.exists() {
            if remove_path.is_dir() {
                fs::remove_dir_all(&remove_path)?;
            } else {
                fs::remove_file(&remove_path)?;
            }
        }
        
        let alternative_path = tmp_dir.path().join(format!("/{}", path.trim_start_matches('/')));
        if alternative_path.exists() {
            if alternative_path.is_dir() {
                fs::remove_dir_all(&alternative_path)?;
            } else {
                fs::remove_file(&alternative_path)?;
            }
        }
        
        // Recreate archive
        fs::remove_file(&self.path)?;
        
        let file = File::create(&self.path)?;
        
        match Self::get_tar_type(&self.path) {
            TarType::Plain => {
                let mut builder = Builder::new(file);
                
                for entry in walkdir::WalkDir::new(tmp_dir.path()) {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path == tmp_dir.path() {
                        continue;
                    }
                    
                    let relative_path = path.strip_prefix(tmp_dir.path())
                        .unwrap_or_else(|_| path);
                    
                    if path.is_dir() {
                        builder.append_dir(relative_path, path)?;
                    } else {
                        let mut file = File::open(path)?;
                        builder.append_file(relative_path, &mut file)?;
                    }
                }
                
                builder.finish()?;
            },
            TarType::Gzip => {
                let encoder = GzEncoder::new(file, Compression::default());
                let mut builder = Builder::new(encoder);
                
                for entry in walkdir::WalkDir::new(tmp_dir.path()) {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path == tmp_dir.path() {
                        continue;
                    }
                    
                    let relative_path = path.strip_prefix(tmp_dir.path())
                        .unwrap_or_else(|_| path);
                    
                    if path.is_dir() {
                        builder.append_dir(relative_path, path)?;
                    } else {
                        let mut file = File::open(path)?;
                        builder.append_file(relative_path, &mut file)?;
                    }
                }
                
                builder.finish()?;
            },
            TarType::Bzip => {
                let encoder = BzEncoder::new(file, bzip2::Compression::default());
                let mut builder = Builder::new(encoder);
                
                for entry in walkdir::WalkDir::new(tmp_dir.path()) {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path == tmp_dir.path() {
                        continue;
                    }
                    
                    let relative_path = path.strip_prefix(tmp_dir.path())
                        .unwrap_or_else(|_| path);
                    
                    if path.is_dir() {
                        builder.append_dir(relative_path, path)?;
                    } else {
                        let mut file = File::open(path)?;
                        builder.append_file(relative_path, &mut file)?;
                    }
                }
                
                builder.finish()?;
            }
        }
        
        self.file_list = None;
        self.cached_headers = None;
        
        Ok(true)
    }
    
    async fn get_stream(&mut self, path: &str, mode: &str) -> io::Result<Option<TempFileHandle>> {
        let mut ext = String::new();
        
        if let Some(dot_pos) = path.rfind('.') {
            ext = path[dot_pos..].to_string();
        }
        
        let temp_file = NamedTempFile::with_prefix("oc_archive_")?;
        let temp_path = temp_file.path().to_

}} // Añadido por reparador automático