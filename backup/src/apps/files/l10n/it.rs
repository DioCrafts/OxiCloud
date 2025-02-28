use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    pub static ref TRANSLATIONS: RwLock<HashMap<&'static str, &'static str>> = RwLock::new({
        let mut m = HashMap::new();
        m.insert("Could not move %s - File with this name already exists", "Impossibile spostare %s - un file con questo nome esiste già");
        m.insert("Could not move %s", "Impossibile spostare %s");
        m.insert("File name cannot be empty.", "Il nome del file non può essere vuoto.");
        m.insert("File name must not contain \"/\". Please choose a different name.", "Il nome del file non può contenere il carattere \"/\". Scegli un nome diverso.");
        m.insert("The name %s is already used in the folder %s. Please choose a different name.", "Il nome %s è attualmente in uso nella cartella %s. Scegli un nome diverso.");
        m.insert("Not a valid source", "Non è una sorgente valida");
        m.insert("Error while downloading %s to %s", "Errore durante lo scaricamento di %s su %s");
        m.insert("Error when creating the file", "Errore durante la creazione del file");
        m.insert("Folder name cannot be empty.", "Il nome della cartella non può essere vuoto.");
        m.insert("Folder name must not contain \"/\". Please choose a different name.", "Il nome della cartella non può contenere il carattere \"/\". Scegli un nome diverso.");
        m.insert("Error when creating the folder", "Errore durante la creazione della cartella");
        m.insert("Unable to set upload directory.", "Impossibile impostare una cartella di caricamento.");
        m.insert("Invalid Token", "Token non valido");
        m.insert("No file was uploaded. Unknown error", "Nessun file è stato inviato. Errore sconosciuto");
        m.insert("There is no error, the file uploaded with success", "Non ci sono errori, il file è stato caricato correttamente");
        m.insert("The uploaded file exceeds the upload_max_filesize directive in php.ini: ", "Il file caricato supera la direttiva upload_max_filesize in php.ini:");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Il file inviato supera la direttiva MAX_FILE_SIZE specificata nel modulo HTML");
        m.insert("The uploaded file was only partially uploaded", "Il file è stato caricato solo parzialmente");
        m.insert("No file was uploaded", "Nessun file è stato caricato");
        m.insert("Missing a temporary folder", "Manca una cartella temporanea");
        m.insert("Failed to write to disk", "Scrittura su disco non riuscita");
        m.insert("Not enough storage available", "Spazio di archiviazione insufficiente");
        m.insert("Upload failed. Could not get file info.", "Caricamento non riuscito. Impossibile ottenere informazioni sul file.");
        m.insert("Upload failed. Could not find uploaded file", "Caricamento non riuscito. Impossibile trovare il file caricato.");
        m.insert("Invalid directory.", "Cartella non valida.");
        m.insert("Files", "File");
        m.insert("Unable to upload {filename} as it is a directory or has 0 bytes", "Impossibile caricare {filename} poiché è una cartella oppure ha una dimensione di 0 byte.");
        m.insert("Not enough space available", "Spazio disponibile insufficiente");
        m.insert("Upload cancelled.", "Invio annullato");
        m.insert("Could not get result from server.", "Impossibile ottenere il risultato dal server.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "Caricamento del file in corso. La chiusura della pagina annullerà il caricamento.");
        m.insert("URL cannot be empty", "L'URL non può essere vuoto.");
        m.insert("In the home folder 'Shared' is a reserved filename", "Nella cartella home 'Shared' è un nome riservato");
        m.insert("{new_name} already exists", "{new_name} esiste già");
        m.insert("Could not create file", "Impossibile creare il file");
        m.insert("Could not create folder", "Impossibile creare la cartella");
        m.insert("Share", "Condividi");
        m.insert("Delete permanently", "Elimina definitivamente");
        m.insert("Rename", "Rinomina");
        m.insert("Pending", "In corso");
        m.insert("Could not rename file", "Impossibile rinominare il file");
        m.insert("replaced {new_name} with {old_name}", "sostituito {new_name} con {old_name}");
        m.insert("undo", "annulla");
        m.insert("_%n folder_::_%n folders_", "_%n cartella_::_%n cartelle_");
        m.insert("_%n file_::_%n files_", "_%n file_::_%n file_");
        m.insert("{dirs} and {files}", "{dirs} e {files}");
        m.insert("_Uploading %n file_::_Uploading %n files_", "_Caricamento di %n file in corso_::_Caricamento di %n file in corso_");
        m.insert("'.' is an invalid file name.", "'.' non è un nome file valido.");
        m.insert("Invalid name, '\\', '/', '<', '>', ':', '\"', '|', '?' and '*' are not allowed.", "Nome non valido, '\\', '/', '<', '>', ':', '\"', '|', '?' e '*' non sono consentiti.");
        m.insert("Your storage is full, files can not be updated or synced anymore!", "Lo spazio di archiviazione è pieno, i file non possono essere più aggiornati o sincronizzati!");
        m.insert("Your storage is almost full ({usedSpacePercent}%)", "Lo spazio di archiviazione è quasi pieno ({usedSpacePercent}%)");
        m.insert("Encryption App is enabled but your keys are not initialized, please log-out and log-in again", "L'applicazione di cifratura è abilitata, ma le chiavi non sono state inizializzate, disconnettiti ed effettua nuovamente l'accesso");
        m.insert("Invalid private key for Encryption App. Please update your private key password in your personal settings to recover access to your encrypted files.", "Chiave privata non valida per l'applicazione di cifratura. Aggiorna la password della chiave privata nelle impostazioni personali per ripristinare l'accesso ai tuoi file cifrati.");
        m.insert("Encryption was disabled but your files are still encrypted. Please go to your personal settings to decrypt your files.", "La cifratura è stata disabilitata ma i tuoi file sono ancora cifrati. Vai nelle impostazioni personali per decifrare i file.");
        m.insert("Your download is being prepared. This might take some time if the files are big.", "Il tuo scaricamento è in fase di preparazione. Ciò potrebbe richiedere del tempo se i file sono grandi.");
        m.insert("Error moving file", "Errore durante lo spostamento del file");
        m.insert("Error", "Errore");
        m.insert("Name", "Nome");
        m.insert("Size", "Dimensione");
        m.insert("Modified", "Modificato");
        m.insert("Invalid folder name. Usage of 'Shared' is reserved.", "Nome della cartella non valido. L'uso di 'Shared' è riservato.");
        m.insert("%s could not be renamed", "%s non può essere rinominato");
        m.insert("Upload", "Carica");
        m.insert("File handling", "Gestione file");
        m.insert("Maximum upload size", "Dimensione massima upload");
        m.insert("max. possible: ", "numero mass.: ");
        m.insert("Needed for multi-file and folder downloads.", "Necessario per lo scaricamento di file multipli e cartelle.");
        m.insert("Enable ZIP-download", "Abilita scaricamento ZIP");
        m.insert("0 is unlimited", "0 è illimitato");
        m.insert("Maximum input size for ZIP files", "Dimensione massima per i file ZIP");
        m.insert("Save", "Salva");
        m.insert("New", "Nuovo");
        m.insert("Text file", "File di testo");
        m.insert("Folder", "Cartella");
        m.insert("From link", "Da collegamento");
        m.insert("Deleted files", "File eliminati");
        m.insert("Cancel upload", "Annulla invio");
        m.insert("You don't have permission to upload or create files here", "Qui non hai i permessi di caricare o creare file");
        m.insert("Nothing in here. Upload something!", "Non c'è niente qui. Carica qualcosa!");
        m.insert("Download", "Scarica");
        m.insert("Unshare", "Rimuovi condivisione");
        m.insert("Delete", "Elimina");
        m.insert("Upload too large", "Caricamento troppo grande");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "I file che stai provando a caricare superano la dimensione massima consentita su questo server.");
        m.insert("Files are being scanned, please wait.", "Scansione dei file in corso, attendi");
        m.insert("Current scanning", "Scansione corrente");
        m.insert("Upgrading filesystem cache...", "Aggiornamento della cache del filesystem in corso...");
        m
    });

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

// Función para obtener una traducción
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.read().unwrap().get(key).copied()
}

// Función para traducir con variables de formato
pub fn translate(key: &str, args: &[&str]) -> String {
    match get_translation(key) {
        Some(translation) => {
            let mut result = translation.to_string();
            for (i, arg) in args.iter().enumerate() {
                result = result.replace(&format!("%s", i + 1), arg);
            }
            result
        }
        None => key.to_string(),
    }
}

// Función para manejar plurales
pub fn translate_plural(singular: &str, plural: &str, count: i64) -> String {
    // Seleccionar forma singular o plural según la regla italiana
    let key = if count != 1 {
        plural
    } else {
        singular
    };
    
    // Obtener la traducción y reemplazar %n con el contador
    match get_translation(key) {
        Some(translation) => translation.replace("%n", &count.to_string()),
        None => {
            if count != 1 {
                plural.replace("%n", &count.to_string())
            } else {
                singular.replace("%n", &count.to_string())
            }
        }
    }
}