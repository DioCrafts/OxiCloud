use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Password", "Lozinka");
    map.insert("Download", "Preuzmi");
    map.insert("Upload", "Pošalji");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";