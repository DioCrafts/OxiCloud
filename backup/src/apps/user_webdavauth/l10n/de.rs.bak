use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "WebDAV Authentifikation");
        m.insert("Address: ", "Addresse: ");
        m.insert("The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.", 
                "Die Benutzerdaten werden an diese Adresse gesendet. Dieses Plugin prüft die Antwort und wird die HTTP-Statuscodes 401 und 403 als ungültige Daten interpretieren und alle anderen Antworten als gültige Daten.");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}