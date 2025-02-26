use std::collections::HashMap;
use rust_i18n::t;

pub fn register_translations() -> (HashMap<String, String>, String) {
    let mut translations = HashMap::new();
    
    translations.insert(
        "Could not revert: %s".to_string(), 
        "Could not revert: %s".to_string()
    );
    translations.insert(
        "Versions".to_string(), 
        "Versions".to_string()
    );
    translations.insert(
        "Failed to revert {file} to revision {timestamp}.".to_string(),
        "Failed to revert {file} to revision {timestamp}.".to_string()
    );
    translations.insert(
        "More versions...".to_string(), 
        "More versions...".to_string()
    );
    translations.insert(
        "No other versions available".to_string(), 
        "No other versions available".to_string()
    );
    translations.insert(
        "Restore".to_string(), 
        "Restore".to_string()
    );
    
    let plural_forms = "nplurals=2; plural=(n != 1);".to_string();
    
    (translations, plural_forms)
}