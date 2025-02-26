use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Italian translations
pub static IT_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("%s shared »%s« with you", "%s ha condiviso «%s» con te");
    map.insert("Couldn't send mail to following users: %s ", "Impossibile inviare email ai seguenti utenti: %s");
    map.insert("Turned on maintenance mode", "Modalità di manutenzione attivata");
    map.insert("Turned off maintenance mode", "Modalità di manutenzione disattivata");
    map.insert("Updated database", "Database aggiornato");
    map.insert("Updating filecache, this may take really long...", "Aggiornamento della cache dei file in corso, potrebbe richiedere molto tempo...");
    map.insert("Updated filecache", "Cache dei file aggiornata");
    map.insert("... %d%% done ...", "... %d%% completato ...");
    map.insert("No image or file provided", "Non è stata fornita alcun immagine o file");
    map.insert("Unknown filetype", "Tipo di file sconosciuto");
    map.insert("Invalid image", "Immagine non valida");
    map.insert("No temporary profile picture available, try again", "Nessuna immagine di profilo provvisoria disponibile, riprova");
    map.insert("No crop data provided", "Dati di ritaglio non forniti");
    map.insert("Sunday", "Domenica");
    map.insert("Monday", "Lunedì");
    map.insert("Tuesday", "Martedì");
    map.insert("Wednesday", "Mercoledì");
    map.insert("Thursday", "Giovedì");
    map.insert("Friday", "Venerdì");
    map.insert("Saturday", "Sabato");
    map.insert("January", "Gennaio");
    map.insert("February", "Febbraio");
    map.insert("March", "Marzo");
    map.insert("April", "Aprile");
    map.insert("May", "Maggio");
    map.insert("June", "Giugno");
    map.insert("July", "Luglio");
    map.insert("August", "Agosto");
    map.insert("September", "Settembre");
    map.insert("October", "Ottobre");
    map.insert("November", "Novembre");
    map.insert("December", "Dicembre");
    map.insert("Settings", "Impostazioni");
    map.insert("seconds ago", "secondi fa");
    map.insert("today", "oggi");
    map.insert("yesterday", "ieri");
    map.insert("last month", "mese scorso");
    map.insert("months ago", "mesi fa");
    map.insert("last year", "anno scorso");
    map.insert("years ago", "anni fa");
    map.insert("Choose", "Scegli");
    map.insert("Error loading file picker template: {error}", "Errore durante il caricamento del modello del selettore file: {error}");
    map.insert("Yes", "Sì");
    map.insert("No", "No");
    map.insert("Ok", "Ok");
    map.insert("Error loading message template: {error}", "Errore durante il caricamento del modello di messaggio: {error}");
    map.insert("One file conflict", "Un file in conflitto");
    map.insert("Which files do you want to keep?", "Quali file vuoi mantenere?");
    map.insert("If you select both versions, the copied file will have a number added to its name.", "Se selezioni entrambe le versioni, sarà aggiunto un numero al nome del file copiato.");
    map.insert("Cancel", "Annulla");
    map.insert("Continue", "Continua");
    map.insert("(all selected)", "(tutti i selezionati)");
    map.insert("Error loading file exists template", "Errore durante il caricamento del modello del file esistente");
    map.insert("Shared", "Condivisi");
    map.insert("Share", "Condividi");
    map.insert("Error", "Errore");
    map.insert("Error while sharing", "Errore durante la condivisione");
    map.insert("Error while unsharing", "Errore durante la rimozione della condivisione");
    map.insert("Error while changing permissions", "Errore durante la modifica dei permessi");
    map.insert("Shared with you and the group {group} by {owner}", "Condiviso con te e con il gruppo {group} da {owner}");
    map.insert("Shared with you by {owner}", "Condiviso con te da {owner}");
    map.insert("Share with user or group …", "Condividi con utente o gruppo ...");
    map.insert("Share link", "Condividi collegamento");
    map.insert("Password protect", "Proteggi con password");
    map.insert("Password", "Password");
    map.insert("Allow Public Upload", "Consenti caricamento pubblico");
    map.insert("Email link to person", "Invia collegamento via email");
    map.insert("Send", "Invia");
    map.insert("Set expiration date", "Imposta data di scadenza");
    map.insert("Expiration date", "Data di scadenza");
    map.insert("Share via email:", "Condividi tramite email:");
    map.insert("No people found", "Non sono state trovate altre persone");
    map.insert("group", "gruppo");
    map.insert("Resharing is not allowed", "La ri-condivisione non è consentita");
    map.insert("Shared in {item} with {user}", "Condiviso in {item} con {user}");
    map.insert("Unshare", "Rimuovi condivisione");
    map.insert("notify by email", "notifica tramite email");
    map.insert("can edit", "può modificare");
    map.insert("access control", "controllo d'accesso");
    map.insert("create", "creare");
    map.insert("update", "aggiornare");
    map.insert("delete", "elimina");
    map.insert("share", "condividi");
    map.insert("Password protected", "Protetta da password");
    map.insert("Error unsetting expiration date", "Errore durante la rimozione della data di scadenza");
    map.insert("Error setting expiration date", "Errore durante l'impostazione della data di scadenza");
    map.insert("Sending ...", "Invio in corso...");
    map.insert("Email sent", "Messaggio inviato");
    map.insert("Warning", "Avviso");
    map.insert("The object type is not specified.", "Il tipo di oggetto non è specificato.");
    map.insert("Enter new", "Inserisci nuovo");
    map.insert("Delete", "Elimina");
    map.insert("Add", "Aggiungi");
    map.insert("Edit tags", "Modifica etichette");
    map.insert("Error loading dialog template: {error}", "Errore durante il caricamento del modello di finestra: {error}");
    map.insert("No tags selected for deletion.", "Nessuna etichetta selezionata per l'eliminazione.");
    map.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "L'aggiornamento non è riuscito. Segnala il problema alla <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">comunità di ownCloud</a>.");
    map.insert("The update was successful. Redirecting you to ownCloud now.", "L'aggiornamento è stato effettuato correttamente. Stai per essere reindirizzato a ownCloud.");
    map.insert("%s password reset", "Ripristino password di %s");
    map.insert("Use the following link to reset your password: {link}", "Usa il collegamento seguente per ripristinare la password: {link}");
    map.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "Il collegamento per ripristinare la password è stato inviato al tuo indirizzo di posta.<br>Se non lo ricevi in tempi ragionevoli, controlla le cartelle della posta indesiderata.<br>Se non dovesse essere nemmeno lì, contatta il tuo amministratore locale.");
    map.insert("Request failed!<br>Did you make sure your email/username was right?", "Richiesta non riuscita!<br>Sei sicuro che l'indirizzo di posta/nome utente fosse corretto?");
    map.insert("You will receive a link to reset your password via Email.", "Riceverai un collegamento per ripristinare la tua password via email");
    map.insert("Username", "Nome utente");
    map.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "I file sono cifrati. Se non hai precedentemente abilitato la chiave di recupero, non sarà più possibile ritrovare i tuoi dati una volta che la password sarà ripristinata. Se non sei sicuro, per favore contatta l'amministratore prima di proseguire. Vuoi davvero continuare?");
    map.insert("Yes, I really want to reset my password now", "Sì, voglio davvero ripristinare la mia password adesso");
    map.insert("Reset", "Ripristina");
    map.insert("Your password was reset", "La password è stata ripristinata");
    map.insert("To login page", "Alla pagina di accesso");
    map.insert("New password", "Nuova password");
    map.insert("Reset password", "Ripristina la password");
    map.insert("Personal", "Personale");
    map.insert("Users", "Utenti");
    map.insert("Apps", "Applicazioni");
    map.insert("Admin", "Admin");
    map.insert("Help", "Aiuto");
    map.insert("Error loading tags", "Errore di caricamento delle etichette");
    map.insert("Tag already exists", "L'etichetta esiste già");
    map.insert("Error deleting tag(s)", "Errore di eliminazione delle etichette");
    map.insert("Error tagging", "Errore di assegnazione delle etichette");
    map.insert("Error untagging", "Errore di rimozione delle etichette");
    map.insert("Error favoriting", "Errore di creazione dei preferiti");
    map.insert("Error unfavoriting", "Errore di rimozione dai preferiti");
    map.insert("Access forbidden", "Accesso negato");
    map.insert("Cloud not found", "Nuvola non trovata");
    map.insert("Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n", "Ciao,\n\nvolevo informarti che %s ha condiviso %s con te.\nVedi: %s\n\n");
    map.insert("The share will expire on %s.\n\n", "La condivisione scadrà il %s.\n\n");
    map.insert("Cheers!", "Saluti!");
    map.insert("Security Warning", "Avviso di sicurezza");
    map.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "La tua versione di PHP è vulnerabile all'attacco NULL Byte (CVE-2006-7243)");
    map.insert("Please update your PHP installation to use %s securely.", "Aggiorna la tua installazione di PHP per utilizzare %s in sicurezza.");
    map.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Non è disponibile alcun generatore di numeri casuali sicuro. Abilita l'estensione OpenSSL di PHP");
    map.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Senza un generatore di numeri casuali sicuro, un malintenzionato potrebbe riuscire a individuare i token di ripristino delle password e impossessarsi del tuo account.");
    map.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "La cartella dei dati e i file sono probabilmente accessibili da Internet poiché il file .htaccess non funziona.");
    map.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "Per informazioni su come configurare correttamente il tuo server, vedi la <a href=\"%s\" target=\"_blank\">documentazione</a>.");
    map.insert("Create an <strong>admin account</strong>", "Crea un <strong>account amministratore</strong>");
    map.insert("Advanced", "Avanzat");
    map.insert("Data folder", "Cartella dati");
    map.insert("Configure the database", "Configura il database");
    map.insert("will be used", "sarà utilizzato");
    map.insert("Database user", "Utente del database");
    map.insert("Database password", "Password del database");
    map.insert("Database name", "Nome del database");
    map.insert("Database tablespace", "Spazio delle tabelle del database");
    map.insert("Database host", "Host del database");
    map.insert("Finish setup", "Termina la configurazione");
    map.insert("Finishing …", "Completamento...");
    map.insert("%s is available. Get more information on how to update.", "%s è disponibile. Ottieni ulteriori informazioni sull'aggiornamento.");
    map.insert("Log out", "Esci");
    map.insert("Automatic logon rejected!", "Accesso automatico rifiutato.");
    map.insert("If you did not change your password recently, your account may be compromised!", "Se non hai cambiato la password recentemente, il tuo account potrebbe essere compromesso.");
    map.insert("Please change your password to secure your account again.", "Cambia la password per rendere nuovamente sicuro il tuo account.");
    map.insert("Server side authentication failed!", "Autenticazione lato server non riuscita!");
    map.insert("Please contact your administrator.", "Contatta il tuo amministratore di sistema.");
    map.insert("Lost your password?", "Hai perso la password?");
    map.insert("remember", "ricorda");
    map.insert("Log in", "Accedi");
    map.insert("Alternative Logins", "Accessi alternativi");
    map.insert("Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>", "Ciao,<br><br>volevo informarti che %s ha condiviso %s con te.<br><a href=\"%s\">Vedi!</a><br><br>");
    map.insert("The share will expire on %s.<br><br>", "La condivisione scadrà il %s.<br><br>");
    map.insert("Updating ownCloud to version %s, this may take a while.", "Aggiornamento di ownCloud alla versione %s in corso, ciò potrebbe richiedere del tempo.");
    map.insert("This ownCloud instance is currently being updated, which may take a while.", "Questa istanza di ownCloud è in fase di aggiornamento, potrebbe richiedere del tempo.");
    map.insert("Please reload this page after a short time to continue using ownCloud.", "Ricarica questa pagina per poter continuare ad usare ownCloud.");
    map.insert("Contact your system administrator if this message persists or appeared unexpectedly.", "Contatta il tuo amministratore di sistema se questo messaggio persiste o appare inaspettatamente.");
    map.insert("Thank you for your patience.", "Grazie per la pazienza.");
    map
});

