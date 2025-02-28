use std::collections::VecDeque;
use std::ffi::OsString;
use std::fs::{self, DirBuilder, DirEntry, File, ReadDir};
use std::io::{self, Read, Write};
use std::os::windows::prelude::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use mime_guess::MimeGuess;
use walkdir::WalkDir;

mod mapper;
use mapper::Mapper;

mod stream;
use stream::dir::Dir;

mod common;
use common::Common;

mod logger;
use logger::Log;

mod helper;
use helper::Helper;

/// For local filestore, we only have to map the paths
pub struct MappedLocal {
    datadir: PathBuf,
    mapper: Mapper,
}

impl MappedLocal {
    pub fn new(arguments: &std::collections::HashMap<String, String>) -> Result<Self, io::Error> {
        let mut datadir = PathBuf::from(&arguments["datadir"]);
        
        // Ensure the datadir ends with a path separator
        if !datadir.as_os_str().to_string_lossy().ends_with('/') 
            && !datadir.as_os_str().to_string_lossy().ends_with('\\') {
            datadir.push("");
        }
        
        let mapper = Mapper::new(&datadir);
        
        Ok(Self { datadir, mapper })
    }
    
    fn build_path(&self, path: &str, create: bool) -> PathBuf {
        let path = self.strip_leading(path);
        let full_path = self.datadir.join(path);
        self.mapper.logic_to_physical(&full_path, create)
    }
    
    fn clean_mapper(&self, path: &Path, is_logic_path: bool, recursive: bool) {
        let full_path = if is_logic_path {
            self.datadir.join(path)
        } else {
            path.to_path_buf()
        };
        self.mapper.remove_path(&full_path, is_logic_path, recursive);
    }
    
    fn copy_mapping(&self, path1: &str, path2: &str) {
        let path1 = self.strip_leading(path1);
        let path2 = self.strip_leading(path2);
        
        let full_path1 = self.datadir.join(path1);
        let full_path2 = self.datadir.join(path2);
        
        self.mapper.copy(&full_path1, &full_path2);
    }
    
    fn strip_leading(&self, path: &str) -> String {
        let mut path_str = path.to_string();
        if path_str.starts_with('/') || path_str.starts_with('\\') {
            path_str = path_str[1..].to_string();
        }
        path_str
    }
    
    fn del_tree(&self, dir: &str, is_logic_path: bool) -> Result<bool, io::Error> {
        let dir_relative = dir;
        let dir_path = if is_logic_path {
            self.build_path(dir, false)
        } else {
            PathBuf::from(dir)
        };
        
        if !dir_path.exists() {
            return Ok(true);
        }
        
        if !dir_path.is_dir() || fs::symlink_metadata(&dir_path)?.file_type().is_symlink() {
            match fs::remove_file(&dir_path) {
                Ok(_) => {
                    self.clean_mapper(&dir_path, false, false);
                    return Ok(true);
                },
                Err(e) => return Err(e),
            }
        }
        
        for entry in fs::read_dir(&dir_path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name();
            
            if file_name == "." || file_name == ".." {
                continue;
            }
            
            if path.is_file() {
                if let Err(e) = fs::remove_file(&path) {
                    return Err(e);
                }
                self.clean_mapper(&path, false, false);
            } else if path.is_dir() {
                if !self.del_tree(path.to_str().unwrap_or(""), false)? {
                    return Ok(false);
                }
            }
        }
        
        match fs::remove_dir(&dir_path) {
            Ok(_) => {
                self.clean_mapper(&dir_path, false, false);
                Ok(true)
            },
            Err(e) => Err(e),
        }
    }
    
