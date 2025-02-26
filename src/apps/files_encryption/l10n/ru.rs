use std::collections::HashMap;
use rust_i18n::locale::Locale;

/// Provides Russian translations for the files_encryption app
pub fn get_russian_translations() -> (HashMap<&'static str, &'static str>, &'static str) {
    let translations = HashMap::from([
        ("Recovery key successfully enabled", "Ключ восстановления успешно установлен"),
        ("Could not enable recovery key. Please check your recovery key password!", "Невозможно включить ключ восстановления. Проверьте правильность пароля от ключа!"),
        ("Recovery key successfully disabled", "Ключ восстановления успешно отключен"),
        ("Could not disable recovery key. Please check your recovery key password!", "Невозможно выключить ключ восстановления. Проверьте правильность пароля от ключа!"),
        ("Password successfully changed.", "Пароль изменен удачно."),
        ("Could not change the password. Maybe the old password was not correct.", "Невозможно изменить пароль. Возможно старый пароль не был верен."),
        ("Private key password successfully updated.", "Пароль секретного ключа успешно обновлён."),
        ("Could not update the private key password. Maybe the old password was not correct.", "Невозможно обновить пароль от секретного ключа. Возможно, старый пароль указан неверно."),
        ("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Приложение шифрации не инициализированно! Возможно приложение шифрации было реактивировано во время вашей сессии. Пожалуйста, попробуйте выйти и войти снова чтобы проинициализировать приложение шифрации."),
        ("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "Ваш секретный ключ не действителен! Вероятно, ваш пароль был изменен вне %s (например, корпоративный каталог). Вы можете обновить секретный ключ в личных настройках на странице восстановления доступа к зашифрованным файлам. "),
        ("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Не могу расшифровать файл, возможно это опубликованный файл. Пожалуйста, попросите владельца файла поделиться им  с вами еще раз."),
        ("Unknown error please check your system settings or contact your administrator", "Неизвестная ошибка, пожалуйста, проверьте системные настройки или свяжитесь с администратором"),
        ("Missing requirements.", "Требования отсутствуют."),
        ("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Пожалуйста, убедитесь, что версия PHP 5.3.3 или новее, а также, что OpenSSL и соответствующее расширение PHP включены и правильно настроены. На данный момент приложение шифрования отключено."),
        ("Following users are not set up for encryption:", "Для следующих пользователей шифрование не настроено:"),
        ("Saving...", "Сохранение..."),
        ("Go directly to your ", "Перейти прямо в"),
        ("personal settings", "персональные настройки"),
        ("Encryption", "Шифрование"),
        ("Enable recovery key (allow to recover users files in case of password loss):", "Включить ключ восстановления (позволяет пользователям восстановить файлы при потере пароля):"),
        ("Recovery key password", "Пароль для ключа восстановления"),
        ("Repeat Recovery key password", "Повторите пароль восстановления ключа"),
        ("Enabled", "Включено"),
        ("Disabled", "Отключено"),
        ("Change recovery key password:", "Сменить пароль для ключа восстановления:"),
        ("Old Recovery key password", "Старый пароль для ключа восстановления"),
        ("New Recovery key password", "Новый пароль для ключа восстановления"),
        ("Repeat New Recovery key password", "Повторите новый пароль восстановления ключа"),
        ("Change Password", "Изменить пароль"),
        ("Your private key password no longer match your log-in password:", "Пароль от секретного ключа больше не соответствует паролю входа:"),
        ("Set your old private key password to your current log-in password.", "Замените старый пароль от секретного ключа на новый пароль входа."),
        (" If you don't remember your old password you can ask your administrator to recover your files.", "Если вы не помните свой старый пароль, вы можете попросить своего администратора восстановить ваши файлы"),
        ("Old log-in password", "Старый пароль для входа"),
        ("Current log-in password", "Текущйи пароль для входа"),
        ("Update Private Key Password", "Обновить пароль от секретного ключа"),
        ("Enable password recovery:", "Включить восстановление пароля:"),
        ("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Включение этой опции позволит вам получить доступ к своим зашифрованным файлам в случае утери пароля"),
        ("File recovery settings updated", "Настройки файла восстановления обновлены"),
        ("Could not update file recovery", "Невозможно обновить файл восстановления"),
    ]);

    let plural_forms = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";

    (translations, plural_forms)
}

/// Registers Russian translations with the i18n system
pub fn register_russian_locale() -> Result<(), Box<dyn std::error::Error>> {
    let (translations, plural_forms) = get_russian_translations();
    let locale = Locale::new("ru", translations, plural_forms)?;
    
    // Register with the i18n system (implementation would depend on the specific i18n library used)
    // For example: i18n::register_locale(locale);
    
    Ok(())
}