use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Impossibile ripristinare: %s");
        m.insert("Versions", "Versioni");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Ripristino di {file} alla revisione {timestamp} non riuscito.");
        m.insert("More versions...", "Altre versioni...");
        m.insert("No other versions available", "Non sono disponibili altre versioni");
        m.insert("Restore", "Ripristina");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(&key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}