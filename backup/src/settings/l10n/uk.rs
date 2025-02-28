use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Unable to load list from App Store", "Не вдалося завантажити список з App Store");
        m.insert("Authentication error", "Помилка автентифікації");
        m.insert("Group already exists", "Група вже існує");
        m.insert("Unable to add group", "Не вдалося додати групу");
        m.insert("Email saved", "Адресу збережено");
        m.insert("Invalid email", "Невірна адреса");
        m.insert("Unable to delete group", "Не вдалося видалити групу");
        m.insert("Unable to delete user", "Не вдалося видалити користувача");
        m.insert("Language changed", "Мова змінена");
        m.insert("Invalid request", "Некоректний запит");
        m.insert("Admins can't remove themself from the admin group", "Адміністратор не може видалити себе з групи адмінів");
        m.insert("Unable to add user to group %s", "Не вдалося додати користувача у групу %s");
        m.insert("Unable to remove user from group %s", "Не вдалося видалити користувача із групи %s");
        m.insert("Couldn't update app.", "Не вдалося оновити програму. ");
        m.insert("Update to {appversion}", "Оновити до {appversion}");
        m.insert("Disable", "Вимкнути");
        m.insert("Enable", "Включити");
        m.insert("Please wait....", "Зачекайте, будь ласка...");
        m.insert("Updating....", "Оновлюється...");
        m.insert("Error while updating app", "Помилка при оновленні програми");
        m.insert("Error", "Помилка");
        m.insert("Update", "Оновити");
        m.insert("Updated", "Оновлено");
        m.insert("Saving...", "Зберігаю...");
        m.insert("deleted", "видалені");
        m.insert("undo", "відмінити");
        m.insert("Unable to remove user", "Неможливо видалити користувача");
        m.insert("Groups", "Групи");
        m.insert("Group Admin", "Адміністратор групи");
        m.insert("Delete", "Видалити");
        m.insert("add group", "додати групу");
        m.insert("A valid username must be provided", "Потрібно задати вірне ім'я користувача");
        m.insert("Error creating user", "Помилка при створенні користувача");
        m.insert("A valid password must be provided", "Потрібно задати вірний пароль");
        m.insert("__language_name__", "__language_name__");
        m.insert("Security Warning", "Попередження про небезпеку");
        m.insert("Setup Warning", "Попередження при Налаштуванні");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Ваш Web-сервер ще не налаштований належним чином для того, щоб дозволити синхронізацію файлів, через те що інтерфейс WebDAV, здається, зламаний.");
        m.insert("Module 'fileinfo' missing", "Модуль 'fileinfo' відсутній");
        m.insert("The PHP module 'fileinfo' is missing. We strongly recommend to enable this module to get best results with mime-type detection.", "PHP модуль 'fileinfo' відсутній. Ми наполегливо рекомендуємо увімкнути цей модуль, щоб отримати кращі результати при виявленні MIME-типів.");
        m.insert("Locale not working", "Локалізація не працює");
        m.insert("Internet connection not working", "Інтернет-з'єднання не працює");
        m.insert("Cron", "Cron");
        m.insert("Execute one task with each page loaded", "Виконати одне завдання для кожної завантаженої сторінки ");
        m.insert("Sharing", "Спільний доступ");
        m.insert("Enable Share API", "Увімкнути API спільного доступу");
        m.insert("Allow apps to use the Share API", "Дозволити програмам використовувати API спільного доступу");
        m.insert("Allow links", "Дозволити посилання");
        m.insert("Allow users to share items to the public with links", "Дозволити користувачам відкривати спільний доступ до елементів за допомогою посилань");
        m.insert("Allow resharing", "Дозволити перевідкривати спільний доступ");
        m.insert("Allow users to share items shared with them again", "Дозволити користувачам знову відкривати спільний доступ до елементів, які вже відкриті для доступу");
        m.insert("Allow users to share with anyone", "Дозволити користувачам відкривати спільний доступ для всіх");
        m.insert("Allow users to only share with users in their groups", "Дозволити користувачам відкривати спільний доступ лише для користувачів з їхньої групи");
        m.insert("Security", "Безпека");
        m.insert("Enforce HTTPS", "Примусове застосування HTTPS");
        m.insert("Log", "Протокол");
        m.insert("Log level", "Рівень протоколювання");
        m.insert("More", "Більше");
        m.insert("Less", "Менше");
        m.insert("Version", "Версія");
        m.insert("Developed by the <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud community</a>, the <a href=\"https://github.com/owncloud\" target=\"_blank\">source code</a> is licensed under the <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.", "Розроблено <a href=\"http://ownCloud.org/contact\" target=\"_blank\">ownCloud громадою</a>, <a href=\"https://github.com/owncloud\" target=\"_blank\">вихідний код</a> має ліцензію <a href=\"http://www.gnu.org/licenses/agpl-3.0.html\" target=\"_blank\"><abbr title=\"Affero General Public License\">AGPL</abbr></a>.");
        m.insert("Add your App", "Додати свою програму");
        m.insert("More Apps", "Більше програм");
        m.insert("Select an App", "Вибрати додаток");
        m.insert("See application page at apps.owncloud.com", "Перегляньте сторінку програм на apps.owncloud.com");
        m.insert("<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>", "<span class=\"licence\"></span>-licensed by <span class=\"author\"></span>");
        m.insert("User Documentation", "Документація Користувача");
        m.insert("Administrator Documentation", "Документація Адміністратора");
        m.insert("Online Documentation", "Он-Лайн Документація");
        m.insert("Forum", "Форум");
        m.insert("Bugtracker", "БагТрекер");
        m.insert("Commercial Support", "Комерційна підтримка");
        m.insert("Get the apps to sync your files", "Отримати додатки для синхронізації ваших файлів");
        m.insert("Show First Run Wizard again", "Показувати Майстер Налаштувань знову");
        m.insert("You have used <strong>%s</strong> of the available <strong>%s</strong>", "Ви використали <strong>%s</strong> із доступних <strong>%s</strong>");
        m.insert("Password", "Пароль");
        m.insert("Your password was changed", "Ваш пароль змінено");
        m.insert("Unable to change your password", "Не вдалося змінити Ваш пароль");
        m.insert("Current password", "Поточний пароль");
        m.insert("New password", "Новий пароль");
        m.insert("Change password", "Змінити пароль");
        m.insert("Email", "Ел.пошта");
        m.insert("Your email address", "Ваша адреса електронної пошти");
        m.insert("Fill in an email address to enable password recovery", "Введіть адресу електронної пошти для відновлення паролю");
        m.insert("Language", "Мова");
        m.insert("Help translate", "Допомогти з перекладом");
        m.insert("WebDAV", "WebDAV");
        m.insert("Encryption", "Шифрування");
        m.insert("Login Name", "Ім'я Логіну");
        m.insert("Create", "Створити");
        m.insert("Default Storage", "сховище за замовчуванням");
        m.insert("Unlimited", "Необмежено");
        m.insert("Other", "Інше");
        m.insert("Username", "Ім'я користувача");
        m.insert("Storage", "Сховище");
        m.insert("set new password", "встановити новий пароль");
        m.insert("Default", "За замовчуванням");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}