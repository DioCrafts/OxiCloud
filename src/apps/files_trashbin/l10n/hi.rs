use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Error", "त्रुटि");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";