// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::error::Error;
use std::fmt;

/// An exception that provides a hint to the user about how to resolve the error.
#[derive(Debug)]
pub struct HintException {
    message: String,
    hint: String,
    code: i32,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl HintException {
    pub fn new<S1, S2>(
        message: S1, 
        hint: S2, 
        code: i32, 
        source: Option<Box<dyn Error + Send + Sync>>
    ) -> Self 
    where 
        S1: Into<String>,
        S2: Into<String>, 
    {
        HintException {
            message: message.into(),
            hint: hint.into(),
            code,
            source,
        }
    }

    pub fn hint(&self) -> &str {
        &self.hint
    }
}

impl fmt::Display for HintException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HintException: [{}]: {} ({})", self.code, self.message, self.hint)
    }
}

impl Error for HintException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}