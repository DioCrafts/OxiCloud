use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Невъзможно перманентното изтриване на %s");
        m.insert("Couldn't restore %s", "Невъзможно възтановяване на %s");
        m.insert("Error", "Грешка");
        m.insert("Nothing in here. Your trash bin is empty!", "Няма нищо. Кофата е празна!");
        m.insert("Name", "Име");
        m.insert("Restore", "Възтановяване");
        m.insert("Deleted", "Изтрито");
        m.insert("Delete", "Изтриване");
        m.insert("Deleted Files", "Изтрити файлове");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}