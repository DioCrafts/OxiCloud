use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("nn_NO");

pub fn initialize_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert(
        "Could not move %s - File with this name already exists".to_string(),
        "Klarte ikkje flytta %s – det finst allereie ei fil med dette namnet".to_string(),
    );
    translations.insert(
        "Could not move %s".to_string(),
        "Klarte ikkje flytta %s".to_string(),
    );
    translations.insert(
        "File name cannot be empty.".to_string(),
        "Filnamnet kan ikkje vera tomt.".to_string(),
    );
    translations.insert(
        "Unable to set upload directory.".to_string(),
        "Klarte ikkje å endra opplastingsmappa.".to_string(),
    );
    translations.insert(
        "Invalid Token".to_string(),
        "Ugyldig token".to_string(),
    );
    translations.insert(
        "No file was uploaded. Unknown error".to_string(),
        "Ingen filer lasta opp. Ukjend feil".to_string(),
    );
    translations.insert(
        "There is no error, the file uploaded with success".to_string(),
        "Ingen feil, fila vart lasta opp".to_string(),
    );
    translations.insert(
        "The uploaded file exceeds the upload_max_filesize directive in php.ini: ".to_string(),
        "Fila du lasta opp er større enn det «upload_max_filesize» i php.ini tilleter: ".to_string(),
    );
    translations.insert(
        "The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form".to_string(),
        "Den opplasta fila er større enn variabelen MAX_FILE_SIZE i HTML-skjemaet".to_string(),
    );
    translations.insert(
        "The uploaded file was only partially uploaded".to_string(),
        "Fila vart berre delvis lasta opp".to_string(),
    );
    translations.insert(
        "No file was uploaded".to_string(),
        "Ingen filer vart lasta opp".to_string(),
    );
    translations.insert(
        "Missing a temporary folder".to_string(),
        "Manglar ei mellombels mappe".to_string(),
    );
    translations.insert(
        "Failed to write to disk".to_string(),
        "Klarte ikkje skriva til disk".to_string(),
    );
    translations.insert(
        "Not enough storage available".to_string(),
        "Ikkje nok lagringsplass tilgjengeleg".to_string(),
    );
    translations.insert(
        "Upload failed. Could not get file info.".to_string(),
        "Feil ved opplasting. Klarte ikkje å henta filinfo.".to_string(),
    );
    translations.insert(
        "Upload failed. Could not find uploaded file".to_string(),
        "Feil ved opplasting. Klarte ikkje å finna opplasta fil.".to_string(),
    );
    translations.insert(
        "Invalid directory.".to_string(),
        "Ugyldig mappe.".to_string(),
    );
    translations.insert(
        "Files".to_string(),
        "Filer".to_string(),
    );
    translations.insert(
        "Unable to upload {filename} as it is a directory or has 0 bytes".to_string(),
        "Klarte ikkje å lasta opp {filename} sidan det er ei mappe eller er 0 byte.".to_string(),
    );
    translations.insert(
        "Not enough space available".to_string(),
        "Ikkje nok lagringsplass tilgjengeleg".to_string(),
    );
    translations.insert(
        "Upload cancelled.".to_string(),
        "Opplasting avbroten.".to_string(),
    );
    translations.insert(
        "Could not get result from server.".to_string(),
        "Klarte ikkje å henta resultat frå tenaren.".to_string(),
    );
    translations.insert(
        "File upload is in progress. Leaving the page now will cancel the upload.".to_string(),
        "Fila lastar no opp. Viss du forlèt sida no vil opplastinga verta avbroten.".to_string(),
    );
    translations.insert(
        "{new_name} already exists".to_string(),
        "{new_name} finst allereie".to_string(),
    );
    translations.insert(
        "Share".to_string(),
        "Del".to_string(),
    );
    translations.insert(
        "Delete permanently".to_string(),
        "Slett for godt".to_string(),
    );
    translations.insert(
        "Rename".to_string(),
        "Endra namn".to_string(),
    );
    translations.insert(
        "Pending".to_string(),
        "Under vegs".to_string(),
    );
    translations.insert(
        "replaced {new_name} with {old_name}".to_string(),
        "bytte ut {new_name} med {old_name}".to_string(),
    );
    translations.insert(
        "undo".to_string(),
        "angre".to_string(),
    );
    translations.insert(
        "{dirs} and {files}".to_string(),
        "{dirs} og {files}".to_string(),
    );
    translations.insert(
        "'.' is an invalid file name.".to_string(),
        "«.» er eit ugyldig filnamn.".to_string(),
    );
    translations.insert(
        "Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.".to_string(),
        "Ugyldig namn, «\\», «/», «<», «>», «:», «\"», «|», «?» og «*» er ikkje tillate.".to_string(),
    );
    translations.insert(
        "Your storage is full, files can not be updated or synced anymore!".to_string(),
        "Lagringa di er full, kan ikkje lenger oppdatera eller synkronisera!".to_string(),
    );
    translations.insert(
        "Your storage is almost full ({usedSpacePercent}%)".to_string(),
        "Lagringa di er nesten full ({usedSpacePercent} %)".to_string(),
    );
    translations.insert(
        "Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.".to_string(),
        "Kryptering er skrudd av, men filene dine er enno krypterte. Du kan dekryptera filene i personlege innstillingar.".to_string(),
    );
    translations.insert(
        "Your download is being prepared. This might take some time if the files are big.".to_string(),
        "Gjer klar nedlastinga di. Dette kan ta ei stund viss filene er store.".to_string(),
    );
    translations.insert(
        "Error moving file".to_string(),
        "Feil ved flytting av fil".to_string(),
    );
    translations.insert(
        "Error".to_string(),
        "Feil".to_string(),
    );
    translations.insert(
        "Name".to_string(),
        "Namn".to_string(),
    );
    translations.insert(
        "Size".to_string(),
        "Storleik".to_string(),
    );
    translations.insert(
        "Modified".to_string(),
        "Endra".to_string(),
    );
    translations.insert(
        "%s could not be renamed".to_string(),
        "Klarte ikkje å omdøypa på %s".to_string(),
    );
    translations.insert(
        "Upload".to_string(),
        "Last opp".to_string(),
    );
    translations.insert(
        "File handling".to_string(),
        "Filhandtering".to_string(),
    );
    translations.insert(
        "Maximum upload size".to_string(),
        "Maksimal opplastingsstorleik".to_string(),
    );
    translations.insert(
        "max. possible: ".to_string(),
        "maks. moglege:".to_string(),
    );
    translations.insert(
        "Needed for multi-file and folder downloads.".to_string(),
        "Nødvendig for fleirfils- og mappenedlastingar.".to_string(),
    );
    translations.insert(
        "Enable ZIP-download".to_string(),
        "Slå på ZIP-nedlasting".to_string(),
    );
    translations.insert(
        "0 is unlimited".to_string(),
        "0 er ubegrensa".to_string(),
    );
    translations.insert(
        "Maximum input size for ZIP files".to_string(),
        "Maksimal storleik for ZIP-filer".to_string(),
    );
    translations.insert(
        "Save".to_string(),
        "Lagre".to_string(),
    );
    translations.insert(
        "New".to_string(),
        "Ny".to_string(),
    );
    translations.insert(
        "Text file".to_string(),
        "Tekst fil".to_string(),
    );
    translations.insert(
        "Folder".to_string(),
        "Mappe".to_string(),
    );
    translations.insert(
        "From link".to_string(),
        "Frå lenkje".to_string(),
    );
    translations.insert(
        "Deleted files".to_string(),
        "Sletta filer".to_string(),
    );
    translations.insert(
        "Cancel upload".to_string(),
        "Avbryt opplasting".to_string(),
    );
    translations.insert(
        "Nothing in here. Upload something!".to_string(),
        "Ingenting her. Last noko opp!".to_string(),
    );
    translations.insert(
        "Download".to_string(),
        "Last ned".to_string(),
    );
    translations.insert(
        "Unshare".to_string(),
        "Udel".to_string(),
    );
    translations.insert(
        "Delete".to_string(),
        "Slett".to_string(),
    );
    translations.insert(
        "Upload too large".to_string(),
        "For stor opplasting".to_string(),
    );
    translations.insert(
        "The files you are trying to upload exceed the maximum size for file uploads on this server.".to_string(),
        "Filene du prøver å lasta opp er større enn maksgrensa til denne tenaren.".to_string(),
    );
    translations.insert(
        "Files are being scanned, please wait.".to_string(),
        "Skannar filer, ver venleg og vent.".to_string(),
    );
    translations.insert(
        "Current scanning".to_string(),
        "Køyrande skanning".to_string(),
    );
    translations.insert(
        "Upgrading filesystem cache...".to_string(),
        "Oppgraderer mellomlageret av filsystemet …".to_string(),
    );
    
    translations
}

pub fn initialize_plurals() -> HashMap<String, Vec<String>> {
    let mut plurals = HashMap::new();
    
    plurals.insert(
        "_%n folder_::_%n folders_".to_string(), 
        vec!["%n mappe".to_string(), "%n mapper".to_string()]
    );
    
    plurals.insert(
        "_%n file_::_%n files_".to_string(), 
        vec!["%n fil".to_string(), "%n filer".to_string()]
    );
    
    plurals.insert(
        "_Uploading %n file_::_Uploading %n files_".to_string(), 
        vec!["Lastar opp %n fil".to_string(), "Lastar opp %n filer".to_string()]
    );
    
    plurals
}

/// Returns the plural form for the given count
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}