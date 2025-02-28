use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Recovery key successfully enabled", "Activada satisfactoriamente a chave de recuperación");
    m.insert("Could not enable recovery key. Please check your recovery key password!", "Non foi posíbel activar a chave de recuperación. Comprobe o contrasinal da chave de recuperación!");
    m.insert("Recovery key successfully disabled", "Desactivada satisfactoriamente a chave de recuperación");
    m.insert("Could not disable recovery key. Please check your recovery key password!", "Non foi posíbel desactivar a chave de recuperación. Comprobe o contrasinal da chave de recuperación!");
    m.insert("Password successfully changed.", "O contrasinal foi cambiado satisfactoriamente");
    m.insert("Could not change the password. Maybe the old password was not correct.", "Non foi posíbel cambiar o contrasinal. Probabelmente o contrasinal antigo non é o  correcto.");
    m.insert("Private key password successfully updated.", "A chave privada foi actualizada correctamente.");
    m.insert("Could not update the private key password. Maybe the old password was not correct.", "Non foi posíbel actualizar o contrasinal da chave privada. É probábel que o contrasinal antigo non sexa correcto.");
    m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Non se iniciou o aplicativo de cifrado! Quizais volva a activarse durante a sesión. Tente pechar a sesión e volver iniciala que tamén se inicie o aplicativo de cifrado.");
    m.insert("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "A chave privada non é correcta! É probábel que o seu contrasinal teña sido cambiado desde o exterior do %s (p.ex. o seu directorio corporativo). Vostede pode actualizar o contrasinal da súa chave privada nos seus axustes persoais para recuperar o acceso aos seus ficheiros");
    m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Non foi posíbel descifrar o ficheiro, probabelmente tratase dun ficheiro compartido. Pidalle ao propietario do ficheiro que  volva compartir o ficheiro con vostede.");
    m.insert("Unknown error please check your system settings or contact your administrator", "Produciuse un erro descoñecido. Comprobe os axustes do sistema ou póñase en contacto co administrador");
    m.insert("Missing requirements.", "Non se cumpren os requisitos.");
    m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Asegúrese de que está instalado o PHP 5.3.3 ou posterior e de o OpenSSL xunto coa extensión PHP estean activados e configurados correctamente. Polo de agora foi desactivado o aplicativo de cifrado.");
    m.insert("Following users are not set up for encryption:", "Os seguintes usuarios non teñen configuración para o cifrado:");
    m.insert("Saving...", "Gardando...");
    m.insert("Go directly to your ", "Vaia directamente ao seu");
    m.insert("personal settings", "axustes persoais");
    m.insert("Encryption", "Cifrado");
    m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Activar a chave de recuperación (permitirá recuperar os ficheiros dos usuarios no caso de perda do contrasinal):");
    m.insert("Recovery key password", "Contrasinal da chave de recuperación");
    m.insert("Repeat Recovery key password", "Repita o contrasinal da chave da recuperación");
    m.insert("Enabled", "Activado");
    m.insert("Disabled", "Desactivado");
    m.insert("Change recovery key password:", "Cambiar o contrasinal da chave de la recuperación:");
    m.insert("Old Recovery key password", "Antigo contrasinal da chave de recuperación");
    m.insert("New Recovery key password", "Novo contrasinal da chave de recuperación");
    m.insert("Repeat New Recovery key password", "Repita o novo contrasinal da chave da recuperación");
    m.insert("Change Password", "Cambiar o contrasinal");
    m.insert("Your private key password no longer match your log-in password:", "O seu contrasinal da chave privada non coincide co seu contrasinal de acceso.");
    m.insert("Set your old private key password to your current log-in password.", "Estabeleza o seu contrasinal antigo da chave de recuperación ao seu contrasinal de acceso actual");
    m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", " Se non lembra o seu antigo contrasinal pode pedírllelo ao seu administrador para recuperar os seus ficheiros.");
    m.insert("Old log-in password", "Contrasinal de acceso antigo");
    m.insert("Current log-in password", "Contrasinal de acceso actual");
    m.insert("Update Private Key Password", "Actualizar o contrasinal da chave privada");
    m.insert("Enable password recovery:", "Activar o  contrasinal de recuperación:");
    m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Ao activar esta opción permitiráselle volver a obter acceso aos ficheiros cifrados no caso de perda do contrasinal");
    m.insert("File recovery settings updated", "Actualizouse o ficheiro de axustes de recuperación");
    m.insert("Could not update file recovery", "Non foi posíbel actualizar o ficheiro de recuperación");
    m
});

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}