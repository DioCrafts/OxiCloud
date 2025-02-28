use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/**
 * Class to provide access to ownCloud filesystem via a "view", and methods for
 * working with files within that view (e.g. read, write, delete, etc.). Each
 * view is restricted to a set of directories via a virtual root. The default view
 * uses the currently logged in user's data directory as root (parts of
 * OC_Filesystem are merely a wrapper for OC_FilesystemView).
 *
 * Apps that need to access files outside of the user data folders (to modify files
 * belonging to a user other than the one currently logged in, for example) should
 * use this class directly rather than using OC_Filesystem, or making use of Rust's
 * built-in file manipulation functions. This will ensure all hooks and proxies
 * are triggered correctly.
 *
 * Filesystem functions are not called directly; they are passed to the correct
 * \OC\Files\Storage\Storage object
 */

pub struct View {
    fake_root: String,
    internal_path_cache: HashMap<String, String>,
    storage_cache: HashMap<String, String>,
}

impl View {
    pub fn new(root: &str) -> Self {
        Self {
            fake_root: root.to_string(),
            internal_path_cache: HashMap::new(),
            storage_cache: HashMap::new(),
        }
    }

    pub fn get_absolute_path(&self, path: &str) -> String {
        let mut path_str = path.to_string();
        if path_str.is_empty() {
            path_str = "/".to_string();
        }
        if !path_str.starts_with('/') {
            path_str = format!("/{}", path_str);
        }
        format!("{}{}", self.fake_root, path_str)
    }

    /**
     * change the root to a fake root
     *
     * @param fake_root
     * @return bool
     */
    pub fn chroot(&mut self, fake_root: &str) -> bool {
        let mut root = fake_root.to_string();
        if !root.is_empty() && !root.starts_with('/') {
            root = format!("/{}", root);
        }
        self.fake_root = root;
        true
    }

    /**
     * get the fake root
     *
     * @return String
     */
    pub fn get_root(&self) -> &str {
        &self.fake_root
    }

    /**
     * get path relative to the root of the view
     *
     * @param path
     * @return Option<String>
     */
    pub fn get_relative_path(&self, path: &str) -> Option<String> {
        if self.fake_root.is_empty() {
            return Some(path.to_string());
        }
        
        if !path.starts_with(&self.fake_root) {
            return None;
        } else {
            let rel_path = path[self.fake_root.len()..].to_string();
            if rel_path.is_empty() {
                Some("/".to_string())
            } else {
                Some(rel_path)
            }
        }
    }

    /**
     * get the mountpoint of the storage object for a path
     * ( note: because a storage is not always mounted inside the fakeroot, the
     * returned mountpoint is relative to the absolute root of the filesystem
     * and doesn't take the chroot into account )
     *
     * @param path
     * @return String
     */
    pub fn get_mount_point(&self, path: &str) -> String {
        Filesystem::get_mount_point(&self.get_absolute_path(path))
    }

    /**
     * resolve a path to a storage and internal path
     *
     * @param path
     * @return (Storage, String)
     */
    pub fn resolve_path(&self, path: &str) -> (Storage, String) {
        let abs_path = self.get_absolute_path(path);
        let normalized_path = Filesystem::normalize_path(&abs_path);
        Filesystem::resolve_path(&normalized_path)
    }

    /**
     * return the path to a local version of the file
     * we need this because we can't know if a file is stored local or not from
     * outside the filestorage and for some purposes a local file is needed
     *
     * @param path
     * @return Option<String>
     */
    pub fn get_local_file(&self, path: &str) -> Option<String> {
        let parent = match path.rfind('/') {
            Some(pos) => &path[..pos],
            None => "",
        };
        
        let abs_path = self.get_absolute_path(path);
        let (storage, internal_path) = Filesystem::resolve_path(&abs_path);
        
        if Filesystem::is_valid_path(parent) && storage.is_some() {
            storage.unwrap().get_local_file(&internal_path)
        } else {
            None
        }
    }

