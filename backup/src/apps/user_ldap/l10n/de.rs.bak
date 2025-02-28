use std::collections::HashMap;
use once_cell::sync::Lazy;

// Definición de los mensajes de traducción
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Failed to clear the mappings.", "Löschen der Zuordnung fehlgeschlagen.");
    translations.insert("Failed to delete the server configuration", "Löschen der Serverkonfiguration fehlgeschlagen");
    translations.insert("The configuration is valid and the connection could be established!", "Die Konfiguration ist gültig und die Verbindung konnte hergestellt werden!");
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Die Konfiguration ist gültig aber die Verbindung ist fehlgeschlagen. Bitte überprüfe die Servereinstellungen und Anmeldeinformationen.");
    translations.insert("The configuration is invalid. Please have a look at the logs for further details.", "Die Konfiguration ist ungültig. Weitere Details kannst Du in den Logdateien nachlesen.");
    translations.insert("No action specified", "Keine Aktion spezifiziert");
    translations.insert("No configuration specified", "Keine Konfiguration spezifiziert");
    translations.insert("No data specified", "Keine Daten spezifiziert");
    translations.insert(" Could not set configuration %s", "Die Konfiguration %s konnte nicht gesetzt werden");
    translations.insert("Deletion failed", "Löschen fehlgeschlagen");
    translations.insert("Take over settings from recent server configuration?", "Einstellungen von letzter Konfiguration übernehmen?");
    translations.insert("Keep settings?", "Einstellungen beibehalten?");
    translations.insert("Cannot add server configuration", "Das Hinzufügen der Serverkonfiguration schlug fehl");
    translations.insert("mappings cleared", "Zuordnungen gelöscht");
    translations.insert("Success", "Erfolgreich");
    translations.insert("Error", "Fehler");
    translations.insert("Select groups", "Wähle Gruppen aus");
    translations.insert("Select object classes", "Objekt-Klassen auswählen");
    translations.insert("Select attributes", "Attribute auswählen");
    translations.insert("Connection test succeeded", "Verbindungstest erfolgreich");
    translations.insert("Connection test failed", "Verbindungstest fehlgeschlagen");
    translations.insert("Do you really want to delete the current Server Configuration?", "Möchtest Du die aktuelle Serverkonfiguration wirklich löschen?");
    translations.insert("Confirm Deletion", "Löschung bestätigen");
    translations.insert("Invalid Host", "Ungültiger Host");
    translations.insert("Could not find the desired feature", "Konnte die gewünschte Funktion nicht finden");
    translations.insert("Save", "Speichern");
    translations.insert("Test Configuration", "Testkonfiguration");
    translations.insert("Help", "Hilfe");
    translations.insert("Limit the access to %s to groups meeting this criteria:", "Beschränke den Zugriff auf %s auf Gruppen, die die folgenden Kriterien erfüllen:");
    translations.insert("only those object classes:", "Nur diese Objekt-Klassen:");
    translations.insert("only from those groups:", "Nur von diesen Gruppen:");
    translations.insert("Edit raw filter instead", "Original-Filter stattdessen bearbeiten");
    translations.insert("Raw LDAP filter", "Original LDAP-Filter");
    translations.insert("The filter specifies which LDAP groups shall have access to the %s instance.", "Der Filter definiert welche LDAP-Gruppen Zugriff auf die %s Instanz haben sollen.");
    translations.insert("groups found", "Gruppen gefunden");
    translations.insert("What attribute shall be used as login name:", "Welches Attribut soll als Login-Name verwendet werden:");
    translations.insert("LDAP Username:", "LDAP-Benutzername:");
    translations.insert("LDAP Email Address:", "LDAP E-Mail-Adresse:");
    translations.insert("Other Attributes:", "Andere Attribute:");
    translations.insert("Add Server Configuration", "Serverkonfiguration hinzufügen");
    translations.insert("Host", "Host");
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Du kannst das Protokoll auslassen, außer wenn Du SSL benötigst. Beginne dann mit ldaps://");
    translations.insert("Port", "Port");
    translations.insert("User DN", "Benutzer-DN");
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "Der DN des Benutzers für LDAP-Bind, z.B.: uid=agent,dc=example,dc=com. Für anonymen Zugriff lasse DN und Passwort leer.");
    translations.insert("Password", "Passwort");
    translations.insert("For anonymous access, leave DN and Password empty.", "Lasse die Felder DN und Passwort für anonymen Zugang leer.");
    translations.insert("One Base DN per line", "Ein Basis-DN pro Zeile");
    translations.insert("You can specify Base DN for users and groups in the Advanced tab", "Du kannst Basis-DN für Benutzer und Gruppen in dem \"Erweitert\"-Reiter konfigurieren");
    translations.insert("Limit the access to %s to users meeting this criteria:", "Beschränke den Zugriff auf %s auf Benutzer, die die folgenden Kriterien erfüllen:");
    translations.insert("The filter specifies which LDAP users shall have access to the %s instance.", "Der Filter definiert welche LDAP-Benutzer Zugriff auf die %s Instanz haben sollen.");
    translations.insert("users found", "Benutzer gefunden");
    translations.insert("Back", "Zurück");
    translations.insert("Continue", "Fortsetzen");
    translations.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.", "<b>Warnung:</b> Die Anwendungen user_ldap und user_webdavauth sind inkompatibel. Es kann demzufolge zu unerwarteten Verhalten kommen. Bitte\ndeinen Systemadministator eine der beiden Anwendungen zu deaktivieren.");
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Warnung:</b> Da das PHP-Modul für LDAP nicht installiert ist, wird das Backend nicht funktionieren. Bitte Deinen Systemadministrator das Modul zu installieren.");
    translations.insert("Connection Settings", "Verbindungseinstellungen");
    translations.insert("Configuration Active", "Konfiguration aktiv");
    translations.insert("When unchecked, this configuration will be skipped.", "Konfiguration wird übersprungen wenn deaktiviert");
    translations.insert("User Login Filter", "Benutzer-Login-Filter");
    translations.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"", "Bestimmt den Filter, welcher bei einer Anmeldung angewandt wird. %%uid ersetzt den Benutzernamen bei der Anmeldung. Beispiel: \"uid=%%uid\"");
    translations.insert("Backup (Replica) Host", "Backup Host (Kopie)");
    translations.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Gib einen optionalen Backup Host an. Es muss sich um eine Kopie des Haupt LDAP/AD Servers handeln.");
    translations.insert("Backup (Replica) Port", "Backup Port");
    translations.insert("Disable Main Server", "Hauptserver deaktivieren");
    translations.insert("Only connect to the replica server.", "Nur zum Replikat-Server verbinden.");
    translations.insert("Case insensitve LDAP server (Windows)", "LDAP-Server (Windows: Groß- und Kleinschreibung bleibt unbeachtet)");
    translations.insert("Turn off SSL certificate validation.", "Schalte die SSL-Zertifikatsprüfung aus.");
    translations.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.", "Nur für Testzwecke geeignet, sollte Standardmäßig nicht verwendet werden. Falls die Verbindung nur mit dieser Option funktioniert, importiere das SSL-Zertifikat des LDAP-Servers in deinen %s Server.");
    translations.insert("Cache Time-To-Live", "Speichere Time-To-Live zwischen");
    translations.insert("in seconds. A change empties the cache.", "in Sekunden. Eine Änderung leert den Cache.");
    translations.insert("Directory Settings", "Ordnereinstellungen");
    translations.insert("User Display Name Field", "Feld für den Anzeigenamen des Benutzers");
    translations.insert("The LDAP attribute to use to generate the user's display name.", "Das LDAP-Attribut zur Generierung des Anzeigenamens des Benutzers.");
    translations.insert("Base User Tree", "Basis-Benutzerbaum");
    translations.insert("One User Base DN per line", "Ein Benutzer Basis-DN pro Zeile");
    translations.insert("User Search Attributes", "Benutzersucheigenschaften");
    translations.insert("Optional; one attribute per line", "Optional; ein Attribut pro Zeile");
    translations.insert("Group Display Name Field", "Feld für den Anzeigenamen der Gruppe");
    translations.insert("The LDAP attribute to use to generate the groups's display name.", "Das LDAP-Attribut zur Generierung des Anzeigenamens der Gruppen.");
    translations.insert("Base Group Tree", "Basis-Gruppenbaum");
    translations.insert("One Group Base DN per line", "Ein Gruppen Basis-DN pro Zeile");
    translations.insert("Group Search Attributes", "Gruppensucheigenschaften");
    translations.insert("Group-Member association", "Assoziation zwischen Gruppe und Benutzer");
    translations.insert("Special Attributes", "Spezielle Eigenschaften");
    translations.insert("Quota Field", "Kontingent Feld");
    translations.insert("Quota Default", "Standard Kontingent");
    translations.insert("in bytes", "in Bytes");
    translations.insert("Email Field", "E-Mail Feld");
    translations.insert("User Home Folder Naming Rule", "Benennungsregel für das Home-Verzeichnis des Benutzers");
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Ohne Eingabe wird der Benutzername (Standard) verwendet. Anderenfall trage ein LDAP/AD-Attribut ein.");
    translations.insert("Internal Username", "Interner Benutzername");
    translations.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.", "Standardmäßig wird der interne Benutzername mittels des UUID-Attributes erzeugt. Dies stellt sicher, dass der Benutzername einzigartig ist und keinerlei Zeichen konvertiert werden müssen. Der interne Benutzername unterliegt Beschränkungen, die nur die nachfolgenden Zeichen erlauben: [ a-zA-Z0-9_.@- ]. Andere Zeichen werden mittels ihrer korrespondierenden Zeichen ersetzt oder einfach ausgelassen. Bei Kollisionen wird ein Zähler hinzugefügt bzw. der Zähler um einen Wert erhöht. Der interne Benutzername wird benutzt, um einen Benutzer intern zu identifizieren. Es ist ebenso der standardmäßig vorausgewählte Namen des Heimatverzeichnisses. Es ist auch ein Teil der Remote-URLs - zum Beispiel für alle *DAV-Dienste. Mit dieser Einstellung kann das Standardverhalten überschrieben werden. Um ein ähnliches Verhalten wie vor ownCloud 5 zu erzielen, fügen Sie das anzuzeigende Attribut des Benutzernamens in das nachfolgende Feld ein. Lassen Sie dies hingegen für das Standardverhalten leer. Die Änderungen werden sich nur auf neu gemappte (hinzugefügte) LDAP-Benutzer auswirken.");
    translations.insert("Internal Username Attribute:", "Attribut für interne Benutzernamen:");
    translations.insert("Override UUID detection", "UUID-Erkennung überschreiben");
    translations.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.", "Standardmäßig wird die UUID-Eigenschaft automatisch erkannt. Die UUID-Eigenschaft wird genutzt, um einen LDAP-Benutzer und Gruppen einwandfrei zu identifizieren. Außerdem wird der interne Benutzername erzeugt, der auf Eigenschaften der UUID basiert, wenn es oben nicht anders angegeben wurde. Du musst allerdings sicherstellen, dass deine gewählten Eigenschaften zur Identifikation der Benutzer und Gruppen eindeutig sind und zugeordnet werden können. Lasse es frei, um es beim Standardverhalten zu belassen. Änderungen wirken sich nur auf neu gemappte (hinzugefügte) LDAP-Benutzer und -Gruppen aus.");
    translations.insert("UUID Attribute for Users:", "UUID-Attribute für Benutzer:");
    translations.insert("UUID Attribute for Groups:", "UUID-Attribute für Gruppen:");
    translations.insert("Username-LDAP User Mapping", "LDAP-Benutzernamenzuordnung");
    translations.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.", "Die Benutzernamen werden genutzt, um (Meta)Daten zuzuordnen und zu speichern. Um Benutzer eindeutig und präzise zu identifizieren, hat jeder LDAP-Benutzer einen internen Benutzernamen. Dies erfordert eine Zuordnung (mappen) von Benutzernamen zum LDAP-Benutzer. Der erstellte Benutzername wird der UUID des LDAP-Benutzernamens zugeordnet. Zusätzlich wird der DN zwischengespeichert, um die Interaktion mit dem LDAP zu minimieren, was aber nicht der Identifikation dient. Ändert sich der DN, werden die Änderungen durch gefunden. Der interne Benutzername, wird in überall verwendet. Werden die Zuordnungen gelöscht, bleiben überall Reste zurück. Die Löschung der Zuordnungen kann nicht in der Konfiguration vorgenommen werden, beeinflusst aber die LDAP-Konfiguration! Löschen Sie niemals die Zuordnungen in einer produktiven Umgebung. Lösche die Zuordnungen nur in einer Test- oder Experimentierumgebung.");
    translations.insert("Clear Username-LDAP User Mapping", "Lösche LDAP-Benutzernamenzuordnung");
    translations.insert("Clear Groupname-LDAP Group Mapping", "Lösche LDAP-Gruppennamenzuordnung");
    translations
});

