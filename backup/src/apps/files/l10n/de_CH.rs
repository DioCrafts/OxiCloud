use std::collections::HashMap;
use once_cell::sync::Lazy;
use rust_fluent::FluentBundle;

/// German (Switzerland) localization for the files app
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Could not move %s - File with this name already exists", "%s konnte nicht verschoben werden. Eine Datei mit diesem Namen existiert bereits.");
    map.insert("Could not move %s", "Konnte %s nicht verschieben");
    map.insert("File name cannot be empty.", "Der Dateiname darf nicht leer sein.");
    map.insert("Unable to set upload directory.", "Das Upload-Verzeichnis konnte nicht gesetzt werden.");
    map.insert("Invalid Token", "Ungültiges Merkmal");
    map.insert("No file was uploaded. Unknown error", "Keine Datei hochgeladen. Unbekannter Fehler");
    map.insert("There is no error, the file uploaded with success", "Es ist kein Fehler aufgetreten. Die Datei wurde erfolgreich hochgeladen.");
    map.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Die hochgeladene Datei überschreitet die upload_max_filesize Vorgabe in php.ini");
    map.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Die Datei ist grösser, als die MAX_FILE_SIZE Vorgabe erlaubt, die im HTML-Formular spezifiziert ist");
    map.insert("The uploaded file was only partially uploaded", "Die Datei konnte nur teilweise übertragen werden");
    map.insert("No file was uploaded", "Keine Datei konnte übertragen werden.");
    map.insert("Missing a temporary folder", "Kein temporärer Ordner vorhanden");
    map.insert("Failed to write to disk", "Fehler beim Schreiben auf die Festplatte");
    map.insert("Not enough storage available", "Nicht genug Speicher vorhanden.");
    map.insert("Invalid directory.", "Ungültiges Verzeichnis.");
    map.insert("Files", "Dateien");
    map.insert("Not enough space available", "Nicht genügend Speicherplatz verfügbar");
    map.insert("Upload cancelled.", "Upload abgebrochen.");
    map.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Dateiupload läuft. Wenn Sie die Seite jetzt verlassen, wird der Upload abgebrochen.");
    map.insert("{new_name} already exists", "{new_name} existiert bereits");
    map.insert("Share", "Teilen");
    map.insert("Delete permanently", "Endgültig löschen");
    map.insert("Rename", "Umbenennen");
    map.insert("Pending", "Ausstehend");
    map.insert("replaced {new_name} with {old_name}", "{old_name} wurde ersetzt durch {new_name}");
    map.insert("undo", "rückgängig machen");
    map.insert("_%n folder_::_%n folders_", "");
    map.insert("_%n folders_", "%n Ordner");
    map.insert("_%n file_::_%n files_", "");
    map.insert("_%n files_", "%n Dateien");
    map.insert("_Uploading %n file_", "%n Datei wird hochgeladen");
    map.insert("_Uploading %n files_", "%n Dateien werden hochgeladen");
    map.insert("'.' is an invalid file name.", "'.' ist kein gültiger Dateiname.");
    map.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Ungültiger Name, «\\», «/», «<», «>», «:», «\"», «|», «?» und «*» sind nicht zulässig.");
    map.insert("Your storage is full, files can not be updated or synced anymore!", "Ihr Speicher ist voll, daher können keine Dateien mehr aktualisiert oder synchronisiert werden!");
    map.insert("Your storage is almost full ({usedSpacePercent}%)", "Ihr Speicher ist fast voll ({usedSpacePercent}%)");
    map.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "Die Verschlüsselung wurde deaktiviert, jedoch sind Ihre Dateien nach wie vor verschlüsselt. Bitte gehen Sie zu Ihren persönlichen Einstellungen, um Ihre Dateien zu entschlüsseln.");
    map.insert("Your download is being prepared. This might take some time if the files are big.", "Ihr Download wird vorbereitet. Dies kann bei grösseren Dateien etwas dauern.");
    map.insert("Error", "Fehler");
    map.insert("Name", "Name");
    map.insert("Size", "Grösse");
    map.insert("Modified", "Geändert");
    map.insert("%s could not be renamed", "%s konnte nicht umbenannt werden");
    map.insert("Upload", "Hochladen");
    map.insert("File handling", "Dateibehandlung");
    map.insert("Maximum upload size", "Maximale Upload-Grösse");
    map.insert("max. possible: ", "maximal möglich:");
    map.insert("Needed for multi-file and folder downloads.", "Für Mehrfachdatei- und Ordnerdownloads benötigt:");
    map.insert("Enable ZIP-download", "ZIP-Download aktivieren");
    map.insert("0 is unlimited", "0 bedeutet unbegrenzt");
    map.insert("Maximum input size for ZIP files", "Maximale Grösse für ZIP-Dateien");
    map.insert("Save", "Speichern");
    map.insert("New", "Neu");
    map.insert("Text file", "Textdatei");
    map.insert("Folder", "Ordner");
    map.insert("From link", "Von einem Link");
    map.insert("Deleted files", "Gelöschte Dateien");
    map.insert("Cancel upload", "Upload abbrechen");
    map.insert("Nothing in here. Upload something!", "Alles leer. Laden Sie etwas hoch!");
    map.insert("Download", "Herunterladen");
    map.insert("Unshare", "Freigabe aufheben");
    map.insert("Delete", "Löschen");
    map.insert("Upload too large", "Der Upload ist zu gross");
    map.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Die Datei überschreitet die Maximalgrösse für Uploads auf diesem Server.");
    map.insert("Files are being scanned, please wait.", "Dateien werden gescannt, bitte warten.");
    map.insert("Current scanning", "Scanne");
    map.insert("Upgrading filesystem cache...", "Dateisystem-Cache wird aktualisiert ...");
    map
});

/// Defines the plural forms rule for German (Switzerland)
pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Initializes the German (Switzerland) locale bundle for the files app
pub fn init_bundle() -> FluentBundle {
    let mut bundle = FluentBundle::new();
    for (key, value) in TRANSLATIONS.iter() {
        bundle.add_message(key, value);
    }
    bundle.set_plural_rule(PLURAL_FORMS);
    bundle
}