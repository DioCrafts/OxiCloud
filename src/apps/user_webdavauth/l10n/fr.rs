use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(
            "WebDAV Authentication",
            "Authentification WebDAV"
        );
        m.insert(
            "Address: ",
            "Adresse :"
        );
        m.insert(
            "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
            "Les informations de connexion de l'utilisateur seront envoyées à cette adresse. Ce module analyse le code de la réponse HTTP et considère les codes 401 et 403 comme une authentification invalide et tout autre valeur comme une authentification valide."
        );
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}