use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("hu_HU");

pub fn translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Could not move %s - File with this name already exists", "%s áthelyezése nem sikerült - már létezik másik fájl ezzel a névvel");
    translations.insert("Could not move %s", "Nem sikerült %s áthelyezése");
    translations.insert("File name cannot be empty.", "A fájlnév nem lehet semmi.");
    translations.insert("File name must not contain \"/\". Please choose a different name.", "Az állomány neve nem tartalmazhatja a \"/\" karaktert. Kérem válasszon másik nevet!");
    translations.insert("The name %s is already used in the folder %s. Please choose a different name.", "A %s név már létezik a %s mappában. Kérem válasszon másik nevet!");
    translations.insert("Not a valid source", "A kiinduló állomány érvénytelen");
    translations.insert("Error while downloading %s to %s", "Hiba történt miközben %s-t letöltöttük %s-be");
    translations.insert("Error when creating the file", "Hiba történt az állomány létrehozásakor");
    translations.insert("Folder name cannot be empty.", "A mappa neve nem maradhat kitöltetlenül");
    translations.insert("Folder name must not contain \"/\". Please choose a different name.", "A mappa neve nem tartalmazhatja a \"/\" karaktert. Kérem válasszon másik nevet!");
    translations.insert("Error when creating the folder", "Hiba történt a mappa létrehozásakor");
    translations.insert("Unable to set upload directory.", "Nem található a mappa, ahova feltölteni szeretne.");
    translations.insert("Invalid Token", "Hibás mappacím");
    translations.insert("No file was uploaded. Unknown error", "Nem történt feltöltés. Ismeretlen hiba");
    translations.insert("There is no error, the file uploaded with success", "A fájlt sikerült feltölteni");
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "A feltöltött fájl mérete meghaladja a php.ini állományban megadott upload_max_filesize paraméter értékét.");
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "A feltöltött fájl mérete meghaladja a MAX_FILE_SIZE paramétert, ami a HTML  formban került megadásra.");
    translations.insert("The uploaded file was only partially uploaded", "Az eredeti fájlt csak részben sikerült feltölteni.");
    translations.insert("No file was uploaded", "Nem töltődött fel állomány");
    translations.insert("Missing a temporary folder", "Hiányzik egy ideiglenes mappa");
    translations.insert("Failed to write to disk", "Nem sikerült a lemezre történő írás");
    translations.insert("Not enough storage available", "Nincs elég szabad hely.");
    translations.insert("Upload failed. Could not get file info.", "A feltöltés nem sikerült. Az állományt leíró információk nem érhetők el.");
    translations.insert("Upload failed. Could not find uploaded file", "A feltöltés nem sikerült. Nem található a feltöltendő állomány.");
    translations.insert("Invalid directory.", "Érvénytelen mappa.");
    translations.insert("Files", "Fájlok");
    translations.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "A(z) {filename} állomány nem tölthető fel, mert ez vagy egy mappa, vagy pedig 0 bájtból áll.");
    translations.insert("Not enough space available", "Nincs elég szabad hely");
    translations.insert("Upload cancelled.", "A feltöltést megszakítottuk.");
    translations.insert("Could not get result from server.", "A kiszolgálótól nem kapható meg az eredmény.");
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Fájlfeltöltés van folyamatban. Az oldal elhagyása megszakítja a feltöltést.");
    translations.insert("URL cannot be empty", "Az URL-cím nem maradhat kitöltetlenül");
    translations.insert("In the home folder 'Shared' is a reserved filename", "A kiindulási mappában a 'Shared' egy belső használatra fenntartott név");
    translations.insert("{new_name} already exists", "{new_name} már létezik");
    translations.insert("Could not create file", "Az állomány nem hozható létre");
    translations.insert("Could not create folder", "A mappa nem hozható létre");
    translations.insert("Share", "Megosztás");
    translations.insert("Delete permanently", "Végleges törlés");
    translations.insert("Rename", "Átnevezés");
    translations.insert("Pending", "Folyamatban");
    translations.insert("Could not rename file", "Az állomány nem nevezhető át");
    translations.insert("replaced {new_name} with {old_name}", "{new_name} fájlt kicseréltük ezzel:  {old_name}");
    translations.insert("undo", "visszavonás");
    translations.insert("_%n folder_::_%n folders_", "%n mappa");
    translations.insert("_%n file_::_%n files_", "%n állomány");
    translations.insert("{dirs} and {files}", "{dirs} és {files}");
    translations.insert("_Uploading %n file_::_Uploading %n files_", "%n állomány feltöltése");
    translations.insert("'.' is an invalid file name.", "'.' fájlnév érvénytelen.");
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Érvénytelen elnevezés. Ezek a karakterek nem használhatók: '\\', '/', '<', '>', ':', '\"', '|', '?' és '*'");
    translations.insert("Your storage is full, files can not be updated or synced anymore!", "A tároló tele van, a fájlok nem frissíthetőek vagy szinkronizálhatóak a jövőben.");
    translations.insert("Your storage is almost full ({usedSpacePercent}%)", "A tároló majdnem tele van ({usedSpacePercent}%)");
    translations.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "Az állományok titkosítása engedélyezve van, de az Ön titkos kulcsai nincsenek beállítva. Ezért kérjük, hogy jelentkezzen ki, és lépjen be újra!");
    translations.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "Az állományok titkosításához használt titkos kulcsa érvénytelen. Kérjük frissítse a titkos kulcs jelszót a személyes beállításokban, hogy ismét hozzáférjen a titkosított állományaihoz!");
    translations.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "A titkosítási funkciót kikapcsolták, de az Ön állományai még mindig titkosított állapotban vannak. A személyes beállításoknál tudja a titkosítást feloldani.");
    translations.insert("Your download is being prepared. This might take some time if the files are big.", "Készül a letöltendő állomány. Ez eltarthat egy ideig, ha nagyok a fájlok.");
    translations.insert("Error moving file", "Az állomány áthelyezése nem sikerült.");
    translations.insert("Error", "Hiba");
    translations.insert("Name", "Név");
    translations.insert("Size", "Méret");
    translations.insert("Modified", "Módosítva");
    translations.insert("%s could not be renamed", "%s átnevezése nem sikerült");
    translations.insert("Upload", "Feltöltés");
    translations.insert("File handling", "Fájlkezelés");
    translations.insert("Maximum upload size", "Maximális feltölthető fájlméret");
    translations.insert("max. possible: ", "max. lehetséges: ");
    translations.insert("Needed for multi-file and folder downloads.", "Kötegelt fájl- vagy mappaletöltéshez szükséges");
    translations.insert("Enable ZIP-download", "A ZIP-letöltés engedélyezése");
    translations.insert("0 is unlimited", "0 = korlátlan");
    translations.insert("Maximum input size for ZIP files", "ZIP-fájlok maximális kiindulási mérete");
    translations.insert("Save", "Mentés");
    translations.insert("New", "Új");
    translations.insert("Text file", "Szövegfájl");
    translations.insert("Folder", "Mappa");
    translations.insert("From link", "Feltöltés linkről");
    translations.insert("Deleted files", "Törölt fájlok");
    translations.insert("Cancel upload", "A feltöltés megszakítása");
    translations.insert("You don't have permission to upload or create files here", "Önnek nincs jogosultsága ahhoz, hogy ide állományokat töltsön föl, vagy itt újakat hozzon létre");
    translations.insert("Nothing in here. Upload something!", "Itt nincs semmi. Töltsön fel valamit!");
    translations.insert("Download", "Letöltés");
    translations.insert("Unshare", "A megosztás visszavonása");
    translations.insert("Delete", "Törlés");
    translations.insert("Upload too large", "A feltöltés túl nagy");
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "A feltöltendő állományok mérete meghaladja a kiszolgálón megengedett maximális méretet.");
    translations.insert("Files are being scanned, please wait.", "A fájllista ellenőrzése zajlik, kis türelmet!");
    translations.insert("Current scanning", "Ellenőrzés alatt");
    translations.insert("Upgrading filesystem cache...", "A fájlrendszer gyorsítótárának frissítése zajlik...");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_plural_translation(key: &str, count: i64) -> String {
    match key {
        "_%n folder_::_%n folders_" => {
            format!("{} mappa", count)
        },
        "_%n file_::_%n files_" => {
            format!("{} állomány", count)
        },
        "_Uploading %n file_::_Uploading %n files_" => {
            format!("{} állomány feltöltése", count)
        },
        _ => String::new(),
    }
}