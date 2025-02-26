use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("WebDAV Authentication", "WebDAV-Authentifizierung");
    m.insert("Address: ", "Adresse:");
    m.insert("The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.", 
             "Die Benutzerdaten werden an diese Adresse gesendet. Dieses Plugin prüft die Antwort und wird die HTTP-Statuscodes 401 und 403 als ungültige Daten interpretieren und alle anderen Antworten als gültige Daten.");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";