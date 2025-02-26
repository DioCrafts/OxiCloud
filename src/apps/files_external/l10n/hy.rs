use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Delete", "Ջնջել");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";