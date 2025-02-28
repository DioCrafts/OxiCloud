use once_cell::sync::Lazy;
use std::collections::HashMap;
use rust_i18n::Locale;

// Dutch translations
pub static NL_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Could not revert: %s", "Kon niet terugdraaien: %s");
    translations.insert("Versions", "Versies");
    translations.insert("Failed to revert {file} to revision {timestamp}.", "Kon {file} niet terugdraaien naar revisie {timestamp}.");
    translations.insert("More versions...", "Meer versies...");
    translations.insert("No other versions available", "Geen andere versies beschikbaar");
    translations.insert("Restore", "Herstellen");
    translations
});

// Plural forms definition for Dutch
pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Register the Dutch locale
pub fn register_locale() {
    Locale::new("nl", NL_TRANSLATIONS.clone(), get_plural_forms());
}