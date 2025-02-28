use std::collections::HashMap;
use rust_gettext::prelude::*;

pub fn get_translation_data() -> (HashMap<&'static str, &'static str>, HashMap<&'static str, Vec<&'static str>>, &'static str) {
    let mut translations = HashMap::new();
    translations.insert("Settings", "ਸੈਟਿੰਗ");
    translations.insert("Files", "ਫਾਇਲਾਂ");
    translations.insert("seconds ago", "ਸਕਿੰਟ ਪਹਿਲਾਂ");
    translations.insert("today", "ਅੱਜ");
    translations.insert("yesterday", "ਕੱਲ੍ਹ");
    translations.insert("last month", "ਪਿਛਲੇ ਮਹੀਨੇ");
    translations.insert("last year", "ਪਿਛਲੇ ਸਾਲ");
    translations.insert("years ago", "ਸਾਲਾਂ ਪਹਿਲਾਂ");

    let mut plural_translations = HashMap::new();
    plural_translations.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
    plural_translations.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
    plural_translations.insert("_%n day go_::_%n days ago_", vec!["", ""]);
    plural_translations.insert("_%n month ago_::_%n months ago_", vec!["", ""]);

    let plural_forms = "nplurals=2; plural=(n != 1);";

    (translations, plural_translations, plural_forms)
}

pub fn register_pa_translations() -> Result<(), TranslationError> {
    let (translations, plural_translations, plural_forms) = get_translation_data();
    let mut catalog = Catalog::new("pa");
    
    for (key, value) in translations {
        catalog.add_simple_translation(key, value);
    }
    
    for (key, values) in plural_translations {
        let parts: Vec<&str> = key.split("::").collect();
        if parts.len() == 2 {
            let singular = parts[0].trim_matches(|c| c == '_');
            let plural = parts[1].trim_matches(|c| c == '_');
            catalog.add_plural_translation(singular, plural, &values);
        }
    }
    
    catalog.set_plural_form_expression(plural_forms);
    
    register_catalog(catalog)?;
    Ok(())
}