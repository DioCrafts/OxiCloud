use std::collections::HashMap;
use rust_fluent::types::{FluentNumber, PluralCategory, NumberOptions};
use rust_fluent::resolver::PluralResolver;

/// Italian localization module
pub struct ItalianLocalization {
    translations: HashMap<&'static str, &'static str>,
    plural_form: &'static str,
}

impl Default for ItalianLocalization {
    fn default() -> Self {
        let mut translations = HashMap::new();
        
        translations.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "L'applicazione \"%s\" non può essere installata poiché non è compatibile con questa versione di ownCloud.");
        translations.insert("No app name specified", "Il nome dell'applicazione non è specificato");
        translations.insert("Help", "Aiuto");
        translations.insert("Personal", "Personale");
        translations.insert("Settings", "Impostazioni");
        translations.insert("Users", "Utenti");
        translations.insert("Admin", "Admin");
        translations.insert("Failed to upgrade \"%s\".", "Aggiornamento non riuscito \"%s\".");
        translations.insert("Unknown filetype", "Tipo di file sconosciuto");
        translations.insert("Invalid image", "Immagine non valida");
        translations.insert("web services under your control", "servizi web nelle tue mani");
        translations.insert("cannot open \"%s\"", "impossibile aprire \"%s\"");
        translations.insert("ZIP download is turned off.", "Lo scaricamento in formato ZIP è stato disabilitato.");
        translations.insert("Files need to be downloaded one by one.", "I file devono essere scaricati uno alla volta.");
        translations.insert("Back to Files", "Torna ai file");
        translations.insert("Selected files too large to generate zip file.", "I  file selezionati sono troppo grandi per generare un file zip.");
        translations.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Scarica i file in blocchi più piccoli, separatamente o chiedi al tuo amministratore.");
        translations.insert("No source specified when installing app", "Nessuna fonte specificata durante l'installazione dell'applicazione");
        translations.insert("No href specified when installing app from http", "Nessun href specificato durante l'installazione dell'applicazione da http");
        translations.insert("No path specified when installing app from local file", "Nessun percorso specificato durante l'installazione dell'applicazione da file locale");
        translations.insert("Archives of type %s are not supported", "Gli archivi di tipo %s non sono supportati");
        translations.insert("Failed to open archive when installing app", "Apertura archivio non riuscita durante l'installazione dell'applicazione");
        translations.insert("App does not provide an info.xml file", "L'applicazione non fornisce un file info.xml");
        translations.insert("App can't be installed because of not allowed code in the App", "L'applicazione non può essere installata a causa di codice non consentito al suo interno");
        translations.insert("App can't be installed because it is not compatible with this version of ownCloud", "L'applicazione non può essere installata poiché non è compatibile con questa versione di ownCloud");
        translations.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps", "L'applicazione non può essere installata poiché contiene il tag <shipped>true<shipped> che non è permesso alle applicazioni non shipped");
        translations.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store", "L'applicazione non può essere installata poiché la versione in info.xml/version non è la stessa riportata dall'app store");
        translations.insert("App directory already exists", "La cartella dell'applicazione esiste già");
        translations.insert("Can't create app folder. Please fix permissions. %s", "Impossibile creare la cartella dell'applicazione. Correggi i permessi. %s");
        translations.insert("Application is not enabled", "L'applicazione  non è abilitata");
        translations.insert("Authentication error", "Errore di autenticazione");
        translations.insert("Token expired. Please reload page.", "Token scaduto. Ricarica la pagina.");
        translations.insert("Files", "File");
        translations.insert("Text", "Testo");
        translations.insert("Images", "Immagini");
        translations.insert("%s enter the database username.", "%s digita il nome utente del database.");
        translations.insert("%s enter the database name.", "%s digita il nome del database.");
        translations.insert("%s you may not use dots in the database name", "%s non dovresti utilizzare punti nel nome del database");
        translations.insert("MS SQL username and/or password not valid: %s", "Nome utente e/o password MS SQL non validi: %s");
        translations.insert("You need to enter either an existing account or the administrator.", "È necessario inserire un account esistente o l'amministratore.");
        translations.insert("MySQL username and/or password not valid", "Nome utente e/o password di MySQL non validi");
        translations.insert("DB Error: \"%s\"", "Errore DB: \"%s\"");
        translations.insert("Offending command was: \"%s\"", "Il comando non consentito era: \"%s\"");
        translations.insert("MySQL user '%s'@'localhost' exists already.", "L'utente MySQL '%s'@'localhost' esiste già.");
        translations.insert("Drop this user from MySQL", "Elimina questo utente da MySQL");
        translations.insert("MySQL user '%s'@'%%' already exists", "L'utente MySQL '%s'@'%%' esiste già");
        translations.insert("Drop this user from MySQL.", "Elimina questo utente da MySQL.");
        translations.insert("Oracle connection could not be established", "La connessione a Oracle non può essere stabilita");
        translations.insert("Oracle username and/or password not valid", "Nome utente e/o password di Oracle non validi");
        translations.insert("Offending command was: \"%s\", name: %s, password: %s", "Il comando non consentito era: \"%s\", nome: %s, password: %s");
        translations.insert("PostgreSQL username and/or password not valid", "Nome utente e/o password di PostgreSQL non validi");
        translations.insert("Set an admin username.", "Imposta un nome utente di amministrazione.");
        translations.insert("Set an admin password.", "Imposta una password di amministrazione.");
        translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Il tuo server web non è configurato correttamente per consentire la sincronizzazione dei file poiché l'interfaccia WebDAV sembra essere danneggiata.");
        translations.insert("Please double check the <a href='%s'>installation guides</a>.", "Leggi attentamente le <a href='%s'>guide d'installazione</a>.");
        translations.insert("Could not find category \"%s\"", "Impossibile trovare la categoria \"%s\"");
        translations.insert("seconds ago", "secondi fa");
        translations.insert("today", "oggi");
        translations.insert("yesterday", "ieri");
        translations.insert("last month", "mese scorso");
        translations.insert("last year", "anno scorso");
        translations.insert("years ago", "anni fa");
        translations.insert("Caused by:", "Causato da:");
        
