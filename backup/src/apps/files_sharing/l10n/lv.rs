use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password", "Parole");
        m.insert("%s shared the folder %s with you", "%s ar jums dalījās ar mapi %s");
        m.insert("%s shared the file %s with you", "%s ar jums dalījās ar datni %s");
        m.insert("Download", "Lejupielādēt");
        m.insert("Upload", "Augšupielādēt");
        m.insert("Cancel upload", "Atcelt augšupielādi");
        m.insert("No preview available for", "Nav pieejams priekšskatījums priekš");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n != 0 ? 1 : 2);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}