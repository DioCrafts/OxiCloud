//! # smb_wrapper
//! 
//! This module implements a SMB client based on 'smbclient'
//!
//! Original Homepage: http://www.phpclasses.org/smb4php
//!
//! Copyright (c) 2007 Victor M. Varela <vmvarela@gmail.com>
//! Copyright (c) 2023 Rust Port Contributors
//!
//! This program is free software; you can redistribute it and/or
//! modify it under the terms of the GNU General Public License
//! as published by the Free Software Foundation; either version 2
//! of the License, or (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.

use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;
use regex::Regex;
use tempfile::NamedTempFile;
use url::Url;

const SMB_VERSION: &str = "0.8";
const SMBCLIENT: &str = "smbclient";
const SMB_OPTIONS: &str = "TCP_NODELAY IPTOS_LOWDELAY SO_KEEPALIVE SO_RCVBUF=8192 SO_SNDBUF=8192";
const AUTH_MODE: &str = "arg"; // set to 'env' to use USER environment variable

lazy_static! {
    static ref STAT_CACHE: Mutex<HashMap<String, FileStat>> = Mutex::new(HashMap::new());
    static ref DIR_CACHE: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone)]
pub struct ParsedUrl {
    domain: String,
    user: String,
    pass: String,
    host: String,
    port: u16,
    share: String,
    path: String,
    url_type: String,
}

#[derive(Debug, Clone)]
pub struct FileStat {
    size: u64,
    atime: u64,
    mtime: u64,
    ctime: u64,
    is_dir: bool,
    attr: String,
}

#[derive(Debug, Clone)]
pub struct SmbInfo {
    info: HashMap<String, FileInfo>,
    disk: Vec<String>,
    server: Vec<String>,
    workgroup: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    name: String,
    file_type: String,
    attr: String,
    size: u64,
    time: u64,
}

impl Default for SmbInfo {
    fn default() -> Self {
        SmbInfo {
            info: HashMap::new(),
            disk: Vec::new(),
            server: Vec::new(),
            workgroup: Vec::new(),
        }
    }
}

pub struct SmbClient;

