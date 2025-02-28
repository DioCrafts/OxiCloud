use std::collections::HashMap;
use phf::phf_map;

// German language translations for the Files app
pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Could not move %s - File with this name already exists" => "Konnte %s nicht verschieben. Eine Datei mit diesem Namen existiert bereits",
    "Could not move %s" => "Konnte %s nicht verschieben",
    "File name cannot be empty." => "Der Dateiname darf nicht leer sein.",
    "File name must not contain \"/\". Please choose a different name." => "Der Dateiname darf kein \"/\" enthalten. Bitte wähle einen anderen Namen.",
    "The name %s is already used in the folder %s. Please choose a different name." => "Der Name %s wird bereits im Ordner %s benutzt. Bitte wähle  einen anderen Namen.",
    "Not a valid source" => "Keine gültige Quelle",
    "Error while downloading %s to %s" => "Fehler beim Herunterladen von %s nach %s",
    "Error when creating the file" => "Fehler beim Erstellen der Datei",
    "Folder name cannot be empty." => "Der Ordner-Name darf nicht leer sein.",
    "Folder name must not contain \"/\". Please choose a different name." => "Der Ordner-Name darf kein \"/\" enthalten. Bitte wähle einen anderen Namen.",
    "Error when creating the folder" => "Fehler beim Erstellen des Ordners",
    "Unable to set upload directory." => "Das Upload-Verzeichnis konnte nicht gesetzt werden.",
    "Invalid Token" => "Ungültiges Merkmal",
    "No file was uploaded. Unknown error" => "Keine Datei hochgeladen. Unbekannter Fehler",
    "There is no error, the file uploaded with success" => "Es ist kein Fehler aufgetreten. Die Datei wurde erfolgreich hochgeladen.",
    "The uploaded file exceeds the upload_max_filesize directive in php.ini: " => "Die hochgeladene Datei überschreitet die upload_max_filesize Vorgabe in php.ini",
    "The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form" => "Die Datei ist größer, als die MAX_FILE_SIZE Direktive erlaubt, die im HTML-Formular spezifiziert ist",
    "The uploaded file was only partially uploaded" => "Die Datei konnte nur teilweise übertragen werden",
    "No file was uploaded" => "Keine Datei konnte übertragen werden.",
    "Missing a temporary folder" => "Kein temporärer Ordner vorhanden",
    "Failed to write to disk" => "Fehler beim Schreiben auf die Festplatte",
    "Not enough storage available" => "Nicht genug Speicher vorhanden.",
    "Upload failed. Could not get file info." => "Hochladen fehlgeschlagen. Dateiinformationen konnten nicht abgerufen werden.",
    "Upload failed. Could not find uploaded file" => "Hochladen fehlgeschlagen. Hochgeladene Datei konnte nicht gefunden werden.",
    "Invalid directory." => "Ungültiges Verzeichnis.",
    "Files" => "Dateien",
    "Unable to upload {filename} as it is a directory or has 0 bytes" => "Die Datei {filename} kann nicht hochgeladen werden, da sie entweder ein Verzeichnis oder 0 Bytes groß ist",
    "Not enough space available" => "Nicht genug Speicherplatz verfügbar",
    "Upload cancelled." => "Upload abgebrochen.",
    "Could not get result from server." => "Ergebnis konnte nicht vom Server abgerufen werden.",
    "File upload is in progress. Leaving the page now will cancel the upload." => "Dateiupload läuft. Wenn Du die Seite jetzt verlässt, wird der Upload abgebrochen.",
    "URL cannot be empty" => "Die URL darf nicht leer sein",
    "In the home folder 'Shared' is a reserved filename" => "Das Benutzerverzeichnis 'Shared' ist ein reservierter Dateiname",
    "{new_name} already exists" => "{new_name} existiert bereits",
    "Could not create file" => "Die Datei konnte nicht erstellt werden",
    "Could not create folder" => "Der Ordner konnte nicht erstellt werden",
    "Share" => "Teilen",
    "Delete permanently" => "Endgültig löschen",
    "Rename" => "Umbenennen",
    "Pending" => "Ausstehend",
    "Could not rename file" => "Die Datei konnte nicht umbenannt werden",
    "replaced {new_name} with {old_name}" => "{old_name} ersetzt durch {new_name}",
    "undo" => "rückgängig machen",
    "{dirs} and {files}" => "{dirs} und {files}",
    "'.' is an invalid file name." => "'.' ist kein gültiger Dateiname.",
    "Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed." => "Ungültiger Name, '\\', '/', '<', '>', ':', '\"', '|', '?' und '*' sind nicht zulässig.",
    "Your storage is full, files can not be updated or synced anymore!" => "Dein Speicher ist voll, daher können keine Dateien mehr aktualisiert oder synchronisiert werden!",
    "Your storage is almost full ({usedSpacePercent}%)" => "Dein Speicher ist fast voll ({usedSpacePercent}%)",
    "Encryption App is enabled but your keys are not initialized, please log-out and log-in again" => "Die Verschlüsselung-App ist aktiviert, aber Deine Schlüssel sind nicht initialisiert. Bitte melden Dich nochmals ab und wieder an.",
    "Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files." => "Ungültiger privater Schlüssel für die Verschlüsselung-App. Bitte aktualisiere Dein privates Schlüssel-Passwort, um den Zugriff auf Deine verschlüsselten Dateien wiederherzustellen.",
    "Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files." => "Die Verschlüsselung wurde deaktiviert, jedoch sind Deine Dateien nach wie vor verschlüsselt. Bitte gehe zu Deinen persönlichen Einstellungen, um Deine Dateien zu entschlüsseln.",
    "Your download is being prepared. This might take some time if the files are big." => "Dein Download wird vorbereitet. Dies kann bei größeren Dateien etwas dauern.",
    "Error moving file" => "Fehler beim Verschieben der Datei",
    "Error" => "Fehler",
    "Name" => "Name",
    "Size" => "Größe",
    "Modified" => "Geändert",
    "Invalid folder name. Usage of 'Shared' is reserved." => "Ungültiger Verzeichnisname. Die Nutzung von 'Shared' ist reserviert.",
    "%s could not be renamed" => "%s konnte nicht umbenannt werden",
    "Upload" => "Hochladen",
    "File handling" => "Dateibehandlung",
    "Maximum upload size" => "Maximale Upload-Größe",
    "max. possible: " => "maximal möglich:",
    "Needed for multi-file and folder downloads." => "Für Mehrfachdatei- und Ordnerdownloads benötigt:",
    "Enable ZIP-download" => "ZIP-Download aktivieren",
    "0 is unlimited" => "0 bedeutet unbegrenzt",
    "Maximum input size for ZIP files" => "Maximale Größe für ZIP-Dateien",
    "Save" => "Speichern",
    "New" => "Neu",
    "Text file" => "Textdatei",
    "Folder" => "Ordner",
    "From link" => "Von einem Link",
    "Deleted files" => "Gelöschte Dateien",
    "Cancel upload" => "Upload abbrechen",
    "You don't have permission to upload or create files here" => "Du besitzt hier keine Berechtigung, um Dateien hochzuladen oder zu erstellen",
    "Nothing in here. Upload something!" => "Alles leer. Lade etwas hoch!",
    "Download" => "Herunterladen",
    "Unshare" => "Freigabe aufheben",
    "Delete" => "Löschen",
    "Upload too large" => "Der Upload ist zu groß",
    "The files you are trying to upload exceed the maximum size for file uploads on this server." => "Die Datei überschreitet die Maximalgröße für Uploads auf diesem Server.",
    "Files are being scanned, please wait." => "Dateien werden gescannt, bitte warten.",
    "Current scanning" => "Scanne",
    "Upgrading filesystem cache..." => "Dateisystem-Cache wird aktualisiert ...",
};

// Plural forms definition for German language
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

// Plural translations for specific strings
pub fn plural(key: &str, count: usize) -> String {
    match key {
        "_%n folder_::_%n folders_" => {
            let forms = ["%n Ordner", "%n Ordner"];
            forms[get_plural_form(count)].replace("%n", &count.to_string())
        },
        "_%n file_::_%n files_" => {
            let forms = ["%n Datei", "%n Dateien"];
            forms[get_plural_form(count)].replace("%n", &count.to_string())
        },
        "_Uploading %n file_::_Uploading %n files_" => {
            let forms = ["%n Datei wird hochgeladen", "%n Dateien werden hochgeladen"];
            forms[get_plural_form(count)].replace("%n", &count.to_string())
        },
        _ => format!("Unknown plural key: {}", key),
    }
}

// Function to get a translated string
pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}

// Function to format a translation with parameters
pub fn format_translation(key: &str, params: &[&str]) -> String {
    let translation = get_translation(key);
    if params.is_empty() {
        return translation.to_string();
    }
    
    let mut result = translation.to_string();
    for (i, param) in params.iter().enumerate() {
        result = result.replace(&format!("%s", if i > 0 { i.to_string() } else { String::new() }), param);
    }
    result
}