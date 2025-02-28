use std::collections::HashMap;
use rust_i18n::i18n;

// Define translations for Icelandic (is)
pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Could not move %s - File with this name already exists".to_string(), "Gat ekki fært %s - Skrá með þessu nafni er þegar til".to_string());
    translations.insert("Could not move %s".to_string(), "Gat ekki fært %s".to_string());
    translations.insert("File name cannot be empty.".to_string(), "Nafn skráar má ekki vera tómt".to_string());
    translations.insert("No file was uploaded. Unknown error".to_string(), "Engin skrá var send inn. Óþekkt villa.".to_string());
    translations.insert("There is no error, the file uploaded with success".to_string(), "Engin villa, innsending heppnaðist".to_string());
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ".to_string(), "Innsend skrá er stærri en upload_max stillingin í php.ini:".to_string());
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form".to_string(), "Innsenda skráin er stærri en MAX_FILE_SIZE sem skilgreint er í HTML sniðinu.".to_string());
    translations.insert("The uploaded file was only partially uploaded".to_string(), "Einungis hluti af innsendri skrá skilaði sér".to_string());
    translations.insert("No file was uploaded".to_string(), "Engin skrá skilaði sér".to_string());
    translations.insert("Missing a temporary folder".to_string(), "Vantar bráðabirgðamöppu".to_string());
    translations.insert("Failed to write to disk".to_string(), "Tókst ekki að skrifa á disk".to_string());
    translations.insert("Invalid directory.".to_string(), "Ógild mappa.".to_string());
    translations.insert("Files".to_string(), "Skrár".to_string());
    translations.insert("Not enough space available".to_string(), "Ekki nægt pláss tiltækt".to_string());
    translations.insert("Upload cancelled.".to_string(), "Hætt við innsendingu.".to_string());
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.".to_string(), "Innsending í gangi. Ef þú ferð af þessari síðu mun innsending misheppnast.".to_string());
    translations.insert("{new_name} already exists".to_string(), "{new_name} er þegar til".to_string());
    translations.insert("Share".to_string(), "Deila".to_string());
    translations.insert("Rename".to_string(), "Endurskýra".to_string());
    translations.insert("Pending".to_string(), "Bíður".to_string());
    translations.insert("replaced {new_name} with {old_name}".to_string(), "yfirskrifaði {new_name} með {old_name}".to_string());
    translations.insert("undo".to_string(), "afturkalla".to_string());
    translations.insert("'.' is an invalid file name.".to_string(), "'.' er ekki leyfilegt nafn.".to_string());
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.".to_string(), "Ógilt nafn, táknin '\\', '/', '<', '>', ':', '\"', '|', '?' og '*' eru ekki leyfð.".to_string());
    translations.insert("Error".to_string(), "Villa".to_string());
    translations.insert("Name".to_string(), "Nafn".to_string());
    translations.insert("Size".to_string(), "Stærð".to_string());
    translations.insert("Modified".to_string(), "Breytt".to_string());
    translations.insert("Upload".to_string(), "Senda inn".to_string());
    translations.insert("File handling".to_string(), "Meðhöndlun skrár".to_string());
    translations.insert("Maximum upload size".to_string(), "Hámarks stærð innsendingar".to_string());
    translations.insert("max. possible: ".to_string(), "hámark mögulegt: ".to_string());
    translations.insert("Needed for multi-file and folder downloads.".to_string(), "Nauðsynlegt til að sækja margar skrár og möppur í einu.".to_string());
    translations.insert("Enable ZIP-download".to_string(), "Virkja ZIP niðurhal.".to_string());
    translations.insert("0 is unlimited".to_string(), "0 er ótakmarkað".to_string());
    translations.insert("Maximum input size for ZIP files".to_string(), "Hámarks inntaksstærð fyrir ZIP skrár".to_string());
    translations.insert("Save".to_string(), "Vista".to_string());
    translations.insert("New".to_string(), "Nýtt".to_string());
    translations.insert("Text file".to_string(), "Texta skrá".to_string());
    translations.insert("Folder".to_string(), "Mappa".to_string());
    translations.insert("From link".to_string(), "Af tengli".to_string());
    translations.insert("Cancel upload".to_string(), "Hætta við innsendingu".to_string());
    translations.insert("Nothing in here. Upload something!".to_string(), "Ekkert hér. Settu eitthvað inn!".to_string());
    translations.insert("Download".to_string(), "Niðurhal".to_string());
    translations.insert("Unshare".to_string(), "Hætta deilingu".to_string());
    translations.insert("Delete".to_string(), "Eyða".to_string());
    translations.insert("Upload too large".to_string(), "Innsend skrá er of stór".to_string());
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.".to_string(), "Skrárnar sem þú ert að senda inn eru stærri en hámarks innsendingarstærð á þessum netþjóni.".to_string());
    translations.insert("Files are being scanned, please wait.".to_string(), "Verið er að skima skrár, vinsamlegast hinkraðu.".to_string());
    translations.insert("Current scanning".to_string(), "Er að skima".to_string());
    
    translations
}

// Define plural forms info
pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Define plural translations
pub fn get_plural_translations() -> HashMap<String, Vec<String>> {
    let mut plural_translations = HashMap::new();
    
    plural_translations.insert("_%n folder_::_%n folders_".to_string(), vec!["".to_string(), "".to_string()]);
    plural_translations.insert("_%n file_::_%n files_".to_string(), vec!["".to_string(), "".to_string()]);
    plural_translations.insert("_Uploading %n file_::_Uploading %n files_".to_string(), vec!["".to_string(), "".to_string()]);
    
    plural_translations
}

// Initialize the i18n if needed
pub fn initialize_is_locale() -> (HashMap<String, String>, HashMap<String, Vec<String>>, &'static str) {
    (get_translations(), get_plural_translations(), get_plural_forms())
}