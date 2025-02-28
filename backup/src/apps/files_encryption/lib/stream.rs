// # Encryption Stream
//
// Transparently encrypted filestream
//
// This module provides the 'crypt://' stream wrapper protocol.
// We use a stream wrapper because it is the most secure way to handle
// decrypted content transfers. There is no safe way to decrypt the entire file
// somewhere on the server, so we have to encrypt and decrypt blocks on the fly.
//
// Paths used with this protocol MUST BE RELATIVE. Use URLs like:
// crypt://filename, or crypt://subdirectory/filename, NOT
// crypt:///home/user/owncloud/data. Otherwise keyfiles will be put in
// [owncloud]/data/user/files_encryption/keyfiles/home/user/owncloud/data and
// will not be accessible to other methods.
//
// Data read and written must always be 8192 bytes long, as this is the
// buffer size used internally by PHP. The encryption process makes the input
// data longer, and input is chunked into smaller pieces in order to result in
// a 8192 encrypted block size.
//
// When files are deleted via webdav, or when they are updated and the
// previous version deleted, this is handled by the FileView, and thus the
// encryption proxies are used and keyfiles deleted.

use std::io::{Read, Write, Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::fmt;

use crate::crypt::Crypt;
use crate::keymanager::Keymanager;
use crate::helper::Helper;
use crate::session::Session;
use crate::util::Util;

// Constants
const BLOCK_SIZE: usize = 8192;
const PLAIN_BLOCK_SIZE: usize = 6126;

#[derive(Debug)]
pub enum StreamError {
    IoError(std::io::Error),
    PathError(String),
    KeyError(String),
    CryptError(String),
    PermissionError(String),
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamError::IoError(err) => write!(f, "IO error: {}", err),
            StreamError::PathError(msg) => write!(f, "Path error: {}", msg),
            StreamError::KeyError(msg) => write!(f, "Key error: {}", msg),
            StreamError::CryptError(msg) => write!(f, "Cryptography error: {}", msg),
            StreamError::PermissionError(msg) => write!(f, "Permission error: {}", msg),
        }
    }
}

impl Error for StreamError {}

impl From<std::io::Error> for StreamError {
    fn from(error: std::io::Error) -> Self {
        StreamError::IoError(error)
    }
}

/// A file view implementation that provides access to the filesystem
pub trait FileView {
    fn file_exists(&self, path: &str) -> bool;
    fn filesize(&self, path: &str, mode: &str) -> Result<usize, StreamError>;
    fn fopen(&self, path: &str, mode: &str) -> Result<Box<dyn FileHandle>, StreamError>;
    fn unlink(&self, path: &str) -> Result<(), StreamError>;
    fn get_file_info(&self, path: &str) -> Result<HashMap<String, Value>, StreamError>;
    fn put_file_info(&self, path: &str, info: HashMap<String, Value>) -> Result<(), StreamError>;
}

/// A file handle trait for abstracting file operations
pub trait FileHandle: Read + Write + Seek {
    fn tell(&self) -> Result<u64, StreamError>;
    fn flush(&mut self) -> Result<(), StreamError>;
    fn close(self: Box<Self>) -> Result<(), StreamError>;
    fn set_blocking(&mut self, blocking: bool) -> Result<bool, StreamError>;
    fn set_timeout(&mut self, seconds: i32, microseconds: i32) -> Result<bool, StreamError>;
    fn set_write_buffer(&mut self, buffer_size: usize) -> Result<bool, StreamError>;
    fn stat(&self) -> Result<HashMap<String, Value>, StreamError>;
    fn lock(&self, mode: i32) -> Result<bool, StreamError>;
    fn eof(&self) -> bool;
}

/// A variant type for storing different value types
#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(HashMap<String, Value>),
}

/// Represents an encrypted file stream
pub struct Stream {
    plain_key: Option<String>,
    enc_keyfiles: Option<HashMap<String, String>>,
    raw_path: String,
    rel_path: String,
    user_id: String,
    handle: Option<Box<dyn FileHandle>>,
    meta: HashMap<String, String>,
    write_cache: String,
    size: usize,
    unencrypted_size: usize,
    public_key: Option<String>,
    enc_keyfile: Option<String>,
    new_file: bool,
    root_view: Arc<dyn FileView>,
    session: Arc<Session>,
    private_key: Option<String>,
}

impl Stream {
    /// Create a new Stream instance
    pub fn new(root_view: Arc<dyn FileView>, session: Arc<Session>) -> Self {
        Stream {
            plain_key: None,
            enc_keyfiles: None,
            raw_path: String::new(),
            rel_path: String::new(),
            user_id: String::new(),
            handle: None,
            meta: HashMap::new(),
            write_cache: String::new(),
            size: 0,
            unencrypted_size: 0,
            public_key: None,
            enc_keyfile: None,
            new_file: false,
            root_view,
            session,
            private_key: None,
        }
    }

