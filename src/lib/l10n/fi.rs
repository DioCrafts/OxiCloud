use std::collections::HashMap;

/// Finnish (fi) translations.
pub struct FiLang;

impl FiLang {
    /// Returns translations mapping.
    pub fn translations() -> HashMap<&'static str, &'static str> {
        let mut translations = HashMap::new();
        translations.insert("Settings", "asetukset");
        translations
    }

    /// Returns plural forms rule.
    pub fn plural_forms() -> &'static str {
        "nplurals=2; plural=(n != 1);"
    }
}