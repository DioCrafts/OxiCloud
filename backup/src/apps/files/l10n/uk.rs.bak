use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "Не вдалося перемістити %s - Файл з таким ім'ям вже існує");
        m.insert("Could not move %s", "Не вдалося перемістити %s");
        m.insert("File name cannot be empty.", " Ім'я файлу не може бути порожнім.");
        m.insert("Folder name cannot be empty.", "Ім'я теки не може бути порожнім.");
        m.insert("Unable to set upload directory.", "Не вдалося встановити каталог завантаження.");
        m.insert("No file was uploaded. Unknown error", "Не завантажено жодного файлу. Невідома помилка");
        m.insert("There is no error, the file uploaded with success", "Файл успішно вивантажено без помилок.");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Розмір звантаження перевищує upload_max_filesize параметра в php.ini: ");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Розмір відвантаженого файлу перевищує директиву MAX_FILE_SIZE вказану в HTML формі");
        m.insert("The uploaded file was only partially uploaded", "Файл відвантажено лише частково");
        m.insert("No file was uploaded", "Не відвантажено жодного файлу");
        m.insert("Missing a temporary folder", "Відсутній тимчасовий каталог");
        m.insert("Failed to write to disk", "Невдалося записати на диск");
        m.insert("Not enough storage available", "Місця більше немає");
        m.insert("Invalid directory.", "Невірний каталог.");
        m.insert("Files", "Файли");
        m.insert("Not enough space available", "Місця більше немає");
        m.insert("Upload cancelled.", "Завантаження перервано.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Виконується завантаження файлу. Закриття цієї сторінки приведе до відміни завантаження.");
        m.insert("URL cannot be empty", "URL не може бути порожнім");
        m.insert("{new_name} already exists", "{new_name} вже існує");
        m.insert("Could not create file", "Не вдалося створити файл");
        m.insert("Could not create folder", "Не вдалося створити теку");
        m.insert("Share", "Поділитися");
        m.insert("Delete permanently", "Видалити назавжди");
        m.insert("Rename", "Перейменувати");
        m.insert("Pending", "Очікування");
        m.insert("Could not rename file", "Неможливо перейменувати файл");
        m.insert("replaced {new_name} with {old_name}", "замінено {new_name} на {old_name}");
        m.insert("undo", "відмінити");
        m.insert("'.' is an invalid file name.", "'.' це невірне ім'я файлу.");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Невірне ім'я, '\\', '/', '<', '>', ':', '\"', '|', '?' та '*' не дозволені.");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "Ваше сховище переповнене, файли більше не можуть бути оновлені або синхронізовані !");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "Ваше сховище майже повне ({usedSpacePercent}%)");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "Ваше завантаження готується. Це може зайняти деякий час, якщо файли завеликі.");
        m.insert("Error moving file", "Помилка переміщення файлу");
        m.insert("Error", "Помилка");
        m.insert("Name", "Ім'я");
        m.insert("Size", "Розмір");
        m.insert("Modified", "Змінено");
        m.insert("%s could not be renamed", "%s не може бути перейменований");
        m.insert("Upload", "Вивантажити");
        m.insert("File handling", "Робота з файлами");
        m.insert("Maximum upload size", "Максимальний розмір відвантажень");
        m.insert("max. possible: ", "макс.можливе:");
        m.insert("Needed for multi-file and folder downloads.", "Необхідно для мульти-файлового та каталогового завантаження.");
        m.insert("Enable ZIP-download", "Активувати ZIP-завантаження");
        m.insert("0 is unlimited", "0 є безліміт");
        m.insert("Maximum input size for ZIP files", "Максимальний розмір завантажуємого ZIP файлу");
        m.insert("Save", "Зберегти");
        m.insert("New", "Створити");
        m.insert("Text file", "Текстовий файл");
        m.insert("Folder", "Тека");
        m.insert("From link", "З посилання");
        m.insert("Deleted files", "Видалено файлів");
        m.insert("Cancel upload", "Перервати завантаження");
        m.insert("Nothing in here. Upload something!", "Тут нічого немає. Відвантажте що-небудь!");
        m.insert("Download", "Завантажити");
        m.insert("Unshare", "Закрити доступ");
        m.insert("Delete", "Видалити");
        m.insert("Upload too large", "Файл занадто великий");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Файли,що ви намагаєтесь відвантажити перевищують максимальний дозволений розмір файлів на цьому сервері.");
        m.insert("Files are being scanned, please wait.", "Файли скануються, зачекайте, будь-ласка.");
        m.insert("Current scanning", "Поточне сканування");
        m.insert("Upgrading filesystem cache...", "Оновлення кеша файлової системи...");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n folder_::_%n folders_", vec!["%n тека", "%n тека", "%n теки"]);
        m.insert("_%n file_::_%n files_", vec!["%n файл", "%n файлів", "%n файли"]);
        m.insert("_Uploading %n file_::_Uploading %n files_", vec!["", "", ""]);
        m
    };
}

pub fn get_plural_form(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}

pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}

pub fn translate_plural(key: &str, count: i64) -> &'static str {
    if let Some(forms) = PLURAL_FORMS.get(key) {
        let form_index = get_plural_form(count);
        if form_index < forms.len() {
            return forms[form_index];
        }
    }
    key
}