use once_cell::sync::Lazy;
use std::collections::HashMap;
use rust_i18n::plural::PluralRules;

// Translation definitions for Romanian (ro)
pub static RO_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    
    translations.insert("Could not move %s - File with this name already exists", "%s nu se poate muta - Fișierul cu acest nume există deja ");
    translations.insert("Could not move %s", "Nu s-a putut muta %s");
    translations.insert("File name cannot be empty.", "Numele fișierului nu poate rămâne gol.");
    translations.insert("Unable to set upload directory.", "Imposibil de a seta directorul pentru incărcare.");
    translations.insert("Invalid Token", "Jeton Invalid");
    translations.insert("No file was uploaded. Unknown error", "Nici un fișier nu a fost încărcat. Eroare necunoscută");
    translations.insert("There is no error, the file uploaded with success", "Nu a apărut nici o eroare, fișierul a fost încărcat cu succes");
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Fisierul incarcat depaseste marimea maxima permisa in php.ini: ");
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Fișierul are o dimensiune mai mare decât variabile MAX_FILE_SIZE specificată în formularul HTML");
    translations.insert("The uploaded file was only partially uploaded", "Fișierul a fost încărcat doar parțial");
    translations.insert("No file was uploaded", "Nu a fost încărcat nici un fișier");
    translations.insert("Missing a temporary folder", "Lipsește un dosar temporar");
    translations.insert("Failed to write to disk", "Eroare la scrierea discului");
    translations.insert("Not enough storage available", "Nu este suficient spațiu disponibil");
    translations.insert("Upload failed. Could not get file info.", "Încărcare eșuată. Nu se pot obține informații despre fișier.");
    translations.insert("Upload failed. Could not find uploaded file", "Încărcare eșuată. Nu se poate găsi fișierul încărcat");
    translations.insert("Invalid directory.", "registru invalid.");
    translations.insert("Files", "Fișiere");
    translations.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Nu se poate încărca {filename} deoarece este un director sau are mărimea de 0 octeți");
    translations.insert("Not enough space available", "Nu este suficient spațiu disponibil");
    translations.insert("Upload cancelled.", "Încărcare anulată.");
    translations.insert("Could not get result from server.", "Nu se poate obține rezultatul de la server.");
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Fișierul este în curs de încărcare. Părăsirea paginii va întrerupe încărcarea.");
    translations.insert("{new_name} already exists", "{new_name} deja exista");
    translations.insert("Share", "a imparti");
    translations.insert("Delete permanently", "Stergere permanenta");
    translations.insert("Rename", "Redenumire");
    translations.insert("Pending", "in timpul");
    translations.insert("replaced {new_name} with {old_name}", "{new_name} inlocuit cu {old_name}");
    translations.insert("undo", "Anulează ultima acțiune");
    translations.insert("_%n folder_::_%n folders_", "_%n director_::_%n directoare_::_%n directoare_");
    translations.insert("_%n file_::_%n files_", "_%n fișier_::_%n fișiere_::_%n fișiere_");
    translations.insert("{dirs} and {files}", "{dirs} și {files}");
    translations.insert("_Uploading %n file_::_Uploading %n files_", "_Se încarcă %n fișier._::_Se încarcă %n fișiere._::_Se încarcă %n fișiere._");
    translations.insert("'.' is an invalid file name.", "'.' este un nume invalid de fișier.");
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Nume invalide, '\\', '/', '<', '>', ':', '\"', '|', '?' si '*' nu sunt permise.");
    translations.insert("Your storage is full, files can not be updated or synced anymore!", "Spatiul de stocare este plin, fisierele nu mai pot fi actualizate sau sincronizate");
    translations.insert("Your storage is almost full ({usedSpacePercent}%)", "Spatiul de stocare este aproape plin {spatiu folosit}%");
    translations.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "criptarea a fost disactivata dar fisierele sant inca criptate.va rog intrati in setarile personale pentru a decripta fisierele");
    translations.insert("Your download is being prepared. This might take some time if the files are big.", "in curs de descarcare. Aceasta poate să dureze ceva timp dacă fișierele sunt mari.");
    translations.insert("Error moving file", "Eroare la mutarea fișierului");
    translations.insert("Error", "Eroare");
    translations.insert("Name", "Nume");
    translations.insert("Size", "Dimensiune");
    translations.insert("Modified", "Modificat");
    translations.insert("%s could not be renamed", "%s nu a putut fi redenumit");
    translations.insert("Upload", "Încărcare");
    translations.insert("File handling", "Manipulare fișiere");
    translations.insert("Maximum upload size", "Dimensiune maximă admisă la încărcare");
    translations.insert("max. possible: ", "max. posibil:");
    translations.insert("Needed for multi-file and folder downloads.", "necesar la descarcarea mai multor liste si fisiere");
    translations.insert("Enable ZIP-download", "permite descarcarea codurilor ZIP");
    translations.insert("0 is unlimited", "0 e nelimitat");
    translations.insert("Maximum input size for ZIP files", "Dimensiunea maximă de intrare pentru fișiere compresate");
    translations.insert("Save", "Salvează");
    translations.insert("New", "Nou");
    translations.insert("Text file", "lista");
    translations.insert("Folder", "Dosar");
    translations.insert("From link", "de la adresa");
    translations.insert("Deleted files", "Sterge fisierele");
    translations.insert("Cancel upload", "Anulează încărcarea");
    translations.insert("Nothing in here. Upload something!", "Nimic aici. Încarcă ceva!");
    translations.insert("Download", "Descarcă");
    translations.insert("Unshare", "Anulare");
    translations.insert("Delete", "Șterge");
    translations.insert("Upload too large", "Fișierul încărcat este prea mare");
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Fișierul care l-ai încărcat a depășită limita maximă admisă la încărcare pe acest server.");
    translations.insert("Files are being scanned, please wait.", "Fișierele sunt scanate, asteptati va rog");
    translations.insert("Current scanning", "În curs de scanare");
    translations.insert("Upgrading filesystem cache...", "Modernizare fisiere de sistem cache..");
    
    translations
});

// Romanian plural rules
pub static RO_PLURAL_RULES: Lazy<PluralRules> = Lazy::new(|| {
    PluralRules::new(
        3, // nplurals=3
        Box::new(|n| {
            if n == 1 {
                0
            } else if (n % 100 > 19) || ((n % 100 == 0) && (n != 0)) {
                2
            } else {
                1
            }
        })
    )
});