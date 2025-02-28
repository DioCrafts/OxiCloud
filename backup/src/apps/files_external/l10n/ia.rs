use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Translation data for Interlingua (ia)
pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Groups", "Gruppos");
    map.insert("Users", "Usatores");
    map.insert("Delete", "Deler");
    map
});

/// Plural forms rule for Interlingua: nplurals=2; plural=(n != 1);
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}