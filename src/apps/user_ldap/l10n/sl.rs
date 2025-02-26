use std::collections::HashMap;
use lazy_static::lazy_static;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Failed to clear the mappings.", "Preslikav ni bilo mogoče izbrisati");
        m.insert("Failed to delete the server configuration", "Brisanje nastavitev strežnika je spodletelo.");
        m.insert("The configuration is valid and the connection could be established!", "Nastavitev je veljavna, zato je povezavo mogoče vzpostaviti!");
        m.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Nastavitev je veljavna, vendar pa je vez Bind spodletela. Preveriti je treba nastavitve strežnika in ustreznost poveril.");
        m.insert("Deletion failed", "Brisanje je spodletelo.");
        m.insert("Take over settings from recent server configuration?", "Ali naj se prevzame nastavitve nedavne nastavitve strežnika?");
        m.insert("Keep settings?", "Ali nas se nastavitve ohranijo?");
        m.insert("Cannot add server configuration", "Ni mogoče dodati nastavitev strežnika");
        m.insert("mappings cleared", "Preslikave so izbrisane");
        m.insert("Success", "Uspešno končano.");
        m.insert("Error", "Napaka");
        m.insert("Select groups", "Izberi skupine");
        m.insert("Connection test succeeded", "Preizkus povezave je uspešno končan.");
        m.insert("Connection test failed", "Preizkus povezave je spodletel.");
        m.insert("Do you really want to delete the current Server Configuration?", "Ali res želite izbrisati trenutne nastavitve strežnika?");
        m.insert("Confirm Deletion", "Potrdi brisanje");
        m.insert("_%s group found_::_%s groups found_", "");
        m.insert("_%s user found_::_%s users found_", "");
        m.insert("Save", "Shrani");
        m.insert("Test Configuration", "Preizkusne nastavitve");
        m.insert("Help", "Pomoč");
        m.insert("Add Server Configuration", "Dodaj nastavitve strežnika");
        m.insert("Host", "Gostitelj");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Protokol je lahko izpuščen, če ni posebej zahtevan SSL. V tem primeru se mora naslov začeti z ldaps://");
        m.insert("Port", "Vrata");
        m.insert("User DN", "Uporabnik DN");
        m.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "DN uporabnikovega odjemalca, s katerim naj se opravi vezava, npr. uid=agent,dc=example,dc=com. Za brezimni dostop sta polji DN in geslo prazni.");
        m.insert("Password", "Geslo");
        m.insert("For anonymous access, leave DN and Password empty.", "Za brezimni dostop sta polji DN in geslo prazni.");
        m.insert("One Base DN per line", "En osnovni DN na vrstico");
        m.insert("You can specify Base DN for users and groups in the Advanced tab", "Osnovni DN za uporabnike in skupine lahko določite v zavihku naprednih možnosti.");
        m.insert("Back", "Nazaj");
        m.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Opozorilo:</b> modul PHP LDAP mora biti nameščen, sicer vmesnik ne bo deloval. Paket je treba namestiti.");
        m.insert("Connection Settings", "Nastavitve povezave");
        m.insert("Configuration Active", "Dejavna nastavitev");
        m.insert("When unchecked, this configuration will be skipped.", "Neizbrana možnost preskoči nastavitev.");
        m.insert("User Login Filter", "Filter prijav uporabnikov");
        m.insert("Backup (Replica) Host", "Varnostna kopija (replika) podatkov gostitelja");
        m.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Podati je treba izbirno varnostno kopijo gostitelja. Ta mora biti natančna replika strežnika LDAP/AD.");
        m.insert("Backup (Replica) Port", "Varnostna kopija (replika) podatka vrat");
        m.insert("Disable Main Server", "Onemogoči glavni strežnik");
        m.insert("Case insensitve LDAP server (Windows)", "Strežnik LDAP ne upošteva velikosti črk (Windows)");
        m.insert("Turn off SSL certificate validation.", "Onemogoči določanje veljavnosti potrdila SSL.");
        m.insert("Cache Time-To-Live", "Predpomni podatke TTL");
        m.insert("in seconds. A change empties the cache.", "v sekundah. Sprememba izprazni predpomnilnik.");
        m.insert("Directory Settings", "Nastavitve mape");
        m.insert("User Display Name Field", "Polje za uporabnikovo prikazano ime");
        m.insert("Base User Tree", "Osnovno uporabniško drevo");
        m.insert("One User Base DN per line", "Eno osnovno uporabniško ime DN na vrstico");
        m.insert("User Search Attributes", "Uporabi atribute iskanja");
        m.insert("Optional; one attribute per line", "Izbirno; en atribut na vrstico");
        m.insert("Group Display Name Field", "Polje za prikazano ime skupine");
        m.insert("Base Group Tree", "Osnovno drevo skupine");
        m.insert("One Group Base DN per line", "Eno osnovno ime skupine DN na vrstico");
        m.insert("Group Search Attributes", "Atributi iskanja skupine");
        m.insert("Group-Member association", "Povezava član-skupina");
        m.insert("Special Attributes", "Posebni atributi");
        m.insert("Quota Field", "Polje količinske omejitve");
        m.insert("Quota Default", "Privzeta količinska omejitev");
        m.insert("in bytes", "v bajtih");
        m.insert("Email Field", "Polje elektronske pošte");
        m.insert("User Home Folder Naming Rule", "Pravila poimenovanja uporabniške osebne mape");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Pustite prazno za uporabniško ime (privzeto), sicer navedite atribut LDAP/AD.");
        m.insert("Internal Username", "Interno uporabniško ime");
        m.insert("Internal Username Attribute:", "Atribut Interno uporabniško ime");
        m.insert("Override UUID detection", "Prezri zaznavo UUID");
        m.insert("Username-LDAP User Mapping", "Preslikava uporabniško ime - LDAP-uporabnik");
        m.insert("Clear Username-LDAP User Mapping", "Izbriši preslikavo Uporabniškega imena in LDAP-uporabnika");
        m.insert("Clear Groupname-LDAP Group Mapping", "Izbriši preslikavo Skupine in LDAP-skupine");
        m
    };
}

pub fn plural_forms(n: i64) -> usize {
    if n % 100 == 1 {
        0
    } else if n % 100 == 2 {
        1
    } else if n % 100 == 3 || n % 100 == 4 {
        2
    } else {
        3
    }
}

// Función para obtener la traducción
pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

// Función para manejar plurales
pub fn get_plural_translation(key: &str, count: i64) -> &'static str {
    let plural_index = plural_forms(count);
    // En un entorno real, aquí habría más lógica para manejar los índices plurales
    get_translation(key)
}