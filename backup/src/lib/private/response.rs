use chrono::{DateTime, Duration, Utc};
use md5::{Digest, Md5};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process;

/// Response handling utilities
pub struct Response;

impl Response {
    pub const STATUS_FOUND: u16 = 304;
    pub const STATUS_NOT_MODIFIED: u16 = 304;
    pub const STATUS_TEMPORARY_REDIRECT: u16 = 307;
    pub const STATUS_NOT_FOUND: u16 = 404;
    pub const STATUS_INTERNAL_SERVER_ERROR: u16 = 500;

    /// Enable response caching by sending correct HTTP headers
    ///
    /// # Arguments
    /// * `cache_time` - Time to cache the response
    ///   * > 0: cache time in seconds
    ///   * 0 and < 0: enable default browser caching
    ///   * None: cache indefinitely
    pub fn enable_caching(cache_time: Option<i64>) {
        if let Some(cache_time) = cache_time {
            println!("Pragma: public"); // enable caching in IE
            if cache_time > 0 {
                Self::set_expires_header(&format!("PT{}S", cache_time));
                println!("Cache-Control: max-age={}, must-revalidate", cache_time);
            } else {
                Self::set_expires_header("0");
                println!("Cache-Control: must-revalidate, post-check=0, pre-check=0");
            }
        } else {
            println!("Cache-Control: cache");
            println!("Pragma: cache");
        }
    }

    /// Disable browser caching
    /// See enable_caching with cache_time = 0
    pub fn disable_caching() {
        Self::enable_caching(Some(0));
    }

    /// Set response status
    ///
    /// # Arguments
    /// * `status` - A HTTP status code, see also the STATUS constants
    pub fn set_status(status: u16) {
        let protocol = std::env::var("SERVER_PROTOCOL").unwrap_or_else(|_| "HTTP/1.0".to_string());
        
        let status_message = match status {
            Self::STATUS_NOT_MODIFIED => "Not Modified",
            Self::STATUS_TEMPORARY_REDIRECT => {
                if protocol == "HTTP/1.1" {
                    "Temporary Redirect"
                } else {
                    return Self::set_status(Self::STATUS_FOUND);
                }
            },
            Self::STATUS_FOUND => "Found",
            Self::STATUS_NOT_FOUND => "Not Found",
            Self::STATUS_INTERNAL_SERVER_ERROR => "Internal Server Error",
            _ => "",
        };

        println!("{} {} {}", protocol, status, status_message);
    }

    /// Send redirect response
    ///
    /// # Arguments
    /// * `location` - URL to redirect to
    pub fn redirect(location: &str) {
        Self::set_status(Self::STATUS_TEMPORARY_REDIRECT);
        println!("Location: {}", location);
    }

    /// Set response expire time
    ///
    /// # Arguments
    /// * `expires` - When the response expires
    ///   * String for Duration from now (ISO 8601 format starting with 'P')
    ///   * DateTime object when to expire response
    pub fn set_expires_header<T: AsRef<str>>(expires: T) {
        let expires_ref = expires.as_ref();
        let header_value = if expires_ref.starts_with('P') {
            // Parse ISO 8601 duration
            let now = Utc::now();
            
            // Simple parsing of PT{n}S format (seconds only)
            if let Some(seconds_str) = expires_ref.strip_prefix("PT").and_then(|s| s.strip_suffix('S')) {
                if let Ok(seconds) = seconds_str.parse::<i64>() {
                    let future_time = now + Duration::seconds(seconds);
                    future_time.to_rfc2822()
                } else {
                    expires_ref.to_string()
                }
            } else {
                expires_ref.to_string()
            }
        } else {
            expires_ref.to_string()
        };

        println!("Expires: {}", header_value);
    }

    /// Checks and set ETag header, when the request matches sends a
    /// 'not modified' response
    ///
    /// # Arguments
    /// * `etag` - Token to use for modification check
    pub fn set_etag_header(etag: &str) {
        if etag.is_empty() {
            return;
        }

        let etag_formatted = format!("\"{}\"", etag);
        
        if let Ok(if_none_match) = std::env::var("HTTP_IF_NONE_MATCH") {
            if if_none_match.trim() == etag_formatted {
                Self::set_status(Self::STATUS_NOT_MODIFIED);
                process::exit(0);
            }
        }

        println!("ETag: {}", etag_formatted);
    }

    /// Checks and set Last-Modified header, when the request matches sends a
    /// 'not modified' response
    ///
    /// # Arguments
    /// * `last_modified` - Time when the response was last modified
    pub fn set_last_modified_header<T>(last_modified: T)
    where
        T: Into<Option<DateTime<Utc>>>,
    {
        let last_modified = match last_modified.into() {
            Some(dt) => dt,
            None => return,
        };

        let formatted = last_modified.to_rfc2822();
        
        if let Ok(if_modified_since) = std::env::var("HTTP_IF_MODIFIED_SINCE") {
            if if_modified_since.trim() == formatted {
                Self::set_status(Self::STATUS_NOT_MODIFIED);
                process::exit(0);
            }
        }

        println!("Last-Modified: {}", formatted);
    }

    /// Send file as response, checking and setting caching headers
    ///
    /// # Arguments
    /// * `filepath` - Path of file to send
    pub fn send_file<P: AsRef<Path>>(filepath: P) -> io::Result<()> {
        let path = filepath.as_ref();
        let metadata = std::fs::metadata(path)?;
        
        // Set Last-Modified header
        let last_modified = DateTime::<Utc>::from(metadata.modified()?);
        Self::set_last_modified_header(last_modified);
        
        // Calculate and set ETag
        let mut file = File::open(path)?;
        let mut hasher = Md5::new();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        hasher.update(&buffer);
        let result = hasher.finalize();
        let etag = format!("{:x}", result);
        Self::set_etag_header(&etag);
        
        // Set content length and send file
        println!("Content-Length: {}", metadata.len());
        print!("{}", String::from_utf8_lossy(&buffer));
        
        Ok(())
    }
}