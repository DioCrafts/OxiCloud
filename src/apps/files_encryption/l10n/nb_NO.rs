use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Saving...", "Lagrer...");
    m.insert("Encryption", "Kryptering");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";