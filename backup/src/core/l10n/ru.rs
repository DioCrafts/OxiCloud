use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("%s shared »%s« with you", "%s поделился »%s« с вами");
        m.insert("Couldn't send mail to following users: %s ", "Невозможно отправить письмо следующим пользователям: %s");
        m.insert("Turned on maintenance mode", "Режим отладки включён");
        m.insert("Turned off maintenance mode", "Режим отладки отключён");
        m.insert("Updated database", "База данных обновлена");
        m.insert("Updating filecache, this may take really long...", "Обновление файлового кэша, это может занять некоторое время...");
        m.insert("Updated filecache", "Обновлен файловый кэш");
        m.insert("... %d%% done ...", "... %d%% завершено ...");
        m.insert("No image or file provided", "Не указано изображение или файл");
        m.insert("Unknown filetype", "Неизвестный тип файла");
        m.insert("Invalid image", "Изображение повреждено");
        m.insert("No temporary profile picture available, try again", "Временная картинка профиля недоступна, повторите попытку");
        m.insert("No crop data provided", "Не указана информация о кадрировании");
        m.insert("Sunday", "Воскресенье");
        m.insert("Monday", "Понедельник");
        m.insert("Tuesday", "Вторник");
        m.insert("Wednesday", "Среда");
        m.insert("Thursday", "Четверг");
        m.insert("Friday", "Пятница");
        m.insert("Saturday", "Суббота");
        m.insert("January", "Январь");
        m.insert("February", "Февраль");
        m.insert("March", "Март");
        m.insert("April", "Апрель");
        m.insert("May", "Май");
        m.insert("June", "Июнь");
        m.insert("July", "Июль");
        m.insert("August", "Август");
        m.insert("September", "Сентябрь");
        m.insert("October", "Октябрь");
        m.insert("November", "Ноябрь");
        m.insert("December", "Декабрь");
        m.insert("Settings", "Конфигурация");
        m.insert("seconds ago", "несколько секунд назад");
        m.insert("today", "сегодня");
        m.insert("yesterday", "вчера");
        m.insert("last month", "в прошлом месяце");
        m.insert("months ago", "несколько месяцев назад");
        m.insert("last year", "в прошлом году");
        m.insert("years ago", "несколько лет назад");
        m.insert("Choose", "Выбрать");
        m.insert("Error loading file picker template: {error}", "Ошибка при загрузке шаблона выбора файлов: {error}");
        m.insert("Yes", "Да");
        m.insert("No", "Нет");
        m.insert("Ok", "Ок");
        m.insert("Error loading message template: {error}", "Ошибка загрузки шаблона сообщений: {error}");
        m.insert("One file conflict", "Один конфликт в файлах");
        m.insert("Which files do you want to keep?", "Какие файлы вы хотите сохранить?");
        m.insert("If you select both versions, the copied file will have a number added to its name.", "При выборе обоих версий,  к названию копируемого файла будет добавлена цифра");
        m.insert("Cancel", "Отменить");
        m.insert("Continue", "Продолжить");
        m.insert("(all selected)", "(выбраны все)");
        m.insert("Error loading file exists template", "Ошибка при загрузке шаблона существующего файла");
        m.insert("Shared", "Общие");
        m.insert("Share", "Открыть доступ");
        m.insert("Error", "Ошибка");
        m.insert("Error while sharing", "Ошибка при открытии доступа");
        m.insert("Error while unsharing", "Ошибка при закрытии доступа");
        m.insert("Error while changing permissions", "Ошибка при смене разрешений");
        m.insert("Shared with you and the group {group} by {owner}", "{owner} открыл доступ для Вас и группы {group} ");
        m.insert("Shared with you by {owner}", "{owner} открыл доступ для Вас");
        m.insert("Share with user or group …", "Поделиться с пользователем или группой...");
        m.insert("Share link", "Поделиться ссылкой");
        m.insert("Password protect", "Защитить паролем");
        m.insert("Password", "Пароль");
        m.insert("Allow Public Upload", "Разрешить открытую загрузку");
        m.insert("Email link to person", "Почтовая ссылка на персону");
        m.insert("Send", "Отправить");
        m.insert("Set expiration date", "Установить срок доступа");
        m.insert("Expiration date", "Дата окончания");
        m.insert("Share via email:", "Поделится через электронную почту:");
        m.insert("No people found", "Ни один человек не найден");
        m.insert("group", "группа");
        m.insert("Resharing is not allowed", "Общий доступ не разрешен");
        m.insert("Shared in {item} with {user}", "Общий доступ к {item} с {user}");
        m.insert("Unshare", "Закрыть общий доступ");
        m.insert("notify by email", "уведомить по почте");
        m.insert("can edit", "может редактировать");
        m.insert("access control", "контроль доступа");
        m.insert("create", "создать");
        m.insert("update", "обновить");
        m.insert("delete", "удалить");
        m.insert("share", "открыть доступ");
        m.insert("Password protected", "Защищено паролем");
        m.insert("Error unsetting expiration date", "Ошибка при отмене срока доступа");
        m.insert("Error setting expiration date", "Ошибка при установке срока доступа");
        m.insert("Sending ...", "Отправляется ...");
        m.insert("Email sent", "Письмо отправлено");
        m.insert("Warning", "Предупреждение");
        m.insert("The object type is not specified.", "Тип объекта не указан");
        m.insert("Enter new", "Ввести новое");
        m.insert("Delete", "Удалить");
        m.insert("Add", "Добавить");
        m.insert("Edit tags", "Изменить метки");
        m.insert("Error loading dialog template: {error}", "Ошибка загрузки шаблона диалога: {error}");
        m.insert("No tags selected for deletion.", "Не выбраны меток для удаления.");
        m.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "При обновлении произошла ошибка. Пожалуйста сообщите об этом в <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud сообщество</a>.");
        m.insert("The update was successful. Redirecting you to ownCloud now.", "Обновление прошло успешно. Перенаправляемся в Ваш ownCloud...");
        m.insert("%s password reset", "%s сброс пароля");
        m.insert("Use the following link to reset your password: {link}", "Используйте следующую ссылку чтобы сбросить пароль: {link}");
        m.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "Ссылка для сброса пароля отправлена вам ​​по электронной почте.<br>Если вы не получите письмо в пределах одной-двух минут, проверьте папку Спам. <br>Если письма там нет, обратитесь к своему администратору.");
        m.insert("Request failed!<br>Did you make sure your email/username was right?", "Запрос не удался. Вы уверены, что email или имя пользователя указаны верно?");
        m.insert("You will receive a link to reset your password via Email.", "На ваш адрес Email выслана ссылка для сброса пароля.");
        m.insert("Username", "Имя пользователя");
        m.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "Ваши файлы зашифрованы. Если вы не активировали ключ восстановления, то после сброса пароля все ваши данные будут потеряны навсегда. Если вы не знаете что делать, свяжитесь со своим администратором до того как продолжить. Вы действительно хотите продолжить?");
        m.insert("Yes, I really want to reset my password now", "Да, я действительно хочу сбросить свой пароль");
        m.insert("Reset", "Сброс");
        m.insert("Your password was reset", "Ваш пароль был сброшен");
        m.insert("To login page", "На страницу авторизации");
        m.insert("New password", "Новый пароль");
        m.insert("Reset password", "Сбросить пароль");
        m.insert("Personal", "Личное");
        m.insert("Users", "Пользователи");
        m.insert("Apps", "Приложения");
        m.insert("Admin", "Админпанель");
        m.insert("Help", "Помощь");
        m.insert("Error loading tags", "Ошибка загрузки меток");
        m.insert("Tag already exists", "Метка уже существует");
        m.insert("Error deleting tag(s)", "Ошибка удаления метки(ок)");
        m.insert("Error tagging", "Ошибка присваивания метки");
        m.insert("Error untagging", "Ошибка снятия метки");
        m.insert("Error favoriting", "Ошибка размещения в любимых");
        m.insert("Error unfavoriting", "Ошибка удаления из любимых");
        m.insert("Access forbidden", "Доступ запрещён");
        m.insert("Cloud not found", "Облако не найдено");
        m.insert("Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n", "Здравствуйте,\n\nпросто даём вам знать, что %s расшарил %s для вас.\nПосмотреть: %s\n\n");
        m.insert("The share will expire on %s.\n\n", "Шара закончится %s\n\n");
        m.insert("Cheers!", "Приветствуем!");
        m.insert("Security Warning", "Предупреждение безопасности");
        m.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "Ваша версия PHP уязвима к атаке NULL Byte (CVE-2006-7243)");
        m.insert("Please update your PHP installation to use %s securely.", "Пожалуйста обновите Вашу PHP конфигурацию для безопасного использования %s.");
        m.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Отсутствует защищенный генератор случайных чисел, пожалуйста, включите расширение PHP OpenSSL.");
        m.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Без защищенного генератора случайных чисел злоумышленник может предугадать токены сброса пароля и завладеть Вашей учетной записью.");
        m.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Ваша папка с данными и файлы возможно доступны из интернета потому что файл .htaccess не работает.");
        m.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "Для информации, как правильно настроить Ваш сервер, пожалуйста загляните в <a hrev=\"%s\"target=\"blank\">документацию</a>.");
        m.insert("Create an <strong>admin account</strong>", "Создать <strong>учётную запись администратора</strong>");
        m.insert("Advanced", "Дополнительно");
        m.insert("Data folder", "Директория с данными");
        m.insert("Configure the database", "Настройка базы данных");
        m.insert("will be used", "будет использовано");
        m.insert("Database user", "Пользователь базы данных");
        m.insert("Database password", "Пароль базы данных");
        m.insert("Database name", "Название базы данных");
        m.insert("Database tablespace", "Табличое пространство базы данных");
        m.insert("Database host", "Хост базы данных");
        m.insert("Finish setup", "Завершить установку");
        m.insert("Finishing …", "Завершаем...");
        m.insert("%s is available. Get more information on how to update.", "%s доступно. Получить дополнительную информацию о порядке обновления.");
        m.insert("Log out", "Выйти");
        m.insert("Automatic logon rejected!", "Автоматический вход в систему отключен!");
        m.insert("If you did not change your password recently, your account may be compromised!", "Если Вы недавно не меняли свой пароль, то Ваша учетная запись может быть скомпрометирована!");
        m.insert("Please change your password to secure your account again.", "Пожалуйста, смените пароль, чтобы обезопасить свою учетную запись.");
        m.insert("Server side authentication failed!", "Неудачная аутентификация с сервером!");
        m.insert("Please contact your administrator.", "Пожалуйста, свяжитесь с вашим администратором.");
        m.insert("Lost your password?", "Забыли пароль?");
        m.insert("remember", "запомнить");
        m.insert("Log in", "Войти");
        m.insert("Alternative Logins", "Альтернативные имена пользователя");
        m.insert("Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>", "Здравствуйте,<br><br>просто даём вам знать, что %s расшарил %s для вас.<br><a href=\"%s\">Посмотреть!</a><br><br>");
        m.insert("The share will expire on %s.<br><br>", "Шара закончится %s.<br><br>");
        m.insert("Updating ownCloud to version %s, this may take a while.", "Идёт обновление ownCloud до версии %s. Это может занять некоторое время.");
        m.insert("This ownCloud instance is currently being updated, which may take a while.", "Производится обновление ownCloud, это может занять некоторое время.");
        m.insert("Please reload this page after a short time to continue using ownCloud.", "Перезагрузите эту страницу через короткое время чтобы продолжить использовать ownCloud.");
        m.insert("Contact your system administrator if this message persists or appeared unexpectedly.", "Обратитесь к вашему системному администратору если это сообщение не исчезает или появляется неожиданно.");
        m.insert("Thank you for your patience.", "Спасибо за ваше терпение.");
        m
    };

    /// Pluralized translations are stored separately for each form
    pub static ref PLURALS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["%n минуту назад", "%n минуты назад", "%n минут назад"]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["%n час назад", "%n часа назад", "%n часов назад"]);
        m.insert("_%n day ago_::_%n days ago_", vec!["%n день назад", "%n дня назад", "%n дней назад"]);
        m.insert("_%n month ago_::_%n months ago_", vec!["%n месяц назад", "%n месяца назад", "%n месяцев назад"]);
        m.insert("_{count} file conflict_::_{count} file conflicts_", vec!["{count} конфликт в файлах", "{count} конфликта в файлах", "{count} конфликтов в файлах"]);
        m.insert("({count} selected)", vec!["({count} выбрано)"]);
        m
    };
}

/// Returns plural form index for Russian language
/// nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);
pub fn get_plural_form(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}

/// Returns the appropriate translation for a given key
pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

/// Returns the appropriate plural form translation for a given key and number
pub fn translate_plural(key: &str, n: i64) -> &'static str {
    if let Some(forms) = PLURALS.get(key) {
        let form_index = get_plural_form(n);
        if form_index < forms.len() {
            return forms[form_index];
        }
    }
    key
}