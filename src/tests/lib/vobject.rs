/*
 * Copyright (c) 2013 Thomas Tanghus (thomas@tanghus.net)
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use std::collections::HashMap;
use std::sync::Once;
use rust_sabre_vobject::{Property, StringProperty, CompoundProperty};

static INIT: Once = Once::new();

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn setup() {
        INIT.call_once(|| {
            let mut class_map = HashMap::new();
            class_map.insert("SUMMARY".to_string(), "OC::VObject::StringProperty".to_string());
            class_map.insert("ORG".to_string(), "OC::VObject::CompoundProperty".to_string());
            Property::register_class_map(class_map);
        });
    }
    
    #[test]
    fn test_string_property() {
        setup();
        
        let property = Property::create("SUMMARY", "Escape;this,please").unwrap();
        assert_eq!("SUMMARY:Escape\\;this\\,please\r\n", property.serialize());
    }
    
    #[test]
    fn test_compound_property() {
        setup();
        
        let arr = vec![
            "ABC, Inc.".to_string(),
            "North American Division".to_string(),
            "Marketing;Sales".to_string(),
        ];
        
        let mut property = Property::create("ORG", "").unwrap();
        property.set_parts(&arr).unwrap();
        
        assert_eq!("ABC\\, Inc.;North American Division;Marketing\\;Sales", property.value());
        assert_eq!("ORG:ABC\\, Inc.;North American Division;Marketing\\;Sales\r\n", property.serialize());
        
        let parts = property.get_parts();
        assert_eq!(3, parts.len());
        assert_eq!("Marketing;Sales", parts[2]);
    }
}