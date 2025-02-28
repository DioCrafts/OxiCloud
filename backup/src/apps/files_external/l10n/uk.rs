use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Доступ дозволено");
        m.insert("Error configuring Dropbox storage", "Помилка при налаштуванні сховища Dropbox");
        m.insert("Grant access", "Дозволити доступ");
        m.insert("Please provide a valid Dropbox app key and secret.", "Будь ласка, надайте дійсний ключ та пароль Dropbox.");
        m.insert("Error configuring Google Drive storage", "Помилка при налаштуванні сховища Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Попередження:</b> Клієнт \"smbclient\" не встановлено. Під'єднанатися до CIFS/SMB тек неможливо. Попрохайте системного адміністратора встановити його.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Попередження:</b> Підтримка FTP в PHP не увімкнута чи не встановлена. Під'єднанатися до FTP тек неможливо. Попрохайте системного адміністратора встановити її.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Попередження:</b> Підтримка CURL в PHP не увімкнута чи не встановлена. Під'єднанатися OwnCloud / WebDav або Google Drive неможливе. Попрохайте системного адміністратора встановити її.");
        m.insert("External Storage", "Зовнішні сховища");
        m.insert("Folder name", "Ім'я теки");
        m.insert("External storage", "Зовнішнє сховище");
        m.insert("Configuration", "Налаштування");
        m.insert("Options", "Опції");
        m.insert("Applicable", "Придатний");
        m.insert("Add storage", "Додати сховище");
        m.insert("None set", "Не встановлено");
        m.insert("All Users", "Усі користувачі");
        m.insert("Groups", "Групи");
        m.insert("Users", "Користувачі");
        m.insert("Delete", "Видалити");
        m.insert("Enable User External Storage", "Активувати користувацькі зовнішні сховища");
        m.insert("Allow users to mount their own external storage", "Дозволити користувачам монтувати власні зовнішні сховища");
        m.insert("SSL root certificates", "SSL корневі сертифікати");
        m.insert("Import Root Certificate", "Імпортувати корневі сертифікати");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}