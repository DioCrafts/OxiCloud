use std::collections::HashMap;
use rust_gettext::prelude::*;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Help", "Hjelp");
    translations.insert("Personal", "Personlig");
    translations.insert("Settings", "Innstillinger");
    translations.insert("Users", "Brukere");
    translations.insert("Admin", "Admin");
    translations.insert("web services under your control", "web tjenester du kontrollerer");
    translations.insert("ZIP download is turned off.", "ZIP-nedlasting av avslått");
    translations.insert("Files need to be downloaded one by one.", "Filene må lastes ned en om gangen");
    translations.insert("Back to Files", "Tilbake til filer");
    translations.insert("Selected files too large to generate zip file.", "De valgte filene er for store til å kunne generere ZIP-fil");
    translations.insert("Application is not enabled", "Applikasjon er ikke påslått");
    translations.insert("Authentication error", "Autentikasjonsfeil");
    translations.insert("Token expired. Please reload page.", "Symbol utløpt. Vennligst last inn siden på nytt.");
    translations.insert("Files", "Filer");
    translations.insert("Text", "Tekst");
    translations.insert("Images", "Bilder");
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Din nettservev er ikke konfigurert korrekt for filsynkronisering. WebDAV ser ut til å ikke funkere.");
    translations.insert("Please double check the <a href='%s'>installation guides</a>.", "Vennligst dobbelsjekk <a href='%s'>installasjonsguiden</a>.");
    translations.insert("Could not find category \"%s\"", "Kunne ikke finne kategori \"%s\"");
    translations.insert("seconds ago", "sekunder siden");
    translations.insert("_%n minute ago_::_%n minutes ago_", "");
    translations.insert("_%n hour ago_::_%n hours ago_", "");
    translations.insert("today", "i dag");
    translations.insert("yesterday", "i går");
    translations.insert("_%n day go_::_%n days ago_", "");
    translations.insert("last month", "forrige måned");
    translations.insert("_%n month ago_::_%n months ago_", "");
    translations.insert("last year", "forrige år");
    translations.insert("years ago", "år siden");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Estructura para representar la configuración regional nb_NO
pub struct NbNO;

impl Locale for NbNO {
    fn translations(&self) -> HashMap<&'static str, &'static str> {
        get_translations()
    }
    
    fn plural_forms(&self) -> &'static str {
        get_plural_forms()
    }
}