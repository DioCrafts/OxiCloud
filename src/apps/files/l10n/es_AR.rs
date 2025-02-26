use std::collections::HashMap;
use fluent::FluentResource;

// Definición de las traducciones para es_AR
pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Could not move %s - File with this name already exists".to_string(), "No se pudo mover %s - Un archivo con este nombre ya existe".to_string());
    translations.insert("Could not move %s".to_string(), "No se pudo mover %s ".to_string());
    translations.insert("File name cannot be empty.".to_string(), "El nombre del archivo no puede quedar vacío.".to_string());
    translations.insert("Unable to set upload directory.".to_string(), "No fue posible crear el directorio de subida.".to_string());
    translations.insert("Invalid Token".to_string(), "Token Inválido".to_string());
    translations.insert("No file was uploaded. Unknown error".to_string(), "El archivo no fue subido. Error desconocido".to_string());
    translations.insert("There is no error, the file uploaded with success".to_string(), "No hay errores, el archivo fue subido con éxito".to_string());
    translations.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ".to_string(), "El archivo que intentás subir excede el tamaño definido por upload_max_filesize en el php.ini:".to_string());
    translations.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form".to_string(), "El archivo subido sobrepasa el valor MAX_FILE_SIZE especificada en el formulario HTML".to_string());
    translations.insert("The uploaded file was only partially uploaded".to_string(), "El archivo fue subido parcialmente".to_string());
    translations.insert("No file was uploaded".to_string(), "No se subió ningún archivo ".to_string());
    translations.insert("Missing a temporary folder".to_string(), "Falta un directorio temporal".to_string());
    translations.insert("Failed to write to disk".to_string(), "Error al escribir en el disco".to_string());
    translations.insert("Not enough storage available".to_string(), "No hay suficiente almacenamiento".to_string());
    translations.insert("Invalid directory.".to_string(), "Directorio inválido.".to_string());
    translations.insert("Files".to_string(), "Archivos".to_string());
    translations.insert("Not enough space available".to_string(), "No hay suficiente espacio disponible".to_string());
    translations.insert("Upload cancelled.".to_string(), "La subida fue cancelada".to_string());
    translations.insert("File upload is in progress. Leaving the page now will cancel the upload.".to_string(), "La subida del archivo está en proceso. Si salís de la página ahora, la subida se cancelará.".to_string());
    translations.insert("{new_name} already exists".to_string(), "{new_name} ya existe".to_string());
    translations.insert("Share".to_string(), "Compartir".to_string());
    translations.insert("Delete permanently".to_string(), "Borrar permanentemente".to_string());
    translations.insert("Rename".to_string(), "Cambiar nombre".to_string());
    translations.insert("Pending".to_string(), "Pendientes".to_string());
    translations.insert("replaced {new_name} with {old_name}".to_string(), "se reemplazó {new_name} con {old_name}".to_string());
    translations.insert("undo".to_string(), "deshacer".to_string());
    translations.insert("_%n folder_::_%n folders_".to_string(), "%n carpeta|%n carpetas".to_string());
    translations.insert("_%n file_::_%n files_".to_string(), "%n archivo|%n archivos".to_string());
    translations.insert("{dirs} and {files}".to_string(), "{carpetas} y {archivos}".to_string());
    translations.insert("_Uploading %n file_::_Uploading %n files_".to_string(), "Subiendo %n archivo|Subiendo %n archivos".to_string());
    translations.insert("'.' is an invalid file name.".to_string(), "'.' es un nombre de archivo inválido.".to_string());
    translations.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.".to_string(), "Nombre invalido, '\\', '/', '<', '>', ':', '\"', '|', '?' y '*' no están permitidos.".to_string());
    translations.insert("Your storage is full, files can not be updated or synced anymore!".to_string(), "El almacenamiento está lleno, los archivos no se pueden seguir actualizando ni sincronizando".to_string());
    translations.insert("Your storage is almost full ({usedSpacePercent}%)".to_string(), "El almacenamiento está casi lleno ({usedSpacePercent}%)".to_string());
    translations.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.".to_string(), "El proceso de cifrado se ha desactivado, pero los archivos aún están encriptados. Por favor, vaya a la configuración personal para descifrar los archivos.".to_string());
    translations.insert("Your download is being prepared. This might take some time if the files are big.".to_string(), "Tu descarga se está preparando. Esto puede demorar si los archivos son muy grandes.".to_string());
    translations.insert("Error".to_string(), "Error".to_string());
    translations.insert("Name".to_string(), "Nombre".to_string());
    translations.insert("Size".to_string(), "Tamaño".to_string());
    translations.insert("Modified".to_string(), "Modificado".to_string());
    translations.insert("%s could not be renamed".to_string(), "No se pudo renombrar %s".to_string());
    translations.insert("Upload".to_string(), "Subir".to_string());
    translations.insert("File handling".to_string(), "Tratamiento de archivos".to_string());
    translations.insert("Maximum upload size".to_string(), "Tamaño máximo de subida".to_string());
    translations.insert("max. possible: ".to_string(), "máx. posible:".to_string());
    translations.insert("Needed for multi-file and folder downloads.".to_string(), "Es necesario para descargas multi-archivo y de directorios.".to_string());
    translations.insert("Enable ZIP-download".to_string(), "Habilitar descarga en formato ZIP".to_string());
    translations.insert("0 is unlimited".to_string(), "0 significa ilimitado".to_string());
    translations.insert("Maximum input size for ZIP files".to_string(), "Tamaño máximo para archivos ZIP de entrada".to_string());
    translations.insert("Save".to_string(), "Guardar".to_string());
    translations.insert("New".to_string(), "Nuevo".to_string());
    translations.insert("Text file".to_string(), "Archivo de texto".to_string());
    translations.insert("Folder".to_string(), "Carpeta".to_string());
    translations.insert("From link".to_string(), "Desde enlace".to_string());
    translations.insert("Deleted files".to_string(), "Archivos borrados".to_string());
    translations.insert("Cancel upload".to_string(), "Cancelar subida".to_string());
    translations.insert("Nothing in here. Upload something!".to_string(), "No hay nada. ¡Subí contenido!".to_string());
    translations.insert("Download".to_string(), "Descargar".to_string());
    translations.insert("Unshare".to_string(), "Dejar de compartir".to_string());
    translations.insert("Delete".to_string(), "Borrar".to_string());
    translations.insert("Upload too large".to_string(), "El tamaño del archivo que querés subir es demasiado grande".to_string());
    translations.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.".to_string(), "Los archivos que intentás subir sobrepasan el tamaño máximo ".to_string());
    translations.insert("Files are being scanned, please wait.".to_string(), "Se están escaneando los archivos, por favor esperá.".to_string());
    translations.insert("Current scanning".to_string(), "Escaneo actual".to_string());
    translations.insert("Upgrading filesystem cache...".to_string(), "Actualizando el cache del sistema de archivos".to_string());
    
    translations
}

pub fn get_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Función para obtener un recurso Fluent que se pueda usar con la biblioteca fluent
pub fn get_fluent_resource() -> Result<FluentResource, fluent::FluentError> {
    let mut ftl_string = String::new();
    
    for (key, value) in get_translations() {
        // Si contiene plural, hacemos un procesamiento especial
        if value.contains('|') {
            let parts: Vec<&str> = value.split('|').collect();
            if parts.len() == 2 {
                ftl_string.push_str(&format!("{} = 
    {{ $n == 1 }} {} 
    *[other] {}
", key, parts[0], parts[1]));
            }
        } else {
            ftl_string.push_str(&format!("{} = {}\n", key, value));
        }
    }
    
    FluentResource::try_new(ftl_string)
}