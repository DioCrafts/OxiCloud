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

// crate for all classes that are considered public.
// This means that they should be used by apps instead of the internal ownCloud classes
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// An exception that is thrown when file content is invalid
#[derive(Debug)]
pub struct InvalidContentException {
    message: String,
}

impl InvalidContentException {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for InvalidContentException {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InvalidContentException {}