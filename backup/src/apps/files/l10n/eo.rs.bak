use std::collections::HashMap;
use once_cell::sync::Lazy;
use rust_i18n::t;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Could not move %s - File with this name already exists", "Ne eblis movi %s: dosiero kun ĉi tiu nomo jam ekzistas");
    m.insert("Could not move %s", "Ne eblis movi %s");
    m.insert("File name cannot be empty.", "Dosiernomo devas ne malpleni.");
    m.insert("File name must not contain \"/\". Please choose a different name.", "La dosieronomo ne devas enhavi "/". Bonvolu elekti malsaman nomon.");
    m.insert("The name %s is already used in the folder %s. Please choose a different name.", "La nomo %s jam uziĝas en la dosierujo %s. Bonvolu elekti malsaman nomon.");
    m.insert("Not a valid source", "Nevalida fonto");
    m.insert("Error while downloading %s to %s", "Eraris elŝuto de %s al %s");
    m.insert("Error when creating the file", "Eraris la kreo de la dosiero");
    m.insert("Folder name cannot be empty.", "La dosierujnomo ne povas malpleni.");
    m.insert("Folder name must not contain \"/\". Please choose a different name.", "La dosiernomo ne devas enhavi "/". Bonvolu elekti malsaman nomon.");
    m.insert("Error when creating the folder", "Eraris la kreo de la dosierujo");
    m.insert("Unable to set upload directory.", "Ne povis agordiĝi la alŝuta dosierujo.");
    m.insert("No file was uploaded. Unknown error", "Neniu dosiero alŝutiĝis. Nekonata eraro.");
    m.insert("There is no error, the file uploaded with success", "Ne estas eraro, la dosiero alŝutiĝis sukcese.");
    m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "La dosiero alŝutita superas la regulon upload_max_filesize el php.ini: ");
    m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "La dosiero alŝutita superas la regulon MAX_FILE_SIZE, kiu estas difinita en la HTML-formularo");
    m.insert("The uploaded file was only partially uploaded", "la alŝutita dosiero nur parte alŝutiĝis");
    m.insert("No file was uploaded", "Neniu dosiero alŝutiĝis.");
    m.insert("Missing a temporary folder", "Mankas provizora dosierujo.");
    m.insert("Failed to write to disk", "Malsukcesis skribo al disko");
    m.insert("Not enough storage available", "Ne haveblas sufiĉa memoro");
    m.insert("Upload failed. Could not get file info.", "La alŝuto malsukcesis. Ne povis ekhaviĝi informo pri dosiero.");
    m.insert("Upload failed. Could not find uploaded file", "La alŝuto malsukcesis. Ne troviĝis alŝutota dosiero.");
    m.insert("Invalid directory.", "Nevalida dosierujo.");
    m.insert("Files", "Dosieroj");
    m.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Ne povis alŝutiĝi {filename} ĉar ĝi estas dosierujo aŭ ĝi havas 0 duumokojn");
    m.insert("Not enough space available", "Ne haveblas sufiĉa spaco");
    m.insert("Upload cancelled.", "La alŝuto nuliĝis.");
    m.insert("Could not get result from server.", "Ne povis ekhaviĝi rezulto el la servilo.");
    m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Dosieralŝuto plenumiĝas. Lasi la paĝon nun nuligus la alŝuton.");
    m.insert("URL cannot be empty", "La URL ne povas malpleni");
    m.insert("{new_name} already exists", "{new_name} jam ekzistas");
    m.insert("Could not create file", "Ne povis kreiĝi dosiero");
    m.insert("Could not create folder", "Ne povis kreiĝi dosierujo");
    m.insert("Share", "Kunhavigi");
    m.insert("Delete permanently", "Forigi por ĉiam");
    m.insert("Rename", "Alinomigi");
    m.insert("Pending", "Traktotaj");
    m.insert("Could not rename file", "Ne povis alinomiĝi dosiero");
    m.insert("replaced {new_name} with {old_name}", "anstataŭiĝis {new_name} per {old_name}");
    m.insert("undo", "malfari");
    m.insert("_%n folder_::_%n folders_", "%n dosierujo|%n dosierujoj");
    m.insert("_%n file_::_%n files_", "%n dosiero|%n dosieroj");
    m.insert("{dirs} and {files}", "{dirs} kaj {files}");
    m.insert("_Uploading %n file_::_Uploading %n files_", "Alŝutatas %n dosiero|Alŝutatas %n dosieroj");
    m.insert("'.' is an invalid file name.", "'.' ne estas valida dosiernomo.");
    m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Nevalida nomo: "\\", "/", "<", ">", ":", "\"", "|", "?" kaj "*" ne permesatas.");
    m.insert("Your storage is full, files can not be updated or synced anymore!", "Via memoro plenas, ne plu eblas ĝisdatigi aŭ sinkronigi dosierojn!");
    m.insert("Your storage is almost full ({usedSpacePercent}%)", "Via memoro preskaŭ plenas ({usedSpacePercent}%)");
    m.insert("Your download is being prepared. This might take some time if the files are big.", "Via elŝuto pretiĝatas. Ĉi tio povas daŭri iom da tempo se la dosieroj grandas.");
    m.insert("Error moving file", "Eraris movo de dosiero");
    m.insert("Error", "Eraro");
    m.insert("Name", "Nomo");
    m.insert("Size", "Grando");
    m.insert("Modified", "Modifita");
    m.insert("%s could not be renamed", "%s ne povis alinomiĝi");
    m.insert("Upload", "Alŝuti");
    m.insert("File handling", "Dosieradministro");
    m.insert("Maximum upload size", "Maksimuma alŝutogrando");
    m.insert("max. possible: ", "maks. ebla: ");
    m.insert("Needed for multi-file and folder downloads.", "Necesa por elŝuto de pluraj dosieroj kaj dosierujoj.");
    m.insert("Enable ZIP-download", "Kapabligi ZIP-elŝuton");
    m.insert("0 is unlimited", "0 signifas senlime");
    m.insert("Maximum input size for ZIP files", "Maksimuma enirgrando por ZIP-dosieroj");
    m.insert("Save", "Konservi");
    m.insert("New", "Nova");
    m.insert("Text file", "Tekstodosiero");
    m.insert("Folder", "Dosierujo");
    m.insert("From link", "El ligilo");
    m.insert("Deleted files", "Forigitaj dosieroj");
    m.insert("Cancel upload", "Nuligi alŝuton");
    m.insert("You don't have permission to upload or create files here", "Vi ne havas permeson alŝuti aŭ krei dosierojn ĉi tie");
    m.insert("Nothing in here. Upload something!", "Nenio estas ĉi tie. Alŝutu ion!");
    m.insert("Download", "Elŝuti");
    m.insert("Unshare", "Malkunhavigi");
    m.insert("Delete", "Forigi");
    m.insert("Upload too large", "Alŝuto tro larĝa");
    m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "La dosieroj, kiujn vi provas alŝuti, transpasas la maksimuman grandon por dosieralŝutoj en ĉi tiu servilo.");
    m.insert("Files are being scanned, please wait.", "Dosieroj estas skanataj, bonvolu atendi.");
    m.insert("Current scanning", "Nuna skano");
    m.insert("Upgrading filesystem cache...", "Ĝisdatiĝas dosiersistema kaŝmemoro...");
    m
});

