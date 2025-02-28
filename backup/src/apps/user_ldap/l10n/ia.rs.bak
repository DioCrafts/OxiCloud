use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Error", "Error");
        m.insert("Save", "Salveguardar");
        m.insert("Help", "Adjuta");
        m.insert("Password", "Contrasigno");
        m.insert("Back", "Retro");
        m
    };

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%s group found_::_%s groups found_", vec!["", ""]);
        m.insert("_%s user found_::_%s users found_", vec!["", ""]);
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(&key)
}

pub fn get_plural_translation(key: &str, n: usize) -> &'static str {
    let plural_idx = if n != 1 { 1 } else { 0 };
    PLURAL_TRANSLATIONS
        .get(key)
        .and_then(|forms| forms.get(plural_idx))
        .unwrap_or(&"")
}