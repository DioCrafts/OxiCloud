use std::collections::HashMap;
use rust_i18n::i18n;

// Localization for ka (Georgian)
pub fn register_ka_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Files".to_string(), "ფაილები".to_string());
    translations.insert("_%n folder_::_%n folders_".to_string(), "".to_string());
    translations.insert("_%n file_::_%n files_".to_string(), "".to_string());
    translations.insert("_Uploading %n file_::_Uploading %n files_".to_string(), "".to_string());
    translations.insert("Download".to_string(), "გადმოწერა".to_string());
    
    translations
}

// Plural forms definition for Georgian
pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";