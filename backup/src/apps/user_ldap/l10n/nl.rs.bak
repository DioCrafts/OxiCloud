use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Failed to clear the mappings.", "Niet gelukt de vertalingen leeg te maken.");
        m.insert("Failed to delete the server configuration", "Verwijderen serverconfiguratie mislukt");
        m.insert("The configuration is valid and the connection could be established!", "De configuratie is geldig en de verbinding is geslaagd!");
        m.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "De configuratie is geldig, maar Bind mislukte. Controleer de serverinstellingen en inloggegevens.");
        m.insert("The configuration is invalid. Please have a look at the logs for further details.", "De configuratie is ongeldig. Bekijk de logbestanden voor meer details.");
        m.insert("No action specified", "Geen actie opgegeven");
        m.insert("No configuration specified", "Geen configuratie opgegeven");
        m.insert("No data specified", "Geen gegevens verstrekt");
        m.insert(" Could not set configuration %s", "Kon configuratie %s niet instellen");
        m.insert("Deletion failed", "Verwijderen mislukt");
        m.insert("Take over settings from recent server configuration?", "Overnemen instellingen van de recente serverconfiguratie?");
        m.insert("Keep settings?", "Instellingen bewaren?");
        m.insert("Cannot add server configuration", "Kon de serverconfiguratie niet toevoegen");
        m.insert("mappings cleared", "vertaaltabel leeggemaakt");
        m.insert("Success", "Succes");
        m.insert("Error", "Fout");
        m.insert("Select groups", "Selecteer groepen");
        m.insert("Select object classes", "Selecteer objectklasse");
        m.insert("Select attributes", "Selecteer attributen");
        m.insert("Connection test succeeded", "Verbindingstest geslaagd");
        m.insert("Connection test failed", "Verbindingstest mislukt");
        m.insert("Do you really want to delete the current Server Configuration?", "Wilt u werkelijk de huidige Serverconfiguratie verwijderen?");
        m.insert("Confirm Deletion", "Bevestig verwijderen");
        m.insert("Invalid Host", "Ongeldige server");
        m.insert("Could not find the desired feature", "Kon de gewenste functie niet vinden");
        m.insert("Save", "Bewaren");
        m.insert("Test Configuration", "Test configuratie");
        m.insert("Help", "Help");
        m.insert("Limit the access to %s to groups meeting this criteria:", "Beperk toegang tot %s tot groepen die voldoen aan deze criteria:");
        m.insert("only those object classes:", "alleen deze objectklassen");
        m.insert("only from those groups:", "alleen van deze groepen:");
        m.insert("Edit raw filter instead", "Bewerk raw filter");
        m.insert("Raw LDAP filter", "Raw LDAP filter");
        m.insert("The filter specifies which LDAP groups shall have access to the %s instance.", "Dit filter geeft aan welke LDAP groepen toegang hebben tot %s.");
        m.insert("groups found", "groepen gevonden");
        m.insert("What attribute shall be used as login name:", "Welk attribuut moet worden gebruikt als inlognaam:");
        m.insert("LDAP Username:", "LDAP Username:");
        m.insert("LDAP Email Address:", "LDAP e-mailadres:");
        m.insert("Other Attributes:", "Overige attributen:");
        m.insert("Add Server Configuration", "Toevoegen serverconfiguratie");
        m.insert("Host", "Host");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Je kunt het protocol weglaten, tenzij je SSL vereist. Start in dat geval met ldaps://");
        m.insert("Port", "Poort");
        m.insert("User DN", "User DN");
        m.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "De DN van de client gebruiker waarmee de verbinding zal worden gemaakt, bijv. uid=agent,dc=example,dc=com. Voor anonieme toegang laat je het DN en het wachtwoord leeg.");
        m.insert("Password", "Wachtwoord");
        m.insert("For anonymous access, leave DN and Password empty.", "Voor anonieme toegang, laat de DN en het wachtwoord leeg.");
        m.insert("One Base DN per line", "Een Base DN per regel");
        m.insert("You can specify Base DN for users and groups in the Advanced tab", "Je kunt het Base DN voor gebruikers en groepen specificeren in het tab Geavanceerd.");
        m.insert("Limit the access to %s to users meeting this criteria:", "Beperk toegang tot %s tot gebruikers die voldoen aan deze criteria:");
        m.insert("The filter specifies which LDAP users shall have access to the %s instance.", "Dit filter geeft aan welke LDAP gebruikers toegang hebben tot %s.");
        m.insert("users found", "gebruikers gevonden");
        m.insert("Back", "Terug");
        m.insert("Continue", "Verder");
        m.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.", "<b>Waarschuwing:</b> De Apps user_ldap en user_webdavauth zijn incompatible. U kunt onverwacht gedrag ervaren. Vraag uw beheerder om een van beide apps de deactiveren.");
        m.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Waarschuwing:</b> De PHP LDAP module is niet geïnstalleerd, het backend zal niet werken. Vraag uw systeembeheerder om de module te installeren.");
        m.insert("Connection Settings", "Verbindingsinstellingen");
        m.insert("Configuration Active", "Configuratie actief");
        m.insert("When unchecked, this configuration will be skipped.", "Als dit niet is ingeschakeld wordt deze configuratie overgeslagen.");
        m.insert("User Login Filter", "Gebruikers Login Filter");
        m.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"", "Definiëert het toe te passen filter als er geprobeerd wordt in te loggen. %%uid vervangt de gebruikersnaam bij het inloggen. Bijvoorbeeld: \"uid=%%uid\"");
        m.insert("Backup (Replica) Host", "Backup (Replica) Host");
        m.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Opgeven optionele backup host. Het moet een replica van de hoofd LDAP/AD server.");
        m.insert("Backup (Replica) Port", "Backup (Replica) Poort");
        m.insert("Disable Main Server", "Deactiveren hoofdserver");
        m.insert("Only connect to the replica server.", "Maak alleen een verbinding met de replica server.");
        m.insert("Case insensitve LDAP server (Windows)", "Niet-hoofdlettergevoelige LDAP server (Windows)");
        m.insert("Turn off SSL certificate validation.", "Schakel SSL certificaat validatie uit.");
        m.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.", "Niet aanbevolen, gebruik alleen om te testen! Als de connectie alleen werkt met deze optie, importeer dan het SSL-certificaat van de LDAP-server naar uw %s server.");
        m.insert("Cache Time-To-Live", "Cache time-to-live");
        m.insert("in seconds. A change empties the cache.", "in seconden. Een verandering maakt de cache leeg.");
        m.insert("Directory Settings", "Mapinstellingen");
        m.insert("User Display Name Field", "Gebruikers Schermnaam Veld");
        m.insert("The LDAP attribute to use to generate the user's display name.", "Het te gebruiken LDAP attribuut voor het genereren van de weergavenaam voor de gebruiker.");
        m.insert("Base User Tree", "Basis Gebruikers Structuur");
        m.insert("One User Base DN per line", "Een User Base DN per regel");
        m.insert("User Search Attributes", "Attributen voor gebruikerszoekopdrachten");
        m.insert("Optional; one attribute per line", "Optioneel; één attribuut per regel");
        m.insert("Group Display Name Field", "Groep Schermnaam Veld");
        m.insert("The LDAP attribute to use to generate the groups's display name.", "Het te gebruiken LDAP attribuut voor het genereren van de weergavenaam voor de groepen.");
        m.insert("Base Group Tree", "Basis Groupen Structuur");
        m.insert("One Group Base DN per line", "Een Group Base DN per regel");
        m.insert("Group Search Attributes", "Attributen voor groepszoekopdrachten");
        m.insert("Group-Member association", "Groepslid associatie");
        m.insert("Special Attributes", "Speciale attributen");
        m.insert("Quota Field", "Quota veld");
        m.insert("Quota Default", "Quota standaard");
        m.insert("in bytes", "in bytes");
        m.insert("Email Field", "E-mailveld");
        m.insert("User Home Folder Naming Rule", "Gebruikers Home map naamgevingsregel");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Laat leeg voor de gebruikersnaam (standaard). Of, specificeer een LDAP/AD attribuut.");
        m.insert("Internal Username", "Interne gebruikersnaam");
        m.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.", "Standaard wordt de interne gebruikersnaam aangemaakt op basis van het UUID attribuut. Het zorgt ervoor dat de gebruikersnaam uniek is en dat tekens niet hoeven te worden geconverteerd. De interne gebruikersnaam heeft als beperking dat alleen deze tekens zijn toegestaan​​: [a-zA-Z0-9_.@- ]. Andere tekens worden vervangen door hun ASCII vertaling of gewoonweg weggelaten. Bij identieke namen wordt een nummer toegevoegd of verhoogd. De interne gebruikersnaam wordt gebruikt om een ​​gebruiker binnen het systeem te herkennen. Het is ook de standaardnaam voor de standaardmap van de gebruiker in ownCloud. Het is ook een vertaling voor externe URL's, bijvoorbeeld voor alle *DAV diensten. Met deze instelling kan het standaardgedrag worden overschreven. Om een soortgelijk gedrag te bereiken als van vóór ownCloud 5, voer het gebruikersweergavenaam attribuut in in het volgende veld. Laat het leeg voor standaard gedrag. Veranderingen worden alleen toegepast op gekoppelde (toegevoegde) LDAP-gebruikers.");
        m.insert("Internal Username Attribute:", "Interne gebruikersnaam attribuut:");
        m.insert("Override UUID detection", "Negeren UUID detectie");
        m.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.", "Standaard herkent ownCloud het UUID-attribuut automatisch. Het UUID attribuut wordt gebruikt om LDAP-gebruikers en -groepen uniek te identificeren. Ook zal de interne gebruikersnaam worden aangemaakt op basis van het UUID, tenzij deze hierboven anders is aangegeven. U kunt de instelling overschrijven en zelf een waarde voor het attribuut opgeven. U moet ervoor zorgen dat het ingestelde attribuut kan worden opgehaald voor zowel gebruikers als groepen en dat het uniek is. Laat het leeg voor standaard gedrag. Veranderingen worden alleen doorgevoerd op nieuw gekoppelde (toegevoegde) LDAP-gebruikers en-groepen.");
        m.insert("UUID Attribute for Users:", "UUID attribuut voor gebruikers:");
        m.insert("UUID Attribute for Groups:", "UUID attribuut voor groepen:");
        m.insert("Username-LDAP User Mapping", "Gebruikersnaam-LDAP gebruikers vertaling");
        m.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.", "ownCloud maakt gebruik van gebruikersnamen om (meta) data op te slaan en toe te wijzen. Om gebruikers uniek te identificeren, krijgt elke LDAP-gebruiker ook een interne gebruikersnaam. Dit vereist een koppeling van de ownCloud gebruikersnaam aan een ​​LDAP-gebruiker. De gecreëerde gebruikersnaam is gekoppeld aan de UUID van de LDAP-gebruiker. Aanvullend wordt ook de 'DN' gecached om het aantal LDAP-interacties te verminderen, maar dit wordt niet gebruikt voor identificatie. Als de DN verandert, zullen de veranderingen worden gevonden. De interne naam wordt overal gebruikt. Het wissen van de koppeling zal overal resten achterlaten. Het wissen van koppelingen is niet configuratiegevoelig, maar het raakt wel alle LDAP instellingen! Zorg ervoor dat deze koppelingen nooit in een productieomgeving gewist worden. Maak ze alleen leeg in een test- of ontwikkelomgeving.");
        m.insert("Clear Username-LDAP User Mapping", "Leegmaken Gebruikersnaam-LDAP gebruikers vertaling");
        m.insert("Clear Groupname-LDAP Group Mapping", "Leegmaken Groepsnaam-LDAP groep vertaling");
        m
    };

    pub static ref PLURAL_FORMS_MAP: HashMap<&'static str, (String, String)> = {
        let mut m = HashMap::new();
        m.insert(
            "_%s group found_::_%s groups found_", 
            ("%s groep gevonden".to_string(), "%s groepen gevonden".to_string())
        );
        m.insert(
            "_%s user found_::_%s users found_", 
            ("%s gebruiker gevonden".to_string(), "%s gebruikers gevonden".to_string())
        );
        m
    };

    pub static ref PLURAL_FORMS_RULE: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn translate(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn translate_plural(key: &str, count: i64) -> Option<String> {
    PLURAL_FORMS_MAP.get(key).map(|(singular, plural)| {
        if count == 1 {
            singular.clone()
        } else {
            plural.clone()
        }
    })
}