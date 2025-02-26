use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Saving...", "ساقلاۋاتىدۇ…");
    map.insert("Encryption", "شىفىرلاش");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";