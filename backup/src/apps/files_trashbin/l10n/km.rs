use once_cell::sync::Lazy;
use std::collections::HashMap;
use rust_i18n::locale::LocaleDefinition;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Delete", "លុប");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";

pub fn get_locale_definition() -> LocaleDefinition {
    LocaleDefinition {
        translations: TRANSLATIONS.clone(),
        plural_forms: PLURAL_FORMS.to_string(),
    }
}