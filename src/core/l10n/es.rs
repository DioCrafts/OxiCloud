use std::collections::HashMap;
use phf::phf_map;

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "%s shared »%s« with you" => "%s ha compatido  »%s« contigo",
    "Couldn't send mail to following users: %s " => "No se pudo enviar el mensaje a los siguientes usuarios: %s",
    "Turned on maintenance mode" => "Modo mantenimiento activado",
    "Turned off maintenance mode" => "Modo mantenimiento desactivado",
    "Updated database" => "Base de datos actualizada",
    "Updating filecache, this may take really long..." => "Actualizando caché de archivos, esto puede tardar bastante tiempo...",
    "Updated filecache" => "Caché de archivos actualizada",
    "... %d%% done ..." => "... %d%% hecho ...",
    "No image or file provided" => "No se especificó ningún archivo o imagen",
    "Unknown filetype" => "Tipo de archivo desconocido",
    "Invalid image" => "Imagen inválida",
    "No temporary profile picture available, try again" => "No hay disponible una imagen temporal de perfil, pruebe de nuevo",
    "No crop data provided" => "No se proporcionó datos del recorte",
    "Sunday" => "Domingo",
    "Monday" => "Lunes",
    "Tuesday" => "Martes",
    "Wednesday" => "Miércoles",
    "Thursday" => "Jueves",
    "Friday" => "Viernes",
    "Saturday" => "Sábado",
    "January" => "Enero",
    "February" => "Febrero",
    "March" => "Marzo",
    "April" => "Abril",
    "May" => "Mayo",
    "June" => "Junio",
    "July" => "Julio",
    "August" => "Agosto",
    "September" => "Septiembre",
    "October" => "Octubre",
    "November" => "Noviembre",
    "December" => "Diciembre",
    "Settings" => "Ajustes",
    "seconds ago" => "segundos antes",
    "_%n minute ago_::_%n minutes ago_" => "_%n minuto_::_%n minutos_",
    "_%n hour ago_::_%n hours ago_" => "_%n hora_::_%n horas_",
    "today" => "hoy",
    "yesterday" => "ayer",
    "_%n day ago_::_%n days ago_" => "_%n día_::_%n días_",
    "last month" => "el mes pasado",
    "_%n month ago_::_%n months ago_" => "_%n mes_::_%n meses_",
    "months ago" => "meses antes",
    "last year" => "el año pasado",
    "years ago" => "años antes",
    "Choose" => "Seleccionar",
    "Error loading file picker template: {error}" => "Error cargando plantilla del seleccionador de archivos: {error}",
    "Yes" => "Sí",
    "No" => "No",
    "Ok" => "Aceptar",
    "Error loading message template: {error}" => "Error cargando plantilla del mensaje: {error}",
    "_{count} file conflict_::_{count} file conflicts_" => "_{count} conflicto de archivo_::_{count} conflictos de archivo_",
    "One file conflict" => "On conflicto de archivo",
    "Which files do you want to keep?" => "¿Que archivos deseas mantener?",
    "If you select both versions, the copied file will have a number added to its name." => "Si seleccionas ambas versiones, el archivo copiado tendrá añadido un número en su nombre.",
    "Cancel" => "Cancelar",
    "Continue" => "Continuar",
    "(all selected)" => "(seleccionados todos)",
    "({count} selected)" => "({count} seleccionados)",
    "Error loading file exists template" => "Error cargando plantilla de archivo existente",
    "Shared" => "Compartido",
    "Share" => "Compartir",
    "Error" => "Error",
    "Error while sharing" => "Error al compartir",
    "Error while unsharing" => "Error al dejar de compartir",
    "Error while changing permissions" => "Error al cambiar permisos",
    "Shared with you and the group {group} by {owner}" => "Compartido contigo y el grupo {group} por {owner}",
    "Shared with you by {owner}" => "Compartido contigo por {owner}",
    "Share with user or group …" => "Compartido con el usuario o con el grupo ...",
    "Share link" => "Enlace compartido",
    "Password protect" => "Protección con contraseña",
    "Password" => "Contraseña",
    "Allow Public Upload" => "Permitir Subida Pública",
    "Email link to person" => "Enviar enlace por correo electrónico a una persona",
    "Send" => "Enviar",
    "Set expiration date" => "Establecer fecha de caducidad",
    "Expiration date" => "Fecha de caducidad",
    "Share via email:" => "Compartir por correo electrónico:",
    "No people found" => "No se encontró gente",
    "group" => "grupo",
    "Resharing is not allowed" => "No se permite compartir de nuevo",
    "Shared in {item} with {user}" => "Compartido en {item} con {user}",
    "Unshare" => "Dejar de compartir",
    "notify by email" => "notificar al usuario por correo electrónico",
    "can edit" => "puede editar",
    "access control" => "control de acceso",
    "create" => "crear",
    "update" => "actualizar",
    "delete" => "eliminar",
    "share" => "compartir",
    "Password protected" => "Protegido con contraseña",
    "Error unsetting expiration date" => "Error eliminando fecha de caducidad",
    "Error setting expiration date" => "Error estableciendo fecha de caducidad",
    "Sending ..." => "Enviando...",
    "Email sent" => "Correo electrónico enviado",
    "Warning" => "Precaución",
    "The object type is not specified." => "El tipo de objeto no está especificado.",
    "Enter new" => "Ingresar nueva",
    "Delete" => "Eliminar",
    "Add" => "Agregar",
    "Edit tags" => "Editar etiquetas",
    "Error loading dialog template: {error}" => "Error cargando plantilla de diálogo: {error}",
    "No tags selected for deletion." => "No hay etiquetas seleccionadas para borrar.",
    "The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>." => "La actualización ha fracasado. Por favor, informe de este problema a la <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">Comunidad de ownCloud</a>.",
    "The update was successful. Redirecting you to ownCloud now." => "La actualización se ha realizado con éxito. Redireccionando a ownCloud ahora.",
    "%s password reset" => "%s restablecer contraseña",
    "Use the following link to reset your password: {link}" => "Utilice el siguiente enlace para restablecer su contraseña: {link}",
    "The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator ." => "El enlace para restablecer la contraseña ha sido enviada a su correo electrónico. <br> Si no lo recibe en un plazo razonable de tiempo, revise su carpeta de spam / correo no deseado. <br> Si no está allí, pregunte a su administrador local.",
    "Request failed!<br>Did you make sure your email/username was right?" => "La petición ha fallado! <br> ¿Está seguro de que su dirección de correo electrónico o nombre de usuario era correcto?",
    "You will receive a link to reset your password via Email." => "Recibirá un enlace por correo electrónico para restablecer su contraseña",
    "Username" => "Nombre de usuario",
    "Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?" => "Sus archivos están cifrados. Si no ha habilitado la clave de recurperación, no habrá forma de recuperar sus datos luego de que la contraseña sea reseteada. Si no está seguro de qué hacer, contacte a su administrador antes de continuar. ¿Realmente desea continuar?",
    "Yes, I really want to reset my password now" => "Sí. Realmente deseo resetear mi contraseña ahora",
    "Reset" => "Reiniciar",
    "Your password was reset" => "Su contraseña fue restablecida",
    "To login page" => "A la página de inicio de sesión",
    "New password" => "Nueva contraseña",
    "Reset password" => "Restablecer contraseña",
    "Personal" => "Personal",
    "Users" => "Usuarios",
    "Apps" => "Aplicaciones",
    "Admin" => "Administración",
    "Help" => "Ayuda",
    "Error loading tags" => "Error cargando etiquetas.",
    "Tag already exists" => "La etiqueta ya existe",
    "Error deleting tag(s)" => "Error borrando etiqueta(s)",
    "Error tagging" => "Error al etiquetar",
    "Error untagging" => "Error al quitar etiqueta",
    "Error favoriting" => "Error al marcar como favorito",
    "Error unfavoriting" => "Error al quitar como favorito",
    "Access forbidden" => "Acceso denegado",
    "Cloud not found" => "No se encuentra la nube",
    "Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n" => "Hola:\n\nTan solo queremos informarte que %s compartió %s contigo.\nMíralo aquí: %s\n\n",
    "The share will expire on %s.\n\n" => "El objeto dejará de ser compartido el %s.\n\n",
    "Cheers!" => "¡Saludos!",
    "Security Warning" => "Advertencia de seguridad",
    "Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)" => "Su versión de PHP es vulnerable al ataque de Byte NULL (CVE-2006-7243)",
    "Please update your PHP installation to use %s securely." => "Por favor, actualice su instalación PHP para usar %s con seguridad.",
    "No secure random number generator is available, please enable the PHP OpenSSL extension." => "No está disponible un generador de números aleatorios seguro, por favor habilite la extensión OpenSSL de PHP.",
    "Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account." => "Sin un generador de números aleatorios seguro, un atacante podría predecir los tokens de restablecimiento de contraseñas y tomar el control de su cuenta.",
    "Your data directory and files are probably accessible from the internet because the .htaccess file does not work." => "Su directorio de datos y sus archivos probablemente sean accesibles a través de internet ya que el archivo .htaccess no funciona.",
    "For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>." => "Para información de cómo configurar apropiadamente su servidor, por favor vea la <a href=\"%s\" target=\"_blank\">documentación</a>.",
    "Create an <strong>admin account</strong>" => "Crear una <strong>cuenta de administrador</strong>",
    "Advanced" => "Avanzado",
    "Data folder" => "Directorio de datos",
    "Configure the database" => "Configurar la base de datos",
    "will be used" => "se utilizarán",
    "Database user" => "Usuario de la base de datos",
    "Database password" => "Contraseña de la base de datos",
    "Database name" => "Nombre de la base de datos",
    "Database tablespace" => "Espacio de tablas de la base de datos",
    "Database host" => "Host de la base de datos",
    "Finish setup" => "Completar la instalación",
    "Finishing …" => "Finalizando...",
    "%s is available. Get more information on how to update." => "%s esta disponible. Obtener mas información de como actualizar.",
    "Log out" => "Salir",
    "Automatic logon rejected!" => "¡Inicio de sesión automático rechazado!",
    "If you did not change your password recently, your account may be compromised!" => "Si no ha cambiado su contraseña recientemente, ¡puede que su cuenta esté comprometida!",
    "Please change your password to secure your account again." => "Por favor cambie su contraseña para asegurar su cuenta nuevamente.",
    "Server side authentication failed!" => "La autenticación a fallado en el servidor.",
    "Please contact your administrator." => "Por favor, contacte con el administrador.",
    "Lost your password?" => "¿Ha perdido su contraseña?",
    "remember" => "recordar",
    "Log in" => "Entrar",
    "Alternative Logins" => "Inicios de sesión alternativos",
    "Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>" => "Hola:<br><br>tan solo queremos informarte que %s compartió «%s» contigo.<br><a href=\"%s\">¡Míralo acá!</a><br><br>",
    "The share will expire on %s.<br><br>" => "El objeto dejará de ser compartido el %s.<br><br>",
    "Updating ownCloud to version %s, this may take a while." => "Actualizando ownCloud a la versión %s, esto puede demorar un tiempo.",
    "This ownCloud instance is currently being updated, which may take a while." => "Esta versión de owncloud se está actualizando, esto puede demorar un tiempo.",
    "Please reload this page after a short time to continue using ownCloud." => "Por favor , recargue esta instancia de onwcloud tras un corto periodo de tiempo y continue usándolo.",
    "Contact your system administrator if this message persists or appeared unexpectedly." => "Contacte con su administrador de sistemas si este mensaje persiste o aparece de forma inesperada.",
    "Thank you for your patience." => "Gracias por su paciencia."
};

pub fn get_plural_translations(key: &str, count: i64) -> Option<String> {
    let plural_idx = if count != 1 { 1 } else { 0 };
    
    match key {
        "_%n minute ago_::_%n minutes ago_" => {
            if plural_idx == 0 {
                Some(format!("Hace {} minuto", count))
            } else {
                Some(format!("Hace {} minutos", count))
            }
        },
        "_%n hour ago_::_%n hours ago_" => {
            if plural_idx == 0 {
                Some(format!("Hace {} hora", count))
            } else {
                Some(format!("Hace {} horas", count))
            }
        },
        "_%n day ago_::_%n days ago_" => {
            if plural_idx == 0 {
                Some(format!("Hace {} día", count))
            } else {
                Some(format!("Hace {} días", count))
            }
        },
        "_%n month ago_::_%n months ago_" => {
            if plural_idx == 0 {
                Some(format!("Hace {} mes", count))
            } else {
                Some(format!("Hace {} meses", count))
            }
        },
        "_{count} file conflict_::_{count} file conflicts_" => {
            if plural_idx == 0 {
                Some(format!("{} conflicto de archivo", count))
            } else {
                Some(format!("{} conflictos de archivo", count))
            }
        },
        _ => None
    }
}