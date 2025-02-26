// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::fmt::Debug;

/// Trait defining the cache interface
pub trait Cache: Send + Sync {
    fn get<T: for<'a> serde::de::Deserialize<'a> + Debug>(&self, key: &str) -> Option<T>;
    fn set<T: serde::Serialize + Debug>(&self, key: &str, value: T) -> bool;
    fn has_key(&self, key: &str) -> bool;
    fn remove(&self, key: &str) -> bool;
    fn clear(&self, prefix: Option<&str>) -> bool;
}

pub struct TestCache<C: Cache> {
    instance: Option<C>,
}

impl<C: Cache> TestCache<C> {
    pub fn new(instance: C) -> Self {
        Self {
            instance: Some(instance),
        }
    }

    pub fn tear_down(&mut self) {
        if let Some(instance) = &self.instance {
            instance.clear(None);
        }
    }

    pub fn test_simple(&self) {
        let instance = self.instance.as_ref().unwrap();
        
        assert_eq!(instance.get::<String>("value1"), None);
        assert_eq!(instance.has_key("value1"), false);
        
        let value = "foobar".to_string();
        instance.set("value1", &value);
        assert_eq!(instance.has_key("value1"), true);
        let received: Option<String> = instance.get("value1");
        assert_eq!(received, Some(value.clone()), "Value received from cache not equal to the original");
        
        let value = "ipsum lorum".to_string();
        instance.set("value1", &value);
        let received: Option<String> = instance.get("value1");
        assert_eq!(received, Some(value.clone()), "Value not overwritten by second set");

        let value2 = "foobar".to_string();
        instance.set("value2", &value2);
        let received2: Option<String> = instance.get("value2");
        assert_eq!(instance.has_key("value1"), true);
        assert_eq!(instance.has_key("value2"), true);
        let received: Option<String> = instance.get("value1");
        assert_eq!(received, Some(value.clone()), "Value changed while setting other variable");
        assert_eq!(received2, Some(value2.clone()), "Second value not equal to original");

        assert_eq!(instance.has_key("not_set"), false);
        assert_eq!(instance.get::<String>("not_set"), None, "Unset value not equal to null");

        assert_eq!(instance.remove("value1"), true);
        assert_eq!(instance.has_key("value1"), false);
    }

    pub fn test_clear(&self) {
        let instance = self.instance.as_ref().unwrap();
        
        let value = "ipsum lorum".to_string();
        instance.set("1_value1", &value);
        instance.set("1_value2", &value);
        instance.set("2_value1", &value);
        instance.set("3_value1", &value);

        assert_eq!(instance.clear(Some("1_")), true);
        assert_eq!(instance.has_key("1_value1"), false);
        assert_eq!(instance.has_key("1_value2"), false);
        assert_eq!(instance.has_key("2_value1"), true);
        assert_eq!(instance.has_key("3_value1"), true);

        assert_eq!(instance.clear(None), true);
        assert_eq!(instance.has_key("1_value1"), false);
        assert_eq!(instance.has_key("1_value2"), false);
        assert_eq!(instance.has_key("2_value1"), false);
        assert_eq!(instance.has_key("3_value1"), false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // A mock implementation would be needed to run these tests
    // Example of how it might be used:
    // 
    // #[test]
    // fn test_simple_cache() {
    //     let mock_cache = MockCache::new();
    //     let mut test_cache = TestCache::new(mock_cache);
    //     test_cache.test_simple();
    //     test_cache.tear_down();
    // }
}