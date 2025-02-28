use std::collections::HashMap;
use fluent_templates::Loader;
use unic_langid::LanguageIdentifier;

// Translations for Khmer (km)
pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), "".to_string());
    translations.insert("_%n hour ago_::_%n hours ago_".to_string(), "".to_string());
    translations.insert("_%n day ago_::_%n days ago_".to_string(), "".to_string());
    translations.insert("_%n month ago_::_%n months ago_".to_string(), "".to_string());
    translations.insert("_{count} file conflict_::_{count} file conflicts_".to_string(), "".to_string());
    translations.insert("Delete".to_string(), "លុប".to_string());
    
    translations
}

pub fn get_plural_form() -> String {
    "nplurals=1; plural=0;".to_string()
}

pub fn get_language_id() -> LanguageIdentifier {
    "km".parse().expect("Failed to parse language identifier")
}