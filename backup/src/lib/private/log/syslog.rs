use libc::{LOG_CONS, LOG_CRIT, LOG_DEBUG, LOG_ERR, LOG_INFO, LOG_PID, LOG_USER, LOG_WARNING};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Once;

/// Syslog implementation for the logging subsystem
pub struct SyslogLogger;

/// Log levels mapping between our internal levels and syslog levels
static LEVELS: Lazy<HashMap<u8, i32>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(log::DEBUG, LOG_DEBUG);
    map.insert(log::INFO, LOG_INFO);
    map.insert(log::WARN, LOG_WARNING);
    map.insert(log::ERROR, LOG_ERR);
    map.insert(log::FATAL, LOG_CRIT);
    map
});

// Used for one-time initialization
static INIT: Once = Once::new();

impl SyslogLogger {
    /// Init class data
    pub fn init() {
        INIT.call_once(|| {
            // Safe because we're calling standard C functions with valid parameters
            unsafe {
                libc::openlog(
                    std::ffi::CString::new("ownCloud").unwrap().as_ptr(),
                    LOG_PID | LOG_CONS,
                    LOG_USER,
                );
            }

            // Register closelog to be called at program exit
            let _ = ctrlc::set_handler(|| {
                unsafe {
                    libc::closelog();
                }
                std::process::exit(0);
            });
        });
    }

    /// Write a message to the log
    ///
    /// # Arguments
    ///
    /// * `app` - The app/component name
    /// * `message` - The message to log
    /// * `level` - The log level
    pub fn write(app: &str, message: &str, level: u8) {
        let min_level = std::cmp::min(
            config::get_value("loglevel").unwrap_or(log::WARN),
            log::ERROR,
        );
        
        if level >= min_level {
            if let Some(&syslog_level) = LEVELS.get(&level) {
                let formatted_message = format!("[{}] {}", app, message);
                
                // Safe because we're calling standard C functions with valid parameters
                unsafe {
                    libc::syslog(
                        syslog_level,
                        std::ffi::CString::new("%s")
                            .unwrap()
                            .as_ptr(),
                        std::ffi::CString::new(formatted_message)
                            .unwrap()
                            .as_ptr(),
                    );
                }
            }
        }
    }
}

// Forward declaration of external dependencies that would be defined elsewhere
mod log {
    pub const DEBUG: u8 = 0;
    pub const INFO: u8 = 1;
    pub const WARN: u8 = 2;
    pub const ERROR: u8 = 3;
    pub const FATAL: u8 = 4;
}

mod config {
    pub fn get_value(key: &str) -> Option<u8> {
        // Placeholder for actual implementation
        match key {
            "loglevel" => Some(crate::log::WARN),
            _ => None,
        }
    }
}