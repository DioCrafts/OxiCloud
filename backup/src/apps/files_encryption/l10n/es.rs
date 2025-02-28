use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Se ha habilitado la recuperación de archivos");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "No se pudo habilitar la clave de recuperación. Por favor compruebe su contraseña.");
        m.insert("Recovery key successfully disabled", "Clave de recuperación deshabilitada");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "No se pudo deshabilitar la clave de recuperación. Por favor compruebe su contraseña!");
        m.insert("Password successfully changed.", "Su contraseña ha sido cambiada");
        m.insert("Could not change the password. Maybe the old password was not correct.", "No se pudo cambiar la contraseña. Compruebe que la contraseña actual sea correcta.");
        m.insert("Private key password successfully updated.", "Contraseña de clave privada actualizada con éxito.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "No se pudo cambiar la contraseña. Puede que la contraseña antigua no sea correcta.");
        m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "¡La aplicación de cifrado no ha sido inicializada! Quizá fue restablecida durante tu sesión. Por favor intenta cerrar la sesión y volver a iniciarla para inicializar la aplicación de cifrado.");
        m.insert("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "¡Su clave privada no es válida! Tal vez su contraseña ha sido cambiada desde fuera. de %s (Ej:Su directorio corporativo). Puede actualizar la contraseña de su clave privada en sus opciones personales para recuperar el acceso a sus archivos.");
        m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "No fue posible descifrar este archivo, probablemente se trate de un archivo compartido. Solicite al propietario del mismo que vuelva a compartirlo con usted.");
        m.insert("Unknown error please check your system settings or contact your administrator", "Error desconocido. Verifique la configuración de su sistema o póngase en contacto con su administrador");
        m.insert("Missing requirements.", "Requisitos incompletos.");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Por favor, asegúrese de que PHP 5.3.3 o posterior está instalado y que la extensión OpenSSL de PHP está habilitada y configurada correctamente. Por el momento, la aplicación de cifrado ha sido deshabilitada.");
        m.insert("Following users are not set up for encryption:", "Los siguientes usuarios no han sido configurados para el cifrado:");
        m.insert("Saving...", "Guardando...");
        m.insert("Go directly to your ", "Ir directamente a su");
        m.insert("personal settings", "opciones personales");
        m.insert("Encryption", "Cifrado");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Habilitar la clave de recuperación (permite recuperar los ficheros del usuario en caso de pérdida de la contraseña);");
        m.insert("Recovery key password", "Contraseña de clave de recuperación");
        m.insert("Repeat Recovery key password", "Repite la contraseña de clave de recuperación");
        m.insert("Enabled", "Habilitar");
        m.insert("Disabled", "Deshabilitado");
        m.insert("Change recovery key password:", "Cambiar la contraseña de la clave de recuperación");
        m.insert("Old Recovery key password", "Antigua clave de recuperación");
        m.insert("New Recovery key password", "Nueva clave de recuperación");
        m.insert("Repeat New Recovery key password", "Repetir la nueva clave de recuperación");
        m.insert("Change Password", "Cambiar contraseña");
        m.insert("Your private key password no longer match your log-in password:", "Su contraseña de clave privada ya no coincide con su contraseña de acceso:");
        m.insert("Set your old private key password to your current log-in password.", "Establecer la contraseña de su antigua clave privada a su contraseña actual de acceso.");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Si no recuerda su antigua contraseña puede pedir a su administrador que le recupere sus ficheros.");
        m.insert("Old log-in password", "Contraseña de acceso antigua");
        m.insert("Current log-in password", "Contraseña de acceso actual");
        m.insert("Update Private Key Password", "Actualizar Contraseña de Clave Privada");
        m.insert("Enable password recovery:", "Habilitar la recuperación de contraseña:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Habilitar esta opción le permitirá volver a tener acceso a sus ficheros cifrados en caso de pérdida de contraseña");
        m.insert("File recovery settings updated", "Opciones de recuperación de archivos actualizada");
        m.insert("Could not update file recovery", "No se pudo actualizar la recuperación de archivos");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}