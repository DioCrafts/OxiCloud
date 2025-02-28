use std::collections::HashMap;
use rust_i18n::t;

pub fn register_translations() -> HashMap<String, String> {
    let mut translations: HashMap<String, String> = HashMap::new();
    
    translations.insert("Failed to delete the server configuration".to_string(), "Zerbitzariaren konfigurazioa ezabatzeak huts egin du".to_string());
    translations.insert("The configuration is valid and the connection could be established!".to_string(), "Konfigurazioa egokia da eta konexioa ezarri daiteke!".to_string());
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.".to_string(), "Konfigurazioa ongi dago, baina Bind-ek huts egin du. Mesedez egiaztatu zerbitzariaren ezarpenak eta kredentzialak.".to_string());
    translations.insert("Deletion failed".to_string(), "Ezabaketak huts egin du".to_string());
    translations.insert("Take over settings from recent server configuration?".to_string(), "oraintsuko zerbitzariaren konfigurazioaren ezarpenen ardura hartu?".to_string());
    translations.insert("Keep settings?".to_string(), "Mantendu ezarpenak?".to_string());
    translations.insert("Cannot add server configuration".to_string(), "Ezin da zerbitzariaren konfigurazioa gehitu".to_string());
    translations.insert("Success".to_string(), "Arrakasta".to_string());
    translations.insert("Error".to_string(), "Errorea".to_string());
    translations.insert("Select groups".to_string(), "Hautatu taldeak".to_string());
    translations.insert("Connection test succeeded".to_string(), "Konexio froga ongi burutu da".to_string());
    translations.insert("Connection test failed".to_string(), "Konexio frogak huts egin du".to_string());
    translations.insert("Do you really want to delete the current Server Configuration?".to_string(), "Ziur zaude Zerbitzariaren Konfigurazioa ezabatu nahi duzula?".to_string());
    translations.insert("Confirm Deletion".to_string(), "Baieztatu Ezabatzea".to_string());
    // Plural forms are handled separately
    translations.insert("Save".to_string(), "Gorde".to_string());
    translations.insert("Test Configuration".to_string(), "Egiaztatu Konfigurazioa".to_string());
    translations.insert("Help".to_string(), "Laguntza".to_string());
    translations.insert("Add Server Configuration".to_string(), "Gehitu Zerbitzariaren Konfigurazioa".to_string());
    translations.insert("Host".to_string(), "Hostalaria".to_string());
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://".to_string(), "Protokoloa ez da beharrezkoa, SSL behar baldin ez baduzu. Honela bada hasi ldaps://".to_string());
    translations.insert("Port".to_string(), "Portua".to_string());
    translations.insert("User DN".to_string(), "Erabiltzaile DN".to_string());
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.".to_string(), "Lotura egingo den bezero erabiltzailearen DNa, adb. uid=agent,dc=example,dc=com. Sarrera anonimoak gaitzeko utzi DN eta Pasahitza hutsik.".to_string());
    translations.insert("Password".to_string(), "Pasahitza".to_string());
    translations.insert("For anonymous access, leave DN and Password empty.".to_string(), "Sarrera anonimoak gaitzeko utzi DN eta Pasahitza hutsik.".to_string());
    translations.insert("One Base DN per line".to_string(), "DN Oinarri bat lerroko".to_string());
    translations.insert("You can specify Base DN for users and groups in the Advanced tab".to_string(), "Erabiltzaile eta taldeentzako Oinarrizko DN zehaztu dezakezu Aurreratu fitxan".to_string());
    translations.insert("Back".to_string(), "Atzera".to_string());
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.".to_string(), "<b>Abisua:</b> PHPk behar duen LDAP modulua ez dago instalaturik, motorrak ez du funtzionatuko. Mesedez eskatu zure sistema kudeatzaileari instala dezan.".to_string());
    translations.insert("Connection Settings".to_string(), "Konexio Ezarpenak".to_string());
    translations.insert("Configuration Active".to_string(), "Konfigurazio Aktiboa".to_string());
    translations.insert("When unchecked, this configuration will be skipped.".to_string(), "Markatuta ez dagoenean, konfigurazio hau ez da kontutan hartuko.".to_string());
    translations.insert("User Login Filter".to_string(), "Erabiltzaileen saioa hasteko iragazkia".to_string());
    translations.insert("Backup (Replica) Host".to_string(), "Babeskopia (Replica) Ostalaria".to_string());
    translations.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.".to_string(), "Eman babeskopia ostalari gehigarri bat. LDAP/AD zerbitzari nagusiaren replica bat izan behar da.".to_string());
    translations.insert("Backup (Replica) Port".to_string(), "Babeskopia (Replica) Ataka".to_string());
    translations.insert("Disable Main Server".to_string(), "Desgaitu Zerbitzari Nagusia".to_string());
    translations.insert("Case insensitve LDAP server (Windows)".to_string(), "Maiuskulak eta minuskulak ezberditzen ez dituen LDAP zerbitzaria (windows)".to_string());
    translations.insert("Turn off SSL certificate validation.".to_string(), "Ezgaitu SSL ziurtagirien egiaztapena.".to_string());
    translations.insert("Cache Time-To-Live".to_string(), "Katxearen Bizi-Iraupena".to_string());
    translations.insert("in seconds. A change empties the cache.".to_string(), "segundutan. Aldaketak katxea husten du.".to_string());
    translations.insert("Directory Settings".to_string(), "Karpetaren Ezarpenak".to_string());
    translations.insert("User Display Name Field".to_string(), "Erabiltzaileen bistaratzeko izena duen eremua".to_string());
    translations.insert("Base User Tree".to_string(), "Oinarrizko Erabiltzaile Zuhaitza".to_string());
    translations.insert("One User Base DN per line".to_string(), "Erabiltzaile DN Oinarri bat lerroko".to_string());
    translations.insert("User Search Attributes".to_string(), "Erabili Bilaketa Atributuak ".to_string());
    translations.insert("Optional; one attribute per line".to_string(), "Aukerakoa; atributu bat lerro bakoitzeko".to_string());
    translations.insert("Group Display Name Field".to_string(), "Taldeen bistaratzeko izena duen eremua".to_string());
    translations.insert("Base Group Tree".to_string(), "Oinarrizko Talde Zuhaitza".to_string());
    translations.insert("One Group Base DN per line".to_string(), "Talde DN Oinarri bat lerroko".to_string());
    translations.insert("Group Search Attributes".to_string(), "Taldekatu Bilaketa Atributuak ".to_string());
    translations.insert("Group-Member association".to_string(), "Talde-Kide elkarketak".to_string());
    translations.insert("Special Attributes".to_string(), "Atributu Bereziak".to_string());
    translations.insert("Quota Field".to_string(), "Kuota Eremua".to_string());
    translations.insert("Quota Default".to_string(), "Kuota Lehenetsia".to_string());
    translations.insert("in bytes".to_string(), "bytetan".to_string());
    translations.insert("Email Field".to_string(), "Eposta eremua".to_string());
    translations.insert("User Home Folder Naming Rule".to_string(), "Erabiltzailearen Karpeta Nagusia Izendatzeko Patroia".to_string());
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.".to_string(), "Utzi hutsik erabiltzaile izenarako (lehentsia). Bestela zehaztu LDAP/AD atributua.".to_string());
    translations.insert("Internal Username".to_string(), "Barneko erabiltzaile izena".to_string());
    
    translations
}

pub fn register_plurals() -> HashMap<String, Vec<String>> {
    let mut plurals: HashMap<String, Vec<String>> = HashMap::new();
    
    plurals.insert(
        "_%s group found_::_%s groups found_".to_string(), 
        vec!["".to_string(), "".to_string()]
    );
    
    plurals.insert(
        "_%s user found_::_%s users found_".to_string(), 
        vec!["".to_string(), "".to_string()]
    );
    
    plurals
}

pub fn get_plural_form() -> String {
    "nplurals=2; plural=(n != 1);".to_string()
}

// Register translations with the i18n system
pub fn init_translations() {
    let translations = register_translations();
    let plurals = register_plurals();
    let plural_form = get_plural_form();
    
    // This part would integrate with an actual i18n system
    // For example, a hypothetical rust_i18n library might have API like:
    // rust_i18n::register_locale("eu", translations, plurals, plural_form);
}