use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "Autenticación de WebDAV");
        m.insert("Address: ", "Dirección:");
        m.insert(
            "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
            "Las credenciales del usuario serán enviadas a esta dirección. Este plug-in verificará la respuesta e interpretará los códigos de estado HTTP 401 y 403 como credenciales inválidas y cualquier otra respuesta como válida."
        );
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}