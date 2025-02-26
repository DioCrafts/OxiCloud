use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Kunde inte återställa: %s");
        m.insert("Versions", "Versioner");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Kunde inte återställa {file} till revision {timestamp}.");
        m.insert("More versions...", "Fler versioner...");
        m.insert("No other versions available", "Inga andra versioner tillgängliga");
        m.insert("Restore", "Återskapa");
        m
    };
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}