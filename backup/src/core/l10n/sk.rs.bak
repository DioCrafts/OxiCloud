use std::collections::HashMap;
use rust_i18n::translation_set::TranslationSet;

pub fn initialize_slovak_translations() -> TranslationSet {
    let mut translations = HashMap::new();
    
    translations.insert(
        "_%n minute ago_::_%n minutes ago_".to_string(),
        vec!["".to_string(), "".to_string(), "".to_string()]
    );
    
    translations.insert(
        "_%n hour ago_::_%n hours ago_".to_string(),
        vec!["".to_string(), "".to_string(), "".to_string()]
    );
    
    translations.insert(
        "_%n day ago_::_%n days ago_".to_string(),
        vec!["".to_string(), "".to_string(), "".to_string()]
    );
    
    translations.insert(
        "_%n month ago_::_%n months ago_".to_string(),
        vec!["".to_string(), "".to_string(), "".to_string()]
    );
    
    translations.insert(
        "_{count} file conflict_::_{count} file conflicts_".to_string(),
        vec!["".to_string(), "".to_string(), "".to_string()]
    );
    
    TranslationSet {
        translations,
        plural_forms: "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;".to_string(),
    }
}