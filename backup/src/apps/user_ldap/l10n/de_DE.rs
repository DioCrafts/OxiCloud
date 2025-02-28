use std::collections::HashMap;
use rust_gettext::Catalog;

/// German (Germany) translation catalog for the user_ldap application
pub fn get_catalog() -> Catalog {
    let mut translations = HashMap::new();
    
    translations.insert("Failed to clear the mappings.".to_string(), "Löschen der Zuordnung fehlgeschlagen.".to_string());
    translations.insert("Failed to delete the server configuration".to_string(), "Löschen der Serverkonfiguration fehlgeschlagen".to_string());
    translations.insert("The configuration is valid and the connection could be established!".to_string(), "Die Konfiguration ist gültig und die Verbindung konnte hergestellt werden!".to_string());
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.".to_string(), "Die Konfiguration ist gültig aber die Verbindung ist fehlgeschlagen. Bitte überprüfen Sie die Servereinstellungen und die Anmeldeinformationen.".to_string());
    translations.insert("The configuration is invalid. Please have a look at the logs for further details.".to_string(), "Die Konfiguration ist ungültig. Weitere Details können Sie in den Logdateien nachlesen.".to_string());
    translations.insert("No action specified".to_string(), "Keine Aktion spezifiziert".to_string());
    translations.insert("No configuration specified".to_string(), "Keine Konfiguration spezifiziert".to_string());
    translations.insert("No data specified".to_string(), "Keine Daten spezifiziert".to_string());
    translations.insert(" Could not set configuration %s".to_string(), "Die Konfiguration %s konnte nicht gesetzt werden".to_string());
    translations.insert("Deletion failed".to_string(), "Löschen fehlgeschlagen".to_string());
    translations.insert("Take over settings from recent server configuration?".to_string(), "Einstellungen von letzter Konfiguration übernehmen?".to_string());
    translations.insert("Keep settings?".to_string(), "Einstellungen beibehalten?".to_string());
    translations.insert("Cannot add server configuration".to_string(), "Das Hinzufügen der Serverkonfiguration schlug fehl".to_string());
    translations.insert("mappings cleared".to_string(), "Zuordnungen gelöscht".to_string());
    translations.insert("Success".to_string(), "Erfolg".to_string());
    translations.insert("Error".to_string(), "Fehler".to_string());
    translations.insert("Select groups".to_string(), "Wähle Gruppen".to_string());
    translations.insert("Select object classes".to_string(), "Objekt-Klassen auswählen".to_string());
    translations.insert("Select attributes".to_string(), "Attribute auswählen".to_string());
    translations.insert("Connection test succeeded".to_string(), "Verbindungstest erfolgreich".to_string());
    translations.insert("Connection test failed".to_string(), "Verbindungstest fehlgeschlagen".to_string());
    translations.insert("Do you really want to delete the current Server Configuration?".to_string(), "Möchten Sie die aktuelle Serverkonfiguration wirklich löschen?".to_string());
    translations.insert("Confirm Deletion".to_string(), "Löschung bestätigen".to_string());
    translations.insert("Invalid Host".to_string(), "Ungültiger Host".to_string());
    translations.insert("Could not find the desired feature".to_string(), "Konnte die gewünschte Funktion nicht finden".to_string());
    translations.insert("Save".to_string(), "Speichern".to_string());
    translations.insert("Test Configuration".to_string(), "Testkonfiguration".to_string());
    translations.insert("Help".to_string(), "Hilfe".to_string());
    translations.insert("Limit the access to %s to groups meeting this criteria:".to_string(), "Beschränke den Zugriff auf %s auf Gruppen, die die folgenden Kriterien erfüllen:".to_string());
    translations.insert("only those object classes:".to_string(), "Nur diese Objekt-Klassen:".to_string());
    translations.insert("only from those groups:".to_string(), "Nur von diesen Gruppen:".to_string());
    translations.insert("Edit raw filter instead".to_string(), "Original-Filter stattdessen bearbeiten".to_string());
    translations.insert("Raw LDAP filter".to_string(), "Original LDAP-Filter".to_string());
    translations.insert("The filter specifies which LDAP groups shall have access to the %s instance.".to_string(), "Der Filter definiert welche LDAP-Gruppen Zugriff auf die %s Instanz haben sollen.".to_string());
    translations.insert("groups found".to_string(), "Gruppen gefunden".to_string());
    translations.insert("What attribute shall be used as login name:".to_string(), "Welches Attribut soll als Login-Name verwendet werden:".to_string());
    translations.insert("LDAP Username:".to_string(), "LDAP-Benutzername:".to_string());
    translations.insert("LDAP Email Address:".to_string(), "LDAP E-Mail-Adresse:".to_string());
    translations.insert("Other Attributes:".to_string(), "Andere Attribute:".to_string());
    translations.insert("Add Server Configuration".to_string(), "Serverkonfiguration hinzufügen".to_string());
    translations.insert("Host".to_string(), "Host".to_string());
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://".to_string(), "Sie können das Protokoll auslassen, außer wenn Sie SSL benötigen. Beginnen Sie dann mit ldaps://".to_string());
    translations.insert("Port".to_string(), "Port".to_string());
    translations.insert("User DN".to_string(), "Benutzer-DN".to_string());
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.".to_string(), "Der DN des Benutzers für LDAP-Bind, z.B.: uid=agent,dc=example,dc=com. Für einen anonymen Zugriff lassen Sie DN und Passwort leer.".to_string());
    translations.insert("Password".to_string(), "Passwort".to_string());
    translations.insert("For anonymous access, leave DN and Password empty.".to_string(), "Lassen Sie die Felder DN und Passwort für einen anonymen Zugang leer.".to_string());
    translations.insert("One Base DN per line".to_string(), "Ein Basis-DN pro Zeile".to_string());
    translations.insert("You can specify Base DN for users and groups in the Advanced tab".to_string(), "Sie können Basis-DN für Benutzer und Gruppen in dem \"Erweitert\"-Reiter konfigurieren".to_string());
    translations.insert("Limit the access to %s to users meeting this criteria:".to_string(), "Beschränke den Zugriff auf %s auf Benutzer, die die folgenden Kriterien erfüllen:".to_string());
    translations.insert("The filter specifies which LDAP users shall have access to the %s instance.".to_string(), "Der Filter definiert welche LDAP-Benutzer Zugriff auf die %s Instanz haben sollen.".to_string());
    translations.insert("users found".to_string(), "Benutzer gefunden".to_string());
    translations.insert("Back".to_string(), "Zurück".to_string());
    translations.insert("Continue".to_string(), "Fortsetzen".to_string());
    translations.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.".to_string(), "<b>Warnung:</b> Die Anwendungen user_ldap und user_webdavauth sind inkompatibel. Es kann demzufolge zu unerwarteten Verhalten kommen. Bitten Sie Ihren Systemadministator eine der beiden Anwendungen zu deaktivieren.".to_string());
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.".to_string(), "<b>Warnung:</b> Da das PHP-Modul für LDAP nicht installiert ist, wird das Backend nicht funktionieren. Bitten Sie Ihren Systemadministrator das Modul zu installieren.".to_string());
    translations.insert("Connection Settings".to_string(), "Verbindungseinstellungen".to_string());
    translations.insert("Configuration Active".to_string(), "Konfiguration aktiv".to_string());
    translations.insert("When unchecked, this configuration will be skipped.".to_string(), "Wenn nicht angehakt, wird diese Konfiguration übersprungen.".to_string());
    translations.insert("User Login Filter".to_string(), "Benutzer-Login-Filter".to_string());
    translations.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"".to_string(), "Bestimmt den Filter, welcher bei einer Anmeldung angewandt wird. %%uid ersetzt den Benutzernamen bei der Anmeldung. Beispiel: \"uid=%%uid\"".to_string());
    translations.insert("Backup (Replica) Host".to_string(), "Backup Host (Kopie)".to_string());
    translations.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.".to_string(), "Geben Sie einen optionalen Backup Host an. Es muss sich um eine Kopie des Haupt LDAP/AD Servers handeln.".to_string());
    translations.insert("Backup (Replica) Port".to_string(), "Backup Port".to_string());
    translations.insert("Disable Main Server".to_string(), "Hauptserver deaktivieren".to_string());
    translations.insert("Only connect to the replica server.".to_string(), "Nur zum Replikat-Server verbinden.".to_string());
    translations.insert("Case insensitve LDAP server (Windows)".to_string(), "LDAP-Server (Windows: Groß- und Kleinschreibung bleibt unbeachtet)".to_string());
    translations.insert("Turn off SSL certificate validation.".to_string(), "Schalten Sie die SSL-Zertifikatsprüfung aus.".to_string());
    translations.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.".to_string(), "Nur für Testzwecke geeignet, sollte Standardmäßig nicht verwendet werden. Falls die Verbindung nur mit dieser Option funktioniert, importieren Sie das SSL-Zertifikat des LDAP-Servers in Ihren %s Server.".to_string());
    translations.insert("Cache Time-To-Live".to_string(), "Speichere Time-To-Live zwischen".to_string());
    translations.insert("in seconds. A change empties the cache.".to_string(), "in Sekunden. Eine Änderung leert den Cache.".to_string());
    translations.insert("Directory Settings".to_string(), "Ordnereinstellungen".to_string());
    translations.insert("User Display Name Field".to_string(), "Feld für den Anzeigenamen des Benutzers".to_string());
    translations.insert("The LDAP attribute to use to generate the user's display name.".to_string(), "Das LDAP-Attribut zur Generierung des Anzeigenamens des Benutzers.".to_string());
    translations.insert("Base User Tree".to_string(), "Basis-Benutzerbaum".to_string());
    translations.insert("One User Base DN per line".to_string(), "Ein Benutzer Basis-DN pro Zeile".to_string());
    translations.insert("User Search Attributes".to_string(), "Benutzersucheigenschaften".to_string());
    translations.insert("Optional; one attribute per line".to_string(), "Optional; ein Attribut pro Zeile".to_string());
    translations.insert("Group Display Name Field".to_string(), "Feld für den Anzeigenamen der Gruppe".to_string());
    translations.insert("The LDAP attribute to use to generate the groups's display name.".to_string(), "Das LDAP-Attribut zur Generierung des Anzeigenamens der Gruppen.".to_string());
    translations.insert("Base Group Tree".to_string(), "Basis-Gruppenbaum".to_string());
    translations.insert("One Group Base DN per line".to_string(), "Ein Gruppen Basis-DN pro Zeile".to_string());
    translations.insert("Group Search Attributes".to_string(), "Gruppensucheigenschaften".to_string());
    translations.insert("Group-Member association".to_string(), "Assoziation zwischen Gruppe und Benutzer".to_string());
    translations.insert("Special Attributes".to_string(), "Spezielle Eigenschaften".to_string());
    translations.insert("Quota Field".to_string(), "Kontingent-Feld".to_string());
    translations.insert("Quota Default".to_string(), "Standard-Kontingent".to_string());
    translations.insert("in bytes".to_string(), "in Bytes".to_string());
    translations.insert("Email Field".to_string(), "E-Mail-Feld".to_string());
    translations.insert("User Home Folder Naming Rule".to_string(), "Benennungsregel für das Home-Verzeichnis des Benutzers".to_string());
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.".to_string(), "Ohne Eingabe wird der Benutzername (Standard) verwendet. Anderenfalls tragen Sie bitte ein LDAP/AD-Attribut ein.".to_string());
    translations.insert("Internal Username".to_string(), "Interner Benutzername".to_string());
    translations.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.".to_string(), "Standardmäßig wird der interne Benutzername mittels des UUID-Attributes erzeugt. Dies stellt sicher, dass der Benutzername einzigartig ist und keinerlei Zeichen konvertiert werden müssen. Der interne Benutzername unterliegt Beschränkungen, die nur die nachfolgenden Zeichen erlauben: [ a-zA-Z0-9_.@- ]. Andere Zeichen werden mittels ihrer korrespondierenden Zeichen ersetzt oder einfach ausgelassen. Bei Kollisionen wird ein Zähler hinzugefügt bzw. der Zähler um einen Wert erhöht. Der interne Benutzername wird benutzt, um einen Benutzer intern zu identifizieren. Es ist ebenso der standardmäßig vorausgewählte Namen des Heimatverzeichnisses. Es ist auch ein Teil der Remote-URLs - zum Beispiel für alle *DAV-Dienste. Mit dieser Einstellung kann das Standardverhalten überschrieben werden. Um ein ähnliches Verhalten wie vor ownCloud 5 zu erzielen, fügen Sie das anzuzeigende Attribut des Benutzernamens in das nachfolgende Feld ein. Lassen Sie dies hingegen für das Standardverhalten leer. Die Änderungen werden sich nur auf neu gemappte (hinzugefügte) LDAP-Benutzer auswirken.".to_string());
    translations.insert("Internal Username Attribute:".to_string(), "Interne Eigenschaften des Benutzers:".to_string());
    translations.insert("Override UUID detection".to_string(), "UUID-Erkennung überschreiben".to_string());
    translations.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.".to_string(), "Standardmäßig wird die UUID-Eigenschaft automatisch erkannt. Die UUID-Eigenschaft wird genutzt, um einen LDAP-Benutzer und Gruppen einwandfrei zu identifizieren. Außerdem wird der interne Benutzername erzeugt, der auf Eigenschaften der UUID basiert, wenn es oben nicht anders angegeben wurde. Sie müssen allerdings sicherstellen, dass Ihre gewählten Eigenschaften zur Identifikation der Benutzer und Gruppen eindeutig sind und zugeordnet werden können. Lassen Sie es frei, um es beim Standardverhalten zu belassen. Änderungen wirken sich nur auf neu gemappte (hinzugefügte) LDAP-Benutzer und -Gruppen aus.".to_string());
    translations.insert("UUID Attribute for Users:".to_string(), "UUID-Attribute für Benutzer:".to_string());
    translations.insert("UUID Attribute for Groups:".to_string(), "UUID-Attribute für Gruppen:".to_string());
    translations.insert("Username-LDAP User Mapping".to_string(), "LDAP-Benutzernamenzuordnung".to_string());
    translations.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.".to_string(), "Die Benutzernamen werden genutzt, um (Meta)Daten zuzuordnen und zu speichern. Um Benutzer eindeutig und präzise zu identifizieren, hat jeder LDAP-Benutzer einen internen Benutzernamen. Dies erfordert eine Zuordnung (mappen) von Benutzernamen zum LDAP-Benutzer. Der erstellte Benutzername wird der UUID des LDAP-Benutzernamens zugeordnet. Zusätzlich wird der DN zwischengespeichert, um die Interaktion mit dem LDAP zu minimieren, was aber nicht der Identifikation dient. Ändert sich der DN, werden die Änderungen durch gefunden. Der interne Benutzername, wird in überall verwendet. Werden die Zuordnungen gelöscht, bleiben überall Reste zurück. Die Löschung der Zuordnungen kann nicht in der Konfiguration vorgenommen werden, beeinflusst aber die LDAP-Konfiguration! Löschen Sie niemals die Zuordnungen in einer produktiven Umgebung. Löschen Sie die Zuordnungen nur in einer Test- oder Experimentierumgebung.".to_string());
    translations.insert("Clear Username-LDAP User Mapping".to_string(), "Lösche LDAP-Benutzernamenzuordnung".to_string());
    translations.insert("Clear Groupname-LDAP Group Mapping".to_string(), "Lösche LDAP-Gruppennamenzuordnung".to_string());
    
    // Plurals
    let mut plurals = HashMap::new();
    plurals.insert("_%s group found_::_%s groups found_".to_string(), vec![
        "%s Gruppe gefunden".to_string(),
        "%s Gruppen gefunden".to_string(),
    ]);
    plurals.insert("_%s user found_::_%s users found_".to_string(), vec![
        "%s Benutzer gefunden".to_string(),
        "%s Benutzer gefunden".to_string(),
    ]);
    
    Catalog {
        translations,
        plurals,
        plural_form: "nplurals=2; plural=(n != 1);".to_string(),
    }
}