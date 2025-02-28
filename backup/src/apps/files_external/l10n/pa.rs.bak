use once_cell::sync::Lazy;
use std::collections::HashMap;

// Static translation map
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Groups", "ਗਰੁੱਪ");
    map.insert("Delete", "ਹਟਾਓ");
    map
});

// Pluralization rule
pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

// Helper function to get translation
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}