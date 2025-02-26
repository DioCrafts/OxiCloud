/// Russian translation file
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("This share is password-protected", "Эта шара защищена паролем");
        m.insert("The password is wrong. Try again.", "Неверный пароль. Попробуйте еще раз.");
        m.insert("Password", "Пароль");
        m.insert("Sorry, this link doesn't seem to work anymore.", "К сожалению, эта ссылка, похоже не будет работать больше.");
        m.insert("Reasons might be:", "Причиной может быть:");
        m.insert("the item was removed", "объект был удалён");
        m.insert("the link expired", "срок ссылки истёк");
        m.insert("sharing is disabled", "обмен отключен");
        m.insert("For more info, please ask the person who sent this link.", "Для получения дополнительной информации, пожалуйста, спросите того кто отослал данную ссылку.");
        m.insert("%s shared the folder %s with you", "%s открыл доступ к папке %s для Вас");
        m.insert("%s shared the file %s with you", "%s открыл доступ к файлу %s для Вас");
        m.insert("Download", "Скачать");
        m.insert("Upload", "Загрузка");
        m.insert("Cancel upload", "Отмена загрузки");
        m.insert("No preview available for", "Предпросмотр недоступен для");
        m.insert("Direct link", "Прямая ссылка");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}

/// Translates a string to Russian
pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}

/// Gets plural form for Russian based on the given count
pub fn get_plural_form(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}