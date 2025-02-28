use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Kunne ikke slette %s permanent");
        m.insert("Couldn't restore %s", "Kunne ikke gendanne %s");
        m.insert("Error", "Fejl");
        m.insert("restored", "Gendannet");
        m.insert("Nothing in here. Your trash bin is empty!", "Intet at se her. Din papirkurv er tom!");
        m.insert("Name", "Navn");
        m.insert("Restore", "Gendan");
        m.insert("Deleted", "Slettet");
        m.insert("Delete", "Slet");
        m.insert("Deleted Files", "Slettede filer");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}