use std::collections::HashMap;
use once_cell::sync::Lazy;

/// WebDAV authentication translations for ka_GE (Georgian)
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("WebDAV Authentication", "WebDAV აუთენთიფიკაცია");
    map
});

/// Plural forms definition for ka_GE language
pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";