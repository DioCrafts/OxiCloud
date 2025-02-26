// Copyright (C) 2013 Thomas Müller <thomas.mueller@tmit.eu>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//! Public interface of ownCloud for apps to use.
//! Contacts Module

use std::collections::HashMap;
use std::sync::Arc;

/// Address Book interface for ownCloud contacts system
pub trait IAddressBook: Send + Sync {}

/// This trait provides access to the contacts app. Use this trait exclusively if you want to access contacts.
///
/// Contacts in general will be expressed as a HashMap of key-value-pairs.
/// The keys will match the property names defined in https://tools.ietf.org/html/rfc2426#section-1
///
/// Proposed workflow for working with contacts:
///  - search for the contacts
///  - manipulate the results array
///  - create_or_update will save the given contacts overwriting the existing data
///
/// For updating it is mandatory to keep the id.
/// Without an id a new contact will be created.
pub trait IManager: Send + Sync {
    /// This function is used to search and find contacts within the users address books.
    /// In case `pattern` is empty all contacts will be returned.
    ///
    /// Example:
    ///  Following function shows how to search for contacts for the name and the email address.
    ///
    ///
    /// pub fn get_matching_recipient(term: &str) -> Vec<HashMap<String, String>> {
    ///     let cm = server.get_contacts_manager();
    ///     // The API is not active -> nothing to do
    ///     if !cm.is_enabled() {
    ///         return vec![];
    ///     }
    ///
    ///     let result = cm.search(term, &["FN".to_string(), "EMAIL".to_string()], &HashMap::new());
    ///     let mut receivers = Vec::new();
    ///     
    ///     for r in result {
    ///         let id = r.get("id").cloned().unwrap_or_default();
    ///         let fn_name = r.get("FN").cloned().unwrap_or_default();
    ///         let email = r.get("EMAIL").cloned();
    ///         
    ///         if let Some(email) = email {
    ///             let emails = if email.contains(',') {
    ///                 email.split(',').map(|s| s.to_string()).collect::<Vec<_>>()
    ///             } else {
    ///                 vec![email]
    ///             };
    ///
    ///             // loop through all email addresses of this contact
    ///             for e in emails {
    ///                 let display_name = format!("{} <{}>", fn_name, e);
    ///                 let mut entry = HashMap::new();
    ///                 entry.insert("id".to_string(), id.clone());
    ///                 entry.insert("label".to_string(), display_name.clone());
    ///                 entry.insert("value".to_string(), display_name);
    ///                 receivers.push(entry);
    ///             }
    ///         }
    ///     }
    ///
    ///     receivers
    /// }
    ///
    ///
    /// # Parameters
    /// * `pattern` - which should match within the `search_properties`
    /// * `search_properties` - defines the properties within the query pattern should match
    /// * `options` - for future use. One should always have options!
    ///
    /// # Returns
    /// * Vector of contacts which are hashmaps of key-value-pairs
    fn search(&self, pattern: &str, search_properties: &[String], options: &HashMap<String, String>) -> Vec<HashMap<String, String>>;

    /// This function can be used to delete the contact identified by the given id
    ///
    /// # Parameters
    /// * `id` - the unique identifier to a contact
    /// * `address_book_key` - the address book identifier
    ///
    /// # Returns
    /// * `true` if successful, `false` otherwise
    fn delete(&self, id: &str, address_book_key: &str) -> bool;

    /// This function is used to create a new contact if 'id' is not given or not present.
    /// Otherwise the contact will be updated by replacing the entire data set.
    ///
    /// # Parameters
    /// * `properties` - this hashmap of key-value-pairs defines a contact
    /// * `address_book_key` - string to identify the address book in which the contact shall be created or updated
    ///
    /// # Returns
    /// * HashMap representing the contact just created or updated
    fn create_or_update(&self, properties: HashMap<String, String>, address_book_key: &str) -> HashMap<String, String>;

    /// Check if contacts are available (e.g. contacts app enabled)
    ///
    /// # Returns
    /// * `true` if enabled, `false` if not
    fn is_enabled(&self) -> bool;

    /// Register an address book implementation
    ///
    /// # Parameters
    /// * `address_book` - the address book to register
    fn register_address_book(&self, address_book: Arc<dyn IAddressBook>);

    /// Unregister an address book implementation
    ///
    /// # Parameters
    /// * `address_book` - the address book to unregister
    fn unregister_address_book(&self, address_book: Arc<dyn IAddressBook>);

    /// In order to improve lazy loading a closure can be registered which will be called in case
    /// address books are actually requested
    ///
    /// # Parameters
    /// * `key` - identifier for the registration
    /// * `callable` - the closure to be called
    fn register<F>(&self, key: &str, callable: F)
    where
        F: Fn() + Send + Sync + 'static;

    /// Get all registered address books
    ///
    /// # Returns
    /// * Vector of address books
    fn get_address_books(&self) -> Vec<Arc<dyn IAddressBook>>;

    /// Removes all registered address book instances
    fn clear(&self);
}