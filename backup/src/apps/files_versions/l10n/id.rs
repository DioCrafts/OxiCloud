use once_cell::sync::Lazy;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

pub static ID_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Could not revert: %s".to_string(), "Tidak dapat mengembalikan: %s".to_string());
    translations.insert("Versions".to_string(), "Versi".to_string());
    translations.insert("Restore".to_string(), "Pulihkan".to_string());
    
    Translations {
        translations,
        plural_form: "nplurals=1; plural=0;".to_string(),
        language_id: "id".parse::<LanguageIdentifier>().unwrap(),
    }
});

pub struct Translations {
    pub translations: HashMap<String, String>,
    pub plural_form: String,
    pub language_id: LanguageIdentifier,
}

impl Translations {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }
    
    pub fn translate(&self, key: &str) -> String {
        self.get(key).map_or_else(|| key.to_string(), |s| s.clone())
    }
}