// Copyright 2013 Thomas Tanghus thomas@tanghus.net
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

//! Public interface of ownCloud for apps to use.
//! Cache interface

use std::time::Duration;

/// This trait defines methods for accessing the file based user cache.
pub trait Cache {
    /// The type of values stored in the cache
    type Value;
    /// The error type returned by cache operations
    type Error;

    /// Get a value from the user cache
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key to retrieve
    ///
    /// # Returns
    ///
    /// * `Ok(Some(value))` - The cached value if found
    /// * `Ok(None)` - If the key doesn't exist
    /// * `Err(error)` - If an error occurred
    fn get(&self, key: &str) -> Result<Option<Self::Value>, Self::Error>;

    /// Set a value in the user cache
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key
    /// * `value` - The value to store
    /// * `ttl` - Time To Live duration. Defaults to 24 hours if None
    ///
    /// # Returns
    ///
    /// * `Ok(())` - On success
    /// * `Err(error)` - If an error occurred
    fn set(&self, key: &str, value: Self::Value, ttl: Option<Duration>) -> Result<(), Self::Error>;

    /// Check if a value is set in the user cache
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key to check
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If the key exists
    /// * `Ok(false)` - If the key doesn't exist
    /// * `Err(error)` - If an error occurred
    fn has_key(&self, key: &str) -> Result<bool, Self::Error>;

    /// Remove an item from the user cache
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key to remove
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If the key was removed
    /// * `Ok(false)` - If the key didn't exist
    /// * `Err(error)` - If an error occurred
    fn remove(&self, key: &str) -> Result<bool, Self::Error>;

    /// Clear the user cache of all entries starting with a prefix
    ///
    /// # Arguments
    ///
    /// * `prefix` - Optional prefix. If None or empty, clears the entire cache
    ///
    /// # Returns
    ///
    /// * `Ok(())` - On success
    /// * `Err(error)` - If an error occurred
    fn clear(&self, prefix: Option<&str>) -> Result<(), Self::Error>;
}