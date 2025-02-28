use fluent_templates::static_loader;
use fluent_templates::LanguageIdentifier;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use unic_langid::langid;

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en",
    };
}

pub static CA_LANG_ID: Lazy<LanguageIdentifier> = Lazy::new(|| langid!("ca"));

pub fn get_ca_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("This share is password-protected".to_string(), "Aquest compartit està protegit amb contrasenya".to_string());
    translations.insert("The password is wrong. Try again.".to_string(), "la contrasenya és incorrecta. Intenteu-ho de nou.".to_string());
    translations.insert("Password".to_string(), "Contrasenya".to_string());
    translations.insert("Sorry, this link doesn't seem to work anymore.".to_string(), "Aquest enllaç sembla que no funciona.".to_string());
    translations.insert("Reasons might be:".to_string(), "Les raons podrien ser:".to_string());
    translations.insert("the item was removed".to_string(), "l'element ha estat eliminat".to_string());
    translations.insert("the link expired".to_string(), "l'enllaç ha vençut".to_string());
    translations.insert("sharing is disabled".to_string(), "s'ha desactivat la compartició".to_string());
    translations.insert("For more info, please ask the person who sent this link.".to_string(), "Per més informació contacteu amb qui us ha enviat l'enllaç.".to_string());
    translations.insert("%s shared the folder %s with you".to_string(), "%s ha compartit la carpeta %s amb vós".to_string());
    translations.insert("%s shared the file %s with you".to_string(), "%s ha compartit el fitxer %s amb vós".to_string());
    translations.insert("Download".to_string(), "Baixa".to_string());
    translations.insert("Upload".to_string(), "Puja".to_string());
    translations.insert("Cancel upload".to_string(), "Cancel·la la pujada".to_string());
    translations.insert("No preview available for".to_string(), "No hi ha vista prèvia disponible per a".to_string());
    translations.insert("Direct link".to_string(), "Enllaç directe".to_string());
    
    translations
}

pub fn get_ca_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}