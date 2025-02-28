use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

/// Detection
///
/// Mimetype detection
pub struct Detection {
    mimetypes: HashMap<String, String>,
}

impl Detection {
    /// Create a new Detection instance
    pub fn new() -> Self {
        Self {
            mimetypes: HashMap::new(),
        }
    }

    /// Add an extension -> mimetype mapping
    ///
    /// # Arguments
    ///
    /// * `extension` - The file extension
    /// * `mimetype` - The corresponding mimetype
    pub fn register_type(&mut self, extension: &str, mimetype: &str) {
        self.mimetypes.insert(extension.to_string(), mimetype.to_string());
    }

    /// Add an array of extension -> mimetype mappings
    ///
    /// # Arguments
    ///
    /// * `types` - A HashMap of extension -> mimetype mappings
    pub fn register_type_array(&mut self, types: HashMap<String, String>) {
        self.mimetypes.extend(types);
    }

    /// Detect mimetype only based on filename, content of file is not used
    ///
    /// # Arguments
    ///
    /// * `path` - The path to detect the mimetype for
    ///
    /// # Returns
    ///
    /// The detected mimetype as a String
    pub fn detect_path<P: AsRef<Path>>(&self, path: P) -> String {
        let path = path.as_ref();
        
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                let ext = ext_str.to_lowercase();
                if let Some(mimetype) = self.mimetypes.get(&ext) {
                    return mimetype.clone();
                }
            }
        }
        
        "application/octet-stream".to_string()
    }

    /// Detect mimetype based on both filename and content
    ///
    /// # Arguments
    ///
    /// * `path` - The path to detect the mimetype for
    ///
    /// # Returns
    ///
    /// The detected mimetype as a String
    pub fn detect<P: AsRef<Path>>(&self, path: P) -> String {
        let path = path.as_ref();
        
        // Directories are easy
        if path.is_dir() {
            return "httpd/unix-directory".to_string();
        }

        let mut mime_type = self.detect_path(path);

        if mime_type == "application/octet-stream" {
            // Try to use libmagic if available
            if let Ok(magic_mime) = get_mime_from_magic(path) {
                if !magic_mime.is_empty() {
                    return magic_mime;
                }
            }
            
            // Check if it's a wrapped path
            let path_str = path.to_string_lossy();
            let is_wrapped = path_str.contains("://") && path_str.starts_with("file://");
            
            // Try the 'file' command if not wrapped
            if !is_wrapped && can_execute("file") {
                if let Ok(cmd_mime) = get_mime_from_command(path) {
                    if !cmd_mime.is_empty() {
                        return cmd_mime;
                    }
                }
            }
        }
        
        mime_type
    }

    /// Detect mimetype based on the content of a string
    ///
    /// # Arguments
    ///
    /// * `data` - The data to detect the mimetype for
    ///
    /// # Returns
    ///
    /// The detected mimetype as a String
    pub fn detect_string(&self, data: &[u8]) -> String {
        if let Ok(mime) = get_mime_from_buffer(data) {
            return mime;
        }
        
        // Fallback to temporary file method
        let tmp_file = create_temp_file().unwrap_or_else(|_| PathBuf::from("/tmp/rust_mimetype_detection"));
        
        if let Ok(mut file) = fs::File::create(&tmp_file) {
            if file.write_all(&data[..std::cmp::min(data.len(), 8024)]).is_ok() {
                let mime = self.detect(&tmp_file);
                let _ = fs::remove_file(&tmp_file);
                return mime;
            }
        }
        
        "application/octet-stream".to_string()
    }
}

/// Try to get the mime type using libmagic
fn get_mime_from_magic<P: AsRef<Path>>(path: P) -> Result<String, &'static str> {
    #[cfg(feature = "magic")]
    {
        use magic::{Cookie, CookieFlags};
        
        let cookie = Cookie::open(CookieFlags::MIME_TYPE).map_err(|_| "Failed to initialize magic cookie")?;
        cookie.load::<&str>(&[]).map_err(|_| "Failed to load magic database")?;
        
        let result = cookie.file(path.as_ref().to_str().unwrap_or(""))
            .map_err(|_| "Failed to get file magic")?;
            
        // Split at first semicolon if present
        let mime_type = match result.find(';') {
            Some(pos) => result[0..pos].to_lowercase(),
            None => result.to_lowercase(),
        };
        
        if mime_type.is_empty() {
            Ok("application/octet-stream".to_string())
        } else {
            Ok(mime_type)
        }
    }
    
    #[cfg(not(feature = "magic"))]
    {
        Err("Magic library not available")
    }
}

/// Try to get the mime type from a buffer using libmagic
fn get_mime_from_buffer(buffer: &[u8]) -> Result<String, &'static str> {
    #[cfg(feature = "magic")]
    {
        use magic::{Cookie, CookieFlags};
        
        let cookie = Cookie::open(CookieFlags::MIME_TYPE).map_err(|_| "Failed to initialize magic cookie")?;
        cookie.load::<&str>(&[]).map_err(|_| "Failed to load magic database")?;
        
        let result = cookie.buffer(buffer)
            .map_err(|_| "Failed to get buffer magic")?;
            
        Ok(result)
    }
    
    #[cfg(not(feature = "magic"))]
    {
        Err("Magic library not available")
    }
}

/// Check if a command can be executed
fn can_execute(cmd: &str) -> bool {
    if cfg!(target_os = "windows") {
        Command::new("where")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    } else {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

/// Try to get the mime type using the file command
fn get_mime_from_command<P: AsRef<Path>>(path: P) -> Result<String, &'static str> {
    let path_str = path.as_ref().to_string_lossy();
    
    let output = Command::new("file")
        .args(&["-b", "--mime-type", &path_str])
        .output()
        .map_err(|_| "Failed to execute file command")?;
    
    if output.status.success() {
        let mime = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
            
        if mime.is_empty() {
            Ok("application/octet-stream".to_string())
        } else {
            Ok(mime)
        }
    } else {
        Ok("application/octet-stream".to_string())
    }
}

/// Create a temporary file
fn create_temp_file() -> Result<PathBuf, std::io::Error> {
    use std::env::temp_dir;
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    
    let mut rng = thread_rng();
    let random_name: String = (0..10).map(|_| rng.sample(Alphanumeric) as char).collect();
    let file_path = temp_dir().join(format!("rust_mime_{}", random_name));
    
    Ok(file_path)
}