use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Failed to clear the mappings.", "dështoi së pastruari planifikimet");
        m.insert("Failed to delete the server configuration", "dështoi fshirjen e konfigurimit të serverit");
        m.insert("The configuration is valid and the connection could be established!", "Konfigurimi është i vlefshem dhe lidhja mund të kryhet");
        m.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Konfigurimi është i saktë por lidhja dështoi. Kontrolloni konfigurimete serverit dhe kredencialet.");
        m.insert("Deletion failed", "Fshirja dështoi");
        m.insert("Take over settings from recent server configuration?", "Doni të rivini konfigurmet më të fundit të serverit?");
        m.insert("Keep settings?", "Doni të mbani konfigurimet?");
        m.insert("Cannot add server configuration", "E pamundur të shtohen konfigurimet në server");
        m.insert("mappings cleared", "planifikimi u fshi");
        m.insert("Success", "Sukses");
        m.insert("Error", "Gabim");
        m.insert("Connection test succeeded", "Prova e lidhjes përfundoi me sukses");
        m.insert("Connection test failed", "Prova e lidhjes dështoi");
        m.insert("Do you really want to delete the current Server Configuration?", "Jeni vërtetë të sigurt të fshini konfigurimet aktuale të serverit?");
        m.insert("Confirm Deletion", "Konfirmoni Fshirjen");
        m.insert("Save", "Ruaj");
        m.insert("Test Configuration", "Provoni konfigurimet");
        m.insert("Help", "Ndihmë");
        m.insert("Add Server Configuration", "Shtoni konfigurimet e serverit");
        m.insert("Host", "Pritësi");
        m.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Ju mund të mos vendosni protokollin ,vetëm nëse ju nevojitet SSL. atherë filloni me ldaps://");
        m.insert("Port", "Porta");
        m.insert("User DN", "Përdoruesi DN");
        m.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "DN -ja e klientit për përdoruesin që kërkon të lidhet duhet të jetë si psh,uid=agent,dc=example,dc=com. Për lidhjet anonime lini boshe hapsirat e DN dhe fjalëkalim ");
        m.insert("Password", "fjalëkalim");
        m.insert("For anonymous access, leave DN and Password empty.", "Për tu lidhur në mënyre anonime, lini bosh hapsirat e DN dhe fjalëkalim");
        m.insert("One Base DN per line", "Një baze DN për rrjesht");
        m.insert("You can specify Base DN for users and groups in the Advanced tab", "Ju mund të specifikoni Bazen DN për përdorues dhe grupe në butonin 'Të Përparuara'");
        m.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Njoftim:</b> moduli PHP LDAP nuk është instaluar, motori nuk do të funksionojë.Kontaktoni me administratorin e sistemit.");
        m.insert("Connection Settings", "Të dhënat e lidhjes");
        m.insert("Configuration Active", "Konfigurimi Aktiv");
        m.insert("When unchecked, this configuration will be skipped.", "Nëse nuk është i zgjedhur, ky konfigurim do të anashkalohet.");
        m.insert("User Login Filter", "Filtri për aksesin e përdoruesit");
        m.insert("Backup (Replica) Host", "Pritësi rezervë (Replika)");
        m.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Jepni një pritës rezervë. Duhet të jetë replikimi i serverit AD/LDAP kryesor.");
        m.insert("Backup (Replica) Port", "Porta rezervë (Replika)");
        m.insert("Disable Main Server", "Ç'aktivizoni serverin kryesor");
        m.insert("Case insensitve LDAP server (Windows)", " Server LDAP i pavëmëndshëm ndaj gërmëzimit të madh apo jo (Windows)");
        m.insert("Turn off SSL certificate validation.", "Ç'aktivizoni kontrollin e certifikatës SSL.");
        m.insert("Cache Time-To-Live", "Cache Time-To-Live");
        m.insert("in seconds. A change empties the cache.", "në sekonda Ndryshimi boshatis 'cache'-n.");
        m.insert("Directory Settings", "Konfigurimet e Dosjeve");
        m.insert("User Display Name Field", "Hapsira e Emrit të Përdoruesit");
        m.insert("Base User Tree", "Struktura bazë e përdoruesit");
        m.insert("One User Base DN per line", "Një përdorues baze DN për rrjesht");
        m.insert("User Search Attributes", "Atributet e kërkimit të përdoruesëve");
        m.insert("Optional; one attribute per line", "Opsionale; një atribut për rrjesht");
        m.insert("Group Display Name Field", "Hapsira e Emrit të Grupit");
        m.insert("Base Group Tree", "Struktura bazë e grupit");
        m.insert("One Group Base DN per line", "Një grup baze DN për rrjesht");
        m.insert("Group Search Attributes", "Atributet e kërkimit të grupit");
        m.insert("Group-Member association", "Pjestar Grup-Përdorues ");
        m.insert("Special Attributes", "Atribute të veçanta");
        m.insert("Quota Field", "Hapsira e Kuotës");
        m.insert("Quota Default", "Kuota e paracaktuar");
        m.insert("in bytes", "në byte");
        m.insert("Email Field", "Hapsira e Postës Elektronike");
        m.insert("User Home Folder Naming Rule", "Rregulli i emërimit të dosjes së përdoruesit");
        m.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Lëreni bosh për emrin e përdoruesit (I Paracaktuar). Ose, përcaktoni një atribut LDAP/AD.");
        m.insert("Internal Username", "Emër i brëndshëm i përdoruesit");
        m.insert("Internal Username Attribute:", "Atributet e emrit të përdoruesit të brëndshëm");
        m.insert("Override UUID detection", "Mbivendosni gjetjen e UUID");
        m.insert("Username-LDAP User Mapping", "Emri përdoruesit-LAPD përcaktues përdoruesi");
        m.insert("Clear Username-LDAP User Mapping", "Fshini Emër përdoruesi-LAPD Përcaktues përdoruesi");
        m.insert("Clear Groupname-LDAP Group Mapping", "Fshini Emër Grupi-LADP Përcaktues grupi");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn init_translations() {
    // Registrar traducciones para cadenas plurales
    rust_i18n::set_locale("sq");
    rust_i18n::add_translation("sq", "_%s group found_::_%s groups found_", |n| {
        if n == 1 {
            format!("{} group found", n)
        } else {
            format!("{} groups found", n)
        }
    });
    
    rust_i18n::add_translation("sq", "_%s user found_::_%s users found_", |n| {
        if n == 1 {
            format!("{} user found", n)
        } else {
            format!("{} users found", n)
        }
    });
}

// Función auxiliar para obtener las traducciones
pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(&key)
}