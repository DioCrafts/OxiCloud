use std::collections::HashMap;
use rust_i18n::i18n;

// Define the Galician (gl) translations
pub fn register_gl_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Couldn't delete %s permanently".to_string(), "Non foi posíbel eliminar %s permanente".to_string());
    translations.insert("Couldn't restore %s".to_string(), "Non foi posíbel restaurar %s".to_string());
    translations.insert("Error".to_string(), "Erro".to_string());
    translations.insert("restored".to_string(), "restaurado".to_string());
    translations.insert("Nothing in here. Your trash bin is empty!".to_string(), "Aquí non hai nada. O cesto do lixo está baleiro!".to_string());
    translations.insert("Name".to_string(), "Nome".to_string());
    translations.insert("Restore".to_string(), "Restablecer".to_string());
    translations.insert("Deleted".to_string(), "Eliminado".to_string());
    translations.insert("Delete".to_string(), "Eliminar".to_string());
    translations.insert("Deleted Files".to_string(), "Ficheiros eliminados".to_string());
    
    translations
}

// Define the plural forms rule for Galician
pub fn gl_plural_forms(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}