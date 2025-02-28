use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("This share is password-protected", "Esta partilha está protegida por palavra-chave");
        m.insert("The password is wrong. Try again.", "Password errada, por favor tente de novo");
        m.insert("Password", "Palavra-passe");
        m.insert("Sorry, this link doesn't seem to work anymore.", "Desculpe, mas este link parece não estar a funcionar.");
        m.insert("Reasons might be:", "As razões poderão ser:");
        m.insert("the item was removed", "O item foi removido");
        m.insert("the link expired", "O link expirou");
        m.insert("sharing is disabled", "A partilha está desativada");
        m.insert("For more info, please ask the person who sent this link.", "Para mais informações, por favor questione a pessoa que lhe enviou este link");
        m.insert("%s shared the folder %s with you", "%s partilhou a pasta %s consigo");
        m.insert("%s shared the file %s with you", "%s partilhou o ficheiro %s consigo");
        m.insert("Download", "Transferir");
        m.insert("Upload", "Carregar");
        m.insert("Cancel upload", "Cancelar envio");
        m.insert("No preview available for", "Não há pré-visualização para");
        m.insert("Direct link", "Link direto");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}