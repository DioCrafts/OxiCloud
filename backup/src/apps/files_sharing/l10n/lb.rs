use std::collections::HashMap;
use rust_i18n::i18n;

// Language module for Luxembourgish (lb)
pub fn lb_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert(
        "The password is wrong. Try again.".to_string(),
        "Den Passwuert ass incorrect. Probeier ed nach eng keier.".to_string(),
    );
    translations.insert(
        "Password".to_string(),
        "Passwuert".to_string(),
    );
    translations.insert(
        "%s shared the folder %s with you".to_string(),
        "%s huet den Dossier %s mad der gedeelt".to_string(),
    );
    translations.insert(
        "%s shared the file %s with you".to_string(),
        "%s deelt den Fichier %s mad dir".to_string(),
    );
    translations.insert(
        "Download".to_string(),
        "Download".to_string(),
    );
    translations.insert(
        "Upload".to_string(),
        "Eroplueden".to_string(),
    );
    translations.insert(
        "Cancel upload".to_string(),
        "Upload ofbriechen".to_string(),
    );
    translations.insert(
        "No preview available for".to_string(),
        "Keeng Preview do fir".to_string(),
    );
    
    translations
}

pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

// Register the language in the i18n system
pub fn register_lb_language() {
    i18n!("lb", translations = lb_translations(), plural_form = get_plural_form);
}