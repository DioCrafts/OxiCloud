//! ownCloud
//!
//! @author Robin Appelman
//! @copyright 2013 Robin Appelman icewind@owncloud.com
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

/// Represents an error when an operation is not permitted.
///
/// Part of the public API for apps to use.
#[derive(Debug)]
pub struct NotPermittedException {
    message: String,
}

impl NotPermittedException {
    /// Creates a new `NotPermittedException` with the given message.
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for NotPermittedException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Operation not permitted: {}", self.message)
    }
}

impl Error for NotPermittedException {}