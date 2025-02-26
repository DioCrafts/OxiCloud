//! Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.

use std::ops::{Index, IndexMut};
use std::collections::HashMap;

/// Interface for session implementations
pub trait ISession {
    /// Construct a new session with the given name
    fn new(name: String) -> Self where Self: Sized;

    /// Check if a key exists in the session
    fn exists(&self, key: &str) -> bool;

    /// Get a value from the session
    fn get(&self, key: &str) -> Option<&str>;

    /// Set a value in the session
    fn set(&mut self, key: &str, value: &str);

    /// Remove a value from the session
    fn remove(&mut self, key: &str);
}

/// Abstract session implementation
pub struct Session {
    name: String,
    data: HashMap<String, String>,
}

impl<'a> Session {
    /// $name serves as a namespace for the session keys
    ///
    /// # Arguments
    /// * `name` - The namespace for session keys
    pub fn new(name: String) -> Self {
        Session {
            name,
            data: HashMap::new(),
        }
    }

    /// Check if a key exists in the session
    pub fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Get a value from the session
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    /// Set a value in the session
    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    /// Remove a value from the session
    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }
}

impl std::ops::Index<&str> for Session {
    type Output = str;

    fn index(&self, key: &str) -> &Self::Output {
        self.get(key).expect("Key not found in session")
    }
}

impl std::ops::IndexMut<&str> for Session {
    fn index_mut(&mut self, _key: &str) -> &mut Self::Output {
        panic!("IndexMut not supported for Session. Use set() instead")
    }
}

impl ISession for Session {
    fn new(name: String) -> Self {
        Session::new(name)
    }

    fn exists(&self, key: &str) -> bool {
        self.exists(key)
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.get(key)
    }

    fn set(&mut self, key: &str, value: &str) {
        self.set(key, value)
    }

    fn remove(&mut self, key: &str) {
        self.remove(key)
    }
}