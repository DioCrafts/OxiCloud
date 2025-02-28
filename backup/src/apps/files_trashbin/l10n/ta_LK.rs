use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Error", "வழு");
    map.insert("Name", "பெயர்");
    map.insert("Delete", "நீக்குக");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";