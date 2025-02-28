use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "WebDAV autentikacija");
        m.insert("Address: ", "Adresas:");
        m.insert("The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.", "Naudotojo duomenys bus nusiųsti šiuo adresu. Šis įskiepis patikrins gautą atsakymą ir interpretuos HTTP būsenos kodą 401 ir 403 kaip negaliojančius duomenis, ir visus kitus gautus atsakymus kaip galiojančius duomenis. ");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && (n%100<10 || n%100>=20) ? 1 : 2);";
}

pub fn init_i18n() {
    // Inicializar traducción con la información definida arriba
    // Esta función sería llamada durante la inicialización de la aplicación
}