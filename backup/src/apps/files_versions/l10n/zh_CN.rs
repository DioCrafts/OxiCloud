use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Translations for zh_CN locale
pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Could not revert: %s", "无法恢复: %s");
    map.insert("Versions", "版本");
    map.insert("Restore", "恢复");
    map
});

/// Plural forms configuration for zh_CN locale
pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";