use rust_i18n::l10n::L10n;
use std::collections::HashMap;

pub fn get_l10n_ach() -> L10n {
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
    
    L10n {
        translations,
        plural_forms: "nplurals=2; plural=(n > 1);".to_string(),
        locale: "ach".to_string(),
    }
}