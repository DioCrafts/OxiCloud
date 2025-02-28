// Pure-Rust implementation of SFTP.
//
// Currently only supports SFTPv2 and v3, which, according to wikipedia.org, "is the most widely used version,
// implemented by the popular OpenSSH SFTP server". If you want SFTPv4/5/6 support, provide access
// to an SFTPv4/5/6 server.
//
// The API for this library is modeled after the API from PHP's FTP extension.
//
// Here's a short example of how to use this library:
// ```rust
// use std::error::Error;
// use sftp::Sftp;
//
// fn main() -> Result<(), Box<dyn Error>> {
//     let mut sftp = Sftp::new("www.domain.tld", 22, 10)?;
//     if !sftp.login("username", "password")? {
//         return Err("Login Failed".into());
//     }
//
//     println!("{}", sftp.pwd()?);
//     sftp.put("filename.ext", "hello, world!", sftp::PutMode::STRING)?;
//     println!("{:?}", sftp.nlist()?);
//     Ok(())
// }
//

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::Instant;
use ssh2::{Channel, Session, Sftp as SSH2Sftp};
use thiserror::Error;

// Constants
pub const CHANNEL: i32 = 2;

// Log levels
pub const LOG_SIMPLE: u8 = 1;
pub const LOG_COMPLEX: u8 = 2;
pub const LOG_REALTIME: u8 = 3;

// Put modes
pub const LOCAL_FILE: u32 = 1;
pub const STRING: u32 = 2;
pub const RESUME: u32 = 4;

// Packet types
const SFTP_INIT: u8 = 1;
const SFTP_VERSION: u8 = 2;
const SFTP_OPEN: u8 = 3;
const SFTP_CLOSE: u8 = 4;
const SFTP_READ: u8 = 5;
const SFTP_WRITE: u8 = 6;
const SFTP_LSTAT: u8 = 7;
const SFTP_SETSTAT: u8 = 9;
const SFTP_OPENDIR: u8 = 11;
const SFTP_READDIR: u8 = 12;
const SFTP_REMOVE: u8 = 13;
const SFTP_MKDIR: u8 = 14;
const SFTP_RMDIR: u8 = 15;
const SFTP_REALPATH: u8 = 16;
const SFTP_STAT: u8 = 17;
const SFTP_RENAME: u8 = 18;
const SFTP_STATUS: u8 = 101;
const SFTP_HANDLE: u8 = 102;
const SFTP_DATA: u8 = 103;
const SFTP_NAME: u8 = 104;
const SFTP_ATTRS: u8 = 105;
const SFTP_EXTENDED: u8 = 200;

// Status codes
const STATUS_OK: u32 = 0;
const STATUS_EOF: u32 = 1;
const STATUS_NO_SUCH_FILE: u32 = 2;
const STATUS_PERMISSION_DENIED: u32 = 3;
const STATUS_FAILURE: u32 = 4;
const STATUS_BAD_MESSAGE: u32 = 5;
const STATUS_NO_CONNECTION: u32 = 6;
const STATUS_CONNECTION_LOST: u32 = 7;
const STATUS_OP_UNSUPPORTED: u32 = 8;

// Attribute flags
const ATTR_SIZE: u32 = 0x00000001;
const ATTR_UIDGID: u32 = 0x00000002;
const ATTR_PERMISSIONS: u32 = 0x00000004;
const ATTR_ACCESSTIME: u32 = 0x00000008;
const ATTR_EXTENDED: u32 = 0x80000000;

// Open flags
const OPEN_READ: u32 = 0x00000001;
const OPEN_WRITE: u32 = 0x00000002;
const OPEN_APPEND: u32 = 0x00000004;
const OPEN_CREATE: u32 = 0x00000008;
const OPEN_TRUNCATE: u32 = 0x00000010;

// File types
const TYPE_REGULAR: u8 = 1;
const TYPE_DIRECTORY: u8 = 2;
const TYPE_SYMLINK: u8 = 3;
const TYPE_SPECIAL: u8 = 4;

