use rust_gettext::prelude::*;
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Could not revert: %s", "Nepavyko atstatyti: %s");
    map.insert("Versions", "Versijos");
    map.insert("Failed to revert {file} to revision {timestamp}.", "Nepavyko atstatyti {file} į būseną {timestamp}.");
    map.insert("More versions...", "Daugiau versijų...");
    map.insert("No other versions available", "Nėra daugiau versijų");
    map.insert("Restore", "Atstatyti");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && (n%100<10 || n%100>=20) ? 1 : 2);";

pub fn get_translation(text: &str) -> &str {
    TRANSLATIONS.get(text).unwrap_or(&text)
}

pub fn initialize_translations() -> Catalog {
    let mut catalog = Catalog::new("lt_LT", PLURAL_FORMS);
    for (key, value) in TRANSLATIONS.iter() {
        catalog.add_message(key, value);
    }
    catalog
}