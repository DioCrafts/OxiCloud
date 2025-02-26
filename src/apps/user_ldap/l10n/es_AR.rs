use std::collections::HashMap;
use rust_i18n::t;

pub fn create_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Failed to clear the mappings.".to_string(), "Hubo un error al borrar las asignaciones.".to_string());
    translations.insert("Failed to delete the server configuration".to_string(), "Fallo al borrar la configuración del servidor".to_string());
    translations.insert("The configuration is valid and the connection could be established!".to_string(), "La configuración es válida y la conexión pudo ser establecida.".to_string());
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.".to_string(), "La configuración es válida, pero el enlace falló. Por favor, comprobá la configuración del servidor y las credenciales.".to_string());
    translations.insert("Deletion failed".to_string(), "Error al borrar".to_string());
    translations.insert("Take over settings from recent server configuration?".to_string(), "Tomar los valores de la anterior configuración de servidor?".to_string());
    translations.insert("Keep settings?".to_string(), "¿Mantener preferencias?".to_string());
    translations.insert("Cannot add server configuration".to_string(), "No se pudo añadir la configuración del servidor".to_string());
    translations.insert("mappings cleared".to_string(), "Asignaciones borradas".to_string());
    translations.insert("Success".to_string(), "Éxito".to_string());
    translations.insert("Error".to_string(), "Error".to_string());
    translations.insert("Select groups".to_string(), "Seleccionar grupos".to_string());
    translations.insert("Connection test succeeded".to_string(), "El este de conexión ha sido completado satisfactoriamente".to_string());
    translations.insert("Connection test failed".to_string(), "Falló es test de conexión".to_string());
    translations.insert("Do you really want to delete the current Server Configuration?".to_string(), "¿Realmente desea borrar la configuración actual del servidor?".to_string());
    translations.insert("Confirm Deletion".to_string(), "Confirmar borrado".to_string());
    translations.insert("Save".to_string(), "Guardar".to_string());
    translations.insert("Test Configuration".to_string(), "Probar configuración".to_string());
    translations.insert("Help".to_string(), "Ayuda".to_string());
    translations.insert("Add Server Configuration".to_string(), "Añadir Configuración del Servidor".to_string());
    translations.insert("Host".to_string(), "Servidor".to_string());
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://".to_string(), "Podés omitir el protocolo, excepto si SSL es requerido. En ese caso, empezá con ldaps://".to_string());
    translations.insert("Port".to_string(), "Puerto".to_string());
    translations.insert("User DN".to_string(), "DN usuario".to_string());
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.".to_string(), "El DN del usuario cliente con el que se hará la asociación, p.ej. uid=agente,dc=ejemplo,dc=com. Para acceso anónimo, dejá DN y contraseña vacíos.".to_string());
    translations.insert("Password".to_string(), "Contraseña".to_string());
    translations.insert("For anonymous access, leave DN and Password empty.".to_string(), "Para acceso anónimo, dejá DN y contraseña vacíos.".to_string());
    translations.insert("One Base DN per line".to_string(), "Una DN base por línea".to_string());
    translations.insert("You can specify Base DN for users and groups in the Advanced tab".to_string(), "Podés especificar el DN base para usuarios y grupos en la pestaña \"Avanzado\"".to_string());
    translations.insert("Back".to_string(), "Volver".to_string());
    translations.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.".to_string(), "<b>Advertencia:</b> Las apps user_ldap y user_webdavauth son incompatibles. Puede ser que experimentes comportamientos inesperados. Pedile al administrador que desactive uno de ellos.".to_string());
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.".to_string(), "<b>Atención:</b> El módulo PHP LDAP no está instalado, este elemento no va a funcionar. Por favor, pedile al administrador que lo instale.".to_string());
    translations.insert("Connection Settings".to_string(), "Configuración de Conección".to_string());
    translations.insert("Configuration Active".to_string(), "Configuración activa".to_string());
    translations.insert("When unchecked, this configuration will be skipped.".to_string(), "Si no está seleccionada, esta configuración será omitida.".to_string());
    translations.insert("User Login Filter".to_string(), "Filtro de inicio de sesión de usuario".to_string());
    translations.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"".to_string(), "Define el filtro a aplicar cuando se intenta ingresar. %%uid remplaza el nombre de usuario en el proceso de identificación. Por ejemplo: \"uid=%%uid\"".to_string());
    translations.insert("Backup (Replica) Host".to_string(), "Host para copia de seguridad (réplica)".to_string());
    translations.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.".to_string(), "Dar un servidor de copia de seguridad opcional. Debe ser una réplica del servidor principal LDAP/AD.".to_string());
    translations.insert("Backup (Replica) Port".to_string(), "Puerto para copia de seguridad (réplica)".to_string());
    translations.insert("Disable Main Server".to_string(), "Deshabilitar el Servidor Principal".to_string());
    translations.insert("Only connect to the replica server.".to_string(), "Conectarse únicamente al servidor de réplica.".to_string());
    translations.insert("Case insensitve LDAP server (Windows)".to_string(), "Servidor de LDAP sensible a mayúsculas/minúsculas (Windows)".to_string());
    translations.insert("Turn off SSL certificate validation.".to_string(), "Desactivar la validación por certificado SSL.".to_string());
    translations.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.".to_string(), "No es recomendado, ¡Usalo solamente para pruebas! Si la conexión únicamente funciona con esta opción, importá el certificado SSL del servidor LDAP en tu servidor %s.".to_string());
    translations.insert("Cache Time-To-Live".to_string(), "Tiempo de vida del caché".to_string());
    translations.insert("in seconds. A change empties the cache.".to_string(), "en segundos. Cambiarlo vacía la cache.".to_string());
    translations.insert("Directory Settings".to_string(), "Configuración de Directorio".to_string());
    translations.insert("User Display Name Field".to_string(), "Campo de nombre de usuario a mostrar".to_string());
    translations.insert("The LDAP attribute to use to generate the user's display name.".to_string(), "El atributo LDAP a usar para generar el nombre de usuario mostrado.".to_string());
    translations.insert("Base User Tree".to_string(), "Árbol base de usuario".to_string());
    translations.insert("One User Base DN per line".to_string(), "Una DN base de usuario por línea".to_string());
    translations.insert("User Search Attributes".to_string(), "Atributos de la búsqueda de usuario".to_string());
    translations.insert("Optional; one attribute per line".to_string(), "Opcional; un atributo por linea".to_string());
    translations.insert("Group Display Name Field".to_string(), "Campo de nombre de grupo a mostrar".to_string());
    translations.insert("The LDAP attribute to use to generate the groups's display name.".to_string(), "El atributo LDAP a usar para generar el nombre de grupo mostrado.".to_string());
    translations.insert("Base Group Tree".to_string(), "Árbol base de grupo".to_string());
    translations.insert("One Group Base DN per line".to_string(), "Una DN base de grupo por línea".to_string());
    translations.insert("Group Search Attributes".to_string(), "Atributos de búsqueda de grupo".to_string());
    translations.insert("Group-Member association".to_string(), "Asociación Grupo-Miembro".to_string());
    translations.insert("Special Attributes".to_string(), "Atributos Especiales".to_string());
    translations.insert("Quota Field".to_string(), "Campo de cuota".to_string());
    translations.insert("Quota Default".to_string(), "Cuota por defecto".to_string());
    translations.insert("in bytes".to_string(), "en bytes".to_string());
    translations.insert("Email Field".to_string(), "Campo de e-mail".to_string());
    translations.insert("User Home Folder Naming Rule".to_string(), "Regla de nombre de los directorios de usuario".to_string());
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.".to_string(), "Vacío para el nombre de usuario (por defecto). En otro caso, especificá un atributo LDAP/AD.".to_string());
    translations.insert("Internal Username".to_string(), "Nombre interno de usuario".to_string());
    translations.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.".to_string(), "Por defecto, el nombre de usuario interno es creado a partir del atributo UUID. Esto asegura que el nombre de usuario es único y no es necesaria una conversión de caracteres. El nombre de usuario interno sólo se pueden usar estos caracteres: [ a-zA-Z0-9_.@- ]. El resto de caracteres son sustituidos por su correspondiente en ASCII o simplemente omitidos. En caso colisiones, se agregará o incrementará un número. El nombre de usuario interno es usado para identificar un usuario. Es también el nombre predeterminado para el directorio personal del usuario en ownCloud. También es parte de las URLs remotas, por ejemplo, para los servicios *DAV. Con esta opción, se puede cambiar el comportamiento  por defecto. Para conseguir un comportamiento similar a versiones anteriores a ownCloud 5, ingresá el atributo del nombre mostrado en el campo siguiente. Dejalo vacío para el comportamiento por defecto. Los cambios solo tendrán efecto en los nuevos usuarios LDAP mapeados (agregados).".to_string());
    translations.insert("Internal Username Attribute:".to_string(), "Atributo Nombre Interno de usuario:".to_string());
    translations.insert("Override UUID detection".to_string(), "Sobrescribir la detección UUID".to_string());
    translations.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.".to_string(), "Por defecto, el atributo UUID es detectado automáticamente. Este atributo es usado para identificar de manera certera usuarios y grupos LDAP. Además, el nombre de usuario interno será creado en base al UUID, si no fue especificado otro comportamiento más arriba. Podés sobrescribir la configuración y pasar un atributo de tu elección. Tenés que asegurarte que el atributo de tu elección sea accesible por los usuarios y grupos y que sea único. Dejalo en blanco para usar el comportamiento por defecto. Los cambios tendrán efecto sólo en los nuevos usuarios y grupos de LDAP mapeados (agregados).".to_string());
    translations.insert("UUID Attribute for Users:".to_string(), "Atributo UUID para usuarios:".to_string());
    translations.insert("UUID Attribute for Groups:".to_string(), "Atributo UUID para grupos:".to_string());
    translations.insert("Username-LDAP User Mapping".to_string(), "Asignación del Nombre de usuario de un usuario LDAP".to_string());
    translations.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.".to_string(), "Los usuarios son usados para almacenar y asignar datos (metadatos). Con el fin de identificar de forma precisa y reconocer usuarios, a cada usuario de LDAP se será asignado un nombre de usuario interno. Esto requiere un mapeo entre el nombre de usuario y el usuario del LDAP. El nombre de usuario creado es mapeado respecto al UUID del usuario en el LDAP. De forma adicional, el DN es dejado en caché para reducir la interacción entre el LDAP, pero no es usado para la identificación. Si el DN cambia, los cambios van a ser aplicados. El nombre de usuario interno es usado en todos los lugares. Vaciar los mapeos, deja restos por todas partes. Vaciar los mapeos, no es sensible a configuración, ¡afecta a todas las configuraciones del LDAP! Nunca limpies los mapeos en un entorno de producción, solamente en fase de desarrollo o experimental.".to_string());
    translations.insert("Clear Username-LDAP User Mapping".to_string(), "Borrar la asignación de los Nombres de usuario de los usuarios LDAP".to_string());
    translations.insert("Clear Groupname-LDAP Group Mapping".to_string(), "Borrar la asignación de los Nombres de grupo de los grupos de LDAP".to_string());

    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Funciones para manejar traducciones plurales
pub fn translate_plural(msg_id: &str, count: i64) -> String {
    match msg_id {
        "_%s group found_::_%s groups found_" => {
            if count == 1 {
                format!("{} group found", count)
            } else {
                format!("{} groups found", count)
            }
        },
        "_%s user found_::_%s users found_" => {
            if count == 1 {
                format!("{} user found", count)
            } else {
                format!("{} users found", count)
            }
        },
        _ => msg_id.to_string()
    }
}

pub fn register_translations() {
    // Esta función sería llamada para registrar las traducciones en el framework i18n
    let translations = create_translations();
    // Aquí iría el código para registrar las traducciones con el sistema i18n
}