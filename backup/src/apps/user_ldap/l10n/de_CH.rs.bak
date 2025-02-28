use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Failed to clear the mappings.", "Löschen der Zuordnung fehlgeschlagen.");
        m.insert("Failed to delete the server configuration", "Löschen der Serverkonfiguration fehlgeschlagen");
        m.insert("The configuration is valid and the connection could be established!", "Die Konfiguration ist gültig und die Verbindung konnte hergestellt werden!");
        m.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Die Konfiguration ist gültig aber die Verbindung ist fehlgeschlagen. Bitte überprüfen Sie die Servereinstellungen und die Anmeldeinformationen.");
        m.insert("Deletion failed", "Löschen fehlgeschlagen");
        m.insert("Take over settings from recent server configuration?", "Einstellungen von letzter Konfiguration übernehmen?");
        m.insert("Keep settings?", "Einstellungen beibehalten?");
        m.insert("Cannot add server configuration", "Das Hinzufügen der Serverkonfiguration schlug fehl");
        m.insert("mappings cleared", "Zuordnungen gelöscht");
        m.insert("Success", "Erfolg");
        m.insert("Error", "Fehler");
        m.insert("Select groups", "Wähle Gruppen");
        m.insert("Connection test succeeded", "Verbindungstest erfolgreich");
        m.insert("Connection test failed", "Verbindungstest fehlgeschlagen");
        m.insert("Do you really want to delete the current Server Configuration?", "Möchten Sie die aktuelle Serverkonfiguration wirklich löschen?");
        m.insert("Confirm Deletion", "Löschung bestätigen");
        m.insert("_%s group found_::_%s groups found_", "");
        m.insert("_%s user found_::_%s users found_", "");
        m.insert("Save", "Speichern");
        m.insert("Test Configuration", "Testkonfiguration");
        m.insert("Help", "Hilfe");
        m.insert("Add Server Configuration", "Serverkonfiguration hinzufügen");
        m.insert("Host", "Host");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Sie können das Protokoll auslassen, ausser wenn Sie SSL benötigen. Beginnen Sie dann mit ldaps://");
        m.insert("Port", "Port");
        m.insert("User DN", "Benutzer-DN");
        m.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "Der DN des Benutzers für LDAP-Bind, z.B.: uid=agent,dc=example,dc=com. Für einen anonymen Zugriff lassen Sie DN und Passwort leer.");
        m.insert("Password", "Passwort");
        m.insert("For anonymous access, leave DN and Password empty.", "Lassen Sie die Felder DN und Passwort für einen anonymen Zugang leer.");
        m.insert("One Base DN per line", "Ein Basis-DN pro Zeile");
        m.insert("You can specify Base DN for users and groups in the Advanced tab", "Sie können Basis-DN für Benutzer und Gruppen in dem «Erweitert»-Reiter konfigurieren");
        m.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.", "<b>Warnung:</b> Die Anwendungen user_ldap und user_webdavauth sind inkompatibel. Es kann demzufolge zu unerwarteten Verhalten kommen. Bitten Sie Ihren Systemadministator eine der beiden Anwendungen zu deaktivieren.");
        m.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Warnung:</b> Da das PHP-Modul für LDAP nicht installiert ist, wird das Backend nicht funktionieren. Bitten Sie Ihren Systemadministrator das Modul zu installieren.");
        m.insert("Connection Settings", "Verbindungseinstellungen");
        m.insert("Configuration Active", "Konfiguration aktiv");
        m.insert("When unchecked, this configuration will be skipped.", "Wenn nicht angehakt, wird diese Konfiguration übersprungen.");
        m.insert("User Login Filter", "Benutzer-Login-Filter");
        m.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"", "Bestimmt den Filter, welcher bei einer Anmeldung angewandt wird. %%uid ersetzt den Benutzernamen bei der Anmeldung. Beispiel: \"uid=%%uid\"");
        m.insert("Backup (Replica) Host", "Backup Host (Kopie)");
        m.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Geben Sie einen optionalen Backup Host an. Es muss sich um eine Kopie des Haupt LDAP/AD Servers handeln.");
        m.insert("Backup (Replica) Port", "Backup Port");
        m.insert("Disable Main Server", "Hauptserver deaktivieren");
        m.insert("Only connect to the replica server.", "Nur zum Replikat-Server verbinden.");
        m.insert("Case insensitve LDAP server (Windows)", "LDAP-Server (Windows: Gross- und Kleinschreibung bleibt unbeachtet)");
        m.insert("Turn off SSL certificate validation.", "Schalten Sie die SSL-Zertifikatsprüfung aus.");
        m.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.", "Nur für Testzwecke geeignet, sollte Standardmäßig nicht verwendet werden. Falls die Verbindung nur mit dieser Option funktioniert, importieren Sie das SSL-Zertifikat des LDAP-Servers in Ihren %s Server.");
        m.insert("Cache Time-To-Live", "Speichere Time-To-Live zwischen");
        m.insert("in seconds. A change empties the cache.", "in Sekunden. Eine Änderung leert den Cache.");
        m.insert("Directory Settings", "Ordnereinstellungen");
        m.insert("User Display Name Field", "Feld für den Anzeigenamen des Benutzers");
        m.insert("The LDAP attribute to use to generate the user's display name.", "Das LDAP-Attribut zur Generierung des Anzeigenamens des Benutzers.");
        m.insert("Base User Tree", "Basis-Benutzerbaum");
        m.insert("One User Base DN per line", "Ein Benutzer Basis-DN pro Zeile");
        m.insert("User Search Attributes", "Benutzersucheigenschaften");
        m.insert("Optional; one attribute per line", "Optional; ein Attribut pro Zeile");
        m.insert("Group Display Name Field", "Feld für den Anzeigenamen der Gruppe");
        m.insert("The LDAP attribute to use to generate the groups's display name.", "Das LDAP-Attribut zur Generierung des Anzeigenamens der Gruppen.");
        m.insert("Base Group Tree", "Basis-Gruppenbaum");
        m.insert("One Group Base DN per line", "Ein Gruppen Basis-DN pro Zeile");
        m.insert("Group Search Attributes", "Gruppensucheigenschaften");
        m.insert("Group-Member association", "Assoziation zwischen Gruppe und Benutzer");
        m.insert("Special Attributes", "Spezielle Eigenschaften");
        m.insert("Quota Field", "Kontingent-Feld");
        m.insert("Quota Default", "Standard-Kontingent");
        m.insert("in bytes", "in Bytes");
        m.insert("Email Field", "E-Mail-Feld");
        m.insert("User Home Folder Naming Rule", "Benennungsregel für das Home-Verzeichnis des Benutzers");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Ohne Eingabe wird der Benutzername (Standard) verwendet. Anderenfalls tragen Sie bitte ein LDAP/AD-Attribut ein.");
        m.insert("Internal Username", "Interner Benutzername");
        m.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.", "Standardmässig wird der interne Benutzername mittels des UUID-Attributes erzeugt. Dies stellt sicher, dass der Benutzername einzigartig ist und keinerlei Zeichen konvertiert werden müssen. Der interne Benutzername unterliegt Beschränkungen, die nur die nachfolgenden Zeichen erlauben: [ a-zA-Z0-9_.@- ]. Andere Zeichen werden mittels ihrer korrespondierenden Zeichen ersetzt oder einfach ausgelassen. Bei Kollisionen wird ein Zähler hinzugefügt bzw. der Zähler um einen Wert erhöht. Der interne Benutzername wird benutzt, um einen Benutzer intern zu identifizieren. Es ist ebenso der standardmässig vorausgewählte Namen des Heimatverzeichnisses. Es ist auch ein Teil der Remote-URLs - zum Beispiel für alle *DAV-Dienste. Mit dieser Einstellung kann das Standardverhalten überschrieben werden. Um ein ähnliches Verhalten wie vor ownCloud 5 zu erzielen, fügen Sie das anzuzeigende Attribut des Benutzernamens in das nachfolgende Feld ein. Lassen Sie dies hingegen für das Standardverhalten leer. Die Änderungen werden sich nur auf neu gemappte (hinzugefügte) LDAP-Benutzer auswirken.");
        m.insert("Internal Username Attribute:", "Interne Eigenschaften des Benutzers:");
        m.insert("Override UUID detection", "UUID-Erkennung überschreiben");
        m.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.", "Standardmässig wird die UUID-Eigenschaft automatisch erkannt. Die UUID-Eigenschaft wird genutzt, um einen LDAP-Benutzer und Gruppen einwandfrei zu identifizieren. Ausserdem wird der interne Benutzername erzeugt, der auf Eigenschaften der UUID basiert, wenn es oben nicht anders angegeben wurde. Sie müssen allerdings sicherstellen, dass Ihre gewählten Eigenschaften zur Identifikation der Benutzer und Gruppen eindeutig sind und zugeordnet werden können. Lassen Sie es frei, um es beim Standardverhalten zu belassen. Änderungen wirken sich nur auf neu gemappte (hinzugefügte) LDAP-Benutzer und -Gruppen aus.");
        m.insert("Username-LDAP User Mapping", "LDAP-Benutzernamenzuordnung");
        m.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.", "Die Benutzernamen werden genutzt, um (Meta)Daten zuzuordnen und zu speichern. Um Benutzer eindeutig und präzise zu identifizieren, hat jeder LDAP-Benutzer einen internen Benutzernamen. Dies erfordert eine Zuordnung (mappen) von Benutzernamen zum LDAP-Benutzer. Der erstellte Benutzername wird der UUID des LDAP-Benutzernamens zugeordnet. Zusätzlich wird der DN zwischengespeichert, um die Interaktion mit dem LDAP zu minimieren, was aber nicht der Identifikation dient. Ändert sich der DN, werden die Änderungen durch gefunden. Der interne Benutzername, wird in überall verwendet. Werden die Zuordnungen gelöscht, bleiben überall Reste zurück. Die Löschung der Zuordnungen kann nicht in der Konfiguration vorgenommen werden, beeinflusst aber die LDAP-Konfiguration! Löschen Sie niemals die Zuordnungen in einer produktiven Umgebung. Löschen Sie die Zuordnungen nur in einer Test- oder Experimentierumgebung.");
        m.insert("Clear Username-LDAP User Mapping", "Lösche LDAP-Benutzernamenzuordnung");
        m.insert("Clear Groupname-LDAP Group Mapping", "Lösche LDAP-Gruppennamenzuordnung");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}