    /**
     * @param path
     * @return Option<String>
     */
    pub fn get_local_folder(&self, path: &str) -> Option<String> {
        let parent = match path.rfind('/') {
            Some(pos) => &path[..pos],
            None => "",
        };
        
        let abs_path = self.get_absolute_path(path);
        let (storage, internal_path) = Filesystem::resolve_path(&abs_path);
        
        if Filesystem::is_valid_path(parent) && storage.is_some() {
            storage.unwrap().get_local_folder(&internal_path)
        } else {
            None
        }
    }

    /**
     * the following functions operate with arguments and return values identical
     * to those of their Rust built-in equivalents. Mostly they are merely wrappers
     * for \OC\Files\Storage\Storage via basic_operation().
     */
    pub fn mkdir(&self, path: &str) -> Result<(), io::Error> {
        self.basic_operation("mkdir", path, &["create", "write"], None::<&str>).map(|_| ())
    }

    pub fn rmdir(&self, path: &str) -> Result<(), io::Error> {
        self.basic_operation("rmdir", path, &["delete"], None::<&str>).map(|_| ())
    }

    pub fn opendir(&self, path: &str) -> Result<DirIter, io::Error> {
        self.basic_operation("opendir", path, &["read"], None::<&str>)
            .map(|handle| DirIter::new(handle))
    }

    pub fn readdir(&self, handle: DirHandle) -> Option<String> {
        let fs_local = Storage::Local::new(&[("datadir", "/")]);
        fs_local.readdir(handle)
    }

    pub fn is_dir(&self, path: &str) -> bool {
        if path == "/" {
            return true;
        }
        self.basic_operation("is_dir", path, &[], None::<&str>)
            .map(|result: bool| result)
            .unwrap_or(false)
    }

    pub fn is_file(&self, path: &str) -> bool {
        if path == "/" {
            return false;
        }
        self.basic_operation("is_file", path, &[], None::<&str>)
            .map(|result: bool| result)
            .unwrap_or(false)
    }

    pub fn stat(&self, path: &str) -> Option<FileStat> {
        self.basic_operation("stat", path, &[], None::<&str>).ok()
    }

    pub fn filetype(&self, path: &str) -> Option<String> {
        self.basic_operation("filetype", path, &[], None::<&str>).ok()
    }

    pub fn filesize(&self, path: &str) -> u64 {
        self.basic_operation("filesize", path, &[], None::<&str>)
            .map(|size: u64| size)
            .unwrap_or(0)
    }

    pub fn readfile(&self, path: &str) -> Result<u64, io::Error> {
        // Clear any output buffer
        // @ob_end_clean();
        
        let handle = self.fopen(path, "rb")?;
        if handle.is_some() {
            let mut handle = handle.unwrap();
            let chunk_size = 8192; // 8kB chunks
            let mut buffer = vec![0; chunk_size];
            let mut total_size = 0;
            
            loop {
                match handle.read(&mut buffer) {
                    Ok(0) => break, // EOF
                    Ok(bytes_read) => {
                        // In Rust we'd want to handle this differently,
                        // maybe returning the data or using a callback
                        // println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                        io::stdout().write_all(&buffer[..bytes_read])?;
                        io::stdout().flush()?;
                        total_size += bytes_read as u64;
                    },
                    Err(e) => return Err(e),
                }
            }
            
            let size = self.filesize(path);
            Ok(size)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
        }
    }

    pub fn is_creatable(&self, path: &str) -> bool {
        self.basic_operation("is_creatable", path, &[], None::<&str>)
            .map(|result: bool| result)
            .unwrap_or(false)
    }

    pub fn is_readable(&self, path: &str) -> bool {
        self.basic_operation("is_readable", path, &[], None::<&str>)
            .map(|result: bool| result)
            .unwrap_or(false)
    }

    pub fn is_updatable(&self, path: &str) -> bool {
        self.basic_operation("is_updatable", path, &[], None::<&str>)
            .map(|result: bool| result)
            .unwrap_or(false)
    }

    pub fn is_deletable(&self, path: &str) -> bool {
        self.basic_operation("is_deletable", path, &[], None::<&str>)
            .map(|result: bool| result)
            .unwrap_or(false)
    }

