use std::collections::HashMap;

pub struct De {
    translations: HashMap<String, Vec<String>>,
    plural_forms: String,
}

impl Default for De {
    fn default() -> Self {
        let mut translations = HashMap::new();
        translations.insert(
            "_%n file__%n files_".to_string(), 
            vec!["%n Datei".to_string(), "%n Dateien".to_string()]
        );

        Self {
            translations,
            plural_forms: "nplurals=2; plural=(n != 1);".to_string(),
        }
    }
}

impl De {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_translations(&self) -> &HashMap<String, Vec<String>> {
        &self.translations
    }

    pub fn get_plural_forms(&self) -> &str {
        &self.plural_forms
    }
}