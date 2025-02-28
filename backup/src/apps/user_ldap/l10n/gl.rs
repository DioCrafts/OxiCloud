use std::collections::HashMap;
use rust_i18n::t;

// Definición del módulo de idioma gallego
pub fn initialize() -> (HashMap<&'static str, &'static str>, &'static str) {
    let mut translations = HashMap::new();
    
    translations.insert("Failed to clear the mappings.", "Non foi posíbel limpar as asignacións.");
    translations.insert("Failed to delete the server configuration", "Non foi posíbel eliminar a configuración do servidor");
    translations.insert("The configuration is valid and the connection could be established!", "A configuración é correcta e pode estabelecerse a conexión.");
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", "A configuración é correcta, mais a ligazón non. Comprobe a configuración do servidor e as credenciais.");
    translations.insert("The configuration is invalid. Please have a look at the logs for further details.", "A configuración non é correcta. Vexa o rexistro de ownCloud para máis detalles");
    translations.insert("No action specified", "Non se especificou unha acción");
    translations.insert("No configuration specified", "Non se especificou unha configuración");
    translations.insert("No data specified", "Non se especificaron datos");
    translations.insert(" Could not set configuration %s", "Non foi posíbel estabelecer a configuración %s");
    translations.insert("Deletion failed", "Produciuse un fallo ao eliminar");
    translations.insert("Take over settings from recent server configuration?", "Tomar os recentes axustes de configuración do servidor?");
    translations.insert("Keep settings?", "Manter os axustes?");
    translations.insert("Cannot add server configuration", "Non é posíbel engadir a configuración do servidor");
    translations.insert("mappings cleared", "limpadas as asignacións");
    translations.insert("Success", "Correcto");
    translations.insert("Error", "Erro");
    translations.insert("Select groups", "Seleccionar grupos");
    translations.insert("Select object classes", "Seleccione as clases de obxectos");
    translations.insert("Select attributes", "Seleccione os atributos");
    translations.insert("Connection test succeeded", "A proba de conexión foi satisfactoria");
    translations.insert("Connection test failed", "A proba de conexión fracasou");
    translations.insert("Do you really want to delete the current Server Configuration?", "Confirma que quere eliminar a configuración actual do servidor?");
    translations.insert("Confirm Deletion", "Confirmar a eliminación");
    translations.insert("_%s group found_::_%s groups found_", "Atopouse %s grupo||Atopáronse %s grupos");
    translations.insert("_%s user found_::_%s users found_", "Atopouse %s usuario||Atopáronse %s usuarios");
    translations.insert("Invalid Host", "Máquina incorrecta");
    translations.insert("Could not find the desired feature", "Non foi posíbel atopar a función desexada");
    translations.insert("Save", "Gardar");
    translations.insert("Test Configuration", "Probar a configuración");
    translations.insert("Help", "Axuda");
    translations.insert("Limit the access to %s to groups meeting this criteria:", "Limitar o acceso a %s aos grupos que coincidan con estes criterios:");
    translations.insert("only those object classes:", "só as clases de obxecto:");
    translations.insert("only from those groups:", "só dos grupos:");
    translations.insert("Edit raw filter instead", "Editar, no seu canto, o filtro en bruto");
    translations.insert("Raw LDAP filter", "Filtro LDAP en bruto");
    translations.insert("The filter specifies which LDAP groups shall have access to the %s instance.", "O filtro especifica que grupos LDAP teñen acceso á instancia %s.");
    translations.insert("groups found", "atopáronse grupos");
    translations.insert("What attribute shall be used as login name:", "Atributo que utilizar como nome de usuario:");
    translations.insert("LDAP Username:", "Nome de usuario LDAP:");
    translations.insert("LDAP Email Address:", "Enderezo de correo LDAP:");
    translations.insert("Other Attributes:", "Outros atributos:");
    translations.insert("Add Server Configuration", "Engadir a configuración do servidor");
    translations.insert("Host", "Servidor");
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", "Pode omitir o protocolo agás que precise de SSL. Nese caso comece con ldaps://");
    translations.insert("Port", "Porto");
    translations.insert("User DN", "DN do usuario");
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", "O DN do cliente do usuario co que hai que estabelecer unha conexión, p.ex uid=axente, dc=exemplo, dc=com. Para o acceso anónimo deixe o DN e o contrasinal baleiros.");
    translations.insert("Password", "Contrasinal");
    translations.insert("For anonymous access, leave DN and Password empty.", "Para o acceso anónimo deixe o DN e o contrasinal baleiros.");
    translations.insert("One Base DN per line", "Un DN base por liña");
    translations.insert("You can specify Base DN for users and groups in the Advanced tab", "Pode especificar a DN base para usuarios e grupos na lapela de «Avanzado»");
    translations.insert("Limit the access to %s to users meeting this criteria:", "Limitar o acceso a %s aos usuarios que coincidan con estes criterios:");
    translations.insert("The filter specifies which LDAP users shall have access to the %s instance.", "O filtro especifica que usuarios LDAP teñen acceso á instancia %s.");
    translations.insert("users found", "atopáronse usuarios");
    translations.insert("Back", "Atrás");
    translations.insert("Continue", "Continuar");
    translations.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.", "<b>Aviso:</b> Os aplicativos user_ldap e user_webdavauth son incompatíbeis. Pode acontecer un comportamento estraño. Consulte co administrador do sistema para desactivar un deles.");
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", "<b>Aviso:</b> O módulo PHP LDAP non está instalado, o servidor non funcionará. Consulte co administrador do sistema para instalalo.");
    translations.insert("Connection Settings", "Axustes da conexión");
    translations.insert("Configuration Active", "Configuración activa");
    translations.insert("When unchecked, this configuration will be skipped.", "Se está sen marcar, omítese esta configuración.");
    translations.insert("User Login Filter", "Filtro de acceso de usuarios");
    translations.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"", "Define o filtro que se aplica cando se intenta o acceso. %%uid substitúe o nome de usuario e a acción de acceso. Exemplo: «uid=%%uid»");
    translations.insert("Backup (Replica) Host", "Servidor da copia de seguranza (Réplica)");
    translations.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", "Indicar un servidor de copia de seguranza opcional. Debe ser unha réplica do servidor principal LDAP/AD.");
    translations.insert("Backup (Replica) Port", "Porto da copia de seguranza (Réplica)");
    translations.insert("Disable Main Server", "Desactivar o servidor principal");
    translations.insert("Only connect to the replica server.", "Conectar só co servidor de réplica.");
    translations.insert("Case insensitve LDAP server (Windows)", "Servidor LDAP que non distingue entre maiúsculas e minúsculas (Windows)");
    translations.insert("Turn off SSL certificate validation.", "Desactiva a validación do certificado SSL.");
    translations.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.", "Non recomendado, utilizar só para probas! Se a conexión só funciona con esta opción importa o certificado SSL do servidor LDAP no seu servidor %s.");
    translations.insert("Cache Time-To-Live", "Tempo de persistencia da caché");
    translations.insert("in seconds. A change empties the cache.", "en segundos. Calquera cambio baleira a caché.");
    translations.insert("Directory Settings", "Axustes do directorio");
    translations.insert("User Display Name Field", "Campo de mostra do nome de usuario");
    translations.insert("The LDAP attribute to use to generate the user's display name.", "O atributo LDAP a empregar para xerar o nome de usuario para amosar.");
    translations.insert("Base User Tree", "Base da árbore de usuarios");
    translations.insert("One User Base DN per line", "Un DN base de usuario por liña");
    translations.insert("User Search Attributes", "Atributos de busca do usuario");
    translations.insert("Optional; one attribute per line", "Opcional; un atributo por liña");
    translations.insert("Group Display Name Field", "Campo de mostra do nome de grupo");
    translations.insert("The LDAP attribute to use to generate the groups's display name.", "O atributo LDAP úsase para xerar os nomes dos grupos que amosar.");
    translations.insert("Base Group Tree", "Base da árbore de grupo");
    translations.insert("One Group Base DN per line", "Un DN base de grupo por liña");
    translations.insert("Group Search Attributes", "Atributos de busca do grupo");
    translations.insert("Group-Member association", "Asociación de grupos e membros");
    translations.insert("Special Attributes", "Atributos especiais");
    translations.insert("Quota Field", "Campo de cota");
    translations.insert("Quota Default", "Cota predeterminada");
    translations.insert("in bytes", "en bytes");
    translations.insert("Email Field", "Campo do correo");
    translations.insert("User Home Folder Naming Rule", "Regra de nomeado do cartafol do usuario");
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", "Deixar baleiro para o nome de usuario (predeterminado). Noutro caso, especifique un atributo LDAP/AD.");
    translations.insert("Internal Username", "Nome de usuario interno");
    translations.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.", "De xeito predeterminado, o nome de usuario interno crease a partires do atributo UUID. Asegurase de que o nome de usuario é único e de non ter que converter os caracteres. O nome de usuario interno ten a limitación de que só están permitidos estes caracteres: [ a-zA-Z0-9_.@- ].  Os outros caracteres substitúense pola súa correspondencia ASCII ou simplemente omítense. Nas colisións engadirase/incrementarase un número. O nome de usuario interno utilizase para identificar a un usuario interno. É tamén o nome predeterminado do cartafol persoal do usuario. Tamén é parte dun URL remoto, por exemplo, para todos os servizos *DAV. Con este axuste, o comportamento predeterminado pode ser sobrescrito. Para lograr un comportamento semellante ao anterior ownCloud 5 introduza o atributo do nome para amosar do usuario no seguinte campo. Déixeo baleiro para o comportamento predeterminado. Os cambios terán efecto só nas novas asignacións (engadidos) de usuarios de LDAP.");
    translations.insert("Internal Username Attribute:", "Atributo do nome de usuario interno:");
    translations.insert("Override UUID detection", "Ignorar a detección do UUID");
    translations.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.", "De xeito predeterminado, o atributo UUID é detectado automaticamente. O atributo UUID utilizase para identificar, sen dúbida, aos usuarios e grupos LDAP. Ademais, crearase o usuario interno baseado no UUID, se non se especifica anteriormente o contrario. Pode anular a configuración e pasar un atributo da súa escolla. Vostede debe asegurarse de que o atributo da súa escolla pode ser recuperado polos usuarios e grupos e de que é único. Déixeo baleiro para o comportamento predeterminado. Os cambios terán efecto só nas novas asignacións (engadidos) de usuarios de LDAP.");
    translations.insert("UUID Attribute for Users:", "Atributo do UUID para usuarios:");
    translations.insert("UUID Attribute for Groups:", "Atributo do UUID para grupos:");
    translations.insert("Username-LDAP User Mapping", "Asignación do usuario ao «nome de usuario LDAP»");
    translations.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.", "Os nomes de usuario empreganse para almacenar e asignar (meta) datos. Coa fin de identificar con precisión e recoñecer aos usuarios, cada usuario LDAP terá un nome de usuario interno. Isto require unha asignación de ownCloud nome de usuario a usuario LDAP. O nome de usuario creado asignase ao UUID do usuario LDAP. Ademais o DN almacenase na caché, para así reducir a interacción do LDAP, mais non se utiliza para a identificación. Se o DN cambia, os cambios poden ser atopados polo ownCloud. O nome interno no ownCloud utilizase en todo o ownCloud. A limpeza das asignacións deixará rastros en todas partes. A limpeza das asignacións non é sensíbel á configuración, afecta a todas as configuracións de LDAP! Non limpar nunca as asignacións nun entorno de produción. Limpar as asignacións só en fases de proba ou experimentais.");
    translations.insert("Clear Username-LDAP User Mapping", "Limpar a asignación do usuario ao «nome de usuario LDAP»");
    translations.insert("Clear Groupname-LDAP Group Mapping", "Limpar a asignación do grupo ao «nome de grupo LDAP»");

    let plural_forms = "nplurals=2; plural=(n != 1);";
    
    (translations, plural_forms)
}

// Función auxiliar para obtener traducciones con plurales
pub fn get_plural_text(key: &str, count: i64) -> String {
    if key == "_%s group found_::_%s groups found_" {
        if count == 1 {
            format!("Atopouse {} grupo", count)
        } else {
            format!("Atopáronse {} grupos", count)
        }
    } else if key == "_%s user found_::_%s users found_" {
        if count == 1 {
            format!("Atopouse {} usuario", count)
        } else {
            format!("Atopáronse {} usuarios", count)
        }
    } else {
        String::new()
    }
}