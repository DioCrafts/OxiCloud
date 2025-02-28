// hr.rs

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Unable to load list from App Store", "Nemogićnost učitavanja liste sa Apps Stora");
        m.insert("Authentication error", "Greška kod autorizacije");
        m.insert("Email saved", "Email spremljen");
        m.insert("Invalid email", "Neispravan email");
        m.insert("Language changed", "Jezik promijenjen");
        m.insert("Invalid request", "Neispravan zahtjev");
        m.insert("Disable", "Isključi");
        m.insert("Enable", "Uključi");
        m.insert("Error", "Greška");
        m.insert("Saving...", "Spremanje...");
        m.insert("deleted", "izbrisano");
        m.insert("undo", "vrati");
        m.insert("Groups", "Grupe");
        m.insert("Group Admin", "Grupa Admin");
        m.insert("Delete", "Obriši");
        m.insert("__language_name__", "__ime_jezika__");
        m.insert("Cron", "Cron");
        m.insert("Log", "dnevnik");
        m.insert("More", "više");
        m.insert("Add your App", "Dodajte vašu aplikaciju");
        m.insert("Select an App", "Odaberite Aplikaciju");
        m.insert("See application page at apps.owncloud.com", "Pogledajte stranicu s aplikacijama na apps.owncloud.com");
        m.insert("Password", "Lozinka");
        m.insert("Unable to change your password", "Nemoguće promijeniti lozinku");
        m.insert("Current password", "Trenutna lozinka");
        m.insert("New password", "Nova lozinka");
        m.insert("Change password", "Izmjena lozinke");
        m.insert("Email", "e-mail adresa");
        m.insert("Your email address", "Vaša e-mail adresa");
        m.insert("Fill in an email address to enable password recovery", "Ispunite vase e-mail adresa kako bi se omogućilo oporavak lozinke");
        m.insert("Language", "Jezik");
        m.insert("Help translate", "Pomoć prevesti");
        m.insert("Create", "Izradi");
        m.insert("Other", "ostali");
        m.insert("Username", "Korisničko ime");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2;";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}