    fn search_in_dir(&self, query: &str, dir: &str) -> Vec<String> {
        let mut files = Vec::new();
        let physical_dir = self.build_path(dir, false);
        
        if !physical_dir.exists() || !physical_dir.is_dir() {
            return files;
        }
        
        if let Ok(entries) = fs::read_dir(&physical_dir) {
            for entry_result in entries {
                if let Ok(entry) = entry_result {
                    let physical_item = self.mapper.physical_to_logic(&entry.path());
                    
                    if let Some(physical_dir_str) = physical_dir.to_str() {
                        if let Some(physical_item_str) = physical_item.to_str() {
                            let item = &physical_item_str[(physical_dir_str.len() + 1)..];
                            
                            if item.to_lowercase().contains(&query.to_lowercase()) {
                                files.push(format!("{}/{}", dir, item));
                            }
                            
                            if entry.path().is_dir() {
                                let sub_dir = format!("{}/{}", dir, item);
                                let sub_files = self.search_in_dir(query, &sub_dir);
                                files.extend(sub_files);
                            }
                        }
                    }
                }
            }
        }
        
        files
    }
    
    fn get_file_size_from_os(full_path: &Path) -> u64 {
        let name = std::env::consts::OS.to_lowercase();
        
        if name.contains("windows") {
            // Windows COM implementation would go here, but we'll use a fallback method
            // since direct COM access is complex in Rust
            if let Ok(metadata) = full_path.metadata() {
                return metadata.file_size();
            }
        } else if name.contains("bsd") {
            if let Ok(output) = Command::new("stat")
                .args(&["-f", "%z", full_path.to_str().unwrap_or("")])
                .output() {
                if let Ok(size_str) = String::from_utf8(output.stdout) {
                    if let Ok(size) = size_str.trim().parse::<u64>() {
                        return size;
                    }
                }
            }
        } else if name.contains("linux") {
            if let Ok(output) = Command::new("stat")
                .args(&["-c", "%s", full_path.to_str().unwrap_or("")])
                .output() {
                if let Ok(size_str) = String::from_utf8(output.stdout) {
                    if let Ok(size) = size_str.trim().parse::<u64>() {
                        return size;
                    }
                }
            }
        } else {
            Log::error(&format!(
                "Unable to determine file size of \"{}\". Unknown OS: {}",
                full_path.display(),
                name
            ));
        }
        
        0
    }
}

impl Drop for MappedLocal {
    fn drop(&mut self) {
        if cfg!(test) {
            self.mapper.remove_path(&self.datadir, true, true);
        }
    }
}

#[async_trait]
impl Common for MappedLocal {
    fn get_id(&self) -> String {
        format!("local::{}", self.datadir.display())
    }
    
