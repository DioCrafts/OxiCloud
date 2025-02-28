use std::collections::HashMap;
use rust_gettext::prelude::*;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Couldn't delete %s permanently", "Nepavyko negrįžtamai ištrinti %s");
    translations.insert("Couldn't restore %s", "Nepavyko atkurti %s");
    translations.insert("Error", "Klaida");
    translations.insert("restored", "atstatyta");
    translations.insert("Nothing in here. Your trash bin is empty!", "Nieko nėra. Jūsų šiukšliadėžė tuščia!");
    translations.insert("Name", "Pavadinimas");
    translations.insert("Restore", "Atstatyti");
    translations.insert("Deleted", "Ištrinti");
    translations.insert("Delete", "Ištrinti");
    translations.insert("Deleted Files", "Ištrinti failai");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && (n%100<10 || n%100>=20) ? 1 : 2);"
}

#[derive(Clone)]
pub struct LtLtLocale;

impl Locale for LtLtLocale {
    fn id(&self) -> &'static str {
        "lt_LT"
    }

    fn translations(&self) -> HashMap<&'static str, &'static str> {
        get_translations()
    }

    fn plural_forms(&self) -> &'static str {
        get_plural_forms()
    }
}