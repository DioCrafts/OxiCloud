use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("The password is wrong. Try again.", "Passordet er feil. Prøv på nytt.");
        m.insert("Password", "Passord");
        m.insert("%s shared the folder %s with you", "%s delte mappen %s med deg");
        m.insert("%s shared the file %s with you", "%s delte filen %s med deg");
        m.insert("Download", "Last ned");
        m.insert("Upload", "Last opp");
        m.insert("Cancel upload", "Avbryt opplasting");
        m.insert("No preview available for", "Forhåndsvisning ikke tilgjengelig for");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}