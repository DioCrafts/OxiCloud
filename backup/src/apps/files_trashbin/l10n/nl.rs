use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::i18n;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Kon %s niet permanent verwijderen");
        m.insert("Couldn't restore %s", "Kon %s niet herstellen");
        m.insert("Error", "Fout");
        m.insert("restored", "hersteld");
        m.insert("Nothing in here. Your trash bin is empty!", "Niets te vinden. Uw prullenbak is leeg!");
        m.insert("Name", "Naam");
        m.insert("Restore", "Herstellen");
        m.insert("Deleted", "Verwijderd");
        m.insert("Delete", "Verwijder");
        m.insert("Deleted Files", "Verwijderde bestanden");
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

// Función helper opcional para obtener traducciones
pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}