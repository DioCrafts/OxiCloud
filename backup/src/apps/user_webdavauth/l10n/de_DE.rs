use std::collections::HashMap;
use rust_i18n::t;

pub fn init_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("WebDAV Authentication", "WebDAV-Authentifizierung");
    translations.insert("Address: ", "Adresse:");
    translations.insert(
        "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
        "Die Benutzerdaten werden an diese Adresse gesendet. Dieses Plugin prüft die Antwort und wird die HTTP-Statuscodes 401 und 403 als ungültige Daten interpretieren und alle anderen Antworten als gültige Daten."
    );
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}