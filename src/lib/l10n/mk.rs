use std::collections::HashMap;
use rust_i18n::locale::Locale;

pub fn get_translation() -> (HashMap<&'static str, &'static str>, &'static str) {
    let mut translations = HashMap::new();
    
    translations.insert("Help", "Помош");
    translations.insert("Personal", "Лично");
    translations.insert("Settings", "Подесувања");
    translations.insert("Users", "Корисници");
    translations.insert("Admin", "Админ");
    translations.insert("Unknown filetype", "Непознат тип на датотека");
    translations.insert("Invalid image", "Невалидна фотографија");
    translations.insert("web services under your control", "веб сервиси под Ваша контрола");
    translations.insert("ZIP download is turned off.", "Преземање во ZIP е исклучено");
    translations.insert("Files need to be downloaded one by one.", "Датотеките треба да се симнат една по една.");
    translations.insert("Back to Files", "Назад кон датотеки");
    translations.insert("Selected files too large to generate zip file.", "Избраните датотеки се преголеми за да се генерира zip.");
    translations.insert("Application is not enabled", "Апликацијата не е овозможена");
    translations.insert("Authentication error", "Грешка во автентикација");
    translations.insert("Token expired. Please reload page.", "Жетонот е истечен. Ве молам превчитајте ја страницата.");
    translations.insert("Files", "Датотеки");
    translations.insert("Text", "Текст");
    translations.insert("Images", "Слики");
    translations.insert("Could not find category \"%s\"", "Не можам да најдам категорија „%s"");
    translations.insert("seconds ago", "пред секунди");
    translations.insert("_%n minute ago_::_%n minutes ago_", "");
    translations.insert("_%n hour ago_::_%n hours ago_", "");
    translations.insert("today", "денеска");
    translations.insert("yesterday", "вчера");
    translations.insert("_%n day go_::_%n days ago_", "");
    translations.insert("last month", "минатиот месец");
    translations.insert("_%n month ago_::_%n months ago_", "");
    translations.insert("last year", "минатата година");
    translations.insert("years ago", "пред години");
    
    let plural_forms = "nplurals=2; plural=(n % 10 == 1 && n % 100 != 11) ? 0 : 1;";
    
    (translations, plural_forms)
}

pub fn register_locale() -> Locale {
    let (translations, plural_forms) = get_translation();
    Locale::new("mk", translations, plural_forms)
}