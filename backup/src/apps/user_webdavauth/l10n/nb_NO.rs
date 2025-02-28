use once_cell::sync::Lazy;
use std::collections::HashMap;
use rust_gettext::Catalog;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Address: ", "Adresse:");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation_catalog() -> Catalog<'static> {
    let mut catalog = Catalog::new();
    
    for (key, value) in TRANSLATIONS.iter() {
        catalog.add_message(key, value);
    }
    
    catalog.set_plural_forms(PLURAL_FORMS);
    catalog
}