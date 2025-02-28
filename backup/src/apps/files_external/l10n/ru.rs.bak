use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Доступ предоставлен");
        m.insert("Error configuring Dropbox storage", "Ошибка при настройке хранилища Dropbox");
        m.insert("Grant access", "Предоставление доступа");
        m.insert("Please provide a valid Dropbox app key and secret.", "Пожалуйста, предоставьте действующий ключ Dropbox и пароль.");
        m.insert("Error configuring Google Drive storage", "Ошибка при настройке хранилища Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Внимание:</b> \"smbclient\" не установлен. Подключение по CIFS/SMB невозможно. Пожалуйста, обратитесь к системному администратору, чтобы установить его.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Внимание:</b> Поддержка FTP не включена в PHP. Подключение по FTP невозможно. Пожалуйста, обратитесь к системному администратору, чтобы включить.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Внимание:</b> Поддержка Curl в PHP не включена или не установлена. Подключение ownCloud / WebDAV или GoogleDrive невозможно. Попросите вашего системного администратора установить его.");
        m.insert("External Storage", "Внешний носитель");
        m.insert("Folder name", "Имя папки");
        m.insert("External storage", "Внешний носитель данных");
        m.insert("Configuration", "Конфигурация");
        m.insert("Options", "Опции");
        m.insert("Applicable", "Применимый");
        m.insert("Add storage", "Добавить носитель данных");
        m.insert("None set", "Не установлено");
        m.insert("All Users", "Все пользователи");
        m.insert("Groups", "Группы");
        m.insert("Users", "Пользователи");
        m.insert("Delete", "Удалить");
        m.insert("Enable User External Storage", "Включить пользовательские внешние носители");
        m.insert("Allow users to mount their own external storage", "Разрешить пользователям монтировать их собственные внешние носители");
        m.insert("SSL root certificates", "Корневые сертификаты SSL");
        m.insert("Import Root Certificate", "Импортировать корневые сертификаты");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}