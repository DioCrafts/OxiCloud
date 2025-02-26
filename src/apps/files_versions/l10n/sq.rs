use once_cell::sync::Lazy;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

pub static SQ_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Restore", "Rivendos");
    m
});

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_language_identifier() -> LanguageIdentifier {
    "sq".parse().expect("Failed to parse language identifier")
}