/// PutMode enum for specifying put operation mode
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PutMode {
    LocalFile = LOCAL_FILE as isize,
    String = STRING as isize,
    Resume = RESUME as isize,
}

#[derive(Error, Debug)]
pub enum SftpError {
    #[error("SSH error: {0}")]
    Ssh(#[from] ssh2::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("SFTP error: {0}")]
    Status(u32, String),
    
    #[error("Unexpected packet type: expected {expected}, got {got}")]
    UnexpectedPacketType { expected: String, got: u8 },
    
    #[error("Not logged in")]
    NotLoggedIn,
    
    #[error("Path not found: {0}")]
    PathNotFound(String),
    
    #[error("Local file not found: {0}")]
    LocalFileNotFound(String),
    
    #[error("Invalid file: {0}")]
    InvalidFile(String),
    
    #[error("{0}")]
    Custom(String),
}

/// Result type for SFTP operations
pub type Result<T> = std::result::Result<T, SftpError>;

/// File attribute information
#[derive(Debug, Clone)]
pub struct FileAttributes {
    pub size: Option<u64>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub permissions: Option<u32>,
    pub atime: Option<u32>,
    pub mtime: Option<u32>,
    pub file_type: Option<u8>,
    pub extended: HashMap<String, String>,
}

impl Default for FileAttributes {
    fn default() -> Self {
        Self {
            size: None,
            uid: None,
            gid: None, 
            permissions: None,
            atime: None,
            mtime: None,
            file_type: None,
            extended: HashMap::new(),
        }
    }
}

/// SFTP client implementation
pub struct Sftp {
    session: Session,
    sftp: Option<SSH2Sftp>,
    /// Request ID for packet handling
    request_id: u32,
    /// Current packet type
    packet_type: i32,
    /// Buffer for packet data
    packet_buffer: Vec<u8>,
    /// Server extensions
    extensions: HashMap<String, String>,
    /// Server SFTP version
    version: u32,
    /// Current working directory
    pwd: Option<String>,
    /// Packet type log
    packet_type_log: Vec<String>,
    /// Packet content log
    packet_log: Vec<Vec<u8>>,
    /// Error information
    sftp_errors: Vec<String>,
    /// File type information
    file_type: u8,
    /// Directory cache
    dirs: HashMap<String, HashMap<String, ()>>,
    /// Whether login was successful
    logged_in: bool,
}

impl Sftp {
    /// Creates a new SFTP client and connects to the server
    pub fn new(host: &str, port: u16, timeout: u64) -> Result<Self> {
        // Set up TCP connection and SSH session
        let tcp = std::net::TcpStream::connect(format!("{}:{}", host, port))?;
        tcp.set_read_timeout(Some(std::time::Duration::from_secs(timeout)))?;
        tcp.set_write_timeout(Some(std::time::Duration::from_secs(timeout)))?;
        
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        
        Ok(Self {
            session,
            sftp: None,
            request_id: 1,
            packet_type: -1,
            packet_buffer: Vec::new(),
            extensions: HashMap::new(),
            version: 0,
            pwd: None,
            packet_type_log: Vec::new(),
            packet_log: Vec::new(),
            sftp_errors: Vec::new(),
            file_type: 0,
            dirs: HashMap::new(),
            logged_in: false,
        })
    }
    
