use std::collections::HashMap;
use rust_i18n::t;

pub fn translations() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    
    map.insert("Failed to delete the server configuration", "Neizdevās izdzēst servera konfigurāciju");
    map.insert("The configuration is valid and the connection could be established!", "Konfigurācija ir derīga un varēja izveidot savienojumu!");
    map.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "Konfigurācija ir derīga, bet sasaiste neizdevās. Lūdzu, pārbaudiet servera iestatījumus un akreditācijas datus.");
    map.insert("Deletion failed", "Neizdevās izdzēst");
    map.insert("Take over settings from recent server configuration?", "Paņemt iestatījumus no nesenas servera konfigurācijas?");
    map.insert("Keep settings?", "Paturēt iestatījumus?");
    map.insert("Cannot add server configuration", "Nevar pievienot servera konfigurāciju");
    map.insert("Error", "Kļūda");
    map.insert("Select groups", "Izvēlieties grupas");
    map.insert("Connection test succeeded", "Savienojuma tests ir veiksmīgs");
    map.insert("Connection test failed", "Savienojuma tests cieta neveiksmi");
    map.insert("Do you really want to delete the current Server Configuration?", "Vai tiešām vēlaties dzēst pašreizējo servera konfigurāciju?");
    map.insert("Confirm Deletion", "Apstiprināt dzēšanu");
    map.insert("Save", "Saglabāt");
    map.insert("Test Configuration", "Testa konfigurācija");
    map.insert("Help", "Palīdzība");
    map.insert("Add Server Configuration", "Pievienot servera konfigurāciju");
    map.insert("Host", "Resursdators");
    map.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Var neiekļaut protokolu, izņemot, ja vajag SSL. Tad sākums ir ldaps://");
    map.insert("Port", "Ports");
    map.insert("User DN", "Lietotāja DN");
    map.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "Klienta lietotāja DN, ar ko veiks sasaisti, piemēram, uid=agent,dc=example,dc=com. Lai piekļūtu anonīmi, atstājiet DN un paroli tukšu.");
    map.insert("Password", "Parole");
    map.insert("For anonymous access, leave DN and Password empty.", "Lai piekļūtu anonīmi, atstājiet DN un paroli tukšu.");
    map.insert("One Base DN per line", "Viena bāzes DN rindā");
    map.insert("You can specify Base DN for users and groups in the Advanced tab", "Lietotājiem un grupām bāzes DN var norādīt cilnē "Paplašināti"");
    map.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Brīdinājums:</b> PHP LDAP modulis nav uzinstalēts, aizmugure nedarbosies. Lūdzu, prasiet savam sistēmas administratoram kādu no tām deaktivēt.");
    map.insert("Connection Settings", "Savienojuma iestatījumi");
    map.insert("Configuration Active", "Konfigurācija ir aktīva");
    map.insert("When unchecked, this configuration will be skipped.", "Ja nav atzīmēts, šī konfigurācija tiks izlaista.");
    map.insert("User Login Filter", "Lietotāja ierakstīšanās filtrs");
    map.insert("Backup (Replica) Host", "Rezerves (kopija) serveris");
    map.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Norādi rezerves serveri (nav obligāti). Tam ir jābūt galvenā LDAP/AD servera kopijai.");
    map.insert("Backup (Replica) Port", "Rezerves (kopijas) ports");
    map.insert("Disable Main Server", "Deaktivēt galveno serveri");
    map.insert("Case insensitve LDAP server (Windows)", "Reģistrnejutīgs LDAP serveris (Windows)");
    map.insert("Turn off SSL certificate validation.", "Izslēgt SSL sertifikātu validēšanu.");
    map.insert("Cache Time-To-Live", "Kešatmiņas dzīvlaiks");
    map.insert("in seconds. A change empties the cache.", "sekundēs. Izmaiņas iztukšos kešatmiņu.");
    map.insert("Directory Settings", "Direktorijas iestatījumi");
    map.insert("User Display Name Field", "Lietotāja redzamā vārda lauks");
    map.insert("Base User Tree", "Bāzes lietotāju koks");
    map.insert("One User Base DN per line", "Viena lietotāju bāzes DN rindā");
    map.insert("User Search Attributes", "Lietotāju meklēšanas atribūts");
    map.insert("Optional; one attribute per line", "Neobligāti; viens atribūts rindā");
    map.insert("Group Display Name Field", "Grupas redzamā nosaukuma lauks");
    map.insert("Base Group Tree", "Bāzes grupu koks");
    map.insert("One Group Base DN per line", "Viena grupu bāzes DN rindā");
    map.insert("Group Search Attributes", "Grupu meklēšanas atribūts");
    map.insert("Group-Member association", "Grupu piederības asociācija");
    map.insert("Special Attributes", "Īpašie atribūti");
    map.insert("Quota Field", "Kvotu lauks");
    map.insert("Quota Default", "Kvotas noklusējums");
    map.insert("in bytes", "baitos");
    map.insert("Email Field", "E-pasta lauks");
    map.insert("User Home Folder Naming Rule", "Lietotāja mājas mapes nosaukšanas kārtula");
    map.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Atstāt tukšu lietotāja vārdam (noklusējuma). Citādi, norādi LDAP/AD atribūtu.");
    
    map
}

pub fn plural_forms() -> &'static str {
    "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n != 0 ? 1 : 2);"
}

pub fn get_plural_form(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n != 0 {
        1
    } else {
        2
    }
}

pub fn translate_plurals(msg_id: &str, count: i64) -> String {
    match msg_id {
        "%s group found" => {
            let forms = ["", "", ""];
            forms[get_plural_form(count)].to_string()
        },
        "%s user found" => {
            let forms = ["", "", ""];
            forms[get_plural_form(count)].to_string()
        },
        _ => String::new(),
    }
}