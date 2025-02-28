// apps/files_versions/l10n/lb.rs

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("History", "Historique");
        m.insert("Files Versioning", "Fichier's Versionéierung ");
        m.insert("Enable", "Aschalten");
        m
    };
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}