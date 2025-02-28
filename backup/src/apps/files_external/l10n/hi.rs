use once_cell::sync::Lazy;
use std::collections::HashMap;
use rust_i18n::Plural;

// Translations map
pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Users", "उपयोगकर्ता");
    map
});

// Plural forms definition
pub static PLURAL_FORMS: Plural = Plural {
    nplurals: 2,
    plural_fn: |n| (n != 1) as usize,
};