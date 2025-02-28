use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Password", "Wagwoord");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";