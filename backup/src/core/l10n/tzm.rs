use std::collections::HashMap;
use rust_i18n::plural::PluralForms;

// Translation mapping for Tamazight (Central Atlas)
pub fn get_translations() -> HashMap<String, Vec<String>> {
    let mut translations = HashMap::new();
    
    translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), vec!["".to_string(), "".to_string()]);
    translations.insert("_%n hour ago_::_%n hours ago_".to_string(), vec!["".to_string(), "".to_string()]);
    translations.insert("_%n day ago_::_%n days ago_".to_string(), vec!["".to_string(), "".to_string()]);
    translations.insert("_%n month ago_::_%n months ago_".to_string(), vec!["".to_string(), "".to_string()]);
    translations.insert("_{count} file conflict_::_{count} file conflicts_".to_string(), vec!["".to_string(), "".to_string()]);
    
    translations
}

pub fn get_plural_forms() -> PluralForms {
    PluralForms {
        nplurals: 2,
        plural_function: Box::new(|n| {
            if n == 0 || n == 1 || (n > 10 && n < 100) {
                0
            } else {
                1
            }
        }),
    }
}