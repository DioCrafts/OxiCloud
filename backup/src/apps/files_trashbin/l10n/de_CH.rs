use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("de_CH");

pub fn init_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Couldn't delete %s permanently", "Konnte %s nicht dauerhaft löschen");
    translations.insert("Couldn't restore %s", "Konnte %s nicht wiederherstellen");
    translations.insert("Error", "Fehler");
    translations.insert("restored", "Wiederhergestellt");
    translations.insert("Nothing in here. Your trash bin is empty!", "Nichts zu löschen, Ihr Papierkorb ist leer!");
    translations.insert("Name", "Name");
    translations.insert("Restore", "Wiederherstellen");
    translations.insert("Deleted", "Gelöscht");
    translations.insert("Delete", "Löschen");
    translations.insert("Deleted Files", "Gelöschte Dateien");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}