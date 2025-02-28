use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("This share is password-protected", "Deze share is met een wachtwoord beveiligd");
        m.insert("The password is wrong. Try again.", "Wachtwoord ongeldig. Probeer het nogmaals.");
        m.insert("Password", "Wachtwoord");
        m.insert("Sorry, this link doesn't seem to work anymore.", "Sorry, deze link lijkt niet meer in gebruik te zijn.");
        m.insert("Reasons might be:", "Redenen kunnen zijn:");
        m.insert("the item was removed", "bestand was verwijderd");
        m.insert("the link expired", "de link is verlopen");
        m.insert("sharing is disabled", "delen is uitgeschakeld");
        m.insert("For more info, please ask the person who sent this link.", "Voor meer informatie, neem contact op met de persoon die u deze link heeft gestuurd.");
        m.insert("%s shared the folder %s with you", "%s deelt de map %s met u");
        m.insert("%s shared the file %s with you", "%s deelt het bestand %s met u");
        m.insert("Download", "Downloaden");
        m.insert("Upload", "Uploaden");
        m.insert("Cancel upload", "Upload afbreken");
        m.insert("No preview available for", "Geen voorbeeldweergave beschikbaar voor");
        m.insert("Direct link", "Directe link");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}