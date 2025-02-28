// Basque translations for the Files app.

use std::collections::HashMap;
use lazy_static::lazy_static;
use rust_i18n::i18n;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "Ezin da %s mugitu - Izen hau duen fitxategia dagoeneko existitzen da");
        m.insert("Could not move %s", "Ezin dira fitxategiak mugitu %s");
        m.insert("File name cannot be empty.", "Fitxategi izena ezin da hutsa izan.");
        m.insert("Unable to set upload directory.", "Ezin da igoera direktorioa ezarri.");
        m.insert("Invalid Token", "Lekuko baliogabea");
        m.insert("No file was uploaded. Unknown error", "Ez da fitxategirik igo. Errore ezezaguna");
        m.insert("There is no error, the file uploaded with success", "Ez da errorerik egon, fitxategia ongi igo da");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Igotako fitxategiak php.ini fitxategian ezarritako upload_max_filesize muga gainditu du:");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Igotako fitxategia HTML formularioan zehaztutako MAX_FILE_SIZE direktiba baino handidagoa da.");
        m.insert("The uploaded file was only partially uploaded", "Igotako fitxategiaren zati bat bakarrik igo da");
        m.insert("No file was uploaded", "Ez da fitxategirik igo");
        m.insert("Missing a temporary folder", "Aldi bateko karpeta falta da");
        m.insert("Failed to write to disk", "Errore bat izan da diskoan idazterakoan");
        m.insert("Not enough storage available", "Ez dago behar aina leku erabilgarri,");
        m.insert("Upload failed. Could not get file info.", "Igoerak huts egin du. Ezin izan da fitxategiaren informazioa eskuratu.");
        m.insert("Upload failed. Could not find uploaded file", "Igoerak huts egin du. Ezin izan da igotako fitxategia aurkitu");
        m.insert("Invalid directory.", "Baliogabeko karpeta.");
        m.insert("Files", "Fitxategiak");
        m.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Ezin da {filename} igo karpeta bat delako edo 0 byte dituelako");
        m.insert("Not enough space available", "Ez dago leku nahikorik.");
        m.insert("Upload cancelled.", "Igoera ezeztatuta");
        m.insert("Could not get result from server.", "Ezin da zerbitzaritik emaitzik lortu");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Fitxategien igoera martxan da. Orria orain uzteak igoera ezeztatutko du.");
        m.insert("{new_name} already exists", "{new_name} dagoeneko existitzen da");
        m.insert("Share", "Elkarbanatu");
        m.insert("Delete permanently", "Ezabatu betirako");
        m.insert("Rename", "Berrizendatu");
        m.insert("Pending", "Zain");
        m.insert("replaced {new_name} with {old_name}", " {new_name}-k {old_name} ordezkatu du");
        m.insert("undo", "desegin");
        m.insert("_%n folder_::_%n folders_", "karpeta %n|%n karpeta");
        m.insert("_%n file_::_%n files_", "fitxategi %n|%n fitxategi");
        m.insert("{dirs} and {files}", "{dirs} eta {files}");
        m.insert("_Uploading %n file_::_Uploading %n files_", "Fitxategi %n igotzen|%n fitxategi igotzen");
        m.insert("'.' is an invalid file name.", "'.' ez da fitxategi izen baliogarria.");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "IZen aliogabea, '\\', '/', '<', '>', ':', '\"', '|', '?' eta '*' ez daude baimenduta.");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "Zure biltegiratzea beterik dago, ezingo duzu aurrerantzean fitxategirik igo edo sinkronizatu!");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "Zure biltegiratzea nahiko beterik dago (%{usedSpacePercent})");
        m.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "Enkriptazio aplikazioa gaituta dago baina zure gakoak ez daude konfiguratuta, mesedez saioa bukatu eta berriro hasi");
        m.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "Enkriptazio aplikaziorako gako pribatu okerra. Mesedez eguneratu zure gako pribatuaren pasahitza zure ezarpen pertsonaletan zure enkriptatuko fitxategietarako sarrera berreskuratzeko.");
        m.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "Enkriptazioa desgaitua izan da baina zure fitxategiak oraindik enkriptatuta daude. Mesedez  jo zure ezarpen pertsonaletara zure fitxategiak dekodifikatzeko.");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "Zure deskarga prestatu egin behar da. Denbora bat har lezake fitxategiak handiak badira. ");
        m.insert("Error moving file", "Errorea fitxategia mugitzean");
        m.insert("Error", "Errorea");
        m.insert("Name", "Izena");
        m.insert("Size", "Tamaina");
        m.insert("Modified", "Aldatuta");
        m.insert("%s could not be renamed", "%s ezin da berrizendatu");
        m.insert("Upload", "Igo");
        m.insert("File handling", "Fitxategien kudeaketa");
        m.insert("Maximum upload size", "Igo daitekeen gehienezko tamaina");
        m.insert("max. possible: ", "max, posiblea:");
        m.insert("Needed for multi-file and folder downloads.", "Beharrezkoa fitxategi-anitz eta karpeten deskargarako.");
        m.insert("Enable ZIP-download", "Gaitu ZIP-deskarga");
        m.insert("0 is unlimited", "0 mugarik gabe esan nahi du");
        m.insert("Maximum input size for ZIP files", "ZIP fitxategien gehienezko tamaina");
        m.insert("Save", "Gorde");
        m.insert("New", "Berria");
        m.insert("Text file", "Testu fitxategia");
        m.insert("Folder", "Karpeta");
        m.insert("From link", "Estekatik");
        m.insert("Deleted files", "Ezabatutako fitxategiak");
        m.insert("Cancel upload", "Ezeztatu igoera");
        m.insert("Nothing in here. Upload something!", "Ez dago ezer. Igo zerbait!");
        m.insert("Download", "Deskargatu");
        m.insert("Unshare", "Ez elkarbanatu");
        m.insert("Delete", "Ezabatu");
        m.insert("Upload too large", "Igoera handiegia da");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Igotzen saiatzen ari zaren fitxategiak zerbitzari honek igotzeko onartzen duena baino handiagoak dira.");
        m.insert("Files are being scanned, please wait.", "Fitxategiak eskaneatzen ari da, itxoin mezedez.");
        m.insert("Current scanning", "Orain eskaneatzen ari da");
        m.insert("Upgrading filesystem cache...", "Fitxategi sistemaren katxea eguneratzen...");
        m
    };
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

