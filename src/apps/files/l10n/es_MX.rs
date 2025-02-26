use std::collections::HashMap;
use rust_i18n::i18n;

// Definición de las traducciones
i18n!("es_MX", {
    "folder": {
        "one": "{} carpeta",
        "other": "{} carpetas"
    },
    "file": {
        "one": "{} archivo",
        "other": "{} archivos"
    },
    "uploading_file": {
        "one": "Subiendo {} archivo",
        "other": "Subiendo {} archivos"
    }
});

// La función para plural en español
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

pub fn initialize_translations() -> HashMap<String, Vec<String>> {
    let mut translations = HashMap::new();
    
    translations.insert("_%n folder_::_%n folders_".to_string(), vec!["".to_string(), "".to_string()]);
    translations.insert("_%n file_::_%n files_".to_string(), vec!["".to_string(), "".to_string()]);
    translations.insert("_Uploading %n file_::_Uploading %n files_".to_string(), vec!["".to_string(), "".to_string()]);
    
    translations
}