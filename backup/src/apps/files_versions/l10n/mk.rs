use rust_i18n::locale;

#[derive(Debug, Clone)]
pub struct MkTranslations {
    translations: std::collections::HashMap<String, String>,
    plural_forms: String,
}

impl MkTranslations {
    pub fn new() -> Self {
        let mut translations = std::collections::HashMap::new();
        
        translations.insert(
            "Could not revert: %s".to_string(),
            "Не можев да го вратам: %s".to_string(),
        );
        translations.insert(
            "Versions".to_string(),
            "Версии".to_string(),
        );
        translations.insert(
            "Failed to revert {file} to revision {timestamp}.".to_string(),
            "Не успеав да го вратам {file} на ревизијата {timestamp}.".to_string(),
        );
        translations.insert(
            "More versions...".to_string(),
            "Повеќе верзии...".to_string(),
        );
        translations.insert(
            "No other versions available".to_string(),
            "Не постојат други верзии".to_string(),
        );
        translations.insert(
            "Restore".to_string(),
            "Врати".to_string(),
        );

        Self {
            translations,
            plural_forms: "nplurals=2; plural=(n % 10 == 1 && n % 100 != 11) ? 0 : 1;".to_string(),
        }
    }

    pub fn get_translation(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }

    pub fn get_plural_forms(&self) -> &str {
        &self.plural_forms
    }
}

impl Default for MkTranslations {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "register_locale")]
pub fn register() {
    locale!(mk);
}