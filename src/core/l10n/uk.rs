use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Sunday", "Неділя");
    translations.insert("Monday", "Понеділок");
    translations.insert("Tuesday", "Вівторок");
    translations.insert("Wednesday", "Середа");
    translations.insert("Thursday", "Четвер");
    translations.insert("Friday", "П'ятниця");
    translations.insert("Saturday", "Субота");
    translations.insert("January", "Січень");
    translations.insert("February", "Лютий");
    translations.insert("March", "Березень");
    translations.insert("April", "Квітень");
    translations.insert("May", "Травень");
    translations.insert("June", "Червень");
    translations.insert("July", "Липень");
    translations.insert("August", "Серпень");
    translations.insert("September", "Вересень");
    translations.insert("October", "Жовтень");
    translations.insert("November", "Листопад");
    translations.insert("December", "Грудень");
    translations.insert("Settings", "Налаштування");
    translations.insert("seconds ago", "секунди тому");
    translations.insert("today", "сьогодні");
    translations.insert("yesterday", "вчора");
    translations.insert("last month", "минулого місяця");
    translations.insert("months ago", "місяці тому");
    translations.insert("last year", "минулого року");
    translations.insert("years ago", "роки тому");
    translations.insert("Choose", "Обрати");
    translations.insert("Yes", "Так");
    translations.insert("No", "Ні");
    translations.insert("Ok", "Ok");
    translations.insert("Cancel", "Відмінити");
    translations.insert("Shared", "Опубліковано");
    translations.insert("Share", "Поділитися");
    translations.insert("Error", "Помилка");
    translations.insert("Error while sharing", "Помилка під час публікації");
    translations.insert("Error while unsharing", "Помилка під час відміни публікації");
    translations.insert("Error while changing permissions", "Помилка при зміні повноважень");
    translations.insert("Shared with you and the group {group} by {owner}", " {owner} опублікував для Вас та для групи {group}");
    translations.insert("Shared with you by {owner}", "{owner} опублікував для Вас");
    translations.insert("Password protect", "Захистити паролем");
    translations.insert("Password", "Пароль");
    translations.insert("Email link to person", "Ел. пошта належить Пану");
    translations.insert("Send", "Надіслати");
    translations.insert("Set expiration date", "Встановити термін дії");
    translations.insert("Expiration date", "Термін дії");
    translations.insert("Share via email:", "Опублікувати через Ел. пошту:");
    translations.insert("No people found", "Жодної людини не знайдено");
    translations.insert("group", "група");
    translations.insert("Resharing is not allowed", "Пере-публікація не дозволяється");
    translations.insert("Shared in {item} with {user}", "Опубліковано {item} для {user}");
    translations.insert("Unshare", "Закрити доступ");
    translations.insert("can edit", "може редагувати");
    translations.insert("access control", "контроль доступу");
    translations.insert("create", "створити");
    translations.insert("update", "оновити");
    translations.insert("delete", "видалити");
    translations.insert("share", "опублікувати");
    translations.insert("Password protected", "Захищено паролем");
    translations.insert("Error unsetting expiration date", "Помилка при відміні терміна дії");
    translations.insert("Error setting expiration date", "Помилка при встановленні терміна дії");
    translations.insert("Sending ...", "Надсилання...");
    translations.insert("Email sent", "Ел. пошта надіслана");
    translations.insert("Warning", "Попередження");
    translations.insert("The object type is not specified.", "Не визначено тип об'єкту.");
    translations.insert("Delete", "Видалити");
    translations.insert("Add", "Додати");
    translations.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "Оновлення виконалось неуспішно. Будь ласка, повідомте про цю проблему в <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">спільноті ownCloud</a>.");
    translations.insert("The update was successful. Redirecting you to ownCloud now.", "Оновлення виконалось успішно. Перенаправляємо вас на  ownCloud.");
    translations.insert("Use the following link to reset your password: {link}", "Використовуйте наступне посилання для скидання пароля: {link}");
    translations.insert("You will receive a link to reset your password via Email.", "Ви отримаєте посилання для скидання вашого паролю на Ел. пошту.");
    translations.insert("Username", "Ім'я користувача");
    translations.insert("Your password was reset", "Ваш пароль був скинутий");
    translations.insert("To login page", "До сторінки входу");
    translations.insert("New password", "Новий пароль");
    translations.insert("Reset password", "Скинути пароль");
    translations.insert("Personal", "Особисте");
    translations.insert("Users", "Користувачі");
    translations.insert("Apps", "Додатки");
    translations.insert("Admin", "Адмін");
    translations.insert("Help", "Допомога");
    translations.insert("Access forbidden", "Доступ заборонено");
    translations.insert("Cloud not found", "Cloud не знайдено");
    translations.insert("Security Warning", "Попередження про небезпеку");
    translations.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "Ваша версія PHP вразлива для атак NULL Byte (CVE-2006-7243)");
    translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Не доступний безпечний генератор випадкових чисел, будь ласка, активуйте PHP OpenSSL додаток.");
    translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Без безпечного генератора випадкових чисел зловмисник може визначити токени скидання пароля і заволодіти Вашим обліковим записом.");
    translations.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Ваші дані каталогів і файлів, ймовірно, доступні з інтернету, тому що  .htaccess файл не працює.");
    translations.insert("Create an <strong>admin account</strong>", "Створити <strong>обліковий запис адміністратора</strong>");
    translations.insert("Advanced", "Додатково");
    translations.insert("Data folder", "Каталог даних");
    translations.insert("Configure the database", "Налаштування бази даних");
    translations.insert("will be used", "буде використано");
    translations.insert("Database user", "Користувач бази даних");
    translations.insert("Database password", "Пароль для бази даних");
    translations.insert("Database name", "Назва бази даних");
    translations.insert("Database tablespace", "Таблиця бази даних");
    translations.insert("Database host", "Хост бази даних");
    translations.insert("Finish setup", "Завершити налаштування");
    translations.insert("Log out", "Вихід");
    translations.insert("Automatic logon rejected!", "Автоматичний вхід в систему відхилений!");
    translations.insert("If you did not change your password recently, your account may be compromised!", "Якщо Ви не міняли пароль останнім часом, Ваш обліковий запис може бути скомпрометованим!");
    translations.insert("Please change your password to secure your account again.", "Будь ласка, змініть свій пароль, щоб знову захистити Ваш обліковий запис.");
    translations.insert("Lost your password?", "Забули пароль?");
    translations.insert("remember", "запам'ятати");
    translations.insert("Log in", "Вхід");
    translations.insert("Alternative Logins", "Альтернативні Логіни");
    translations.insert("Updating ownCloud to version %s, this may take a while.", "Оновлення ownCloud до версії %s, це може зайняти деякий час.");
    translations
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";

// Función para manejar las formas plurales
pub fn get_plural_forms() -> &'static str {
    PLURAL_FORMS
}

pub fn get_plural_translation(key: &str, count: i64) -> Vec<&'static str> {
    match key {
        "_%n minute ago_::_%n minutes ago_" => vec!["", "", ""],
        "_%n hour ago_::_%n hours ago_" => vec!["", "", ""],
        "_%n day ago_::_%n days ago_" => vec!["", "", ""],
        "_%n month ago_::_%n months ago_" => vec!["", "", ""],
        "_{count} file conflict_::_{count} file conflicts_" => vec!["", "", ""],
        _ => vec!["", "", ""],
    }
}