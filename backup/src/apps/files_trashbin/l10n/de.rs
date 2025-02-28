use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Konnte %s nicht dauerhaft löschen");
        m.insert("Couldn't restore %s", "Konnte %s nicht wiederherstellen");
        m.insert("Error", "Fehler");
        m.insert("restored", "Wiederhergestellt");
        m.insert("Nothing in here. Your trash bin is empty!", "Nichts zu löschen, der Papierkorb ist leer!");
        m.insert("Name", "Name");
        m.insert("Restore", "Wiederherstellen");
        m.insert("Deleted", "gelöscht");
        m.insert("Delete", "Löschen");
        m.insert("Deleted Files", "Gelöschte Dateien");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}