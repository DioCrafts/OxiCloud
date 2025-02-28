use std::collections::HashMap;

/// Localización para de_AT (Alemán/Austria).
#[derive(Debug, Clone)]
pub struct DeAt {
    translations: HashMap<String, Vec<String>>,
    plural_forms: String,
}

impl DeAt {
    /// Crea una nueva instancia con las traducciones predefinidas.
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        translations.insert("_%n folder_::_%n folders_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n file_::_%n files_".to_string(), vec!["".to_string(), "".to_string()]);
        
        Self {
            translations,
            plural_forms: "nplurals=2; plural=(n != 1);".to_string(),
        }
    }

    /// Obtiene las traducciones disponibles.
    pub fn get_translations(&self) -> &HashMap<String, Vec<String>> {
        &self.translations
    }

    /// Obtiene la fórmula de pluralización.
    pub fn get_plural_forms(&self) -> &str {
        &self.plural_forms
    }
}

impl Default for DeAt {
    fn default() -> Self {
        Self::new()
    }
}