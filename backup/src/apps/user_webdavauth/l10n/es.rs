// apps/user_webdavauth/l10n/es.rs

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "Autenticación mediante WevDAV");
        m.insert("Address: ", "Dirección:");
        m.insert(
            "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
            "Las credenciales de usuario se enviarán a esta dirección. Este complemento verifica la respuesta e interpretará los códigos de respuesta HTTP 401 y 403 como credenciales inválidas y todas las otras respuestas como credenciales válidas."
        );
        m
    };
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}