    /// Open a stream with the given path and mode
    pub fn stream_open(&mut self, path: &str, mode: &str) -> Result<bool, StreamError> {
        // assume that the file already exist before we decide it finally in get_key()
        self.new_file = false;

        let util = Util::new(Arc::clone(&self.root_view), crate::user::get_user());
        self.user_id = util.get_user_id();
        self.private_key = self.session.get_private_key(&self.user_id);

        // rawPath is relative to the data directory
        self.raw_path = path.replace("crypt://", "").trim_matches('/').to_string();

        // Strip identifier text from path, this gives us the path relative to data/<user>/files
        self.rel_path = match Helper::strip_user_files_path(&self.raw_path) {
            Some(path) => path,
            None => {
                // if raw path doesn't point to a real file, check if it is a version or a file in the trash bin
                match Helper::get_path_to_real_file(&self.raw_path) {
                    Some(path) => path,
                    None => {
                        log::error!(
                            "Encryption library: failed to open file \"{}\" expecting a path to user/files or to user/files_versions", 
                            self.raw_path
                        );
                        return Err(StreamError::PathError(format!(
                            "Invalid path: {}", self.raw_path
                        )));
                    }
                }
            }
        };

        // Disable fileproxies so we can get the file size and open the source file without recursive encryption
        let proxy_status = crate::file_proxy::disable();

        if mode == "w" || mode == "w+" || mode == "wb" || mode == "wb+" {
            // We're writing a new file so start write counter with 0 bytes
            self.size = 0;
            self.unencrypted_size = 0;
        } else {
            if self.private_key.is_none() {
                // if private key is not valid redirect user to a error page
                Helper::redirect_to_error_page(&self.session);
                return Err(StreamError::KeyError("Private key is not valid".to_string()));
            }

            self.size = self.root_view.filesize(&self.raw_path, mode)?;
        }

        match self.root_view.fopen(&self.raw_path, mode) {
            Ok(handle) => {
                self.handle = Some(handle);
                let meta = HashMap::new();
                meta.insert("mode".to_string(), mode.to_string());
                self.meta = meta;
                crate::file_proxy::set_enabled(proxy_status);
                Ok(true)
            }
            Err(e) => {
                log::error!(
                    "Encryption library: failed to open file \"{}\"", 
                    self.raw_path
                );
                crate::file_proxy::set_enabled(proxy_status);
                Err(e)
            }
        }
    }

