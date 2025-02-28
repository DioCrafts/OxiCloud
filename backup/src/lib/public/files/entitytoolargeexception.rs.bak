/*!
 * ownCloud
 *
 * @author Thomas Müller
 * @copyright 2013 Thomas Müller deepdiver@owncloud.com
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use std::error::Error;
use std::fmt;

/// Custom error type for when an entity is too large
#[derive(Debug)]
pub struct EntityTooLargeException {
    message: String,
}

impl EntityTooLargeException {
    pub fn new<S: Into<String>>(message: S) -> Self {
        EntityTooLargeException {
            message: message.into(),
        }
    }
}

impl fmt::Display for EntityTooLargeException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity too large: {}", self.message)
    }
}

impl Error for EntityTooLargeException {}