//! # OwnCloud Logging Module
//!
//! Logging utilities for ownCloud.
//! Log is saved at data/owncloud.log (on default)
//!
//! Originally written by Robin Appelman
//! Copyright 2012 Robin Appelman icewind1991@gmail.com
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::OnceLock;

use crate::config::Config;
use crate::server::Server;

/// Log entry structure for serialization/deserialization
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub app: String,
    pub message: String,
    pub level: u8,
    pub time: String,
}

pub struct OwnCloudLogger;

static LOG_FILE: OnceLock<String> = OnceLock::new();

impl OwnCloudLogger {
    /// Initialize the logger
    pub fn init() -> Result<(), std::io::Error> {
        if LOG_FILE.get().is_some() {
            return Ok(());
        }

        let server_root = Server::server_root();
        let default_data_dir = format!("{}/data", server_root);
        let data_directory = Config::get_value("datadirectory", &default_data_dir);
        let default_log_file = format!("{}/owncloud.log", data_directory);
        let log_file = Config::get_value("logfile", &default_log_file);

        // Fallback to default log file if specified logfile does not exist
        // and cannot be created
        let path = Path::new(&log_file);
        if !path.exists() {
            match File::create(path) {
                Ok(_) => (),
                Err(_) => {
                    // Fallback to default log file
                    let _ = LOG_FILE.set(default_log_file);
                    return Ok(());
                }
            }
        }

        let _ = LOG_FILE.set(log_file);
        Ok(())
    }

    /// Write a message to the log
    pub fn write(app: &str, message: &str, level: u8) -> Result<(), std::io::Error> {
        Self::init()?;

        let min_level = std::cmp::min(
            Config::get_value_as_u8("loglevel", LogLevel::WARN),
            LogLevel::ERROR,
        );

        if level < min_level {
            return Ok(());
        }

        let log_file = LOG_FILE.get().unwrap();
        
        // Default to ISO8601
        let format = Config::get_value("logdateformat", "c");
        let log_timezone = Config::get_value("logtimezone", "UTC");
        
        let timezone = match chrono_tz::Tz::from_str_insensitive(&log_timezone) {
            Ok(tz) => tz,
            Err(_) => chrono_tz::UTC,
        };
        
        let time: DateTime<_> = Utc::now().with_timezone(&timezone);
        let formatted_time = match format.as_str() {
            "c" => time.to_rfc3339(),
            _ => time.format(&format).to_string(),
        };

        let entry = LogEntry {
            app: app.to_string(),
            message: message.to_string(),
            level,
            time: formatted_time,
        };

        let entry_json = serde_json::to_string(&entry)?;
        
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)
        {
            Ok(mut file) => {
                // Set permissions to 0640
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let permissions = std::fs::Permissions::from_mode(0o640);
                    std::fs::set_permissions(log_file, permissions)?;
                }
                
                writeln!(file, "{}", entry_json)?;
                Ok(())
            }
            Err(e) => {
                // Fall back to error_log
                eprintln!("{}", entry_json);
                Err(e)
            }
        }
    }

    /// Get entries from the log in reverse chronological order
    pub fn get_entries(limit: usize, offset: usize) -> Result<Vec<LogEntry>, std::io::Error> {
        Self::init()?;
        
        let min_level = Config::get_value_as_u8("loglevel", LogLevel::WARN);
        let log_file = LOG_FILE.get().unwrap();
        
        let mut entries = Vec::new();
        let file = match File::open(log_file) {
            Ok(file) => file,
            Err(_) => return Ok(entries),
        };
        
        let mut reader = BufReader::new(file);
        let mut pos = reader.seek(SeekFrom::End(0))?;
        
        let mut line = String::new();
        let mut entries_count = 0;
        let mut lines = 0;
        
        // Buffer for reading characters
        let mut buf = [0; 1];
        
        // Loop through each character of the file looking for new lines
        while pos > 0 && entries_count < limit {
            reader.seek(SeekFrom::Start(pos - 1))?;
            if reader.read_exact(&mut buf).is_err() {
                break;
            }
            
            let ch = buf[0] as char;
            
            if ch == '\n' || pos == 1 {
                if !line.is_empty() {
                    // Add the first character if at the start of the file
                    if pos == 1 {
                        line.insert(0, ch);
                    }
                    
                    // Parse the JSON line
                    if let Ok(entry) = serde_json::from_str::<LogEntry>(&line) {
                        // Add the line as an entry if it is passed the offset and is equal or above the log level
                        if entry.level >= min_level {
                            lines += 1;
                            if lines > offset {
                                entries.push(entry);
                                entries_count += 1;
                            }
                        }
                    }
                    
                    line.clear();
                }
            } else {
                line.insert(0, ch);
            }
            
            pos -= 1;
        }
        
        Ok(entries)
    }
}

/// Log level constants
pub struct LogLevel;

impl LogLevel {
    pub const DEBUG: u8 = 0;
    pub const INFO: u8 = 1;
    pub const WARN: u8 = 2;
    pub const ERROR: u8 = 3;
    pub const FATAL: u8 = 4;
}