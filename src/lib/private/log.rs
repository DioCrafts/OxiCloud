//! Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use std::collections::HashMap;
use std::sync::Arc;

/// Logging utilities
///
/// This is a stand in, this should be replaced by a Psr\Log\LoggerInterface
/// compatible logger. See https://github.com/php-fig/fig-standards/blob/master/accepted/PSR-3-logger-interface.md
/// for the full interface specification.
///
/// MonoLog is an example implementing this interface.
pub struct Log {
    log_class: Arc<dyn LogBackend>,
}

/// Log level definitions
pub enum LogLevel {
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
}

/// Trait defining the backend logging functionality
pub trait LogBackend: Send + Sync {
    fn write(&self, app: &str, message: &str, level: LogLevel);
}

impl Log {
    /// Create a new logger instance
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = crate::config::Config::get_instance()?;
        let log_type = config.get_value("log_type").unwrap_or("owncloud".to_string());
        
        let log_class = match log_type.to_lowercase().as_str() {
            "file" => Arc::new(crate::log_backend::FileLogger::new()?) as Arc<dyn LogBackend>,
            "database" => Arc::new(crate::log_backend::DatabaseLogger::new()?) as Arc<dyn LogBackend>,
            "owncloud" => Arc::new(crate::log_backend::OwncloudLogger::new()?) as Arc<dyn LogBackend>,
            // Add other logger types as needed
            _ => return Err(format!("Unknown logger type: {}", log_type).into()),
        };
        
        Ok(Log { log_class })
    }

    /// System is unusable.
    pub fn emergency(&self, message: &str, context: HashMap<String, String>) {
        self.log(LogLevel::Fatal, message, context);
    }

    /// Action must be taken immediately.
    ///
    /// Example: Entire website down, database unavailable, etc. This should
    /// trigger the SMS alerts and wake you up.
    pub fn alert(&self, message: &str, context: HashMap<String, String>) {
        self.log(LogLevel::Error, message, context);
    }

    /// Critical conditions.
    ///
    /// Example: Application component unavailable, unexpected exception.
    pub fn critical(&self, message: &str, context: HashMap<String, String>) {
        self.log(LogLevel::Error, message, context);
    }

    /// Runtime errors that do not require immediate action but should typically
    /// be logged and monitored.
    pub fn error(&self, message: &str, context: HashMap<String, String>) {
        self.log(LogLevel::Error, message, context);
    }

    /// Exceptional occurrences that are not errors.
    ///
    /// Example: Use of deprecated APIs, poor use of an API, undesirable things
    /// that are not necessarily wrong.
    pub fn warning(&self, message: &str, context: HashMap<String, String>) {
        self.log(LogLevel::Warn, message, context);
    }

    /// Normal but significant events.
    pub fn notice(&self, message: &str, context: HashMap<String, String>) {
        self.log(LogLevel::Info, message, context);
    }

    /// Interesting events.
    ///
    /// Example: User logs in, SQL logs.
    pub fn info(&self, message: &str, context: HashMap<String, String>) {
        self.log(LogLevel::Info, message, context);
    }

    /// Detailed debug information.
    pub fn debug(&self, message: &str, context: HashMap<String, String>) {
        self.log(LogLevel::Debug, message, context);
    }

    /// Logs with an arbitrary level.
    pub fn log(&self, level: LogLevel, message: &str, context: HashMap<String, String>) {
        let app = context.get("app").map_or("no app in context", |s| s.as_str());
        self.log_class.write(app, message, level);
    }
}