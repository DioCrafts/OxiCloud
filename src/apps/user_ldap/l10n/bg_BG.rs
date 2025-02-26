use std::collections::HashMap;
use rust_i18n::i18n;

// Translation file for Bulgarian (bg_BG)
pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    translations.insert("Error".to_string(), "Грешка".to_string());
    translations.insert("Save".to_string(), "Запис".to_string());
    translations.insert("Help".to_string(), "Помощ".to_string());
    translations.insert("Password".to_string(), "Парола".to_string());
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_plurals() -> HashMap<String, Vec<String>> {
    let mut plurals = HashMap::new();
    plurals.insert("_%s group found_::_%s groups found_".to_string(), vec!["".to_string(), "".to_string()]);
    plurals.insert("_%s user found_::_%s users found_".to_string(), vec!["".to_string(), "".to_string()]);
    plurals
}

i18n!("bg_BG");