    async fn mkdir(&self, path: &str) -> Result<bool, io::Error> {
        match DirBuilder::new().recursive(false).create(self.build_path(path, true)) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    async fn rmdir(&self, path: &str) -> Result<bool, io::Error> {
        let path_buf = self.build_path(path, false);
        
        if !path_buf.exists() {
            return Ok(false);
        }
        
        let walkdir = match WalkDir::new(&path_buf).contents_first(true).into_iter() {
            dir => dir,
        };
        
        for entry_result in walkdir {
            match entry_result {
                Ok(entry) => {
                    let file_path = entry.path();
                    let file_name = file_path.file_name().unwrap_or_default();
                    
                    if file_name == "." || file_name == ".." {
                        continue;
                    } else if entry.file_type().is_dir() {
                        if let Err(_) = fs::remove_dir(file_path) {
                            return Ok(false);
                        }
                    } else {
                        if let Err(_) = fs::remove_file(file_path) {
                            return Ok(false);
                        }
                    }
                },
                Err(_) => return Ok(false),
            }
        }
        
        match fs::remove_dir(self.build_path(path, false)) {
            Ok(_) => {
                self.clean_mapper(Path::new(path), true, false);
                Ok(true)
            },
            Err(_) => Ok(false),
        }
    }
    
    async fn opendir(&self, path: &str) -> Result<Box<dyn Read + Send>, io::Error> {
        let mut files = Vec::new();
        files.push(".".to_string());
        files.push("..".to_string());
        
        let physical_path = self.build_path(path, false);
        let logical_path = self.mapper.physical_to_logic(&physical_path);
        
        if let Ok(dir_entries) = fs::read_dir(&physical_path) {
            for entry_result in dir_entries {
                if let Ok(entry) = entry_result {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    
                    if file_name_str == "." || file_name_str == ".." {
                        continue;
                    }
                    
                    let logical_file_path = self.mapper.physical_to_logic(&physical_path.join(&file_name));
                    
                    if let (Some(logical_file_path_str), Some(logical_path_str)) = 
                        (logical_file_path.to_str(), logical_path.to_str()) {
                        let file = self.mapper.strip_root_folder(logical_file_path_str, logical_path_str);
                        let file = self.strip_leading(&file);
                        files.push(file);
                    }
                }
            }
        }
        
        Dir::register(&format!("local-win32{}", path), files);
        
        Dir::open(&format!("fakedir://local-win32{}", path))
    }
    
    async fn is_dir(&self, path: &str) -> Result<bool, io::Error> {
        let mut path_str = path.to_string();
        if path_str.ends_with('/') {
            path_str = path_str[0..path_str.len()-1].to_string();
        }
        
        Ok(self.build_path(&path_str, false).is_dir())
    }
    
    async fn is_file(&self, path: &str) -> Result<bool, io::Error> {
        Ok(self.build_path(path, false).is_file())
    }
    
    async fn stat(&self, path: &str) -> Result<fs::Metadata, io::Error> {
        let full_path = self.build_path(path, false);
        let metadata = fs::metadata(&full_path)?;
        
        // On Windows, we might need to handle negative size values differently
        // This is a simplified approach
        Ok(metadata)
    }
    
    async fn filetype(&self, path: &str) -> Result<String, io::Error> {
        let full_path = self.build_path(path, false);
        let file_type = fs::symlink_metadata(&full_path)?.file_type();
        
        let result = if file_type.is_dir() {
            "dir".to_string()
        } else if file_type.is_file() {
            "file".to_string()
        } else if file_type.is_symlink() {
            match fs::metadata(&full_path) {
                Ok(real_meta) => {
                    if real_meta.is_dir() {
                        "dir".to_string()
                    } else {
                        "file".to_string()
                    }
                },
                Err(_) => "unknown".to_string(),
            }
        } else {
            "unknown".to_string()
        };
        
        Ok(result)
    }
    
    async fn filesize(&self, path: &str) -> Result<u64, io::Error> {
        if self.is_dir(path).await? {
            Ok(0)
        } else {
            let full_path = self.build_path(path, false);
            let size = match fs::metadata(&full_path) {
                Ok(metadata) => metadata.len(),
                Err(_) => Self::get_file_size_from_os(&full_path),
            };
            
            Ok(size)
        }
    }
    
    async fn is_readable(&self, path: &str) -> Result<bool, io::Error> {
        // In Rust we don't have a direct equivalent to PHP's is_readable
        // We'll try to open the file for reading as a test
        let path_buf = self.build_path(path, false);
        
        if path_buf.is_dir() {
            if let Ok(_) = fs::read_dir(&path_buf) {
                return Ok(true);
            }
        } else {
            if let Ok(_) = File::open(&path_buf) {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    async fn is_updatable(&self, path: &str) -> Result<bool, io::Error> {
        // Similar to is_readable, we'll test write permissions
        let path_buf = self.build_path(path, false);
        
        if path_buf.exists() {
            if path_buf.is_dir() {
                // Test by trying to create a temporary file inside the directory
                let test_file = path_buf.join(".write_test_tmp");
                match File::create(&test_file) {
                    Ok(_) => {
                        let _ = fs::remove_file(&test_file);
                        return Ok(true);
                    },
                    Err(_) => return Ok(false),
                }
            } else {
                // Test by opening the file in append mode
                match OpenOptions::new().write(true).open(&path_buf) {
                    Ok(_) => return Ok(true),
                    Err(_) => return Ok(false),
                }
            }
        } else {
            // Test if we can create a file at this location
            if let Some(parent) = path_buf.parent() {
                if parent.exists() {
                    match File::create(&path_buf) {
                        Ok(_) => {
                            let _ = fs::remove_file(&path_buf);
                            return Ok(true);
                        },
                        Err(_) => return Ok(false),
                    }
                }
            }
        }
        
        Ok(false)
    }
    
    async fn file_exists(&self, path: &str) -> Result<bool, io::Error> {
        Ok(self.build_path(path, false).exists())
    }
    
    async fn filemtime(&self, path: &str) -> Result<u64, io::Error> {
        let metadata = fs::metadata(self.build_path(path, false))?;
        
        if let Ok(time) = metadata.modified() {
            if let Ok(duration) = time.duration_since(UNIX_EPOCH) {
                return Ok(duration.as_secs());
            }
        }
        
        Err(io::Error::new(io::ErrorKind::Other, "Could not determine mtime"))
    }
    
    async fn touch(&self, path: &str, mtime: Option<u64>) -> Result<bool, io::Error> {
        let file_path = self.build_path(path, true);
        
        // Make sure the file exists by creating it if it doesn't
        if !file_path.exists() {
            let _ = File::create(&file_path)?;
        }
        
        // Set the file's modification time if specified
        if let Some(mtime) = mtime {
            let system_time = UNIX_EPOCH + std::time::Duration::from_secs(mtime);
            let filetime = filetime::FileTime::from_system_time(system_time);
            filetime::set_file_mtime(&file_path, filetime)?;
        }
        
        Ok(true)
    }
    
    async fn file_get_contents(&self, path: &str) -> Result<Vec<u8>, io::Error> {
        let mut file = File::open(self.build_path(path, false))?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        Ok(contents)
    }
    
    async fn file_put_contents(&self, path: &str, data: &[u8]) -> Result<usize, io::Error> {
        let mut file = File::create(self.build_path(path, true))?;
        file.write_all(data)?;
        Ok(data.len())
    }
    
    async fn unlink(&self, path: &str) -> Result<bool, io::Error> {
        self.del_tree(path, true)
    }
    
    async fn rename(&self, path1: &str, path2: &str) -> Result<bool, io::Error> {
        if !self.is_updatable(path1).await? {
            Log::error(&format!("unable to rename, file is not writable : {}", path1));
            return Ok(false);
        }
        
        if !self.file_exists(path1).await? {
            Log::error(&format!("unable to rename, file does not exists : {}", path1));
            return Ok(false);
        }
        
        let physic_path1 = self.build_path(path1, false);
        let physic_path2 = self.build_path(path2, true);
        
        match fs::rename(&physic_path1, &physic_path2) {
            Ok(_) => {
                // mapper needs to create copies or all children
                self.copy_mapping(path1, path2);
                self.clean_mapper(&physic_path1, false, true);
                Ok(true)
            },
            Err(_) => Ok(false),
        }
    }
    
    async fn copy(&self, path1: &str, path2: &str) -> Result<bool, io::Error> {
        let mut dest_path = path2.to_string();
        
        if self.is_dir(path2).await? {
            if !self.file_exists(path2).await? {
                self.mkdir(path2).await?;
            }
            
            if let Some(source_name) = Path::new(path1).file_name() {
                dest_path = format!("{}/{}", path2, source_name.to_string_lossy());
            }
        }
        
        let source_path = self.build_path(path1, false);
        let target_path = self.build_path(&dest_path, true);
        
        if source_path.is_dir() {
            // Recursively copy directory
            for entry in WalkDir::new(&source_path) {
                let entry = entry?;
                let rel_path = entry.path().strip_prefix(&source_path).unwrap();
                let dest = target_path.join(rel_path);
                
                if entry.file_type().is_dir() {
                    fs::create_dir_all(&dest)?;
                } else {
                    if let Some(parent) = dest.parent() {
                        if !parent.exists() {
                            fs::create_dir_all(parent)?;
                        }
                    }
                    fs::copy(entry.path(), &dest)?;
                }
            }
            
            // mapper needs to create copies or all children
            self.copy_mapping(path1, &dest_path);
            Ok(true)
        } else {
            // Copy a single file
            if let Some(parent) = target_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            
            match fs::copy(&source_path, &target_path) {
                Ok(_) => {
                    // mapper needs to create copies or all children
                    self.copy_mapping(path1, &dest_path);
                    Ok(true)
                },
                Err(_) => Ok(false),
            }
        }
    }
    
    async fn fopen(&self, path: &str, mode: &str) -> Result<Box<dyn ReadWrite + Send>, io::Error> {
        use std::fs::OpenOptions;
        
        let path_buf = self.build_path(path, true);
        
        let mut options = OpenOptions::new();
        match mode {
            "r" => {
                options.read(true);
            },
            "r+" => {
                options.read(true).write(true);
            },
            "w" => {
                options.write(true).create(true).truncate(true);
            },
            "w+" => {
                options.read(true).write(true).create(true).truncate(true);
            },
            "a" => {
                options.write(true).create(true).append(true);
            },
            "a+" => {
                options.read(true).write(true).create(true).append(true);
            },
            "x" => {
                options.write(true).create_new(true);
            },
            "x+" => {
                options.read(true).write(true).create_new(true);
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file mode")),
        }
        
        let file = options.open(path_buf)?;
        Ok(Box::new(FileReadWrite::new(file)))
    }
    
    async fn get_mime_type(&self, path: &str) -> Result<String, io::Error> {
        if self.is_readable(path).await? {
            let path_buf = self.build_path(path, false);
            let mime = MimeGuess::from_path(&path_buf).first_or_octet_stream();
            Ok(mime.to_string())
        } else {
            Ok("application/octet-stream".to_string())
        }
    }
    
    async fn hash(&self, path: &str, type_: &str, raw: bool) -> Result<String, io::Error> {
        use sha1::{Sha1, Digest};
        use sha2::{Sha256, Sha512};
        use md5::Md5;
        
        let path_buf = self.build_path(path, false);
        let mut file = File::open(path_buf)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        match type_.to_lowercase().as_str() {
            "md5" => {
                let mut hasher = Md5::new();
                hasher.update(&buffer);
                let result = hasher.finalize();
                if raw {
                    Ok(String::from_utf8_lossy(&result).to_string())
                } else {
                    Ok(format!("{:x}", result))
                }
            },
            "sha1" => {
                let mut hasher = Sha1::new();
                hasher.update(&buffer);
                let result = hasher.finalize();
                if raw {
                    Ok(String::from_utf8_lossy(&result).to_string())
                } else {
                    Ok(format!("{:x}", result))
                }
            },
            "sha256" => {
                let mut hasher = Sha256::new();
                hasher.update(&buffer);
                let result = hasher.finalize();
                if raw {
                    Ok(String::from_utf8_lossy(&result).to_string())
                } else {
                    Ok(format!("{:x}", result))
                }
            },
            "sha512" => {
                let mut hasher = Sha512::new();
                hasher.update(&buffer);
                let result = hasher.finalize();
                if raw {
                    Ok(String::from_utf8_lossy(&result).to_string())
                } else {
                    Ok(format!("{:x}", result))
                }
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported hash type")),
        }
    }
    
    async fn free_space(&self, path: &str) -> Result<u64, io::Error> {
        if let Ok(stats) = fs2::available_space(self.build_path(path, false)) {
            Ok(stats)
        } else {
            Ok(0)
        }
    }
    
    async fn search(&self, query: &str) -> Result<Vec<String>, io::Error> {
        Ok(self.search_in_dir(query, ""))
    }
    
    async fn get_local_file(&self, path: &str) -> Result<PathBuf, io::Error> {
        Ok(self.build_path(path, false))
    }
    
    async fn get_local_folder(&self, path: &str) -> Result<PathBuf, io::Error> {
        Ok(self.build_path(path, false))
    }
    
    async fn has_updated(&self, path: &str, time: u64) -> Result<bool, io::Error> {
        let mtime = self.filemtime(path).await?;
        Ok(mtime > time)
    }
}

// This is a custom struct that combines Read and Write traits
use std::io::{Read, Write, Seek};
use std::fs::{File, OpenOptions};

pub trait ReadWrite: Read + Write + Seek {}

struct FileReadWrite {
    file: File,
}

impl FileReadWrite {
    fn new(file: File) -> Self {
        Self { file }
    }
}

impl Read for FileReadWrite {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read(buf)
    }
}

impl Write for FileReadWrite {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write(buf)
    }
    
    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

impl Seek for FileReadWrite {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.file.seek(pos)
    }
}

impl ReadWrite for FileReadWrite {}