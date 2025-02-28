use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::plural_forms::PluralCategory;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Could not move %s - File with this name already exists", "Nie je možné presunúť %s - súbor s týmto menom už existuje");
        map.insert("Could not move %s", "Nie je možné presunúť %s");
        map.insert("File name cannot be empty.", "Meno súboru nemôže byť prázdne");
        map.insert("Unable to set upload directory.", "Nemožno nastaviť priečinok pre nahrané súbory.");
        map.insert("Invalid Token", "Neplatný token");
        map.insert("No file was uploaded. Unknown error", "Žiaden súbor nebol nahraný. Neznáma chyba");
        map.insert("There is no error, the file uploaded with success", "Nenastala žiadna chyba, súbor bol úspešne nahraný");
        map.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Nahraný súbor prekročil limit nastavený v upload_max_filesize v súbore php.ini:");
        map.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Ukladaný súbor prekračuje nastavenie MAX_FILE_SIZE z volieb HTML formulára.");
        map.insert("The uploaded file was only partially uploaded", "Ukladaný súbor sa nahral len čiastočne");
        map.insert("No file was uploaded", "Žiadny súbor nebol uložený");
        map.insert("Missing a temporary folder", "Chýba dočasný priečinok");
        map.insert("Failed to write to disk", "Zápis na disk sa nepodaril");
        map.insert("Not enough storage available", "Nedostatok dostupného úložného priestoru");
        map.insert("Upload failed. Could not get file info.", "Nahrávanie zlyhalo. Nepodarilo sa získať informácie o súbore.");
        map.insert("Upload failed. Could not find uploaded file", "Nahrávanie zlyhalo. Nepodarilo sa nájsť nahrávaný súbor");
        map.insert("Invalid directory.", "Neplatný priečinok.");
        map.insert("Files", "Súbory");
        map.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Nemožno nahrať súbor {filename}, pretože je to priečinok, alebo má 0 bitov");
        map.insert("Not enough space available", "Nie je k dispozícii dostatok miesta");
        map.insert("Upload cancelled.", "Odosielanie zrušené.");
        map.insert("Could not get result from server.", "Nepodarilo sa dostať výsledky zo servera.");
        map.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Opustenie stránky zruší práve prebiehajúce odosielanie súboru.");
        map.insert("{new_name} already exists", "{new_name} už existuje");
        map.insert("Share", "Zdieľať");
        map.insert("Delete permanently", "Zmazať  trvalo");
        map.insert("Rename", "Premenovať");
        map.insert("Pending", "Prebieha");
        map.insert("replaced {new_name} with {old_name}", "prepísaný {new_name} súborom {old_name}");
        map.insert("undo", "vrátiť");
        map.insert("_{dirs} and {files}", "{dirs} a {files}");
        map.insert("'.' is an invalid file name.", "'.' je neplatné meno súboru.");
        map.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Nesprávne meno, '\\', '/', '<', '>', ':', '\"', '|', '?' a '*' nie sú povolené hodnoty.");
        map.insert("Your storage is full, files can not be updated or synced anymore!", "Vaše úložisko je plné. Súbory nemožno aktualizovať ani synchronizovať!");
        map.insert("Your storage is almost full ({usedSpacePercent}%)", "Vaše úložisko je takmer plné ({usedSpacePercent}%)");
        map.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "Aplikácia na šifrovanie je zapnutá, ale vaše kľúče nie sú inicializované. Odhláste sa a znovu sa prihláste.");
        map.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "Chybný súkromný kľúč na šifrovanie aplikácií. Zaktualizujte si heslo súkromného kľúča v svojom osobnom nastavení, aby ste znovu získali prístup k svojim zašifrovaným súborom.");
        map.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "Šifrovanie bolo zakázané, ale vaše súbory sú stále zašifrované. Prosím, choďte do osobného nastavenia pre dešifrovanie súborov.");
        map.insert("Your download is being prepared. This might take some time if the files are big.", "Vaše sťahovanie sa pripravuje. Ak sú sťahované súbory veľké, môže to chvíľu trvať.");
        map.insert("Error moving file", "Chyba pri presúvaní súboru");
        map.insert("Error", "Chyba");
        map.insert("Name", "Názov");
        map.insert("Size", "Veľkosť");
        map.insert("Modified", "Upravené");
        map.insert("%s could not be renamed", "%s nemohol byť premenovaný");
        map.insert("Upload", "Odoslať");
        map.insert("File handling", "Nastavenie správania sa k súborom");
        map.insert("Maximum upload size", "Maximálna veľkosť odosielaného súboru");
        map.insert("max. possible: ", "najväčšie možné:");
        map.insert("Needed for multi-file and folder downloads.", "Vyžadované pre sťahovanie viacerých súborov a priečinkov.");
        map.insert("Enable ZIP-download", "Povoliť sťahovanie ZIP súborov");
        map.insert("0 is unlimited", "0 znamená neobmedzené");
        map.insert("Maximum input size for ZIP files", "Najväčšia veľkosť ZIP súborov");
        map.insert("Save", "Uložiť");
        map.insert("New", "Nová");
        map.insert("Text file", "Textový súbor");
        map.insert("Folder", "Priečinok");
        map.insert("From link", "Z odkazu");
        map.insert("Deleted files", "Zmazané súbory");
        map.insert("Cancel upload", "Zrušiť odosielanie");
        map.insert("Nothing in here. Upload something!", "Žiadny súbor. Nahrajte niečo!");
        map.insert("Download", "Sťahovanie");
        map.insert("Unshare", "Zrušiť zdieľanie");
        map.insert("Delete", "Zmazať");
        map.insert("Upload too large", "Nahrávanie je príliš veľké");
        map.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Súbory, ktoré sa snažíte nahrať, presahujú maximálnu veľkosť pre nahratie súborov na tento server.");
        map.insert("Files are being scanned, please wait.", "Čakajte, súbory sú prehľadávané.");
        map.insert("Current scanning", "Práve prezerané");
        map.insert("Upgrading filesystem cache...", "Aktualizujem medzipamäť súborového systému...");
        map
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("_%n folder_::_%n folders_", vec!["%n priečinok", "%n priečinky", "%n priečinkov"]);
        map.insert("_%n file_::_%n files_", vec!["%n súbor", "%n súbory", "%n súborov"]);
        map.insert("_Uploading %n file_::_Uploading %n files_", vec!["Nahrávam %n súbor", "Nahrávam %n súbory", "Nahrávam %n súborov"]);
        map
    };
}

// Función que implementa la lógica de pluralización del idioma eslovaco
pub fn get_plural_category(n: usize) -> PluralCategory {
    if n == 1 {
        PluralCategory::One
    } else if n >= 2 && n <= 4 {
        PluralCategory::Few
    } else {
        PluralCategory::Other
    }
}

pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(&key)
}

pub fn translate_plural(key: &str, count: usize) -> &'static str {
    if let Some(forms) = PLURAL_FORMS.get(key) {
        let category = get_plural_category(count);
        match category {
            PluralCategory::One => forms.get(0),
            PluralCategory::Few => forms.get(1),
            _ => forms.get(2),
        }.unwrap_or(&key)
    } else {
        key
    }
}