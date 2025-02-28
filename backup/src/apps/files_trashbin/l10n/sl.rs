use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Datoteke %s ni mogoče dokončno izbrisati.");
        m.insert("Couldn't restore %s", "Ni mogoče obnoviti %s");
        m.insert("Error", "Napaka");
        m.insert("Nothing in here. Your trash bin is empty!", "Mapa smeti je prazna.");
        m.insert("Name", "Ime");
        m.insert("Restore", "Obnovi");
        m.insert("Deleted", "Izbrisano");
        m.insert("Delete", "Izbriši");
        m.insert("Deleted Files", "Izbrisane datoteke");
        m
    };
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=4; plural=(n%100==1 ? 0 : n%100==2 ? 1 : n%100==3 || n%100==4 ? 2 : 3);"
}