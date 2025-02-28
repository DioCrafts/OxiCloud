use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "La aplicación \"%s\" no puede ser instalada porque no es compatible con esta versión de ownCloud");
        m.insert("No app name specified", "No se ha especificado nombre de la aplicación");
        m.insert("Help", "Ayuda");
        m.insert("Personal", "Personal");
        m.insert("Settings", "Ajustes");
        m.insert("Users", "Usuarios");
        m.insert("Admin", "Administración");
        m.insert("Failed to upgrade \"%s\".", "Falló la actualización \"%s\".");
        m.insert("Unknown filetype", "Tipo de archivo desconocido");
        m.insert("Invalid image", "Imagen inválida");
        m.insert("web services under your control", "Servicios web bajo su control");
        m.insert("cannot open \"%s\"", "No se puede abrir \"%s\"");
        m.insert("ZIP download is turned off.", "La descarga en ZIP está desactivada.");
        m.insert("Files need to be downloaded one by one.", "Los archivos deben ser descargados uno por uno.");
        m.insert("Back to Files", "Volver a Archivos");
        m.insert("Selected files too large to generate zip file.", "Los archivos seleccionados son demasiado grandes para generar el archivo zip.");
        m.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Descargue los archivos en trozos más pequeños, por separado o solicítelos amablemente su administrador.");
        m.insert("No source specified when installing app", "No se ha especificado origen cuando se ha instalado la aplicación");
        m.insert("No href specified when installing app from http", "No href especificado cuando se ha instalado la aplicación");
        m.insert("No path specified when installing app from local file", "Sin path especificado  cuando se ha instalado la aplicación desde el fichero local");
        m.insert("Archives of type %s are not supported", "Ficheros de tipo %s no son soportados");
        m.insert("Failed to open archive when installing app", "Fallo de apertura de fichero mientras se instala la aplicación");
        m.insert("App does not provide an info.xml file", "La aplicación no suministra un fichero info.xml");
        m.insert("App can't be installed because of not allowed code in the App", "La aplicación no puede ser instalada por tener código no autorizado en la aplicación");
        m.insert("App can't be installed because it is not compatible with this version of ownCloud", "La aplicación no se puede instalar porque no es compatible con esta versión de ownCloud");
        m.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps", "La aplicación no se puede instalar porque contiene la etiqueta\n<shipped>\ntrue\n</shipped>\nque no está permitida para aplicaciones no distribuidas");
        m.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store", "La aplicación no puede ser instalada por que la versión en info.xml/version no es la misma que la establecida en la app store");
        m.insert("App directory already exists", "El directorio de la aplicación ya existe");
        m.insert("Can't create app folder. Please fix permissions. %s", "No se puede crear la carpeta de la aplicación. Corrija los permisos. %s");
        m.insert("Application is not enabled", "La aplicación no está habilitada");
        m.insert("Authentication error", "Error de autenticación");
        m.insert("Token expired. Please reload page.", "Token expirado. Por favor, recarga la página.");
        m.insert("Files", "Archivos");
        m.insert("Text", "Texto");
        m.insert("Images", "Imágenes");
        m.insert("%s enter the database username.", "%s ingresar el usuario de la base de datos.");
        m.insert("%s enter the database name.", "%s ingresar el nombre de la base de datos");
        m.insert("%s you may not use dots in the database name", "%s puede utilizar puntos en el nombre de la base de datos");
        m.insert("MS SQL username and/or password not valid: %s", "Usuario y/o contraseña de MS SQL no válidos: %s");
        m.insert("You need to enter either an existing account or the administrator.", "Tiene que ingresar una cuenta existente o la del administrador.");
        m.insert("MySQL username and/or password not valid", "Usuario y/o contraseña de MySQL no válidos");
        m.insert("DB Error: \"%s\"", "Error BD: \"%s\"");
        m.insert("Offending command was: \"%s\"", "Comando infractor: \"%s\"");
        m.insert("MySQL user '%s'@'localhost' exists already.", "Usuario MySQL '%s'@'localhost' ya existe.");
        m.insert("Drop this user from MySQL", "Eliminar este usuario de MySQL");
        m.insert("MySQL user '%s'@'%%' already exists", "Usuario MySQL '%s'@'%%' ya existe");
        m.insert("Drop this user from MySQL.", "Eliminar este usuario de MySQL.");
        m.insert("Oracle connection could not be established", "No se pudo establecer la conexión a Oracle");
        m.insert("Oracle username and/or password not valid", "Usuario y/o contraseña de Oracle no válidos");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "Comando infractor: \"%s\", nombre: %s, contraseña: %s");
        m.insert("PostgreSQL username and/or password not valid", "Usuario y/o contraseña de PostgreSQL no válidos");
        m.insert("Set an admin username.", "Configurar un nombre de usuario del administrador");
        m.insert("Set an admin password.", "Configurar la contraseña del administrador.");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Su servidor web aún no está configurado adecuadamente para permitir sincronización de archivos ya que la interfaz WebDAV parece no estar funcionando.");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "Por favor, vuelva a comprobar las <a href='%s'>guías de instalación</a>.");
        m.insert("Could not find category \"%s\"", "No puede encontrar la categoria \"%s\"");
        m.insert("seconds ago", "hace segundos");
        m.insert("today", "hoy");
        m.insert("yesterday", "ayer");
        m.insert("last month", "mes pasado");
        m.insert("last year", "año pasado");
        m.insert("years ago", "hace años");
        m.insert("Caused by:", "Causado por:");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["Hace %n minuto", "Hace %n minutos"]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["Hace %n hora", "Hace %n horas"]);
        m.insert("_%n day go_::_%n days ago_", vec!["Hace %n día", "Hace %n días"]);
        m.insert("_%n month ago_::_%n months ago_", vec!["Hace %n mes", "Hace %n meses"]);
        m
    };
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_translation(key: &str, count: usize) -> Option<&'static str> {
    PLURAL_TRANSLATIONS.get(key).and_then(|forms| {
        let index = if count != 1 { 1 } else { 0 };
        forms.get(index).copied()
    })
}