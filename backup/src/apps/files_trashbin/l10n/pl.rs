use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Nie można trwale usunąć %s");
        m.insert("Couldn't restore %s", "Nie można przywrócić %s");
        m.insert("Error", "Błąd");
        m.insert("restored", "przywrócony");
        m.insert("Nothing in here. Your trash bin is empty!", "Nic tu nie ma. Twój kosz jest pusty!");
        m.insert("Name", "Nazwa");
        m.insert("Restore", "Przywróć");
        m.insert("Deleted", "Usunięte");
        m.insert("Delete", "Usuń");
        m.insert("Deleted Files", "Usunięte pliki");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(&key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}