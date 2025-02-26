use rust_gettext::prelude::*;
use rust_gettext::locale_data::LocaleData;
use std::collections::HashMap;

pub fn get_translation_de_at() -> LocaleData {
    let mut translations = HashMap::new();
    
    translations.insert(
        "_%n minute ago_::_%n minutes ago_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    translations.insert(
        "_%n hour ago_::_%n hours ago_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    translations.insert(
        "_%n day go_::_%n days ago_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    translations.insert(
        "_%n month ago_::_%n months ago_".to_string(),
        vec!["".to_string(), "".to_string()]
    );
    
    LocaleData {
        translations,
        plural_forms: "nplurals=2; plural=(n != 1);".to_string(),
        ..Default::default()
    }
}