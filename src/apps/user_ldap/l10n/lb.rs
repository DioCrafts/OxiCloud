use std::collections::HashMap;
use rust_i18n::i18n;

// Translation file for Luxembourgish (lb)

lazy_static::lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Deletion failed", "Konnt net läschen");
        m.insert("Error", "Fehler");
        m.insert("Save", "Späicheren");
        m.insert("Help", "Hëllef");
        m.insert("Host", "Host");
        m.insert("Password", "Passwuert");
        m.insert("Back", "Zeréck");
        m.insert("Continue", "Weider");
        m
    };
}

// Plural forms handler
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

// Plural translations
pub fn translate_plural(key: &str, count: usize) -> String {
    match key {
        "%s group found" => {
            match get_plural_form(count) {
                0 => format!("{} group found", count),
                _ => format!("{} groups found", count),
            }
        },
        "%s user found" => {
            match get_plural_form(count) {
                0 => format!("{} user found", count),
                _ => format!("{} users found", count),
            }
        },
        _ => String::from(key),
    }
}

// Register translation module
i18n!("lb");