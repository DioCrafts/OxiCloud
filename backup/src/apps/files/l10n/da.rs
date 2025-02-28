use std::collections::HashMap;
use once_cell::sync::Lazy;

// Static translations map with custom pluralization support
pub static TRANSLATIONS: Lazy<HashMap<&'static str, Translation>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    
    // Simple translations
    translations.insert("Could not move %s - File with this name already exists", Translation::Singular("Kunne ikke flytte %s - der findes allerede en fil med dette navn"));
    translations.insert("Could not move %s", Translation::Singular("Kunne ikke flytte %s"));
    translations.insert("File name cannot be empty.", Translation::Singular("Filnavnet kan ikke stå tomt."));
    translations.insert("Unable to set upload directory.", Translation::Singular("Ude af stand til at vælge upload mappe."));
    translations.insert("Invalid Token", Translation::Singular("Ugyldig Token "));
    translations.insert("No file was uploaded. Unknown error", Translation::Singular("Ingen fil blev uploadet. Ukendt fejl."));
    translations.insert("There is no error, the file uploaded with success", Translation::Singular("Der skete ingen fejl, filen blev succesfuldt uploadet"));
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", Translation::Singular("Den uploadede fil overstiger upload_max_filesize direktivet i php.ini"));
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", Translation::Singular("Den uploadede fil overstiger MAX_FILE_SIZE indstilingen, som specificeret i HTML formularen"));
    translations.insert("The uploaded file was only partially uploaded", Translation::Singular("Filen blev kun delvist uploadet."));
    translations.insert("No file was uploaded", Translation::Singular("Ingen fil uploadet"));
    translations.insert("Missing a temporary folder", Translation::Singular("Manglende midlertidig mappe."));
    translations.insert("Failed to write to disk", Translation::Singular("Fejl ved skrivning til disk."));
    translations.insert("Not enough storage available", Translation::Singular("Der er ikke nok plads til rådlighed"));
    translations.insert("Upload failed. Could not get file info.", Translation::Singular("Upload fejlede. Kunne ikke hente filinformation."));
    translations.insert("Upload failed. Could not find uploaded file", Translation::Singular("Upload fejlede. Kunne ikke finde den uploadede fil."));
    translations.insert("Invalid directory.", Translation::Singular("Ugyldig mappe."));
    translations.insert("Files", Translation::Singular("Filer"));
    translations.insert("Unable to upload {filename} as it is a directory or has 0 bytes", Translation::Singular("Kan ikke upload {filename} da det er enten en mappe eller indholder 0 bytes."));
    translations.insert("Not enough space available", Translation::Singular("ikke nok tilgængelig ledig plads "));
    translations.insert("Upload cancelled.", Translation::Singular("Upload afbrudt."));
    translations.insert("Could not get result from server.", Translation::Singular("Kunne ikke hente resultat fra server."));
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.", Translation::Singular("Fil upload kører. Hvis du forlader siden nu, vil uploadet blive annuleret."));
    translations.insert("{new_name} already exists", Translation::Singular("{new_name} eksisterer allerede"));
    translations.insert("Share", Translation::Singular("Del"));
    translations.insert("Delete permanently", Translation::Singular("Slet permanent"));
    translations.insert("Rename", Translation::Singular("Omdøb"));
    translations.insert("Pending", Translation::Singular("Afventer"));
    translations.insert("replaced {new_name} with {old_name}", Translation::Singular("erstattede {new_name} med {old_name}"));
    translations.insert("undo", Translation::Singular("fortryd"));
    translations.insert("_{dirs} and {files}", Translation::Singular("{dirs} og {files}"));
    translations.insert("'.' is an invalid file name.", Translation::Singular("'.' er et ugyldigt filnavn."));
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", Translation::Singular("Ugyldigt navn, '\\', '/', '<', '>', ':' | '?', '\"', '', og '*' er ikke tilladt."));
    translations.insert("Your storage is full, files can not be updated or synced anymore!", Translation::Singular("Din opbevaringsplads er fyldt op, filer kan ikke opdateres eller synkroniseres længere!"));
    translations.insert("Your storage is almost full ({usedSpacePercent}%)", Translation::Singular("Din opbevaringsplads er næsten fyldt op ({usedSpacePercent}%)"));
    translations.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", Translation::Singular("Krypteringen blev deaktiveret, men dine filer er stadig krypteret. Gå venligst til dine personlige indstillinger for at dekryptere dine filer. "));
    translations.insert("Your download is being prepared. This might take some time if the files are big.", Translation::Singular("Dit download forberedes. Dette kan tage lidt tid ved større filer."));
    translations.insert("Error moving file", Translation::Singular("Fejl ved flytning af fil"));
    translations.insert("Error", Translation::Singular("Fejl"));
    translations.insert("Name", Translation::Singular("Navn"));
    translations.insert("Size", Translation::Singular("Størrelse"));
    translations.insert("Modified", Translation::Singular("Ændret"));
    translations.insert("%s could not be renamed", Translation::Singular("%s kunne ikke omdøbes"));
    translations.insert("Upload", Translation::Singular("Upload"));
    translations.insert("File handling", Translation::Singular("Filhåndtering"));
    translations.insert("Maximum upload size", Translation::Singular("Maksimal upload-størrelse"));
    translations.insert("max. possible: ", Translation::Singular("max. mulige: "));
    translations.insert("Needed for multi-file and folder downloads.", Translation::Singular("Nødvendigt for at kunne downloade mapper og flere filer ad gangen."));
    translations.insert("Enable ZIP-download", Translation::Singular("Tillad ZIP-download"));
    translations.insert("0 is unlimited", Translation::Singular("0 er ubegrænset"));
    translations.insert("Maximum input size for ZIP files", Translation::Singular("Maksimal størrelse på ZIP filer"));
    translations.insert("Save", Translation::Singular("Gem"));
    translations.insert("New", Translation::Singular("Ny"));
    translations.insert("Text file", Translation::Singular("Tekstfil"));
    translations.insert("Folder", Translation::Singular("Mappe"));
    translations.insert("From link", Translation::Singular("Fra link"));
    translations.insert("Deleted files", Translation::Singular("Slettede filer"));
    translations.insert("Cancel upload", Translation::Singular("Fortryd upload"));
    translations.insert("Nothing in here. Upload something!", Translation::Singular("Her er tomt. Upload noget!"));
    translations.insert("Download", Translation::Singular("Download"));
    translations.insert("Unshare", Translation::Singular("Fjern deling"));
    translations.insert("Delete", Translation::Singular("Slet"));
    translations.insert("Upload too large", Translation::Singular("Upload er for stor"));
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", Translation::Singular("Filerne, du prøver at uploade, er større end den maksimale størrelse for fil-upload på denne server."));
    translations.insert("Files are being scanned, please wait.", Translation::Singular("Filerne bliver indlæst, vent venligst."));
    translations.insert("Current scanning", Translation::Singular("Indlæser"));
    translations.insert("Upgrading filesystem cache...", Translation::Singular("Opgraderer filsystems cachen..."));

    // Plural translations
    translations.insert("_%n folder_::_%n folders_", Translation::Plural(vec!["%n mappe", "%n mapper"]));
    translations.insert("_%n file_::_%n files_", Translation::Plural(vec!["%n fil", "%n filer"]));
    translations.insert("_Uploading %n file_::_Uploading %n files_", Translation::Plural(vec!["Uploader %n fil", "Uploader %n filer"]));

    translations
});

// Plural forms function for Danish
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

// Enum to store singular or plural translations
#[derive(Debug, Clone)]
pub enum Translation {
    Singular(&'static str),
    Plural(Vec<&'static str>),
}

// Translation functions
pub fn translate(key: &str) -> &'static str {
    match TRANSLATIONS.get(key) {
        Some(Translation::Singular(val)) => val,
        Some(Translation::Plural(vals)) => vals[0],
        None => key,
    }
}

pub fn translate_plural(key: &str, count: usize) -> &'static str {
    let plural_index = get_plural_form(count);
    
    match TRANSLATIONS.get(key) {
        Some(Translation::Plural(vals)) => {
            if plural_index < vals.len() {
                vals[plural_index]
            } else {
                vals[0]
            }
        },
        Some(Translation::Singular(val)) => val,
        None => key,
    }
}