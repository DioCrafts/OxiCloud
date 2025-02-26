use std::collections::HashMap;

// Translations for Bulgarian (bg_BG)
pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Saving...", "Записване...");
    translations.insert("Encryption", "Криптиране");
    translations
}

// Plural forms rule for Bulgarian
pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}