/// Translates a string using the loaded translations.
/// 
/// # Arguments
/// 
/// * `key` - The translation key to look up
/// * `args` - Optional format arguments
/// 
/// # Returns
/// 
/// The translated string or the key itself if no translation exists
pub fn translate(key: &str, args: &[&str]) -> String {
    match TRANSLATIONS.get(key) {
        Some(translation) => {
            if args.is_empty() {
                translation.to_string()
            } else {
                // Simple formatting implementation for %s placeholders
                let mut result = translation.to_string();
                for arg in args {
                    result = result.replacen("%s", arg, 1);
                }
                result
            }
        },
        None => key.to_string(),
    }
}

/// Handles pluralized translations.
/// 
/// # Arguments
/// 
/// * `singular` - The singular form key
/// * `plural` - The plural form key
/// * `count` - The count to determine which form to use
/// 
/// # Returns
/// 
/// The translated string with count inserted
pub fn translate_plural(key: &str, count: usize) -> String {
    if let Some(translation) = TRANSLATIONS.get(key) {
        let forms: Vec<&str> = translation.split('|').collect();
        let form = if count == 1 { 0 } else { 1 };
        
        if form < forms.len() {
            return forms[form].replace("%n", &count.to_string());
        }
    }
    
    // Fallback
    format!("{}_{}", key, count)
}