use std::collections::HashMap;
use rust_i18n::Translations;

pub fn get_translations() -> Translations {
    let mut translations = HashMap::new();
    
    translations.insert(
        "_%n folder_::_%n folders_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    translations.insert(
        "_%n file_::_%n files_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    Translations {
        translations,
        plural_forms: "nplurals=2; plural=(n != 1);".to_string(),
    }
}

#[derive(Debug, Clone)]
pub struct Translations {
    pub translations: HashMap<String, Vec<String>>,
    pub plural_forms: String,
}