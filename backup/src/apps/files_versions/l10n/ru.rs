use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Не может быть возвращён: %s");
        m.insert("Versions", "Версии");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Не удалось возвратить {file} к ревизии {timestamp}.");
        m.insert("More versions...", "Ещё версии...");
        m.insert("No other versions available", "Других версий не доступно");
        m.insert("Restore", "Восстановить");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}