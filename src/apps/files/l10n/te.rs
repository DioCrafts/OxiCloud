use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::i18n;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Delete permanently", "శాశ్వతంగా తొలగించు");
        m.insert("Error", "పొరపాటు");
        m.insert("Name", "పేరు");
        m.insert("Size", "పరిమాణం");
        m.insert("Save", "భద్రపరచు");
        m.insert("Folder", "సంచయం");
        m.insert("Delete", "తొలగించు");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

// Definición de funciones para manejar los plurales
pub fn translate_folders(n: usize) -> String {
    match n {
        1 => "1 folder".to_string(),
        _ => format!("{} folders", n),
    }
}

pub fn translate_files(n: usize) -> String {
    match n {
        1 => "1 file".to_string(),
        _ => format!("{} files", n),
    }
}

pub fn translate_uploading(n: usize) -> String {
    match n {
        1 => "Uploading 1 file".to_string(),
        _ => format!("Uploading {} files", n),
    }
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}