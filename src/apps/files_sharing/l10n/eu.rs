use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("The password is wrong. Try again.", "Pasahitza ez da egokia. Saiatu berriro.");
        m.insert("Password", "Pasahitza");
        m.insert("Sorry, this link doesn't seem to work anymore.", "Barkatu, lotura ez dirudi eskuragarria dagoenik.");
        m.insert("Reasons might be:", "Arrazoiak hurrengoak litezke:");
        m.insert("the item was removed", "fitxategia ezbatua izan da");
        m.insert("the link expired", "lotura iraungi da");
        m.insert("sharing is disabled", "elkarbanatzea ez dago gaituta");
        m.insert("For more info, please ask the person who sent this link.", "Informazio gehiagorako, mesedez eskatu lotura hau bidali zuen pertsonari");
        m.insert("%s shared the folder %s with you", "%sk zurekin %s karpeta elkarbanatu du");
        m.insert("%s shared the file %s with you", "%sk zurekin %s fitxategia elkarbanatu du");
        m.insert("Download", "Deskargatu");
        m.insert("Upload", "Igo");
        m.insert("Cancel upload", "Ezeztatu igoera");
        m.insert("No preview available for", "Ez dago aurrebista eskuragarririk hauentzat ");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}