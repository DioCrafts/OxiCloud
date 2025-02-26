use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "%s ni mogoče premakniti  - datoteka s tem imenom že obstaja");
        m.insert("Could not move %s", "Ni mogoče premakniti %s");
        m.insert("File name cannot be empty.", "Ime datoteke ne sme biti prazno polje.");
        m.insert("Unable to set upload directory.", "Mapo, v katero boste prenašali dokumente, ni mogoče določiti");
        m.insert("Invalid Token", "Neveljaven žeton");
        m.insert("No file was uploaded. Unknown error", "Ni poslane datoteke. Neznana napaka.");
        m.insert("There is no error, the file uploaded with success", "Datoteka je uspešno naložena.");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Poslana datoteka presega dovoljeno velikost, ki je določena z možnostjo upload_max_filesize v datoteki php.ini:");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Poslana datoteka presega velikost, ki jo določa parameter največje dovoljene velikosti v obrazcu HTML.");
        m.insert("The uploaded file was only partially uploaded", "Poslan je le del datoteke.");
        m.insert("No file was uploaded", "Ni poslane datoteke");
        m.insert("Missing a temporary folder", "Manjka začasna mapa");
        m.insert("Failed to write to disk", "Pisanje na disk je spodletelo");
        m.insert("Not enough storage available", "Na voljo ni dovolj prostora");
        m.insert("Invalid directory.", "Neveljavna mapa.");
        m.insert("Files", "Datoteke");
        m.insert("Not enough space available", "Na voljo ni dovolj prostora.");
        m.insert("Upload cancelled.", "Pošiljanje je preklicano.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "V teku je pošiljanje datoteke. Če zapustite to stran zdaj, bo pošiljanje preklicano.");
        m.insert("{new_name} already exists", "{new_name} že obstaja");
        m.insert("Share", "Souporaba");
        m.insert("Delete permanently", "Izbriši dokončno");
        m.insert("Rename", "Preimenuj");
        m.insert("Pending", "V čakanju ...");
        m.insert("replaced {new_name} with {old_name}", "preimenovano ime {new_name} z imenom {old_name}");
        m.insert("undo", "razveljavi");
        m.insert("_%n folder_::_%n folders_", "");
        m.insert("_%n file_::_%n files_", "");
        m.insert("_Uploading %n file_::_Uploading %n files_", "");
        m.insert("'.' is an invalid file name.", "'.' je neveljavno ime datoteke.");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Neveljavno ime, znaki '\\', '/', '<', '>', ':', '\"', '|', '?' in '*' niso dovoljeni.");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "Shramba je povsem napolnjena. Datotek ni več mogoče posodabljati in usklajevati!");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "Mesto za shranjevanje je skoraj polno ({usedSpacePercent}%)");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "Postopek priprave datoteke za prejem je lahko dolgotrajen, če je datoteka zelo velika.");
        m.insert("Error", "Napaka");
        m.insert("Name", "Ime");
        m.insert("Size", "Velikost");
        m.insert("Modified", "Spremenjeno");
        m.insert("%s could not be renamed", "%s ni bilo mogoče preimenovati");
        m.insert("Upload", "Pošlji");
        m.insert("File handling", "Upravljanje z datotekami");
        m.insert("Maximum upload size", "Največja velikost za pošiljanja");
        m.insert("max. possible: ", "največ mogoče:");
        m.insert("Needed for multi-file and folder downloads.", "Uporabljeno za prejem več datotek in map.");
        m.insert("Enable ZIP-download", "Omogoči prejemanje arhivov ZIP");
        m.insert("0 is unlimited", "0 predstavlja neomejeno vrednost");
        m.insert("Maximum input size for ZIP files", "Največja vhodna velikost za datoteke ZIP");
        m.insert("Save", "Shrani");
        m.insert("New", "Novo");
        m.insert("Text file", "Besedilna datoteka");
        m.insert("Folder", "Mapa");
        m.insert("From link", "Iz povezave");
        m.insert("Deleted files", "Izbrisane datoteke");
        m.insert("Cancel upload", "Prekliči pošiljanje");
        m.insert("Nothing in here. Upload something!", "Tukaj še ni ničesar. Najprej je treba kakšno datoteko poslati v oblak!");
        m.insert("Download", "Prejmi");
        m.insert("Unshare", "Prekliči souporabo");
        m.insert("Delete", "Izbriši");
        m.insert("Upload too large", "Prekoračenje omejitve velikosti");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Datoteke, ki jih želite poslati, presegajo največjo dovoljeno velikost na strežniku.");
        m.insert("Files are being scanned, please wait.", "Poteka preučevanje datotek, počakajte ...");
        m.insert("Current scanning", "Trenutno poteka preučevanje");
        m.insert("Upgrading filesystem cache...", "Nadgrajevanje predpomnilnika datotečnega sistema ...");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=4; plural=(n%100==1 ? 0 : n%100==2 ? 1 : n%100==3 || n%100==4 ? 2 : 3);";

    pub static ref PLURAL_MAPPINGS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n folder_::_%n folders_", vec!["", "", "", ""]);
        m.insert("_%n file_::_%n files_", vec!["", "", "", ""]);
        m.insert("_Uploading %n file_::_Uploading %n files_", vec!["", "", "", ""]);
        m
    };
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form(n: usize) -> usize {
    let n = n as u64;
    if n % 100 == 1 {
        0
    } else if n % 100 == 2 {
        1
    } else if n % 100 == 3 || n % 100 == 4 {
        2
    } else {
        3
    }
}

pub fn get_plural_translation(key: &str, n: usize) -> Option<&'static str> {
    let form = get_plural_form(n);
    PLURAL_MAPPINGS.get(key).and_then(|forms| forms.get(form).copied())
}