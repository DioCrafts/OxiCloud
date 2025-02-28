// ownCloud
//
// A collection of useful helper functions

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::str;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};

/// Collection of useful functions
pub struct OcHelper {
    tmp_files: Arc<RwLock<Vec<PathBuf>>>,
    mimetype_icons: Arc<RwLock<HashMap<String, String>>>,
    mimetype_detector: Option<Arc<FilesTypeDetection>>,
    template_manager: Option<Arc<FilesTypeTemplateManager>>,
}

impl OcHelper {
    pub fn new() -> Self {
        OcHelper {
            tmp_files: Arc::new(RwLock::new(Vec::new())),
            mimetype_icons: Arc::new(RwLock::new(HashMap::new())),
            mimetype_detector: None,
            template_manager: None,
        }
    }

    /// Creates an url using a defined route
    ///
    /// # Arguments
    ///
    /// * `route` - route name
    /// * `parameters` - parameters to be included in the URL
    ///
    /// # Returns
    ///
    /// The generated URL
    pub fn link_to_route(&self, route: &str, parameters: HashMap<&str, &str>) -> String {
        // Delegate to URL generator server
        OcServer::get_url_generator().link_to_route(route, parameters)
    }

    /// Creates an url
    ///
    /// # Arguments
    ///
    /// * `app` - app name
    /// * `file` - file name
    /// * `args` - parameters to be included in the URL
    ///
    /// # Returns
    ///
    /// The generated URL
    pub fn link_to(&self, app: &str, file: &str, args: HashMap<&str, &str>) -> String {
        // Delegate to URL generator server
        OcServer::get_url_generator().link_to(app, file, args)
    }

    /// Creates a link to online documentation
    ///
    /// # Arguments
    ///
    /// * `key` - documentation key
    ///
    /// # Returns
    ///
    /// URL to documentation
    pub fn link_to_docs(&self, key: &str) -> String {
        let theme = OcDefaults::new();
        format!("{}/server/6.0/go.php?to={}", theme.get_doc_base_url(), key)
    }

    /// Creates an absolute url
    ///
    /// # Arguments
    ///
    /// * `app` - app name
    /// * `file` - file name
    /// * `args` - parameters to be included in the URL
    ///
    /// # Returns
    ///
    /// The absolute URL
    pub fn link_to_absolute(&self, app: &str, file: &str, args: HashMap<&str, &str>) -> String {
        let url_link_to = self.link_to(app, file, args);
        self.make_url_absolute(&url_link_to)
    }

    /// Makes a URL absolute
    ///
    /// # Arguments
    ///
    /// * `url` - the relative URL
    ///
    /// # Returns
    ///
    /// The absolute URL
    pub fn make_url_absolute(&self, url: &str) -> String {
        OcServer::get_url_generator().get_absolute_url(url)
    }

    /// Creates an url for remote use
    ///
    /// # Arguments
    ///
    /// * `service` - service identifier
    ///
    /// # Returns
    ///
    /// URL to the remote service
    pub fn link_to_remote_base(&self, service: &str) -> String {
        format!("{}/{}", self.link_to("", "remote.php", HashMap::new()), service)
    }

    /// Creates an absolute url for remote use
    ///
    /// # Arguments
    ///
    /// * `service` - service identifier
    /// * `add_slash` - whether to add a trailing slash
    ///
    /// # Returns
    ///
    /// Absolute URL to the remote service
    pub fn link_to_remote(&self, service: &str, add_slash: bool) -> String {
        let mut url = self.make_url_absolute(&self.link_to_remote_base(service));
        
        if add_slash && !service.ends_with('/') {
            url.push('/');
        }
        
        url
    }

    /// Creates an absolute url for public use
    ///
    /// # Arguments
    ///
    /// * `service` - service identifier
    /// * `add_slash` - whether to add a trailing slash
    ///
    /// # Returns
    ///
    /// Absolute URL to the public service
    pub fn link_to_public(&self, service: &str, add_slash: bool) -> String {
        let mut url = format!(
            "{}?service={}",
            self.link_to_absolute("", "public.php", HashMap::new()),
            service
        );
        
        if add_slash && !service.ends_with('/') {
            url.push('/');
        }
        
        url
    }

