use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::locale::LocaleId;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%s group found_::_%s groups found_", vec![""]);
        m.insert("_%s user found_::_%s users found_", vec![""]);
        m.insert("Help", vec!["შველა"]);
        m.insert("Password", vec!["პაროლი"]);
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).and_then(|translations| translations.first().copied())
}

pub fn get_plural_translation(key: &str, count: usize) -> Option<&'static str> {
    TRANSLATIONS.get(key).and_then(|translations| {
        let plural_index = match *PLURAL_FORMS {
            "nplurals=1; plural=0;" => 0,
            _ => 0 // Default to first form if plural rule isn't recognized
        };
        translations.get(plural_index).copied()
    })
}

pub fn initialize_locale() -> LocaleId {
    LocaleId::new("ka")
}