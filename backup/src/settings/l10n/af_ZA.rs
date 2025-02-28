use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_gettext::Plural;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Password", "Wagwoord");
        map.insert("New password", "Nuwe wagwoord");
        map.insert("Username", "Gebruikersnaam");
        map
    };

    pub static ref PLURAL_FORMS: Plural = Plural::new(2, "(n != 1)").unwrap();
}