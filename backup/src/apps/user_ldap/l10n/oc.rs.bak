use std::collections::HashMap;
use rust_i18n::i18n;

// Definición de traducciones para OC (occitano)
pub fn load_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    translations.insert("Deletion failed".to_string(), "Fracàs d'escafatge".to_string());
    translations.insert("Error".to_string(), "Error".to_string());
    translations.insert("Save".to_string(), "Enregistra".to_string());
    translations.insert("Help".to_string(), "Ajuda".to_string());
    translations.insert("Password".to_string(), "Senhal".to_string());
    translations
}

pub fn load_plurals() -> HashMap<String, Vec<String>> {
    let mut plurals = HashMap::new();
    plurals.insert("_%s group found_::_%s groups found_".to_string(), vec!["".to_string(), "".to_string()]);
    plurals.insert("_%s user found_::_%s users found_".to_string(), vec!["".to_string(), "".to_string()]);
    plurals
}

pub fn get_plural_form() -> String {
    "nplurals=2; plural=(n > 1);".to_string()
}

// Función para obtener una traducción con formato de plural
pub fn translate_plural(key: &str, count: i64) -> String {
    let plurals = load_plurals();
    let plural_form = if count > 1 { 1 } else { 0 };
    
    if let Some(forms) = plurals.get(key) {
        if let Some(form) = forms.get(plural_form) {
            return form.replace("%s", &count.to_string());
        }
    }
    
    key.to_string()
}

// Función para obtener una traducción simple
pub fn translate(key: &str) -> String {
    let translations = load_translations();
    translations.get(key).cloned().unwrap_or_else(|| key.to_string())
}