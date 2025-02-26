use std::collections::HashMap;
use phf::phf_map;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Help", "Допомога");
    translations.insert("Personal", "Особисте");
    translations.insert("Settings", "Налаштування");
    translations.insert("Users", "Користувачі");
    translations.insert("Admin", "Адмін");
    translations.insert("web services under your control", "підконтрольні Вам веб-сервіси");
    translations.insert("ZIP download is turned off.", "ZIP завантаження вимкнено.");
    translations.insert("Files need to be downloaded one by one.", "Файли повинні бути завантаженні послідовно.");
    translations.insert("Back to Files", "Повернутися до файлів");
    translations.insert("Selected files too large to generate zip file.", "Вибрані фали завеликі для генерування zip файлу.");
    translations.insert("Application is not enabled", "Додаток не увімкнений");
    translations.insert("Authentication error", "Помилка автентифікації");
    translations.insert("Token expired. Please reload page.", "Строк дії токена скінчився. Будь ласка, перезавантажте сторінку.");
    translations.insert("Files", "Файли");
    translations.insert("Text", "Текст");
    translations.insert("Images", "Зображення");
    translations.insert("%s enter the database username.", "%s введіть ім'я користувача бази даних.");
    translations.insert("%s enter the database name.", "%s введіть назву бази даних.");
    translations.insert("%s you may not use dots in the database name", "%s не можна використовувати крапки в назві бази даних");
    translations.insert("MS SQL username and/or password not valid: %s", "MS SQL ім'я користувача та/або пароль не дійсні: %s");
    translations.insert("You need to enter either an existing account or the administrator.", "Вам потрібно ввести або існуючий обліковий запис або administrator.");
    translations.insert("MySQL username and/or password not valid", "MySQL ім'я користувача та/або пароль не дійсні");
    translations.insert("DB Error: \"%s\"", "Помилка БД: \"%s\"");
    translations.insert("Offending command was: \"%s\"", "Команда, що викликала проблему: \"%s\"");
    translations.insert("MySQL user '%s'@'localhost' exists already.", "Користувач MySQL '%s'@'localhost' вже існує.");
    translations.insert("Drop this user from MySQL", "Видалити цього користувача з MySQL");
    translations.insert("MySQL user '%s'@'%%' already exists", "Користувач MySQL '%s'@'%%' вже існує");
    translations.insert("Drop this user from MySQL.", "Видалити цього користувача з MySQL.");
    translations.insert("Oracle username and/or password not valid", "Oracle ім'я користувача та/або пароль не дійсні");
    translations.insert("Offending command was: \"%s\", name: %s, password: %s", "Команда, що викликала проблему: \"%s\", ім'я: %s, пароль: %s");
    translations.insert("PostgreSQL username and/or password not valid", "PostgreSQL ім'я користувача та/або пароль не дійсні");
    translations.insert("Set an admin username.", "Встановіть ім'я адміністратора.");
    translations.insert("Set an admin password.", "Встановіть пароль адміністратора.");
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Ваш Web-сервер ще не налаштований належним чином для того, щоб дозволити синхронізацію файлів, через те що інтерфейс WebDAV, здається, зламаний.");
    translations.insert("Please double check the <a href='%s'>installation guides</a>.", "Будь ласка, перевірте <a href='%s'>інструкції по встановленню</a>.");
    translations.insert("Could not find category \"%s\"", "Не вдалося знайти категорію \"%s\"");
    translations.insert("seconds ago", "секунди тому");
    translations.insert("today", "сьогодні");
    translations.insert("yesterday", "вчора");
    translations.insert("last month", "минулого місяця");
    translations.insert("last year", "минулого року");
    translations.insert("years ago", "роки тому");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);"
}

pub struct PluralForms {
    pub minute: [&'static str; 3],
    pub hour: [&'static str; 3],
    pub day: [&'static str; 3],
    pub month: [&'static str; 3],
}

pub fn get_plural_translations() -> PluralForms {
    PluralForms {
        minute: ["", "", ""],
        hour: ["", "", ""],
        day: ["", "", ""],
        month: ["", "", ""],
    }
}