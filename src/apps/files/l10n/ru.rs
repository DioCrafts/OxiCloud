use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut t = HashMap::new();

    t.insert("Could not move %s - File with this name already exists", "Невозможно переместить %s - файл с таким именем уже существует");
    t.insert("Could not move %s", "Невозможно переместить %s");
    t.insert("File name cannot be empty.", "Имя файла не может быть пустым.");
    t.insert("File name must not contain \"/\". Please choose a different name.", "Имя файла не должно содержать символ \"/\". Пожалуйста, выберите другое имя.");
    t.insert("The name %s is already used in the folder %s. Please choose a different name.", "Имя %s уже используется в папке %s. Пожалуйста выберите другое имя.");
    t.insert("Not a valid source", "Неправильный источник");
    t.insert("Error while downloading %s to %s", "Ошибка при загрузке %s в %s");
    t.insert("Error when creating the file", "Ошибка при создании файла");
    t.insert("Folder name cannot be empty.", "Имя папки не может быть пустым.");
    t.insert("Folder name must not contain \"/\". Please choose a different name.", "Имя папки не должно содержать символ \"/\". Пожалуйста, выберите другое имя.");
    t.insert("Error when creating the folder", "Ошибка при создании папки");
    t.insert("Unable to set upload directory.", "Не удалось установить каталог загрузки.");
    t.insert("Invalid Token", "Недопустимый маркер");
    t.insert("No file was uploaded. Unknown error", "Файл не был загружен. Неизвестная ошибка");
    t.insert("There is no error, the file uploaded with success", "Файл загружен успешно.");
    t.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Файл превышает размер установленный upload_max_filesize в php.ini:");
    t.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Загружаемый файл превосходит значение переменной MAX_FILE_SIZE, указанной в форме HTML");
    t.insert("The uploaded file was only partially uploaded", "Файл загружен частично");
    t.insert("No file was uploaded", "Файл не был загружен");
    t.insert("Missing a temporary folder", "Отсутствует временная папка");
    t.insert("Failed to write to disk", "Ошибка записи на диск");
    t.insert("Not enough storage available", "Недостаточно доступного места в хранилище");
    t.insert("Upload failed. Could not get file info.", "Загрузка не удалась. Невозможно получить информацию о файле");
    t.insert("Upload failed. Could not find uploaded file", "Загрузка не удалась. Невозможно найти загруженный файл");
    t.insert("Invalid directory.", "Неправильный каталог.");
    t.insert("Files", "Файлы");
    t.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Невозможно загрузить файл  {filename} так как он является директорией либо имеет размер 0 байт");
    t.insert("Not enough space available", "Недостаточно свободного места");
    t.insert("Upload cancelled.", "Загрузка отменена.");
    t.insert("Could not get result from server.", "Не получен ответ от сервера");
    t.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Файл в процессе загрузки. Покинув страницу вы прервёте загрузку.");
    t.insert("URL cannot be empty", "Ссылка не может быть пустой.");
    t.insert("In the home folder 'Shared' is a reserved filename", "В домашней папке 'Shared' зарезервированное имя файла");
    t.insert("{new_name} already exists", "{new_name} уже существует");
    t.insert("Could not create file", "Не удалось создать файл");
    t.insert("Could not create folder", "Не удалось создать папку");
    t.insert("Share", "Открыть доступ");
    t.insert("Delete permanently", "Удалено навсегда");
    t.insert("Rename", "Переименовать");
    t.insert("Pending", "Ожидание");
    t.insert("Could not rename file", "Не удалось переименовать файл");
    t.insert("replaced {new_name} with {old_name}", "заменено {new_name} на {old_name}");
    t.insert("undo", "отмена");
    t.insert("{dirs} and {files}", "{dirs} и {files}");
    t.insert("'.' is an invalid file name.", "'.' - неправильное имя файла.");
    t.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Неправильное имя, '\\', '/', '<', '>', ':', '\"', '|', '?' и '*' недопустимы.");
    t.insert("Your storage is full, files can not be updated or synced anymore!", "Ваше дисковое пространство полностью заполнено, произведите очистку перед загрузкой новых файлов.");
    t.insert("Your storage is almost full ({usedSpacePercent}%)", "Ваше хранилище почти заполнено ({usedSpacePercent}%)");
    t.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "Приложение для шифрования активно, но ваши ключи не инициализированы, пожалуйста, перелогиньтесь");
    t.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "Неверный приватный ключ для приложения шифрования. Пожалуйста, обноваите ваш приватный ключ в персональных настройках чтобы восстановить доступ к вашим зашифрованным файлам.");
    t.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "Шифрование было отключено, но ваши файлы все еще зашифрованы. Пожалуйста, зайдите на страницу персональных настроек для того, чтобы расшифровать ваши файлы.");
    t.insert("Your download is being prepared. This might take some time if the files are big.", "Загрузка началась. Это может потребовать много времени, если файл большого размера.");
    t.insert("Error moving file", "Ошибка при перемещении файла");
    t.insert("Error", "Ошибка");
    t.insert("Name", "Имя");
    t.insert("Size", "Размер");
    t.insert("Modified", "Изменён");
    t.insert("Invalid folder name. Usage of 'Shared' is reserved.", "Неправильное имя каталога. Имя 'Shared' зарезервировано.");
    t.insert("%s could not be renamed", "%s не может быть переименован");
    t.insert("Upload", "Загрузка");
    t.insert("File handling", "Управление файлами");
    t.insert("Maximum upload size", "Максимальный размер загружаемого файла");
    t.insert("max. possible: ", "макс. возможно: ");
    t.insert("Needed for multi-file and folder downloads.", "Требуется для скачивания нескольких файлов и папок");
    t.insert("Enable ZIP-download", "Включить ZIP-скачивание");
    t.insert("0 is unlimited", "0 - без ограничений");
    t.insert("Maximum input size for ZIP files", "Максимальный исходный размер для ZIP файлов");
    t.insert("Save", "Сохранить");
    t.insert("New", "Новый");
    t.insert("Text file", "Текстовый файл");
    t.insert("Folder", "Папка");
    t.insert("From link", "Из ссылки");
    t.insert("Deleted files", "Удалённые файлы");
    t.insert("Cancel upload", "Отмена загрузки");
    t.insert("You don't have permission to upload or create files here", "У вас недостаточно прав для загрузки или создания файлов отсюда.");
    t.insert("Nothing in here. Upload something!", "Здесь ничего нет. Загрузите что-нибудь!");
    t.insert("Download", "Скачать");
    t.insert("Unshare", "Закрыть общий доступ");
    t.insert("Delete", "Удалить");
    t.insert("Upload too large", "Файл слишком велик");
    t.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Файлы, которые вы пытаетесь загрузить, превышают лимит для файлов на этом сервере.");
    t.insert("Files are being scanned, please wait.", "Подождите, файлы сканируются.");
    t.insert("Current scanning", "Текущее сканирование");
    t.insert("Upgrading filesystem cache...", "Обновление кэша файловой системы...");

    t
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";

pub fn plural_text(key: &str, count: usize) -> Option<&'static str> {
    match (key, count % 10, count % 100) {
        ("_%n folder_::_%n folders_", 1, n) if n != 11 => Some("%n папка"),
        ("_%n folder_::_%n folders_", 2..=4, n) if n < 10 || n >= 20 => Some("%n папки"),
        ("_%n folder_::_%n folders_", _, _) => Some("%n папок"),
        
        ("_%n file_::_%n files_", 1, n) if n != 11 => Some("%n файл"),
        ("_%n file_::_%n files_", 2..=4, n) if n < 10 || n >= 20 => Some("%n файла"),
        ("_%n file_::_%n files_", _, _) => Some("%n файлов"),
        
        ("_Uploading %n file_::_Uploading %n files_", 1, n) if n != 11 => Some("Закачка %n файла"),
        ("_Uploading %n file_::_Uploading %n files_", 2..=4, n) if n < 10 || n >= 20 => Some("Закачка %n файлов"),
        ("_Uploading %n file_::_Uploading %n files_", _, _) => Some("Закачка %n файлов"),
        
        _ => None,
    }
}