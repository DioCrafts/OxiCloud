use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::i18n;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Deletion failed", "Feil ved sletting");
        m.insert("Error", "Feil");
        m.insert("Select groups", "Vel grupper");
        m.insert("Save", "Lagra");
        m.insert("Help", "Hjelp");
        m.insert("Host", "Tenar");
        m.insert("Password", "Passord");
        m.insert("Back", "Tilbake");
        m.insert("Continue", "Gå vidare");
        m
    };

    pub static ref PLURALS: HashMap<&'static str, [&'static str; 2]> = {
        let mut m = HashMap::new();
        m.insert("%s group found", ["", ""]);
        m.insert("%s user found", ["", ""]);
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_translation(key: &str, count: usize) -> &'static str {
    let plural_index = if count != 1 { 1 } else { 0 };
    PLURALS.get(key).map_or(key, |forms| forms[plural_index])
}