use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("bg_BG");

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Access granted", "Достъпът е даден");
    translations.insert("Grant access", "Даване на достъп");
    translations.insert("External Storage", "Външно хранилище");
    translations.insert("Folder name", "Име на папката");
    translations.insert("Configuration", "Конфигурация");
    translations.insert("Options", "Опции");
    translations.insert("Applicable", "Приложимо");
    translations.insert("None set", "Няма избрано");
    translations.insert("All Users", "Всички потребители");
    translations.insert("Groups", "Групи");
    translations.insert("Users", "Потребители");
    translations.insert("Delete", "Изтриване");
    translations.insert("Enable User External Storage", "Вкл. на поддръжка за външно потр. хранилище");
    translations.insert("Allow users to mount their own external storage", "Позволено е на потребителите да ползват тяхно лично външно хранилище");
    translations.insert("SSL root certificates", "SSL основни сертификати");
    translations.insert("Import Root Certificate", "Импортиране на основен сертификат");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}