use std::collections::HashMap;
use rust_gettext::Catalog;

pub fn get_translation_map() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Could not move %s - File with this name already exists".to_string(), "No s'ha pogut moure %s - Ja hi ha un fitxer amb aquest nom".to_string());
    translations.insert("Could not move %s".to_string(), " No s'ha pogut moure %s".to_string());
    translations.insert("File name cannot be empty.".to_string(), "El nom del fitxer no pot ser buit.".to_string());
    translations.insert("File name must not contain \"/\". Please choose a different name.".to_string(), "El nom de fitxer no pot contenir \"/\". Indiqueu un nom diferent.".to_string());
    translations.insert("The name %s is already used in the folder %s. Please choose a different name.".to_string(), "El nom %s ja s'usa en la carpeta %s. Indiqueu un nom diferent.".to_string());
    translations.insert("Not a valid source".to_string(), "No és un origen vàlid".to_string());
    translations.insert("Error while downloading %s to %s".to_string(), "S'ha produït un error en baixar %s a %s".to_string());
    translations.insert("Error when creating the file".to_string(), "S'ha produït un error en crear el fitxer".to_string());
    translations.insert("Folder name cannot be empty.".to_string(), "El nom de la carpeta no pot ser buit.".to_string());
    translations.insert("Folder name must not contain \"/\". Please choose a different name.".to_string(), "El nom de la carpeta no pot contenir \"/\". Indiqueu un nom diferent.".to_string());
    translations.insert("Error when creating the folder".to_string(), "S'ha produït un error en crear la carpeta".to_string());
    translations.insert("Unable to set upload directory.".to_string(), "No es pot establir la carpeta de pujada.".to_string());
    translations.insert("Invalid Token".to_string(), "Testimoni no vàlid".to_string());
    translations.insert("No file was uploaded. Unknown error".to_string(), "No s'ha carregat cap fitxer. Error desconegut".to_string());
    translations.insert("There is no error, the file uploaded with success".to_string(), "No hi ha errors, el fitxer s'ha carregat correctament".to_string());
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ".to_string(), "L'arxiu que voleu carregar supera el màxim definit en la directiva upload_max_filesize del php.ini:".to_string());
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form".to_string(), "El fitxer carregat supera la directiva MAX_FILE_SIZE especificada al formulari HTML".to_string());
    translations.insert("The uploaded file was only partially uploaded".to_string(), "El fitxer només s'ha carregat parcialment".to_string());
    translations.insert("No file was uploaded".to_string(), "No s'ha carregat cap fitxer".to_string());
    translations.insert("Missing a temporary folder".to_string(), "Falta un fitxer temporal".to_string());
    translations.insert("Failed to write to disk".to_string(), "Ha fallat en escriure al disc".to_string());
    translations.insert("Not enough storage available".to_string(), "No hi ha prou espai disponible".to_string());
    translations.insert("Upload failed. Could not get file info.".to_string(), "La pujada ha fallat. No s'ha pogut obtenir informació del fitxer.".to_string());
    translations.insert("Upload failed. Could not find uploaded file".to_string(), "La pujada ha fallat. El fitxer pujat no s'ha trobat.".to_string());
    translations.insert("Invalid directory.".to_string(), "Directori no vàlid.".to_string());
    translations.insert("Files".to_string(), "Fitxers".to_string());
    translations.insert("Unable to upload {filename} as it is a directory or has 0 bytes".to_string(), "No es pot pujar {filename} perquè és una carpeta o té 0 bytes".to_string());
    translations.insert("Not enough space available".to_string(), "No hi ha prou espai disponible".to_string());
    translations.insert("Upload cancelled.".to_string(), "La pujada s'ha cancel·lat.".to_string());
    translations.insert("Could not get result from server.".to_string(), "No hi ha resposta del servidor.".to_string());
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.".to_string(), "Hi ha una pujada en curs. Si abandoneu la pàgina la pujada es cancel·larà.".to_string());
    translations.insert("URL cannot be empty".to_string(), "L'URL no pot ser buit".to_string());
    translations.insert("In the home folder 'Shared' is a reserved filename".to_string(), "A la carpeta inici 'Compartit' és un nom de fitxer reservat".to_string());
    translations.insert("{new_name} already exists".to_string(), "{new_name} ja existeix".to_string());
    translations.insert("Could not create file".to_string(), "No s'ha pogut crear el fitxer".to_string());
    translations.insert("Could not create folder".to_string(), "No s'ha pogut crear la carpeta".to_string());
    translations.insert("Share".to_string(), "Comparteix".to_string());
    translations.insert("Delete permanently".to_string(), "Esborra permanentment".to_string());
    translations.insert("Rename".to_string(), "Reanomena".to_string());
    translations.insert("Pending".to_string(), "Pendent".to_string());
    translations.insert("Could not rename file".to_string(), "No es pot canviar el nom de fitxer".to_string());
    translations.insert("replaced {new_name} with {old_name}".to_string(), "s'ha substituït {old_name} per {new_name}".to_string());
    translations.insert("undo".to_string(), "desfés".to_string());
    translations.insert("_%n folder_::_%n folders_".to_string(), "%n carpeta|%n carpetes".to_string());
    translations.insert("_%n file_::_%n files_".to_string(), "%n fitxer|%n fitxers".to_string());
    translations.insert("{dirs} and {files}".to_string(), "{dirs} i {files}".to_string());
    translations.insert("_Uploading %n file_::_Uploading %n files_".to_string(), "Pujant %n fitxer|Pujant %n fitxers".to_string());
    translations.insert("'.' is an invalid file name.".to_string(), "'.' és un nom no vàlid per un fitxer.".to_string());
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.".to_string(), "El nóm no és vàlid, '\\', '/', '<', '>', ':', '\"', '|', '?' i '*' no estan permesos.".to_string());
    translations.insert("Your storage is full, files can not be updated or synced anymore!".to_string(), "El vostre espai d'emmagatzemament és ple, els fitxers ja no es poden actualitzar o sincronitzar!".to_string());
    translations.insert("Your storage is almost full ({usedSpacePercent}%)".to_string(), "El vostre espai d'emmagatzemament és gairebé ple ({usedSpacePercent}%)".to_string());
    translations.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again".to_string(), "L'aplicació d'encriptació està activada però les claus no estan inicialitzades, sortiu i acrediteu-vos de nou.".to_string());
    translations.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.".to_string(), "La clau privada de l'aplicació d'encriptació no és vàlida! Actualitzeu la contrasenya de la clau privada a l'arranjament personal per recuperar els fitxers encriptats.".to_string());
    translations.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.".to_string(), "L'encriptació s'ha desactivat però els vostres fitxers segueixen encriptats. Aneu a la vostra configuració personal per desencriptar els vostres fitxers.".to_string());
    translations.insert("Your download is being prepared. This might take some time if the files are big.".to_string(), "S'està preparant la baixada. Pot trigar una estona si els fitxers són grans.".to_string());
    translations.insert("Error moving file".to_string(), "Error en moure el fitxer".to_string());
    translations.insert("Error".to_string(), "Error".to_string());
    translations.insert("Name".to_string(), "Nom".to_string());
    translations.insert("Size".to_string(), "Mida".to_string());
    translations.insert("Modified".to_string(), "Modificat".to_string());
    translations.insert("Invalid folder name. Usage of 'Shared' is reserved.".to_string(), "Nom de carpeta no vàlid. L'ús de 'Shared' és reservat".to_string());
    translations.insert("%s could not be renamed".to_string(), "%s no es pot canviar el nom".to_string());
    translations.insert("Upload".to_string(), "Puja".to_string());
    translations.insert("File handling".to_string(), "Gestió de fitxers".to_string());
    translations.insert("Maximum upload size".to_string(), "Mida màxima de pujada".to_string());
    translations.insert("max. possible: ".to_string(), "màxim possible:".to_string());
    translations.insert("Needed for multi-file and folder downloads.".to_string(), "Necessari per fitxers múltiples i baixada de carpetes".to_string());
    translations.insert("Enable ZIP-download".to_string(), "Activa la baixada ZIP".to_string());
    translations.insert("0 is unlimited".to_string(), "0 és sense límit".to_string());
    translations.insert("Maximum input size for ZIP files".to_string(), "Mida màxima d'entrada per fitxers ZIP".to_string());
    translations.insert("Save".to_string(), "Desa".to_string());
    translations.insert("New".to_string(), "Nou".to_string());
    translations.insert("Text file".to_string(), "Fitxer de text".to_string());
    translations.insert("Folder".to_string(), "Carpeta".to_string());
    translations.insert("From link".to_string(), "Des d'enllaç".to_string());
    translations.insert("Deleted files".to_string(), "Fitxers esborrats".to_string());
    translations.insert("Cancel upload".to_string(), "Cancel·la la pujada".to_string());
    translations.insert("You don't have permission to upload or create files here".to_string(), "No teniu permisos per a pujar o crear els fitxers aquí".to_string());
    translations.insert("Nothing in here. Upload something!".to_string(), "Res per aquí. Pugeu alguna cosa!".to_string());
    translations.insert("Download".to_string(), "Baixa".to_string());
    translations.insert("Unshare".to_string(), "Deixa de compartir".to_string());
    translations.insert("Delete".to_string(), "Esborra".to_string());
    translations.insert("Upload too large".to_string(), "La pujada és massa gran".to_string());
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.".to_string(), "Els fitxers que esteu intentant pujar excedeixen la mida màxima de pujada del servidor".to_string());
    translations.insert("Files are being scanned, please wait.".to_string(), "S'estan escanejant els fitxers, espereu".to_string());
    translations.insert("Current scanning".to_string(), "Actualment escanejant".to_string());
    translations.insert("Upgrading filesystem cache...".to_string(), "Actualitzant la memòria de cau del sistema de fitxers...".to_string());
    
    translations
}

pub fn get_plural_form() -> String {
    "nplurals=2; plural=(n != 1);".to_string()
}

pub fn create_catalog() -> Catalog {
    let translations = get_translation_map();
    let plural_form = get_plural_form();
    
    Catalog::new("ca".to_string(), translations, Some(plural_form))
}