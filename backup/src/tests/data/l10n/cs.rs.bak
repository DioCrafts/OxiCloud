use std::collections::HashMap;

pub struct CsTranslation {
    pub translations: HashMap<String, Vec<String>>,
    pub plural_forms: String,
}

impl Default for CsTranslation {
    fn default() -> Self {
        let mut translations = HashMap::new();
        translations.insert(
            "_%n window__%n windows_".to_string(),
            vec!["%n okno".to_string(), "%n okna".to_string(), "%n oken".to_string()],
        );

        CsTranslation {
            translations,
            plural_forms: "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;".to_string(),
        }
    }
}

pub fn get_cs_translation() -> CsTranslation {
    CsTranslation::default()
}