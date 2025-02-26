//! ownCloud
//!
//! @author Thomas Müller
//! @copyright 2013 Thomas Müller deepdiver@owncloud.com
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::error::Error;
use std::fmt;

/// Error type representing an invalid path in the file system
#[derive(Debug, Clone)]
pub struct InvalidPathException {
    message: String,
}

impl InvalidPathException {
    /// Create a new InvalidPathException with the given message
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for InvalidPathException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid path: {}", self.message)
    }
}

impl Error for InvalidPathException {}