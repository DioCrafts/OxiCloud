// Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use encoding_rs::UTF_8;
use encoding_rs_io::DecodeReaderBytesBuilder;

pub struct OcRequest {
    // Prevent instantiation as this is a static utility class
    _private: (),
}

impl OcRequest {
    /// Check overwrite condition
    /// Returns bool
    fn is_overwrite_condition(server_vars: &HashMap<String, String>, request_type: Option<&str>) -> bool {
        let regex_str = OcConfig::get_value("overwritecondaddr", "");
        let regex = format!("/{}/", regex_str);
        
        regex == "//" 
            || match Regex::new(&regex) {
                Ok(re) => re.is_match(server_vars.get("REMOTE_ADDR").unwrap_or(&String::new())),
                Err(_) => false,
            }
            || (request_type != Some("protocol") && OcConfig::get_value_bool("forcessl", false))
    }

    /// Returns the server host
    /// 
    /// Returns the server host, even if the website uses one or more
    /// reverse proxies
    pub fn server_host(server_vars: &HashMap<String, String>, is_cli: bool) -> String {
        if is_cli {
            return "localhost".to_string();
        }

        if OcConfig::get_value("overwritehost", "") != "" && Self::is_overwrite_condition(server_vars, None) {
            return OcConfig::get_value("overwritehost", "");
        }

        if let Some(forwarded_host) = server_vars.get("HTTP_X_FORWARDED_HOST") {
            if forwarded_host.contains(",") {
                let parts: Vec<&str> = forwarded_host.split(',').collect();
                if let Some(last) = parts.last() {
                    return last.trim().to_string();
                }
            } else {
                return forwarded_host.clone();
            }
        }

        if let Some(host) = server_vars.get("HTTP_HOST") {
            return host.clone();
        }

        if let Some(server_name) = server_vars.get("SERVER_NAME") {
            return server_name.clone();
        }

        "localhost".to_string()
    }

    /// Returns the server protocol
    /// 
    /// Returns the server protocol. It respects reverse proxy servers and load balancers
    pub fn server_protocol(server_vars: &HashMap<String, String>) -> String {
        if OcConfig::get_value("overwriteprotocol", "") != "" && 
           Self::is_overwrite_condition(server_vars, Some("protocol")) {
            return OcConfig::get_value("overwriteprotocol", "");
        }

        if let Some(proto) = server_vars.get("HTTP_X_FORWARDED_PROTO") {
            return proto.to_lowercase();
        }

        if let Some(https) = server_vars.get("HTTPS") {
            if !https.is_empty() && https != "off" {
                return "https".to_string();
            }
        }

        "http".to_string()
    }

    /// Returns the request uri
    /// 
    /// Returns the request uri, even if the website uses one or more
    /// reverse proxies
    pub fn request_uri(server_vars: &HashMap<String, String>) -> String {
        let uri = server_vars.get("REQUEST_URI").cloned().unwrap_or_default();
        
        if OcConfig::get_value("overwritewebroot", "") != "" && Self::is_overwrite_condition(server_vars, None) {
            let script_name = server_vars.get("SCRIPT_NAME").cloned().unwrap_or_default();
            if uri.starts_with(&script_name) {
                let suffix = &uri[script_name.len()..];
                return format!("{}{}", Self::script_name(server_vars), suffix);
            }
        }
        
        uri
    }

    /// Returns the script name
    /// 
    /// Returns the script name, even if the website uses one or more
    /// reverse proxies
    pub fn script_name(server_vars: &HashMap<String, String>) -> String {
        let name = server_vars.get("SCRIPT_NAME").cloned().unwrap_or_default();
        
        if OcConfig::get_value("overwritewebroot", "") != "" && Self::is_overwrite_condition(server_vars, None) {
            let dir = Path::new(file!()).parent().unwrap_or_else(|| Path::new("."));
            let serverroot = dir.parent().unwrap_or_else(|| Path::new("."))
                .to_string_lossy()
                .replace('\\', "/");
                
            if let Some(script_filename) = server_vars.get("SCRIPT_FILENAME") {
                if let Ok(real_path) = std::fs::canonicalize(script_filename) {
                    let real_path_str = real_path.to_string_lossy().replace('\\', "/");
                    if real_path_str.starts_with(&serverroot) {
                        let suburi = &real_path_str[serverroot.len()..];
                        return format!("{}{}", OcConfig::get_value("overwritewebroot", ""), suburi);
                    }
                }
            }
        }
        
        name
    }

    /// Get Path info from request
    /// Returns Path info or empty string when not found
    pub fn get_path_info(server_vars: &HashMap<String, String>) -> String {
        if let Some(path_info) = server_vars.get("PATH_INFO") {
            return path_info.clone();
        }
        
        let path_info = Self::get_raw_path_info(server_vars);
        let decoded_path = urlencoding::decode(&path_info).unwrap_or_else(|_| std::borrow::Cow::Borrowed(&path_info));
        
        // Check encoding and convert if necessary
        let (cow, _encoding_used, _had_errors) = UTF_8.decode(decoded_path.as_bytes());
        cow.into_owned()
    }

    /// Get Path info from request, not urldecoded
    /// Returns Path info or empty string when not found
    pub fn get_raw_path_info(server_vars: &HashMap<String, String>) -> String {
        let request_uri = server_vars.get("REQUEST_URI").cloned().unwrap_or_default();
        let script_name = server_vars.get("SCRIPT_NAME").cloned().unwrap_or_default();
        
        if !request_uri.starts_with(&script_name) {
            return String::new();
        }
        
        let path_info = &request_uri[script_name.len()..];
        
        if let Some(pos) = path_info.find('?') {
            path_info[..pos].to_string()
        } else {
            path_info.to_string()
        }
    }

    /// Check if this is a no-cache request
    /// Returns true for no-cache
    pub fn is_no_cache(server_vars: &HashMap<String, String>) -> bool {
        server_vars.get("HTTP_CACHE_CONTROL") == Some(&"no-cache".to_string())
    }

    /// Check if the requestor understands gzip
    /// Returns gzip encoding type if supported, otherwise None
    pub fn accept_gzip(server_vars: &HashMap<String, String>) -> Option<String> {
        if let Some(encoding) = server_vars.get("HTTP_ACCEPT_ENCODING") {
            if encoding.contains("x-gzip") {
                return Some("x-gzip".to_string());
            } else if encoding.contains("gzip") {
                return Some("gzip".to_string());
            }
        }
        None
    }

    /// Check if the requester sent along an mtime
    /// Returns Option with the mtime if available
    pub fn has_modification_time(server_vars: &HashMap<String, String>) -> Option<String> {
        server_vars.get("HTTP_X_OC_MTIME").cloned()
    }
}

// Mock implementation of OcConfig to simulate the PHP OC_Config class
struct OcConfig {}

impl OcConfig {
    fn get_value(key: &str, default: &str) -> String {
        // This would actually fetch from configuration
        // Mock implementation returns default
        default.to_string()
    }
    
    fn get_value_bool(key: &str, default: bool) -> bool {
        // This would actually fetch from configuration
        // Mock implementation returns default
        default
    }
}

// Mock implementation of Oc to simulate the PHP OC class
struct Oc {
    pub cli: bool,
}