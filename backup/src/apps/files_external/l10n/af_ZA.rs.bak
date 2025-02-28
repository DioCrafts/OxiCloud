pub struct L10n {
    translations: std::collections::HashMap<&'static str, &'static str>,
    plural_forms: &'static str,
}

impl L10n {
    pub fn new() -> Self {
        let mut translations = std::collections::HashMap::new();
        translations.insert("Users", "Gebruikers");
        
        L10n {
            translations,
            plural_forms: "nplurals=2; plural=(n != 1);",
        }
    }
    
    pub fn get_translation(&self, key: &str) -> Option<&str> {
        self.translations.get(key).copied()
    }
    
    pub fn get_plural_forms(&self) -> &str {
        self.plural_forms
    }
}

pub fn get_af_za_l10n() -> L10n {
    L10n::new()
}