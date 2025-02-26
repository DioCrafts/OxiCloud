use std::collections::HashMap;

pub struct NeLocalization {
    pub translations: HashMap<String, Vec<String>>,
    pub plural_forms: String,
}

impl Default for NeLocalization {
    fn default() -> Self {
        let mut translations = HashMap::new();
        
        translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n hour ago_::_%n hours ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n day go_::_%n days ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n month ago_::_%n months ago_".to_string(), vec!["".to_string(), "".to_string()]);
        
        NeLocalization {
            translations,
            plural_forms: "nplurals=2; plural=(n != 1);".to_string(),
        }
    }
}

pub fn create_ne_localization() -> NeLocalization {
    NeLocalization::default()
}