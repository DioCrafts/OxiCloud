use std::collections::HashMap;
use fluent::FluentBundle;
use unic_langid::LanguageIdentifier;

pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    translations.insert("Password".to_string(), "Passcode".to_string());
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_plural_translations() -> HashMap<String, Vec<String>> {
    let mut plural_translations = HashMap::new();
    plural_translations.insert("_%s group found_::_%s groups found_".to_string(), vec!["".to_string(), "".to_string()]);
    plural_translations.insert("_%s user found_::_%s users found_".to_string(), vec!["".to_string(), "".to_string()]);
    plural_translations
}

pub fn create_bundle() -> FluentBundle<&'static [&'static str]> {
    let lang_id: LanguageIdentifier = "en-Pirate".parse().expect("Invalid language identifier");
    FluentBundle::new(&[lang_id])
}