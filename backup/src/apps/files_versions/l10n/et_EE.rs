use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Ei suuda taastada faili: %s");
        m.insert("Versions", "Versioonid");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Ebaõnnestus faili {file} taastamine revisjonile {timestamp}");
        m.insert("More versions...", "Rohkiem versioone...");
        m.insert("No other versions available", "Muid versioone pole saadaval");
        m.insert("Restore", "Taasta");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}