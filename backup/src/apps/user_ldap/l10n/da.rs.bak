use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Failed to delete the server configuration", "Kunne ikke slette server konfigurationen");
    map.insert("The configuration is valid and the connection could be established!", "Konfigurationen er korrekt og forbindelsen kunne etableres!");
    map.insert("Deletion failed", "Fejl ved sletning");
    map.insert("Take over settings from recent server configuration?", "Overtag indstillinger fra nylig server konfiguration? ");
    map.insert("Keep settings?", "Behold indstillinger?");
    map.insert("Cannot add server configuration", "Kan ikke tilføje serverkonfiguration");
    map.insert("Success", "Succes");
    map.insert("Error", "Fejl");
    map.insert("Select groups", "Vælg grupper");
    map.insert("Connection test succeeded", "Forbindelsestest lykkedes");
    map.insert("Connection test failed", "Forbindelsestest mislykkedes");
    map.insert("Do you really want to delete the current Server Configuration?", "Ønsker du virkelig at slette den nuværende Server Konfiguration?");
    map.insert("Confirm Deletion", "Bekræft Sletning");
    map.insert("Save", "Gem");
    map.insert("Test Configuration", "Test Konfiguration");
    map.insert("Help", "Hjælp");
    map.insert("Add Server Configuration", "Tilføj Server Konfiguration");
    map.insert("Host", "Host");
    map.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Du kan udelade protokollen, medmindre du skal bruge SSL. Start i så fald med ldaps://");
    map.insert("Port", "Port");
    map.insert("User DN", "Bruger DN");
    map.insert("Password", "Kodeord");
    map.insert("For anonymous access, leave DN and Password empty.", "For anonym adgang, skal du lade DN og Adgangskode tomme.");
    map.insert("You can specify Base DN for users and groups in the Advanced tab", "You can specify Base DN for users and groups in the Advanced tab");
    map.insert("Back", "Tilbage");
    map.insert("Continue", "Videre");
    map.insert("Connection Settings", "Forbindelsesindstillinger ");
    map.insert("Configuration Active", "Konfiguration Aktiv");
    map.insert("User Login Filter", "Bruger Login Filter");
    map.insert("Backup (Replica) Host", "Backup (Replika) Vært");
    map.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Opgiv en ikke obligatorisk backup server. Denne skal være en replikation af hoved-LDAP/AD serveren.");
    map.insert("Backup (Replica) Port", "Backup (Replika) Port");
    map.insert("Disable Main Server", "Deaktiver Hovedserver");
    map.insert("Only connect to the replica server.", "Forbind kun til replika serveren.");
    map.insert("Case insensitve LDAP server (Windows)", "Ikke versalfølsom LDAP server (Windows)");
    map.insert("Turn off SSL certificate validation.", "Deaktiver SSL certifikat validering");
    map.insert("Cache Time-To-Live", "Cache Time-To-Live");
    map.insert("User Display Name Field", "User Display Name Field");
    map.insert("Base User Tree", "Base Bruger Træ");
    map.insert("Base Group Tree", "Base Group Tree");
    map.insert("Group-Member association", "Group-Member association");
    map.insert("Quota Field", "Kvote Felt");
    map.insert("in bytes", "i bytes");
    map.insert("Email Field", "Email Felt");
    map.insert("Internal Username", "Internt Brugernavn");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_plural_message(message: &str, count: i64) -> &'static str {
    match message {
        "_%s group found_::_%s groups found_" => {
            if count != 1 { "" } else { "" }
        },
        "_%s user found_::_%s users found_" => {
            if count != 1 { "" } else { "" }
        },
        _ => "",
    }
}

pub fn translate(message: &str) -> &'static str {
    TRANSLATIONS.get(message).copied().unwrap_or(message)
}