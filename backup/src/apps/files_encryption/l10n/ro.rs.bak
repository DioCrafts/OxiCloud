use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Saving...", "Se salvează...");
    map.insert("Encryption", "Încriptare");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=(n==1?0:(((n%100>19)||((n%100==0)&&(n!=0)))?2:1));";