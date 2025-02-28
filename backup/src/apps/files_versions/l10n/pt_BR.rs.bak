use std::collections::HashMap;
use rust_gettext::Catalog;

// Brazilian Portuguese translations
pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    translations.insert("Could not revert: %s".to_string(), "Impossível reverter: %s".to_string());
    translations.insert("Versions".to_string(), "Versões".to_string());
    translations.insert("Failed to revert {file} to revision {timestamp}.".to_string(), "Falha ao reverter {file} para a revisão {timestamp}.".to_string());
    translations.insert("More versions...".to_string(), "Mais versões...".to_string());
    translations.insert("No other versions available".to_string(), "Nenhuma outra versão disponível".to_string());
    translations.insert("Restore".to_string(), "Restaurar".to_string());
    translations
}

// Initialize the pt_BR catalog with plural forms
pub fn init_catalog() -> Catalog {
    let mut catalog = Catalog::new("pt_BR".to_string());
    catalog.set_plural_forms("nplurals=2; plural=(n > 1);".to_string());
    
    for (key, value) in get_translations() {
        catalog.add_message(key, value);
    }
    
    catalog
}