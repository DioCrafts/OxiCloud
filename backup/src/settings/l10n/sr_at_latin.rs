use std::collections::HashMap;
use rust_i18n::i18n;

/// Serbian Latin (sr@latin) translation strings
pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Authentication error", "Greška pri autentifikaciji");
    translations.insert("Language changed", "Jezik je izmenjen");
    translations.insert("Invalid request", "Neispravan zahtev");
    translations.insert("Error", "Greška");
    translations.insert("Groups", "Grupe");
    translations.insert("Delete", "Obriši");
    translations.insert("Security Warning", "Bezbednosno upozorenje");
    translations.insert("Select an App", "Izaberite program");
    translations.insert("Password", "Lozinka");
    translations.insert("Unable to change your password", "Ne mogu da izmenim vašu lozinku");
    translations.insert("Current password", "Trenutna lozinka");
    translations.insert("New password", "Nova lozinka");
    translations.insert("Change password", "Izmeni lozinku");
    translations.insert("Email", "E-mail");
    translations.insert("Language", "Jezik");
    translations.insert("Create", "Napravi");
    translations.insert("Other", "Drugo");
    translations.insert("Username", "Korisničko ime");
    translations
}

/// Serbian Latin (sr@latin) plural forms configuration
pub fn get_plural_forms() -> &'static str {
    "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);"
}

// Register translations with the i18n system
pub fn register_translations() {
    i18n!(
        translations: get_translations(),
        plural_forms: get_plural_forms(),
        locale: "sr@latin"
    );
}