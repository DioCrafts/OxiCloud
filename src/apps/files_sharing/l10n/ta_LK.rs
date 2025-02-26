use std::collections::HashMap;
use rust_gettext::Catalog;

pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    translations.insert("Password".to_string(), "கடவுச்சொல்".to_string());
    translations.insert("%s shared the folder %s with you".to_string(), "%s கோப்புறையானது %s உடன் பகிரப்பட்டது".to_string());
    translations.insert("%s shared the file %s with you".to_string(), "%s கோப்பானது %s உடன் பகிரப்பட்டது".to_string());
    translations.insert("Download".to_string(), "பதிவிறக்குக".to_string());
    translations.insert("Upload".to_string(), "பதிவேற்றுக".to_string());
    translations.insert("Cancel upload".to_string(), "பதிவேற்றலை இரத்து செய்க".to_string());
    translations.insert("No preview available for".to_string(), "அதற்கு முன்னோக்கு ஒன்றும் இல்லை".to_string());
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn create_catalog() -> Catalog {
    let mut catalog = Catalog::new();
    for (key, value) in get_translations() {
        catalog.add_message(key, value);
    }
    catalog.set_plural_forms(get_plural_forms());
    catalog
}