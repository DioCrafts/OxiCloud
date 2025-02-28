/// Translations for Punjabi language
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password", "ਪਾਸਵਰ");
        m.insert("Download", "ਡਾਊਨਲੋਡ");
        m.insert("Upload", "ਅੱਪਲੋਡ");
        m.insert("Cancel upload", "ਅੱਪਲੋਡ ਰੱਦ ਕਰੋ");
        m
    };
}

/// Plural forms definition for Punjabi language
pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}