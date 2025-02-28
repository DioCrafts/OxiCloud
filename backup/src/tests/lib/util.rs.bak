use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use html_escape::encode_safe;
use rand::{thread_rng, RngCore};
use std::path::Path;
use url::percent_encoding::{percent_encode, NON_ALPHANUMERIC};

/**
 * Copyright (c) 2012 Lukas Reschke <lukas@statuscode.ch>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

#[cfg(test)]
mod test_util {
    use super::*;
    use crate::config;
    use crate::util;

    #[test]
    fn test_get_version() {
        let version = util::get_version();
        assert!(version.is_array());
        for num in version.iter() {
            assert!(num.is_integer());
        }
    }

    #[test]
    fn test_get_version_string() {
        let version = util::get_version_string();
        assert!(version.is_string());
    }

    #[test]
    fn test_get_edition_string() {
        let edition = util::get_edition_string();
        assert!(edition.is_string());
    }

    #[test]
    fn test_format_date() {
        // Set timezone to UTC for test
        let timestamp = 1350129205;
        let result = util::format_date(timestamp, false);
        let expected = "October 13, 2012 11:53";
        assert_eq!(expected, result);

        let timestamp = 1102831200;
        let result = util::format_date(timestamp, true);
        let expected = "December 12, 2004";
        assert_eq!(expected, result);
    }

    #[test]
    fn test_call_register() {
        let result = util::call_register();
        assert_eq!(20, result.len());
    }

    #[test]
    fn test_sanitize_html() {
        let bad_string = "<script>alert('Hacked!');</script>";
        let result = util::sanitize_html(bad_string);
        assert_eq!("&lt;script&gt;alert(&#39;Hacked!&#39;);&lt;/script&gt;", result);

        let good_string = "This is an harmless string.";
        let result = util::sanitize_html(good_string);
        assert_eq!("This is an harmless string.", result);
    }
    
    #[test]
    fn test_encode_path() {
        let component = "/§#@test%&^ä/-child";
        let result = util::encode_path(component);
        assert_eq!("/%C2%A7%23%40test%25%26%5E%C3%A4/-child", result);
    }

    #[test]
    fn test_file_info_loaded() {
        // In Rust, we'd implement appropriate logic to check for file info capability
        let expected = cfg!(feature = "file_info");
        assert_eq!(expected, util::file_info_loaded());
    }

    #[test]
    fn test_is_internet_connection_enabled() {
        config::set_value("has_internet_connection", &false);
        assert_eq!(false, util::is_internet_connection_enabled());

        config::set_value("has_internet_connection", &true);
        assert_eq!(true, util::is_internet_connection_enabled());
    }

    #[test]
    fn test_generate_random_bytes() {
        let result = util::generate_random_bytes(59);
        assert_eq!(59, result.len());
    }

    #[test]
    fn test_get_default_email_address() {
        let email = util::get_default_email_address("no-reply");
        assert_eq!("no-reply@localhost.localdomain", email);
    }

    #[test]
    fn test_get_default_email_address_from_config() {
        config::set_value("mail_domain", &"example.com".to_string());
        let email = util::get_default_email_address("no-reply");
        assert_eq!("no-reply@example.com", email);
        config::delete_key("mail_domain");
    }

    #[test]
    fn test_get_instance_id_generates_valid_id() {
        config::delete_key("instanceid");
        let instance_id = util::get_instance_id();
        assert!(instance_id.starts_with("oc"));
    }

    #[test]
    fn test_base_name() {
        let test_cases = vec![
            ("/home/user/public_html/", "public_html"),
            ("/home/user/public_html", "public_html"),
            ("/", ""),
            ("public_html", "public_html"),
            ("local::C:\\Users\\ADMINI~1\\AppData\\Local\\Temp\\2/442aa682de2a64db1e010f50e60fd9c9/", "442aa682de2a64db1e010f50e60fd9c9"),
        ];

        for (input, expected) in test_cases {
            let base = util::basename(input);
            assert_eq!(expected, base);
        }
    }
}

// Implementaciones necesarias para que los tests funcionen
pub mod util {
    use super::*;
    use crate::config;
    use std::path::Path;
    use uuid::Uuid;

    pub fn get_version() -> Vec<i32> {
        // Sample implementation
        vec![10, 0, 0]
    }

    pub fn get_version_string() -> String {
        "10.0.0".to_string()
    }

    pub fn get_edition_string() -> String {
        "Community".to_string()
    }

    pub fn format_date(timestamp: i64, date_only: bool) -> String {
        let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0)
            .expect("Invalid timestamp");
        let datetime: DateTime<Utc> = Utc.from_utc_datetime(&naive);
        
        if date_only {
            datetime.format("%B %d, %Y").to_string()
        } else {
            datetime.format("%B %d, %Y %H:%M").to_string()
        }
    }

    pub fn call_register() -> String {
        generate_random_bytes(20)
    }

    pub fn sanitize_html(input: &str) -> String {
        encode_safe(input).to_string()
    }

    pub fn encode_path(path: &str) -> String {
        percent_encode(path.as_bytes(), NON_ALPHANUMERIC).to_string()
    }

    pub fn file_info_loaded() -> bool {
        // Rust implementation would depend on specific file info libraries
        cfg!(feature = "file_info")
    }

    pub fn is_internet_connection_enabled() -> bool {
        match config::get_value::<bool>("has_internet_connection") {
            Some(enabled) => enabled,
            None => false
        }
    }

    pub fn generate_random_bytes(length: usize) -> String {
        let mut rng = thread_rng();
        let mut buffer = vec![0u8; length];
        rng.fill_bytes(&mut buffer);
        
        // Convert to hex string for consistent length
        buffer.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
            .chars()
            .take(length)
            .collect()
    }

    pub fn get_default_email_address(user_part: &str) -> String {
        let domain = match config::get_value::<String>("mail_domain") {
            Some(domain) => domain,
            None => "localhost.localdomain".to_string()
        };
        
        format!("{}@{}", user_part, domain)
    }

    pub fn get_instance_id() -> String {
        if let Some(id) = config::get_value::<String>("instanceid") {
            return id;
        }
        
        let new_id = format!("oc{}", Uuid::new_v4().to_simple());
        config::set_value("instanceid", &new_id);
        new_id
    }

    pub fn basename(path: &str) -> String {
        if path == "/" {
            return "".to_string();
        }
        
        // Handle Windows UNC paths
        let clean_path = if path.contains("::") {
            let parts: Vec<&str> = path.split("::").collect();
            parts[1]
        } else {
            path
        };
        
        let path_obj = Path::new(clean_path);
        let filename = path_obj.file_name().map(|f| f.to_string_lossy().to_string());
        
        if let Some(name) = filename {
            return name;
        }
        
        // Handle trailing slash
        let parent_path = clean_path.trim_end_matches('/');
        let parent_obj = Path::new(parent_path);
        
        parent_obj.file_name().map_or_else(
            || "".to_string(),
            |name| name.to_string_lossy().to_string()
        )
    }
}

// Módulo simulado para las configuraciones
pub mod config {
    use std::collections::HashMap;
    use std::sync::Mutex;
    use once_cell::sync::Lazy;
    
    static CONFIG: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
        Mutex::new(HashMap::new())
    });
    
    pub fn set_value<T: ToString>(key: &str, value: &T) {
        let mut config = CONFIG.lock().unwrap();
        config.insert(key.to_string(), value.to_string());
    }
    
    pub fn get_value<T: std::str::FromStr>(key: &str) -> Option<T> {
        let config = CONFIG.lock().unwrap();
        config.get(key)
            .and_then(|val| val.parse::<T>().ok())
    }
    
    pub fn delete_key(key: &str) {
        let mut config = CONFIG.lock().unwrap();
        config.remove(key);
    }
}