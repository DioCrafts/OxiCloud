use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Nevarēja pilnībā izdzēst %s");
        m.insert("Couldn't restore %s", "Nevarēja atjaunot %s");
        m.insert("Error", "Kļūda");
        m.insert("restored", "atjaunots");
        m.insert("Nothing in here. Your trash bin is empty!", "Šeit nekā nav. Jūsu miskaste ir tukša!");
        m.insert("Name", "Nosaukums");
        m.insert("Restore", "Atjaunot");
        m.insert("Deleted", "Dzēsts");
        m.insert("Delete", "Dzēst");
        m.insert("Deleted Files", "Dzēstās datnes");
        m
    };
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n != 0 ? 1 : 2);"
}