use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "%s jäädavalt kustutamine ebaõnnestus");
        m.insert("Couldn't restore %s", "%s ei saa taastada");
        m.insert("Error", "Viga");
        m.insert("restored", "taastatud");
        m.insert("Nothing in here. Your trash bin is empty!", "Siin pole midagi. Sinu prügikast on tühi!");
        m.insert("Name", "Nimi");
        m.insert("Restore", "Taasta");
        m.insert("Deleted", "Kustutatud");
        m.insert("Delete", "Kustuta");
        m.insert("Deleted Files", "Kustutatud failid");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}