// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::panic::{self, PanicInfo};
use std::sync::{Arc, Mutex, Once};
use log::{warn, critical};
use std::sync::atomic::{AtomicBool, Ordering};

/// Interface for logger implementations
pub trait LoggerInterface: Send + Sync {
    fn critical(&self, msg: &str, app: &str);
    fn warning(&self, msg: &str, app: &str);
}

pub struct ErrorHandler {
    logger: Option<Arc<Mutex<dyn LoggerInterface>>>
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler {
            logger: None
        }
    }

    /// Register error handlers
    pub fn register() {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            // Set custom panic hook
            panic::set_hook(Box::new(|panic_info| {
                if let Some(handler) = INSTANCE.lock().unwrap().as_ref() {
                    handler.on_panic(panic_info);
                }
            }));
            
            // Register shutdown handler
            let already_handling_panic = Arc::new(AtomicBool::new(false));
            let already_handling_panic_clone = already_handling_panic.clone();
            
            // This ensures we don't double-handle panics
            std::panic::set_hook(Box::new(move |panic_info| {
                if !already_handling_panic_clone.swap(true, Ordering::SeqCst) {
                    if let Some(handler) = INSTANCE.lock().unwrap().as_ref() {
                        handler.on_panic(panic_info);
                    }
                }
            }));
        });
    }

    /// Set the logger instance
    pub fn set_logger(logger: Arc<Mutex<dyn LoggerInterface>>) {
        if let Some(handler) = INSTANCE.lock().unwrap().as_mut() {
            handler.logger = Some(logger);
        }
    }

    /// Handle panics
    fn on_panic(&self, panic_info: &PanicInfo) {
        if let Some(logger) = &self.logger {
            let location = panic_info.location()
                .map(|loc| format!(" at {}#{}", loc.file(), loc.line()))
                .unwrap_or_else(|| String::from(""));
            
            let message = match panic_info.payload().downcast_ref::<&str>() {
                Some(s) => *s,
                None => match panic_info.payload().downcast_ref::<String>() {
                    Some(s) => s.as_str(),
                    None => "Unknown panic",
                },
            };
            
            let msg = format!("{}{}", message, location);
            
            let mut logger = logger.lock().unwrap();
            logger.critical(&msg, "RUST");
        }
    }

    /// Process errors
    pub fn on_error(&self, message: &str, file: &str, line: u32) {
        if let Some(logger) = &self.logger {
            let msg = format!("{} at {}#{}", message, file, line);
            let mut logger = logger.lock().unwrap();
            logger.warning(&msg, "RUST");
        }
    }
}

// Global singleton instance
lazy_static::lazy_static! {
    static ref INSTANCE: Mutex<Option<ErrorHandler>> = Mutex::new(Some(ErrorHandler::new()));
}

// Public functions for error handling
pub fn register() {
    ErrorHandler::register();
}

pub fn set_logger(logger: Arc<Mutex<dyn LoggerInterface>>) {
    ErrorHandler::set_logger(logger);
}

pub fn on_error(message: &str, file: &str, line: u32) {
    if let Some(handler) = INSTANCE.lock().unwrap().as_ref() {
        handler.on_error(message, file, line);
    }
}