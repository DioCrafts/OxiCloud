use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FtpConfig {
    pub run: bool,
    pub host: String,
    pub user: String,
    pub password: String,
    pub root: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WebdavConfig {
    pub run: bool,
    pub host: String,
    pub user: String,
    pub password: String,
    pub root: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GoogleConfig {
    pub run: bool,
    pub configured: String,
    pub client_id: String,
    pub client_secret: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SwiftConfig {
    pub run: bool,
    pub user: String,
    pub bucket: String,
    pub region: String,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SmbConfig {
    pub run: bool,
    pub user: String,
    pub password: String,
    pub host: String,
    pub share: String,
    pub root: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AmazonS3Config {
    pub run: bool,
    pub key: String,
    pub secret: String,
    pub bucket: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ssl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropboxConfig {
    pub run: bool,
    pub root: String,
    pub configured: String,
    pub app_key: String,
    pub app_secret: String,
    pub token: String,
    pub token_secret: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SftpConfig {
    pub run: bool,
    pub host: String,
    pub user: String,
    pub password: String,
    pub root: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub ftp: FtpConfig,
    pub webdav: WebdavConfig,
    pub google: GoogleConfig,
    pub swift: SwiftConfig,
    pub smb: SmbConfig,
    pub amazons3: AmazonS3Config,
    pub dropbox: DropboxConfig,
    pub sftp: SftpConfig,
}

pub fn load_config() -> Config {
    // in case there are private configurations in the users home -> use them
    if let Ok(home_dir) = env::var("HOME") {
        let private_config_file = Path::new(&home_dir).join("owncloud-extfs-test-config.json");
        if private_config_file.exists() {
            if let Ok(config_str) = fs::read_to_string(private_config_file) {
                if let Ok(config) = serde_json::from_str::<Config>(&config_str) {
                    return config;
                }
            }
        }
    }

    // this is now more a template for private configurations
    Config {
        ftp: FtpConfig {
            run: false,
            host: "localhost".to_string(),
            user: "test".to_string(),
            password: "test".to_string(),
            root: "/test".to_string(),
        },
        webdav: WebdavConfig {
            run: false,
            host: "localhost".to_string(),
            user: "test".to_string(),
            password: "test".to_string(),
            root: "/owncloud/files/webdav.php".to_string(),
        },
        google: GoogleConfig {
            run: false,
            configured: "true".to_string(),
            client_id: "".to_string(),
            client_secret: "".to_string(),
            token: "".to_string(),
        },
        swift: SwiftConfig {
            run: false,
            user: "test".to_string(),
            bucket: "test".to_string(),
            region: "DFW".to_string(),
            key: "test".to_string(), // to be used only with Rackspace Cloud Files
            tenant: None, // to be used only with OpenStack Object Storage
            password: None, // to be use only with OpenStack Object Storage
            service_name: None, // should be 'swift' for OpenStack Object Storage and 'cloudFiles' for Rackspace Cloud Files (default value)
            url: None, // to be used with Rackspace Cloud Files and OpenStack Object Storage
            timeout: None, // timeout of HTTP requests in seconds
        },
        smb: SmbConfig {
            run: false,
            user: "test".to_string(),
            password: "test".to_string(),
            host: "localhost".to_string(),
            share: "/test".to_string(),
            root: "/test/".to_string(),
        },
        amazons3: AmazonS3Config {
            run: false,
            key: "test".to_string(),
            secret: "test".to_string(),
            bucket: "bucket".to_string(),
            hostname: None,
            port: None,
            use_ssl: None,
            region: None,
            test: None,
            timeout: None,
        },
        dropbox: DropboxConfig {
            run: false,
            root: "owncloud".to_string(),
            configured: "true".to_string(),
            app_key: "".to_string(),
            app_secret: "".to_string(),
            token: "".to_string(),
            token_secret: "".to_string(),
        },
        sftp: SftpConfig {
            run: false,
            host: "localhost".to_string(),
            user: "test".to_string(),
            password: "test".to_string(),
            root: "/test".to_string(),
        },
    }
}