impl SmbClient {
    pub fn parse_url(url_str: &str) -> Result<ParsedUrl, io::Error> {
        let url = match Url::parse(url_str) {
            Ok(u) => u,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid URL")),
        };

        let host = url.host_str().unwrap_or("").to_string();
        let port = url.port().unwrap_or(139);
        let path = url.path().trim_start_matches('/').trim_end_matches('/');
        
        let mut user = url.username().to_string();
        let pass = url.password().unwrap_or("").to_string();
        let mut domain = String::new();

        // Parse domain from username if it contains ';'
        if user.contains(';') {
            let parts: Vec<&str> = user.split(';').collect();
            if parts.len() > 1 {
                domain = parts[0].to_string();
                user = parts[1].to_string();
            }
        }
        
        // Parse share and path
        let (share, path) = if path.contains('/') {
            let parts: Vec<&str> = path.splitn(2, '/').collect();
            (parts[0].to_string(), parts[1].replace('/', "\\"))
        } else {
            (path.to_string(), String::new())
        };
        
        // Determine URL type
        let url_type = if !path.is_empty() {
            "path".to_string()
        } else if !share.is_empty() {
            "share".to_string()
        } else if !host.is_empty() {
            "host".to_string()
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "URL error"));
        };
        
        Ok(ParsedUrl {
            domain,
            user: url::percent_decode(user.as_bytes()).decode_utf8_lossy().to_string(),
            pass: url::percent_decode(pass.as_bytes()).decode_utf8_lossy().to_string(),
            host,
            port,
            share,
            path,
            url_type,
        })
    }

    pub fn look(purl: &ParsedUrl) -> Option<SmbInfo> {
        Self::client(&format!("-L {}", shell_escape::escape(purl.host.clone().into())), purl)
    }

    pub fn execute(command: &str, purl: &ParsedUrl) -> Option<SmbInfo> {
        Self::client(
            &format!(
                "-d 0 {} -c {}",
                shell_escape::escape(format!("//{}/{}", purl.host, purl.share).into()),
                shell_escape::escape(command.into())
            ),
            purl
        )
    }

    pub fn client(params: &str, purl: &ParsedUrl) -> Option<SmbInfo> {
        let auth = if AUTH_MODE == "env" {
            env::set_var("USER", format!("{}%{}", purl.user, purl.pass));
            String::new()
        } else if !purl.user.is_empty() {
            format!(" -U {}", shell_escape::escape(format!("{}%{}", purl.user, purl.pass).into()))
        } else {
            String::new()
        };

        let domain_param = if !purl.domain.is_empty() {
            format!(" -W {}", shell_escape::escape(purl.domain.clone().into()))
        } else {
            String::new()
        };

        let port_param = if purl.port != 139 {
            format!(" -p {}", shell_escape::escape(purl.port.to_string().into()))
        } else {
            String::new()
        };

        let options = format!("-O {}", shell_escape::escape(SMB_OPTIONS.into()));

        // Save current locale and set to en_US.UTF-8 for smbclient output parsing
        let old_locale = env::var("LC_ALL").ok();
        env::set_var("LC_ALL", "en_US.UTF-8");

        let cmd = format!(
            "{} -N {}{} {} {}{} {}",
            SMBCLIENT, auth, domain_param, options, port_param, options, params
        );

        let process = match Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn() {
                Ok(p) => p,
                Err(_) => {
                    // Restore locale
                    if let Some(locale) = old_locale {
                        env::set_var("LC_ALL", locale);
                    } else {
                        env::remove_var("LC_ALL");
                    }
                    return None;
                }
            };

        let mut info = SmbInfo::default();
        let mut mode = String::new();
        
        if let Some(mut stdout) = process.stdout {
            let mut buffer = String::new();
            if stdout.read_to_string(&mut buffer).is_err() {
                // Restore locale
                if let Some(locale) = old_locale {
                    env::set_var("LC_ALL", locale);
                } else {
                    env::remove_var("LC_ALL");
                }
                return None;
            }

            for line in buffer.lines() {
                // Create regex patterns similar to PHP regex
                if line.starts_with("added interface") || 
                   line == "Anonymous login successful" ||
                   line.starts_with("Domain=[") || 
                   line.starts_with("\tSharename") || 
                   line.starts_with("\t---------") || 
                   line.starts_with("\tServer") || 
                   line.starts_with("\tWorkgroup") ||
                   line.contains("IPC") ||
                   line.contains("blocks of size") ||
                   line.starts_with("Got a positive name query response") ||
                   line.contains("Job") && line.contains("cancelled") {
                    continue;
                } else if line.starts_with("\tSharename") {
                    mode = "shares".to_string();
                } else if line.starts_with("\tServer") {
                    mode = "servers".to_string();
                } else if line.starts_with("\tWorkgroup") {
                    mode = "workgroups".to_string();
                } else if line.contains("Disk") && !line.contains("IPC") {
                    // Share line
                    let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();
                    if parts.len() >= 2 {
                        let name = parts[0].trim();
                        info.disk.push(name.to_string());
                    }
                } else if line.starts_with("session setup failed:") ||
                          line.ends_with("ERRSRV - ERRbadpw") ||
                          line.starts_with("Error returning browse list:") ||
                          line.starts_with("tree connect failed:") ||
                          line.starts_with("Connection to") && line.contains("failed") ||
                          line.starts_with("NT_STATUS_") ||
                          line.contains("ERRDOS - ERRbadpath") ||
                          line.starts_with("cd") && line.contains(":") ||
                          line.contains("message start: ERRSRV - ERRmsgoff") {
                    // Error handling
                    if line.starts_with("NT_STATUS_NO_SUCH_FILE") ||
                       line.starts_with("NT_STATUS_OBJECT_NAME_COLLISION") ||
                       line.starts_with("NT_STATUS_OBJECT_PATH_NOT_FOUND") ||
                       line.starts_with("NT_STATUS_OBJECT_NAME_NOT_FOUND") ||
                       line.starts_with("NT_STATUS_FILE_IS_A_DIRECTORY") {
                        // Restore locale
                        if let Some(locale) = old_locale {
                            env::set_var("LC_ALL", locale);
                        } else {
                            env::remove_var("LC_ALL");
                        }
                        return None;
                    }
                    // Log other errors
                    eprintln!("SMB error: {} params({})", line, params);
                    if line.starts_with("Connection to") && line.contains("failed") {
                        // Restore locale
                        if let Some(locale) = old_locale {
                            env::set_var("LC_ALL", locale);
                        } else {
                            env::remove_var("LC_ALL");
                        }
                        return None;
                    }
                } else if mode == "servers" || mode == "workgroups" {
                    // Server or workgroup lines
                    if line.starts_with("\t") {
                        let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
                        if parts.len() >= 1 {
                            let name = parts[0].trim().to_lowercase();
                            if mode == "servers" {
                                info.server.push(name);
                            } else {
                                info.workgroup.push(name);
                                // Master info could be extracted as parts[1] if needed
                            }
                        }
                    }
                } else if line.trim().len() > 0 {
                    // Try to match file lines like: "  filename          123456  Mon Jan  1 12:34:56 2023"
                    let re = Regex::new(r"^\s+(.+?)\s+([0-9]+)\s+(Mon|Tue|Wed|Thu|Fri|Sat|Sun)\s+(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\s+([0-9]+)\s+([0-9]{2}:[0-9]{2}:[0-9]{2})\s+([0-9]{4})$").unwrap();
                    if let Some(caps) = re.captures(line) {
                        let name_with_attr = caps.get(1).unwrap().as_str().trim();
                        let (name, attr) = if name_with_attr.ends_with('D') || 
                                             name_with_attr.ends_with('A') || 
                                             name_with_attr.ends_with('H') || 
                                             name_with_attr.ends_with('S') || 
                                             name_with_attr.ends_with('R') {
                            let re_attr = Regex::new(r"^(.*)[ ]+([D|A|H|S|R]+)$").unwrap();
                            if let Some(attr_caps) = re_attr.captures(name_with_attr) {
                                (attr_caps.get(1).unwrap().as_str().trim(), attr_caps.get(2).unwrap().as_str())
                            } else {
                                (name_with_attr, "")
                            }
                        } else {
                            (name_with_attr, "")
                        };

                        if name != "." && name != ".." {
                            let size = caps.get(2).unwrap().as_str().parse::<u64>().unwrap_or(0);
                            let month = match caps.get(4).unwrap().as_str() {
                                "Jan" => 1, "Feb" => 2, "Mar" => 3, "Apr" => 4,
                                "May" => 5, "Jun" => 6, "Jul" => 7, "Aug" => 8,
                                "Sep" => 9, "Oct" => 10, "Nov" => 11, "Dec" => 12,
                                _ => 1,
                            };
                            let day = caps.get(5).unwrap().as_str().parse::<u32>().unwrap_or(1);
                            let year = caps.get(7).unwrap().as_str().parse::<i32>().unwrap_or(1970);
                            let time_parts: Vec<&str> = caps.get(6).unwrap().as_str().split(':').collect();
                            let hour = time_parts[0].parse::<u32>().unwrap_or(0);
                            let minute = time_parts[1].parse::<u32>().unwrap_or(0);
                            let second = time_parts[2].parse::<u32>().unwrap_or(0);

                            // Simplified timestamp calculation
                            let time = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs();

                            let file_type = if attr.contains('D') { "folder" } else { "file" };
                            
                            let file_info = FileInfo {
                                name: name.to_string(),
                                file_type: file_type.to_string(),
                                attr: attr.to_string(),
                                size,
                                time,
                            };
                            
                            info.info.insert(name.to_string(), file_info);
                        }
                    }
                }
            }
        }

        // Restore previous locale
        if let Some(locale) = old_locale {
            env::set_var("LC_ALL", locale);
        } else {
            env::remove_var("LC_ALL");
        }

        Some(info)
    }

    // Stats functions
    pub fn url_stat(url_str: &str) -> Option<FileStat> {
        if let Some(s) = Self::get_stat_cache(url_str) {
            return Some(s);
        }

        let pu = match Self::parse_url(url_str) {
            Ok(p) => p,
            Err(_) => return None,
        };

        match pu.url_type.as_str() {
            "host" => {
                if Self::look(&pu).is_some() {
                    // Return default stat for host (similar to dir stat in PHP)
                    Some(FileStat {
                        size: 0,
                        atime: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
                        mtime: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
                        ctime: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
                        is_dir: true,
                        attr: String::new(),
                    })
                } else {
                    eprintln!("url_stat(): list failed for host '{}'", pu.host);
                    None
                }
            },
            "share" => {
                if let Some(o) = Self::look(&pu) {
                    let lshare = pu.share.to_lowercase();
                    let mut found = false;
                    
                    for s in &o.disk {
                        if lshare == s.to_lowercase() {
                            found = true;
                            break;
                        }
                    }
                    
                    if found {
                        // Return default stat for share (similar to dir stat in PHP)
                        Some(FileStat {
                            size: 0,
                            atime: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
                            mtime: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
                            ctime: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
                            is_dir: true,
                            attr: String::new(),
                        })
                    } else {
                        eprintln!("url_stat(): disk resource '{}' not found in '{}'", lshare, pu.host);
                        None
                    }
                } else {
                    None
                }
            },
            "path" => {
                if let Some(o) = Self::execute(&format!("dir \"{}\"", pu.path), &pu) {
                    let parts: Vec<&str> = pu.path.split('\\').collect();
                    let name = parts.last().unwrap_or(&"");
                    
                    if let Some(file_info) = o.info.get(*name) {
                        let stat = FileStat {
                            size: file_info.size,
                            atime: file_info.time,
                            mtime: file_info.time,
                            ctime: file_info.time,
                            is_dir: file_info.file_type == "folder",
                            attr: file_info.attr.clone(),
                        };
                        
                        Self::add_stat_cache(url_str, stat.clone());
                        Some(stat)
                    } else {
                        eprintln!("url_stat(): path '{}' not found", pu.path);
                        None
                    }
                } else {
                    // Don't report error for path not found
                    None
                }
            },
            _ => {
                eprintln!("error in URL");
                None
            }
        }
    }

    fn add_stat_cache(url_str: &str, info: FileStat) -> FileStat {
        let url = url_str.replace("//", "/").trim_end_matches('/').to_string();
        
        let mut cache = STAT_CACHE.lock().unwrap();
        cache.insert(url, info.clone());
        info
    }

    fn get_stat_cache(url_str: &str) -> Option<FileStat> {
        let url = url_str.replace("//", "/").trim_end_matches('/').to_string();
        
        let cache = STAT_CACHE.lock().unwrap();
        cache.get(&url).cloned()
    }

    pub fn clear_stat_cache(url_str: &str) {
        let url = if url_str.is_empty() {
            String::new()
        } else {
            url_str.replace("//", "/").trim_end_matches('/').to_string()
        };
        
        let mut cache = STAT_CACHE.lock().unwrap();
        if url.is_empty() {
            cache.clear();
        } else {
            cache.remove(&url);
        }
    }

    // Commands
    pub fn unlink(url_str: &str) -> bool {
        let pu = match Self::parse_url(url_str) {
            Ok(p) => p,
            Err(_) => return false,
        };
        
        if pu.url_type != "path" {
            eprintln!("unlink(): error in URL");
            return false;
        }
        
        Self::clear_stat_cache(url_str);
        
        if let Some(parent) = Path::new(url_str).parent() {
            SmbStreamWrapper::clear_dir_cache(parent.to_str().unwrap_or(""));
        }
        
        Self::execute(&format!("del \"{}\"", pu.path), &pu).is_some()
    }

    pub fn rename(url_from: &str, url_to: &str) -> bool {
        let from = match Self::parse_url(url_from) {
            Ok(p) => p,
            Err(_) => return false,
        };
        
        let to = match Self::parse_url(url_to) {
            Ok(p) => p,
            Err(_) => return false,
        };
        
        if from.host != to.host || 
           from.share != to.share || 
           from.user != to.user || 
           from.pass != to.pass || 
           from.domain != to.domain {
            eprintln!("rename(): FROM & TO must be in same server-share-user-pass-domain");
            return false;
        }
        
        if from.url_type != "path" || to.url_type != "path" {
            eprintln!("rename(): error in URL");
            return false;
        }
        
        Self::clear_stat_cache(url_from);
        Self::execute(&format!("rename \"{}\" \"{}\"", from.path, to.path), &to).is_some()
    }

    pub fn mkdir(url_str: &str) -> bool {
        let pu = match Self::parse_url(url_str) {
            Ok(p) => p,
            Err(_) => return false,
        };
        
        if pu.url_type != "path" {
            eprintln!("mkdir(): error in URL");
            return false;
        }
        
        Self::execute(&format!("mkdir \"{}\"", pu.path), &pu).is_some()
    }

    pub fn rmdir(url_str: &str) -> bool {
        let pu = match Self::parse_url(url_str) {
            Ok(p) => p,
            Err(_) => return false,
        };
        
        if pu.url_type != "path" {
            eprintln!("rmdir(): error in URL");
            return false;
        }
        
        Self::clear_stat_cache(url_str);
        
        if let Some(parent) = Path::new(url_str).parent() {
            SmbStreamWrapper::clear_dir_cache(parent.to_str().unwrap_or(""));
        }
        
        Self::execute(&format!("rmdir \"{}\"", pu.path), &pu).is_some()
    }
}

