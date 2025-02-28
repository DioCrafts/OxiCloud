use std::collections::HashMap;
use rust_i18n::t;

/// Estonian (Estonia) language translations
pub fn load_et_ee_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Failed to clear the mappings.".to_string(), "Vastendususte puhastamine ebaõnnestus.".to_string());
    translations.insert("Failed to delete the server configuration".to_string(), "Serveri seadistuse kustutamine ebaõnnestus".to_string());
    translations.insert("The configuration is valid and the connection could be established!".to_string(), "Seadistus on korrektne ning ühendus on olemas!".to_string());
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.".to_string(), "Seadistus on korrektne, kuid ühendus ebaõnnestus. Palun kontrolli serveri seadeid ja ühenduseks kasutatavaid kasutajatunnuseid.".to_string());
    translations.insert("The configuration is invalid. Please have a look at the logs for further details.".to_string(), "Seadistus on vigane. Lisainfot vaata palun logidest.".to_string());
    translations.insert("No action specified".to_string(), "Tegevusi pole määratletud".to_string());
    translations.insert("No configuration specified".to_string(), "Seadistust pole määratletud".to_string());
    translations.insert("No data specified".to_string(), "Andmeid pole määratletud".to_string());
    translations.insert(" Could not set configuration %s".to_string(), "Ei suutnud seadistada %s".to_string());
    translations.insert("Deletion failed".to_string(), "Kustutamine ebaõnnestus".to_string());
    translations.insert("Take over settings from recent server configuration?".to_string(), "Võta sätted viimasest serveri seadistusest?".to_string());
    translations.insert("Keep settings?".to_string(), "Säilitada seadistused?".to_string());
    translations.insert("Cannot add server configuration".to_string(), "Ei suuda lisada serveri seadistust".to_string());
    translations.insert("mappings cleared".to_string(), "vastendused puhastatud".to_string());
    translations.insert("Success".to_string(), "Korras".to_string());
    translations.insert("Error".to_string(), "Viga".to_string());
    translations.insert("Select groups".to_string(), "Vali grupid".to_string());
    translations.insert("Select object classes".to_string(), "Vali objekti klassid".to_string());
    translations.insert("Select attributes".to_string(), "Vali atribuudid".to_string());
    translations.insert("Connection test succeeded".to_string(), "Ühenduse testimine õnnestus".to_string());
    translations.insert("Connection test failed".to_string(), "Ühenduse testimine ebaõnnestus".to_string());
    translations.insert("Do you really want to delete the current Server Configuration?".to_string(), "Oled kindel, et tahad kustutada praegust serveri seadistust?".to_string());
    translations.insert("Confirm Deletion".to_string(), "Kinnita kustutamine".to_string());
    translations.insert("Invalid Host".to_string(), "Vigane server".to_string());
    translations.insert("Could not find the desired feature".to_string(), "Ei suuda leida soovitud funktsioonaalsust".to_string());
    translations.insert("Save".to_string(), "Salvesta".to_string());
    translations.insert("Test Configuration".to_string(), "Testi seadistust".to_string());
    translations.insert("Help".to_string(), "Abiinfo".to_string());
    translations.insert("Limit the access to %s to groups meeting this criteria:".to_string(), "Piira ligipääs %s grupile, mis sobivad kriteeriumiga:".to_string());
    translations.insert("only those object classes:".to_string(), "ainult need objektiklassid:".to_string());
    translations.insert("only from those groups:".to_string(), "ainult nendest gruppidest:".to_string());
    translations.insert("Edit raw filter instead".to_string(), "Selle asemel muuda filtrit".to_string());
    translations.insert("Raw LDAP filter".to_string(), "LDAP filter".to_string());
    translations.insert("The filter specifies which LDAP groups shall have access to the %s instance.".to_string(), "Filter määrab millised LDAP grupid saavad ligipääsu sellele %s instantsile.".to_string());
    translations.insert("groups found".to_string(), "gruppi leitud".to_string());
    translations.insert("What attribute shall be used as login name:".to_string(), "Mis atribuuti kasutada sisselogimise kasutajatunnusena:".to_string());
    translations.insert("LDAP Username:".to_string(), "LDAP kasutajanimi:".to_string());
    translations.insert("LDAP Email Address:".to_string(), "LDAP e-posti aadress:".to_string());
    translations.insert("Other Attributes:".to_string(), "Muud atribuudid:".to_string());
    translations.insert("Add Server Configuration".to_string(), "Lisa serveri seadistus".to_string());
    translations.insert("Host".to_string(), "Host".to_string());
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://".to_string(), "Sa ei saa protokolli ära jätta, välja arvatud siis, kui sa nõuad SSL-ühendust. Sel juhul alusta eesliitega ldaps://".to_string());
    translations.insert("Port".to_string(), "Port".to_string());
    translations.insert("User DN".to_string(), "Kasutaja DN".to_string());
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.".to_string(), "Klientkasutaja DN, kellega seotakse, nt. uid=agent,dc=näidis,dc=com. Anonüümseks ligipääsuks jäta DN ja parool tühjaks.".to_string());
    translations.insert("Password".to_string(), "Parool".to_string());
    translations.insert("For anonymous access, leave DN and Password empty.".to_string(), "Anonüümseks ligipääsuks jäta DN ja parool tühjaks.".to_string());
    translations.insert("One Base DN per line".to_string(), "Üks baas-DN rea kohta".to_string());
    translations.insert("You can specify Base DN for users and groups in the Advanced tab".to_string(), "Sa saad kasutajate ja gruppide baas DN-i määrata lisavalikute vahekaardilt".to_string());
    translations.insert("Limit the access to %s to users meeting this criteria:".to_string(), "Piira ligipääs %s kasutajale, kes sobivad kriteeriumiga:".to_string());
    translations.insert("The filter specifies which LDAP users shall have access to the %s instance.".to_string(), "Filter määrab millised LDAP kasutajad pääsevad ligi %s instantsile.".to_string());
    translations.insert("users found".to_string(), "kasutajat leitud".to_string());
    translations.insert("Back".to_string(), "Tagasi".to_string());
    translations.insert("Continue".to_string(), "Jätka".to_string());
    translations.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.".to_string(), "<b>Hoiatus:</b> rakendused user_ldap ja user_webdavauht ei ole ühilduvad. Töös võib esineda ootamatuid tõrkeid.\nPalu oma süsteemihalduril üks neist rakendustest kasutusest eemaldada.".to_string());
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.".to_string(), "<b>Hoiatus:</b>PHP LDAP moodul pole paigaldatud ning LDAP kasutamine ei ole võimalik. Palu oma süsteeihaldurit see paigaldada.".to_string());
    translations.insert("Connection Settings".to_string(), "Ühenduse seaded".to_string());
    translations.insert("Configuration Active".to_string(), "Seadistus aktiivne".to_string());
    translations.insert("When unchecked, this configuration will be skipped.".to_string(), "Kui on märkimata, siis seadistust ei kasutata.".to_string());
    translations.insert("User Login Filter".to_string(), "Kasutajanime filter".to_string());
    translations.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"".to_string(), "Määrab sisselogimisel kasutatava filtri. %%uid asendab sisselogimistegevuses kasutajanime. Näide: \"uid=%%uid\"".to_string());
    translations.insert("Backup (Replica) Host".to_string(), "Varuserver".to_string());
    translations.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.".to_string(), "Lisa valikuline varuserver. See peab olema koopia peamisest LDAP/AD serverist.".to_string());
    translations.insert("Backup (Replica) Port".to_string(), "Varuserveri (replika) port".to_string());
    translations.insert("Disable Main Server".to_string(), "Ära kasuta peaserverit".to_string());
    translations.insert("Only connect to the replica server.".to_string(), "Ühendu ainult replitseeriva serveriga.".to_string());
    translations.insert("Case insensitve LDAP server (Windows)".to_string(), "Mittetõstutundlik LDAP server (Windows)".to_string());
    translations.insert("Turn off SSL certificate validation.".to_string(), "Lülita SSL sertifikaadi kontrollimine välja.".to_string());
    translations.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.".to_string(), "Pole soovitatav, kasuta seda ainult testimiseks! Kui ühendus toimib ainult selle valikuga, siis impordi LDAP serveri SSL sertifikaat oma %s serverisse.".to_string());
    translations.insert("Cache Time-To-Live".to_string(), "Puhvri iga".to_string());
    translations.insert("in seconds. A change empties the cache.".to_string(), "sekundites. Muudatus tühjendab vahemälu.".to_string());
    translations.insert("Directory Settings".to_string(), "Kausta seaded".to_string());
    translations.insert("User Display Name Field".to_string(), "Kasutaja näidatava nime väli".to_string());
    translations.insert("The LDAP attribute to use to generate the user's display name.".to_string(), "LDAP atribuut, mida kasutatakse kasutaja kuvatava nime loomiseks.".to_string());
    translations.insert("Base User Tree".to_string(), "Baaskasutaja puu".to_string());
    translations.insert("One User Base DN per line".to_string(), "Üks kasutaja baas-DN rea kohta".to_string());
    translations.insert("User Search Attributes".to_string(), "Kasutaja otsingu atribuudid".to_string());
    translations.insert("Optional; one attribute per line".to_string(), "Valikuline; üks atribuut rea kohta".to_string());
    translations.insert("Group Display Name Field".to_string(), "Grupi näidatava nime väli".to_string());
    translations.insert("The LDAP attribute to use to generate the groups's display name.".to_string(), "LDAP atribuut, mida kasutatakse ownCloudi grupi kuvatava nime loomiseks.".to_string());
    translations.insert("Base Group Tree".to_string(), "Baasgrupi puu".to_string());
    translations.insert("One Group Base DN per line".to_string(), "Üks grupi baas-DN rea kohta".to_string());
    translations.insert("Group Search Attributes".to_string(), "Grupi otsingu atribuudid".to_string());
    translations.insert("Group-Member association".to_string(), "Grupiliikme seotus".to_string());
    translations.insert("Special Attributes".to_string(), "Spetsiifilised atribuudid".to_string());
    translations.insert("Quota Field".to_string(), "Mahupiirangu atribuut".to_string());
    translations.insert("Quota Default".to_string(), "Vaikimisi mahupiirang".to_string());
    translations.insert("in bytes".to_string(), "baitides".to_string());
    translations.insert("Email Field".to_string(), "E-posti väli".to_string());
    translations.insert("User Home Folder Naming Rule".to_string(), "Kasutaja kodukataloogi nimetamise reegel".to_string());
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.".to_string(), "Kasutajanime (vaikeväärtus) kasutamiseks jäta tühjaks. Vastasel juhul määra LDAP/AD omadus.".to_string());
    translations.insert("Internal Username".to_string(), "Sisemine kasutajanimi".to_string());
    translations.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.".to_string(), "Vaikimisi tekitatakse sisemine kasutajanimi UUID atribuudist. See tagab, et kasutajanimi on unikaalne ja sümboleid pole vaja muuta. Sisemisel kasutajatunnuse puhul on lubatud ainult järgmised sümbolid: [ a-zA-Z0-9_.@- ]. Muud sümbolid asendatakse nende ASCII vastega või lihtsalt hüljatakse. Tõrgete korral lisatakse number või suurendatakse seda. Sisemist kasutajatunnust kasutatakse kasutaja sisemiseks tuvastamiseks. Ühtlasi on see ownCloudis kasutaja vaikimisi kodukataloogi nimeks. See on ka serveri URLi osaks, näiteks kõikidel *DAV teenustel. Selle seadistusega saab tühistada vaikimisi käitumise. Saavutamaks sarnast käitumist eelnevate ownCloud 5 versioonidega, sisesta kasutaja kuvatava nime atribuut järgnevale väljale. Vaikimisi seadistuseks jäta tühjaks. Muudatused mõjutavad ainult uusi (lisatud) LDAP kasutajate vastendusi.".to_string());
    translations.insert("Internal Username Attribute:".to_string(), "Sisemise kasutajatunnuse atribuut:".to_string());
    translations.insert("Override UUID detection".to_string(), "Tühista UUID tuvastus".to_string());
    translations.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.".to_string(), "Vaikimis ownCloud tuvastab automaatselt UUID atribuudi. UUID atribuuti kasutatakse LDAP kasutajate ja gruppide kindlaks tuvastamiseks. Samuti tekitatakse sisemine kasutajanimi UUID alusel, kui pole määratud teisiti. Sa saad tühistada selle seadistuse ning määrata atribuudi omal valikul. Pead veenduma, et valitud atribuut toimib nii kasutajate kui gruppide puhul ning on unikaalne. Vaikimisi seadistuseks jäta tühjaks. Muudatused mõjutavad ainult uusi (lisatud) LDAP kasutajate vastendusi.".to_string());
    translations.insert("UUID Attribute for Users:".to_string(), "UUID atribuut kasutajatele:".to_string());
    translations.insert("UUID Attribute for Groups:".to_string(), "UUID atribuut gruppidele:".to_string());
    translations.insert("Username-LDAP User Mapping".to_string(), "LDAP-Kasutajatunnus Kasutaja Vastendus".to_string());
    translations.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.".to_string(), "ownCloud kasutab kasutajanime talletamaks ja omistamaks (pseudo) andmeid. Et täpselt tuvastada ja määratleda kasutajaid, peab iga LDAP kasutaja omama sisemist kasutajatunnust. See vajab ownCloud kasutajatunnuse vastendust LDAP kasutajaks. Tekitatud kasutajanimi vastendatakse LDAP kasutaja UUID-iks. Lisaks puhverdatakse DN vähendamaks LDAP päringuid, kuid seda ei kasutata tuvastamisel. ownCloud suudab tuvastada ka DN muutumise. ownCloud sisemist kasutajatunnust kasutatakse üle kogu ownCloudi. Eemaldates vastenduse tekivad kõikjal andmejäägid. Vastenduste eemaldamine ei ole konfiguratsiooni tundlik, see mõjutab kõiki LDAP seadistusi! Ära kunagi eemalda vastendusi produktsioonis! Seda võid teha ainult testis või katsetuste masinas.".to_string());
    translations.insert("Clear Username-LDAP User Mapping".to_string(), "Puhasta LDAP-Kasutajatunnus Kasutaja Vastendus".to_string());
    translations.insert("Clear Groupname-LDAP Group Mapping".to_string(), "Puhasta LDAP-Grupinimi Grupp Vastendus".to_string());
    
    translations
}

pub fn register_plural_rules() -> (&'static str, fn(n: f64) -> usize) {
    // nplurals=2; plural=(n != 1);
    ("et_EE", |n| if n != 1.0 { 1 } else { 0 })
}

pub fn get_plural_translations() -> HashMap<String, Vec<String>> {
    let mut plural_translations = HashMap::new();
    
    plural_translations.insert(
        "_%s group found_::_%s groups found_".to_string(), 
        vec!["%s grupp leitud".to_string(), "%s gruppi leitud".to_string()]
    );
    
    plural_translations.insert(
        "_%s user found_::_%s users found_".to_string(), 
        vec!["%s kasutaja leitud".to_string(), "%s kasutajat leitud".to_string()]
    );
    
    plural_translations
}