use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "Non se moveu %s - Xa existe un ficheiro con ese nome.");
        m.insert("Could not move %s", "Non foi posíbel mover %s");
        m.insert("File name cannot be empty.", "O nome de ficheiro non pode estar baleiro");
        m.insert("File name must not contain \"/\". Please choose a different name.", "O nome do ficheiro non pode conter «/». Escolla outro nome.");
        m.insert("The name %s is already used in the folder %s. Please choose a different name.", "Xa existe o nome %s no cartafol %s. Escolla outro nome.");
        m.insert("Not a valid source", "Esta orixe non é correcta");
        m.insert("Error while downloading %s to %s", "Produciuse un erro ao descargar %s en %s");
        m.insert("Error when creating the file", "Produciuse un erro ao crear o ficheiro");
        m.insert("Folder name cannot be empty.", "O nome de cartafol non pode estar baleiro.");
        m.insert("Folder name must not contain \"/\". Please choose a different name.", "O nome do cartafol non pode conter «/». Escolla outro nome.");
        m.insert("Error when creating the folder", "Produciuse un erro ao crear o cartafol");
        m.insert("Unable to set upload directory.", "Non é posíbel configurar o directorio de envíos.");
        m.insert("Invalid Token", "Marca incorrecta");
        m.insert("No file was uploaded. Unknown error", "Non se enviou ningún ficheiro. Produciuse un erro descoñecido.");
        m.insert("There is no error, the file uploaded with success", "Non houbo erros, o ficheiro enviouse correctamente");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "O ficheiro enviado excede a directiva indicada por upload_max_filesize de php.ini:");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "O ficheiro enviado excede da directiva MAX_FILE_SIZE especificada no formulario HTML");
        m.insert("The uploaded file was only partially uploaded", "O ficheiro so foi parcialmente enviado");
        m.insert("No file was uploaded", "Non se enviou ningún ficheiro");
        m.insert("Missing a temporary folder", "Falta o cartafol temporal");
        m.insert("Failed to write to disk", "Produciuse un erro ao escribir no disco");
        m.insert("Not enough storage available", "Non hai espazo de almacenamento abondo");
        m.insert("Upload failed. Could not get file info.", "O envío fracasou. Non foi posíbel obter información do ficheiro.");
        m.insert("Upload failed. Could not find uploaded file", "O envío fracasou. Non foi posíbel atopar o ficheiro enviado");
        m.insert("Invalid directory.", "O directorio é incorrecto.");
        m.insert("Files", "Ficheiros");
        m.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Non é posíbel enviar {filename}, xa que ou é un directorio ou ten 0 bytes");
        m.insert("Not enough space available", "O espazo dispoñíbel é insuficiente");
        m.insert("Upload cancelled.", "Envío cancelado.");
        m.insert("Could not get result from server.", "Non foi posíbel obter o resultado do servidor.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "O envío do ficheiro está en proceso. Saír agora da páxina cancelará o envío.");
        m.insert("URL cannot be empty", "O URL non pode quedar en branco.");
        m.insert("In the home folder 'Shared' is a reserved filename", "«Shared» dentro do cartafol persoal é un nome reservado");
        m.insert("{new_name} already exists", "Xa existe un {new_name}");
        m.insert("Could not create file", "Non foi posíbel crear o ficheiro");
        m.insert("Could not create folder", "Non foi posíbel crear o cartafol");
        m.insert("Share", "Compartir");
        m.insert("Delete permanently", "Eliminar permanentemente");
        m.insert("Rename", "Renomear");
        m.insert("Pending", "Pendentes");
        m.insert("Could not rename file", "Non foi posíbel renomear o ficheiro");
        m.insert("replaced {new_name} with {old_name}", "substituír {new_name} por {old_name}");
        m.insert("undo", "desfacer");
        m.insert("_%n folder_::_%n folders_", "%n cartafol::%n cartafoles");
        m.insert("_%n file_::_%n files_", "%n ficheiro::%n ficheiros");
        m.insert("{dirs} and {files}", "{dirs} e {files}");
        m.insert("_Uploading %n file_::_Uploading %n files_", "Cargando %n ficheiro::Cargando %n ficheiros");
        m.insert("'.' is an invalid file name.", "«.» é un nome de ficheiro incorrecto");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Nome incorrecto, non se permite «\\», «/», «<», «>», «:», «\"», «|», «?» e «*».");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "O seu espazo de almacenamento está cheo, non é posíbel actualizar ou sincronizar máis os ficheiros!");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "O seu espazo de almacenamento está case cheo ({usedSpacePercent}%)");
        m.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "O aplicativo de cifrado está activado, mais as chaves non foron inicializadas, saia da sesión e volva a acceder de novo");
        m.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "A chave privada para o aplicativo de cifrado non é correcta. Actualice o contrasinal da súa chave privada nos seus axustes persoais para recuperar o acceso aos seus ficheiros cifrados.");
        m.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "O cifrado foi desactivado, mais os ficheiros están cifrados. Vaia á configuración persoal para descifrar os ficheiros.");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "Está a prepararse a súa descarga. Isto pode levar bastante tempo se os ficheiros son grandes.");
        m.insert("Error moving file", "Produciuse un erro ao mover o ficheiro");
        m.insert("Error", "Erro");
        m.insert("Name", "Nome");
        m.insert("Size", "Tamaño");
        m.insert("Modified", "Modificado");
        m.insert("Invalid folder name. Usage of 'Shared' is reserved.", "Nome de cartafol non válido. O uso de «Shared» está reservado.");
        m.insert("%s could not be renamed", "%s non pode cambiar de nome");
        m.insert("Upload", "Enviar");
        m.insert("File handling", "Manexo de ficheiro");
        m.insert("Maximum upload size", "Tamaño máximo do envío");
        m.insert("max. possible: ", "máx. posíbel: ");
        m.insert("Needed for multi-file and folder downloads.", "Precísase para a descarga de varios ficheiros e cartafoles.");
        m.insert("Enable ZIP-download", "Activar a descarga ZIP");
        m.insert("0 is unlimited", "0 significa ilimitado");
        m.insert("Maximum input size for ZIP files", "Tamaño máximo de descarga para os ficheiros ZIP");
        m.insert("Save", "Gardar");
        m.insert("New", "Novo");
        m.insert("Text file", "Ficheiro de texto");
        m.insert("Folder", "Cartafol");
        m.insert("From link", "Desde a ligazón");
        m.insert("Deleted files", "Ficheiros eliminados");
        m.insert("Cancel upload", "Cancelar o envío");
        m.insert("You don't have permission to upload or create files here", "Non ten permisos para enviar ou crear ficheiros aquí.");
        m.insert("Nothing in here. Upload something!", "Aquí non hai nada. Envíe algo.");
        m.insert("Download", "Descargar");
        m.insert("Unshare", "Deixar de compartir");
        m.insert("Delete", "Eliminar");
        m.insert("Upload too large", "Envío demasiado grande");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Os ficheiros que tenta enviar exceden do tamaño máximo permitido neste servidor");
        m.insert("Files are being scanned, please wait.", "Estanse analizando os ficheiros. Agarde.");
        m.insert("Current scanning", "Análise actual");
        m.insert("Upgrading filesystem cache...", "Anovando a caché do sistema de ficheiros...");
        m
    };
}

pub fn plural_forms(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

// Función para obtener una traducción
pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

// Función para obtener una traducción con formato
pub fn get_translation_fmt(key: &str, args: &[&str]) -> String {
    let translation = get_translation(key);
    let mut result = translation.to_string();
    
    for (i, arg) in args.iter().enumerate() {
        result = result.replace(&format!("%s", i + 1), arg);
    }
    
    result
}

// Función para manejar las traducciones plurales
pub fn get_plural_translation(singular_key: &str, plural_key: &str, n: usize) -> String {
    let key = if plural_forms(n) == 0 { singular_key } else { plural_key };
    let translation = get_translation(key);
    translation.replace("%n", &n.to_string())
}