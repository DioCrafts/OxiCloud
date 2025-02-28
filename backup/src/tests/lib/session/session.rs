/*
 * Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

#[cfg(test)]
mod tests {
    use std::ops::{Index, IndexMut};
    use std::collections::HashMap;
    
    // Mock for Rust implementation of OC\Session\Session
    pub trait SessionTrait {
        fn exists(&self, key: &str) -> bool;
        fn set(&mut self, key: &str, value: impl Into<String>);
        fn get(&self, key: &str) -> Option<String>;
        fn remove(&mut self, key: &str);
        fn clear(&mut self);
    }
    
    pub struct Session {
        data: HashMap<String, String>,
    }
    
    impl SessionTrait for Session {
        fn exists(&self, key: &str) -> bool {
            self.data.contains_key(key)
        }
        
        fn set(&mut self, key: &str, value: impl Into<String>) {
            self.data.insert(key.to_string(), value.into());
        }
        
        fn get(&self, key: &str) -> Option<String> {
            self.data.get(key).cloned()
        }
        
        fn remove(&mut self, key: &str) {
            self.data.remove(key);
        }
        
        fn clear(&mut self) {
            self.data.clear();
        }
    }
    
    impl Index<&str> for Session {
        type Output = String;
        
        fn index(&self, key: &str) -> &Self::Output {
            &self.data[key]
        }
    }
    
    impl IndexMut<&str> for Session {
        fn index_mut(&mut self, key: &str) -> &mut Self::Output {
            self.data.entry(key.to_string()).or_insert_with(String::new)
        }
    }
    
    // Abstract test class becoming a trait with default implementations
    pub trait TestSession {
        fn setup(&mut self) -> Box<dyn SessionTrait>;
        fn tear_down(&mut self, instance: &mut Box<dyn SessionTrait>);
        
        fn test_not_exists_empty(&mut self) {
            let instance = self.setup();
            assert!(!instance.exists("foo"));
        }
        
        fn test_exists_after_set(&mut self) {
            let mut instance = self.setup();
            instance.set("foo", "1");
            assert!(instance.exists("foo"));
            self.tear_down(&mut instance);
        }
        
        fn test_not_exists_after_remove(&mut self) {
            let mut instance = self.setup();
            instance.set("foo", "1");
            instance.remove("foo");
            assert!(!instance.exists("foo"));
            self.tear_down(&mut instance);
        }
        
        fn test_get_non_existing(&mut self) {
            let instance = self.setup();
            assert_eq!(None, instance.get("foo"));
        }
        
        fn test_get_after_set(&mut self) {
            let mut instance = self.setup();
            instance.set("foo", "bar");
            assert_eq!(Some("bar".to_string()), instance.get("foo"));
            self.tear_down(&mut instance);
        }
        
        fn test_remove_non_existing(&mut self) {
            let mut instance = self.setup();
            assert!(!instance.exists("foo"));
            instance.remove("foo");
            assert!(!instance.exists("foo"));
            self.tear_down(&mut instance);
        }
        
        fn test_not_exists_after_clear(&mut self) {
            let mut instance = self.setup();
            instance.set("foo", "1");
            instance.clear();
            assert!(!instance.exists("foo"));
            self.tear_down(&mut instance);
        }
        
        fn test_array_interface(&mut self) {
            let mut session = Session { data: HashMap::new() };
            
            assert!(!session.exists("foo"));
            session["foo"] = "bar".to_string();
            assert!(session.exists("foo"));
            assert_eq!("bar", session["foo"]);
            session.remove("foo");
            assert!(!session.exists("foo"));
        }
    }
    
    // Concrete implementation of test trait
    pub struct TestSessionImpl {
        instance: Option<Box<dyn SessionTrait>>,
    }
    
    impl TestSessionImpl {
        pub fn new() -> Self {
            TestSessionImpl { instance: None }
        }
    }
    
    impl TestSession for TestSessionImpl {
        fn setup(&mut self) -> Box<dyn SessionTrait> {
            let session = Box::new(Session { data: HashMap::new() });
            self.instance = Some(session);
            self.instance.as_ref().unwrap().clone()
        }
        
        fn tear_down(&mut self, instance: &mut Box<dyn SessionTrait>) {
            instance.clear();
        }
    }
    
    // This would be implemented by the testing framework
    impl Clone for Box<dyn SessionTrait> {
        fn clone(&self) -> Self {
            // In real implementation, this would properly clone
            Box::new(Session { data: HashMap::new() })
        }
    }
}