use std::collections::HashMap;
use unic_langid::LanguageIdentifier;
use fluent_templates::static_loader;
use fluent_templates::LanguageIdentifier as FluentLanguageIdentifier;
use once_cell::sync::Lazy;
use std::str::FromStr;

// Define a static resource for the pirate English translations
static_loader! {
    static LOCALES = {
        locales: "./apps/files/l10n/",
        fallback_language: "en",
    };
}

// Lazy initialization of the language identifier
static PIRATE_LANG_ID: Lazy<FluentLanguageIdentifier> = Lazy::new(|| {
    FluentLanguageIdentifier::from_str("en-PIRATE").unwrap()
});

// Define the translation keys and their values
pub fn init_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    // Add singular and plural forms
    translations.insert(String::from("_%n folder_::_%n folders_"), String::from(""));
    translations.insert(String::from("_%n file_::_%n files_"), String::from(""));
    translations.insert(String::from("_Uploading %n file_::_Uploading %n files_"), String::from(""));
    translations.insert(String::from("Download"), String::from("Download"));
    
    translations
}

// Get the plural form rule for pirate English
pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Helper function to get a translation by key
pub fn get_translation(key: &str) -> Option<String> {
    let translations = init_translations();
    translations.get(key).cloned()
}

// Helper function to get plural form for a specific count
pub fn get_plural_translation(key: &str, count: i64) -> Option<String> {
    let translations = init_translations();
    let plural_index = if count != 1 { 1 } else { 0 };
    
    // In a real implementation, this would handle the pluralization based on the rule
    // For now, we just return the key as a placeholder
    Some(key.to_string())
}