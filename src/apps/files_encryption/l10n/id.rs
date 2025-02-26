use std::collections::HashMap;
use once_cell::sync::Lazy;

// Define las traducciones como un HashMap estático
pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Saving...", "Menyimpan...");
    map.insert("Encryption", "Enkripsi");
    map
});

// Define la forma plural
pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";