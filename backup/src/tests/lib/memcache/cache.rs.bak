// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

pub mod test {
    pub mod memcache {
        use crate::test::test_cache::TestCache;
        use std::ops::{Index, IndexMut};

        pub trait Cache: Index<&'static str, Output = Option<String>> + IndexMut<&'static str> {
            fn has_key(&self, key: &str) -> bool;
            fn get(&self, key: &str) -> Option<String>;
            fn set(&mut self, key: &str, value: &str);
            fn remove(&mut self, key: &str);
            fn clear(&mut self);
        }

        pub struct CacheTest<T: Cache> {
            pub instance: T,
        }

        impl<T: Cache> TestCache for CacheTest<T> {
            fn test_exists_after_set(&mut self) {
                assert!(!self.instance.has_key("foo"));
                self.instance.set("foo", "bar");
                assert!(self.instance.has_key("foo"));
            }

            fn test_get_after_set(&mut self) {
                assert_eq!(self.instance.get("foo"), None);
                self.instance.set("foo", "bar");
                assert_eq!(self.instance.get("foo"), Some("bar".to_string()));
            }

            fn test_does_not_exist_after_remove(&mut self) {
                self.instance.set("foo", "bar");
                self.instance.remove("foo");
                assert!(!self.instance.has_key("foo"));
            }

            fn test_array_access_set(&mut self) {
                self.instance["foo"] = Some("bar".to_string());
                assert_eq!(self.instance.get("foo"), Some("bar".to_string()));
            }

            fn test_array_access_get(&mut self) {
                self.instance.set("foo", "bar");
                assert_eq!(self.instance["foo"], Some("bar".to_string()));
            }

            fn test_array_access_exists(&mut self) {
                assert_eq!(self.instance["foo"], None);
                self.instance.set("foo", "bar");
                assert_ne!(self.instance["foo"], None);
            }

            fn test_array_access_unset(&mut self) {
                self.instance.set("foo", "bar");
                self.instance["foo"] = None;
                assert!(!self.instance.has_key("foo"));
            }

            fn tear_down(&mut self) {
                self.instance.clear();
            }
        }
    }
}