// SMB Stream Wrapper implementation
pub struct SmbStreamWrapper {
    stream: Option<File>,
    url: String,
    parsed_url: ParsedUrl,
    mode: String,
    tmpfile: Option<PathBuf>,
    need_flush: bool,
    dir: Vec<String>,
    dir_index: i32,
}

impl SmbStreamWrapper {
    pub fn new() -> Self {
        SmbStreamWrapper {
            stream: None,
            url: String::new(),
            parsed_url: ParsedUrl {
                domain: String::new(),
                user: String::new(),
                pass: String::new(),
                host: String::new(),
                port: 139,
                share: String::new(),
                path: String::new(),
                url_type: String::new(),
            },
            mode: String::new(),
            tmpfile: None,
            need_flush: false,
            dir: Vec::new(),
            dir_index: -1,
        }
    }

    pub fn stream_open(&mut self, url_str: &str, mode: &str) -> bool {
        self.url = url_str.to_string();
        self.mode = mode.to_string();
        
        match SmbClient::parse_url(url_str) {
            Ok(pu) => {
                self.parsed_url = pu;
                if self.parsed_url.url_type != "path" {
                    eprintln!("stream_open(): error in URL");
                    return false;
                }
            },
            Err(_) => return false,
        }
        
        match mode {
            "r" | "r+" | "rb" | "a" | "a+" => {
                // Create temp file for downloading
                let tmp = match NamedTempFile::new() {
                    Ok(tmp) => tmp,
                    Err(_) => return false,
                };
                let path = tmp.path().to_path_buf();
                
                // Download the file
                if SmbClient::execute(
                    &format!("get \"{}\" \"{}\"", self.parsed_url.path, path.display()),
                    &self.parsed_url
                ).is_none() {
                    return false;
                }
                
                // Open the temp file
                let file = match File::open(&path) {
                    Ok(f) => f,
                    Err(_) => return false,
                };
                
                self.stream = Some(file);
                self.tmpfile = Some(path);
            },
            "w" | "w+" | "wb" | "x" | "x+" => {
                // Create temp file for uploading
                let tmp = match NamedTempFile::new() {
                    Ok(tmp) => tmp,
                    Err(_) => return false,
                };
                let path = tmp.path().to_path_buf();
                
                // Open the temp file
                let file = match File::create(&path) {
                    Ok(f) => f,
                    Err(_) => return false,
                };
                
                self.clear_dir_cache();
                self.stream = Some(file);
                self.tmpfile = Some(path);
                self.need_flush = true;
            },
            _ => return false,
        }
        
        true
    }

