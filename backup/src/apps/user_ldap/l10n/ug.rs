use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_gettext::Catalog;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Deletion failed", "ئۆچۈرۈش مەغلۇپ بولدى");
        map.insert("Error", "خاتالىق");
        map.insert("_%s group found_::_%s groups found_", "");
        map.insert("_%s user found_::_%s users found_", "");
        map.insert("Save", "ساقلا");
        map.insert("Help", "ياردەم");
        map.insert("Host", "باش ئاپپارات");
        map.insert("Port", "ئېغىز");
        map.insert("Password", "ئىم");
        map.insert("Connection Settings", "باغلىنىش تەڭشىكى");
        map.insert("Configuration Active", "سەپلىمە ئاكتىپ");
        map.insert("User Login Filter", "ئىشلەتكۈچى تىزىمغا كىرىش سۈزگۈچى");
        map
    };

    pub static ref CATALOG: Catalog = {
        let mut catalog = Catalog::new();
        catalog.set_plural_forms("nplurals=1; plural=0;");
        
        for (key, value) in TRANSLATIONS.iter() {
            if key.contains("::") {
                // Handle plural forms
                let singular_plural: Vec<&str> = key.split("::").collect();
                catalog.add_plural(singular_plural[0], singular_plural[1], &[*value]);
            } else {
                catalog.add(key, value);
            }
        }
        
        catalog
    };
}

pub fn get_translation(text: &str) -> &'static str {
    TRANSLATIONS.get(text).copied().unwrap_or(text)
}

pub fn get_plural_translation(singular: &str, plural: &str, n: u32) -> &'static str {
    let key = format!("_{}_::_{}_", singular, plural);
    match TRANSLATIONS.get(key.as_str()) {
        Some(translation) if !translation.is_empty() => translation,
        _ => if n == 1 { singular } else { plural }
    }
}