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
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::sync::Arc;

pub mod ocp {
    pub mod contacts {
        use std::collections::HashMap;
        
        pub trait IManager {
            fn search(&self, pattern: &str, search_properties: &[&str], options: &[(&str, &str)]) -> Vec<HashMap<String, String>>;
            fn delete(&self, id: &str, address_book_key: &str) -> Option<bool>;
            fn create_or_update(&self, properties: HashMap<String, String>, address_book_key: &str) -> Option<HashMap<String, String>>;
            fn is_enabled(&self) -> bool;
            fn register_address_book(&mut self, address_book: Arc<dyn IAddressBook>);
            fn unregister_address_book(&mut self, address_book: Arc<dyn IAddressBook>);
            fn get_address_books(&self) -> HashMap<String, String>;
            fn clear(&mut self);
            fn register(&mut self, key: &str, callable: Box<dyn Fn() + Send + Sync>);
        }

        pub trait IAddressBook: Send + Sync {
            fn get_key(&self) -> String;
            fn get_display_name(&self) -> String;
            fn get_permissions(&self) -> u32;
            fn search(&self, pattern: &str, search_properties: &[&str], options: &[(&str, &str)]) -> Vec<HashMap<String, String>>;
            fn delete(&self, id: &str) -> Option<bool>;
            fn create_or_update(&self, properties: HashMap<String, String>) -> Option<HashMap<String, String>>;
        }
    }

    pub const PERMISSION_CREATE: u32 = 0x00000004;
    pub const PERMISSION_DELETE: u32 = 0x00000008;
}

pub struct ContactsManager {
    address_books: HashMap<String, Arc<dyn ocp::contacts::IAddressBook>>,
}

impl Default for ContactsManager {
    fn default() -> Self {
        Self {
            address_books: HashMap::new(),
        }
    }
}

impl ocp::contacts::IManager for ContactsManager {
    /// This function is used to search and find contacts within the users address books.
    /// In case pattern is empty all contacts will be returned.
    ///
    /// # Arguments
    /// * `pattern` - which should match within the search_properties
    /// * `search_properties` - defines the properties within the query pattern should match
    /// * `options` - for future use. One should always have options!
    ///
    /// # Returns
    /// * Vector of contacts which are maps of key-value-pairs
    fn search(&self, pattern: &str, search_properties: &[&str], options: &[(&str, &str)]) -> Vec<HashMap<String, String>> {
        let mut result = Vec::new();
        
        for address_book in self.address_books.values() {
            let r = address_book.search(pattern, search_properties, options);
            result.extend(r);
        }
        
        result
    }

    /// This function can be used to delete the contact identified by the given id
    ///
    /// # Arguments
    /// * `id` - the unique identifier to a contact
    /// * `address_book_key` - key of the address book
    ///
    /// # Returns
    /// * `Option<bool>` - Some(true) if successful, Some(false) if not, None if not possible
    fn delete(&self, id: &str, address_book_key: &str) -> Option<bool> {
        let address_book = self.address_books.get(address_book_key)?;
        
        if address_book.get_permissions() & ocp::PERMISSION_DELETE == 0 {
            return None;
        }
        
        address_book.delete(id)
    }

    /// This function is used to create a new contact if 'id' is not given or not present.
    /// Otherwise the contact will be updated by replacing the entire data set.
    ///
    /// # Arguments
    /// * `properties` - this map of key-value-pairs defines a contact
    /// * `address_book_key` - string to identify the address book in which the contact shall be created or updated
    ///
    /// # Returns
    /// * `Option<HashMap<String, String>>` - representing the contact just created or updated
    fn create_or_update(&self, properties: HashMap<String, String>, address_book_key: &str) -> Option<HashMap<String, String>> {
        let address_book = self.address_books.get(address_book_key)?;
        
        if address_book.get_permissions() & ocp::PERMISSION_CREATE == 0 {
            return None;
        }
        
        address_book.create_or_update(properties)
    }

    /// Check if contacts are available (e.g. contacts app enabled)
    ///
    /// # Returns
    /// * `bool` - true if enabled, false if not
    fn is_enabled(&self) -> bool {
        !self.address_books.is_empty()
    }

    /// Register a new address book
    ///
    /// # Arguments
    /// * `address_book` - the address book implementation to register
    fn register_address_book(&mut self, address_book: Arc<dyn ocp::contacts::IAddressBook>) {
        self.address_books.insert(address_book.get_key(), address_book);
    }

    /// Unregister an address book
    ///
    /// # Arguments
    /// * `address_book` - the address book implementation to unregister
    fn unregister_address_book(&mut self, address_book: Arc<dyn ocp::contacts::IAddressBook>) {
        self.address_books.remove(&address_book.get_key());
    }

    /// Get all registered address books
    ///
    /// # Returns
    /// * `HashMap<String, String>` - Map of address book keys to display names
    fn get_address_books(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        
        for address_book in self.address_books.values() {
            result.insert(address_book.get_key(), address_book.get_display_name());
        }
        
        result
    }

    /// Removes all registered address book instances
    fn clear(&mut self) {
        self.address_books.clear();
    }

    /// In order to improve lazy loading a closure can be registered which will be called in case
    /// address books are actually requested
    ///
    /// # Arguments
    /// * `key` - identifier for the closure
    /// * `callable` - the closure to register
    fn register(&mut self, _key: &str, _callable: Box<dyn Fn() + Send + Sync>) {
        // TODO: implement
    }
}