    pub fn stream_close(&mut self) -> bool {
        if let Some(ref mut stream) = self.stream {
            stream.flush().is_ok()
        } else {
            false
        }
    }

    pub fn stream_read(&mut self, count: usize) -> Vec<u8> {
        let mut buffer = vec![0; count];
        if let Some(ref mut stream) = self.stream {
            match stream.read(&mut buffer) {
                Ok(n) => buffer[..n].to_vec(),
                Err(_) => Vec::new(),
            }
        } else {
            Vec::new()
        }
    }

    pub fn stream_write(&mut self, data: &[u8]) -> usize {
        self.need_flush = true;
        if let Some(ref mut stream) = self.stream {
            match stream.write(data) {
                Ok(n) => n,
                Err(_) => 0,
            }
        } else {
            0
        }
    }

    pub fn stream_eof(&mut self) -> bool {
        if let Some(ref mut stream) = self.stream {
            let pos = match stream.stream_position() {
                Ok(p) => p,
                Err(_) => return true,
            };
            
            let len = match stream.metadata() {
                Ok(m) => m.len(),
                Err(_) => return true,
            };
            
            pos >= len
        } else {
            true
        }
    }

    pub fn stream_tell(&mut self) -> u64 {
        if let Some(ref mut stream) = self.stream {
            match stream.stream_position() {
                Ok(p) => p,
                Err(_) => 0,
            }
        } else {
            0
        }
    }

    pub fn stream_seek(&mut self, offset: i64, whence: io::SeekFrom) -> bool {
        if let Some(ref mut stream) = self.stream {
            match stream.seek(whence) {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }

    pub fn stream_flush(&mut self) -> bool {
        if self.mode != "r" && self.need_flush {
            if let Some(ref path) = self.tmpfile {
                SmbClient::clear_stat_cache(&self.url);
                if SmbClient::execute(
                    &format!("put \"{}\" \"{}\"", path.display(), self.parsed_url.path),
                    &self.parsed_url
                ).is_some() {
                    self.need_flush = false;
                    return true;
                }
            }
            false
        } else {
            true
        }
    }

    pub fn stream_stat(&self) -> Option<FileStat> {
        SmbClient::url_stat(&self.url)
    }

    pub fn dir_opendir(&mut self, url_str: &str) -> bool {
        if let Some(d) = self.get_dir_cache(url_str) {
            self.dir = d;
            self.dir_index = 0;
            return true;
        }
        
        let pu = match SmbClient::parse_url(url_str) {