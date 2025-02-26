use std::collections::HashMap;
use rust_gettext::Catalog;

pub fn get_translation_strings() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Help", "Adjuta");
    translations.insert("Personal", "Personal");
    translations.insert("Settings", "Configurationes");
    translations.insert("Users", "Usatores");
    translations.insert("Admin", "Administration");
    translations.insert("web services under your control", "servicios web sub tu controlo");
    translations.insert("Files", "Files");
    translations.insert("Text", "Texto");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_plural_translations() -> HashMap<&'static str, Vec<&'static str>> {
    let mut plural_translations = HashMap::new();
    plural_translations.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
    plural_translations.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
    plural_translations.insert("_%n day go_::_%n days ago_", vec!["", ""]);
    plural_translations.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
    plural_translations
}

pub fn create_catalog() -> Catalog {
    let mut catalog = Catalog::new();
    catalog.set_plural_form_count(2);
    catalog.set_plural_form_expression("n != 1");
    
    for (key, value) in get_translation_strings() {
        catalog.add_simple_translation(key, value);
    }
    
    for (key, values) in get_plural_translations() {
        let parts: Vec<&str> = key.split("::").collect();
        if parts.len() == 2 {
            catalog.add_plural_translation(parts[0], parts[1], &values);
        }
    }
    
    catalog
}