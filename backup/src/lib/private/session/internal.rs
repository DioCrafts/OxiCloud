// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during session operations
#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Failed to start session")]
    StartFailure,
    
    #[error("Session error: {0}")]
    Other(String),
}

/// Session trait defining the interface for all session implementations
pub trait Session {
    fn get(&self, key: &str) -> Option<String>;
    fn get_all(&self) -> HashMap<String, String>;
    fn set(&mut self, key: &str, value: String);
    fn remove(&mut self, key: &str);
    fn clear(&mut self);
    fn close(&mut self);
}

/// Memory-based session implementation that stores data in a HashMap
pub struct Memory {
    data: HashMap<String, String>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Session for Memory {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn get_all(&self) -> HashMap<String, String> {
        self.data.clone()
    }

    fn set(&mut self, key: &str, value: String) {
        self.data.insert(key.to_string(), value);
    }

    fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn close(&mut self) {
        // Memory implementation does nothing on close
    }
}

/**
 * Internal session implementation
 *
 * Wraps PHP's internal session handling into the Session interface.
 * In Rust, we use a native session management library.
 */
pub struct Internal {
    inner: Memory,
    session_name: String,
    session_handler: PhpSessionHandler,
}

// This would be an actual PHP session handling integration
struct PhpSessionHandler {}

impl PhpSessionHandler {
    fn new(name: &str) -> Result<Self, SessionError> {
        // In a real implementation, this would initialize the PHP session
        // For this example, we're just creating a stub
        Ok(Self {})
    }

    fn load_session_data(&self) -> Result<HashMap<String, String>, SessionError> {
        // In a real implementation, this would fetch data from $_SESSION
        Ok(HashMap::new())
    }

    fn save_session_data(&self, data: &HashMap<String, String>) -> Result<(), SessionError> {
        // In a real implementation, this would save to $_SESSION
        Ok(())
    }

    fn close_session(&self) -> Result<(), SessionError> {
        // Would call session_write_close()
        Ok(())
    }

    fn regenerate_id(&self) -> Result<(), SessionError> {
        // Would call session_regenerate_id(true)
        Ok(())
    }

    fn unset_session(&self) -> Result<(), SessionError> {
        // Would call session_unset()
        Ok(())
    }
}

impl Internal {
    pub fn new(name: &str) -> Result<Self, SessionError> {
        let session_handler = PhpSessionHandler::new(name)?;
        let session_data = session_handler.load_session_data()?;

        let mut memory = Memory::new();
        for (key, value) in session_data {
            memory.set(&key, value);
        }

        Ok(Self {
            inner: memory,
            session_name: name.to_string(),
            session_handler,
        })
    }
}

impl Session for Internal {
    fn get(&self, key: &str) -> Option<String> {
        self.inner.get(key)
    }

    fn get_all(&self) -> HashMap<String, String> {
        self.inner.get_all()
    }

    fn set(&mut self, key: &str, value: String) {
        self.inner.set(key, value);
    }

    fn remove(&mut self, key: &str) {
        self.inner.remove(key);
    }

    fn clear(&mut self) {
        // This matches the PHP implementation
        if let Err(e) = self.session_handler.unset_session() {
            eprintln!("Warning: Failed to unset session: {}", e);
        }
        
        if let Err(e) = self.session_handler.regenerate_id() {
            eprintln!("Warning: Failed to regenerate session ID: {}", e);
        }
        
        // We're ignoring errors in regenerate_id, just like the @ operator in PHP
        self.inner.clear();
    }

    fn close(&mut self) {
        if let Err(e) = self.session_handler.save_session_data(&self.inner.data) {
            eprintln!("Warning: Failed to save session data: {}", e);
        }
        
        if let Err(e) = self.session_handler.close_session() {
            eprintln!("Warning: Failed to close session: {}", e);
        }
    }
}

impl Drop for Internal {
    fn drop(&mut self) {
        self.close();
    }
}