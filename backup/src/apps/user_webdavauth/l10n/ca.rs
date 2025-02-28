use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "Autenticació WebDAV");
        m.insert("Address: ", "Adreça:");
        m.insert(
            "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
            "Les credencials d'usuari s'enviaran a aquesta adreça. Aquest connector comprova la resposta i interpreta els codis d'estat 401 i 403 com a credencials no vàlides, i qualsevol altra resposta com a credencials vàlides."
        );
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}