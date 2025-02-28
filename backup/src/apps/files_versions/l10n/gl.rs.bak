use std::collections::HashMap;
use once_cell::sync::Lazy;

// GL translations
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Could not revert: %s", "Non foi posíbel reverter: %s");
    m.insert("Versions", "Versións");
    m.insert("Failed to revert {file} to revision {timestamp}.", "Non foi posíbel reverter {file} á revisión {timestamp}.");
    m.insert("More versions...", "Máis versións...");
    m.insert("No other versions available", "Non hai outras versións dispoñíbeis");
    m.insert("Restore", "Restablecer");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";