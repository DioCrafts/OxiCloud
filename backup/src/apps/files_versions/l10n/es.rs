use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "No se puede revertir: %s");
        m.insert("Versions", "Revisiones");
        m.insert("Failed to revert {file} to revision {timestamp}.", "No se ha podido revertir {archivo} a revisión {timestamp}.");
        m.insert("More versions...", "Más versiones...");
        m.insert("No other versions available", "No hay otras versiones disponibles");
        m.insert("Restore", "Recuperar");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}