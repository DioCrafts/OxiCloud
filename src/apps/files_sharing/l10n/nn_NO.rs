use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("The password is wrong. Try again.", "Passordet er gale. Prøv igjen.");
        m.insert("Password", "Passord");
        m.insert("Sorry, this link doesn't seem to work anymore.", "Orsak, denne lenkja fungerer visst ikkje lenger.");
        m.insert("Reasons might be:", "Moglege grunnar:");
        m.insert("the item was removed", "fila/mappa er fjerna");
        m.insert("the link expired", "lenkja har gått ut på dato");
        m.insert("sharing is disabled", "deling er slått av");
        m.insert("For more info, please ask the person who sent this link.", "Spør den som sende deg lenkje om du vil ha meir informasjon.");
        m.insert("%s shared the folder %s with you", "%s delte mappa %s med deg");
        m.insert("%s shared the file %s with you", "%s delte fila %s med deg");
        m.insert("Download", "Last ned");
        m.insert("Upload", "Last opp");
        m.insert("Cancel upload", "Avbryt opplasting");
        m.insert("No preview available for", "Inga førehandsvising tilgjengeleg for");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}