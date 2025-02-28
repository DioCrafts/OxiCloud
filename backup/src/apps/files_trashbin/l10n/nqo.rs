use std::collections::HashMap;
use rust_i18n::Translations;

pub fn init_translations() -> Translations {
    let mut translations = HashMap::new();
    
    // Define translations
    translations.insert("_%n folder_::_%n folders_".to_string(), vec!["".to_string()]);
    translations.insert("_%n file_::_%n files_".to_string(), vec!["".to_string()]);
    
    // Create and return Translations object
    Translations {
        translations,
        plural_forms: "nplurals=1; plural=0;".to_string(),
    }
}