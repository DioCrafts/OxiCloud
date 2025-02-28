use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_gettext::Catalog;

lazy_static! {
    static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "WebDAV-aŭtentigo");
        m.insert("Address: ", "Adreso:");
        m
    };
}

pub fn get_translation(text: &str) -> &'static str {
    TRANSLATIONS.get(text).unwrap_or(&text)
}

pub fn get_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn init_catalog() -> Catalog {
    let mut catalog = Catalog::new();
    catalog.set_plural_form_function(Box::new(|n| if n != 1 { 1 } else { 0 }));
    
    for (msgid, msgstr) in TRANSLATIONS.iter() {
        catalog.add_message(*msgid, *msgstr);
    }
    
    catalog
}