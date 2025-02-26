use std::collections::HashMap;
use rust_fluent::FluentBundle;
use unic_langid::langid;

pub struct EsAr;

impl EsAr {
    pub fn get_translations() -> HashMap<&'static str, &'static str> {
        let mut translations = HashMap::new();
        
        translations.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "La app \"%s\" no puede ser instalada porque no es compatible con esta versión de ownCloud");
        translations.insert("No app name specified", "No fue especificado el nombre de la app");
        translations.insert("Help", "Ayuda");
        translations.insert("Personal", "Personal");
        translations.insert("Settings", "Configuración");
        translations.insert("Users", "Usuarios");
        translations.insert("Admin", "Administración");
        translations.insert("Failed to upgrade \"%s\".", "No se pudo actualizar \"%s\".");
        translations.insert("Unknown filetype", "Tipo de archivo desconocido");
        translations.insert("Invalid image", "Imagen inválida");
        translations.insert("web services under your control", "servicios web sobre los que tenés control");
        translations.insert("cannot open \"%s\"", "no se puede abrir \"%s\"");
        translations.insert("ZIP download is turned off.", "La descarga en ZIP está desactivada.");
        translations.insert("Files need to be downloaded one by one.", "Los archivos deben ser descargados de a uno.");
        translations.insert("Back to Files", "Volver a Archivos");
        translations.insert("Selected files too large to generate zip file.", "Los archivos seleccionados son demasiado grandes para generar el archivo zip.");
        translations.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Descargá los archivos en partes más chicas, de forma separada, o pedíselos al administrador");
        translations.insert("No source specified when installing app", "No se especificó el origen al instalar la app");
        translations.insert("No href specified when installing app from http", "No se especificó href al instalar la app");
        translations.insert("No path specified when installing app from local file", "No se especificó PATH al instalar la app desde el archivo local");
        translations.insert("Archives of type %s are not supported", "No hay soporte para archivos de tipo %s");
        translations.insert("Failed to open archive when installing app", "Error al abrir archivo mientras se instalaba la app");
        translations.insert("App does not provide an info.xml file", "La app no suministra un archivo info.xml");
        translations.insert("App can't be installed because of not allowed code in the App", "No puede ser instalada la app por tener código no autorizado");
        translations.insert("App can't be installed because it is not compatible with this version of ownCloud", "No se puede instalar la app porque no es compatible con esta versión de ownCloud");
        translations.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps", "La app no se puede instalar porque contiene la etiqueta <shipped>true</shipped> que no está permitida para apps no distribuidas");
        translations.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store", "La app no puede ser instalada porque la versión en info.xml/version no es la misma que la establecida en el app store");
        translations.insert("App directory already exists", "El directorio de la app ya existe");
        translations.insert("Can't create app folder. Please fix permissions. %s", "No se puede crear el directorio para la app. Corregí los permisos. %s");
        translations.insert("Application is not enabled", "La aplicación no está habilitada");
        translations.insert("Authentication error", "Error al autenticar");
        translations.insert("Token expired. Please reload page.", "Token expirado. Por favor, recargá la página.");
        translations.insert("Files", "Archivos");
        translations.insert("Text", "Texto");
        translations.insert("Images", "Imágenes");
        translations.insert("%s enter the database username.", "%s Entrá el usuario de la base de datos");
        translations.insert("%s enter the database name.", "%s Entrá el nombre de la base de datos.");
        translations.insert("%s you may not use dots in the database name", "%s no podés usar puntos en el nombre de la base de datos");
        translations.insert("MS SQL username and/or password not valid: %s", "Nombre de usuario y contraseña de MS SQL no son válidas: %s");
        translations.insert("You need to enter either an existing account or the administrator.", "Tenés que ingresar una cuenta existente o el administrador.");
        translations.insert("MySQL username and/or password not valid", "Usuario y/o contraseña MySQL no válido");
        translations.insert("DB Error: \"%s\"", "Error DB: \"%s\"");
        translations.insert("Offending command was: \"%s\"", "El comando no comprendido es: \"%s\"");
        translations.insert("MySQL user '%s'@'localhost' exists already.", "Usuario MySQL '%s'@'localhost' ya existe.");
        translations.insert("Drop this user from MySQL", "Borrar este usuario de MySQL");
        translations.insert("MySQL user '%s'@'%%' already exists", "Usuario MySQL '%s'@'%%' ya existe");
        translations.insert("Drop this user from MySQL.", "Borrar este usuario de MySQL");
        translations.insert("Oracle connection could not be established", "No fue posible establecer la conexión a Oracle");
        translations.insert("Oracle username and/or password not valid", "El nombre de usuario y/o contraseña no son válidos");
        translations.insert("Offending command was: \"%s\", name: %s, password: %s", "El comando no comprendido es: \"%s\", nombre: \"%s\", contraseña: \"%s\"");
        translations.insert("PostgreSQL username and/or password not valid", "Nombre de usuario o contraseña PostgradeSQL inválido.");
        translations.insert("Set an admin username.", "Configurar un nombre de administrador.");
        translations.insert("Set an admin password.", "Configurar una contraseña de administrador.");
        translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Tu servidor web no está configurado todavía para permitir sincronización de archivos porque la interfaz WebDAV parece no funcionar.");
        translations.insert("Please double check the <a href='%s'>installation guides</a>.", "Por favor, comprobá nuevamente la <a href='%s'>guía de instalación</a>.");
        translations.insert("Could not find category \"%s\"", "No fue posible encontrar la categoría \"%s\"");
        translations.insert("seconds ago", "segundos atrás");
        translations.insert("_%n minute ago_::_%n minutes ago_", "Hace %n minuto");
        translations.insert("_%n hour ago_::_%n hours ago_", "Hace %n hora");
        translations.insert("today", "hoy");
        translations.insert("yesterday", "ayer");
        translations.insert("_%n day go_::_%n days ago_", "Hace %n día");
        translations.insert("last month", "el mes pasado");
        translations.insert("_%n month ago_::_%n months ago_", "Hace %n mes");
        translations.insert("last year", "el año pasado");
        translations.insert("years ago", "años atrás");
        translations.insert("Caused by:", "Provocado por:");
        
        translations
    }
    
    pub fn get_plural_forms() -> &'static str {
        "nplurals=2; plural=(n != 1);"
    }
    
    pub fn create_bundle() -> FluentBundle {
        let lang_id = langid!("es-AR");
        let mut bundle = FluentBundle::new(&[lang_id]);
        
        // Aquí se configuraría el bundle con los recursos de traducción
        // Este paso requeriría convertir las traducciones al formato Fluent
        
        bundle
    }
    
    pub fn get_plural(key: &str, count: i64) -> String {
        match key {
            "_%n minute ago_::_%n minutes ago_" => {
                if count == 1 {
                    format!("Hace {} minuto", count)
                } else {
                    format!("Hace {} minutos", count)
                }
            },
            "_%n hour ago_::_%n hours ago_" => {
                if count == 1 {
                    format!("Hace {} hora", count)
                } else {
                    format!("Hace {} horas", count)
                }
            },
            "_%n day go_::_%n days ago_" => {
                if count == 1 {
                    format!("Hace {} día", count)
                } else {
                    format!("Hace {} días", count)
                }
            },
            "_%n month ago_::_%n months ago_" => {
                if count == 1 {
                    format!("Hace {} mes", count)
                } else {
                    format!("Hace {} meses", count)
                }
            },
            _ => String::from("Traducción no encontrada"),
        }
    }
}