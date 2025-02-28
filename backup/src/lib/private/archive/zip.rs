use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use zip::{ZipArchive, ZipWriter};
use zip::write::FileOptions;
use zip::result::ZipResult;
use log::warn;
use tempfile::NamedTempFile;

/// Archive implementation for ZIP files
pub struct ArchiveZip {
    zip: Option<ZipArchive<File>>,
    path: PathBuf,
    temp_files: HashMap<String, String>,
}

impl ArchiveZip {
    pub fn new<P: AsRef<Path>>(source: P) -> io::Result<Self> {
        let path = source.as_ref().to_path_buf();
        let file = File::open(&path).or_else(|_| File::create(&path))?;
        
        let zip = match ZipArchive::new(file) {
            Ok(archive) => Some(archive),
            Err(err) => {
                warn!("Error while opening archive {}: {}", path.display(), err);
                None
            }
        };

        Ok(Self {
            zip,
            path,
            temp_files: HashMap::new(),
        })
    }

    /// Add an empty folder to the archive
    ///
    /// # Arguments
    /// * `path` - Path within the archive
    pub fn add_folder<P: AsRef<Path>>(&mut self, path: P) -> io::Result<bool> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        
        // Reopen the archive in write mode
        let file = File::create(&self.path)?;
        let mut zip_writer = ZipWriter::new(file);
        
