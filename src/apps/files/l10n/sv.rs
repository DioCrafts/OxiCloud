use std::collections::HashMap;
use rust_i18n::i18n;

// Define translations for Swedish locale
i18n!("sv", {
    "Could not move %s - File with this name already exists": "Kunde inte flytta %s - Det finns redan en fil med detta namn",
    "Could not move %s": "Kan inte flytta %s",
    "File name cannot be empty.": "Filnamn kan inte vara tomt.",
    "File name must not contain \"/\". Please choose a different name.": "Filnamnet får ej innehålla \"/\". Välj ett annat namn.",
    "The name %s is already used in the folder %s. Please choose a different name.": "Namnet %s används  redan i katalogen %s. Välj ett annat namn.",
    "Not a valid source": "Inte en giltig källa",
    "Error while downloading %s to %s": "Fel under nerladdning från %s till %s",
    "Error when creating the file": "Fel under skapande utav filen",
    "Folder name cannot be empty.": "Katalognamn kan ej vara tomt.",
    "Folder name must not contain \"/\". Please choose a different name.": "Katalog namnet får ej innehålla \"/\". Välj ett annat namn.",
    "Error when creating the folder": "Fel under skapande utav en katalog",
    "Unable to set upload directory.": "Kan inte sätta mapp för uppladdning.",
    "Invalid Token": "Ogiltig token",
    "No file was uploaded. Unknown error": "Ingen fil uppladdad. Okänt fel",
    "There is no error, the file uploaded with success": "Inga fel uppstod. Filen laddades upp utan problem.",
    "The uploaded file exceeds the upload_max_filesize directive in php.ini: ": "Den uppladdade filen överskrider upload_max_filesize direktivet php.ini:",
    "The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form": "Den uppladdade filen överskrider MAX_FILE_SIZE direktivet som har angetts i HTML formuläret",
    "The uploaded file was only partially uploaded": "Den uppladdade filen var endast delvis uppladdad",
    "No file was uploaded": "Ingen fil laddades upp",
    "Missing a temporary folder": "En temporär mapp saknas",
    "Failed to write to disk": "Misslyckades spara till disk",
    "Not enough storage available": "Inte tillräckligt med lagringsutrymme tillgängligt",
    "Upload failed. Could not get file info.": "Uppladdning misslyckades. Gick inte att hämta filinformation.",
    "Upload failed. Could not find uploaded file": "Uppladdning misslyckades. Kunde inte hitta den uppladdade filen",
    "Invalid directory.": "Felaktig mapp.",
    "Files": "Filer",
    "Unable to upload {filename} as it is a directory or has 0 bytes": "Kan inte ladda upp {filename} eftersom den antingen är en mapp eller har 0 bytes.",
    "Not enough space available": "Inte tillräckligt med utrymme tillgängligt",
    "Upload cancelled.": "Uppladdning avbruten.",
    "Could not get result from server.": "Gick inte att hämta resultat från server.",
    "File upload is in progress. Leaving the page now will cancel the upload.": "Filuppladdning pågår. Lämnar du sidan så avbryts uppladdningen.",
    "URL cannot be empty": "URL kan ej vara tomt",
    "In the home folder 'Shared' is a reserved filename": "I hemma katalogen 'Delat' är ett reserverat filnamn",
    "{new_name} already exists": "{new_name} finns redan",
    "Could not create file": "Kunde ej skapa fil",
    "Could not create folder": "Kunde ej skapa katalog",
    "Share": "Dela",
    "Delete permanently": "Radera permanent",
    "Rename": "Byt namn",
    "Pending": "Väntar",
    "Could not rename file": "Kan ej byta filnamn",
    "replaced {new_name} with {old_name}": "ersatt {new_name} med {old_name}",
    "undo": "ångra",
    "_%n folder_::_%n folders_[one]": "%n mapp",
    "_%n folder_::_%n folders_[other]": "%n mappar",
    "_%n file_::_%n files_[one]": "%n fil",
    "_%n file_::_%n files_[other]": "%n filer",
    "{dirs} and {files}": "{dirs} och {files}",
    "_Uploading %n file_::_Uploading %n files_[one]": "Laddar upp %n fil",
    "_Uploading %n file_::_Uploading %n files_[other]": "Laddar upp %n filer",
    "'.' is an invalid file name.": "'.' är ett ogiltigt filnamn.",
    "Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.": "Ogiltigt namn, '\\', '/', '<', '>', ':', '\"', '|', '?' och '*' är inte tillåtet.",
    "Your storage is full, files can not be updated or synced anymore!": "Ditt lagringsutrymme är fullt, filer kan inte längre uppdateras eller synkroniseras!",
    "Your storage is almost full ({usedSpacePercent}%)": "Ditt lagringsutrymme är nästan fullt ({usedSpacePercent}%)",
    "Encryption App is enabled but your keys are not initialized, please log-out and log-in again": "Krypteringsprogrammet är aktiverat men dina nycklar är inte initierade. Vänligen logga ut och in igen",
    "Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.": "Ogiltig privat nyckel i krypteringsprogrammet. Vänligen uppdatera lösenordet till din privata nyckel under dina personliga inställningar för att återfå tillgång till dina krypterade filer.",
    "Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.": "Kryptering inaktiverades men dina filer är fortfarande krypterade. Vänligen gå till sidan för dina personliga inställningar för att dekryptera dina filer.",
    "Your download is being prepared. This might take some time if the files are big.": "Din nedladdning förbereds. Det kan ta tid om det är stora filer.",
    "Error moving file": "Fel uppstod vid flyttning av fil",
    "Error": "Fel",
    "Name": "Namn",
    "Size": "Storlek",
    "Modified": "Ändrad",
    "%s could not be renamed": "%s kunde inte namnändras",
    "Upload": "Ladda upp",
    "File handling": "Filhantering",
    "Maximum upload size": "Maximal storlek att ladda upp",
    "max. possible: ": "max. möjligt:",
    "Needed for multi-file and folder downloads.": "Krävs för nerladdning av flera mappar och filer.",
    "Enable ZIP-download": "Aktivera ZIP-nerladdning",
    "0 is unlimited": "0 är oändligt",
    "Maximum input size for ZIP files": "Största tillåtna storlek för ZIP-filer",
    "Save": "Spara",
    "New": "Ny",
    "Text file": "Textfil",
    "Folder": "Mapp",
    "From link": "Från länk",
    "Deleted files": "Raderade filer",
    "Cancel upload": "Avbryt uppladdning",
    "You don't have permission to upload or create files here": "Du har ej tillåtelse att ladda upp eller skapa filer här",
    "Nothing in here. Upload something!": "Ingenting här. Ladda upp något!",
    "Download": "Ladda ner",
    "Unshare": "Sluta dela",
    "Delete": "Radera",
    "Upload too large": "För stor uppladdning",
    "The files you are trying to upload exceed the maximum size for file uploads on this server.": "Filerna du försöker ladda upp överstiger den maximala storleken för filöverföringar på servern.",
    "Files are being scanned, please wait.": "Filer skannas, var god vänta",
    "Current scanning": "Aktuell skanning",
    "Upgrading filesystem cache...": "Uppgraderar filsystemets cache..."
});

// Define the plural forms function for Swedish
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

pub fn init_translations() -> HashMap<String, String> {
    // This function can be used to initialize the translations if needed
    // outside of the i18n! macro usage
    let mut translations = HashMap::new();
    
    // Add all translations to the HashMap
    translations.insert(
        "Could not move %s - File with this name already exists".to_string(),
        "Kunde inte flytta %s - Det finns redan en fil med detta namn".to_string(),
    );
    // All other translations would be added here...
    
    translations
}