    /// Creates path to an image
    ///
    /// # Arguments
    ///
    /// * `app` - app name
    /// * `image` - image name
    ///
    /// # Returns
    ///
    /// Path to the image
    pub fn image_path(&self, app: &str, image: &str) -> String {
        OcServer::get_url_generator().image_path(app, image)
    }

    /// Get path to icon of file type
    ///
    /// # Arguments
    ///
    /// * `mimetype` - MIME type
    ///
    /// # Returns
    ///
    /// Path to the icon for this file type
    pub fn mimetype_icon(&self, mimetype: &str) -> String {
        // Create alias mapping
        let alias = self.get_mimetype_aliases();
        
        let mimetype = if let Some(aliased_type) = alias.get(mimetype) {
            aliased_type
        } else {
            mimetype
        };
        
        // Check if already in cache
        {
            let icons = self.mimetype_icons.read().unwrap();
            if let Some(icon) = icons.get(mimetype) {
                return icon.clone();
            }
        }
        
        // Replace slash and backslash with a minus
        let icon = mimetype.replace('/', "-").replace('\\', "-");
        
        // Special handling for directories
        let web_root = OcServer::get_webroot();
        
        match mimetype {
            "dir" => {
                let path = format!("{}/core/img/filetypes/folder.png", web_root);
                let mut icons = self.mimetype_icons.write().unwrap();
                icons.insert(mimetype.to_string(), path.clone());
                return path;
            },
            "dir-shared" => {
                let path = format!("{}/core/img/filetypes/folder-shared.png", web_root);
                let mut icons = self.mimetype_icons.write().unwrap();
                icons.insert(mimetype.to_string(), path.clone());
                return path;
            },
            "dir-external" => {
                let path = format!("{}/core/img/filetypes/folder-external.png", web_root);
                let mut icons = self.mimetype_icons.write().unwrap();
                icons.insert(mimetype.to_string(), path.clone());
                return path;
            },
            _ => {}
        }
        
        // Check if icon exists
        let server_root = OcServer::get_server_root();
        let icon_path = format!("{}/core/img/filetypes/{}.png", server_root, icon);
        
        if Path::new(&icon_path).exists() {
            let path = format!("{}/core/img/filetypes/{}.png", web_root, icon);
            let mut icons = self.mimetype_icons.write().unwrap();
            icons.insert(mimetype.to_string(), path.clone());
            return path;
        }
        
        // Try only the first part of the filetype
        if let Some(pos) = icon.find('-') {
            let mime_part = &icon[..pos];
            let icon_path = format!("{}/core/img/filetypes/{}.png", server_root, mime_part);
            
            if Path::new(&icon_path).exists() {
                let path = format!("{}/core/img/filetypes/{}.png", web_root, mime_part);
                let mut icons = self.mimetype_icons.write().unwrap();
                icons.insert(mimetype.to_string(), path.clone());
                return path;
            }
        }
        
        // Default fallback
        let path = format!("{}/core/img/filetypes/file.png", web_root);
        let mut icons = self.mimetype_icons.write().unwrap();
        icons.insert(mimetype.to_string(), path.clone());
        path
    }
    
