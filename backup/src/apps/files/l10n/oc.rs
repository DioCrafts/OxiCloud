use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("There is no error, the file uploaded with success", "Amontcargament capitat, pas d'errors");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Lo fichièr amontcargat es mai gròs que la directiva «MAX_FILE_SIZE» especifiada dins lo formulari HTML");
        m.insert("The uploaded file was only partially uploaded", "Lo fichièr foguèt pas completament amontcargat");
        m.insert("No file was uploaded", "Cap de fichièrs son estats amontcargats");
        m.insert("Missing a temporary folder", "Un dorsièr temporari manca");
        m.insert("Failed to write to disk", "L'escriptura sul disc a fracassat");
        m.insert("Files", "Fichièrs");
        m.insert("Upload cancelled.", "Amontcargar anullat.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Un amontcargar es a se far. Daissar aquesta pagina ara tamparà lo cargament. ");
        m.insert("Share", "Parteja");
        m.insert("Rename", "Torna nomenar");
        m.insert("Pending", "Al esperar");
        m.insert("undo", "defar");
        m.insert("_%n folder_::_%n folders_", "");
        m.insert("_%n file_::_%n files_", "");
        m.insert("_Uploading %n file_::_Uploading %n files_", "");
        m.insert("Error", "Error");
        m.insert("Name", "Nom");
        m.insert("Size", "Talha");
        m.insert("Modified", "Modificat");
        m.insert("Upload", "Amontcarga");
        m.insert("File handling", "Manejament de fichièr");
        m.insert("Maximum upload size", "Talha maximum d'amontcargament");
        m.insert("max. possible: ", "max. possible: ");
        m.insert("Needed for multi-file and folder downloads.", "Requesit per avalcargar gropat de fichièrs e dorsièr");
        m.insert("Enable ZIP-download", "Activa l'avalcargament de ZIP");
        m.insert("0 is unlimited", "0 es pas limitat");
        m.insert("Maximum input size for ZIP files", "Talha maximum de dintrada per fichièrs ZIP");
        m.insert("Save", "Enregistra");
        m.insert("New", "Nòu");
        m.insert("Text file", "Fichièr de tèxte");
        m.insert("Folder", "Dorsièr");
        m.insert("Cancel upload", " Anulla l'amontcargar");
        m.insert("Nothing in here. Upload something!", "Pas res dedins. Amontcarga qualquaren");
        m.insert("Download", "Avalcarga");
        m.insert("Unshare", "Pas partejador");
        m.insert("Delete", "Escafa");
        m.insert("Upload too large", "Amontcargament tròp gròs");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Los fichièrs que sias a amontcargar son tròp pesucs per la talha maxi pel servidor.");
        m.insert("Files are being scanned, please wait.", "Los fiichièrs son a èsser explorats, ");
        m.insert("Current scanning", "Exploracion en cors");
        m
    };
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n > 1);"
}

pub fn initialize_translations() {
    // Este es un punto de entrada para inicializar traducciones
    // que podría ser llamado al inicio de la aplicación
}

pub fn translate(key: &str) -> &str {
    TRANSLATIONS.get(key).unwrap_or(&key)
}

pub fn translate_plural(singular: &str, plural: &str, count: i64) -> String {
    // Implementación básica de la lógica de pluralización para el idioma occitano
    if count > 1 {
        String::from(TRANSLATIONS.get(plural).unwrap_or(&plural))
    } else {
        String::from(TRANSLATIONS.get(singular).unwrap_or(&singular))
    }
}