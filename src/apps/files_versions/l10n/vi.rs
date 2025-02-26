use lazy_static::lazy_static;
use std::collections::HashMap;
use fluent::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Không thể khôi phục: %s");
        m.insert("Versions", "Phiên bản");
        m.insert("Restore", "Khôi phục");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
    
    pub static ref BUNDLE: FluentBundle<FluentResource> = {
        let lang_id: LanguageIdentifier = "vi".parse().expect("Failed to parse language identifier");
        let mut bundle = FluentBundle::new(vec![lang_id]);
        
        for (key, value) in TRANSLATIONS.iter() {
            let resource = FluentResource::try_new(format!("{} = {}", key, value))
                .expect("Failed to parse translation resource");
            bundle.add_resource(resource).expect("Failed to add translation resource");
        }
        
        bundle
    };
}

pub fn get_translation(key: &str) -> String {
    TRANSLATIONS.get(key).map_or(key, |&val| val).to_string()
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}