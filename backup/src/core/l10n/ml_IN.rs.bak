use std::collections::HashMap;
use rust_i18n::plural::PluralForm;

/// Translations for Malayalam (India) locale
pub struct MalayalamIndia;

impl MalayalamIndia {
    pub fn translations() -> HashMap<String, Vec<String>> {
        let mut translations = HashMap::new();
        
        translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n hour ago_::_%n hours ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n day ago_::_%n days ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n month ago_::_%n months ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_{count} file conflict_::_{count} file conflicts_".to_string(), vec!["".to_string(), "".to_string()]);
        
        translations
    }

    pub fn plural_form() -> PluralForm {
        // nplurals=2; plural=(n != 1);
        Box::new(|n| if n != 1 { 1 } else { 0 })
    }
}