use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Versions", "وه‌شان");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";