    fn get_mimetype_aliases(&self) -> HashMap<&'static str, &'static str> {
        let mut alias = HashMap::new();
        alias.insert("application/xml", "code/xml");
        alias.insert("application/msword", "x-office/document");
        alias.insert("application/vnd.openxmlformats-officedocument.wordprocessingml.document", "x-office/document");
        alias.insert("application/vnd.openxmlformats-officedocument.wordprocessingml.template", "x-office/document");
        alias.insert("application/vnd.ms-word.document.macroEnabled.12", "x-office/document");
        alias.insert("application/vnd.ms-word.template.macroEnabled.12", "x-office/document");
        alias.insert("application/vnd.oasis.opendocument.text", "x-office/document");
        alias.insert("application/vnd.oasis.opendocument.text-template", "x-office/document");
        alias.insert("application/vnd.oasis.opendocument.text-web", "x-office/document");
        alias.insert("application/vnd.oasis.opendocument.text-master", "x-office/document");
        alias.insert("application/vnd.ms-powerpoint", "x-office/presentation");
        alias.insert("application/vnd.openxmlformats-officedocument.presentationml.presentation", "x-office/presentation");
        alias.insert("application/vnd.openxmlformats-officedocument.presentationml.template", "x-office/presentation");
        alias.insert("application/vnd.openxmlformats-officedocument.presentationml.slideshow", "x-office/presentation");
        alias.insert("application/vnd.ms-powerpoint.addin.macroEnabled.12", "x-office/presentation");
        alias.insert("application/vnd.ms-powerpoint.presentation.macroEnabled.12", "x-office/presentation");
        alias.insert("application/vnd.ms-powerpoint.template.macroEnabled.12", "x-office/presentation");
        alias.insert("application/vnd.ms-powerpoint.slideshow.macroEnabled.12", "x-office/presentation");
        alias.insert("application/vnd.oasis.opendocument.presentation", "x-office/presentation");
        alias.insert("application/vnd.oasis.opendocument.presentation-template", "x-office/presentation");
        alias.insert("application/vnd.ms-excel", "x-office/spreadsheet");
        alias.insert("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", "x-office/spreadsheet");
        alias.insert("application/vnd.openxmlformats-officedocument.spreadsheetml.template", "x-office/spreadsheet");
        alias.insert("application/vnd.ms-excel.sheet.macroEnabled.12", "x-office/spreadsheet");
        alias.insert("application/vnd.ms-excel.template.macroEnabled.12", "x-office/spreadsheet");
        alias.insert("application/vnd.ms-excel.addin.macroEnabled.12", "x-office/spreadsheet");
        alias.insert("application/vnd.ms-excel.sheet.binary.macroEnabled.12", "x-office/spreadsheet");
        alias.insert("application/vnd.oasis.opendocument.spreadsheet", "x-office/spreadsheet");
        alias.insert("application/vnd.oasis.opendocument.spreadsheet-template", "x-office/spreadsheet");
        alias
    }

    /// Get path to preview icon of file
    ///
    /// # Arguments
    ///
    /// * `path` - file path
    ///
    /// # Returns
    ///
    /// Path to file preview
    pub fn preview_icon(&self, path: &str) -> String {
        let mut params = HashMap::new();
        params.insert("x", "36");
        params.insert("y", "36");
        params.insert("file", path);
        
        self.link_to_route("core_ajax_preview", params)
    }

    /// Get path to public preview icon of file
    ///
    /// # Arguments
    ///
    /// * `path` - file path
    /// * `token` - access token
    ///
    /// # Returns
    ///
    /// Path to public file preview
    pub fn public_preview_icon(&self, path: &str, token: &str) -> String {
        let mut params = HashMap::new();
        params.insert("x", "36");
        params.insert("y", "36");
        params.insert("file", path);
        params.insert("t", token);
        
        self.link_to_route("core_ajax_public_preview", params)
    }

    /// Format file size for human readability
    ///
    /// # Arguments
    ///
    /// * `bytes` - size in bytes
    ///
    /// # Returns
    ///
    /// Human-readable file size string
    pub fn human_file_size(&self, bytes: i64) -> String {
        if bytes < 0 {
            return "?".to_string();
        }
        
        if bytes < 1024 {
            return format!("{} B", bytes);
        }
        
        let mut bytes = (bytes as f64) / 1024.0;
        bytes = (bytes * 10.0).round() / 10.0;
        
        if bytes < 1024.0 {
            return format!("{} kB", bytes);
        }
        
        bytes = bytes / 1024.0;
        bytes = (bytes * 10.0).round() / 10.0;
        
        if bytes < 1024.0 {
            return format!("{} MB", bytes);
        }
        
        bytes = bytes / 1024.0;
        bytes = (bytes * 10.0).round() / 10.0;
        
        if bytes < 1024.0 {
            return format!("{} GB", bytes);
        }
        
        bytes = bytes / 1024.0;
        bytes = (bytes * 10.0).round() / 10.0;
        
        if bytes < 1024.0 {
            return format!("{} TB", bytes);
        }
        
        bytes = bytes / 1024.0;
        bytes = (bytes * 10.0).round() / 10.0;
        
        format!("{} PB", bytes)
    }

