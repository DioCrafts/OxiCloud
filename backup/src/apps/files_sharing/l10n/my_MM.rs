use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Password", "စကားဝှက်");
    m.insert("Download", "ဒေါင်းလုတ်");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";