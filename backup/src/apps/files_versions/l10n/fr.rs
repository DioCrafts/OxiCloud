use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Impossible de restaurer %s");
        m.insert("Versions", "Versions");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Échec du retour du fichier {file} à la révision {timestamp}.");
        m.insert("More versions...", "Plus de versions...");
        m.insert("No other versions available", "Aucune autre version disponible");
        m.insert("Restore", "Restaurer");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}