use once_cell::sync::Lazy;
use std::collections::HashMap;

// Static translations map
pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Saving...", "Yn cadw...");
    map.insert("Encryption", "Amgryptiad");
    map
});

// Plural forms rule
pub fn get_plural_form(n: usize) -> usize {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else if n != 8 && n != 11 {
        2
    } else {
        3
    }
}

pub const PLURAL_FORMS: &str = "nplurals=4; plural=(n==1) ? 0 : (n==2) ? 1 : (n != 8 && n != 11) ? 2 : 3;";