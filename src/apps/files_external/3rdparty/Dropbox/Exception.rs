//! Dropbox base exception
//!
//! @package Dropbox
//! @copyright Copyright (C) 2010 Rooftop Solutions. All rights reserved.
//! @author Evert Pot (http://www.rooftopsolutions.nl/) 
//! @license http://code.google.com/p/dropbox-php/wiki/License MIT

use std::error::Error;
use std::fmt;

/// Base exception class
#[derive(Debug)]
pub struct DropboxError {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl DropboxError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source<S, E>(message: S, source: E) -> Self
    where
        S: Into<String>,
        E: Error + Send + Sync + 'static,
    {
        Self {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }
}

impl fmt::Display for DropboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DropboxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}