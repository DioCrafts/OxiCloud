use rust_fluent::fluent::bundle::{FluentBundle, FluentResource};
use std::collections::HashMap;

pub fn get_translation_bundle() -> FluentBundle<FluentResource> {
    let mut translations = HashMap::new();
    translations.insert("Help".to_string(), "Помощ".to_string());
    translations.insert("Personal".to_string(), "Лични".to_string());
    translations.insert("Settings".to_string(), "Настройки".to_string());
    translations.insert("Users".to_string(), "Потребители".to_string());
    translations.insert("Admin".to_string(), "Админ".to_string());
    translations.insert("web services under your control".to_string(), "уеб услуги под Ваш контрол".to_string());
    translations.insert("ZIP download is turned off.".to_string(), "Изтеглянето като ZIP е изключено.".to_string());
    translations.insert("Files need to be downloaded one by one.".to_string(), "Файловете трябва да се изтеглят един по един.".to_string());
    translations.insert("Back to Files".to_string(), "Назад към файловете".to_string());
    translations.insert("Selected files too large to generate zip file.".to_string(), "Избраните файлове са прекалено големи за генерирането на ZIP архив.".to_string());
    translations.insert("Application is not enabled".to_string(), "Приложението не е включено.".to_string());
    translations.insert("Authentication error".to_string(), "Възникна проблем с идентификацията".to_string());
    translations.insert("Token expired. Please reload page.".to_string(), "Ключът е изтекъл, моля презаредете страницата".to_string());
    translations.insert("Files".to_string(), "Файлове".to_string());
    translations.insert("Text".to_string(), "Текст".to_string());
    translations.insert("Images".to_string(), "Снимки".to_string());
    translations.insert("%s enter the database username.".to_string(), "%s въведете потребителско име за базата с данни.".to_string());
    translations.insert("%s enter the database name.".to_string(), "%s въведете име на базата с данни.".to_string());
    translations.insert("%s you may not use dots in the database name".to_string(), "%s, не можете да ползвате точки в името на базата от данни".to_string());
    translations.insert("MS SQL username and/or password not valid: %s".to_string(), "Невалидно MS SQL потребителско име и/или парола: %s".to_string());
    translations.insert("You need to enter either an existing account or the administrator.".to_string(), "Необходимо е да влезете в всъществуващ акаунт или като администратора".to_string());
    translations.insert("MySQL username and/or password not valid".to_string(), "Невалидно MySQL потребителско име и/или парола".to_string());
    translations.insert("DB Error: \"%s\"".to_string(), "Грешка в базата от данни: \"%s\"".to_string());
    translations.insert("Offending command was: \"%s\"".to_string(), "Проблемната команда беше: \"%s\"".to_string());
    translations.insert("MySQL user '%s'@'localhost' exists already.".to_string(), "MySQL потребителят '%s'@'localhost' вече съществува".to_string());
    translations.insert("Drop this user from MySQL".to_string(), "Изтриване на потребителя от MySQL".to_string());
    translations.insert("MySQL user '%s'@'%%' already exists".to_string(), "MySQL потребителят  '%s'@'%%' вече съществува.".to_string());
    translations.insert("Drop this user from MySQL.".to_string(), "Изтриване на потребителя от MySQL.".to_string());
    translations.insert("Oracle connection could not be established".to_string(), "Oracle връзка не можа да се осъществи".to_string());
    translations.insert("Oracle username and/or password not valid".to_string(), "Невалидно Oracle потребителско име и/или парола".to_string());
    translations.insert("Offending command was: \"%s\", name: %s, password: %s".to_string(), "Проблемната команда беше: \"%s\", име: %s, парола: %s".to_string());
    translations.insert("PostgreSQL username and/or password not valid".to_string(), "Невалидно PostgreSQL потребителско име и/или парола".to_string());
    translations.insert("Set an admin username.".to_string(), "Въведете потребителско име за администратор.".to_string());
    translations.insert("Set an admin password.".to_string(), "Въведете парола за администратор.".to_string());
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.".to_string(), "Вашият web сървър все още не е удачно настроен да позволява синхронизация на файлове, защото WebDAV интерфейсът изглежда не работи.".to_string());
    translations.insert("Please double check the <a href='%s'>installation guides</a>.".to_string(), "Моля направете повторна справка с <a href='%s'>ръководството за инсталиране</a>.".to_string());
    translations.insert("Could not find category \"%s\"".to_string(), "Невъзможно откриване на категорията \"%s\"".to_string());
    translations.insert("seconds ago".to_string(), "преди секунди".to_string());
    translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), "".to_string());
    translations.insert("_%n hour ago_::_%n hours ago_".to_string(), "".to_string());
    translations.insert("today".to_string(), "днес".to_string());
    translations.insert("yesterday".to_string(), "вчера".to_string());
    translations.insert("_%n day go_::_%n days ago_".to_string(), "".to_string());
    translations.insert("last month".to_string(), "последният месец".to_string());
    translations.insert("_%n month ago_::_%n months ago_".to_string(), "".to_string());
    translations.insert("last year".to_string(), "последната година".to_string());
    translations.insert("years ago".to_string(), "последните години".to_string());

    // Crear recursos Fluent desde nuestro HashMap
    let mut resources = String::new();
    for (key, value) in translations {
        resources.push_str(&format!("{} = {}\n", key, value));
    }
    
    let resource = FluentResource::try_new(resources)
        .expect("Failed to create FluentResource");

    let mut bundle = FluentBundle::new(vec!["bg-BG".parse().unwrap()]);
    bundle.add_resource(resource)
        .expect("Failed to add resource to bundle");
    
    // Establecer reglas de pluralización
    bundle.set_use_isolating(false);
    
    bundle
}

pub fn get_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}