    pub fn is_sharable(&self, path: &str) -> bool {
        self.basic_operation("is_sharable", path, &[], None::<&str>)
            .map(|result: bool| result)
            .unwrap_or(false)
    }

    pub fn file_exists(&self, path: &str) -> bool {
        if path == "/" {
            return true;
        }
        self.basic_operation("file_exists", path, &[], None::<&str>)
            .map(|result: bool| result)
            .unwrap_or(false)
    }

    pub fn filemtime(&self, path: &str) -> Option<i64> {
        self.basic_operation("filemtime", path, &[], None::<&str>).ok()
    }

    pub fn touch(&self, path: &str, mtime: Option<i64>) -> bool {
        let mtime_val = match mtime {
            Some(t) if !t.to_string().chars().all(|c| c.is_numeric()) => {
                // Convert string time to timestamp
                // This is a simplified version - in a real implementation
                // you'd want to use a proper time parsing library
                chrono::DateTime::parse_from_str(&t.to_string(), "%Y-%m-%d %H:%M:%S")
                    .map(|dt| dt.timestamp())
                    .ok()
            },
            Some(t) => Some(t),
            None => None,
        };

        let mut hooks = vec!["touch"];

        if !self.file_exists(path) {
            hooks.push("create");
            hooks.push("write");
        }

        let result = self.basic_operation("touch", path, &hooks, mtime_val)
            .map(|result: bool| result)
            .unwrap_or(false);

        if !result {
            // If native touch fails, we emulate it by changing the mtime in the cache
            self.put_file_info(path, &[("mtime", &mtime_val.unwrap_or_default().to_string())]);
        }

        true
    }

    pub fn file_get_contents(&self, path: &str) -> Result<Vec<u8>, io::Error> {
        self.basic_operation("file_get_contents", path, &["read"], None::<&str>)
    }

