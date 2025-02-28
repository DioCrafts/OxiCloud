// Copyright (c) 2013 Thomas Tanghus <thomas@tanghus.net>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

// Public interface of ownCloud for apps to use.
// Tag manager interface

use crate::tags::ITags;
use std::error::Error;

/// Factory trait for creating instances of `ITags`
///
/// A tag can be e.g. 'Family', 'Work', 'Chore', 'Special Occation' or
/// anything else that is either parsed from a vobject or that the user chooses
/// to add.
/// Tag names are not case-sensitive, but will be saved with the case they
/// are entered in. If a user already has a tag 'family' for a type, and
/// tries to add a tag named 'Family' it will be silently ignored.
pub trait ITagManager {
    /// Create a new `ITags` instance and load tags from db.
    ///
    /// # Parameters
    ///
    /// * `type_name` - The type identifier e.g. 'contact' or 'event'.
    /// * `default_tags` - An array of default tags to be used if none are stored.
    ///
    /// # Returns
    ///
    /// A Result containing the ITags instance or an error
    fn load(&self, type_name: &str, default_tags: Vec<String>) -> Result<Box<dyn ITags>, Box<dyn Error>>;
}