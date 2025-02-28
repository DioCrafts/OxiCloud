use std::collections::HashMap;
use rust_gettext::prelude::*;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Could not move %s - File with this name already exists", "Kan ikke flytte %s - En fil med samme navn finnes allerede");
    translations.insert("Could not move %s", "Kunne ikke flytte %s");
    translations.insert("File name cannot be empty.", "Filnavn kan ikke være tomt.");
    translations.insert("Unable to set upload directory.", "Kunne ikke sette opplastingskatalog.");
    translations.insert("Invalid Token", "Ugyldig nøkkel");
    translations.insert("No file was uploaded. Unknown error", "Ingen filer ble lastet opp. Ukjent feil.");
    translations.insert("There is no error, the file uploaded with success", "Pust ut, ingen feil. Filen ble lastet opp problemfritt");
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Filstørrelsen overskrider maksgrensedirektivet upload_max_filesize i php.ini-konfigurasjonen.");
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Filen du prøvde å laste opp var større enn grensen satt i MAX_FILE_SIZE i HTML-skjemaet.");
    translations.insert("The uploaded file was only partially uploaded", "Filen du prøvde å laste opp ble kun delvis lastet opp");
    translations.insert("No file was uploaded", "Ingen filer ble lastet opp");
    translations.insert("Missing a temporary folder", "Mangler midlertidig mappe");
    translations.insert("Failed to write to disk", "Klarte ikke å skrive til disk");
    translations.insert("Not enough storage available", "Ikke nok lagringsplass");
    translations.insert("Invalid directory.", "Ugyldig katalog.");
    translations.insert("Files", "Filer");
    translations.insert("Not enough space available", "Ikke nok lagringsplass");
    translations.insert("Upload cancelled.", "Opplasting avbrutt.");
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Filopplasting pågår. Forlater du siden nå avbrytes opplastingen.");
    translations.insert("{new_name} already exists", "{new_name} finnes allerede");
    translations.insert("Share", "Del");
    translations.insert("Delete permanently", "Slett permanent");
    translations.insert("Rename", "Gi nytt navn");
    translations.insert("Pending", "Ventende");
    translations.insert("replaced {new_name} with {old_name}", "erstattet {new_name} med {old_name}");
    translations.insert("undo", "angre");
    translations.insert("_%n folder_::_%n folders_", "%n mappe\u{0}%n mapper");
    translations.insert("_%n file_::_%n files_", "%n fil\u{0}%n filer");
    translations.insert("_Uploading %n file_::_Uploading %n files_", "Laster opp %n fil\u{0}Laster opp %n filer");
    translations.insert("'.' is an invalid file name.", "'.' er et ugyldig filnavn.");
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Ugyldig navn, '\\', '/', '<', '>', ':', '\"', '|', '?' og '*' er ikke tillatt.");
    translations.insert("Your storage is full, files can not be updated or synced anymore!", "Lagringsplass er oppbrukt, filer kan ikke lenger oppdateres eller synkroniseres!");
    translations.insert("Your storage is almost full ({usedSpacePercent}%)", "Lagringsplass er nesten brukt opp ([usedSpacePercent}%)");
    translations.insert("Your download is being prepared. This might take some time if the files are big.", "Nedlastingen din klargjøres. Hvis filene er store kan dette ta litt tid.");
    translations.insert("Error", "Feil");
    translations.insert("Name", "Navn");
    translations.insert("Size", "Størrelse");
    translations.insert("Modified", "Endret");
    translations.insert("%s could not be renamed", "Kunne ikke gi nytt navn til %s");
    translations.insert("Upload", "Last opp");
    translations.insert("File handling", "Filhåndtering");
    translations.insert("Maximum upload size", "Maksimum opplastingsstørrelse");
    translations.insert("max. possible: ", "max. mulige:");
    translations.insert("Needed for multi-file and folder downloads.", "Nødvendig for å laste ned mapper og mer enn én fil om gangen.");
    translations.insert("Enable ZIP-download", "Aktiver nedlasting av ZIP");
    translations.insert("0 is unlimited", "0 er ubegrenset");
    translations.insert("Maximum input size for ZIP files", "Maksimal størrelse på ZIP-filer");
    translations.insert("Save", "Lagre");
    translations.insert("New", "Ny");
    translations.insert("Text file", "Tekstfil");
    translations.insert("Folder", "Mappe");
    translations.insert("From link", "Fra link");
    translations.insert("Deleted files", "Slettet filer");
    translations.insert("Cancel upload", "Avbryt opplasting");
    translations.insert("Nothing in here. Upload something!", "Ingenting her. Last opp noe!");
    translations.insert("Download", "Last ned");
    translations.insert("Unshare", "Avslutt deling");
    translations.insert("Delete", "Slett");
    translations.insert("Upload too large", "Filen er for stor");
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Filene du prøver å laste opp er for store for å laste opp til denne serveren.");
    translations.insert("Files are being scanned, please wait.", "Skanner filer, vennligst vent.");
    translations.insert("Current scanning", "Pågående skanning");
    translations.insert("Upgrading filesystem cache...", "Oppgraderer filsystemets  mellomlager...");
    
    translations
}

pub fn get_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Configuración del catálogo de traducción para noruego
pub fn setup_nb_no_locale() -> Catalog {
    let catalog = Catalog::new("nb_NO")
        .set_plural_form_function(|n| if n != 1 { 1 } else { 0 })
        .set_translations(get_translations());
    
    catalog
}