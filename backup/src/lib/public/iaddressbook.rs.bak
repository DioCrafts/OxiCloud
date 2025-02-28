// Copyright (C) 2012 Thomas Müller
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

use std::collections::HashMap;
use async_trait::async_trait;

/// Public interface for address book functionality.
/// Apps should use this interface instead of internal implementation details.
#[async_trait]
pub trait IAddressBook {
    /// Returns a string defining the technical unique key
    fn get_key(&self) -> String;

    /// In comparison to get_key() this function returns a human readable (maybe translated) name
    fn get_display_name(&self) -> String;

    /// Search for contacts matching the given pattern
    ///
    /// # Parameters
    /// * `pattern` - Pattern which should match within the search_properties
    /// * `search_properties` - Defines the properties within which the query pattern should match
    /// * `options` - For future use. One should always have options!
    ///
    /// # Returns
    /// Array of contacts which are maps of key-value-pairs
    ///
    /// # Example Return Value
    ///
    /// // [
    /// //     {"id": 0, "FN": "Thomas Müller", "EMAIL": "a@b.c", "GEO": "37.386013;-122.082932"},
    /// //     {"id": 5, "FN": "Thomas Tanghus", "EMAIL": ["d@e.f", "g@h.i"]},
    /// // ]
    ///
    async fn search(&self, pattern: &str, search_properties: Vec<String>, options: HashMap<String, String>) -> Vec<HashMap<String, serde_json::Value>>;

    /// Create or update a contact
    ///
    /// # Parameters
    /// * `properties` - This map of key-value-pairs defines a contact
    ///
    /// # Returns
    /// A map representing the contact just created or updated
    ///
    /// # Example Return Value
    ///
    /// // {
    /// //     "id": 0,
    /// //     "FN": "Thomas Müller",
    /// //     "EMAIL": "a@b.c",
    /// //     "PHOTO": "VALUE=uri:http://www.abc.com/pub/photos/jqpublic.gif",
    /// //     "ADR": ";;123 Main Street;Any Town;CA;91921-1234"
    /// // }
    ///
    async fn create_or_update(&self, properties: HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error + Send + Sync>>;

    /// Returns the permissions for this address book
    fn get_permissions(&self) -> u32;

    /// Delete a contact
    ///
    /// # Parameters
    /// * `id` - The unique identifier to a contact
    ///
    /// # Returns
    /// `true` if successful, `false` otherwise
    async fn delete(&self, id: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
}