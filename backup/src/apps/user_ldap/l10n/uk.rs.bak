use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::i18n;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Failed to delete the server configuration", "Не вдалося видалити конфігурацію сервера");
        map.insert("The configuration is valid and the connection could be established!", "Конфігурація вірна і зв'язок може бути встановлений ​​!");
        map.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Конфігурація вірна, але встановити зв'язок не вдалося. Будь ласка, перевірте налаштування сервера і облікові дані.");
        map.insert("Deletion failed", "Видалення не було виконано");
        map.insert("Take over settings from recent server configuration?", "Застосувати налаштування  з останньої конфігурації сервера ?");
        map.insert("Keep settings?", "Зберегти налаштування ?");
        map.insert("Cannot add server configuration", "Неможливо додати конфігурацію сервера");
        map.insert("Success", "Успіх");
        map.insert("Error", "Помилка");
        map.insert("Select groups", "Оберіть групи");
        map.insert("Connection test succeeded", "Перевірка з'єднання пройшла успішно");
        map.insert("Connection test failed", "Перевірка з'єднання завершилась неуспішно");
        map.insert("Do you really want to delete the current Server Configuration?", "Ви дійсно бажаєте видалити поточну конфігурацію сервера ?");
        map.insert("Confirm Deletion", "Підтвердіть Видалення");
        map.insert("Save", "Зберегти");
        map.insert("Test Configuration", "Тестове налаштування");
        map.insert("Help", "Допомога");
        map.insert("Add Server Configuration", "Додати налаштування Сервера");
        map.insert("Host", "Хост");
        map.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Можна не вказувати протокол, якщо вам не потрібен SSL. Тоді почніть з ldaps://");
        map.insert("Port", "Порт");
        map.insert("User DN", "DN Користувача");
        map.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "DN клієнтського користувача для прив'язки, наприклад: uid=agent,dc=example,dc=com. Для анонімного доступу, залиште DN і Пароль порожніми.");
        map.insert("Password", "Пароль");
        map.insert("For anonymous access, leave DN and Password empty.", "Для анонімного доступу, залиште DN і Пароль порожніми.");
        map.insert("One Base DN per line", "Один Base DN на одній строчці");
        map.insert("You can specify Base DN for users and groups in the Advanced tab", "Ви можете задати Базовий DN для користувачів і груп на вкладинці Додатково");
        map.insert("Back", "Назад");
        map.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Увага:</ b> Потрібний модуль PHP LDAP не встановлено, базова програма працювати не буде. Будь ласка, зверніться до системного адміністратора, щоб встановити його.");
        map.insert("Connection Settings", "Налаштування З'єднання");
        map.insert("Configuration Active", "Налаштування Активне");
        map.insert("When unchecked, this configuration will be skipped.", "Якщо \"галочка\" знята, ця конфігурація буде пропущена.");
        map.insert("User Login Filter", "Фільтр Користувачів, що під'єднуються");
        map.insert("Backup (Replica) Host", "Сервер для резервних копій");
        map.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Вкажіть додатковий резервний сервер. Він повинен бути копією головного LDAP/AD сервера.");
        map.insert("Backup (Replica) Port", "Порт сервера для резервних копій");
        map.insert("Disable Main Server", "Вимкнути Головний Сервер");
        map.insert("Case insensitve LDAP server (Windows)", "Нечутливий до регістру LDAP сервер (Windows)");
        map.insert("Turn off SSL certificate validation.", "Вимкнути перевірку SSL сертифіката.");
        map.insert("Cache Time-To-Live", "Час актуальності Кеша");
        map.insert("in seconds. A change empties the cache.", "в секундах. Зміна очищує кеш.");
        map.insert("Directory Settings", "Налаштування Каталога");
        map.insert("User Display Name Field", "Поле, яке відображає Ім'я Користувача");
        map.insert("Base User Tree", "Основне Дерево Користувачів");
        map.insert("One User Base DN per line", "Один Користувач Base DN на одній строчці");
        map.insert("User Search Attributes", "Пошукові Атрибути Користувача");
        map.insert("Optional; one attribute per line", "Додатково; один атрибут на строчку");
        map.insert("Group Display Name Field", "Поле, яке відображає Ім'я Групи");
        map.insert("Base Group Tree", "Основне Дерево Груп");
        map.insert("One Group Base DN per line", "Одна Група Base DN на одній строчці");
        map.insert("Group Search Attributes", "Пошукові Атрибути Групи");
        map.insert("Group-Member association", "Асоціація Група-Член");
        map.insert("Special Attributes", "Спеціальні Атрибути");
        map.insert("Quota Field", "Поле Квоти");
        map.insert("Quota Default", "Квота за замовчанням");
        map.insert("in bytes", "в байтах");
        map.insert("Email Field", "Поле Ел. пошти");
        map.insert("User Home Folder Naming Rule", "Правило іменування домашньої теки користувача");
        map.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Залиште порожнім для імені користувача (за замовчанням). Інакше, вкажіть атрибут LDAP/AD.");
        map
    };
}

// Plural forms handler
pub fn get_plural_form(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}

pub fn plural_format(n: i64, forms: [&str; 3]) -> String {
    let form = get_plural_form(n);
    forms[form].to_string().replace("%s", &n.to_string())
}

// Usage examples:
// let users_found = plural_format(5, ["_%s user found_", "_%s users found_", "_%s users found_"]);
// let groups_found = plural_format(3, ["_%s group found_", "_%s groups found_", "_%s groups found_"]);