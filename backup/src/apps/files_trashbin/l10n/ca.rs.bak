use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "No s'ha pogut esborrar permanentment %s");
        m.insert("Couldn't restore %s", "No s'ha pogut restaurar %s");
        m.insert("Error", "Error");
        m.insert("restored", "restaurat");
        m.insert("Nothing in here. Your trash bin is empty!", "La paperera està buida!");
        m.insert("Name", "Nom");
        m.insert("Restore", "Recupera");
        m.insert("Deleted", "Eliminat");
        m.insert("Delete", "Esborra");
        m.insert("Deleted Files", "Fitxers eliminats");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}