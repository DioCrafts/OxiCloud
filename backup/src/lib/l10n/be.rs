use std::collections::HashMap;
use rust_i18n::Locale;

pub fn get_translation_data() -> (HashMap<String, Vec<String>>, String) {
    let mut translations = HashMap::new();
    
    translations.insert(
        "_%n minute ago_::_%n minutes ago_".to_string(),
        vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]
    );
    
    translations.insert(
        "_%n hour ago_::_%n hours ago_".to_string(),
        vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]
    );
    
    translations.insert(
        "_%n day go_::_%n days ago_".to_string(),
        vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]
    );
    
    translations.insert(
        "_%n month ago_::_%n months ago_".to_string(),
        vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]
    );
    
    let plural_forms = "nplurals=4; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);".to_string();
    
    (translations, plural_forms)
}

pub fn register_locale() -> Locale {
    let (translations, plural_forms) = get_translation_data();
    
    Locale::new("be")
        .with_translations(translations)
        .with_plural_forms(plural_forms)
}