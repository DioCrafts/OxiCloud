use once_cell::sync::Lazy;
use std::collections::HashMap;

// Translations
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Versions", "பதிப்புகள்");
    map
});

// Plural forms rule
pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";