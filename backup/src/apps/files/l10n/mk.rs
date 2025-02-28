use std::collections::HashMap;
use rust_i18n::i18n;

/// Module containing Macedonian (mk) translations for the files app
pub fn get_mk_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Could not move %s - File with this name already exists".to_string(), "Не можам да го преместам %s - Датотека со такво име веќе постои".to_string());
    translations.insert("Could not move %s".to_string(), "Не можам да ги префрлам %s".to_string());
    translations.insert("File name cannot be empty.".to_string(), "Името на датотеката не може да биде празно.".to_string());
    translations.insert("Not a valid source".to_string(), "Не е валиден извор".to_string());
    translations.insert("Error while downloading %s to %s".to_string(), "Грешка додека преземам %s to %s".to_string());
    translations.insert("Error when creating the file".to_string(), "Грешка при креирање на датотека".to_string());
    translations.insert("Folder name cannot be empty.".to_string(), "Името на папката не може да биде празно.".to_string());
    translations.insert("Folder name must not contain \"/\". Please choose a different name.".to_string(), "Името на папката не смее да содржи \"/\". Одберете друго име.".to_string());
    translations.insert("Error when creating the folder".to_string(), "Грешка при креирање на папка".to_string());
    translations.insert("Unable to set upload directory.".to_string(), "Не може да се постави папката за префрлање на податоци.".to_string());
    translations.insert("Invalid Token".to_string(), "Грешен токен".to_string());
    translations.insert("No file was uploaded. Unknown error".to_string(), "Ниту еден фајл не се вчита. Непозната грешка".to_string());
    translations.insert("There is no error, the file uploaded with success".to_string(), "Датотеката беше успешно подигната.".to_string());
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ".to_string(), "Подигнатата датотека ја надминува upload_max_filesize директивата во php.ini:".to_string());
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form".to_string(), "Големината на датотеката ја надминува MAX_FILE_SIZE директивата која беше специфицирана во HTML формата".to_string());
    translations.insert("The uploaded file was only partially uploaded".to_string(), "Датотеката беше само делумно подигната.".to_string());
    translations.insert("No file was uploaded".to_string(), "Не беше подигната датотека.".to_string());
    translations.insert("Missing a temporary folder".to_string(), "Недостасува привремена папка".to_string());
    translations.insert("Failed to write to disk".to_string(), "Неуспеав да запишам на диск".to_string());
    translations.insert("Not enough storage available".to_string(), "Нема доволно слободен сториџ".to_string());
    translations.insert("Upload failed. Could not find uploaded file".to_string(), "Префрлањето е неуспешно. Не можам да го најдам префрлената датотека.".to_string());
    translations.insert("Invalid directory.".to_string(), "Погрешна папка.".to_string());
    translations.insert("Files".to_string(), "Датотеки".to_string());
    translations.insert("Not enough space available".to_string(), "Немате доволно дисков простор".to_string());
    translations.insert("Upload cancelled.".to_string(), "Преземањето е прекинато.".to_string());
    translations.insert("Could not get result from server.".to_string(), "Не можам да добијам резултат од серверот.".to_string());
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.".to_string(), "Подигање на датотека е во тек. Напуштење на страницата ќе го прекине.".to_string());
    translations.insert("URL cannot be empty".to_string(), "URL-то не може да биде празно".to_string());
    translations.insert("In the home folder 'Shared' is a reserved filename".to_string(), "Во домашната папка, 'Shared' е резервирано има на датотека/папка".to_string());
    translations.insert("{new_name} already exists".to_string(), "{new_name} веќе постои".to_string());
    translations.insert("Could not create file".to_string(), "Не множам да креирам датотека".to_string());
    translations.insert("Could not create folder".to_string(), "Не можам да креирам папка".to_string());
    translations.insert("Share".to_string(), "Сподели".to_string());
    translations.insert("Delete permanently".to_string(), "Трајно избришани".to_string());
    translations.insert("Rename".to_string(), "Преименувај".to_string());
    translations.insert("Pending".to_string(), "Чека".to_string());
    translations.insert("Could not rename file".to_string(), "Не можам да ја преименувам датотеката".to_string());
    translations.insert("replaced {new_name} with {old_name}".to_string(), "заменета {new_name} со {old_name}".to_string());
    translations.insert("undo".to_string(), "врати".to_string());
    translations.insert("{dirs} and {files}".to_string(), "{dirs} и {files}".to_string());
    translations.insert("'.' is an invalid file name.".to_string(), "'.' е грешно име за датотека.".to_string());
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.".to_string(), "Неправилно име. , '\\', '/', '<', '>', ':', '\"', '|', '?' и '*' не се дозволени.".to_string());
    translations.insert("Your storage is full, files can not be updated or synced anymore!".to_string(), "Вашиот сториџ е полн, датотеките веќе не можат да се освежуваат или синхронизираат!".to_string());
    translations.insert("Your storage is almost full ({usedSpacePercent}%)".to_string(), "Вашиот сториџ е скоро полн ({usedSpacePercent}%)".to_string());
    translations.insert("Your download is being prepared. This might take some time if the files are big.".to_string(), "Вашето преземање се подготвува. Ова може да потрае до колку датотеките се големи.".to_string());
    translations.insert("Error moving file".to_string(), "Грешка при префрлање на датотека".to_string());
    translations.insert("Error".to_string(), "Грешка".to_string());
    translations.insert("Name".to_string(), "Име".to_string());
    translations.insert("Size".to_string(), "Големина".to_string());
    translations.insert("Modified".to_string(), "Променето".to_string());
    translations.insert("%s could not be renamed".to_string(), "%s не може да биде преименуван".to_string());
    translations.insert("Upload".to_string(), "Подигни".to_string());
    translations.insert("File handling".to_string(), "Ракување со датотеки".to_string());
    translations.insert("Maximum upload size".to_string(), "Максимална големина за подигање".to_string());
    translations.insert("max. possible: ".to_string(), "макс. можно:".to_string());
    translations.insert("Needed for multi-file and folder downloads.".to_string(), "Потребно за симнување повеќе-датотеки и папки.".to_string());
    translations.insert("Enable ZIP-download".to_string(), "Овозможи ZIP симнување ".to_string());
    translations.insert("0 is unlimited".to_string(), "0 е неограничено".to_string());
    translations.insert("Maximum input size for ZIP files".to_string(), "Максимална големина за внес на ZIP датотеки".to_string());
    translations.insert("Save".to_string(), "Сними".to_string());
    translations.insert("New".to_string(), "Ново".to_string());
    translations.insert("Text file".to_string(), "Текстуална датотека".to_string());
    translations.insert("Folder".to_string(), "Папка".to_string());
    translations.insert("From link".to_string(), "Од врска".to_string());
    translations.insert("Deleted files".to_string(), "Избришани датотеки".to_string());
    translations.insert("Cancel upload".to_string(), "Откажи прикачување".to_string());
    translations.insert("Nothing in here. Upload something!".to_string(), "Тука нема ништо. Снимете нешто!".to_string());
    translations.insert("Download".to_string(), "Преземи".to_string());
    translations.insert("Unshare".to_string(), "Не споделувај".to_string());
    translations.insert("Delete".to_string(), "Избриши".to_string());
    translations.insert("Upload too large".to_string(), "Фајлот кој се вчитува е преголем".to_string());
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.".to_string(), "Датотеките кои се обидувате да ги подигнете ја надминуваат максималната големина за подигнување датотеки на овој сервер.".to_string());
    translations.insert("Files are being scanned, please wait.".to_string(), "Се скенираат датотеки, ве молам почекајте.".to_string());
    translations.insert("Current scanning".to_string(), "Моментално скенирам".to_string());
    translations.insert("Upgrading filesystem cache...".to_string(), "Го надградувам кешот на фјал системот...".to_string());
    
    // Plural forms handling
    let plural_definitions = HashMap::new();
    i18n!("mk", translations, plural_definitions);
    
    // Define the plural form rule for Macedonian
    fn get_plural_form(n: i64) -> usize {
        if n % 10 == 1 && n % 100 != 11 {
            0
        } else {
            1
        }
    }
    
    // Add plural form entries
    let mut plural_entries = HashMap::new();
    
    // _%n folder_::_%n folders_
    plural_entries.insert("folder".to_string(), vec!["".to_string(), "".to_string()]);
    
    // _%n file_::_%n files_
    plural_entries.insert("file".to_string(), vec!["".to_string(), "".to_string()]);
    
    // _Uploading %n file_::_Uploading %n files_
    plural_entries.insert("uploading".to_string(), vec!["".to_string(), "".to_string()]);
    
    translations
}