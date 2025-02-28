use std::collections::HashMap;

/// Translations for the Tamazight language (tzm)
pub struct TzmeTranslations {
    translations: HashMap<&'static str, Vec<&'static str>>,
    plural_forms: &'static str,
}

impl TzmeTranslations {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        // Add plural form translations
        translations.insert("_%s group found_::_%s groups found_", vec!["", ""]);
        translations.insert("_%s user found_::_%s users found_", vec!["", ""]);
        
        TzmeTranslations {
            translations,
            plural_forms: "nplurals=2; plural=(n == 0 || n == 1 || (n > 10 && n < 100) ? 0 : 1;"
        }
    }
    
    pub fn get_translation(&self, key: &str) -> Option<&Vec<&'static str>> {
        self.translations.get(key)
    }
    
    pub fn get_plural_forms(&self) -> &'static str {
        self.plural_forms
    }
}

impl Default for TzmeTranslations {
    fn default() -> Self {
        Self::new()
    }
}