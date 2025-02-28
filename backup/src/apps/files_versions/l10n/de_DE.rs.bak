use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Konnte %s nicht zurücksetzen");
        m.insert("Versions", "Versionen");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Konnte {file} der Revision {timestamp} nicht rückgänging machen.");
        m.insert("More versions...", "Mehrere Versionen...");
        m.insert("No other versions available", "Keine anderen Versionen verfügbar");
        m.insert("Restore", "Wiederherstellen");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}