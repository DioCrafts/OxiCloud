use std::collections::HashMap;
use rust_gettext::prelude::*;

// Translations for Icelandic (is)
pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Saving...", "Er að vista ...");
    translations.insert("Encryption", "Dulkóðun");
    translations
}

// Plural forms for Icelandic
pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Initialize gettext catalog for Icelandic
pub fn init_is_locale() -> Catalog {
    let mut catalog = Catalog::new("is", get_plural_forms());
    for (msgid, msgstr) in get_translations() {
        catalog.add_message(msgid, msgstr);
    }
    catalog
}