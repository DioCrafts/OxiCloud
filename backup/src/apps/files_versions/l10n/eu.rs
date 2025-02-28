use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Ezin izan da leheneratu: %s");
        m.insert("Versions", "Bertsioak");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Errore bat izan da {fitxategia} {timestamp} bertsiora leheneratzean.");
        m.insert("More versions...", "Bertsio gehiago...");
        m.insert("No other versions available", "Ez dago bertsio gehiago eskuragarri");
        m.insert("Restore", "Berrezarri");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

// Función para facilitar la traducción
pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(&key)
}

// Función para traducción con formato
pub fn translate_fmt(key: &str, params: &[&str]) -> String {
    let template = translate(key);
    if params.is_empty() {
        return template.to_string();
    }
    
    let mut result = template.to_string();
    for (i, param) in params.iter().enumerate() {
        result = result.replace(&format!("%{}", i + 1), param);
        result = result.replace("%s", param); // Para el primer %s encontrado
    }
    result
}