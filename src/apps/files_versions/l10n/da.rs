use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_gettext::Catalog;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Kunne ikke genskabe: %s");
        m.insert("Versions", "Versioner");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Kunne ikke tilbagerulle {file} til den tidligere udgave: {timestamp}.");
        m.insert("More versions...", "Flere versioner...");
        m.insert("No other versions available", "Ingen andre versioner tilgængelig");
        m.insert("Restore", "Gendan");
        m
    };
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn init_catalog() -> Catalog {
    let mut catalog = Catalog::new();
    for (key, value) in TRANSLATIONS.iter() {
        catalog.add_string(key, value);
    }
    catalog.set_plural_forms(get_plural_forms());
    catalog
}