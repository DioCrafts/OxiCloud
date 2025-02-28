use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Неможливо видалити %s назавжди");
        m.insert("Couldn't restore %s", "Неможливо відновити %s");
        m.insert("Error", "Помилка");
        m.insert("restored", "відновлено");
        m.insert("Nothing in here. Your trash bin is empty!", "Нічого немає. Ваший кошик для сміття пустий!");
        m.insert("Name", "Ім'я");
        m.insert("Restore", "Відновити");
        m.insert("Deleted", "Видалено");
        m.insert("Delete", "Видалити");
        m.insert("Deleted Files", "Видалено Файлів");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}