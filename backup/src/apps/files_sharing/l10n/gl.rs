use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("locales");

#[allow(dead_code)]
pub fn load_gl_translations() -> (HashMap<String, String>, String) {
    let mut translations = HashMap::new();
    
    translations.insert(
        "This share is password-protected".to_string(),
        "Esta compartición está protexida con contrasinal".to_string(),
    );
    translations.insert(
        "The password is wrong. Try again.".to_string(),
        "O contrasinal é incorrecto. Ténteo de novo.".to_string(),
    );
    translations.insert(
        "Password".to_string(),
        "Contrasinal".to_string(),
    );
    translations.insert(
        "Sorry, this link doesn't seem to work anymore.".to_string(),
        "Semella que esta ligazón non funciona.".to_string(),
    );
    translations.insert(
        "Reasons might be:".to_string(),
        "As razóns poderían ser:".to_string(),
    );
    translations.insert(
        "the item was removed".to_string(),
        "o elemento foi retirado".to_string(),
    );
    translations.insert(
        "the link expired".to_string(),
        "a ligazón caducou".to_string(),
    );
    translations.insert(
        "sharing is disabled".to_string(),
        "foi desactivada a compartición".to_string(),
    );
    translations.insert(
        "For more info, please ask the person who sent this link.".to_string(),
        "Para obter máis información, pregúntelle á persoa que lle enviou a ligazón.".to_string(),
    );
    translations.insert(
        "%s shared the folder %s with you".to_string(),
        "%s compartiu o cartafol %s con vostede".to_string(),
    );
    translations.insert(
        "%s shared the file %s with you".to_string(),
        "%s compartiu o ficheiro %s con vostede".to_string(),
    );
    translations.insert(
        "Download".to_string(),
        "Descargar".to_string(),
    );
    translations.insert(
        "Upload".to_string(),
        "Enviar".to_string(),
    );
    translations.insert(
        "Cancel upload".to_string(),
        "Cancelar o envío".to_string(),
    );
    translations.insert(
        "No preview available for".to_string(),
        "Sen vista previa dispoñíbel para".to_string(),
    );
    translations.insert(
        "Direct link".to_string(),
        "Ligazón directa".to_string(),
    );

    let plural_forms = "nplurals=2; plural=(n != 1);".to_string();

    (translations, plural_forms)
}