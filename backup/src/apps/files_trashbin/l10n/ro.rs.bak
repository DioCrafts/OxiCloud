use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Error", "Eroare");
    map.insert("Name", "Nume");
    map.insert("Delete", "Șterge");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=(n==1?0:(((n%100>19)||((n%100==0)&&(n!=0)))?2:1));";