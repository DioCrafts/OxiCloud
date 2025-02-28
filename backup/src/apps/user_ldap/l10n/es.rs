use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

/// Translations for Spanish language
pub static ES_TRANSLATIONS: Lazy<RwLock<HashMap<&'static str, TranslationEntry>>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    
    translations.insert("Failed to clear the mappings.", TranslationEntry::String("Ocurrió un fallo al borrar las asignaciones."));
    translations.insert("Failed to delete the server configuration", TranslationEntry::String("No se pudo borrar la configuración del servidor"));
    translations.insert("The configuration is valid and the connection could be established!", TranslationEntry::String("¡La configuración es válida y la conexión puede establecerse!"));
    translations.insert("The configuration is valid, but the Bind failed. Please check the server settings and credentials.", TranslationEntry::String("La configuración es válida, pero falló el Enlace. Por favor, compruebe la configuración del servidor y las credenciales."));
    translations.insert("The configuration is invalid. Please have a look at the logs for further details.", TranslationEntry::String("La configuración no es válida. Por favor, busque en el log para más detalles."));
    translations.insert("No action specified", TranslationEntry::String("No se ha especificado la acción"));
    translations.insert("No configuration specified", TranslationEntry::String("No se ha especificado la configuración"));
    translations.insert("No data specified", TranslationEntry::String("No se han especificado los datos"));
    translations.insert(" Could not set configuration %s", TranslationEntry::String("No se pudo establecer la configuración %s"));
    translations.insert("Deletion failed", TranslationEntry::String("Falló el borrado"));
    translations.insert("Take over settings from recent server configuration?", TranslationEntry::String("¿Asumir los ajustes actuales de la configuración del servidor?"));
    translations.insert("Keep settings?", TranslationEntry::String("¿Mantener la configuración?"));
    translations.insert("Cannot add server configuration", TranslationEntry::String("No se puede añadir la configuración del servidor"));
    translations.insert("mappings cleared", TranslationEntry::String("Asignaciones borradas"));
    translations.insert("Success", TranslationEntry::String("Éxito"));
    translations.insert("Error", TranslationEntry::String("Error"));
    translations.insert("Select groups", TranslationEntry::String("Seleccionar grupos"));
    translations.insert("Select object classes", TranslationEntry::String("Seleccionar la clase de objeto"));
    translations.insert("Select attributes", TranslationEntry::String("Seleccionar atributos"));
    translations.insert("Connection test succeeded", TranslationEntry::String("La prueba de conexión fue exitosa"));
    translations.insert("Connection test failed", TranslationEntry::String("La prueba de conexión falló"));
    translations.insert("Do you really want to delete the current Server Configuration?", TranslationEntry::String("¿Realmente desea eliminar la configuración actual del servidor?"));
    translations.insert("Confirm Deletion", TranslationEntry::String("Confirmar eliminación"));
    translations.insert("_%s group found_::_%s groups found_", TranslationEntry::Plural(vec!["Grupo %s encontrado", "Grupos %s encontrados"]));
    translations.insert("_%s user found_::_%s users found_", TranslationEntry::Plural(vec!["Usuario %s encontrado", "Usuarios %s encontrados"]));
    translations.insert("Invalid Host", TranslationEntry::String("Host inválido"));
    translations.insert("Could not find the desired feature", TranslationEntry::String("No se puede encontrar la función deseada."));
    translations.insert("Save", TranslationEntry::String("Guardar"));
    translations.insert("Test Configuration", TranslationEntry::String("Configuración de prueba"));
    translations.insert("Help", TranslationEntry::String("Ayuda"));
    translations.insert("Limit the access to %s to groups meeting this criteria:", TranslationEntry::String("Limitar el acceso a %s a los grupos que cumplan este criterio:"));
    translations.insert("only those object classes:", TranslationEntry::String("solamente de estas clases de objeto:"));
    translations.insert("only from those groups:", TranslationEntry::String("solamente de estos grupos:"));
    translations.insert("Edit raw filter instead", TranslationEntry::String("Editar el filtro en bruto en su lugar"));
    translations.insert("Raw LDAP filter", TranslationEntry::String("Filtro LDAP en bruto"));
    translations.insert("The filter specifies which LDAP groups shall have access to the %s instance.", TranslationEntry::String("El filtro especifica que grupos LDAP tendrán acceso a %s."));
    translations.insert("groups found", TranslationEntry::String("grupos encontrados"));
    translations.insert("What attribute shall be used as login name:", TranslationEntry::String("Que atributo debe ser usado como login:"));
    translations.insert("LDAP Username:", TranslationEntry::String("Nombre de usuario LDAP:"));
    translations.insert("LDAP Email Address:", TranslationEntry::String("Dirección e-mail LDAP:"));
    translations.insert("Other Attributes:", TranslationEntry::String("Otros atributos:"));
    translations.insert("Add Server Configuration", TranslationEntry::String("Agregar configuracion del servidor"));
    translations.insert("Host", TranslationEntry::String("Servidor"));
    translations.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", TranslationEntry::String("Puede omitir el protocolo, excepto si requiere SSL. En ese caso, empiece con ldaps://"));
    translations.insert("Port", TranslationEntry::String("Puerto"));
    translations.insert("User DN", TranslationEntry::String("DN usuario"));
    translations.insert("The DN of the client user with which the bind shall be done, e.g. uid=agent,dc=example,dc=com. For anonymous access, leave DN and Password empty.", TranslationEntry::String("El DN del usuario cliente con el que se hará la asociación, p.ej. uid=agente,dc=ejemplo,dc=com. Para acceso anónimo, deje DN y contraseña vacíos."));
    translations.insert("Password", TranslationEntry::String("Contraseña"));
    translations.insert("For anonymous access, leave DN and Password empty.", TranslationEntry::String("Para acceso anónimo, deje DN y contraseña vacíos."));
    translations.insert("One Base DN per line", TranslationEntry::String("Un DN Base por línea"));
    translations.insert("You can specify Base DN for users and groups in the Advanced tab", TranslationEntry::String("Puede especificar el DN base para usuarios y grupos en la pestaña Avanzado"));
    translations.insert("Limit the access to %s to users meeting this criteria:", TranslationEntry::String("Limitar el acceso a %s a los usuarios que cumplan el siguiente criterio:"));
    translations.insert("The filter specifies which LDAP users shall have access to the %s instance.", TranslationEntry::String("El filtro especifica que usuarios LDAP pueden tener acceso a %s."));
    translations.insert("users found", TranslationEntry::String("usuarios encontrados"));
    translations.insert("Back", TranslationEntry::String("Atrás"));
    translations.insert("Continue", TranslationEntry::String("Continuar"));
    translations.insert("<b>Warning:</b> Apps user_ldap and user_webdavauth are incompatible. You may experience unexpected behavior. Please ask your system administrator to disable one of them.", TranslationEntry::String("<b>Advertencia:</b> Las apps user_ldap y user_webdavauth son incompatibles. Puede que experimente un comportamiento inesperado. Pregunte al su administrador de sistemas para desactivar uno de ellos."));
    translations.insert("<b>Warning:</b> The PHP LDAP module is not installed, the backend will not work. Please ask your system administrator to install it.", TranslationEntry::String("<b>Advertencia:</b> El módulo LDAP de PHP no está instalado, el sistema no funcionará. Por favor consulte al administrador del sistema para instalarlo."));
    translations.insert("Connection Settings", TranslationEntry::String("Configuración de conexión"));
    translations.insert("Configuration Active", TranslationEntry::String("Configuracion activa"));
    translations.insert("When unchecked, this configuration will be skipped.", TranslationEntry::String("Cuando deseleccione, esta configuracion sera omitida."));
    translations.insert("User Login Filter", TranslationEntry::String("Filtro de inicio de sesión de usuario"));
    translations.insert("Defines the filter to apply, when login is attempted. %%uid replaces the username in the login action. Example: \"uid=%%uid\"", TranslationEntry::String("Define el filtro a aplicar cuando se intenta identificar. %%uid remplazará al nombre de usuario en el proceso de identificación. Por ejemplo: \"uid=%%uid\""));
    translations.insert("Backup (Replica) Host", TranslationEntry::String("Servidor de copia de seguridad (Replica)"));
    translations.insert("Give an optional backup host. It must be a replica of the main LDAP/AD server.", TranslationEntry::String("Dar un servidor de copia de seguridad opcional. Debe ser una réplica del servidor principal LDAP / AD."));
    translations.insert("Backup (Replica) Port", TranslationEntry::String("Puerto para copias de seguridad (Replica)"));
    translations.insert("Disable Main Server", TranslationEntry::String("Deshabilitar servidor principal"));
    translations.insert("Only connect to the replica server.", TranslationEntry::String("Conectar sólo con el servidor de réplica."));
    translations.insert("Case insensitve LDAP server (Windows)", TranslationEntry::String("Servidor de LDAP no sensible a mayúsculas/minúsculas (Windows)"));
    translations.insert("Turn off SSL certificate validation.", TranslationEntry::String("Apagar la validación por certificado SSL."));
    translations.insert("Not recommended, use it for testing only! If connection only works with this option, import the LDAP server's SSL certificate in your %s server.", TranslationEntry::String("No se recomienda, ¡utilízalo únicamente para pruebas! Si la conexión únicamente funciona con esta opción, importa el certificado SSL del servidor LDAP en tu servidor %s."));
    translations.insert("Cache Time-To-Live", TranslationEntry::String("Cache TTL"));
    translations.insert("in seconds. A change empties the cache.", TranslationEntry::String("en segundos. Un cambio vacía la caché."));
    translations.insert("Directory Settings", TranslationEntry::String("Configuracion de directorio"));
    translations.insert("User Display Name Field", TranslationEntry::String("Campo de nombre de usuario a mostrar"));
    translations.insert("The LDAP attribute to use to generate the user's display name.", TranslationEntry::String("El campo LDAP a usar para generar el nombre para mostrar del usuario."));
    translations.insert("Base User Tree", TranslationEntry::String("Árbol base de usuario"));
    translations.insert("One User Base DN per line", TranslationEntry::String("Un DN Base de Usuario por línea"));
    translations.insert("User Search Attributes", TranslationEntry::String("Atributos de la busqueda de usuario"));
    translations.insert("Optional; one attribute per line", TranslationEntry::String("Opcional; un atributo por linea"));
    translations.insert("Group Display Name Field", TranslationEntry::String("Campo de nombre de grupo a mostrar"));
    translations.insert("The LDAP attribute to use to generate the groups's display name.", TranslationEntry::String("El campo LDAP a usar para generar el nombre para mostrar del grupo."));
    translations.insert("Base Group Tree", TranslationEntry::String("Árbol base de grupo"));
    translations.insert("One Group Base DN per line", TranslationEntry::String("Un DN Base de Grupo por línea"));
    translations.insert("Group Search Attributes", TranslationEntry::String("Atributos de busqueda de grupo"));
    translations.insert("Group-Member association", TranslationEntry::String("Asociación Grupo-Miembro"));
    translations.insert("Special Attributes", TranslationEntry::String("Atributos especiales"));
    translations.insert("Quota Field", TranslationEntry::String("Cuota"));
    translations.insert("Quota Default", TranslationEntry::String("Cuota por defecto"));
    translations.insert("in bytes", TranslationEntry::String("en bytes"));
    translations.insert("Email Field", TranslationEntry::String("E-mail"));
    translations.insert("User Home Folder Naming Rule", TranslationEntry::String("Regla para la carpeta Home de usuario"));
    translations.insert("Leave empty for user name (default). Otherwise, specify an LDAP/AD attribute.", TranslationEntry::String("Vacío para el nombre de usuario (por defecto). En otro caso, especifique un atributo LDAP/AD."));
    translations.insert("Internal Username", TranslationEntry::String("Nombre de usuario interno"));
    translations.insert("By default the internal username will be created from the UUID attribute. It makes sure that the username is unique and characters do not need to be converted. The internal username has the restriction that only these characters are allowed: [ a-zA-Z0-9_.@- ].  Other characters are replaced with their ASCII correspondence or simply omitted. On collisions a number will be added/increased. The internal username is used to identify a user internally. It is also the default name for the user home folder. It is also a part of remote URLs, for instance for all *DAV services. With this setting, the default behavior can be overridden. To achieve a similar behavior as before ownCloud 5 enter the user display name attribute in the following field. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users.", TranslationEntry::String("El nombre de usuario interno será creado de forma predeterminada desde el atributo UUID. Esto asegura que el nombre de usuario es único y los caracteres no necesitan ser convertidos. En el nombre de usuario interno sólo se pueden usar estos caracteres: [ a-zA-Z0-9_.@- ]. El resto de caracteres son sustituidos por su correspondiente en ASCII o simplemente omitidos. En caso de duplicidades, se añadirá o incrementará un número. El nombre de usuario interno es usado para identificar un usuario. Es también el nombre predeterminado para la carpeta personal del usuario en ownCloud. También es parte de URLs remotas, por ejemplo, para todos los servicios *DAV. Con esta configuración el comportamiento predeterminado puede ser cambiado. Para conseguir un comportamiento similar a como era antes de ownCloud 5, introduzca el campo del nombre para mostrar del usuario en la siguiente caja. Déjelo vacío para el comportamiento predeterminado. Los cambios solo tendrán efecto en los usuarios LDAP mapeados (añadidos) recientemente."));
    translations.insert("Internal Username Attribute:", TranslationEntry::String("Atributo Nombre de usuario Interno:"));
    translations.insert("Override UUID detection", TranslationEntry::String("Sobrescribir la detección UUID"));
    translations.insert("By default, the UUID attribute is automatically detected. The UUID attribute is used to doubtlessly identify LDAP users and groups. Also, the internal username will be created based on the UUID, if not specified otherwise above. You can override the setting and pass an attribute of your choice. You must make sure that the attribute of your choice can be fetched for both users and groups and it is unique. Leave it empty for default behavior. Changes will have effect only on newly mapped (added) LDAP users and groups.", TranslationEntry::String("Por defecto, el atributo UUID es autodetectado. Este atributo es usado para identificar indudablemente usuarios y grupos LDAP. Además, el nombre de usuario interno será creado en base al UUID, si no ha sido especificado otro comportamiento arriba. Puedes sobrescribir la configuración y pasar un atributo de tu elección. Debes asegurarte de que el atributo de tu elección sea accesible por los usuarios y grupos y ser único. Déjalo en blanco para usar el comportamiento por defecto. Los cambios tendrán efecto solo en los usuarios y grupos de LDAP mapeados (añadidos) recientemente."));
    translations.insert("UUID Attribute for Users:", TranslationEntry::String("Atributo UUID para usuarios:"));
    translations.insert("UUID Attribute for Groups:", TranslationEntry::String("Atributo UUID para Grupos:"));
    translations.insert("Username-LDAP User Mapping", TranslationEntry::String("Asignación del Nombre de usuario de un usuario LDAP"));
    translations.insert("Usernames are used to store and assign (meta) data. In order to precisely identify and recognize users, each LDAP user will have a internal username. This requires a mapping from username to LDAP user. The created username is mapped to the UUID of the LDAP user. Additionally the DN is cached as well to reduce LDAP interaction, but it is not used for identification. If the DN changes, the changes will be found. The internal username is used all over. Clearing the mappings will have leftovers everywhere. Clearing the mappings is not configuration sensitive, it affects all LDAP configurations! Never clear the mappings in a production environment, only in a testing or experimental stage.", TranslationEntry::String("Los usuarios son usados para almacenar y asignar (meta) datos. Con el fin de identificar de forma precisa y reconocer usuarios, cada usuario de LDAP tendrá un nombre de usuario interno. Esto requiere un mapeo entre el nombre de usuario y el usuario del LDAP. El nombre de usuario creado es mapeado respecto al UUID del usuario en el LDAP. De forma adicional, el DN es cacheado para reducir la interacción entre el LDAP, pero no es usado para identificar. Si el DN cambia, los cambios serán aplicados. El nombre de usuario interno es usado por encima de todo. Limpiar los mapeos dejará restos por todas partes, no es sensible a configuración, ¡afecta a todas las configuraciones del LDAP! Nunca limpies los mapeos en un entorno de producción, únicamente en una fase de desarrollo o experimental."));
    translations.insert("Clear Username-LDAP User Mapping", TranslationEntry::String("Borrar la asignación de los Nombres de usuario de los usuarios LDAP"));
    translations.insert("Clear Groupname-LDAP Group Mapping", TranslationEntry::String("Borrar la asignación de los Nombres de grupo de los grupos de LDAP"));
    
    RwLock::new(translations)
});

