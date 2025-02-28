// config.rs

// Only enable this for local development and not in productive environments
// This will disable the minifier and outputs some additional debug informations
pub const DEBUG: bool = true;

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPath {
    pub path: String,
    pub url: String,
    pub writable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBackend {
    pub class: String,
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSslConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /* Flag to indicate ownCloud is successfully installed (true = installed) */
    pub installed: bool,

    /* Type of database, can be sqlite, mysql or pgsql */
    pub dbtype: String,

    /* Name of the ownCloud database */
    pub dbname: String,

    /* User to access the ownCloud database */
    pub dbuser: String,

    /* Password to access the ownCloud database */
    pub dbpassword: String,

    /* Host running the ownCloud database */
    pub dbhost: String,

    /* Prefix for the ownCloud tables in the database */
    pub dbtableprefix: String,

    /* Define the salt used to hash the user passwords. All your user passwords are lost if you lose this string. */
    pub passwordsalt: String,

    /* Force use of HTTPS connection (true = use HTTPS) */
    pub forcessl: bool,

    /* Blacklist a specific file and disallow the upload of files with this name - WARNING: USE THIS ONLY IF YOU KNOW WHAT YOU ARE DOING. */
    pub blacklisted_files: Vec<String>,

    /* The automatic hostname detection of ownCloud can fail in certain reverse proxy situations. This option allows to manually override the automatic detection. You can also add a port. For example "www.example.com:88" */
    pub overwritehost: String,

    /* The automatic protocol detection of ownCloud can fail in certain reverse proxy situations. This option allows to manually override the protocol detection. For example "https" */
    pub overwriteprotocol: String,

    /* The automatic webroot detection of ownCloud can fail in certain reverse proxy situations. This option allows to manually override the automatic detection. For example "/domain.tld/ownCloud" */
    pub overwritewebroot: String,

    /* The automatic detection of ownCloud can fail in certain reverse proxy situations. This option allows to define a manually override condition as regular expression for the remote ip address. For example "^10\.0\.0\.[1-3]$" */
    pub overwritecondaddr: String,

    /* A proxy to use to connect to the internet. For example "myproxy.org:88" */
    pub proxy: String,

    /* The optional authentication for the proxy to use to connect to the internet. The format is: [username]:[password] */
    pub proxyuserpwd: String,

    /* Theme to use for ownCloud */
    pub theme: String,

    /* Optional ownCloud default language - overrides automatic language detection on public pages like login or shared items. This has no effect on the user's language preference configured under "personal -> language" once they have logged in */
    pub default_language: String,

    /* Path to the parent directory of the 3rdparty directory */
    #[serde(rename = "3rdpartyroot")]
    pub third_party_root: String,

    /* URL to the parent directory of the 3rdparty directory, as seen by the browser */
    #[serde(rename = "3rdpartyurl")]
    pub third_party_url: String,

    /* Default app to load on login */
    pub defaultapp: String,

    /* Enable the help menu item in the settings */
    pub knowledgebaseenabled: bool,

    /* Enable installing apps from the appstore */
    pub appstoreenabled: bool,

    /* URL of the appstore to use, server should understand OCS */
    pub appstoreurl: String,

    /* Domain name used by ownCloud for the sender mail address, e.g. no-reply@example.com */
    pub mail_domain: String,

    /* Enable SMTP class debugging */
    pub mail_smtpdebug: bool,

    /* Mode to use for sending mail, can be sendmail, smtp, qmail or php, see PHPMailer docs */
    pub mail_smtpmode: String,

    /* Host to use for sending mail, depends on mail_smtpmode if this is used */
    pub mail_smtphost: String,

    /* Port to use for sending mail, depends on mail_smtpmode if this is used */
    pub mail_smtpport: u16,

    /* SMTP server timeout in seconds for sending mail, depends on mail_smtpmode if this is used */
    pub mail_smtptimeout: u32,

    /* SMTP connection prefix or sending mail, depends on mail_smtpmode if this is used.
       Can be '', ssl or tls */
    pub mail_smtpsecure: String,

    /* authentication needed to send mail, depends on mail_smtpmode if this is used
     * (false = disable authentication)
     */
    pub mail_smtpauth: bool,

    /* authentication type needed to send mail, depends on mail_smtpmode if this is used
     * Can be LOGIN (default), PLAIN or NTLM */
    pub mail_smtpauthtype: String,

    /* Username to use for sendmail mail, depends on mail_smtpauth if this is used */
    pub mail_smtpname: String,

    /* Password to use for sendmail mail, depends on mail_smtpauth if this is used */
    pub mail_smtppassword: String,

    /* How long should ownCloud keep deleted files in the trash bin, default value:  30 days */
    pub trashbin_retention_obligation: u32,

    /* allow user to change his display name, if it is supported by the back-end */
    pub allow_user_to_change_display_name: bool,

    /* Check 3rdparty apps for malicious code fragments */
    pub appcodechecker: String,

    /* Check if ownCloud is up to date */
    pub updatechecker: bool,

    /* Are we connected to the internet or are we running in a closed network? */
    pub has_internet_connection: bool,

    /* Check if the ownCloud WebDAV server is working correctly. Can be disabled if not needed in special situations*/
    pub check_for_working_webdav: bool,

    /* Check if .htaccess protection of data is working correctly. Can be disabled if not needed in special situations*/
    pub check_for_working_htaccess: bool,

    /* Place to log to, can be owncloud and syslog (owncloud is log menu item in admin menu) */
    pub log_type: String,

    /* File for the owncloud logger to log to, (default is ownloud.log in the data dir) */
    pub logfile: String,

    /* Loglevel to start logging at. 0=DEBUG, 1=INFO, 2=WARN, 3=ERROR (default is WARN) */
    pub loglevel: String,

    /* date format to be used while writing to the owncloud logfile */
    pub logdateformat: String,

    /* timezone used while writing to the owncloud logfile (default: UTC) */
    pub logtimezone: String,

    /* Append all database queries and parameters to the log file.
     (watch out, this option can increase the size of your log file)*/
    pub log_query: bool,

    /* Enable or disable the logging of IP addresses in case of webform auth failures */
    pub log_authfailip: bool,

    /*
     * Configure the size in bytes log rotation should happen, 0 or false disables the rotation.
     * This rotates the current owncloud logfile to a new name, this way the total log usage
     * will stay limited and older entries are available for a while longer. The
     * total disk usage is twice the configured size.
     * WARNING: When you use this, the log entries will eventually be lost.
     */
    pub log_rotate_size: Option<u64>,

    /* Lifetime of the remember login cookie, default is 15 days */
    pub remember_login_cookie_lifetime: u64,

    /* Life time of a session after inactivity */
    pub session_lifetime: u64,

    /* Custom CSP policy, changing this will overwrite the standard policy */
    pub custom_csp_policy: String,

    /* Enable/disable X-Frame-Restriction */
    /* HIGH SECURITY RISK IF DISABLED*/
    pub xframe_restriction: bool,

    /* The directory where the user data is stored, default to data in the owncloud
     * directory. The sqlite database is also stored here, when sqlite is used.
     */
    pub datadirectory: Option<String>,

    /* Enable maintenance mode to disable ownCloud
       If you want to prevent users to login to ownCloud before you start doing some maintenance work, 
       you need to set the value of the maintenance parameter to true. 
       Please keep in mind that users who are already logged-in are kicked out of ownCloud instantly.
    */
    pub maintenance: bool,

    pub apps_paths: Vec<AppPath>,
    
    pub user_backends: Vec<UserBackend>,
    
    //links to custom clients
    pub customclient_desktop: String, // http://owncloud.org/sync-clients/
    pub customclient_android: String, // https://play.google.com/store/apps/details?id=com.owncloud.android
    pub customclient_ios: String, // https://itunes.apple.com/us/app/owncloud/id543672169?mt=8

    // PREVIEW
    pub enable_previews: bool,
    /* the max width of a generated preview, if value is null, there is no limit */
    pub preview_max_x: Option<u32>,
    /* the max height of a generated preview, if value is null, there is no limit */
    pub preview_max_y: Option<u32>,
    /* the max factor to scale a preview, default is set to 10 */
    pub preview_max_scale_factor: u32,
    /* custom path for libreoffice / openoffice binary */
    pub preview_libreoffice_path: String,
    /* cl parameters for libreoffice / openoffice */
    pub preview_office_cl_parameters: String,

    /* whether avatars should be enabled */
    pub enable_avatars: bool,

    // Extra SSL options to be used for configuration
    pub openssl: OpenSslConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            installed: false,
            dbtype: "sqlite".to_string(),
            dbname: "owncloud".to_string(),
            dbuser: "".to_string(),
            dbpassword: "".to_string(),
            dbhost: "".to_string(),
            dbtableprefix: "".to_string(),
            passwordsalt: "".to_string(),
            forcessl: false,
            blacklisted_files: vec![".htaccess".to_string()],
            overwritehost: "".to_string(),
            overwriteprotocol: "".to_string(),
            overwritewebroot: "".to_string(),
            overwritecondaddr: "".to_string(),
            proxy: "".to_string(),
            proxyuserpwd: "".to_string(),
            theme: "".to_string(),
            default_language: "en".to_string(),
            third_party_root: "".to_string(),
            third_party_url: "".to_string(),
            defaultapp: "files".to_string(),
            knowledgebaseenabled: true,
            appstoreenabled: true,
            appstoreurl: "http://api.apps.owncloud.com/v1".to_string(),
            mail_domain: "example.com".to_string(),
            mail_smtpdebug: false,
            mail_smtpmode: "sendmail".to_string(),
            mail_smtphost: "127.0.0.1".to_string(),
            mail_smtpport: 25,
            mail_smtptimeout: 10,
            mail_smtpsecure: "".to_string(),
            mail_smtpauth: false,
            mail_smtpauthtype: "LOGIN".to_string(),
            mail_smtpname: "".to_string(),
            mail_smtppassword: "".to_string(),
            trashbin_retention_obligation: 30,
            allow_user_to_change_display_name: true,
            appcodechecker: "".to_string(),
            updatechecker: true,
            has_internet_connection: true,
            check_for_working_webdav: true,
            check_for_working_htaccess: true,
            log_type: "owncloud".to_string(),
            logfile: "".to_string(),
            loglevel: "".to_string(),
            logdateformat: "F d, Y H:i:s".to_string(),
            logtimezone: "Europe/Berlin".to_string(),
            log_query: false,
            log_authfailip: false,
            log_rotate_size: None,
            remember_login_cookie_lifetime: 60 * 60 * 24 * 15,
            session_lifetime: 60 * 60 * 24,
            custom_csp_policy: "default-src 'self'; script-src 'self' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; frame-src *; img-src *; font-src 'self' data:; media-src *".to_string(),
            xframe_restriction: true,
            datadirectory: None,
            maintenance: false,
            apps_paths: vec![
                AppPath {
                    path: "/var/www/owncloud/apps".to_string(),
                    url: "/apps".to_string(),
                    writable: true,
                },
            ],
            user_backends: vec![
                UserBackend {
                    class: "OC_User_IMAP".to_string(),
                    arguments: vec!["{imap.gmail.com:993/imap/ssl}INBOX".to_string()],
                },
            ],
            customclient_desktop: "".to_string(),
            customclient_android: "".to_string(),
            customclient_ios: "".to_string(),
            enable_previews: true,
            preview_max_x: None,
            preview_max_y: None,
            preview_max_scale_factor: 10,
            preview_libreoffice_path: "/usr/bin/libreoffice".to_string(),
            preview_office_cl_parameters: "".to_string(),
            enable_avatars: true,
            openssl: OpenSslConfig {
                config: None,
            },
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}