/*
 * ownCloud
 *
 * @author Thomas Müller
 * @copyright 2012 Thomas Müller thomas.mueller@tmit.eu
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
 */

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use mockall::{mock, predicate::*};

// Define the IAddressBook trait
pub trait IAddressBook: Send + Sync {
    fn get_key(&self) -> String;
    fn get_display_name(&self) -> String;
    fn search(&self, query: &str, properties: &[&str]) -> Vec<HashMap<String, serde_json::Value>>;
}

// Mock implementation for IAddressBook
mock! {
    pub AddressBook {}
    impl IAddressBook for AddressBook {
        fn get_key(&self) -> String;
        fn get_display_name(&self) -> String;
        fn search(&self, query: &str, properties: &[&str]) -> Vec<HashMap<String, serde_json::Value>>;
    }
}

// The Contacts module
pub struct Contacts;

// Singleton implementation for storing address books
lazy_static::lazy_static! {
    static ref ADDRESS_BOOKS: Mutex<HashMap<String, Arc<dyn IAddressBook>>> = Mutex::new(HashMap::new());
}

impl Contacts {
    pub fn clear() {
        let mut books = ADDRESS_BOOKS.lock().unwrap();
        books.clear();
    }

    pub fn is_enabled() -> bool {
        let books = ADDRESS_BOOKS.lock().unwrap();
        !books.is_empty()
    }

    pub fn register_address_book(address_book: Arc<dyn IAddressBook>) {
        let key = address_book.get_key();
        let mut books = ADDRESS_BOOKS.lock().unwrap();
        books.insert(key, address_book);
    }

    pub fn unregister_address_book(address_book: Arc<dyn IAddressBook>) {
        let key = address_book.get_key();
        let mut books = ADDRESS_BOOKS.lock().unwrap();
        books.remove(&key);
    }

    pub fn get_address_books() -> HashMap<String, String> {
        let books = ADDRESS_BOOKS.lock().unwrap();
        books.iter()
            .map(|(key, book)| (key.clone(), book.get_display_name()))
            .collect()
    }

    pub fn search(query: &str, properties: Vec<&str>) -> Vec<HashMap<String, serde_json::Value>> {
        let books = ADDRESS_BOOKS.lock().unwrap();
        let mut results = Vec::new();
        
        for book in books.values() {
            let search_results = book.search(query, &properties);
            results.extend(search_results);
        }
        
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use serde_json::json;

    #[test]
    fn test_disabled_if_empty() {
        Contacts::clear();
        assert!(!Contacts::is_enabled());
    }

    #[test]
    fn test_enabled_after_register() {
        Contacts::clear();
        
        let mut mock = MockAddressBook::new();
        mock.expect_get_key()
            .times(2)
            .returning(|| "SIMPLE_ADDRESS_BOOK".to_string());

        // not enabled before register
        assert!(!Contacts::is_enabled());

        // register the address book
        let address_book = Arc::new(mock);
        Contacts::register_address_book(address_book.clone());

        // contacts api shall be enabled
        assert!(Contacts::is_enabled());

        // unregister the address book
        Contacts::unregister_address_book(address_book);

        // not enabled after register
        assert!(!Contacts::is_enabled());
    }

    #[test]
    fn test_address_book_enumeration() {
        Contacts::clear();
        
        let mut mock = MockAddressBook::new();
        mock.expect_get_key()
            .returning(|| "SIMPLE_ADDRESS_BOOK".to_string());
        mock.expect_get_display_name()
            .returning(|| "A very simple Addressbook".to_string());

        // register the address book
        Contacts::register_address_book(Arc::new(mock));
        let all_books = Contacts::get_address_books();

        assert_eq!(1, all_books.len());
        assert_eq!("A very simple Addressbook", all_books["SIMPLE_ADDRESS_BOOK"]);
    }

    #[test]
    fn test_search_in_address_book() {
        Contacts::clear();
        
        // Create mock for the first address book
        let mut mock1 = MockAddressBook::new();
        mock1.expect_get_key()
            .returning(|| "SIMPLE_ADDRESS_BOOK1".to_string());
        mock1.expect_get_display_name()
            .returning(|| "Address book ownCloud Inc".to_string());
        
        let search_result1 = vec![
            {
                let mut map = HashMap::new();
                map.insert("id".to_string(), json!(0));
                map.insert("FN".to_string(), json!("Frank Karlitschek"));
                map.insert("EMAIL".to_string(), json!("a@b.c"));
                map.insert("GEO".to_string(), json!("37.386013;-122.082932"));
                map
            },
            {
                let mut map = HashMap::new();
                map.insert("id".to_string(), json!(5));
                map.insert("FN".to_string(), json!("Klaas Freitag"));
                map.insert("EMAIL".to_string(), json!(["d@e.f", "g@h.i"]));
                map
            },
        ];
        
        mock1.expect_search()
            .with(always(), always())
            .returning(move |_, _| search_result1.clone());

        // Create mock for the second address book
        let mut mock2 = MockAddressBook::new();
        mock2.expect_get_key()
            .returning(|| "SIMPLE_ADDRESS_BOOK2".to_string());
        mock2.expect_get_display_name()
            .returning(|| "Address book ownCloud Community".to_string());
        
        let search_result2 = vec![
            {
                let mut map = HashMap::new();
                map.insert("id".to_string(), json!(0));
                map.insert("FN".to_string(), json!("Thomas Müller"));
                map.insert("EMAIL".to_string(), json!("a@b.c"));
                map
            },
            {
                let mut map = HashMap::new();
                map.insert("id".to_string(), json!(5));
                map.insert("FN".to_string(), json!("Thomas Tanghus"));
                map.insert("EMAIL".to_string(), json!(["d@e.f", "g@h.i"]));
                map
            },
        ];
        
        mock2.expect_search()
            .with(always(), always())
            .returning(move |_, _| search_result2.clone());

        // register the address books
        Contacts::register_address_book(Arc::new(mock1));
        Contacts::register_address_book(Arc::new(mock2));
        let all_books = Contacts::get_address_books();

        // assert the count - doesn't hurt
        assert_eq!(2, all_books.len());

        // perform the search
        let result = Contacts::search("x", vec![]);

        // we expect 4 hits
        assert_eq!(4, result.len());
    }
}