/// Represents a translation entry which can be either a string or a plural form
#[derive(Debug, Clone)]
pub enum TranslationEntry {
    String(&'static str),
    Plural(Vec<&'static str>),
}

/// Returns the plural form expression for Spanish language
pub fn get_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

/// Get a translation by key
pub fn get_translation(key: &str) -> Option<TranslationEntry> {
    ES_TRANSLATIONS.read().unwrap().get(key).cloned()
}

/// Get a formatted translation, replacing placeholders
pub fn format_translation(key: &str, args: &[&str]) -> Option<String> {
    if let Some(entry) = get_translation(key) {
        match entry {
            TranslationEntry::String(value) => {
                let mut result = value.to_string();
                for (i, arg) in args.iter().enumerate() {
                    result = result.replace(&format!("%s", i + 1), arg);
                    result = result.replace("%s", arg); // For the first replacement or if no index
                }
                Some(result)
            },
            TranslationEntry::Plural(forms) => {
                // This is simplified, actual plural form handling may be more complex
                if args.is_empty() {
                    return None;
                }
                let n = match args[0].parse::<i64>() {
                    Ok(n) => n,
                    Err(_) => return None,
                };
                
                let form_index = if n != 1 { 1 } else { 0 };
                if form_index < forms.len() {
                    let mut result = forms[form_index].to_string();
                    for (i, arg) in args.iter().enumerate() {
                        result = result.replace(&format!("%s", i + 1), arg);
                        result = result.replace("%s", arg);
                    }
                    Some(result)
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}