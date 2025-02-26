use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Configuration", "Innstillingar");
        m.insert("Groups", "Grupper");
        m.insert("Users", "Brukarar");
        m.insert("Delete", "Slett");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}