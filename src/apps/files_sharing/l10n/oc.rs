use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password", "Senhal");
        m.insert("Download", "Avalcarga");
        m.insert("Upload", "Amontcarga");
        m.insert("Cancel upload", " Anulla l'amontcargar");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}