use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

// Equivalent to the global OC::$SERVERROOT
// In a real implementation this would come from an actual config or environment
fn get_server_root() -> PathBuf {
    // This is just a placeholder - in a real app this would be properly implemented
    PathBuf::from(env::var("SERVER_ROOT").unwrap_or_else(|_| ".".to_string()))
}

#[derive(Debug, Clone)]
pub struct AppsPath {
    pub path: PathBuf,
    pub url: String,
    pub writable: bool,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub appstoreenabled: bool,
    pub apps_paths: Vec<AppsPath>,
    pub openssl: Option<HashMap<String, String>>,
}

pub fn get_default_config() -> Config {
    let server_root = get_server_root();
    
    let mut config = Config {
        appstoreenabled: false,
        apps_paths: vec![
            AppsPath {
                path: server_root.join("apps"),
                url: "/apps".to_string(),
                writable: false,
            },
            AppsPath {
                path: server_root.join("apps2"),
                url: "/apps2".to_string(),
                writable: false,
            },
        ],
        openssl: None,
    };

    // Check if running on Windows
    if cfg!(windows) {
        let mut openssl_config = HashMap::new();
        openssl_config.insert(
            "config".to_string(),
            server_root.join("tests/data/openssl.cnf").to_string_lossy().to_string()
        );
        config.openssl = Some(openssl_config);
    }

    config
}