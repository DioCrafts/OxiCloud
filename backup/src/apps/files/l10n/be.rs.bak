use rust_i18n::I18nConfig;
use std::collections::HashMap;
use rust_i18n::plural::PluralForms;

lazy_static::lazy_static! {
    static ref BE_TRANSLATIONS: HashMap<String, Vec<String>> = {
        let mut m = HashMap::new();
        m.insert("_%n folder_::_%n folders_".to_string(), vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]);
        m.insert("_%n file_::_%n files_".to_string(), vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]);
        m.insert("_Uploading %n file_::_Uploading %n files_".to_string(), vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]);
        m
    };
}

pub fn register_be_translations(config: &mut I18nConfig) {
    config.add_language("be", PLURAL_FORMS);
    
    for (key, values) in BE_TRANSLATIONS.iter() {
        for (idx, value) in values.iter().enumerate() {
            if !value.is_empty() {
                config.add_translation("be", key, idx as u32, value);
            }
        }
    }
}

// Belarusian plural forms function
const PLURAL_FORMS: PluralForms = |n| {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
};