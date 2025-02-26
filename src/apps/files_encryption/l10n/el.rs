use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password successfully changed.", "Ο κωδικός αλλάχτηκε επιτυχώς.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Αποτυχία αλλαγής κωδικού ίσως ο παλιός κωδικός να μην ήταν σωστός.");
        m.insert("Saving...", "Γίνεται αποθήκευση...");
        m.insert("personal settings", "προσωπικές ρυθμίσεις");
        m.insert("Encryption", "Κρυπτογράφηση");
        m.insert("Enabled", "Ενεργοποιημένο");
        m.insert("Disabled", "Απενεργοποιημένο");
        m.insert("Change Password", "Αλλαγή Κωδικού Πρόσβασης");
        m.insert("File recovery settings updated", "Οι ρυθμίσεις επαναφοράς αρχείων ανανεώθηκαν");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}