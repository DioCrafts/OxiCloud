// Public interface of Nextcloud for apps to use.
// Session interface.
//
// This file is licensed under the Affero General Public License version 3 or
// later. See the LICENSE.md file.

/// Interface ISession
///
/// Wrap session handling into the ISession trait
pub trait ISession {
    /// Set a value in the session
    ///
    /// # Arguments
    ///
    /// * `key` - The key under which to store the value
    /// * `value` - The value to store
    fn set<T>(&mut self, key: &str, value: T) where T: serde::Serialize;

    /// Get a value from the session
    ///
    /// # Arguments
    ///
    /// * `key` - The key to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` if `key` exists, `None` otherwise
    fn get<T>(&self, key: &str) -> Option<T> where T: serde::de::DeserializeOwned;

    /// Check if a named key exists in the session
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check
    ///
    /// # Returns
    ///
    /// Returns `true` if the key exists, `false` otherwise
    fn exists(&self, key: &str) -> bool;

    /// Remove a key/value pair from the session
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove
    fn remove(&mut self, key: &str);

    /// Reset and recreate the session
    fn clear(&mut self);
}