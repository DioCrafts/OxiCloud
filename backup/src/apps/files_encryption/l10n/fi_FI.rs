use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Palautusavain kytketty päälle onnistuneesti");
        m.insert("Password successfully changed.", "Salasana vaihdettiin onnistuneesti.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Salasanan vaihto epäonnistui. Kenties vanha salasana oli väärin.");
        m.insert("Following users are not set up for encryption:", "Seuraavat käyttäjät eivät ole määrittäneet salausta:");
        m.insert("Saving...", "Tallennetaan...");
        m.insert("personal settings", "henkilökohtaiset asetukset");
        m.insert("Encryption", "Salaus");
        m.insert("Recovery key password", "Palautusavaimen salasana");
        m.insert("Enabled", "Käytössä");
        m.insert("Disabled", "Ei käytössä");
        m.insert("Change recovery key password:", "Vaihda palautusavaimen salasana:");
        m.insert("Old Recovery key password", "Vanha palautusavaimen salasana");
        m.insert("New Recovery key password", "Uusi palautusavaimen salasana");
        m.insert("Change Password", "Vaihda salasana");
        m.insert("Old log-in password", "Vanha kirjautumis-salasana");
        m.insert("Current log-in password", "Nykyinen kirjautumis-salasana");
        m.insert("Enable password recovery:", "Ota salasanan palautus käyttöön:");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}