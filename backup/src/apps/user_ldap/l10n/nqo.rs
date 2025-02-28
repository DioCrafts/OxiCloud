use std::collections::HashMap;
use rust_gettext::prelude::*;

pub fn get_translation_map() -> HashMap<String, Vec<String>> {
    let mut translations = HashMap::new();
    translations.insert("_%s group found_::_%s groups found_".to_string(), vec!["".to_string()]);
    translations.insert("_%s user found_::_%s users found_".to_string(), vec!["".to_string()]);
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}

pub struct NqoTranslations;

impl NqoTranslations {
    pub fn new() -> Self {
        NqoTranslations
    }

    pub fn translations(&self) -> HashMap<String, Vec<String>> {
        get_translation_map()
    }

    pub fn plural_forms(&self) -> &'static str {
        get_plural_forms()
    }
}