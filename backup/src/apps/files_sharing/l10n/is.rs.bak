use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password", "Lykilorð");
        m.insert("%s shared the folder %s with you", "%s deildi möppunni %s með þér");
        m.insert("%s shared the file %s with you", "%s deildi skránni %s með þér");
        m.insert("Download", "Niðurhal");
        m.insert("Upload", "Senda inn");
        m.insert("Cancel upload", "Hætta við innsendingu");
        m.insert("No preview available for", "Yfirlit ekki í boði fyrir");
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn init_translations() {
    // Initialize translations if needed
}