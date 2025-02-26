use std::collections::HashMap;

pub struct EsMX;

impl EsMX {
    pub fn get_translations() -> HashMap<String, Vec<String>> {
        let mut translations = HashMap::new();
        
        translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n hour ago_::_%n hours ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n day go_::_%n days ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n month ago_::_%n months ago_".to_string(), vec!["".to_string(), "".to_string()]);
        
        translations
    }
    
    pub fn get_plural_forms() -> &'static str {
        "nplurals=2; plural=(n != 1);"
    }
}