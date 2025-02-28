use std::collections::HashMap;
use rust_i18n::t;

// Definición de las traducciones al italiano
pub fn register_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Failed to clear the mappings.".to_string(), "Cancellazione delle associazioni non riuscita.".to_string());
    translations.insert("Failed to delete the server configuration".to_string(), "Eliminazione della configurazione del server non riuscita".to_string());
    translations.insert("The configuration is valid and the connection could be established!".to_string(), "La configurazione è valida e la connessione può essere stabilita.".to_string());
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.".to_string(), "La configurazione è valida, ma il Bind non è riuscito. Controlla le impostazioni del server e le credenziali.".to_string());
    translations.insert("The configuration is invalid. Please have a look at the logs for further details.".to_string(), "La configurazione non è valida. Controlla i log per ulteriori dettagli.".to_string());
    translations.insert("No action specified".to_string(), "Nessuna azione specificata".to_string());
    translations.insert("No configuration specified".to_string(), "Nessuna configurazione specificata".to_string());
    translations.insert("No data specified".to_string(), "Nessun dato specificato".to_string());
    translations.insert(" Could not set configuration %s".to_string(), "Impossibile impostare la configurazione %s".to_string());
    translations.insert("Deletion failed".to_string(), "Eliminazione non riuscita".to_string());
    translations.insert("Take over settings from recent server configuration?".to_string(), "Vuoi recuperare le impostazioni dalla configurazione recente del server?".to_string());
    translations.insert("Keep settings?".to_string(), "Vuoi mantenere le impostazioni?".to_string());
    translations.insert("Cannot add server configuration".to_string(), "Impossibile aggiungere la configurazione del server".to_string());
    translations.insert("mappings cleared".to_string(), "associazioni cancellate".to_string());
    translations.insert("Success".to_string(), "Riuscito".to_string());
    translations.insert("Error".to_string(), "Errore".to_string());
    translations.insert("Select groups".to_string(), "Seleziona i gruppi".to_string());
    translations.insert("Select object classes".to_string(), "Seleziona le classi di oggetti".to_string());
    translations.insert("Select attributes".to_string(), "Seleziona gli attributi".to_string());
    translations.insert("Connection test succeeded".to_string(), "Prova di connessione riuscita".to_string());
    translations.insert("Connection test failed".to_string(), "Prova di connessione non riuscita".to_string());
    translations.insert("Do you really want to delete the current Server Configuration?".to_string(), "Vuoi davvero eliminare la configurazione attuale del server?".to_string());
    translations.insert("Confirm Deletion".to_string(), "Conferma l'eliminazione".to_string());
    translations.insert("_%s group found_::_%s groups found_".to_string(), "%s gruppo trovato||%s gruppi trovati".to_string());
    translations.insert("_%s user found_::_%s users found_".to_string(), "%s utente trovato||%s utenti trovati".to_string());
    translations.insert("Invalid Host".to_string(), "Host non valido".to_string());
    translations.insert("Could not find the desired feature".to_string(), "Impossibile trovare la funzionalità desiderata".to_string());
    translations.insert("Save".to_string(), "Salva".to_string());
    translations.insert("Test Configuration".to_string(), "Prova configurazione".to_string());
    translations.insert("Help".to_string(), "Aiuto".to_string());
    translations.insert("Limit the access to %s to groups meeting this criteria:".to_string(), "Limita l'accesso a %s ai gruppi che verificano questi criteri:".to_string());
    translations.insert("only those object classes:".to_string(), "solo queste classi di oggetti:".to_string());
    translations.insert("only from those groups:".to_string(), "solo da questi gruppi:".to_string());
    translations.insert("Edit raw filter instead".to_string(), "Modifica invece il filtro grezzo".to_string());
    translations.insert("Raw LDAP filter".to_string(), "Filtro LDAP grezzo".to_string());
    translations.insert("The filter specifies which LDAP groups shall have access to the %s instance.".to_string(), "Il filtro specifica quali gruppi LDAP devono avere accesso all'istanza %s.".to_string());
    translations.insert("groups found".to_string(), "gruppi trovati".to_string());
    translations.insert("What attribute shall be used as login name:".to_string(), "Quale attributo deve essere usato come nome di accesso:".to_string());
    translations.insert("LDAP Username:".to_string(), "Nome utente LDAP:".to_string());
    translations.insert("LDAP Email Address:".to_string(), "Indirizzo email LDAP:".to_string());
    translations.insert("Other Attributes:".to_string(), "Altri attributi:".to_string());
    translations.insert("Add Server Configuration".to_string(), "Aggiungi configurazione del server".to_string());
    translations.insert("Host".to_string(), "Host".to_string());
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://".to_string(), "È possibile omettere il protocollo, ad eccezione se è necessario SSL. Quindi inizia con ldaps://".to_string());
    translations.insert("Port".to_string(), "Porta".to_string());
    translations.insert("User DN".to_string(), "DN utente".to_string());
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.".to_string(), "Il DN per il client dell'utente con cui deve essere associato, ad esempio uid=agent,dc=example,dc=com. Per l'accesso anonimo, lasciare vuoti i campi DN e Password".to_string());
    translations.insert("Password".to_string(), "Password".to_string());
    translations.insert("For anonymous access, leave DN and Password empty.".to_string(), "Per l'accesso anonimo, lasciare vuoti i campi DN e Password".to_string());
    translations.insert("One Base DN per line".to_string(), "Un DN base per riga".to_string());
    translations.insert("You can specify Base DN for users and groups in the Advanced tab".to_string(), "Puoi specificare una DN base per gli utenti ed i gruppi nella scheda Avanzate".to_string());
    translations.insert("Limit the access to %s to users meeting this criteria:".to_string(), "Limita l'accesso a %s ai gruppi che verificano questi criteri:".to_string());
    translations.insert("The filter specifies which LDAP users shall have access to the %s instance.".to_string(), "Il filtro specifica quali utenti LDAP devono avere accesso all'istanza %s.".to_string());
    translations.insert("users found".to_string(), "utenti trovati".to_string());
    translations.insert("Back".to_string(), "Indietro".to_string());
    translations.insert("Continue".to_string(), "Continua".to_string());
    translations.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.".to_string(), "<b>Avviso:</b> le applicazioni user_ldap e user_webdavauth sono incompatibili. Potresti riscontrare un comportamento inatteso. Chiedi al tuo amministratore di sistema di disabilitarne una.".to_string());
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.".to_string(), "<b>Avviso:</b> il modulo PHP LDAP non è installato, il motore non funzionerà. Chiedi al tuo amministratore di sistema di installarlo.".to_string());
    translations.insert("Connection Settings".to_string(), "Impostazioni di connessione".to_string());
    translations.insert("Configuration Active".to_string(), "Configurazione attiva".to_string());
    translations.insert("When unchecked, this configuration will be skipped.".to_string(), "Se deselezionata, questa configurazione sarà saltata.".to_string());
    translations.insert("User Login Filter".to_string(), "Filtro per l'accesso utente".to_string());
    translations.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"".to_string(), "Specifica quale filtro utilizzare quando si tenta l'accesso. %%uid sostituisce il nome utente all'atto dell'accesso. Esempio: \"uid=%%uid\"".to_string());
    translations.insert("Backup (Replica) Host".to_string(), "Host di backup (Replica)".to_string());
    translations.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.".to_string(), "Fornisci un host di backup opzionale. Deve essere una replica del server AD/LDAP principale.".to_string());
    translations.insert("Backup (Replica) Port".to_string(), "Porta di backup (Replica)".to_string());
    translations.insert("Disable Main Server".to_string(), "Disabilita server principale".to_string());
    translations.insert("Only connect to the replica server.".to_string(), "Collegati solo al server di replica.".to_string());
    translations.insert("Case insensitve LDAP server (Windows)".to_string(), "Case insensitve LDAP server (Windows)".to_string());
    translations.insert("Turn off SSL certificate validation.".to_string(), "Disattiva il controllo del certificato SSL.".to_string());
    translations.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.".to_string(), "Non consigliata, da utilizzare solo per test! Se la connessione funziona solo con questa opzione, importa il certificate SSL del server LDAP sul tuo server %s.".to_string());
    translations.insert("Cache Time-To-Live".to_string(), "Tempo di vita della cache".to_string());
    translations.insert("in seconds. A change empties the cache.".to_string(), "in secondi. Il cambio svuota la cache.".to_string());
    translations.insert("Directory Settings".to_string(), "Impostazioni delle cartelle".to_string());
    translations.insert("User Display Name Field".to_string(), "Campo per la visualizzazione del nome utente".to_string());
    translations.insert("The LDAP attribute to use to generate the user's display name.".to_string(), "L'attributo LDAP da usare per generare il nome visualizzato dell'utente.".to_string());
    translations.insert("Base User Tree".to_string(), "Struttura base dell'utente".to_string());
    translations.insert("One User Base DN per line".to_string(), "Un DN base utente per riga".to_string());
    translations.insert("User Search Attributes".to_string(), "Attributi di ricerca utente".to_string());
    translations.insert("Optional; one attribute per line".to_string(), "Opzionale; un attributo per riga".to_string());
    translations.insert("Group Display Name Field".to_string(), "Campo per la visualizzazione del nome del gruppo".to_string());
    translations.insert("The LDAP attribute to use to generate the groups's display name.".to_string(), "L'attributo LDAP da usare per generare il nome visualizzato del gruppo.".to_string());
    translations.insert("Base Group Tree".to_string(), "Struttura base del gruppo".to_string());
    translations.insert("One Group Base DN per line".to_string(), "Un DN base gruppo per riga".to_string());
    translations.insert("Group Search Attributes".to_string(), "Attributi di ricerca gruppo".to_string());
    translations.insert("Group-Member association".to_string(), "Associazione gruppo-utente ".to_string());
    translations.insert("Special Attributes".to_string(), "Attributi speciali".to_string());
    translations.insert("Quota Field".to_string(), "Campo Quota".to_string());
    translations.insert("Quota Default".to_string(), "Quota predefinita".to_string());
    translations.insert("in bytes".to_string(), "in byte".to_string());
    translations.insert("Email Field".to_string(), "Campo Email".to_string());
    translations.insert("User Home Folder Naming Rule".to_string(), "Regola di assegnazione del nome della cartella utente".to_string());
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.".to_string(), "Lascia vuoto per il nome utente (predefinito). Altrimenti, specifica un attributo LDAP/AD.".to_string());
    translations.insert("Internal Username".to_string(), "Nome utente interno".to_string());
    translations.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.".to_string(), "In modo predefinito, il nome utente interno sarà creato dall'attributo UUID. Ciò assicura che il nome utente sia univoco e che non sia necessario convertire i caratteri. Il nome utente interno consente l'uso di determinati caratteri:  [ a-zA-Z0-9_.@- ]. Altri caratteri sono sostituiti con il corrispondente ASCII o sono semplicemente omessi. In caso di conflitto, sarà aggiunto/incrementato un numero. Il nome utente interno è utilizzato per identificare un utente internamente. Rappresenta, inoltre, il nome predefinito per la cartella home dell'utente in ownCloud. Costituisce anche una parte di URL remoti, ad esempio per tutti i servizi *DAV. Con questa impostazione, il comportamento predefinito può essere scavalcato. Per ottenere un comportamento simile alle versioni precedenti ownCloud 5, inserisci l'attributo del nome visualizzato dell'utente nel campo seguente. Lascialo vuoto per il comportamento predefinito. Le modifiche avranno effetto solo sui nuovo utenti LDAP associati (aggiunti).".to_string());
    translations.insert("Internal Username Attribute:".to_string(), "Attributo nome utente interno:".to_string());
    translations.insert("Override UUID detection".to_string(), "Ignora rilevamento UUID".to_string());
    translations.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.".to_string(), "In modo predefinito, l'attributo UUID viene rilevato automaticamente. L'attributo UUID è utilizzato per identificare senza alcun dubbio gli utenti e i gruppi LDAP. Inoltre, il nome utente interno sarà creato sulla base dell'UUID, se non è specificato in precedenza. Puoi ignorare l'impostazione e fornire un attributo di tua scelta. Assicurati che l'attributo scelto possa essere ottenuto sia per gli utenti che per i gruppi e che sia univoco. Lascialo vuoto per ottenere il comportamento predefinito. Le modifiche avranno effetto solo sui nuovi utenti e gruppi LDAP associati (aggiunti).".to_string());
    translations.insert("UUID Attribute for Users:".to_string(), "Attributo UUID per gli utenti:".to_string());
    translations.insert("UUID Attribute for Groups:".to_string(), "Attributo UUID per i gruppi:".to_string());
    translations.insert("Username-LDAP User Mapping".to_string(), "Associazione Nome utente-Utente LDAP".to_string());
    translations.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.".to_string(), "I nomi utente sono utilizzati per archiviare e assegnare i (meta) dati. Per identificare con precisione e riconoscere gli utenti, ogni utente LDAP avrà un nome utente interno. Ciò richiede un'associazione tra il nome utente e l'utente LDAP. In aggiunta, il DN viene mantenuto in cache per ridurre l'interazione con LDAP, ma non è utilizzato per l'identificazione. Se il DN cambia, le modifiche saranno rilevate. Il nome utente interno è utilizzato dappertutto. La cancellazione delle associazioni lascerà tracce residue ovunque e interesserà esclusivamente la configurazione LDAP. Non cancellare mai le associazioni in un ambiente di produzione, ma solo in una fase sperimentale o di test.".to_string());
    translations.insert("Clear Username-LDAP User Mapping".to_string(), "Cancella associazione Nome utente-Utente LDAP".to_string());
    translations.insert("Clear Groupname-LDAP Group Mapping".to_string(), "Cancella associazione Nome gruppo-Gruppo LDAP".to_string());
    
    translations
}

// Configuración de plurales para italiano
pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}