        // Add the directory
        match zip_writer.add_directory(&path_str, FileOptions::default()) {
            Ok(_) => {
                drop(zip_writer);
                
                // Reopen in read mode
                let file = File::open(&self.path)?;
                self.zip = Some(ZipArchive::new(file)?);
                Ok(true)
            },
            Err(_) => Ok(false),
        }
    }

    /// Add a file to the archive
    ///
    /// # Arguments
    /// * `path` - Path within the archive
    /// * `source` - Either a local file path or string data
    pub fn add_file<P, S>(&mut self, path: P, source: S) -> io::Result<bool> 
    where 
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let source_str = source.as_ref();
        
        // Create a new ZipWriter
        let file = File::create(&self.path)?;
        let mut zip_writer = ZipWriter::new(file);
        
        let result = if !source_str.is_empty() && source_str.starts_with('/') && Path::new(source_str).exists() {
            // Add from file
            let mut source_file = File::open(source_str)?;
            let mut buffer = Vec::new();
            source_file.read_to_end(&mut buffer)?;
            
            zip_writer.start_file(&path_str, FileOptions::default())?;
            zip_writer.write_all(&buffer)?;
            true
        } else {
            // Add from string
            zip_writer.start_file(&path_str, FileOptions::default())?;
            zip_writer.write_all(source_str.as_bytes())?;
            true
        };
        
        // Close and reopen to save the zip
        drop(zip_writer);
        
        // Reopen in read mode
        let file = File::open(&self.path)?;
        self.zip = Some(ZipArchive::new(file)?);
        
        Ok(result)
    }

    /// Rename a file or folder in the archive
    ///
    /// # Arguments
    /// * `source` - Original path
    /// * `dest` - New path
    pub fn rename<P, Q>(&mut self, source: P, dest: Q) -> io::Result<()> 
    where 
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let source = self.strip_path(source);
        let dest = self.strip_path(dest);
        
        // This is a complex operation that would require reading all files,
        // creating a new zip, and copying everything except renamed files
        // Not directly supported by the zip crate, would need implementation
        
        // Simplified placeholder for now
        Ok(())
    }

    /// Get the uncompressed size of a file in the archive
    ///
    /// # Arguments
    /// * `path` - Path to the file
    pub fn filesize<P: AsRef<Path>>(&self, path: P) -> io::Result<u64> {
        if let Some(ref zip) = self.zip {
            let path_str = path.as_ref().to_string_lossy();
            match zip.by_name(&path_str) {
                Ok(file) => Ok(file.size()),
                Err(_) => Err(io::Error::new(io::ErrorKind::NotFound, "File not found in archive")),
            }
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Archive not opened"))
        }
    }

    /// Get the last modified time of a file in the archive
    ///
    /// # Arguments
    /// * `path` - Path to the file
    pub fn mtime<P: AsRef<Path>>(&self, _path: P) -> io::Result<SystemTime> {
        // Return the mtime of the archive itself
        fs::metadata(&self.path)
            .map(|metadata| metadata.modified()
            .unwrap_or_else(|_| SystemTime::now()))
    }

    /// Get the files in a folder
    ///
    /// # Arguments
    /// * `path` - Path to the folder
    pub fn get_folder<P: AsRef<Path>>(&self, path: P) -> io::Result<Vec<String>> {
        let files = self.get_files()?;
        let mut folder_content = Vec::new();
        
        let path_str = path.as_ref().to_string_lossy();
        let path_len = path_str.len();
        
        for file in files {
            if file.starts_with(&path_str) && file != path_str {
                let remaining = &file[path_len..];
                if !remaining.contains('/') || remaining.find('/').unwrap() == remaining.len() - 1 {
                    folder_content.push(remaining.to_string());
                }
            }
        }
        
        Ok(folder_content)
    }

    /// Get all files in the archive
    pub fn get_files(&self) -> io::Result<Vec<String>> {
        if let Some(ref zip) = self.zip {
            let mut files = Vec::new();
            
            for i in 0..zip.len() {
                let file = zip.by_index(i).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                files.push(file.name().to_string());
            }
            
            Ok(files)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Archive not opened"))
        }
    }

    /// Get the content of a file
    ///
    /// # Arguments
    /// * `path` - Path to the file
    pub fn get_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<Vec<u8>> {
        if let Some(ref mut zip) = self.zip {
            let path_str = path.as_ref().to_string_lossy();
            let mut file = zip.by_name(&path_str)
                .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
            
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            
            Ok(contents)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Archive not opened"))
        }
    }

    /// Extract a single file from the archive
    ///
    /// # Arguments
    /// * `path` - Path within the archive
    /// * `dest` - Destination path on disk
    pub fn extract_file<P, Q>(&mut self, path: P, dest: Q) -> io::Result<()> 
    where 
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        if let Some(ref mut zip) = self.zip {
            let path_str = path.as_ref().to_string_lossy();
            let mut source = zip.by_name(&path_str)
                .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
            
            let mut dest_file = File::create(dest.as_ref())?;
            
            let mut buffer = Vec::new();
            source.read_to_end(&mut buffer)?;
            dest_file.write_all(&buffer)?;
            
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Archive not opened"))
        }
    }

    /// Extract the archive
    ///
    /// # Arguments
    /// * `dest` - Destination path
    pub fn extract<P: AsRef<Path>>(&mut self, dest: P) -> io::Result<bool> {
        if let Some(ref mut zip) = self.zip {
            zip.extract(dest.as_ref())
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Check if a file or folder exists in the archive
    ///
    /// # Arguments
    /// * `path` - Path to check
    pub fn file_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Some(ref zip) = self.zip {
            let path_str = path.as_ref().to_string_lossy();
            let path_with_slash = format!("{}/", path_str);
            
            zip.by_name(&path_str).is_ok() || zip.by_name(&path_with_slash).is_ok()
        } else {
            false
        }
    }

    /// Remove a file or folder from the archive
    ///
    /// # Arguments
    /// * `path` - Path to remove
    pub fn remove<P: AsRef<Path>>(&mut self, path: P) -> io::Result<bool> {
        // Removal would require recreating the archive without the file
        // Not directly supported by the zip crate
        
        // Simplified placeholder
        Ok(false)
    }

    /// Get a file handler
    ///
    /// # Arguments
    /// * `path` - Path to the file
    /// * `mode` - Access mode (r, rb, w, wb)
    pub fn get_stream<P: AsRef<Path>>(&mut self, path: P, mode: &str) -> io::Result<Box<dyn Read>> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        
        if mode == "r" || mode == "rb" {
            if let Some(ref mut zip) = self.zip {
                let file = zip.by_name(&path_str)
                    .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
                
                return Ok(Box::new(file));
            }
        } else {
            // For write modes, create a temporary file
            let ext = match path_str.rfind('.') {
                Some(pos) => path_str[pos..].to_string(),
                None => String::new(),
            };
            
            let temp_file = NamedTempFile::new()?;
            let temp_path = temp_file.path().to_string_lossy().to_string();
            
            // If file exists, extract it to the temp file
            if self.file_exists(&path_str) {
                self.extract_file(&path_str, &temp_path)?;
            }
            
            // When temp file is closed, it will be written back to the archive
            self.temp_files.insert(temp_path.clone(), path_str);
            
            // Return a File that will be handled in write_back
            return Ok(Box::new(File::open(temp_path)?));
        }
        
        Err(io::Error::new(io::ErrorKind::Other, "Could not open stream"))
    }

    /// Write back temporary files to the archive
    ///
    /// # Arguments
    /// * `temp_file` - Path to temporary file
    pub fn write_back(&mut self, temp_file: &str) -> io::Result<()> {
        if let Some(path) = self.temp_files.remove(temp_file) {
            self.add_file(path, temp_file)?;
            fs::remove_file(temp_file)?;
        }
        
        Ok(())
    }

    fn strip_path<P: AsRef<Path>>(&self, path: P) -> String {
        let path_str = path.as_ref().to_string_lossy();
        if path_str.starts_with('/') {
            path_str[1..].to_string()
        } else {
            path_str.to_string()
        }
    }
}