    /// Log in to the SFTP server
    pub fn login(&mut self, username: &str, password: &str) -> Result<bool> {
        self.session.userauth_password(username, password)?;
        
        if !self.session.authenticated() {
            return Ok(false);
        }
        
        // Initialize SFTP subsystem
        let sftp = self.session.sftp()?;
        self.sftp = Some(sftp);
        
        // Send SFTP init packet
        let init_data = &[0, 0, 0, 3]; // version 3
        self.send_sftp_packet(SFTP_INIT, init_data)?;
        
        // Get server response
        let response = self.get_sftp_packet()?;
        if self.packet_type as u8 != SFTP_VERSION {
            return Err(SftpError::UnexpectedPacketType {
                expected: "SSH_FXP_VERSION".to_string(),
                got: self.packet_type as u8,
            });
        }
        
        // Parse version and extensions
        let version = u32::from_be_bytes([response[0], response[1], response[2], response[3]]);
        self.version = version;
        
        let mut pos = 4;
        while pos < response.len() {
            let key_len = u32::from_be_bytes([
                response[pos], response[pos+1], response[pos+2], response[pos+3]
            ]) as usize;
            pos += 4;
            
            let key = String::from_utf8_lossy(&response[pos..pos+key_len]).to_string();
            pos += key_len;
            
            let value_len = u32::from_be_bytes([
                response[pos], response[pos+1], response[pos+2], response[pos+3]
            ]) as usize;
            pos += 4;
            
            let value = String::from_utf8_lossy(&response[pos..pos+value_len]).to_string();
            pos += value_len;
            
            self.extensions.insert(key, value);
        }
        
        match self.version {
            2 | 3 => {}, // Supported versions
            _ => return Ok(false),
        }
        
        // Get current working directory
        self.pwd = Some(self.realpath(".", false)?);
        
        if let Some(pwd) = &self.pwd {
            self.save_dir(pwd);
        }
        
        self.logged_in = true;
        Ok(true)
    }
    
    /// Returns the current directory
    pub fn pwd(&self) -> Result<String> {
        self.pwd.clone().ok_or_else(|| SftpError::Custom("No current directory".to_string()))
    }
    
    /// Logs SFTP errors
    fn log_error(&mut self, response: &[u8], status: Option<u32>) -> SftpError {
        let status_code = match status {
            Some(s) => s,
            None => u32::from_be_bytes([response[0], response[1], response[2], response[3]]),
        };
        
        let error = match status_code {
            STATUS_OK => "OK",
            STATUS_EOF => "End of file",
            STATUS_NO_SUCH_FILE => "No such file",
            STATUS_PERMISSION_DENIED => "Permission denied",
            STATUS_FAILURE => "Failure",
            STATUS_BAD_MESSAGE => "Bad message",
            STATUS_NO_CONNECTION => "No connection",
            STATUS_CONNECTION_LOST => "Connection lost",
            STATUS_OP_UNSUPPORTED => "Operation unsupported",
            _ => "Unknown error",
        };
        
        let mut error_str = error.to_string();
        
        // Extract detailed error message from response if available (SFTPv3+)
        if self.version > 2 && response.len() >= 8 {
            let msg_len = u32::from_be_bytes([
                response[4], response[5], response[6], response[7]
            ]) as usize;
            
            if response.len() >= 8 + msg_len {
                let msg = String::from_utf8_lossy(&response[8..8+msg_len]);
                error_str = format!("{}: {}", error, msg);
            }
        }
        
        self.sftp_errors.push(error_str.clone());
        SftpError::Status(status_code, error_str)
    }
    
