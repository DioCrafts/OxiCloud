use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Error", "పొరపాటు");
        m.insert("Save", "భద్రపరచు");
        m.insert("Help", "సహాయం");
        m.insert("Password", "సంకేతపదం");
        m
    };

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, (&'static str, &'static str)> = {
        let mut m = HashMap::new();
        m.insert("%s group found", ("", ""));
        m.insert("%s user found", ("", ""));
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_translation(key: &str, count: i64) -> String {
    if let Some((singular, plural)) = PLURAL_TRANSLATIONS.get(key) {
        if count != 1 {
            return plural.to_string();
        }
        return singular.to_string();
    }
    key.to_string()
}