use std::collections::HashMap;
use once_cell::sync::Lazy;

// Bulgarian (bg_BG) translation strings
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Authentication error", "Възникна проблем с идентификацията");
    m.insert("Group already exists", "Групата вече съществува");
    m.insert("Unable to add group", "Невъзможно добавяне на група");
    m.insert("Email saved", "Email адреса е записан");
    m.insert("Invalid email", "Невалиден Email адрес");
    m.insert("Unable to delete group", "Невъзможно изтриване на група");
    m.insert("Unable to delete user", "Невъзможно изтриване на потребител");
    m.insert("Language changed", "Езикът е променен");
    m.insert("Invalid request", "Невалидна заявка");
    m.insert("Update to {appversion}", "Обновяване до {appversion}");
    m.insert("Disable", "Изключено");
    m.insert("Enable", "Включено");
    m.insert("Please wait....", "Моля почакайте....");
    m.insert("Updating....", "Обновява се...");
    m.insert("Error", "Грешка");
    m.insert("Update", "Обновяване");
    m.insert("Updated", "Обновено");
    m.insert("Saving...", "Записване...");
    m.insert("deleted", "изтрито");
    m.insert("undo", "възтановяване");
    m.insert("Groups", "Групи");
    m.insert("Delete", "Изтриване");
    m.insert("add group", "нова група");
    m.insert("__language_name__", "__language_name__");
    m.insert(
        "Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.",
        "Вашият web сървър все още не е удачно настроен да позволява синхронизация на файлове, защото WebDAV интерфейсът изглежда не работи."
    );
    m.insert("Cron", "Крон");
    m.insert("Sharing", "Споделяне");
    m.insert("More", "Още");
    m.insert("Less", "По-малко");
    m.insert("Version", "Версия");
    m.insert("Add your App", "Добавете Ваше приложение");
    m.insert("More Apps", "Още приложения");
    m.insert("Select an App", "Изберете приложение");
    m.insert("User Documentation", "Потребителска документация");
    m.insert("Administrator Documentation", "Административна документация");
    m.insert("Online Documentation", "Документация");
    m.insert("Forum", "Форум");
    m.insert("Bugtracker", "Докладвани грешки");
    m.insert("Commercial Support", "Платена поддръжка");
    m.insert("Show First Run Wizard again", "Покажи настройките за първоначално зареждане отново");
    m.insert("Password", "Парола");
    m.insert("Unable to change your password", "Промяната на паролата не беше извършена");
    m.insert("Current password", "Текуща парола");
    m.insert("New password", "Нова парола");
    m.insert("Change password", "Промяна на паролата");
    m.insert("Email", "E-mail");
    m.insert("Your email address", "Вашия email адрес");
    m.insert("Fill in an email address to enable password recovery", "Въведете е-поща за възстановяване на паролата");
    m.insert("Language", "Език");
    m.insert("Help translate", "Помогнете с превода");
    m.insert("WebDAV", "WebDAV");
    m.insert("Encryption", "Криптиране");
    m.insert("Login Name", "Потребител");
    m.insert("Create", "Създаване");
    m.insert("Default Storage", "Хранилище по подразбиране");
    m.insert("Unlimited", "Неограничено");
    m.insert("Other", "Други");
    m.insert("Username", "Потребител");
    m.insert("Storage", "Хранилище");
    m.insert("Default", "По подразбиране");
    m
});

// Plural forms rule for Bulgarian
pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_forms() -> &'static str {
    PLURAL_FORMS
}