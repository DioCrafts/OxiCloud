use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::Plural;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Folder name", "సంచయం పేరు");
        map.insert("Users", "వాడుకరులు");
        map.insert("Delete", "తొలగించు");
        map
    };
}

pub fn get_plural_form(n: usize) -> Plural {
    if n != 1 {
        Plural::Other
    } else {
        Plural::One
    }
}

pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}