use std::collections::HashMap;
use rust_i18n::t;

pub fn register() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Access granted", "Пристапот е дозволен");
    translations.insert("Error configuring Dropbox storage", "Грешка при конфигурација на Dropbox");
    translations.insert("Grant access", "Дозволи пристап");
    translations.insert("Please provide a valid Dropbox app key and secret.", "Ве молам доставите валиден Dropbox клуч и тајна лозинка.");
    translations.insert("Error configuring Google Drive storage", "Грешка при конфигурација на Google Drive");
    translations.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Внимание:</b> \"smbclient\" не е инсталиран. Не е можно монтирање на CIFS/SMB дискови. Замолете го Вашиот систем администратор да го инсталира.");
    translations.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Внимание:</b> Не е овозможена или инсталирани FTP подршка во PHP. Не е можно монтирање на FTP дискови. Замолете го Вашиот систем администратор да го инсталира.");
    translations.insert("External Storage", "Надворешно складиште");
    translations.insert("Folder name", "Име на папка");
    translations.insert("Configuration", "Конфигурација");
    translations.insert("Options", "Опции");
    translations.insert("Applicable", "Применливо");
    translations.insert("None set", "Ништо поставено");
    translations.insert("All Users", "Сите корисници");
    translations.insert("Groups", "Групи");
    translations.insert("Users", "Корисници");
    translations.insert("Delete", "Избриши");
    translations.insert("Enable User External Storage", "Овозможи надворешни за корисници");
    translations.insert("Allow users to mount their own external storage", "Дозволи им на корисниците да монтираат свои надворешни дискови");
    translations.insert("SSL root certificates", "SSL root сертификати");
    translations.insert("Import Root Certificate", "Увези");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n % 10 == 1 && n % 100 != 11) ? 0 : 1;"
}