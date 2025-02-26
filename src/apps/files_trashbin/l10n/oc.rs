use std::collections::HashMap;

// Translations
pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Error", "Error");
    translations.insert("Name", "Nom");
    translations.insert("Delete", "Escafa");
    translations
}

// Plural forms
pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n > 1);"
}