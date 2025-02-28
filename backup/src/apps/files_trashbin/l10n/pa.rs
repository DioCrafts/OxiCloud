use std::collections::HashMap;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Error", "ਗਲਤੀ");
    translations.insert("Delete", "ਹਟਾਓ");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}