    /// Canonicalize the Server-Side Path Name
    ///
    /// SFTP doesn't provide a mechanism by which the current working directory can be changed,
    /// so we'll emulate it. Returns the absolute (canonicalized) path.
    fn realpath(&mut self, dir: &str, check_dir: bool) -> Result<String> {
        if check_dir && self.is_dir(dir) {
            return Ok(dir.to_string());
        }
        
        let mut file = String::new();
        let mut dir_path = dir.to_string();
        
        if let Some(pwd) = &self.pwd {
            // If the SFTP server returned the canonicalized path even for non-existent files this wouldn't be necessary
            // on OpenSSH it isn't necessary but on other SFTP servers it is.
            if dir.is_empty() || !dir.ends_with('/') {
                let path = Path::new(dir);
                if let Some(filename) = path.file_name() {
                    file = filename.to_string_lossy().to_string();
                    dir_path = match path.parent() {
                        Some(p) => p.to_string_lossy().to_string(),
                        None => ".".to_string(),
                    };
                }
            }
            
            if dir_path.starts_with('/') {
                dir_path = format!("/{}", dir_path.trim_start_matches('/').trim_end_matches('/'));
            } else {
                dir_path = dir_path.trim_end_matches('/').to_string();
            }
            
            if dir_path == "." || dir_path == *pwd {
                let mut temp = pwd.clone();
                if !file.is_empty() {
                    temp = format!("{}/{}", temp, file);
                }
                return Ok(temp);
            }
            
            if !dir_path.starts_with('/') {
                dir_path = format!("{}/{}", pwd, dir_path);
            }
        }
        
        // Send REALPATH request
        let dir_bytes = dir_path.as_bytes();
        let mut packet = Vec::with_capacity(4 + dir_bytes.len());
        packet.extend_from_slice(&(dir_bytes.len() as u32).to_be_bytes());
        packet.extend_from_slice(dir_bytes);
        
        self.send_sftp_packet(SFTP_REALPATH, &packet)?;
        
        let response = self.get_sftp_packet()?;
        
        match self.packet_type as u8 {
            SFTP_NAME => {
                // Skip count field
                let mut pos = 4;
                
                // Extract realpath
                let path_len = u32::from_be_bytes([
                    response[pos], response[pos+1], response[pos+2], response[pos+3]
                ]) as usize;
                pos += 4;
                
                let realpath = String::from_utf8_lossy(&response[pos..pos+path_len]).to_string();
                pos += path_len;
                
                // Extract longname for file type info (SFTPv3 only)
                let longname_len = u32::from_be_bytes([
                    response[pos], response[pos+1], response[pos+2], response[pos+3]
                ]) as usize;
                pos += 4;
                
                if pos + longname_len <= response.len() {
                    let longname = String::from_utf8_lossy(&response[pos..pos+longname_len]).to_string();
                    self.file_type = self.parse_longname(&longname);
                }
                
                // Append filename if needed
                let mut final_path = realpath;
                if !file.is_empty() {
                    final_path = format!("{}/{}", final_path, file);
                }
                
                Ok(final_path)
            },
            SFTP_STATUS => {
                Err(self.log_error(&response, None))
            },
            _ => {
                Err(SftpError::UnexpectedPacketType {
                    expected: "SSH_FXP_NAME or SSH_FXP_STATUS".to_string(),
                    got: self.packet_type as u8,
                })
            }
        }
    }
    
    /// Changes the current directory
    pub fn chdir(&mut self, dir: &str) -> Result<bool> {
        if !self.logged_in {
            return Err(SftpError::NotLoggedIn);
        }
        
        let mut dir_path = dir.to_string();
        if !dir_path.ends_with('/') {
            dir_path.push('/');
        }
        
        // Check if this is a valid directory
        if self.is_dir(&dir_path) {
            self.pwd = Some(dir_path);
            return Ok(true);
        }
        
        let dir_path = self.realpath(&dir_path, false)?;
        
        if self.is_dir(&dir_path) {
            self.pwd = Some(dir_path);
            return Ok(true);
        }
        
        // Try to open as directory to validate
        let dir_bytes = dir_path.as_bytes();
        let mut packet = Vec::with_capacity(4 + dir_bytes.len());
        packet.extend_from_slice(&(dir_bytes.len() as u32).to_be_bytes());
        packet.extend_from_slice(dir_bytes);
        
        self.send_sftp_packet(SFTP_OPENDIR, &packet)?;
        
        let response = self.get_sftp_packet()?;
        
        match self.packet_type as u8 {
            SFTP_HANDLE => {
                let handle = response[4..].to_vec();
                
                // Close directory handle
                let mut close_packet = Vec::with_capacity(4 + handle.len());
                close_packet.extend_from_slice(&(handle.len() as u32).to_be_bytes());
                close_packet.extend_from_slice(&handle);
                
                self.send_sftp_packet(SFTP_CLOSE, &close_packet)?;
                
                let close_response = self.get_sftp_packet()?;
                if self.packet_type as u8 != SFTP_STATUS {
                    return Err(SftpError::UnexpectedPacketType {
                        expected: "SSH_FXP_STATUS".to_string(),
                        got: self.packet_type as u8,
                    });
                }
                
                let status = u32::from_be_bytes([
                    close_response[0], close_response[1], close_response[2], close_response[3]
                ]);
                
                if status != STATUS_OK {
                    return Err(self.log_error(&close_response, Some(status)));
                }
                
                self.save_dir(&dir_path);
                self.pwd = Some(dir_path);
                Ok(true)
            },
            SFTP_STATUS => {
                Err(self.log_error(&response, None))
            },
            _ => {
                Err(SftpError::UnexpectedPacketType {
                    expected: "SSH_FXP_HANDLE or SSH_FXP_STATUS".to_string(),
                    got: self.packet_type as u8,
                })
            }
        }
    }
    