    /// Convert human-readable file size to bytes
    ///
    /// # Arguments
    ///
    /// * `str` - file size in a human-readable format
    ///
    /// # Returns
    ///
    /// Size in bytes
    pub fn computer_file_size(&self, size_str: &str) -> i64 {
        let size_str = size_str.to_lowercase();
        
        let bytes_array = HashMap::from([
            ("b", 1),
            ("k", 1024),
            ("kb", 1024),
            ("mb", 1024 * 1024),
            ("m", 1024 * 1024),
            ("gb", 1024 * 1024 * 1024),
            ("g", 1024 * 1024 * 1024),
            ("tb", 1024 * 1024 * 1024 * 1024),
            ("t", 1024 * 1024 * 1024 * 1024),
            ("pb", 1024 * 1024 * 1024 * 1024 * 1024),
            ("p", 1024 * 1024 * 1024 * 1024 * 1024),
        ]);
        
        // Extract numeric part
        let mut numeric_part = String::new();
        for c in size_str.chars() {
            if c.is_digit(10) || c == '.' {
                numeric_part.push(c);
            } else {
                break;
            }
        }
        
        let mut bytes = numeric_part.parse::<f64>().unwrap_or(0.0);
        
        // Extract unit part
        let unit_part = size_str.trim_start_matches(|c: char| c.is_digit(10) || c == '.');
        
        if let Some(multiplier) = bytes_array.get(unit_part) {
            bytes *= *multiplier as f64;
        }
        
        bytes.round() as i64
    }

    /// Recursively set file permissions
    ///
    /// # Arguments
    ///
    /// * `path` - path to file or directory
    /// * `filemode` - Unix style file permissions
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub fn chmodr<P: AsRef<Path>>(&self, path: P, filemode: u32) -> io::Result<()> {
        let path = path.as_ref();
        
        if !path.is_dir() {
            return fs::set_permissions(path, fs::Permissions::from_mode(filemode));
        }
        
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            
            if file_name_str != "." && file_name_str != ".." {
                let full_path = path.join(file_name);
                
                if fs::symlink_metadata(&full_path)?.file_type().is_symlink() {
                    return Err(io::Error::new(io::ErrorKind::Other, "Cannot chmod on symlinks"));
                } else if !full_path.is_dir() {
                    fs::set_permissions(&full_path, fs::Permissions::from_mode(filemode))?;
                } else {
                    self.chmodr(&full_path, filemode)?;
                }
            }
        }
        
