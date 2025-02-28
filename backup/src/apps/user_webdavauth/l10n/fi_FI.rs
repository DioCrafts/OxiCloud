use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "WebDAV-todennus");
        m.insert("Address: ", "Osoite:");
        m.insert(
            "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
            "Käyttäjätiedot lähetetään tähän osoitteeseen. Liitännäinen tarkistaa vastauksen, ja tulkitsee HTTP-tilakoodit 401 ja 403 vääriksi käyttäjätiedoiksi. Kaikki muut vastaukset tulkitaan kelvollisiksi käyttäjätiedoiksi."
        );
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}