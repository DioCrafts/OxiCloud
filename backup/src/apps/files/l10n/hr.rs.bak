use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("There is no error, the file uploaded with success", "Nema pogreške, datoteka je poslana uspješno.");
    m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Poslana datoteka prelazi veličinu prikazanu u MAX_FILE_SIZE direktivi u HTML formi");
    m.insert("The uploaded file was only partially uploaded", "Poslana datoteka je parcijalno poslana");
    m.insert("No file was uploaded", "Datoteka nije poslana");
    m.insert("Missing a temporary folder", "Nedostaje privremeni direktorij");
    m.insert("Failed to write to disk", "Neuspjelo pisanje na disk");
    m.insert("Files", "Datoteke");
    m.insert("Upload cancelled.", "Slanje poništeno.");
    m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Učitavanje datoteke. Napuštanjem stranice će prekinuti učitavanje.");
    m.insert("Share", "Podijeli");
    m.insert("Rename", "Promjeni ime");
    m.insert("Pending", "U tijeku");
    m.insert("undo", "vrati");
    m.insert("_%n folder_::_%n folders_", "");
    m.insert("_%n file_::_%n files_", "");
    m.insert("_Uploading %n file_::_Uploading %n files_", "");
    m.insert("Error", "Greška");
    m.insert("Name", "Ime");
    m.insert("Size", "Veličina");
    m.insert("Modified", "Zadnja promjena");
    m.insert("Upload", "Učitaj");
    m.insert("File handling", "datoteka za rukovanje");
    m.insert("Maximum upload size", "Maksimalna veličina prijenosa");
    m.insert("max. possible: ", "maksimalna moguća: ");
    m.insert("Needed for multi-file and folder downloads.", "Potrebno za preuzimanje više datoteke i mape");
    m.insert("Enable ZIP-download", "Omogući ZIP-preuzimanje");
    m.insert("0 is unlimited", "0 je \"bez limita\"");
    m.insert("Maximum input size for ZIP files", "Maksimalna veličina za ZIP datoteke");
    m.insert("Save", "Snimi");
    m.insert("New", "novo");
    m.insert("Text file", "tekstualna datoteka");
    m.insert("Folder", "mapa");
    m.insert("Cancel upload", "Prekini upload");
    m.insert("Nothing in here. Upload something!", "Nema ničega u ovoj mapi. Pošalji nešto!");
    m.insert("Download", "Preuzimanje");
    m.insert("Unshare", "Makni djeljenje");
    m.insert("Delete", "Obriši");
    m.insert("Upload too large", "Prijenos je preobiman");
    m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Datoteke koje pokušavate prenijeti prelaze maksimalnu veličinu za prijenos datoteka na ovom poslužitelju.");
    m.insert("Files are being scanned, please wait.", "Datoteke se skeniraju, molimo pričekajte.");
    m.insert("Current scanning", "Trenutno skeniranje");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2;";

pub fn get_plural_index(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}

pub struct PluralForms {
    pub forms: Vec<&'static str>,
}

pub static PLURAL_TRANSLATIONS: Lazy<HashMap<&'static str, PluralForms>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("_%n folder_::_%n folders_", PluralForms { forms: vec!["", "", ""] });
    m.insert("_%n file_::_%n files_", PluralForms { forms: vec!["", "", ""] });
    m.insert("_Uploading %n file_::_Uploading %n files_", PluralForms { forms: vec!["", "", ""] });
    m
});