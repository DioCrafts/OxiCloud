use rust_i18n::translation_hashmap;
use std::collections::HashMap;

pub fn get_translation() -> HashMap<String, String> {
    translation_hashmap!(
        "Help" => "सहयोग",
        "Personal" => "यक्तिगत",
        "Settings" => "सेटिंग्स",
        "Users" => "उपयोगकर्ता",
    )
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_plural_translations() -> HashMap<String, Vec<String>> {
    let mut plural_translations = HashMap::new();
    
    plural_translations.insert(
        "_%n minute ago_::_%n minutes ago_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    plural_translations.insert(
        "_%n hour ago_::_%n hours ago_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    plural_translations.insert(
        "_%n day go_::_%n days ago_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    plural_translations.insert(
        "_%n month ago_::_%n months ago_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    plural_translations
}