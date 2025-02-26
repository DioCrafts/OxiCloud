use std::collections::HashMap;
use once_cell::sync::Lazy;

// Definición de las traducciones para español de Argentina
pub static ES_AR_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("%s shared »%s« with you", "%s compartió \"%s\" con vos");
    translations.insert("Turned on maintenance mode", "Modo de mantenimiento activado");
    translations.insert("Turned off maintenance mode", "Modo de mantenimiento desactivado");
    translations.insert("Updated database", "Base de datos actualizada");
    translations.insert("Updating filecache, this may take really long...", "Actualizando caché de archivos, esto puede tardar mucho tiempo...");
    translations.insert("Updated filecache", "Caché de archivos actualizada");
    translations.insert("... %d%% done ...", "... %d%% hecho ...");
    translations.insert("Unknown filetype", "Tipo de archivo desconocido");
    translations.insert("Invalid image", "Imagen inválida");
    translations.insert("Sunday", "Domingo");
    translations.insert("Monday", "Lunes");
    translations.insert("Tuesday", "Martes");
    translations.insert("Wednesday", "Miércoles");
    translations.insert("Thursday", "Jueves");
    translations.insert("Friday", "Viernes");
    translations.insert("Saturday", "Sábado");
    translations.insert("January", "enero");
    translations.insert("February", "febrero");
    translations.insert("March", "marzo");
    translations.insert("April", "abril");
    translations.insert("May", "mayo");
    translations.insert("June", "junio");
    translations.insert("July", "julio");
    translations.insert("August", "agosto");
    translations.insert("September", "septiembre");
    translations.insert("October", "octubre");
    translations.insert("November", "noviembre");
    translations.insert("December", "diciembre");
    translations.insert("Settings", "Configuración");
    translations.insert("seconds ago", "segundos atrás");
    translations.insert("today", "hoy");
    translations.insert("yesterday", "ayer");
    translations.insert("last month", "el mes pasado");
    translations.insert("months ago", "meses atrás");
    translations.insert("last year", "el año pasado");
    translations.insert("years ago", "años atrás");
    translations.insert("Choose", "Elegir");
    translations.insert("Yes", "Sí");
    translations.insert("No", "No");
    translations.insert("Ok", "Aceptar");
    translations.insert("Cancel", "Cancelar");
    translations.insert("Shared", "Compartido");
    translations.insert("Share", "Compartir");
    translations.insert("Error", "Error");
    translations.insert("Error while sharing", "Error al compartir");
    translations.insert("Error while unsharing", "Error en al dejar de compartir");
    translations.insert("Error while changing permissions", "Error al cambiar permisos");
    translations.insert("Shared with you and the group {group} by {owner}", "Compartido con vos y el grupo {group} por {owner}");
    translations.insert("Shared with you by {owner}", "Compartido con vos por {owner}");
    translations.insert("Password protect", "Proteger con contraseña ");
    translations.insert("Password", "Contraseña");
    translations.insert("Allow Public Upload", "Permitir Subida Pública");
    translations.insert("Email link to person", "Enviar el enlace por e-mail.");
    translations.insert("Send", "Mandar");
    translations.insert("Set expiration date", "Asignar fecha de vencimiento");
    translations.insert("Expiration date", "Fecha de vencimiento");
    translations.insert("Share via email:", "Compartir a través de e-mail:");
    translations.insert("No people found", "No se encontraron usuarios");
    translations.insert("group", "grupo");
    translations.insert("Resharing is not allowed", "No se permite volver a compartir");
    translations.insert("Shared in {item} with {user}", "Compartido en {item} con {user}");
    translations.insert("Unshare", "Dejar de compartir");
    translations.insert("can edit", "podés editar");
    translations.insert("access control", "control de acceso");
    translations.insert("create", "crear");
    translations.insert("update", "actualizar");
    translations.insert("delete", "borrar");
    translations.insert("share", "compartir");
    translations.insert("Password protected", "Protegido por contraseña");
    translations.insert("Error unsetting expiration date", "Error al remover la fecha de vencimiento");
    translations.insert("Error setting expiration date", "Error al asignar fecha de vencimiento");
    translations.insert("Sending ...", "Mandando...");
    translations.insert("Email sent", "e-mail mandado");
    translations.insert("Warning", "Atención");
    translations.insert("The object type is not specified.", "El tipo de objeto no está especificado. ");
    translations.insert("Delete", "Borrar");
    translations.insert("Add", "Agregar");
    translations.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "La actualización no pudo ser completada. Por favor, reportá el inconveniente a la comunidad <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud</a>.");
    translations.insert("The update was successful. Redirecting you to ownCloud now.", "La actualización fue exitosa. Estás siendo redirigido a ownCloud.");
    translations.insert("%s password reset", "%s restablecer contraseña");
    translations.insert("Use the following link to reset your password: {link}", "Usá este enlace para restablecer tu contraseña: {link}");
    translations.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "El enlace para restablecer la contraseña fue enviada a tu e-mail. <br> Si no lo recibís en un plazo de tiempo razonable,  revisá tu carpeta de spam / correo no deseado. <br> Si no está ahí, preguntale a tu administrador.");
    translations.insert("Request failed!<br>Did you make sure your email/username was right?", "¡Error en el pedido! <br> ¿Estás seguro de que tu dirección de correo electrónico o nombre de usuario son correcto?");
    translations.insert("You will receive a link to reset your password via Email.", "Vas a recibir un enlace por e-mail para restablecer tu contraseña.");
    translations.insert("Username", "Nombre de usuario");
    translations.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "Tus archivos están encriptados. Si no habilitaste la clave de recuperación, no vas a tener manera de obtener nuevamente tus datos después que se restablezca tu contraseña. Si no estás seguro sobre qué hacer, ponete en contacto con el administrador antes de seguir. ¿Estás seguro/a que querés continuar?");
    translations.insert("Yes, I really want to reset my password now", "Sí, definitivamente quiero restablecer mi contraseña ahora");
    translations.insert("Reset", "Resetear");
    translations.insert("Your password was reset", "Tu contraseña fue restablecida");
    translations.insert("To login page", "A la página de inicio de sesión");
    translations.insert("New password", "Nueva contraseña:");
    translations.insert("Reset password", "Restablecer contraseña");
    translations.insert("Personal", "Personal");
    translations.insert("Users", "Usuarios");
    translations.insert("Apps", "Apps");
    translations.insert("Admin", "Administración");
    translations.insert("Help", "Ayuda");
    translations.insert("Access forbidden", "Acceso prohibido");
    translations.insert("Cloud not found", "No se encontró ownCloud");
    translations.insert("Security Warning", "Advertencia de seguridad");
    translations.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "La versión de PHP que tenés, es vulnerable al ataque de byte NULL (CVE-2006-7243)");
    translations.insert("Please update your PHP installation to use %s securely.", "Por favor, actualizá tu instalación PHP para poder usar %s de manera segura.");
    translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "No hay disponible ningún generador de números aleatorios seguro. Por favor, habilitá la extensión OpenSSL de PHP.");
    translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Sin un generador de números aleatorios seguro un atacante podría predecir las pruebas de reinicio de tu contraseña y tomar control de tu cuenta.");
    translations.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Tu directorio de datos y tus archivos probablemente son accesibles a través de internet, ya que el archivo .htaccess no está funcionando.");
    translations.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "Para información sobre cómo configurar apropiadamente tu servidor, por favor mirá la <a href=\"%s\" target=\"_blank\">documentación</a>.");
    translations.insert("Create an <strong>admin account</strong>", "Crear una <strong>cuenta de administrador</strong>");
    translations.insert("Advanced", "Avanzado");
    translations.insert("Data folder", "Directorio de almacenamiento");
    translations.insert("Configure the database", "Configurar la base de datos");
    translations.insert("will be used", "se usarán");
    translations.insert("Database user", "Usuario de la base de datos");
    translations.insert("Database password", "Contraseña de la base de datos");
    translations.insert("Database name", "Nombre de la base de datos");
    translations.insert("Database tablespace", "Espacio de tablas de la base de datos");
    translations.insert("Database host", "Huésped de la base de datos");
    translations.insert("Finish setup", "Completar la instalación");
    translations.insert("%s is available. Get more information on how to update.", "%s está disponible. Obtené más información sobre cómo actualizar.");
    translations.insert("Log out", "Cerrar la sesión");
    translations.insert("Automatic logon rejected!", "¡El inicio de sesión automático fue rechazado!");
    translations.insert("If you did not change your password recently, your account may be compromised!", "¡Si no cambiaste tu contraseña recientemente, puede ser que tu cuenta esté comprometida!");
    translations.insert("Please change your password to secure your account again.", "Por favor, cambiá tu contraseña para incrementar la seguridad de tu cuenta.");
    translations.insert("Lost your password?", "¿Perdiste tu contraseña?");
    translations.insert("remember", "recordame");
    translations.insert("Log in", "Iniciar sesión");
    translations.insert("Alternative Logins", "Nombre alternativos de usuarios");
    translations.insert("Updating ownCloud to version %s, this may take a while.", "Actualizando ownCloud a la versión %s, puede demorar un rato.");
    translations
});

// Definición de las traducciones con plurales
pub static ES_AR_PLURALS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut plurals = HashMap::new();
    plurals.insert("_%n minute ago_::_%n minutes ago_", vec!["Hace %n minuto", "Hace %n minutos"]);
    plurals.insert("_%n hour ago_::_%n hours ago_", vec!["Hace %n hora", "Hace %n horas"]);
    plurals.insert("_%n day ago_::_%n days ago_", vec!["Hace %n día", "Hace %n días"]);
    plurals.insert("_%n month ago_::_%n months ago_", vec!["Hace %n mes", "Hace %n meses"]);
    plurals.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", ""]);
    plurals
});

// Definición de la forma plural
pub static ES_AR_PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

// Función para traducir texto (singular)
pub fn translate(key: &str) -> &'static str {
    ES_AR_TRANSLATIONS.get(key).copied().unwrap_or(key)
}

// Función para traducir texto con plurales
pub fn translate_plural(key: &str, count: usize) -> &'static str {
    if let Some(forms) = ES_AR_PLURALS.get(key) {
        let plural_index = if count != 1 { 1 } else { 0 };
        forms.get(plural_index).copied().unwrap_or(key)
    } else {
        key
    }
}