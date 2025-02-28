use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Bulgarian (Bulgaria) localization strings
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Sunday", "Неделя");
    map.insert("Monday", "Понеделник");
    map.insert("Tuesday", "Вторник");
    map.insert("Wednesday", "Сряда");
    map.insert("Thursday", "Четвъртък");
    map.insert("Friday", "Петък");
    map.insert("Saturday", "Събота");
    map.insert("January", "Януари");
    map.insert("February", "Февруари");
    map.insert("March", "Март");
    map.insert("April", "Април");
    map.insert("May", "Май");
    map.insert("June", "Юни");
    map.insert("July", "Юли");
    map.insert("August", "Август");
    map.insert("September", "Септември");
    map.insert("October", "Октомври");
    map.insert("November", "Ноември");
    map.insert("December", "Декември");
    map.insert("Settings", "Настройки");
    map.insert("seconds ago", "преди секунди");
    map.insert("today", "днес");
    map.insert("yesterday", "вчера");
    map.insert("last month", "последният месец");
    map.insert("last year", "последната година");
    map.insert("years ago", "последните години");
    map.insert("Yes", "Да");
    map.insert("No", "Не");
    map.insert("Ok", "Добре");
    map.insert("Cancel", "Отказ");
    map.insert("Share", "Споделяне");
    map.insert("Error", "Грешка");
    map.insert("Password", "Парола");
    map.insert("create", "създаване");
    map.insert("Warning", "Внимание");
    map.insert("Delete", "Изтриване");
    map.insert("Add", "Добавяне");
    map.insert("You will receive a link to reset your password via Email.", "Ще получите връзка за нулиране на паролата Ви.");
    map.insert("Username", "Потребител");
    map.insert("Your password was reset", "Вашата парола е нулирана");
    map.insert("New password", "Нова парола");
    map.insert("Reset password", "Нулиране на парола");
    map.insert("Personal", "Лични");
    map.insert("Users", "Потребители");
    map.insert("Apps", "Приложения");
    map.insert("Admin", "Админ");
    map.insert("Help", "Помощ");
    map.insert("Access forbidden", "Достъпът е забранен");
    map.insert("Cloud not found", "облакът не намерен");
    map.insert("Create an <strong>admin account</strong>", "Създаване на <strong>админ профил</strong>");
    map.insert("Advanced", "Разширено");
    map.insert("Data folder", "Директория за данни");
    map.insert("Configure the database", "Конфигуриране на базата");
    map.insert("will be used", "ще се ползва");
    map.insert("Database user", "Потребител за базата");
    map.insert("Database password", "Парола за базата");
    map.insert("Database name", "Име на базата");
    map.insert("Database host", "Хост за базата");
    map.insert("Finish setup", "Завършване на настройките");
    map.insert("Log out", "Изход");
    map.insert("Lost your password?", "Забравена парола?");
    map.insert("remember", "запомни");
    map.insert("Log in", "Вход");
    map
});

/// Pluralization forms for Bulgarian language
pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Stores translations for plural forms
pub static PLURAL_TRANSLATIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
    map.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
    map.insert("_%n day ago_::_%n days ago_", vec!["", ""]);
    map.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
    map.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", ""]);
    map
});

/// Get translation for a given key
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

/// Get plural translation for a given key and count
pub fn get_plural_translation(key: &str, count: i64) -> Option<&'static str> {
    PLURAL_TRANSLATIONS.get(key).and_then(|forms| {
        let plural_index = if count != 1 { 1 } else { 0 };
        forms.get(plural_index).copied()
    })
}