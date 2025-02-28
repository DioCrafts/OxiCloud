// ownCloud
//
// @author Thomas Müller
// @copyright 2013 Thomas Müller <thomas.mueller@tmit.eu>
//
// @license AGPL3

use crate::sabre::dav::exception::Exception;
use std::error::Error as StdError;
use std::fmt;

/// Represents a service unavailable exception in the DAV protocol
#[derive(Debug)]
pub struct ServiceUnavailable {
    message: String,
    code: Option<i32>,
}

impl ServiceUnavailable {
    /// Create a new ServiceUnavailable exception
    pub fn new(message: impl Into<String>, code: Option<i32>) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }

    /// Returns the HTTP status code for this exception
    ///
    /// @return i32
    pub fn get_http_code(&self) -> i32 {
        503
    }
}

impl fmt::Display for ServiceUnavailable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for ServiceUnavailable {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl Exception for ServiceUnavailable {
    fn get_message(&self) -> &str {
        &self.message
    }

    fn get_code(&self) -> Option<i32> {
        self.code
    }
}