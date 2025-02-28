use std::collections::HashMap;

/// Translations for zh_HK (Chinese Hong Kong)
pub struct Translations {
    translations: HashMap<&'static str, &'static str>,
    plural_forms: &'static str,
}

impl Default for Translations {
    fn default() -> Self {
        let mut translations = HashMap::new();
        translations.insert("Password", "密碼");
        translations.insert("Download", "下載");
        translations.insert("Upload", "上傳");

        Self {
            translations,
            plural_forms: "nplurals=1; plural=0;",
        }
    }
}

impl Translations {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.translations.get(key).copied()
    }

    pub fn plural_forms(&self) -> &str {
        self.plural_forms
    }
}