pub fn get_plural_form(n: i64) -> usize {
    if n != 1 { 1 } else { 0 }
}

pub fn register_plurals() {
    rust_i18n::set_locale("eo");
    rust_i18n::set_plural_rule(|n| get_plural_form(n));
}

// Función auxiliar para traducir con formato
pub fn translate(key: &str, params: &[&str]) -> String {
    let translation = TRANSLATIONS.get(key).copied().unwrap_or(key);
    
    if params.is_empty() {
        return translation.to_string();
    }
    
    let mut result = translation.to_string();
    for (i, param) in params.iter().enumerate() {
        result = result.replace(&format!("%s", i + 1), param);
    }
    result
}

// Función auxiliar para traducir texto con plurales
pub fn translate_plural(singular: &str, plural: &str, n: i64) -> String {
    let key = format!("_{}_::_{}_", singular, plural);
    
    if let Some(translation) = TRANSLATIONS.get(key.as_str()) {
        let parts: Vec<&str> = translation.split('|').collect();
        let idx = get_plural_form(n);
        
        if idx < parts.len() {
            return parts[idx].replace("%n", &n.to_string());
        }
    }
    
    if n == 1 {
        singular.replace("%n", &n.to_string())
    } else {
        plural.replace("%n", &n.to_string())
    }
}