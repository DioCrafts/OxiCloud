use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Spanish (Argentina) translations for files_encryption app
pub static ES_AR_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Recovery key successfully enabled", "Se habilitó la recuperación de archivos");
    m.insert("Could not enable recovery key. Please check your recovery key password!", "No se pudo habilitar la clave de recuperación. Por favor, comprobá tu contraseña.");
    m.insert("Recovery key successfully disabled", "Clave de recuperación deshabilitada");
    m.insert("Could not disable recovery key. Please check your recovery key password!", "No fue posible deshabilitar la clave de recuperación.  Por favor, comprobá tu contraseña.");
    m.insert("Password successfully changed.", "Tu contraseña fue cambiada");
    m.insert("Could not change the password. Maybe the old password was not correct.", "No se pudo cambiar la contraseña. Comprobá que la contraseña actual sea correcta.");
    m.insert("Private key password successfully updated.", "Contraseña de clave privada actualizada con éxito.");
    m.insert("Could not update the private key password. Maybe the old password was not correct.", "No fue posible actualizar la contraseña de clave privada. Tal vez la contraseña anterior no es correcta.");
    m.insert("Missing requirements.", "Requisitos incompletos.");
    m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Por favor, asegúrese de que PHP 5.3.3 o una versión más reciente esté instalado y que OpenSSL junto con la extensión PHP esté habilitado y configurado apropiadamente. Por ahora, la aplicación de encriptación ha sido deshabilitada.");
    m.insert("Following users are not set up for encryption:", "Los siguientes usuarios no fueron configurados para encriptar:");
    m.insert("Saving...", "Guardando...");
    m.insert("personal settings", "Configuración personal");
    m.insert("Encryption", "Encriptación");
    m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Habilitar clave de recuperación (te permite recuperar los archivos de usuario en el caso que pierdas la contraseña):");
    m.insert("Recovery key password", "Contraseña de recuperación de clave");
    m.insert("Enabled", "Habilitado");
    m.insert("Disabled", "Deshabilitado");
    m.insert("Change recovery key password:", "Cambiar contraseña para recuperar la clave:");
    m.insert("Old Recovery key password", "Contraseña antigua de recuperación de clave");
    m.insert("New Recovery key password", "Nueva contraseña de recuperación de clave");
    m.insert("Change Password", "Cambiar contraseña");
    m.insert("Your private key password no longer match your log-in password:", "Tu contraseña de clave privada ya no coincide con la contraseña de ingreso:");
    m.insert("Set your old private key password to your current log-in password.", "Usá tu contraseña de clave privada antigua para tu contraseña de ingreso actual.");
    m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Si no te acordás de tu contraseña antigua, pedile al administrador que recupere tus archivos");
    m.insert("Old log-in password", "Contraseña anterior");
    m.insert("Current log-in password", "Contraseña actual");
    m.insert("Update Private Key Password", "Actualizar contraseña de la clave privada");
    m.insert("Enable password recovery:", "Habilitar recuperación de contraseña:");
    m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Habilitando esta opción, vas a tener acceso a tus archivos encriptados, incluso si perdés la contraseña");
    m.insert("File recovery settings updated", "Las opciones de recuperación de archivos fueron actualizadas");
    m.insert("Could not update file recovery", "No fue posible actualizar la recuperación de archivos");
    m
});

/// Provides the plural forms rule for Spanish (Argentina)
pub fn plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}