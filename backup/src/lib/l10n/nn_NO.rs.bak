use std::collections::HashMap;
use fluent_templates::Loader;
use rust_fluent::fluent_bundle::FluentValue;
use unic_langid::LanguageIdentifier;

pub struct NnNOResource {}

impl NnNOResource {
    pub fn translations() -> HashMap<&'static str, &'static str> {
        let mut translations = HashMap::new();
        translations.insert("Help", "Hjelp");
        translations.insert("Personal", "Personleg");
        translations.insert("Settings", "Innstillingar");
        translations.insert("Users", "Brukarar");
        translations.insert("Admin", "Administrer");
        translations.insert("Unknown filetype", "Ukjend filtype");
        translations.insert("Invalid image", "Ugyldig bilete");
        translations.insert("web services under your control", "Vev tjenester under din kontroll");
        translations.insert("Authentication error", "Feil i autentisering");
        translations.insert("Files", "Filer");
        translations.insert("Text", "Tekst");
        translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", 
                           "Tenaren din er ikkje enno rett innstilt til å tilby filsynkronisering sidan WebDAV-grensesnittet ser ut til å vera øydelagt.");
        translations.insert("Please double check the <a href='%s'>installation guides</a>.", 
                           "Ver venleg og dobbeltsjekk <a href='%s'>installasjonsrettleiinga</a>.");
        translations.insert("seconds ago", "sekund sidan");
        translations.insert("today", "i dag");
        translations.insert("yesterday", "i går");
        translations.insert("last month", "førre månad");
        translations.insert("last year", "i fjor");
        translations.insert("years ago", "år sidan");
        translations
    }

    pub fn plural_forms() -> &'static str {
        "nplurals=2; plural=(n != 1);"
    }

    pub fn plurals() -> HashMap<&'static str, Vec<&'static str>> {
        let mut plurals = HashMap::new();
        plurals.insert("_%n minute ago_::_%n minutes ago_", vec!["", "%n minutt sidan"]);
        plurals.insert("_%n hour ago_::_%n hours ago_", vec!["", "%n timar sidan"]);
        plurals.insert("_%n day go_::_%n days ago_", vec!["", "%n dagar sidan"]);
        plurals.insert("_%n month ago_::_%n months ago_", vec!["", "%n månadar sidan"]);
        plurals
    }

    pub fn get_nn_no_lang_id() -> LanguageIdentifier {
        "nn-NO".parse().expect("Failed to parse language identifier")
    }
}