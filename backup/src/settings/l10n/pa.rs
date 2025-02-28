// pa.rs

use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Translations for Punjabi language
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Language changed", "ਭਾਸ਼ਾ ਬਦਲੀ");
    map.insert("Disable", "ਬੰਦ");
    map.insert("Enable", "ਚਾਲੂ");
    map.insert("Please wait....", "...ਉਡੀਕੋ ਜੀ");
    map.insert("Updating....", "...ਅੱਪਡੇਟ ਕੀਤਾ ਜਾ ਰਿਹਾ ਹੈ");
    map.insert("Error", "ਗਲਤੀ");
    map.insert("Updated", "ਅੱਪਡੇਟ ਕੀਤਾ");
    map.insert("Saving...", "...ਸੰਭਾਲਿਆ ਜਾ ਰਿਹਾ ਹੈ");
    map.insert("deleted", "ਹਟਾਈ");
    map.insert("undo", "ਵਾਪਸ");
    map.insert("Groups", "ਗਰੁੱਪ");
    map.insert("Group Admin", "ਗਰੁੱਪ ਐਡਮਿਨ");
    map.insert("Delete", "ਹਟਾਓ");
    map.insert("add group", "ਗਰੁੱਪ ਸ਼ਾਮਲ");
    map.insert("__language_name__", "__ਭਾਸ਼ਾ_ਨਾਂ__");
    map.insert("Security Warning", "ਸੁਰੱਖਿਆ ਚੇਤਾਵਨੀ");
    map.insert("Setup Warning", "ਸੈਟਅੱਪ ਚੇਤਾਵਨੀ");
    map.insert("Password", "ਪਾਸਵਰ");
    map.insert("Change password", "ਪਾਸਵਰਡ ਬਦਲੋ");
    map.insert("Username", "ਯੂਜ਼ਰ-ਨਾਂ");
    map
});

/// Plural forms rule for Punjabi language
pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Get a translation by key
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}