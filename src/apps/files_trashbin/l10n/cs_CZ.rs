use std::collections::HashMap;
use rust_gettext::Catalog;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Couldn't delete %s permanently", "Nelze trvale odstranit %s");
    translations.insert("Couldn't restore %s", "Nelze obnovit %s");
    translations.insert("Error", "Chyba");
    translations.insert("restored", "obnoveno");
    translations.insert("Nothing in here. Your trash bin is empty!", "Žádný obsah. Váš koš je prázdný.");
    translations.insert("Name", "Název");
    translations.insert("Restore", "Obnovit");
    translations.insert("Deleted", "Smazáno");
    translations.insert("Delete", "Smazat");
    translations.insert("Deleted Files", "Smazané soubory");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;"
}

pub fn create_catalog() -> Catalog {
    let mut catalog = Catalog::new();
    for (key, value) in get_translations() {
        catalog.add_string(key, value);
    }
    catalog.set_plural_forms(get_plural_forms());
    catalog
}