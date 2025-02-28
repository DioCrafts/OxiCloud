use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "Не могу да преместим %s – датотека с овим именом већ постоји");
        m.insert("Could not move %s", "Не могу да преместим %s");
        m.insert("File name cannot be empty.", "Име датотеке не може бити празно.");
        m.insert("No file was uploaded. Unknown error", "Ниједна датотека није отпремљена услед непознате грешке");
        m.insert("There is no error, the file uploaded with success", "Није дошло до грешке. Датотека је успешно отпремљена.");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Отпремљена датотека прелази смерницу upload_max_filesize у датотеци php.ini:");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Отпремљена датотека прелази смерницу MAX_FILE_SIZE која је наведена у HTML обрасцу");
        m.insert("The uploaded file was only partially uploaded", "Датотека је делимично отпремљена");
        m.insert("No file was uploaded", "Датотека није отпремљена");
        m.insert("Missing a temporary folder", "Недостаје привремена фасцикла");
        m.insert("Failed to write to disk", "Не могу да пишем на диск");
        m.insert("Not enough storage available", "Нема довољно простора");
        m.insert("Invalid directory.", "неисправна фасцикла.");
        m.insert("Files", "Датотеке");
        m.insert("Not enough space available", "Нема довољно простора");
        m.insert("Upload cancelled.", "Отпремање је прекинуто.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Отпремање датотеке је у току. Ако сада напустите страницу, прекинућете отпремање.");
        m.insert("{new_name} already exists", "{new_name} већ постоји");
        m.insert("Share", "Дели");
        m.insert("Delete permanently", "Обриши за стално");
        m.insert("Rename", "Преименуј");
        m.insert("Pending", "На чекању");
        m.insert("replaced {new_name} with {old_name}", "замењено {new_name} са {old_name}");
        m.insert("undo", "опозови");
        m.insert("'.' is an invalid file name.", "Датотека „." је неисправног имена.");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Неисправан назив. Следећи знакови нису дозвољени: \\, /, <, >, :, \", |, ? и *.");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "Ваше складиште је пуно. Датотеке више не могу бити ажуриране ни синхронизоване.");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "Ваше складиште је скоро па пуно ({usedSpacePercent}%)");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "Припремам преузимање. Ово може да потраје ако су датотеке велике.");
        m.insert("Error", "Грешка");
        m.insert("Name", "Име");
        m.insert("Size", "Величина");
        m.insert("Modified", "Измењено");
        m.insert("Upload", "Отпреми");
        m.insert("File handling", "Управљање датотекама");
        m.insert("Maximum upload size", "Највећа величина датотеке");
        m.insert("max. possible: ", "највећа величина:");
        m.insert("Needed for multi-file and folder downloads.", "Неопходно за преузимање вишеделних датотека и фасцикли.");
        m.insert("Enable ZIP-download", "Омогући преузимање у ZIP-у");
        m.insert("0 is unlimited", "0 је неограничено");
        m.insert("Maximum input size for ZIP files", "Највећа величина ZIP датотека");
        m.insert("Save", "Сачувај");
        m.insert("New", "Нова");
        m.insert("Text file", "текстуална датотека");
        m.insert("Folder", "фасцикла");
        m.insert("From link", "Са везе");
        m.insert("Deleted files", "Обрисане датотеке");
        m.insert("Cancel upload", "Прекини отпремање");
        m.insert("Nothing in here. Upload something!", "Овде нема ничег. Отпремите нешто!");
        m.insert("Download", "Преузми");
        m.insert("Unshare", "Укини дељење");
        m.insert("Delete", "Обриши");
        m.insert("Upload too large", "Датотека је превелика");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Датотеке које желите да отпремите прелазе ограничење у величини.");
        m.insert("Files are being scanned, please wait.", "Скенирам датотеке…");
        m.insert("Current scanning", "Тренутно скенирање");
        m.insert("Upgrading filesystem cache...", "Дограђујем кеш система датотека…");
        m
    };

    pub static ref PLURAL_FORMS: PluralForms = PluralForms {
        nplurals: 3,
        plural_expression: "(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2)",
        plural_map: {
            let mut map = HashMap::new();
            map.insert("_%n folder_::_%n folders_", vec!["", "", ""]);
            map.insert("_%n file_::_%n files_", vec!["", "", ""]);
            map.insert("_Uploading %n file_::_Uploading %n files_", vec!["", "", ""]);
            map
        },
    };
}

pub struct PluralForms {
    pub nplurals: usize,
    pub plural_expression: &'static str,
    pub plural_map: HashMap<&'static str, Vec<&'static str>>,
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form(key: &str, count: usize) -> Option<&'static str> {
    let form = calculate_plural_form(count);
    PLURAL_FORMS.plural_map.get(key).and_then(|forms| forms.get(form).copied())
}

fn calculate_plural_form(n: usize) -> usize {
    let n = n as u64;
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}