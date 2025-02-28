use std::collections::HashMap;
use rust_gettext::Catalog;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Saving...", "Зберігаю...");
        m.insert("personal settings", "особисті налаштування");
        m.insert("Encryption", "Шифрування");
        m.insert("Change Password", "Змінити Пароль");
        m
    };
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);"
}

pub fn init_catalog() -> Catalog {
    let mut catalog = Catalog::new();
    for (key, value) in TRANSLATIONS.iter() {
        catalog.add_message(key, value);
    }
    catalog.set_plural_forms(get_plural_forms());
    catalog
}