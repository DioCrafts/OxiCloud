use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("pt_PT");

pub fn load_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Couldn't delete %s permanently".to_string(), "Não foi possível eliminar %s de forma permanente".to_string());
    translations.insert("Couldn't restore %s".to_string(), "Não foi possível restaurar %s".to_string());
    translations.insert("Error".to_string(), "Erro".to_string());
    translations.insert("restored".to_string(), "Restaurado".to_string());
    translations.insert("Nothing in here. Your trash bin is empty!".to_string(), "Não hà ficheiros. O lixo está vazio!".to_string());
    translations.insert("Name".to_string(), "Nome".to_string());
    translations.insert("Restore".to_string(), "Restaurar".to_string());
    translations.insert("Deleted".to_string(), "Apagado".to_string());
    translations.insert("Delete".to_string(), "Eliminar".to_string());
    translations.insert("Deleted Files".to_string(), "Ficheiros Apagados".to_string());
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}