/// Italian plural forms handling
pub static IT_PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Italian plural translations
pub static IT_PLURAL_TRANSLATIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("_%n minute ago_::_%n minutes ago_", vec!["%n minuto fa", "%n minuti fa"]);
    map.insert("_%n hour ago_::_%n hours ago_", vec!["%n ora fa", "%n ore fa"]);
    map.insert("_%n day ago_::_%n days ago_", vec!["%n giorno fa", "%n giorni fa"]);
    map.insert("_%n month ago_::_%n months ago_", vec!["%n mese fa", "%n mesi fa"]);
    map.insert("_{count} file conflict_::_{count} file conflicts_", vec!["{count} file in conflitto", "{count} file in conflitto"]);
    map
});

/// Gets the appropriate plural form for Italian based on number
pub fn get_plural_form(n: i64) -> usize {
    if n != 1 { 1 } else { 0 }
}

/// Returns the translation for a given key
pub fn translate(key: &str) -> &'static str {
    IT_TRANSLATIONS.get(key).copied().unwrap_or(key)
}

/// Returns the plural translation for a given key and count
pub fn translate_plural(key: &str, count: i64) -> &'static str {
    let form = get_plural_form(count);
    IT_PLURAL_TRANSLATIONS
        .get(key)
        .and_then(|forms| forms.get(form))
        .copied()
        .unwrap_or(key)
}