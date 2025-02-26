use std::collections::HashMap;
use once_cell::sync::Lazy;

// Translations mapping
pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Save", "Zapisz");
    map
});

// Plural forms formula for Polish
pub const PLURAL_FORMS: &str = "nplurals=3; plural=(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";

// Function to resolve plural form
pub fn get_plural_form(n: usize) -> usize {
    if n == 1 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}

// Utility function to get a translation
pub fn get_translation(key: &str) -> &str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}