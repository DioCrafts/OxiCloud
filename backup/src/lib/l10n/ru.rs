use std::collections::HashMap;
use unic_langid::LanguageIdentifier;
use rust_fluent::FluentBundle;

pub fn get_translation_bundle() -> FluentBundle {
    let lang_id: LanguageIdentifier = "ru".parse().unwrap();
    let mut bundle = FluentBundle::new(vec![lang_id]);
    
    let translations = get_translations();
    let plural_forms = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
    
    bundle.set_plural_rule(plural_forms.to_string());
    
    for (key, value) in translations {
        match bundle.add_message(key, value) {
            Ok(_) => {},
            Err(e) => log::warn!("Failed to add message '{}': {:?}", key, e),
        }
    }
    
    bundle
}

fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert(
        "App \"%s\" can't be installed because it is not compatible with this version of ownCloud.".to_string(),
        "Приложение \"%s\" нельзя установить, так как оно не совместимо с текущей версией ownCloud.".to_string(),
    );
    translations.insert(
        "No app name specified".to_string(),
        "Не выбрано имя приложения".to_string(),
    );
    translations.insert(
        "Help".to_string(),
        "Помощь".to_string(),
    );
    translations.insert(
        "Personal".to_string(),
        "Личное".to_string(),
    );
    translations.insert(
        "Settings".to_string(),
        "Конфигурация".to_string(),
    );
    translations.insert(
        "Users".to_string(),
        "Пользователи".to_string(),
    );
    translations.insert(
        "Admin".to_string(),
        "Admin".to_string(),
    );
    translations.insert(
        "Failed to upgrade \"%s\".".to_string(),
        "Не смог обновить \"%s\".".to_string(),
    );
    translations.insert(
        "Unknown filetype".to_string(),
        "Неизвестный тип файла".to_string(),
    );
    translations.insert(
        "Invalid image".to_string(),
        "Изображение повреждено".to_string(),
    );
    translations.insert(
        "web services under your control".to_string(),
        "веб-сервисы под вашим управлением".to_string(),
    );
    translations.insert(
        "cannot open \"%s\"".to_string(),
        "не могу открыть \"%s\"".to_string(),
    );
    translations.insert(
        "ZIP download is turned off.".to_string(),
        "ZIP-скачивание отключено.".to_string(),
    );
    translations.insert(
        "Files need to be downloaded one by one.".to_string(),
        "Файлы должны быть загружены по одному.".to_string(),
    );
    translations.insert(
        "Back to Files".to_string(),
        "Назад к файлам".to_string(),
    );
    translations.insert(
        "Selected files too large to generate zip file.".to_string(),
        "Выбранные файлы слишком велики, чтобы создать zip файл.".to_string(),
    );
    translations.insert(
        "Download the files in smaller chunks, seperately or kindly ask your administrator.".to_string(),
        "Загрузите файл маленьшими порциями, раздельно или вежливо попросите Вашего администратора.".to_string(),
    );
    translations.insert(
        "No source specified when installing app".to_string(),
        "Не указан источник при установке приложения".to_string(),
    );
    translations.insert(
        "No href specified when installing app from http".to_string(),
        "Не указан атрибут href при установке приложения через http".to_string(),
    );
    translations.insert(
        "No path specified when installing app from local file".to_string(),
        "Не указан путь при установке приложения из локального файла".to_string(),
    );
    translations.insert(
        "Archives of type %s are not supported".to_string(),
        "Архивы %s не поддерживаются".to_string(),
    );
    translations.insert(
        "Failed to open archive when installing app".to_string(),
        "Не возможно открыть архив при установке приложения".to_string(),
    );
    translations.insert(
        "App does not provide an info.xml file".to_string(),
        "Приложение не имеет файла info.xml".to_string(),
    );
    translations.insert(
        "App can't be installed because of not allowed code in the App".to_string(),
        "Приложение невозможно установить. В нем содержится запрещенный код.".to_string(),
    );
    translations.insert(
        "App can't be installed because it is not compatible with this version of ownCloud".to_string(),
        "Приложение невозможно установить. Не совместимо с текущей версией ownCloud.".to_string(),
    );
    translations.insert(
        "App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps".to_string(),
        "Приложение невозможно установить. Оно содержит параметр <shipped>true</shipped> который не допустим для приложений, не входящих в поставку.".to_string(),
    );
    translations.insert(
        "App can't be installed because the version in info.xml/version is not the same as the version reported from the app store".to_string(),
        "Приложение невозможно установить. Версия в info.xml/version не совпадает с версией заявленной в магазине приложений".to_string(),
    );
    translations.insert(
        "App directory already exists".to_string(),
        "Папка приложения уже существует".to_string(),
    );
    translations.insert(
        "Can't create app folder. Please fix permissions. %s".to_string(),
        "Не удалось создать директорию. Исправьте права доступа. %s".to_string(),
    );
    translations.insert(
        "Application is not enabled".to_string(),
        "Приложение не разрешено".to_string(),
    );
    translations.insert(
        "Authentication error".to_string(),
        "Ошибка аутентификации".to_string(),
    );
    translations.insert(
        "Token expired. Please reload page.".to_string(),
        "Токен просрочен. Перезагрузите страницу.".to_string(),
    );
    translations.insert(
        "Files".to_string(),
        "Файлы".to_string(),
    );
    translations.insert(
        "Text".to_string(),
        "Текст".to_string(),
    );
    translations.insert(
        "Images".to_string(),
        "Изображения".to_string(),
    );
    translations.insert(
        "%s enter the database username.".to_string(),
        "%s введите имя пользователя базы данных.".to_string(),
    );
    translations.insert(
        "%s enter the database name.".to_string(),
        "%s введите имя базы данных.".to_string(),
    );
    translations.insert(
        "%s you may not use dots in the database name".to_string(),
        "%s Вы не можете использовать точки в имени базы данных".to_string(),
    );
    translations.insert(
        "MS SQL username and/or password not valid: %s".to_string(),
        "Имя пользователя и/или пароль MS SQL не подходит: %s".to_string(),
    );
    translations.insert(
        "You need to enter either an existing account or the administrator.".to_string(),
        "Вы должны войти или в существующий аккаунт или под администратором.".to_string(),
    );
    translations.insert(
        "MySQL username and/or password not valid".to_string(),
        "Неверное имя пользователя и/или пароль MySQL".to_string(),
    );
    translations.insert(
        "DB Error: \"%s\"".to_string(),
        "Ошибка БД: \"%s\"".to_string(),
    );
    translations.insert(
        "Offending command was: \"%s\"".to_string(),
        "Вызываемая команда была: \"%s\"".to_string(),
    );
    translations.insert(
        "MySQL user '%s'@'localhost' exists already.".to_string(),
        "Пользователь MySQL '%s'@'localhost' уже существует.".to_string(),
    );
    translations.insert(
        "Drop this user from MySQL".to_string(),
        "Удалить этого пользователя из MySQL".to_string(),
    );
    translations.insert(
        "MySQL user '%s'@'%%' already exists".to_string(),
        "Пользователь MySQL '%s'@'%%' уже существует".to_string(),
    );
    translations.insert(
        "Drop this user from MySQL.".to_string(),
        "Удалить этого пользователя из MySQL.".to_string(),
    );
    translations.insert(
        "Oracle connection could not be established".to_string(),
        "соединение с Oracle не может быть установлено".to_string(),
    );
    translations.insert(
        "Oracle username and/or password not valid".to_string(),
        "Неверное имя пользователя и/или пароль Oracle".to_string(),
    );
    translations.insert(
        "Offending command was: \"%s\", name: %s, password: %s".to_string(),
        "Вызываемая команда была: \"%s\", имя: %s, пароль: %s".to_string(),
    );
    translations.insert(
        "PostgreSQL username and/or password not valid".to_string(),
        "Неверное имя пользователя и/или пароль PostgreSQL".to_string(),
    );
    translations.insert(
        "Set an admin username.".to_string(),
        "Установить имя пользователя для admin.".to_string(),
    );
    translations.insert(
        "Set an admin password.".to_string(),
        "становит пароль для admin.".to_string(),
    );
    translations.insert(
        "Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.".to_string(),
        "Ваш веб сервер до сих пор не настроен правильно для возможности синхронизации файлов, похоже что проблема в неисправности интерфейса WebDAV.".to_string(),
    );
    translations.insert(
        "Please double check the <a href='%s'>installation guides</a>.".to_string(),
        "Пожалуйста, дважды просмотрите <a href='%s'>инструкции по установке</a>.".to_string(),
    );
    translations.insert(
        "Could not find category \"%s\"".to_string(),
        "Категория \"%s\"  не найдена".to_string(),
    );
    translations.insert(
        "seconds ago".to_string(),
        "несколько секунд назад".to_string(),
    );
    translations.insert(
        "_%n minute ago_::_%n minutes ago_".to_string(),
        "%n минута назад|%n минуты назад|%n минут назад".to_string(),
    );
    translations.insert(
        "_%n hour ago_::_%n hours ago_".to_string(),
        "%n час назад|%n часа назад|%n часов назад".to_string(),
    );
    translations.insert(
        "today".to_string(),
        "сегодня".to_string(),
    );
    translations.insert(
        "yesterday".to_string(),
        "вчера".to_string(),
    );
    translations.insert(
        "_%n day go_::_%n days ago_".to_string(),
        "%n день назад|%n дня назад|%n дней назад".to_string(),
    );
    translations.insert(
        "last month".to_string(),
        "в прошлом месяце".to_string(),
    );
    translations.insert(
        "_%n month ago_::_%n months ago_".to_string(),
        "%n месяц назад|%n месяца назад|%n месяцев назад".to_string(),
    );
    translations.insert(
        "last year".to_string(),
        "в прошлом году".to_string(),
    );
    translations.insert(
        "years ago".to_string(),
        "несколько лет назад".to_string(),
    );
    translations.insert(
        "Caused by:".to_string(),
        "Вызвано:".to_string(),
    );

    translations
}