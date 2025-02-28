// Translation file for Romanian (ro)

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("The password is wrong. Try again.", "Parola este incorectă. Încercaţi din nou.");
        m.insert("Password", "Parolă");
        m.insert("%s shared the folder %s with you", "%s a partajat directorul %s cu tine");
        m.insert("%s shared the file %s with you", "%s a partajat fișierul %s cu tine");
        m.insert("Download", "Descarcă");
        m.insert("Upload", "Încărcare");
        m.insert("Cancel upload", "Anulează încărcarea");
        m.insert("No preview available for", "Nici o previzualizare disponibilă pentru ");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1?0:(((n%100>19)||((n%100==0)&&(n!=0)))?2:1));";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}