    /// Seek to a position in the stream
    pub fn stream_seek(&mut self, offset: i64, whence: SeekFrom) -> Result<u64, StreamError> {
        self.flush()?;
        
        if let Some(handle) = &mut self.handle {
            Ok(handle.seek(whence)?)
        } else {
            Err(StreamError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No handle available"
            )))
        }
    }

    /// Read from the stream
    pub fn stream_read(&mut self, count: usize) -> Result<Vec<u8>, StreamError> {
        self.write_cache.clear();

        if count != BLOCK_SIZE {
            // This makes this function a lot simpler, but will break this class if the standard block size changes
            return Err(StreamError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Expected block size of {}, got {}", BLOCK_SIZE, count)
            )));
        }

        // Get the data from the file handle
        let mut data = vec![0u8; count];
        let bytes_read = if let Some(handle) = &mut self.handle {
            handle.read(&mut data)?
        } else {
            return Err(StreamError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No handle available"
            )));
        };
        
        data.truncate(bytes_read);

        if !data.is_empty() {
            if !self.get_key()? {
                // Error! We don't have a key to decrypt the file with
                return Err(StreamError::KeyError(format!(
                    "Encryption key not found for \"{}\" during attempted read via stream",
                    self.raw_path
                )));
            } else {
                // Decrypt data
                return match Crypt::symmetric_decrypt_file_content(&data, self.plain_key.as_ref().unwrap()) {
                    Ok(decrypted) => Ok(decrypted),
                    Err(e) => Err(StreamError::CryptError(format!("Failed to decrypt: {}", e))),
                };
            }
        }

        Ok(Vec::new())
    }

    /// Encrypt and pad data ready for writing to disk
    fn pre_write_encrypt(&self, plain_data: &[u8], key: &str) -> Result<Vec<u8>, StreamError> {
        match Crypt::symmetric_encrypt_file_content(plain_data, key) {
            Ok(encrypted) => Ok(encrypted),
            Err(e) => Err(StreamError::CryptError(format!("Failed to encrypt: {}", e))),
        }
    }

    /// Fetch the plain encryption key for the file and set it as plain_key property
    fn get_key(&mut self) -> Result<bool, StreamError> {
        // Check if key is already set
        if self.plain_key.is_some() && self.enc_keyfile.is_some() {
            return Ok(true);
        }

        // Fetch and decrypt keyfile
        // Fetch existing keyfile
        self.enc_keyfile = Keymanager::get_file_key(&self.root_view, &self.user_id, &self.rel_path)?;

        // If a keyfile already exists
        if let Some(enc_keyfile) = &self.enc_keyfile {
            let share_key = Keymanager::get_share_key(&self.root_view, &self.user_id, &self.rel_path)?;

            // if there is no valid private key return false
            if self.private_key.is_none() {
                // if private key is not valid redirect user to a error page
                Helper::redirect_to_error_page(&self.session);
                return Err(StreamError::KeyError("Private key is not valid".to_string()));
            }

            if share_key.is_none() {
                // if no share key is available redirect user to a error page
                Helper::redirect_to_error_page(
                    &self.session, 
                    Some(crate::crypt::ENCRYPTION_NO_SHARE_KEY_FOUND)
                );
                return Err(StreamError::KeyError("No share key available".to_string()));
            }

            match Crypt::multi_key_decrypt(
                enc_keyfile,
                &share_key.unwrap(),
                self.private_key.as_ref().unwrap()
            ) {
                Ok(plain_key) => {
                    self.plain_key = Some(plain_key);
                    Ok(true)
                },
                Err(e) => Err(StreamError::CryptError(format!("Failed to decrypt key: {}", e))),
            }
        } else {
            self.new_file = true;
            Ok(false)
        }
    }

    /// Write to the stream
    pub fn stream_write(&mut self, data: &[u8]) -> Result<usize, StreamError> {
        // if there is no valid private key return false
        if self.private_key.is_none() {
            self.size = 0;
            return Ok(data.len());
        }

        // Disable the file proxies so that encryption is not
        // automatically attempted when the file is written to disk -
        // we are handling that separately here and we don't want to
        // get into an infinite loop
        let proxy_status = crate::file_proxy::disable();

        // Get the length of the unencrypted data that we are handling
        let length = data.len();

        // Find out where we are up to in the writing of data to the file
        let pointer = if let Some(handle) = &mut self.handle {
            handle.tell()?
        } else {
            return Err(StreamError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No handle available"
            )));
        };

        // Get / generate the keyfile for the file we're handling
        // If we're writing a new file (not overwriting an existing
        // one), save the newly generated keyfile
        if !self.get_key()? {
            self.plain_key = Some(Crypt::generate_key()?);
        }

        // Process data with write cache
        let mut processed_data = Vec::new();
        if !self.write_cache.is_empty() {
            processed_data.extend_from_slice(self.write_cache.as_bytes());
            processed_data.extend_from_slice(data);
            self.write_cache.clear();
        } else {
            processed_data.extend_from_slice(data);
        }

        let mut current_data = processed_data;

        // While there still remains some data to be processed & written
        while !current_data.is_empty() {
            // Remaining length for this iteration
            let remaining_length = current_data.len();

            // If data remaining to be written is less than the size of 1 block
            if remaining_length < PLAIN_BLOCK_SIZE {
                // Set writeCache to contents of current_data
                self.write_cache = String::from_utf8_lossy(&current_data).to_string();
                current_data.clear();
            } else {
                // Read the chunk from the start of current_data
                let chunk = &current_data[0..PLAIN_BLOCK_SIZE];

                match self.pre_write_encrypt(chunk, self.plain_key.as_ref().unwrap()) {
                    Ok(encrypted) => {
                        if let Some(handle) = &mut self.handle {
                            handle.write_all(&encrypted)?;
                        } else {
                            return Err(StreamError::IoError(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "No handle available"
                            )));
                        }
                    },
                    Err(e) => return Err(e),
                }

                // Remove the chunk we just processed
                current_data = current_data[PLAIN_BLOCK_SIZE..].to_vec();
            }
        }

        self.size = std::cmp::max(self.size, pointer as usize + length);
        self.unencrypted_size += length;

        crate::file_proxy::set_enabled(proxy_status);

        Ok(length)
    }

    /// Set stream options
    pub fn stream_set_option(&mut self, option: i32, arg1: i32, arg2: i32) -> Result<bool, StreamError> {
        if let Some(handle) = &mut self.handle {
            match option {
                1 /* STREAM_OPTION_BLOCKING */ => handle.set_blocking(arg1 != 0),
                2 /* STREAM_OPTION_READ_TIMEOUT */ => handle.set_timeout(arg1, arg2),
                3 /* STREAM_OPTION_WRITE_BUFFER */ => handle.set_write_buffer(arg1 as usize),
                _ => Err(StreamError::IoError(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Unknown stream option: {}", option)
                ))),
            }
        } else {
            Err(StreamError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No handle available"
            )))
        }
    }

    /// Get stream stats
    pub fn stream_stat(&self) -> Result<HashMap<String, Value>, StreamError> {
        if let Some(handle) = &self.handle {
            handle.stat()
        } else {
            Err(StreamError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No handle available"
            )))
        }
    }

    /// Lock the stream
    pub fn stream_lock(&self, mode: i32) -> Result<bool, StreamError> {
        if let Some(handle) = &self.handle {
            handle.lock(mode)
        } else {
            Err(StreamError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No handle available"
            )))
        }
    }

    /// Flush the stream
    pub fn stream_flush(&mut self) -> Result<bool, StreamError> {
        if let Some(handle) = &mut self.handle {
            handle.flush()?;
            Ok(true)
        } else {
            Err(StreamError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No handle available"
            )))
        }
    }

    /// Check if at end of stream
    pub fn stream_eof(&self) -> bool {
        if let Some(handle) = &self.handle {
            handle.eof()
        } else {
            true
        }
    }

    /// Flush write cache to disk
    fn flush(&mut self) -> Result<(), StreamError> {
        if !self.write_cache.is_empty() {
            // Set keyfile property for file in question
            self.get_key()?;

            let encrypted = self.pre_write_encrypt(
                self.write_cache.as_bytes(),
                self.plain_key.as_ref().unwrap()
            )?;

            if let Some(handle) = &mut self.handle {
                handle.write_all(&encrypted)?;
                self.write_cache.clear();
                Ok(())
            } else {
                Err(StreamError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No handle available"
                )))
            }
        } else {
            Ok(())
        }
    }

    /// Close the stream
    pub fn stream_close(&mut self) -> Result<(), StreamError> {
        self.flush()?;

        // if there is no valid private key
        if self.private_key.is_none() {
            // cleanup
            if let Some(mode) = self.meta.get("mode") {
                if mode != "r" && mode != "rb" {
                    // Disable encryption proxy to prevent recursive calls
                    let proxy_status = crate::file_proxy::disable();

                    if self.root_view.file_exists(&self.raw_path) && self.size == 0 {
                        self.root_view.unlink(&self.raw_path)?;
                    }

                    // Re-enable proxy - our work is done
                    crate::file_proxy::set_enabled(proxy_status);
                }
            }

            // if private key is not valid redirect user to a error page
            Helper::redirect_to_error_page(&self.session);
            return Err(StreamError::KeyError("Private key is not valid".to_string()));
        }

        if let Some(mode) = self.meta.get("mode") {
            if mode != "r" && mode != "rb" && self.size > 0 {
                // only write keyfiles if it was a new file
                if self.new_file {
                    // Disable encryption proxy to prevent recursive calls
                    let proxy_status = crate::file_proxy::disable();

                    // Fetch user's public key
                    self.public_key = Keymanager::get_public_key(&self.root_view, &self.user_id)?;

                    // Check if sharing API is enabled
                    let sharing_enabled = crate::share::is_enabled();

                    let util = Util::new(Arc::clone(&self.root_view), &self.user_id);

                    // Get all users sharing the file includes current user
                    let unique_user_ids = util.get_sharing_users_array(
                        sharing_enabled, 
                        &self.rel_path, 
                        &self.user_id
                    )?;
                    
                    let checked_user_ids = util.filter_share_ready_users(unique_user_ids)?;

                    // Fetch public keys for all sharing users
                    let public_keys = Keymanager::get_public_keys(
                        &self.root_view, 
                        &checked_user_ids.ready
                    )?;

                    // Encrypt enc key for all sharing users
                    self.enc_keyfiles = Some(Crypt::multi_key_encrypt(
                        self.plain_key.as_ref().unwrap(), 
                        &public_keys
                    )?);

                    if let Some(enc_keyfiles) = &self.enc_keyfiles {
                        // Save the new encrypted file key
                        Keymanager::set_file_key(
                            &self.root_view, 
                            &self.rel_path, 
                            &self.user_id, 
                            &enc_keyfiles["data"]
                        )?;

                        // Save the sharekeys
                        Keymanager::set_share_keys(
                            &self.root_view, 
                            &self.rel_path, 
                            &enc_keyfiles["keys"]
                        )?;
                    }

                    // Re-enable proxy - our work is done
                    crate::file_proxy::set_enabled(proxy_status);
                }

                // get file info
                let mut file_info = match self.root_view.get_file_info(&self.raw_path) {
                    Ok(info) => info,
                    Err(_) => HashMap::new(),
                };

                // set encryption data
                file_info.insert("encrypted".to_string(), Value::Bool(true));
                file_info.insert("size".to_string(), Value::Int(self.size as i64));
                file_info.insert("unencrypted_size".to_string(), Value::Int(self.unencrypted_size as i64));

                // set fileinfo
                self.root_view.put_file_info(&self.raw_path, file_info)?;
            }
        }

        if let Some(handle) = self.handle.take() {
            handle.close()?;
        }

        Ok(())
    }
}