        Self {
            translations,
            plural_form: "nplurals=2; plural=(n != 1);",
        }
    }
}

impl PluralResolver for ItalianLocalization {
    fn resolve_plural(&self, num: FluentNumber) -> PluralCategory {
        let n = num.value as i64;
        if n != 1 {
            PluralCategory::Other
        } else {
            PluralCategory::One
        }
    }
}

impl ItalianLocalization {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn get_translation(&self, key: &str) -> Option<&str> {
        self.translations.get(key).copied()
    }
    
    pub fn get_formatted_translation(&self, key: &str, args: &[&str]) -> String {
        if let Some(template) = self.get_translation(key) {
            let mut result = String::from(template);
            for (i, arg) in args.iter().enumerate() {
                result = result.replace(&format!("%s", i+1), arg);
                result = result.replace("%s", arg);
            }
            result
        } else {
            key.to_string()
        }
    }
    
    pub fn get_plural_translation(&self, singular: &str, plural: &str, count: i64) -> String {
        let key = if count == 1 { singular } else { plural };
        
        match (singular, plural, count) {
            ("_%n minute ago_", "_%n minutes ago_", 1) => "1 minuto fa".to_string(),
            ("_%n minute ago_", "_%n minutes ago_", n) => format!("{} minuti fa", n),
            ("_%n hour ago_", "_%n hours ago_", 1) => "1 ora fa".to_string(),
            ("_%n hour ago_", "_%n hours ago_", n) => format!("{} ore fa", n),
            ("_%n day go_", "_%n days ago_", 1) => "1 giorno fa".to_string(),
            ("_%n day go_", "_%n days ago_", n) => format!("{} giorni fa", n),
            ("_%n month ago_", "_%n months ago_", 1) => "1 mese fa".to_string(),
            ("_%n month ago_", "_%n months ago_", n) => format!("{} mesi fa", n),
            _ => self.get_translation(key).unwrap_or(key).to_string(),
        }
    }
}