// Security exception handling for the app framework middleware.
//
// This module contains the `SecurityException` error type used by the
// security middleware to communicate security-related issues.

use std::error::Error;
use std::fmt;

/// Thrown when the security middleware encounters a security problem
#[derive(Debug)]
pub struct SecurityException {
    /// The error message
    message: String,
    /// The error code
    code: i32,
}

impl SecurityException {
    /// Creates a new SecurityException
    ///
    /// # Arguments
    ///
    /// * `message` - The security error message
    /// * `code` - The error code, defaults to 0
    pub fn new<S: Into<String>>(message: S, code: i32) -> Self {
        SecurityException {
            message: message.into(),
            code,
        }
    }
}

impl fmt::Display for SecurityException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SecurityException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}