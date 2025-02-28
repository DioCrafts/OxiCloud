use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("ru");

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Couldn't delete %s permanently", "%s не может быть удалён навсегда");
    translations.insert("Couldn't restore %s", "%s не может быть восстановлен");
    translations.insert("Error", "Ошибка");
    translations.insert("restored", "восстановлен");
    translations.insert("Nothing in here. Your trash bin is empty!", "Здесь ничего нет. Ваша корзина пуста!");
    translations.insert("Name", "Имя");
    translations.insert("Restore", "Восстановить");
    translations.insert("Deleted", "Удалён");
    translations.insert("Delete", "Удалить");
    translations.insert("Deleted Files", "Удаленные файлы");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);"
}