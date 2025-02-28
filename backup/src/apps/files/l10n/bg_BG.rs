use std::collections::HashMap;
use rust_i18n::i18n;

// Language file for Bulgarian (bg_BG)
pub fn register_translations() -> HashMap<String, rust_i18n::Translation> {
    let mut translations = HashMap::new();
    
    translations.insert(
        "There is no error, the file uploaded with success".to_string(),
        "Файлът е качен успешно".to_string(),
    );
    translations.insert(
        "The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form".to_string(),
        "Файлът който се опитвате да качите надвишава стойностите в MAX_FILE_SIZE в HTML формата.".to_string(),
    );
    translations.insert(
        "The uploaded file was only partially uploaded".to_string(),
        "Файлът е качен частично".to_string(),
    );
    translations.insert(
        "No file was uploaded".to_string(),
        "Фахлът не бе качен".to_string(),
    );
    translations.insert(
        "Missing a temporary folder".to_string(),
        "Липсва временна папка".to_string(),
    );
    translations.insert(
        "Failed to write to disk".to_string(),
        "Възникна проблем при запис в диска".to_string(),
    );
    translations.insert(
        "Invalid directory.".to_string(),
        "Невалидна директория.".to_string(),
    );
    translations.insert(
        "Files".to_string(),
        "Файлове".to_string(),
    );
    translations.insert(
        "Upload cancelled.".to_string(),
        "Качването е спряно.".to_string(),
    );
    translations.insert(
        "Share".to_string(),
        "Споделяне".to_string(),
    );
    translations.insert(
        "Delete permanently".to_string(),
        "Изтриване завинаги".to_string(),
    );
    translations.insert(
        "Rename".to_string(),
        "Преименуване".to_string(),
    );
    translations.insert(
        "Pending".to_string(),
        "Чакащо".to_string(),
    );
    translations.insert(
        "undo".to_string(),
        "възтановяване".to_string(),
    );
    
    // Plurals
    let mut folder_plural = HashMap::new();
    folder_plural.insert(0, "".to_string());
    folder_plural.insert(1, "".to_string());
    translations.insert(
        "_%n folder_::_%n folders_".to_string(),
        rust_i18n::Translation::Plural(folder_plural),
    );
    
    let mut file_plural = HashMap::new();
    file_plural.insert(0, "".to_string());
    file_plural.insert(1, "".to_string());
    translations.insert(
        "_%n file_::_%n files_".to_string(),
        rust_i18n::Translation::Plural(file_plural),
    );
    
    let mut uploading_plural = HashMap::new();
    uploading_plural.insert(0, "".to_string());
    uploading_plural.insert(1, "".to_string());
    translations.insert(
        "_Uploading %n file_::_Uploading %n files_".to_string(),
        rust_i18n::Translation::Plural(uploading_plural),
    );
    
    translations.insert(
        "Error".to_string(),
        "Грешка".to_string(),
    );
    translations.insert(
        "Name".to_string(),
        "Име".to_string(),
    );
    translations.insert(
        "Size".to_string(),
        "Размер".to_string(),
    );
    translations.insert(
        "Modified".to_string(),
        "Променено".to_string(),
    );
    translations.insert(
        "Upload".to_string(),
        "Качване".to_string(),
    );
    translations.insert(
        "Maximum upload size".to_string(),
        "Максимален размер за качване".to_string(),
    );
    translations.insert(
        "0 is unlimited".to_string(),
        "Ползвайте 0 за без ограничения".to_string(),
    );
    translations.insert(
        "Save".to_string(),
        "Запис".to_string(),
    );
    translations.insert(
        "New".to_string(),
        "Ново".to_string(),
    );
    translations.insert(
        "Text file".to_string(),
        "Текстов файл".to_string(),
    );
    translations.insert(
        "Folder".to_string(),
        "Папка".to_string(),
    );
    translations.insert(
        "Cancel upload".to_string(),
        "Спри качването".to_string(),
    );
    translations.insert(
        "Nothing in here. Upload something!".to_string(),
        "Няма нищо тук. Качете нещо.".to_string(),
    );
    translations.insert(
        "Download".to_string(),
        "Изтегляне".to_string(),
    );
    translations.insert(
        "Delete".to_string(),
        "Изтриване".to_string(),
    );
    translations.insert(
        "Upload too large".to_string(),
        "Файлът който сте избрали за качване е прекалено голям".to_string(),
    );
    translations.insert(
        "The files you are trying to upload exceed the maximum size for file uploads on this server.".to_string(),
        "Файловете които се опитвате да качите са по-големи от позволеното за сървъра.".to_string(),
    );
    translations.insert(
        "Files are being scanned, please wait.".to_string(),
        "Файловете се претърсват, изчакайте.".to_string(),
    );
    
    // Set the plural forms rule
    i18n::set_plural_rule("bg_BG", "nplurals=2; plural=(n != 1);");
    
    translations
}