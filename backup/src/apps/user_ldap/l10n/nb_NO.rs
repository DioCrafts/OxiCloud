use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Failed to delete the server configuration", "Klarte ikke å slette tjener-konfigurasjonen.");
        m.insert("The configuration is valid and the connection could be established!", "Konfigurasjonen er i orden og tilkoblingen skal være etablert!");
        m.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Konfigurasjonen er i orden, men Bind mislyktes. Vennligst sjekk tjener-konfigurasjonen og påloggingsinformasjonen.");
        m.insert("Deletion failed", "Sletting mislyktes");
        m.insert("Take over settings from recent server configuration?", "Hent innstillinger fra tidligere tjener-konfigurasjon?");
        m.insert("Keep settings?", "Behold innstillinger?");
        m.insert("Cannot add server configuration", "Kan ikke legge til tjener-konfigurasjon");
        m.insert("Success", "Suksess");
        m.insert("Error", "Feil");
        m.insert("Select groups", "Velg grupper");
        m.insert("Connection test succeeded", "Tilkoblingstest lyktes");
        m.insert("Connection test failed", "Tilkoblingstest mislyktes");
        m.insert("Do you really want to delete the current Server Configuration?", "Er du sikker på at du vil slette aktiv tjener-konfigurasjon?");
        m.insert("Confirm Deletion", "Bekreft sletting");
        m.insert("Save", "Lagre");
        m.insert("Help", "Hjelp");
        m.insert("Add Server Configuration", "Legg til tjener-konfigurasjon");
        m.insert("Host", "Tjener");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Du kan utelate protokollen, men du er påkrevd å bruke SSL.  Deretter starte med ldaps://");
        m.insert("Port", "Port");
        m.insert("User DN", "Bruker DN");
        m.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "DN nummeret til klienten som skal bindes til, f.eks. uid=agent,dc=example,dc=com. For anonym tilgang, la DN- og passord-feltet stå tomt.");
        m.insert("Password", "Passord");
        m.insert("For anonymous access, leave DN and Password empty.", "For anonym tilgang, la DN- og passord-feltet stå tomt.");
        m.insert("One Base DN per line", "En hoved DN pr. linje");
        m.insert("You can specify Base DN for users and groups in the Advanced tab", "Du kan spesifisere Base DN for brukere og grupper under Avansert fanen");
        m.insert("Back", "Tilbake");
        m.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Warning:</b> PHP LDAP modulen er ikke installert, hjelperen vil ikke virke. Vennligst be din system-administrator om å installere den.");
        m.insert("Configuration Active", "Konfigurasjon aktiv");
        m.insert("When unchecked, this configuration will be skipped.", "Når ikke huket av så vil denne konfigurasjonen bli hoppet over.");
        m.insert("User Login Filter", "Brukerpålogging filter");
        m.insert("Backup (Replica) Host", "Sikkerhetskopierings (Replica) vert");
        m.insert("Case insensitve LDAP server (Windows)", "Case-insensitiv LDAP tjener (Windows)");
        m.insert("Turn off SSL certificate validation.", "Slå av SSL-sertifikat validering");
        m.insert("in seconds. A change empties the cache.", "i sekunder. En endring tømmer bufferen.");
        m.insert("User Display Name Field", "Vis brukerens navnfelt");
        m.insert("Base User Tree", "Hovedbruker tre");
        m.insert("One User Base DN per line", "En Bruker Base DN pr. linje");
        m.insert("Group Display Name Field", "Vis gruppens navnfelt");
        m.insert("Base Group Tree", "Hovedgruppe tre");
        m.insert("One Group Base DN per line", "En gruppe hoved-DN pr. linje");
        m.insert("Group-Member association", "gruppe-medlem assosiasjon");
        m.insert("in bytes", "i bytes");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "La stå tom for brukernavn (standard). Ellers, spesifiser en LDAP/AD attributt.");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, (String, String)> = {
        let mut m = HashMap::new();
        m.insert("_%s group found_::_%s groups found_", ("".to_string(), "".to_string()));
        m.insert("_%s user found_::_%s users found_", ("".to_string(), "".to_string()));
        m
    };
}

pub fn get_plural_form(n: i64) -> usize {
    if n != 1 { 1 } else { 0 }
}

pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}

pub fn translate_plural(key: &str, count: i64) -> String {
    if let Some((singular, plural)) = PLURAL_FORMS.get(key) {
        if get_plural_form(count) == 0 {
            return singular.clone();
        } else {
            return plural.clone();
        }
    }
    key.to_string()
}