// Definición de las traducciones para el idioma georgiano (ka)
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Traducciones disponibles para el idioma georgiano
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Password", "პაროლი");
    map
});

/// Información sobre formas plurales para el idioma georgiano
pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";