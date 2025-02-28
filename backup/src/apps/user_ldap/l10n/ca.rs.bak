use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Failed to clear the mappings.", "Ha fallat en eliminar els mapatges");
        map.insert("Failed to delete the server configuration", "Ha fallat en eliminar la configuració del servidor");
        map.insert("The configuration is valid and the connection could be established!", "La configuració és vàlida i s'ha pogut establir la comunicació!");
        map.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "La configuració és vàlida, però ha fallat el Bind. Comproveu les credencials i l'arranjament del servidor.");
        map.insert("The configuration is invalid. Please have a look at the logs for further details.", "La configuració no és vàlida. Per més detalls mireu al registre del sistema.");
        map.insert("No action specified", "No heu especificat cap acció");
        map.insert("No configuration specified", "No heu especificat cap configuració");
        map.insert("No data specified", "No heu especificat cap dada");
        map.insert(" Could not set configuration %s", "No s'ha pogut establir la configuració %s");
        map.insert("Deletion failed", "Eliminació fallida");
        map.insert("Take over settings from recent server configuration?", "Voleu prendre l'arranjament de la configuració actual del servidor?");
        map.insert("Keep settings?", "Voleu mantenir la configuració?");
        map.insert("Cannot add server configuration", "No es pot afegir la configuració del servidor");
        map.insert("mappings cleared", "s'han eliminat els mapatges");
        map.insert("Success", "Èxit");
        map.insert("Error", "Error");
        map.insert("Select groups", "Selecciona els grups");
        map.insert("Select object classes", "Seleccioneu les classes dels objectes");
        map.insert("Select attributes", "Seleccioneu els atributs");
        map.insert("Connection test succeeded", "La prova de connexió ha reeixit");
        map.insert("Connection test failed", "La prova de connexió ha fallat");
        map.insert("Do you really want to delete the current Server Configuration?", "Voleu eliminar la configuració actual del servidor?");
        map.insert("Confirm Deletion", "Confirma l'eliminació");
        map.insert("Invalid Host", "Ordinador central no vàlid");
        map.insert("Could not find the desired feature", "La característica desitjada no s'ha trobat");
        map.insert("Save", "Desa");
        map.insert("Test Configuration", "Comprovació de la configuració");
        map.insert("Help", "Ajuda");
        map.insert("Limit the access to %s to groups meeting this criteria:", "Limita l'accés a %s grups que compleixin amb el criteri:");
        map.insert("only those object classes:", "només aquestes classes d'objecte:");
        map.insert("only from those groups:", "només d'aquests grups");
        map.insert("Edit raw filter instead", "Edita filtre raw");
        map.insert("Raw LDAP filter", "Filtre raw LDAP");
        map.insert("The filter specifies which LDAP groups shall have access to the %s instance.", "El filtre especifica quins grups LDAP haurien de tenir accés a la instància %s.");
        map.insert("groups found", "grups trobats");
        map.insert("What attribute shall be used as login name:", "Quin atribut s'hauria d'utilitzar com a nom per a l'acreditació:");
        map.insert("LDAP Username:", "Nom d'usuari LDAP:");
        map.insert("LDAP Email Address:", "Adreça de correu electrònic LDAP:");
        map.insert("Other Attributes:", "Altres atributs:");
        map.insert("Add Server Configuration", "Afegeix la configuració del servidor");
        map.insert("Host", "Equip remot");
        map.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Podeu ometre el protocol, excepte si requeriu SSL. Llavors comenceu amb ldaps://");
        map.insert("Port", "Port");
        map.insert("User DN", "DN Usuari");
        map.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "La DN de l'usuari client amb la que s'haurà de fer, per exemple uid=agent,dc=exemple,dc=com. Per un accés anònim, deixeu la DN i la contrasenya en blanc.");
        map.insert("Password", "Contrasenya");
        map.insert("For anonymous access, leave DN and Password empty.", "Per un accés anònim, deixeu la DN i la contrasenya en blanc.");
        map.insert("One Base DN per line", "Una DN Base per línia");
        map.insert("You can specify Base DN for users and groups in the Advanced tab", "Podeu especificar DN Base per usuaris i grups a la pestanya Avançat");
        map.insert("Limit the access to %s to users meeting this criteria:", "Limita l'accés a %s usuaris que compleixin amb el criteri:");
        map.insert("The filter specifies which LDAP users shall have access to the %s instance.", "El filtre especifica quins usuaris LDAP haurien de tenir accés a la instància %s");
        map.insert("users found", "usuaris trobats");
        map.insert("Back", "Enrera");
        map.insert("Continue", "Continua");
        map.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.", "<b>Avís:</b> Les aplicacions user_ldap i user_webdavauth són incompatibles. Podeu experimentar comportaments inesperats. Demaneu a l'administrador del sistema que en desactivi una.");
        map.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Avís:</b> El mòdul PHP LDAP no està instal·lat, el dorsal no funcionarà. Demaneu a l'administrador del sistema que l'instal·li.");
        map.insert("Connection Settings", "Arranjaments de connexió");
        map.insert("Configuration Active", "Configuració activa");
        map.insert("When unchecked, this configuration will be skipped.", "Si està desmarcat, aquesta configuració s'ometrà.");
        map.insert("User Login Filter", "Filtre d'inici de sessió d'usuari");
        map.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"", "Defineix el filtre a aplicar quan s'intenta iniciar la sessió. %%uid reemplaça el nom d'usuari en l'acció d'inici de sessió. Per exemple: \"uid=%%uid\"");
        map.insert("Backup (Replica) Host", "Màquina de còpia de serguretat (rèplica)");
        map.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Afegiu una màquina de còpia de seguretat opcional. Ha de ser una rèplica del servidor LDAP/AD principal.");
        map.insert("Backup (Replica) Port", "Port de la còpia de seguretat (rèplica)");
        map.insert("Disable Main Server", "Desactiva el servidor principal");
        map.insert("Only connect to the replica server.", "Connecta només al servidor rèplica.");
        map.insert("Case insensitve LDAP server (Windows)", "Servidor LDAP sense distinció entre majúscules i minúscules (Windows)");
        map.insert("Turn off SSL certificate validation.", "Desactiva la validació de certificat SSL.");
        map.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.", "No es recomana, useu-ho només com a prova! Importeu el certificat SSL del servidor LDAP al servidor %s només si la connexió funciona amb aquesta opció.");
        map.insert("Cache Time-To-Live", "Memòria de cau Time-To-Live");
        map.insert("in seconds. A change empties the cache.", "en segons. Un canvi buidarà la memòria de cau.");
        map.insert("Directory Settings", "Arranjaments de carpetes");
        map.insert("User Display Name Field", "Camp per mostrar el nom d'usuari");
        map.insert("The LDAP attribute to use to generate the user's display name.", "Atribut LDAP a usar per generar el nom a mostrar de l'usuari.");
        map.insert("Base User Tree", "Arbre base d'usuaris");
        map.insert("One User Base DN per line", "Una DN Base d'Usuari per línia");
        map.insert("User Search Attributes", "Atributs de cerca d'usuari");
        map.insert("Optional; one attribute per line", "Opcional; Un atribut per línia");
        map.insert("Group Display Name Field", "Camp per mostrar el nom del grup");
        map.insert("The LDAP attribute to use to generate the groups's display name.", "Atribut LDAP a usar per generar el nom a mostrar del grup.");
        map.insert("Base Group Tree", "Arbre base de grups");
        map.insert("One Group Base DN per line", "Una DN Base de Grup per línia");
        map.insert("Group Search Attributes", "Atributs de cerca de grup");
        map.insert("Group-Member association", "Associació membres-grup");
        map.insert("Special Attributes", "Atributs especials");
        map.insert("Quota Field", "Camp de quota");
        map.insert("Quota Default", "Quota per defecte");
        map.insert("in bytes", "en bytes");
        map.insert("Email Field", "Camp de correu electrònic");
        map.insert("User Home Folder Naming Rule", "Norma per anomenar la carpeta arrel d'usuari");
        map.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Deixeu-ho buit pel nom d'usuari (per defecte). Altrament, especifiqueu un atribut LDAP/AD.");
        map.insert("Internal Username", "Nom d'usuari intern");
        map.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.", "Per defecte el nom d'usuari intern es crearà a partir de l'atribut UUID. Això assegura que el nom d'usuari és únic i que els caràcters no s'han de convertir. El nom d'usuari intern té la restricció que només estan permesos els caràcters: [ a-zA-Z0-9_.@- ]. Els altres caràcters es substitueixen pel seu corresponent ASCII o simplement s'ometen. En cas de col·lisió s'incrementa/decrementa en un. El nom d'usuari intern s'utilitza per identificar un usuari internament. També és el nom per defecte de la carpeta home d'usuari. És també un port de URLs remotes, per exemple tots els serveis *DAV. Amb aquest arranjament es pot variar el comportament per defecte. Per obtenir un comportament similar al d'abans de ownCloud 5, escriviu el nom d'usuari a mostrar en el camp següent. Deixei-lo en blanc si preferiu el comportament per defecte. Els canvis tindran efecte només en els nous usuaris LDAP mapats (afegits).");
        map.insert("Internal Username Attribute:", "Atribut nom d'usuari intern:");
        map.insert("Override UUID detection", "Sobrescriu la detecció UUID");
        map.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.", "Per defecte, owncloud autodetecta l'atribut UUID. L'atribut UUID s'utilitza per identificar usuaris i grups de forma indubtable. També el nom d'usuari intern es crearà en base a la UUIS, si no heu especificat res diferent a dalt. Podeu sobreescriure l'arranjament i passar l'atribut que desitgeu. Heu d'assegurar-vos que l'atribut que escolliu pot ser recollit tant pels usuaris com pels grups i que és únic. Deixeu-ho en blanc si preferiu el comportament per defecte. els canvis s'aplicaran als usuaris i grups LDAP mapats de nou (afegits).");
        map.insert("UUID Attribute for Users:", "Atribut UUID per Usuaris:");
        map.insert("UUID Attribute for Groups:", "Atribut UUID per Grups:");
        map.insert("Username-LDAP User Mapping", "Mapatge d'usuari Nom d'usuari-LDAP");
        map.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.", "Els noms d'usuari s'usen per desar i assignar (meta)dades. Per tal d'identificar amb precisió i reconèixer els usuaris, cada usuari LDAP tindrà un nom d'usuari intern. Això requereix mapatge del nom d'usuari a l'usuari LDAP. El nom d'usuari creat es mapa a la UUID de l'usuari LDAP. A més, la DN es posa a la memòria de cau per reduir la interacció LDAP, però no s'usa per identificació. En cas que la DN canvïi, els canvis es trobaran. El nom d'usuari intern s'usa a tot arreu. Si esborreu els mapatges quedaran sobrants a tot arreu. Esborrar els mapatges no és sensible a la configuració, afecta a totes les configuracions LDAP! No esborreu mai els mapatges en un entorn de producció, només en un estadi de prova o experimental.");
        map.insert("Clear Username-LDAP User Mapping", "Elimina el mapatge d'usuari Nom d'usuari-LDAP");
        map.insert("Clear Groupname-LDAP Group Mapping", "Elimina el mapatge de grup Nom de grup-LDAP");
        map
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}

pub fn plural(key: &str, count: usize) -> String {
    match key {
        "_%s group found_::_%s groups found_" => {
            if count == 1 {
                format!("S'ha trobat {} grup", count)
            } else {
                format!("S'han trobat {} grups", count)
            }
        },
        "_%s user found_::_%s users found_" => {
            if count == 1 {
                format!("S'ha trobat {} usuari", count)
            } else {
                format!("S'han trobat {} usuaris", count)
            }
        },
        _ => key.to_string(),
    }
}