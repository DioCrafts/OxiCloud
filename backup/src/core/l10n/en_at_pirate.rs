use std::collections::HashMap;
use rust_i18n::i18n;

/// Translations for English (Pirate)
pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    translations.insert("Password".to_string(), "Passcode".to_string());
    translations
}

/// Plural forms definition for English (Pirate)
pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

/// Get plural translations for specific phrases
pub fn get_plural_translations() -> HashMap<String, Vec<String>> {
    let mut plural_translations = HashMap::new();
    
    plural_translations.insert(
        "_%n minute ago_::_%n minutes ago_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    plural_translations.insert(
        "_%n hour ago_::_%n hours ago_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    plural_translations.insert(
        "_%n day ago_::_%n days ago_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    plural_translations.insert(
        "_%n month ago_::_%n months ago_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    plural_translations.insert(
        "_{count} file conflict_::_{count} file conflicts_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    plural_translations
}