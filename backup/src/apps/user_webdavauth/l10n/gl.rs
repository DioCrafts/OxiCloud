use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "Autenticación WebDAV");
        m.insert("Address: ", "Enderezo:");
        m.insert("The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.", "As credenciais do usuario serán enviadas a este enderezo. Este engadido comproba a resposta e interpretará os códigos de estado 401 e 403 como credenciais incorrectas, e todas as outras respostas como credenciais correctas.");
        m
    };
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}