use rust_fluent::types::{PluralCategory, PluralRules};
use std::collections::HashMap;

pub struct MlIn;

impl MlIn {
    pub fn translations() -> HashMap<String, Vec<String>> {
        let mut translations = HashMap::new();
        
        translations.insert(
            "_%n minute ago_::_%n minutes ago_".to_string(),
            vec!["".to_string(), "".to_string()]
        );
        
        translations.insert(
            "_%n hour ago_::_%n hours ago_".to_string(),
            vec!["".to_string(), "".to_string()]
        );
        
        translations.insert(
            "_%n day go_::_%n days ago_".to_string(),
            vec!["".to_string(), "".to_string()]
        );
        
        translations.insert(
            "_%n month ago_::_%n months ago_".to_string(),
            vec!["".to_string(), "".to_string()]
        );
        
        translations
    }
    
    pub fn plural_forms() -> PluralRules {
        PluralRules {
            nplurals: 2,
            plural_rule: Box::new(|n| {
                if n != 1 {
                    PluralCategory::Other
                } else {
                    PluralCategory::One
                }
            }),
            plural_rule_text: "nplurals=2; plural=(n != 1);".to_string(),
        }
    }
}