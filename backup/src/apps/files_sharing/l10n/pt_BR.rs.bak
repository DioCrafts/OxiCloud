use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("This share is password-protected", "Este compartilhamento esta protegido por senha");
        map.insert("The password is wrong. Try again.", "Senha incorreta. Tente novamente.");
        map.insert("Password", "Senha");
        map.insert("Sorry, this link doesn't seem to work anymore.", "Desculpe, este link parece não mais  funcionar.");
        map.insert("Reasons might be:", "As razões podem ser:");
        map.insert("the item was removed", "o item foi removido");
        map.insert("the link expired", "o link expirou");
        map.insert("sharing is disabled", "compartilhamento está desativada");
        map.insert("For more info, please ask the person who sent this link.", "Para mais informações, por favor, pergunte a pessoa que enviou este link.");
        map.insert("%s shared the folder %s with you", "%s compartilhou a pasta %s com você");
        map.insert("%s shared the file %s with you", "%s compartilhou o arquivo %s com você");
        map.insert("Download", "Baixar");
        map.insert("Upload", "Upload");
        map.insert("Cancel upload", "Cancelar upload");
        map.insert("No preview available for", "Nenhuma visualização disponível para");
        map.insert("Direct link", "Link direto");
        map
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn format_translation(key: &str, args: &[&str]) -> Option<String> {
    get_translation(key).map(|template| {
        let mut result = template.to_string();
        for arg in args {
            result = result.replacen("%s", arg, 1);
        }
        result
    })
}