// Definición de las formas plurales
pub fn get_plural_form(n: i64) -> usize {
    if n != 1 {
        1
    } else {
        0
    }
}

// Función para traducir cadenas con formas plurales
pub fn translate_plural(text: &str, n: i64) -> String {
    match text {
        "_%s group found_::_%s groups found_" => {
            let forms = ["%s Gruppe gefunden", "%s Gruppen gefunden"];
            let idx = get_plural_form(n);
            forms[idx].replace("%s", &n.to_string())
        },
        "_%s user found_::_%s users found_" => {
            let forms = ["%s Benutzer gefunden", "%s Benutzer gefunden"];
            let idx = get_plural_form(n);
            forms[idx].replace("%s", &n.to_string())
        },
        _ => text.to_string(),
    }
}

// Función para traducir una cadena simple
pub fn translate(text: &str) -> &str {
    TRANSLATIONS.get(text).unwrap_or(&text)
}

// Función para traducir una cadena con parámetros
pub fn translate_with_params(text: &str, params: &[&str]) -> String {
    let mut result = translate(text).to_string();
    for (i, param) in params.iter().enumerate() {
        result = result.replace(&format!("%{}", i + 1), param);
        result = result.replace("%s", param); // Para compatibilidad con el formato PHP
    }
    result
}