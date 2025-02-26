use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("The password is wrong. Try again.", "Kodeordet er forkert. Prøv igen.");
        m.insert("Password", "Kodeord");
        m.insert("Sorry, this link doesn't seem to work anymore.", "Desværre, dette link ser ikke ud til at fungerer længere.");
        m.insert("Reasons might be:", "Årsagen kan være:");
        m.insert("the item was removed", "Filen blev fjernet");
        m.insert("the link expired", "linket udløb");
        m.insert("sharing is disabled", "deling er deaktiveret");
        m.insert("For more info, please ask the person who sent this link.", "For yderligere information, kontakt venligst personen der sendte linket. ");
        m.insert("%s shared the folder %s with you", "%s delte mappen %s med dig");
        m.insert("%s shared the file %s with you", "%s delte filen %s med dig");
        m.insert("Download", "Download");
        m.insert("Upload", "Upload");
        m.insert("Cancel upload", "Fortryd upload");
        m.insert("No preview available for", "Forhåndsvisning ikke tilgængelig for");
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