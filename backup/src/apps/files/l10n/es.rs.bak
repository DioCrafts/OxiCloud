use std::collections::HashMap;
use once_cell::sync::Lazy;
use rust_i18n::t;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Could not move %s - File with this name already exists", "No se pudo mover %s - Ya existe un archivo con ese nombre.");
    m.insert("Could not move %s", "No se pudo mover %s");
    m.insert("File name cannot be empty.", "El nombre de archivo no puede estar vacío.");
    m.insert("File name must not contain \"/\". Please choose a different name.", "El nombre del archivo, NO puede contener el simbolo\"/\", por favor elija un nombre diferente.");
    m.insert("The name %s is already used in the folder %s. Please choose a different name.", "El nombre %s ya está en uso por la carpeta %s. Por favor elija uno diferente.");
    m.insert("Not a valid source", "No es un origen válido");
    m.insert("Error while downloading %s to %s", "Error mientras se descargaba %s a %s");
    m.insert("Error when creating the file", "Error al crear el archivo");
    m.insert("Folder name cannot be empty.", "El nombre de la carpeta no puede estar vacío.");
    m.insert("Folder name must not contain \"/\". Please choose a different name.", "El nombre de la carpeta, NO puede contener el simbolo\"/\", por favor elija un nombre diferente.");
    m.insert("Error when creating the folder", "Error al crear la carpeta.");
    m.insert("Unable to set upload directory.", "Incapaz de crear directorio de subida.");
    m.insert("Invalid Token", "Token Inválido");
    m.insert("No file was uploaded. Unknown error", "No se subió ningún archivo. Error desconocido");
    m.insert("There is no error, the file uploaded with success", "No hubo ningún problema, el archivo se subió con éxito");
    m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "El archivo subido sobrepasa la directiva 'upload_max_filesize' en php.ini:");
    m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "El archivo subido sobrepasa la directiva 'MAX_FILE_SIZE' especificada en el formulario HTML");
    m.insert("The uploaded file was only partially uploaded", "El archivo subido fue sólo subido parcialmente");
    m.insert("No file was uploaded", "No se subió ningún archivo");
    m.insert("Missing a temporary folder", "Falta la carpeta temporal");
    m.insert("Failed to write to disk", "Falló al escribir al disco");
    m.insert("Not enough storage available", "No hay suficiente espacio disponible");
    m.insert("Upload failed. Could not get file info.", "Actualización fallida. No se pudo obtener información del archivo.");
    m.insert("Upload failed. Could not find uploaded file", "Actualización fallida. No se pudo encontrar el archivo subido");
    m.insert("Invalid directory.", "Directorio inválido.");
    m.insert("Files", "Archivos");
    m.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "No ha sido posible subir {filename} porque es un directorio o tiene 0 bytes");
    m.insert("Not enough space available", "No hay suficiente espacio disponible");
    m.insert("Upload cancelled.", "Subida cancelada.");
    m.insert("Could not get result from server.", "No se pudo obtener respuesta del servidor.");
    m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "La subida del archivo está en proceso. Si sale de la página ahora, la subida será cancelada.");
    m.insert("URL cannot be empty", "La dirección URL no puede estar vacía");
    m.insert("In the home folder 'Shared' is a reserved filename", "En la carpeta de inicio, 'Shared' es un nombre reservado");
    m.insert("{new_name} already exists", "{new_name} ya existe");
    m.insert("Could not create file", "No se pudo crear el archivo");
    m.insert("Could not create folder", "No se pudo crear la carpeta");
    m.insert("Share", "Compartir");
    m.insert("Delete permanently", "Eliminar permanentemente");
    m.insert("Rename", "Renombrar");
    m.insert("Pending", "Pendiente");
    m.insert("Could not rename file", "No se pudo renombrar el archivo");
    m.insert("replaced {new_name} with {old_name}", "reemplazado {new_name} con {old_name}");
    m.insert("undo", "deshacer");
    m.insert("_%n folder_::_%n folders_", "%n carpeta|%n carpetas");
    m.insert("_%n file_::_%n files_", "%n archivo|%n archivos");
    m.insert("{dirs} and {files}", "{dirs} y {files}");
    m.insert("_Uploading %n file_::_Uploading %n files_", "Subiendo %n archivo|Subiendo %n archivos");
    m.insert("'.' is an invalid file name.", "'.' no es un nombre de archivo válido.");
    m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Nombre inválido, los caracteres \"\\\", \"/\", \"<\", \">\", \":\", \"\", \"|\" \"?\" y \"*\" no están permitidos ");
    m.insert("Your storage is full, files can not be updated or synced anymore!", "Su almacenamiento está lleno, ¡los archivos no se actualizarán ni sincronizarán más!");
    m.insert("Your storage is almost full ({usedSpacePercent}%)", "Su almacenamiento está casi lleno ({usedSpacePercent}%)");
    m.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "La app de crifrado está habilitada pero tus claves no han sido inicializadas, por favor, cierra la sesión y vuelva a iniciarla de nuevo.");
    m.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "La clave privada no es válida para la app de cifrado. Por favor, actualiza la contraseña de tu clave privada en tus ajustes personales para recuperar el acceso a tus archivos cifrados.");
    m.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "El cifrado ha sido deshabilitado pero tus archivos permanecen cifrados. Por favor, ve a tus ajustes personales para descifrar tus archivos.");
    m.insert("Your download is being prepared. This might take some time if the files are big.", "Su descarga está siendo preparada. Esto podría tardar algo de tiempo si los archivos son grandes.");
    m.insert("Error moving file", "Error moviendo archivo");
    m.insert("Error", "Error");
    m.insert("Name", "Nombre");
    m.insert("Size", "Tamaño");
    m.insert("Modified", "Modificado");
    m.insert("Invalid folder name. Usage of 'Shared' is reserved.", "Nombre de carpeta inválido. El uso de \"Shared\" esta reservado.");
    m.insert("%s could not be renamed", "%s no pudo ser renombrado");
    m.insert("Upload", "Subir");
    m.insert("File handling", "Administración de archivos");
    m.insert("Maximum upload size", "Tamaño máximo de subida");
    m.insert("max. possible: ", "máx. posible:");
    m.insert("Needed for multi-file and folder downloads.", "Necesario para multi-archivo y descarga de carpetas");
    m.insert("Enable ZIP-download", "Habilitar descarga en ZIP");
    m.insert("0 is unlimited", "0 significa ilimitado");
    m.insert("Maximum input size for ZIP files", "Tamaño máximo para archivos ZIP de entrada");
    m.insert("Save", "Guardar");
    m.insert("New", "Nuevo");
    m.insert("Text file", "Archivo de texto");
    m.insert("Folder", "Carpeta");
    m.insert("From link", "Desde enlace");
    m.insert("Deleted files", "Archivos eliminados");
    m.insert("Cancel upload", "Cancelar subida");
    m.insert("You don't have permission to upload or create files here", "No tienes permisos para subir o crear archivos aquí.");
    m.insert("Nothing in here. Upload something!", "No hay nada aquí. ¡Suba algo!");
    m.insert("Download", "Descargar");
    m.insert("Unshare", "Dejar de compartir");
    m.insert("Delete", "Eliminar");
    m.insert("Upload too large", "Subida demasido grande");
    m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Los archivos que estás intentando subir sobrepasan el tamaño máximo permitido en este servidor.");
    m.insert("Files are being scanned, please wait.", "Los archivos están siendo escaneados,  por favor espere.");
    m.insert("Current scanning", "Escaneo actual");
    m.insert("Upgrading filesystem cache...", "Actualizando caché del sistema de archivos...");
    m
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn register_translation() {
    rust_i18n::set_locale("es");
}

#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        crate::i18n::TRANSLATIONS.get($key).unwrap_or(&$key)
    };
    ($key:expr, $($args:expr),*) => {{
        let text = crate::i18n::TRANSLATIONS.get($key).unwrap_or(&$key);
        format!($text, $($args),*)
    }};
}

#[macro_export]
macro_rules! tr_n {
    ($key:expr, $count:expr) => {{
        let key = if $count == 1 {
            concat!("_", $key, "_::_", $key, "s_")
        } else {
            concat!("_", $key, "_::_", $key, "s_")
        };
        let value = crate::i18n::TRANSLATIONS.get(key).unwrap_or(&key);
        let parts: Vec<&str> = value.split('|').collect();
        if $count == 1 && parts.len() > 0 {
            parts[0].replace("%n", &$count.to_string())
        } else if parts.len() > 1 {
            parts[1].replace("%n", &$count.to_string())
        } else {
            value.replace("%n", &$count.to_string())
        }
    }};
}