    /// Returns a list of files in the given directory
    pub fn nlist(&mut self, dir: Option<&str>) -> Result<Vec<String>> {
        let entries = self.list(dir.unwrap_or("."), false)?;
        Ok(entries.into_iter().map(|(name, _)| name).collect())
    }
    
    /// Returns a detailed list of files in the given directory
    pub fn rawlist(&mut self, dir: Option<&str>) -> Result<HashMap<String, FileAttributes>> {
        self.list(dir.unwrap_or("."), true)
    }
    
    /// Reads a list of files in the given directory
    fn list<T>(&mut self, dir: &str, raw: bool) -> Result<T>
    where {
        T: From<HashMap<String, FileAttributes>> + From<Vec<String>>
    {
        if !self.logged_in {
            return Err(SftpError::NotLoggedIn);
        }
        
        let dir_path = self.realpath(dir, true)?;
        if dir_path.is_empty() {
            return Err(SftpError::PathNotFound(dir.to_string()));
        }
        
        let dir_bytes = format!("{}/", dir_path).as_bytes();
        let mut packet = Vec::with_capacity(4 + dir_bytes.len());
        packet.extend_from_slice(&(dir_bytes.len() as u32).to_be_bytes());
        packet.extend_from_slice(dir_bytes);
        
        self.send_sftp_packet(SFTP_OPENDIR, &packet)?;
        
        let response = self.get_sftp_packet()?;
        
        let handle = match self.packet_type as u8 {
            SFTP_HANDLE => {
                response[4..].to_vec()
            },
            SFTP_STATUS => {
                return Err(self.log_error(&response, None));
            },
            _ => {
                return Err(SftpError::UnexpectedPacketType {
                    expected: "SSH_FXP_HANDLE or SSH_FXP_STATUS".to_string(),
                    got: self.packet_type as u8,
                });
            }
        };
        
        self.save_dir(&dir_path);
        
        let mut contents: HashMap<String, FileAttributes> = HashMap::new();
        
        loop {
            let mut packet = Vec::with_capacity(4 + handle.len());
            packet.extend_from_slice(&(handle.len() as u32).to_be_bytes());
            packet.extend_from_slice(&handle);
            
            self.send_sftp_packet(SFTP_READDIR, &packet)?;
            
            let response = self.get_sftp_packet()?;
            
            match self.packet_type as u8 {
                SFTP_NAME => {
                    let mut pos = 0;
                    let count = u32::from_be_bytes([
                        response[pos], response[pos+1], response[pos+2], response[pos+3]
                    ]) as usize;
                    pos += 4;
                    
                    for _ in 0..count {
                        // Extract filename
                        let name_len = u32::from_be_bytes([
                            response[pos], response[pos+1], response[pos+2], response[pos+3]
                        ]) as usize;
                        pos += 4;
                        
                        let shortname = String::from_utf8_lossy(&response[pos..pos+name_len]).to_string();
                        pos += name_len;
                        
                        // Extract longname
                        let longname_len = u32::from_be_bytes([
                            response[pos], response[pos+1], response[pos+2], response[pos+3]
                        ]) as usize;
                        pos += 4;
                        
                        let longname = String::from_utf8_lossy(&response[pos..pos+longname_len]).to_string();
                        pos += longname_len;
                        
                        // Parse attributes
                        let (attributes, new_pos) = self.parse_attributes(&response[pos..]);
                        pos += new_pos;
                        
                        if !raw {
                            // For non-raw listing, we just collect the names
                            continue;
                        }
                        
                        // For raw listing, collect attributes with file type
                        let mut attrs = attributes;
                        let file_type = self.parse_longname(&longname);
                        if file_type != 0 {
                            if file_type == TYPE_DIRECTORY && shortname != "." && shortname != ".." {
                                self.save_dir(&format!("{}/{}", dir_path, shortname));
                            }
                            attrs.file_type = Some(file_type);
                        }
                        
                        contents.insert(shortname, attrs);
                    }
                },
                SFTP_STATUS => {
                    let status = u32::from_be_bytes([
                        response[0], response[1], response[2], response[3]
                    ]);
                    
                    if status != STATUS_EOF {
                        return Err(self.log_error(&response, Some(status)));
                    }
                    break;
                },
                _ => {
                    return Err(SftpError::UnexpectedPacketType {
                        expected: "SSH_FXP_NAME or SSH_FXP_STATUS".to_string(),
                        got: self.packet_type as u8,
                    });
                }
            }
        }
        
        // Close directory handle
        let mut close_packet = Vec::with_capacity(4 + handle.len());
        close_packet.extend_from_slice(&(handle.len() as u32).to_be_bytes());
        close_packet.extend_from_slice(&handle);
        
        self.send_sftp_packet(SFTP_CLOSE, &close_packet)?;
        
        let close_response = self.get_sftp_packet()?;
        if self.packet_type as u8 != SFTP_STATUS {
            return Err(SftpError::UnexpectedPacketType {
                expected: "SSH_FXP_STATUS".to_string(),
                got: self.packet_type as u8,
            });
        }
        
        let status = u32::from_be_bytes([
            close_response[0], close_response[1], close_response[2], close_response[3]
        ]);
        
        if status != STATUS_OK {
            return Err(self.log_error(&close_response, Some(status)));
        }
        
        if raw {
            Ok(T::from(contents))
        } else {
            let names: Vec<String> = contents.keys().cloned().collect();
            Ok(T::from(names))
        }
    }
    
    /// Returns the file size in bytes, or error on failure
    pub fn size(&mut self, filename: &str) -> Result<u64> {
        if !self.logged_in {
            return Err(SftpError::NotLoggedIn);
        }
        
        let path = self.realpath(filename, false)?;
        
        self.size_internal(&path)
    }
    
    /// Save directories to cache
    fn save_dir(&mut self, dir: &str) {
        let parts: Vec<&str> = dir
            .trim_start_matches('/')
            .trim_end_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        
        let mut current_map = &mut self.dirs;
        
        for part in parts {
            current_map.entry(part.to_string())
                .or_insert_with(HashMap::new);
            
            if let Some(next_map) = current_map.get_mut(part) {
                current_map = next_map;
            }
        }
    }
    
    /// Remove directories from cache
    fn remove_dir(&mut self, dir: &str) -> bool {
        let parts: Vec<&str> = dir
            .trim_start_matches('/')
            .trim_end_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        
        if parts.is_empty() {
            return false;
        }
        
        let mut maps = Vec::new();
        maps.push(&mut self.dirs);
        
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                if let Some(map) = maps.last_mut() {
                    map.remove(*part);
                    return true;
                }
                return false;
            }
            
            if let Some(map) = maps.last_mut() {
                if let Some(next_map) = map.get_mut(*part) {
                    maps.push(next_map);
                } else {
                    return false;
                }
            }
        }
        
        false
    }
    
    /// Checks cache for directory
    fn is_dir(&self, dir: &str) -> bool {
        let parts: Vec<&str> = dir
            .trim_start_matches('/')
            .trim_end_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        
        let mut current_map = &self.dirs;
        
        for part in parts {
            if let Some(next_map) = current_map.get(part) {
                current_map = next_map;
            } else {
                return false;
            }
        }
        
        true
    }
    
    /// Returns general information about a file
    pub fn stat(&mut self, filename: &str) -> Result<FileAttributes> {
        if !self.logged_in {
            return Err(SftpError::NotLoggedIn);
        }
        
        let path = self.realpath(filename, false)?;
        
        let stat = self.stat_internal(&path, SFTP_STAT)?;
        
        // Determine if it's a directory by trying to chdir to it
        let

}} // Añadido por reparador automático

} // Añadido por reparador automático