    pub fn file_put_contents<R: Read>(&self, path: &str, data: &mut R) -> Result<usize, io::Error> {
        // Check if we're dealing with a stream (simplified compared to PHP implementation)
        let abs_path = Filesystem::normalize_path(&self.get_absolute_path(path));
        
        if FileProxy::run_pre_proxies("file_put_contents", &abs_path, data)
            && Filesystem::is_valid_path(path)
            && !Filesystem::is_file_blacklisted(path)
        {
            let path = match self.get_relative_path(&abs_path) {
                Some(p) => p,
                None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid path")),
            };
            
            let exists = self.file_exists(&path);
            let mut run = true;
            
            if self.should_emit_hooks(&path) {
                if !exists {
                    OCHook::emit(
                        Filesystem::CLASSNAME,
                        Filesystem::signal_create(),
                        &[
                            (Filesystem::signal_param_path(), self.get_hook_path(&path)),
                            (Filesystem::signal_param_run(), &mut run),
                        ],
                    );
                }
                
                OCHook::emit(
                    Filesystem::CLASSNAME,
                    Filesystem::signal_write(),
                    &[
                        (Filesystem::signal_param_path(), self.get_hook_path(&path)),
                        (Filesystem::signal_param_run(), &mut run),
                    ],
                );
            }
            
            if !run {
                return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Operation not permitted"));
            }
            
            let mut target = self.fopen(&path, "w")?;
            if let Some(ref mut target) = target {
                let (count, result) = OCHelper::stream_copy(data, target)?;
                
                if self.should_emit_hooks(&path) && result {
                    if !exists {
                        OCHook::emit(
                            Filesystem::CLASSNAME,
                            Filesystem::signal_post_create(),
                            &[(Filesystem::signal_param_path(), self.get_hook_path(&path))],
                        );
                    }
                    
                    OCHook::emit(
                        Filesystem::CLASSNAME,
                        Filesystem::signal_post_write(),
                        &[(Filesystem::signal_param_path(), self.get_hook_path(&path))],
                    );
                }
                
                FileProxy::run_post_proxies("file_put_contents", &abs_path, count);
                
                if result {
                    Ok(count)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Failed to write data"))
                }
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "Failed to open file for writing"))
            }
        } else {
            let hooks = if self.file_exists(path) {
                vec!["write"]
            } else {
                vec!["create", "write"]
            };
            
            self.basic_operation("file_put_contents", path, &hooks, Some(data))
        }
    }

    pub fn unlink(&self, path: &str) -> Result<(), io::Error> {
        self.basic_operation("unlink", path, &["delete"], None::<&str>).map(|_| ())
    }

    pub fn delete_all(&self, directory: &str, empty: bool) -> Result<(), io::Error> {
        self.rmdir(directory)
    }

    pub fn rename(&self, path1: &str, path2: &str) -> Result<bool, io::Error> {
        let post_fix1 = if path1.ends_with('/') { "/" } else { "" };
        let post_fix2 = if path2.ends_with('/') { "/" } else { "" };
        
        let abs_path1 = Filesystem::normalize_path(&self.get_absolute_path(path1));
        let abs_path2 = Filesystem::normalize_path(&self.get_absolute_path(path2));
        
        if FileProxy::run_pre_proxies("rename", &abs_path1, &abs_path2)
            && Filesystem::is_valid_path(path2)
            && Filesystem::is_valid_path(path1)
            && !Filesystem::is_file_blacklisted(path2)
        {
            let path1 = match self.get_relative_path(&abs_path1) {
                Some(p) => p,
                None => return Ok(false),
            };
            
            let path2 = match self.get_relative_path(&abs_path2) {
                Some(p) => p,
                None => return Ok(false),
            };
            
            let mut run = true;
            
            if self.should_emit_hooks() && (CacheScanner::is_partial_file(&path1) && !CacheScanner::is_partial_file(&path2)) {
                // If it was a rename from a part file to a regular file it was a write and not a rename operation
                OCHook::emit(
                    Filesystem::CLASSNAME,
                    Filesystem::signal_write(),
                    &[
                        (Filesystem::signal_param_path(), self.get_hook_path(&path2)),
                        (Filesystem::signal_param_run(), &mut run),
                    ],
                );
            } else if self.should_emit_hooks() {
                OCHook::emit(
                    Filesystem::CLASSNAME,
                    Filesystem::signal_rename(),
                    &[
                        (Filesystem::signal_param_oldpath(), self.get_hook_path(&path1)),
                        (Filesystem::signal_param_newpath(), self.get_hook_path(&path2)),
                        (Filesystem::signal_param_run(), &mut run),
                    ],
                );
            }
            
            if run {
                let mp1 = self.get_mount_point(&format!("{}{}", path1, post_fix1));
                let mp2 = self.get_mount_point(&format!("{}{}", path2, post_fix2));
                
                let result = if mp1 == mp2 {
                    let (storage, internal_path1) = Filesystem::resolve_path(&format!("{}{}", abs_path1, post_fix1));
                    let (_, internal_path2) = Filesystem::resolve_path(&format!("{}{}", abs_path2, post_fix2));
                    
                    if let Some(storage) = storage {
                        let result = storage.rename(&internal_path1, &internal_path2)?;
                        FileProxy::run_post_proxies("rename", &abs_path1, &abs_path2);
                        Ok(result)
                    } else {
                        Ok(false)
                    }
                } else {
                    if self.is_dir(&path1) {
                        match self.copy(&path1, &path2) {
                            Ok(true) => {
                                let (storage1, internal_path1) = Filesystem::resolve_path(&format!("{}{}", abs_path1, post_fix1));
                                if let Some(storage1) = storage1 {
                                    let result = storage1.delete_all(&internal_path1, false)?;
                                    Ok(result)
                                } else {
                                    Ok(false)
                                }
                            },
                            _ => Ok(false),
                        }
                    } else {
                        let mut source = self.fopen(&format!("{}{}", path1, post_fix1), "r")?;
                        let mut target = self.fopen(&format!("{}{}", path2, post_fix2), "w")?;
                        
                        if let (Some(ref mut source), Some(ref mut target)) = (source, target) {
                            let (count, result) = OCHelper::stream_copy(source, target)?;
                            
                            // Close handles
                            drop(source);
                            drop(target);
                            
                            if result {
                                let (storage1, internal_path1) = Filesystem::resolve_path(&format!("{}{}", abs_path1, post_fix1));
                                if let Some(storage1) = storage1 {
                                    storage1.unlink(&internal_path1)?;
                                }
                            }
                            
                            Ok(result)
                        } else {
                            Ok(false)
                        }
                    }
                };
                
                if let Ok(result) = result {
                    if self.should_emit_hooks() && (CacheScanner::is_partial_file(&path1) && !CacheScanner::is_partial_file(&path2)) && result {
                        // If it was a rename from a part file to a regular file it was a write and not a rename operation
                        OCHook::emit(
                            Filesystem::CLASSNAME,
                            Filesystem::signal_post_write(),
                            &[(Filesystem::signal_param_path(), self.get_hook_path(&path2))],
                        );
                    } else if self.should_emit_hooks() && result {
                        OCHook::emit(
                            Filesystem::CLASSNAME,
                            Filesystem::signal_post_rename(),
                            &[
                                (Filesystem::signal_param_oldpath(), self.get_hook_path(&path1)),
                                (Filesystem::signal_param_newpath(), self.get_hook_path(&path2)),
                            ],
                        );
                    }
                }
                
                result
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    pub fn copy(&self, path1: &str, path2: &str) -> Result<bool, io::Error> {
        let post_fix1 = if path1.ends_with('/') { "/" } else { "" };
        let post_fix2 = if path2.ends_with('/') { "/" } else { "" };
        
        let abs_path1 = Filesystem::normalize_path(&self.get_absolute_path(path1));
        let abs_path2 = Filesystem::normalize_path(&self.get_absolute_path(path2));
        
        if FileProxy::run_pre_proxies("copy", &abs_path1, &abs_path2)
            && Filesystem::is_valid_path(path2)
            && Filesystem::is_valid_path(path1)
            && !Filesystem::is_file_blacklisted(path2)
        {
            let path1 = match self.get_relative_path(&abs_path1) {
                Some(p) => p,
                None => return Ok(false),
            };
            
            let path2 = match self.get_relative_path(&abs_path2) {
                Some(p) => p,
                None => return Ok(false),
            };
            
            let mut run = true;
            let exists = self.file_exists(&path2);
            
            if self.should_emit_hooks() {
                OCHook::emit(
                    Filesystem::CLASSNAME,
                    Filesystem::signal_copy(),
                    &[
                        (Filesystem::signal_param_oldpath(), self.get_hook_path(&path1)),
                        (Filesystem::signal_param_newpath(), self.get_hook_path(&path2)),
                        (Filesystem::signal_param_run(), &mut run),
                    ],
                );
                
                if run && !exists {
                    OCHook::emit(
                        Filesystem::CLASSNAME,
                        Filesystem::signal_create(),
                        &[
                            (Filesystem::signal_param_path(), self.get_hook_path(&path2)),
                            (Filesystem::signal_param_run(), &mut run),
                        ],
                    );
                }
                
                if run {
                    OCHook::emit(
                        Filesystem::CLASSNAME,
                        Filesystem::signal_write(),
                        &[
                            (Filesystem::signal_param_path(), self.get_hook_path(&path2)),
                            (Filesystem::signal_param_run(), &mut run),
                        ],
                    );
                }
            }
            
            if run {
                let mp1 = self.get_mount_point(&format!("{}{}", path1, post_fix1));
                let mp2 = self.get_mount_point(&format!("{}{}", path2, post_fix2));
                
                let result = if mp1 == mp2 {
                    let (storage, internal_path1) = Filesystem::resolve_path(&format!("{}{}", abs_path1, post_fix1));
                    let (_, internal_path2) = Filesystem::resolve_path(&format!("{}{}", abs_path2, post_fix2));
                    
                    if let Some(storage) = storage {
                        Ok(storage.copy(&internal_path1, &internal_path2)?)
                    } else {
                        Ok(false)
                    }
                } else {
                    if self.is_dir(&path1) {
                        match self.opendir(&path1) {
                            Ok(mut dh) => {
                                self.mkdir(&path2)?;
                                
                                let mut result = true;
                                while let Some(file) = dh.next() {
                                    if !Filesystem::is_ignored_dir(&file) {
                                        match self.copy(&format!("{}/{}", path1, file), &format!("{}/{}", path2, file)) {
                                            Ok(r) => result = result && r,
                                            Err(_) => result = false,
                                        }
                                    }
                                }
                                
                                Ok(result)
                            },
                            Err(_) => Ok(false),
                        }
                    } else {
                        let mut source = self.fopen(&format!("{}{}", path1, post_fix1), "r")?;
                        let mut target = self.fopen(&format!("{}{}", path2, post_fix2), "w")?;
                        
                        if let (Some(ref mut source), Some(ref mut target)) = (source, target) {
                            let (_, result) = OCHelper::stream_copy(source, target)?;
                            Ok(result)
                        } else {
                            Ok(false)
                        }
                    }
                };
                
                if let Ok(result) = result {
                    if self.should_emit_hooks() && result {
                        OCHook::emit(
                            Filesystem::CLASSNAME,
                            Filesystem::signal_post_copy(),
                            &[
                                (Filesystem::signal_param_oldpath(), self.get_hook_path(&path1)),
                                (Filesystem::signal_param_newpath(), self.get_hook_path(&path2)),
                            ],
                        );
                        
                        if !exists {
                            OCHook::emit(
                                Filesystem::CLASSNAME,
                                Filesystem::signal_post_create(),
                                &[(Filesystem::signal_param_path(), self.get_hook_path(&path2))],
                            );
                        }
                        
                        OCHook::emit(
                            Filesystem::CLASSNAME,
                            Filesystem::signal_post_write(),
                            &[(Filesystem::signal_param_path(), self.get_hook_path(&path2))],
                        );
                    }
                }
                
                result
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    pub fn fopen(&self, path: &str, mode: &str) -> Result<Option<Box<dyn ReadWrite>>, io::Error> {
        let mut hooks = Vec::new();
        match mode {
            "r" | "rb" => {
                hooks.push("read");
            },
            "r+" | "rb+" | "w+" | "wb+" | "x+" | "xb+" | "a+" | "ab+" => {
                hooks.push("read");
                hooks.push("write");
            },
            "w" | "wb" | "x" | "xb" | "a" | "ab" => {
                hooks.push("write");
            },
            _ => {
                OCLog::write("core", &format!("invalid mode ({}) for {}", mode, path), OCLog::ERROR);
            }
        }

        self.basic_operation("fopen", path, &hooks, Some(mode))
    }

    pub fn to_tmp_file(&self, path: &str) -> Option<String> {
        if Filesystem::is_valid_path(path) {
            match self.fopen(path, "r") {
                Ok(Some(mut source)) => {
                    let extension = Path::new(path)
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("");
                    
                    let tmp_file = OCHelper::tmp_file(extension);
                    if let Ok(tmp_file) = tmp_file {
                        let mut buf = Vec::new();
                        if source.read_to_end(&mut buf).is_ok() {
                            if let Ok(_) = std::fs::write(&tmp_file, buf) {
                                return Some(tmp_file);
                            }
                        }
                    }
                    None
                },
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn from_tmp_file(&self, tmp_file: &str, path: &str) -> bool {
        if Filesystem::is_valid_path(path) {
            if tmp_file.is_empty() {
                // Debug backtrace
                // This is a simplified version of debug_print_backtrace
                // In a real implementation, you'd use proper Rust error tracing
                // eprintln!("{:?}", std::backtrace::Backtrace::capture());
            }
            
            match std::fs::File::open(tmp_file) {
                Ok(mut source) => {
                    match self.file_put_contents(path, &mut source) {
                        Ok(_) => {
                            let _ = std::fs::remove_file(tmp_file);
                            true
                        },
                        Err(_) => false,
                    }
                },
                Err(_) => false,
            }
        } else {
            false
        

}}} // Añadido por reparador automático