        fs::set_permissions(path, fs::Permissions::from_mode(filemode))?;
        Ok(())
    }

    /// Recursively copy folders
    ///
    /// # Arguments
    ///
    /// * `src` - source directory
    /// * `dest` - destination directory
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub fn copyr<P: AsRef<Path>>(&self, src: P, dest: P) -> io::Result<()> {
        let src = src.as_ref();
        let dest = dest.as_ref();
        
        if src.is_dir() {
            if !dest.exists() {
                fs::create_dir_all(dest)?;
            }
            
            for entry in fs::read_dir(src)? {
                let entry = entry?;
                let file_name = entry.file_name();
                
                if file_name != "." && file_name != ".." {
                    self.copyr(src.join(&file_name), dest.join(&file_name))?;
                }
            }
        } else if src.exists() && !FilesFilesystem::is_file_blacklisted(src) {
            fs::copy(src, dest)?;
        }
        
        Ok(())
    }

    /// Recursively delete folders
    ///
    /// # Arguments
    ///
    /// * `dir` - directory to delete
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub fn rmdirr<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let dir = dir.as_ref();
        
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.file_name().unwrap() != "." && path.file_name().unwrap() != ".." {
                    self.rmdirr(&path)?;
                }
            }
            
            fs::remove_dir(dir)?;
        } else if dir.exists() {
            fs::remove_file(dir)?;
        }
        
        Ok(())
    }

    /// Get the mimetype detector
    ///
    /// # Returns
    ///
    /// Reference to the mimetype detector
    pub fn get_mimetype_detector(&mut self) -> Arc<FilesTypeDetection> {
        if self.mimetype_detector.is_none() {
            let mut detector = FilesTypeDetection::new();
            detector.register_type_array(include!("mimetypes.list.rs"));
            self.mimetype_detector = Some(Arc::new(detector));
        }
        
        self.mimetype_detector.as_ref().unwrap().clone()
    }

    /// Get the file template manager
    ///
    /// # Returns
    ///
    /// Reference to the file template manager
    pub fn get_file_template_manager(&mut self) -> Arc<FilesTypeTemplateManager> {
        if self.template_manager.is_none() {
            self.template_manager = Some(Arc::new(FilesTypeTemplateManager::new()));
        }
        
        self.template_manager.as_ref().unwrap().clone()
    }

    /// Try to guess the mimetype based on filename
    ///
    /// # Arguments
    ///
    /// * `path` - file path
    ///
    /// # Returns
    ///
    /// Detected MIME type
    pub fn get_file_name_mime_type(&mut self, path: &str) -> String {
        self.get_mimetype_detector().detect_path(path)
    }

    /// Get the mimetype from a local file
    ///
    /// # Arguments
    ///
    /// * `path` - file path
    ///
    /// # Returns
    ///
    /// Detected MIME type
    pub fn get_mime_type(&mut self, path: &str) -> String {
        self.get_mimetype_detector().detect(path)
    }

    /// Get the mimetype from a data string
    ///
    /// # Arguments
    ///
    /// * `data` - data to analyze
    ///
    /// # Returns
    ///
    /// Detected MIME type
    pub fn get_string_mime_type(&mut self, data: &str) -> String {
        self.get_mimetype_detector().detect_string(data)
    }

    /// Get value from request or default
    ///
    /// # Arguments
    ///
    /// * `name` - parameter name
    /// * `default` - default value
    ///
    /// # Returns
    ///
    /// Parameter value or default
    pub fn init_var(&self, name: &str, default: &str) -> String {
        match OcRequest::get_param(name) {
            Some(value) if !value.is_empty() => OcUtil::sanitize_html(&value),
            _ => default.to_string(),
        }
    }

    /// Returns "checked" attribute for radio buttons if selected
    ///
    /// # Arguments
    ///
    /// * `name` - radio button name
    /// * `value` - current radio button value
    /// * `default` - default value
    ///
    /// # Returns
    ///
    /// "checked" attribute if appropriate
    pub fn init_radio(&self, name: &str, value: &str, default: &str) -> String {
        match OcRequest::get_param(name) {
            Some(param) if param == value => "checked=\"checked\"".to_string(),
            None if value == default => "checked=\"checked\"".to_string(),
            _ => "".to_string(),
        }
    }

    /// Detect if a given program is found in the search PATH
    ///
    /// # Arguments
    ///
    /// * `name` - program name
    /// * `path` - optional custom PATH
    ///
    /// # Returns
    ///
    /// true if program is found and executable
    pub fn can_execute(&self, name: &str, path: Option<&str>) -> bool {
        let path = match path {
            Some(p) => p.to_string(),
            None => match std::env::var("PATH") {
                Ok(p) => p,
                Err(_) => return false,
            },
        };
        
        let (exts, check_fn): (Vec<&str>, fn(&Path) -> bool) = if cfg!(windows) {
            (vec![".exe", ".com"], |p| p.exists())
        } else {
            (vec![""], |p| p.exists() && is_executable(p))
        };
        
        // Check if open_basedir is enabled
        let dirs = if let Some(obd) = get_open_basedir() {
            if !obd.is_empty() {
                obd
            } else {
                path.split(if cfg!(windows) { ';' } else { ':' }).collect()
            }
        } else {
            path.split(if cfg!(windows) { ';' } else { ':' }).collect()
        };
        
        for dir in dirs {
            for ext in &exts {
                let full_path = Path::new(dir).join(format!("{}{}", name, ext));
                if check_fn(&full_path) {
                    return true;
                }
            }
        }
        
        false
    }

    /// Copy the contents of one stream to another
    ///
    /// # Arguments
    ///
    /// * `source` - source reader
    /// * `target` - target writer
    ///
    /// # Returns
    ///
    /// Tuple with bytes copied and success status
    pub fn stream_copy<R: Read, W: Write>(&self, source: &mut R, target: &mut W) -> (usize, bool) {
        let mut buffer = [0; 8192];
        let mut total_copied = 0;
        let mut result = true;
        
        loop {
            match source.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    match target.write_all(&buffer[..n]) {
                        Ok(_) => total_copied += n,
                        Err(_) => {
                            result = false;
                            break;
                        }
                    }
                },
                Err(_) => {
                    result = false;
                    break;
                }
            }
        }
        
        (total_copied, result)
    }

    /// Create a temporary file with a unique filename
    ///
    /// # Arguments
    ///
    /// * `postfix` - optional file suffix
    ///
    /// # Returns
    ///
    /// Path to temporary file
    pub fn tmp_file(&self, postfix: &str) -> io::Result<PathBuf> {
        let dir = get_temp_dir();
        let unique = format!("{:x}{}", md5::compute(format!("{}{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), rand::random::<u32>())), postfix);
        let file = dir.join(unique);
        
        File::create(&file)?;
        
        let mut tmp_files = self.tmp_files.write().unwrap();
        tmp_files.push(file.clone());
        
        Ok(file)
    }

    /// Move a file to the no-clean temporary directory
    ///
    /// # Arguments
    ///
    /// * `filename` - file to move
    ///
    /// # Returns
    ///
    /// New path or None on failure
    pub fn move_to_no_clean(&self, filename: &str) -> Option<String> {
        if filename.is_empty() {
            return None;
        }
        
        let temp_dir = get_temp_dir();
        let tmp_dir_no_clean = temp_dir.join("oc-noclean");
        
        if !tmp_dir_no_clean.exists() || !tmp_dir_no_clean.is_dir() {
            if tmp_dir_no_clean.exists() {
                if let Err(_) = fs::remove_file(&tmp_dir_no_clean) {
                    return None;
                }
            }
            
            if let Err(_) = fs::create_dir(&tmp_dir_no_clean) {
                return None;
            }
        }
        
        let filename_path = Path::new(filename);
        let new_name = tmp_dir_no_clean.join(filename_path.file_name().unwrap_or_default());
        
        match fs::rename(filename, &new_name) {
            Ok(_) => Some(new_name.to_string_lossy().into_owned()),
            Err(_) => None,
        }
    }

    /// Create a temporary folder with a unique name
    ///
    /// # Returns
    ///
    /// Path to temporary folder
    pub fn tmp_folder(&self) -> io::Result<PathBuf> {
        let dir = get_temp_dir();
        let unique = format!("{:x}", md5::compute(format!("{}{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), rand::random::<u32>())));
        let folder = dir.join(unique);
        
        fs::create_dir(&folder)?;
        
        let mut tmp_files = self.tmp_files.write().unwrap();
        tmp_files.push(folder.clone());
        
        Ok(folder)
    }

    /// Remove all temporary files created by tmp_file
    pub fn clean_tmp(&self) {
        let leftovers_file = get_temp_dir().join("oc-not-deleted");
        
        if leftovers_file.exists() {
            if let Ok(content) = fs::read_to_string(&leftovers_file) {
                for file in content.lines() {
                    let _ = self.rmdirr(file);
                }
            }
            let _ = fs::remove_file(leftovers_file.clone());
        }
        
        let tmp_files = self.tmp_files.read().unwrap();
        for

}} // Añadido por reparador automático