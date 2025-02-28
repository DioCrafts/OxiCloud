// SPDX-FileCopyrightText: 2013 Robin Appelman <icewind@owncloud.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// OCP Files module for public API

use std::error::Error;
use std::fmt;

/// Error returned when a file or directory is not found
#[derive(Debug)]
pub struct NotFoundException {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl NotFoundException {
    /// Create a new NotFoundException
    pub fn new<S: Into<String>>(message: S) -> Self {
        NotFoundException {
            message: message.into(),
            source: None,
        }
    }

    /// Create a new NotFoundException with a source error
    pub fn with_source<S, E>(message: S, source: E) -> Self
    where
        S: Into<String>,
        E: Error + Send + Sync + 'static,
    {
        NotFoundException {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }
}

impl fmt::Display for NotFoundException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for NotFoundException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}