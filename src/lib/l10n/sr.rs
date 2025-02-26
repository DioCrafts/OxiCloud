use std::collections::HashMap;
use rust_i18n::PluralizationRule;

// Serbian translations
pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Help".to_string(), "Помоћ".to_string());
    translations.insert("Personal".to_string(), "Лично".to_string());
    translations.insert("Settings".to_string(), "Поставке".to_string());
    translations.insert("Users".to_string(), "Корисници".to_string());
    translations.insert("Admin".to_string(), "Администратор".to_string());
    translations.insert("web services under your control".to_string(), "веб сервиси под контролом".to_string());
    translations.insert("ZIP download is turned off.".to_string(), "Преузимање ZIP-а је искључено.".to_string());
    translations.insert("Files need to be downloaded one by one.".to_string(), "Датотеке морате преузимати једну по једну.".to_string());
    translations.insert("Back to Files".to_string(), "Назад на датотеке".to_string());
    translations.insert("Selected files too large to generate zip file.".to_string(), "Изабране датотеке су превелике да бисте направили ZIP датотеку.".to_string());
    translations.insert("Application is not enabled".to_string(), "Апликација није омогућена".to_string());
    translations.insert("Authentication error".to_string(), "Грешка при провери идентитета".to_string());
    translations.insert("Token expired. Please reload page.".to_string(), "Жетон је истекао. Поново учитајте страницу.".to_string());
    translations.insert("Files".to_string(), "Датотеке".to_string());
    translations.insert("Text".to_string(), "Текст".to_string());
    translations.insert("Images".to_string(), "Слике".to_string());
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.".to_string(), "Ваш веб сервер тренутно не подржава синхронизацију датотека јер се чини да је WebDAV сучеље неисправно.".to_string());
    translations.insert("Please double check the <a href='%s'>installation guides</a>.".to_string(), "Погледајте <a href='%s'>водиче за инсталацију</a>.".to_string());
    translations.insert("Could not find category \"%s\"".to_string(), "Не могу да пронађем категорију „%s".".to_string());
    translations.insert("seconds ago".to_string(), "пре неколико секунди".to_string());
    translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), "".to_string());
    translations.insert("_%n hour ago_::_%n hours ago_".to_string(), "".to_string());
    translations.insert("today".to_string(), "данас".to_string());
    translations.insert("yesterday".to_string(), "јуче".to_string());
    translations.insert("_%n day go_::_%n days ago_".to_string(), "".to_string());
    translations.insert("last month".to_string(), "прошлог месеца".to_string());
    translations.insert("_%n month ago_::_%n months ago_".to_string(), "".to_string());
    translations.insert("last year".to_string(), "прошле године".to_string());
    translations.insert("years ago".to_string(), "година раније".to_string());
    
    translations
}

// Serbian plural forms rule
pub fn get_plural_form() -> PluralizationRule {
    Box::new(|n| {
        if n % 10 == 1 && n % 100 != 11 {
            0
        } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
            1
        } else {
            2
        }
    })
}

// Serbian plural forms
pub fn get_plural_forms() -> Vec<String> {
    vec!["".to_string(), "".to_string(), "".to_string()]
}